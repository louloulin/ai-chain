use ai_chain::traits::Executor;
use std::env;
use ai_chain::{executor, parameters, prompt};

// Declare an async main function
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env::set_var("OPENAI_API_KEY", "sk-16519115bd424029ad9477a612bf5a9d");
    // Create a new ChatGPT executor.
    let exec = executor!(custom, ai_chain_qwen)?;
    // Create our prompt...
    let res = prompt!(
        "你是一个rust专家和actor并发编程专家",
        "使用rust实现一个简单的actor模型，不要使用actix框架实现"
    )
    .run(&parameters!(), &exec) // ...and run it
    .await?;
    println!("{}", res);
    Ok(())
}
