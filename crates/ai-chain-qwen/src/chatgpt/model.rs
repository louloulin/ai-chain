use ai_chain::options::{ModelRef, Opt};
use serde::{Deserialize, Serialize};
use strum_macros::{EnumIter,EnumString};

/// The `Model` enum represents the available ChatGPT models that you can use through the OpenAI
/// API.
///
/// These models have different capabilities and performance characteristics, allowing you to choose
/// the one that best suits your needs. See <https://platform.openai.com/docs/models> for more
/// information.
///
/// # Example
///
/// ```
/// use ai_chain_qwen::chatgpt::Model;
///
/// let turbo_model = Model::default();
/// let custom_model = Model::Other("your_custom_model_name".to_string());
/// ```

#[derive(EnumIter,Debug,Default, Clone, Serialize, Deserialize, EnumString, PartialEq, Eq)]
#[non_exhaustive]
pub enum Model {
    /// A high-performance and versatile model from the "moonshot" series.
    #[strum(serialize = "qwen-turbo")]
    #[default]
    QwenTurbo,
    #[strum(serialize = "qwen-long")]
    QwenLong,
    #[strum(serialize = "qwen-max")]
    QwenMax,
    #[strum(serialize = "qwen-max-longcontext")]
    QwenMax30k,
    #[strum(serialize = "qwen-plus")]
    QwenPlus,


    /// A variant that allows you to specify a custom model name as a string,
    /// in case new models are introduced or you have access to specialized models.
    #[strum(default)]
    Other(String),
}
impl Model {

}

/// The `Model` enum implements the `ToString` trait, allowing you to easily convert it to a string.
impl ToString for Model {
    fn to_string(&self) -> String {
        match &self {
            Model::QwenTurbo => "qwen-turbo".to_string(),
            Model::QwenLong => "qwen-long".to_string(),
            Model::QwenMax => "qwen-max".to_string(),
            Model::QwenPlus => "qwen-plus".to_string(),
            Model::QwenMax30k => "qwen-max-longcontext".to_string(),
            Model::Other(model) => model.to_string(),
        }
    }
}



/// Conversion from Model to ModelRef
impl From<Model> for ModelRef {
    fn from(value: Model) -> Self {
        ModelRef::from_model_name(value.to_string())
    }
}

/// Conversion from Model to Option
impl From<Model> for Opt {
    fn from(value: Model) -> Self {
        Opt::Model(value.into())
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use strum::IntoEnumIterator;

    use super::*;

    // Tests for FromStr
    #[test]
    fn test_from_str() -> Result<(), Box<dyn std::error::Error>> {
        // Model::iter()
        let model_names =  Model::iter().map(|model| model.to_string()).collect::<Vec<String>>();
        println!("{:?}", model_names);
        Ok(())
    }

    // Test ToString
    #[test]
    fn test_to_string() {

    }

    #[test]
    #[allow(deprecated)]
    fn test_to_string_deprecated() {

    }
}
