use std::marker::PhantomData;
use super::prompt::completion_to_output;
use super::prompt::stream_to_output;
use async_openai::types::ChatCompletionRequestMessage;

use async_openai::types::ChatCompletionRequestUserMessageContent;
use ai_chain::options::Opt;
use ai_chain::options::Options;
use ai_chain::options::OptionsCascade;
use ai_chain::output::Output;
use ai_chain::tokens::TokenCollection;
use tiktoken_rs::{cl100k_base, get_bpe_from_tokenizer};
use tiktoken_rs::tokenizer::get_tokenizer;

use super::prompt::create_chat_completion_request;
use super::prompt::format_chat_messages;
use async_openai::error::OpenAIError;
use ai_chain::prompt::Prompt;

use ai_chain::tokens::PromptTokensError;
use ai_chain::tokens::{Tokenizer, TokenizerError};
use ai_chain::traits;
use ai_chain::traits::{ExecutorCreationError, ExecutorError};

use async_trait::async_trait;
use ai_chain::tokens::TokenCount;

use std::sync::Arc;
use tiktoken_rs::tokenizer::Tokenizer::Cl100kBase;
use crate::chatgpt::config::OAIConfig;
use crate::chatgpt::OpenAICompatibleInnerError;

/// The `Executor` struct for the ChatGPT model. This executor uses the `async_openai` crate to communicate with the OpenAI API.
#[derive(Clone)]
pub struct Executor<C: OAIConfig> {
    /// The client used to communicate with the OpenAI API.
    client: Arc<async_openai::Client<C>>,
    /// The per-invocation options for this executor.
    options: Options,
    config: PhantomData<C>,
}


impl<C: OAIConfig> Executor<C> {
    /// Creates a new `Executor` with the given client.
    pub fn for_client(client: async_openai::Client<C>, options: Options) -> Self {
        use ai_chain::traits::Executor as _;
        let mut exec = Self::new_with_options(options).unwrap();
        exec.client = Arc::new(client);
        exec
    }

    fn get_model_from_invocation_options(&self, opts: &OptionsCascade) -> String {
        let Some(Opt::Model(model)) = opts.get(ai_chain::options::OptDiscriminants::Model) else {
            return C::model_config().0
        };
        model.to_name()
    }

    fn cascade<'a>(&'a self, opts: Option<&'a Options>) -> OptionsCascade<'a> {
        let mut v: Vec<&'a Options> = vec![&self.options];
        if let Some(o) = opts {
            v.push(o);
        }
        OptionsCascade::from_vec(v)
    }
}

#[derive(thiserror::Error, Debug)]
#[error(transparent)]
pub enum Error {
    OpenAIError(#[from] OpenAIError),
}


impl<OConfig: OAIConfig> Executor<OConfig> {
    fn create_config() -> OConfig {
        OConfig::create()
    }

    fn num_tokens_from_messages(
        model: &str,
        messages: &[ChatCompletionRequestMessage],
    ) -> Result<usize, PromptTokensError> {
        let mut tokenizer = Cl100kBase;
        let (_, config_params) = OConfig::model_config();
        if config_params.iter().any(|x| model.starts_with(x)) {
            tokenizer = Cl100kBase;
        } else {
            let tokenizer1 = get_tokenizer(model).ok_or_else(|| PromptTokensError::NotAvailable)?;
            if tokenizer1 != Cl100kBase {
                return Err(PromptTokensError::NotAvailable);
            }
            tokenizer = tokenizer1;
        }

        let bpe = get_bpe_from_tokenizer(tokenizer).map_err(|_| PromptTokensError::NotAvailable)?;

        let (tokens_per_message, tokens_per_name) = if model.starts_with("gpt-3.5") {
            (
                4,  // every message follows <im_start>{role/name}\n{content}<im_end>\n
                -1, // if there's a name, the role is omitted
            )
        } else {
            (3, 1)
        };

        let mut num_tokens: i32 = 0;
        for message in messages {
            let (role, content, name) = match message {
                ChatCompletionRequestMessage::System(x) => (
                    x.role.to_string(),
                    x.content.to_owned().unwrap_or_default(),
                    None,
                ),
                ChatCompletionRequestMessage::User(x) => (
                    x.role.to_string(),
                    x.content
                        .as_ref()
                        .and_then(|x| match x {
                            ChatCompletionRequestUserMessageContent::Text(x) => Some(x.to_string()),
                            _ => None,
                        })
                        .unwrap_or_default(),
                    None,
                ),
                ChatCompletionRequestMessage::Assistant(x) => (
                    x.role.to_string(),
                    x.content.to_owned().unwrap_or_default(),
                    None,
                ),
                ChatCompletionRequestMessage::Tool(x) => (
                    x.role.to_string(),
                    x.content.to_owned().unwrap_or_default(),
                    None,
                ),
                ChatCompletionRequestMessage::Function(x) => (
                    x.role.to_string(),
                    x.content.to_owned().unwrap_or_default(),
                    None,
                ),
            };
            num_tokens += tokens_per_message;
            num_tokens += bpe.encode_with_special_tokens(&role).len() as i32;
            num_tokens += bpe.encode_with_special_tokens(&content).len() as i32;
            if let Some(name) = name {
                num_tokens += bpe.encode_with_special_tokens(name).len() as i32;
                num_tokens += tokens_per_name;
            }
        }
        num_tokens += 3; // every reply is primed with <|start|>assistant<|message|>
        Ok(num_tokens as usize)
    }
}

#[async_trait]
impl<OConfig: OAIConfig> traits::Executor for Executor<OConfig> {
    type StepTokenizer<'a> = OpenAITokenizer<OConfig>;
    /// Creates a new `Executor` with the given options.
    ///
    /// if the `OPENAI_ORG_ID` environment variable is present, it will be used as the org_ig for the OpenAI client.
    fn new_with_options(options: Options) -> Result<Self, ExecutorCreationError> {
        let mut cfg = Self::create_config();

        let opts = OptionsCascade::new().with_options(&options);

        if let Some(Opt::ApiKey(api_key)) = opts.get(ai_chain::options::OptDiscriminants::ApiKey) {
            cfg = cfg.with_api_key(api_key)
        }
        if let Ok(base_url) = std::env::var("OPENAI_API_BASE_URL") {
            cfg = cfg.with_api_base(base_url);
        }

        let client = Arc::new(async_openai::Client::with_config(cfg));
        Ok(Self {
            client,
            options,
            config: PhantomData,
        })
    }

    async fn execute(&self, options: &Options, prompt: &Prompt) -> Result<Output, ExecutorError> {
        let opts = self.cascade(Some(options));
        let client = self.client.clone();
        let model = self.get_model_from_invocation_options(&opts);
        let input = create_chat_completion_request(model, prompt, opts.is_streaming()).unwrap();
        if opts.is_streaming() {
            let res = async move { client.chat().create_stream(input).await }
                .await
                .map_err(|e| ExecutorError::InnerError(e.into()))?;
            Ok(stream_to_output(res))
        } else {
            let res = async move { client.chat().create(input).await }
                .await
                .map_err(|e| ExecutorError::InnerError(e.into()))?;
            Ok(completion_to_output(res))
        }
    }

    fn tokens_used(
        &self,
        opts: &Options,
        prompt: &Prompt,
    ) -> Result<TokenCount, PromptTokensError> {
        let opts_cas = self.cascade(Some(opts));
        let model = self.get_model_from_invocation_options(&opts_cas);
        let messages = format_chat_messages(prompt.to_chat()).map_err(|e| match e {
            OpenAICompatibleInnerError::StringTemplateError(e) => PromptTokensError::PromptFormatFailed(e),
            _ => PromptTokensError::UnableToCompute,
        })?;
        let tokens_used = Self::num_tokens_from_messages(&model, &messages)
            .map_err(|_| PromptTokensError::NotAvailable)?;

        Ok(TokenCount::new(
            self.max_tokens_allowed(opts),
            tokens_used as i32,
        ))
    }
    /// Get the context size from the model or return default context size
    fn max_tokens_allowed(&self, opts: &Options) -> i32 {
        let opts_cas = self.cascade(Some(opts));
        let model = self.get_model_from_invocation_options(&opts_cas);
        tiktoken_rs::model::get_context_size(&model)
            .try_into()
            .unwrap_or(4096)
    }

    fn answer_prefix(&self, _prompt: &Prompt) -> Option<String> {
        None
    }

    fn get_tokenizer(&self, options: &Options) -> Result<OpenAITokenizer<OConfig>, TokenizerError> {
        Ok(OpenAITokenizer::new(self.cascade(Some(options))))
    }
}


pub struct OpenAITokenizer<C> {
    model_name: String,
    config: Option<C>,
}

impl<C: OAIConfig> OpenAITokenizer<C> {
    pub fn new(options: OptionsCascade) -> Self {
        let model_name = match options.get(ai_chain::options::OptDiscriminants::Model) {
            Some(Opt::Model(model_name)) => model_name.to_name(),
            _ => C::model_config().0
        };
        Self::for_model_name(model_name)
    }
    /// Creates an OpenAITokenizer for the passed in model name
    pub fn for_model_name<S: Into<String>>(model_name: S) -> Self {
        let model_name: String = model_name.into();
        Self { model_name, config: None }
    }

    fn get_bpe_from_model(&self) -> Result<tiktoken_rs::CoreBPE, PromptTokensError> {
        use tiktoken_rs::get_bpe_from_model;
        let (_, config_params) = C::model_config();
        if config_params.iter().any(|x| self.model_name.starts_with(x)) {
            return cl100k_base().map_err(|_| PromptTokensError::NotAvailable);
        }
        get_bpe_from_model(&self.model_name).map_err(|_| PromptTokensError::NotAvailable)
    }
}

impl<C: OAIConfig> Tokenizer for OpenAITokenizer<C> {
    fn tokenize_str(&self, doc: &str) -> Result<TokenCollection, TokenizerError> {
        // if model.starts_with("moonshot") {
        //     tokenizer = Cl100kBase;
        Ok(self
            .get_bpe_from_model()
            .map_err(|_| TokenizerError::TokenizationError)?
            .encode_ordinary(doc)
            .into())
    }

    fn to_string(&self, tokens: TokenCollection) -> Result<String, TokenizerError> {
        let res = self
            .get_bpe_from_model()
            .map_err(|_e| TokenizerError::ToStringError)?
            .decode(tokens.as_usize()?)
            .map_err(|_e| TokenizerError::ToStringError)?;
        Ok(res)
    }
}
