use async_openai::config::Config;

/// config extension
pub trait OAIConfig: Config + Send + 'static {}