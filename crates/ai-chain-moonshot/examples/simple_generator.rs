use std::env;
use ai_chain::{executor, parameters, prompt};
use ai_chain::options::{ModelRef, Opt, Options};

// Declare an async main function
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env::set_var("OPENAI_API_KEY", "sk-9azCvQDTh8pKfLft2dfHYoh0mtGFUvvdjA9LNUBRBWTjNZu4");
    let mut builder = Options::builder();
    builder.add_option(Opt::Model(ModelRef::from_model_name("moonshot-v1-8k")));
    let option = builder.build();
    // Create a new ChatGPT executor.
    let exec = executor!(mooonshot,option)?;
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
