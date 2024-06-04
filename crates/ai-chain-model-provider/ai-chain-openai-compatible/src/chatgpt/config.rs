use async_openai::config::{Config, OPENAI_API_BASE};
use ai_chain::tokens::Tokenizer;

/// config extension
pub trait OAIConfig: Config + Send + Sync + 'static {


    fn create() -> Self;

    fn with_api_key<S: Into<String>>(&mut self, api_key: S) -> Self;

    /// To use a API base url different from default [OPENAI_API_BASE]
    fn with_api_base<S: Into<String>>(&mut self, api_base: S) -> Self;

    fn model_config() ->(String, Vec<String>);


    fn tokenizer(&self)->Box<dyn Tokenizer>;
}

trait OpenAICompatibleTokenizer: Tokenizer {

}

