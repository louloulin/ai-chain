
use async_openai::types::ChatCompletionRequestMessage;

use async_openai::types::ChatCompletionRequestUserMessageContent;
use ai_chain::options::Opt;
use ai_chain::options::OptionsCascade;
use ai_chain::tokens::TokenCollection;
use tiktoken_rs::{cl100k_base, get_bpe_from_tokenizer};
use tiktoken_rs::tokenizer::get_tokenizer;


use ai_chain::tokens::PromptTokensError;
use ai_chain::tokens::{Tokenizer, TokenizerError};

use std::sync::Arc;
use tiktoken_rs::tokenizer::Tokenizer::Cl100kBase;
use crate::chatgpt::Config::MoonConfig;





fn num_tokens_from_messages(
    model: &str,
    messages: &[ChatCompletionRequestMessage],
) -> Result<usize, PromptTokensError> {
    let mut tokenizer = Cl100kBase;

    if model.starts_with("moonshot") {
        tokenizer = Cl100kBase;
    } else {
        let tokenizer1 = get_tokenizer(model).ok_or_else(|| PromptTokensError::NotAvailable)?;
        if tokenizer1 != Cl100kBase {
            return Err(PromptTokensError::NotAvailable);
        }
        tokenizer = tokenizer1;
    }

    let bpe = get_bpe_from_tokenizer(tokenizer).map_err(|_| PromptTokensError::NotAvailable)?;

    let (tokens_per_message, tokens_per_name) = if model.starts_with("moonshot-v1-8k") {
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

pub struct OpenAITokenizer {
    model_name: String,
}

pub type Executor = ai_chain_openai_compatible::chatgpt::Executor<MoonConfig>;

impl OpenAITokenizer {
    pub fn new(options: OptionsCascade) -> Self {
        let model_name = match options.get(ai_chain::options::OptDiscriminants::Model) {
            Some(Opt::Model(model_name)) => model_name.to_name(),
            _ => "moonshot-v1-8k".to_string(),
        };
        Self::for_model_name(model_name)
    }
    /// Creates an OpenAITokenizer for the passed in model name
    pub fn for_model_name<S: Into<String>>(model_name: S) -> Self {
        let model_name: String = model_name.into();
        Self { model_name }
    }

    fn get_bpe_from_model(&self) -> Result<tiktoken_rs::CoreBPE, PromptTokensError> {
        use tiktoken_rs::get_bpe_from_model;
        if self.model_name.starts_with("moonshot") {
            return cl100k_base().map_err(|_| PromptTokensError::NotAvailable);
        }
        get_bpe_from_model(&self.model_name).map_err(|_| PromptTokensError::NotAvailable)
    }
}

impl Tokenizer for OpenAITokenizer {
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
