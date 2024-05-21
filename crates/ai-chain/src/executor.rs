//! Utilities for working with executors
//!
/// A macro that creates a new executor for a specified model.
///
/// This macro makes it easy to create a new executor for a specific model without having to
/// directly call the constructor functions of the respective executor structs. The macro
/// supports creating executors for ChatGPT and LLaMA models.
///
/// # Usage
///
/// ```ignore
/// # use ai_chain::executor;
/// executor!(); // Creates a ChatGPT executor with default options.
/// ```
///
/// # Examples
///
/// ```ignore
/// // Create a ChatGPT executor with default options.
/// let chatgpt_executor = executor!();
///
/// // Create a ChatGPT executor with custom per-executor options.
/// let chatgpt_executor_with_options = executor!(chatgpt, per_executor_options);
///
/// // Create a ChatGPT executor with custom per-executor and per-invocation options.
/// let chatgpt_executor_with_both_options = executor!(chatgpt, per_executor_options, per_invocation_options);
///
/// // Create a LLaMA executor with default options.
/// let llama_executor = executor!(llama);
///
/// // Create a LLaMA executor with custom per-executor options.
/// let llama_executor_with_options = executor!(llama, per_executor_options);
///
/// // Create a LLaMA executor with custom per-executor and per-invocation options.
/// let llama_executor_with_both_options = executor!(llama, per_executor_options, per_invocation_options);
/// ```
///
/// # Parameters
///
/// - `()` or `chatgpt`: Creates a ChatGPT executor with default options.
/// - `chatgpt, per_executor_options`: Creates a ChatGPT executor with custom per-executor options.
/// - `chatgpt, per_executor_options, per_invocation_options`: Creates a ChatGPT executor with custom per-executor and per-invocation options.
/// - `llama`: Creates a LLaMA executor with default options.
/// - `llama, per_executor_options`: Creates a LLaMA executor with custom per-executor options.
/// - `llama, per_executor_options, per_invocation_options`: Creates a LLaMA executor with custom per-executor and per-invocation options.s
#[macro_export]
macro_rules! executor {
    () => {
        executor!(chatgpt)
    };
    (chatgpt) => {{
        use ai_chain::traits::Executor;
        ai_chain_openai::chatgpt::Executor::new()
    }};
    (chatgpt, $options:expr) => {{
        use ai_chain::traits::Executor;
        ai_chain_openai::chatgpt::Executor::new_with_options($options)
    }};
     (mooonshot) => {{
        use ai_chain::traits::Executor;
        ai_chain_moonshot::chatgpt::Executor::new()
    }};
    (mooonshot, $options:expr) => {{
        use ai_chain::traits::Executor;
        ai_chain_moonshot::chatgpt::Executor::new_with_options($options)
    }};
     (glm) => {{
        use ai_chain::traits::Executor;
        ai_chain_glm::chatgpt::Executor::new()
    }};
    (glm, $options:expr) => {{
        use ai_chain::traits::Executor;
        ai_chain_glm::chatgpt::Executor::new_with_options($options)
    }};
    (gemma, $options:expr) => {{
        use ai_chain::traits::Executor;
        ai_chain_gemma::Executor::new_with_options($options)
    }};
    (llama) => {{
        use ai_chain::traits::Executor;
        ai_chain_llama::Executor::new()
    }};
    (llama, $options:expr) => {{
        use ai_chain::traits::Executor;
        ai_chain_llama::Executor::new_with_options($options)
    }};
    (local, $options:expr) => {{
        use ai_chain::traits::Executor;
        ai_chain_local::Executor::new_with_options($options)
    }};
    (mock) => {{
        use ai_chain::traits::Executor;
        ai_chain_mock::Executor::new()
    }};
    (sagemaker_endpoint, $options:expr) => {{
        use ai_chain::traits::Executor;
        ai_chain_sagemaker_endpoint::Executor::new_with_options($options)
    }};
    (deno, $options:expr) => {{
    use ai_chain::traits::Executor;
    deno_executor::Executor::new_with_options($options)
    }};
    (wasmtime, $options:expr) => {{
    use ai_chain::traits::Executor;
    wasmtime_executor::Executor::new_with_options($options)
    }};
    (custom_model, $model:expr, $options:expr) => {{
    use ai_chain::traits::Executor;
    $model::Executor::new_with_options($options)
    }};
}
