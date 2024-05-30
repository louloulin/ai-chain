use async_trait::async_trait;
use reqwest::Method;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::tools::{Describe, Tool, ToolDescription, ToolError};

pub struct Scraper {
    base_url: String,
}

impl Scraper {
    pub fn new(base_url: String) -> Self {
        Self { base_url }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ScraperInput {
    pub query: String,
    pub selector: String,
}

impl From<&str> for ScraperInput {
    fn from(value: &str) -> Self {
        let mut parts = value.splitn(2, ' ');
        let query = parts.next().unwrap_or_default().to_string();
        let selector = parts.next().unwrap_or_default().to_string();
        Self { query, selector }
    }
}

impl From<String> for ScraperInput {
    fn from(value: String) -> Self {
        let mut parts = value.splitn(2, ' ');
        let query = parts.next().unwrap_or_default().to_string();
        let selector = parts.next().unwrap_or_default().to_string();
        Self { query, selector }
    }
}

impl Describe for ScraperInput {
    fn describe() -> crate::tools::Format {
        vec![
            ("query", "Search query to find necessary information").into(),
            ("selector", "CSS selector to extract information from the webpage").into(),
        ]
            .into()
    }
}

#[derive(Serialize, Deserialize)]
pub struct ScraperOutput {
    pub result: String,
}

impl From<String> for ScraperOutput {
    fn from(value: String) -> Self {
        Self { result: value }
    }
}

impl From<ScraperOutput> for String {
    fn from(val: ScraperOutput) -> Self {
        val.result
    }
}

impl Describe for ScraperOutput {
    fn describe() -> crate::tools::Format {
        vec![(
            "result",
            "Information extracted from the webpage that should answer your query",
        )
            .into()]
            .into()
    }
}

#[derive(Debug, Error)]
pub enum ScraperError {
    #[error(transparent)]
    YamlError(#[from] serde_yaml::Error),
    #[error("No information found using the provided selector")]
    NoInformationFound,
    #[error(transparent)]
    HtmlParse(#[from] Box<dyn std::error::Error>),
    #[error(transparent)]
    Request(#[from] reqwest::Error),
}

impl ToolError for ScraperError {}

#[async_trait]
impl Tool for Scraper {
    type Input = ScraperInput;

    type Output = ScraperOutput;

    type Error = ScraperError;

    async fn invoke_typed(&self, input: &Self::Input) -> Result<Self::Output, Self::Error> {
        let client = reqwest::Client::new();
        let response = client
            .request(Method::GET, &self.base_url)
            .send()
            .await?
            .text()
            .await?;
        let document = Html::parse_document(&response);
        let selector = Selector::parse(&input.selector).map_err(|_| ScraperError::NoInformationFound)?;
        let mut results = document.select(&selector).map(|node| node.text().collect::<String>()).collect::<Vec<_>>();
        if results.is_empty() {
            Err(ScraperError::NoInformationFound)
        } else {
            Ok(results.join(" ").into())
        }
    }

    fn description(&self) -> ToolDescription {
        ToolDescription::new(
            "Webpage Scraper",
            "Useful for extracting information from webpages based on a CSS selector. Input should be a search query and a CSS selector.",
            "Use this to extract information from webpages based on the provided query and selector.",
            ScraperInput::describe(),
            ScraperOutput::describe(),
        )
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;

    #[tokio::test]
    async fn test_scraper_invoke_typed() {
        // 准备测试数据
        let base_url = "https://wwww.baidu.com".to_string();
        let scraper = Scraper::new(base_url);
        let input = ScraperInput {
            query: "test".to_string(),
            selector: "div.test".to_string(),
        };

        // 调用 invoke_typed 方法
        let result = scraper.invoke_typed(&input).await;

        // 断言结果
        match result {
            Ok(output) => {
                assert!(!output.result.is_empty());
            }
            Err(e) => {
                // 根据需要处理错误
                eprintln!("Error: {}", e);
            }
        }
    }

    #[test]
    fn test_scraper_input_from_str() {
        // 测试从 &str 转换为 ScraperInput
        let input: ScraperInput = "test div.test".into();
        assert_eq!(input.query, "test");
        assert_eq!(input.selector, "div.test");
    }

    #[test]
    fn test_scraper_input_from_string() {
        // 测试从 String 转换为 ScraperInput
        let input: ScraperInput = "test div.test".to_string().into();
        assert_eq!(input.query, "test");
        assert_eq!(input.selector, "div.test");
    }

    // 可以添加更多的测试用例来测试不同的功能和错误处理
}