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
/// use ai_chain_glm::chatgpt::Model;
///
/// let turbo_model = Model::default();
/// let custom_model = Model::Other("your_custom_model_name".to_string());
/// ```

#[derive(EnumIter,Debug,Default, Clone, Serialize, Deserialize, EnumString, PartialEq, Eq)]
#[non_exhaustive]
pub enum Model {
    /// A high-performance and versatile model from the "moonshot" series.
    #[strum(serialize = "glm-4")]
    #[default]
    GLM4,
    #[strum(serialize = "glm-4v")]
    GLM4V,
    #[strum(serialize = "glm-3-turbo")]
    GLM3Turbo,


    // ... 你可以继续添加更多的 "moonshot" 模型 ...

    /// A variant that allows you to specify a custom model name as a string,
    /// in case new models are introduced or you have access to specialized models.
    #[strum(default)]
    Other(String),
}
impl Model {

}

impl ToString for Model {
    fn to_string(&self) -> String {
        match &self {
            Model::GLM4 => "glm-4".to_string(),
            Model::GLM4V => "glm-4v".to_string(),
            Model::GLM3Turbo => "glm-3-turbo".to_string(),

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
