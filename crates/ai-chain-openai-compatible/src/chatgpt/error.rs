use async_openai::error::OpenAIError;
use ai_chain::prompt::StringTemplateError;
use thiserror::Error;

#[derive(Debug, Error)]
#[error(transparent)]
pub enum OpenAICompatibleInnerError {
    #[error(transparent)]
    OpenAIError(#[from] OpenAIError),
    #[error(transparent)]
    StringTemplateError(#[from] StringTemplateError),
}
