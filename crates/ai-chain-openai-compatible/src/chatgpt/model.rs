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
/// ```
///
///
///
pub trait ModelTrait  {
    /// 获取模型的名称
    fn name(&self) -> String;

    /// 获取模型的描述
    fn description(&self) -> String;
}

