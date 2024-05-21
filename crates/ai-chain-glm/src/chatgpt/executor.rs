use async_openai::config::{Config, OPENAI_API_BASE, OPENAI_BETA_HEADER, OPENAI_ORGANIZATION_HEADER, OpenAIConfig};
use reqwest::header::{HeaderMap, AUTHORIZATION};
use secrecy::{ExposeSecret, Secret};
use strum::IntoEnumIterator;
use ai_chain::tokens::Tokenizer;
use ai_chain_openai_compatible::chatgpt::OAIConfig;
use crate::chatgpt::Model;


const GLM_BASE_URL: &str = "https://open.bigmodel.cn/api/paas/v4/";
pub type Executor = ai_chain_openai_compatible::chatgpt::Executor<GLMConfig>;

pub struct GLMConfig {
    api_base: String,
    api_key: Secret<String>,
}


impl Default for GLMConfig {
    fn default() -> Self {
        Self {
            api_base: GLM_BASE_URL.to_string(),
            api_key: std::env::var("OPENAI_API_KEY")
                .unwrap_or_else(|_| "".to_string())
                .into(),
        }
    }
}

impl Config for GLMConfig {
    fn headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();

        headers.insert(
            AUTHORIZATION,
            format!("Bearer {}", self.api_key.expose_secret())
                .as_str()
                .parse()
                .unwrap(),
        );

        // hack for Assistants APIs
        // Calls to the Assistants API require that you pass a Beta header
        headers.insert(OPENAI_BETA_HEADER, "assistants=v1".parse().unwrap());

        headers
    }

    fn url(&self, path: &str) -> String {
        format!("{}{}", self.api_base, path)
    }

    fn api_base(&self) -> &str {
        &self.api_base
    }

    fn api_key(&self) -> &Secret<String> {
        &self.api_key
    }

    fn query(&self) -> Vec<(&str, &str)> {
        vec![]
    }
}

impl Clone for GLMConfig {
    fn clone(&self) -> Self {
        Self {
            api_base: self.api_base.clone(),
            api_key: self.api_key.clone(),
        }
    }
}

impl OAIConfig for GLMConfig {
    fn create() -> Self {
        Self::default()
    }

    fn with_api_key<S: Into<String>>(&mut self, api_key: S) -> Self {
        self.api_key = Secret::from(api_key.into());
        self.clone()
    }

    /// To use a API base url different from default [OPENAI_API_BASE]
    fn with_api_base<S: Into<String>>(&mut self, api_base: S) -> Self {
        self.api_base = api_base.into();
        self.clone()
    }

    fn model_config() -> (String, Vec<String>) {
        return (Model::default().to_string(), Model::iter().map(|model| model.to_string()).collect::<Vec<String>>());
    }

    fn tokenizer(&self) -> Box<dyn Tokenizer> {
        todo!()
    }
}