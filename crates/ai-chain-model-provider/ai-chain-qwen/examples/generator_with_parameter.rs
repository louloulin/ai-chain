use std::env;
use ai_chain::{executor, parameters, prompt, step::Step};
use ai_chain::options::{ModelRef, Opt, Options};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new ChatGPT executor
    env::set_var("OPENAI_API_KEY", "sk-16519115bd424029ad9477a612bf5a9d");
    // env::set_var("OPENAI_API_BASE_URL", "https://api.moonshot.cn/v1");
    // let mut builder = Options::builder();
    // builder.add_option(Opt::Model(ModelRef::from_model_name("moonshot-v1-32k")));
    // let option = builder.build();

    let exec = executor!(qwen)?;

    // Create our step containing our prompt template
    let step = Step::for_prompt_template(prompt!(
        "You are a bot for making personalized greetings",
        "Make a personalized greeting tweet for {{text}}" // Text is the default parameter name, but you can use whatever you want
    ));

    // A greeting for emil!
    let res = step.run(&parameters!("Emil"), &exec).await?;
    println!("{}", res.to_immediate().await?.as_content());

    // A greeting for you
    let res = step.run(&parameters!("Your Name Here"), &exec).await?;

    println!("{}", res.to_immediate().await?.as_content());

    Ok(())
}
