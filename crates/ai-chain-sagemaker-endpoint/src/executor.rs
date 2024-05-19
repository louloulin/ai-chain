use crate::model::Formatter;
use crate::model::Model;
use async_trait::async_trait;

use ai_chain::options::Opt;
use ai_chain::options::Options;
use ai_chain::options::OptionsCascade;
use ai_chain::output::Output;
use ai_chain::prompt::Prompt;
use ai_chain::tokens::{
    PromptTokensError, TokenCollection, TokenCount, Tokenizer, TokenizerError,
};
use ai_chain::traits::{ExecutorCreationError, ExecutorError};

use std::str::FromStr;

/// Executor is responsible for running the LLM and managing its context.
pub struct Executor {
    #[allow(dead_code)]
    options: Options,
    sagemaker_client: aws_sdk_sagemakerruntime::Client,
}

impl Executor {
    fn get_model_from_invocation_options(&self, opts: &OptionsCascade) -> Model {
        let Some(Opt::Model(model)) = opts.get(ai_chain::options::OptDiscriminants::Model) else {
            // TODO: fail gracefully
            panic!("The Model option must not be empty. This option does not have a default.");
        };
        Model::from_str(&model.to_name()).unwrap()
    }

    fn cascade<'a>(&'a self, opts: Option<&'a Options>) -> OptionsCascade<'a> {
        let mut v: Vec<&'a Options> = vec![&self.options];
        if let Some(o) = opts {
            v.push(o);
        }
        OptionsCascade::from_vec(v)
    }
}

#[async_trait]
impl ai_chain::traits::Executor for Executor {
    type StepTokenizer<'a> = SageMakerEndpointTokenizer;

    fn new_with_options(options: Options) -> Result<Self, ExecutorCreationError> {
        let config = futures::executor::block_on(aws_config::load_from_env());
        let client = aws_sdk_sagemakerruntime::Client::new(&config);
        Ok(Executor {
            options,
            sagemaker_client: client,
        })
    }

    async fn execute(&self, options: &Options, prompt: &Prompt) -> Result<Output, ExecutorError> {
        let opts = self.cascade(Some(options));
        let model = self.get_model_from_invocation_options(&opts);

        let body_blob = model.format_request(prompt, &opts);

        let result = self
            .sagemaker_client
            .invoke_endpoint()
            .endpoint_name(model.to_jumpstart_endpoint_name())
            .content_type(model.request_content_type())
            .body(body_blob)
            .send()
            .await;
        let response = result.map_err(|e| ExecutorError::InnerError(e.into()))?;
        let generated_text = model.parse_response(response);

        Ok(Output::new_immediate(Prompt::text(generated_text)))
    }

    fn tokens_used(
        &self,
        _options: &Options,
        _prompt: &Prompt,
    ) -> Result<TokenCount, PromptTokensError> {
        // Not all models expose this information.
        unimplemented!();
    }

    fn max_tokens_allowed(&self, _: &Options) -> i32 {
        // Not all models expose this information.
        unimplemented!();
    }

    fn answer_prefix(&self, _prompt: &Prompt) -> Option<String> {
        // Not all models expose this information.
        unimplemented!();
    }

    fn get_tokenizer(&self, _: &Options) -> Result<Self::StepTokenizer<'_>, TokenizerError> {
        // Not all models expose this information.
        unimplemented!();
    }
}

pub struct SageMakerEndpointTokenizer {}

impl SageMakerEndpointTokenizer {
    pub fn new(_executor: &Executor) -> Self {
        SageMakerEndpointTokenizer {}
    }
}

impl Tokenizer for SageMakerEndpointTokenizer {
    fn tokenize_str(&self, _doc: &str) -> Result<TokenCollection, TokenizerError> {
        unimplemented!();
    }

    fn to_string(&self, _tokens: TokenCollection) -> Result<String, TokenizerError> {
        unimplemented!();
    }
}
