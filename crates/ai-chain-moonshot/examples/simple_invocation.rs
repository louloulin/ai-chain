use std::env;
use ai_chain::executor;
use ai_chain::options::{ModelRef, Opt, Options};
use ai_chain::parameters;

use ai_chain::prompt::{ConversationTemplate, StringTemplate};
use ai_chain::step::Step;
use ai_chain::tools::tools::BashTool;
use ai_chain::tools::ToolCollection;
use ai_chain_moonshot::chatgpt;
// A simple example generating a prompt with some tools.

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env::set_var("OPENAI_API_KEY", "sk-7LVW4lfKX3ZL01Iwuz8H0oZsUaLsEuO7ri9bfRKV36NrTE1A");
    // env::set_var("OPENAI_API_BASE_URL", "https://api.moonshot.cn/v1");
    // let mut builder = Options::builder();
    // builder.add_option(Opt::Model(ModelRef::from_model_name("moonshot-v1-32k")));
    // let option = builder.build();

    let exec = executor!(mooonshot)?;

    let mut tool_collection = ToolCollection::new();
    tool_collection.add_tool(BashTool::new());

    let template = StringTemplate::combine(vec![
        tool_collection.to_prompt_template().unwrap(),
        StringTemplate::tera("Please perform the following task: {{task}}."),
    ]);

    let task = "Find the file GOAL.txt and tell me its content";

    let prompt = ConversationTemplate::new()
        .with_system_template(
            "You are an automated agent for performing tasks. Your output must always be YAML.",
        )
        .with_user(template);

    let result = Step::for_prompt_template(prompt.into())
        .run(&parameters!("task" => task), &exec)
        .await?;

    println!("{}", result);
    match tool_collection
        .process_chat_input(
            &result
                .to_immediate()
                .await?
                .primary_textual_output()
                .unwrap(),
        )
        .await
    {
        Ok(output) => println!("{}", output),
        Err(e) => println!("Error: {}", e),
    }
    Ok(())
}
