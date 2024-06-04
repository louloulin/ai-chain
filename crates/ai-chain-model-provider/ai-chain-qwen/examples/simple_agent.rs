use std::env;
use ai_chain::{executor, prompt};
use async_trait::async_trait;

use ai_chain::tools::{Tool, ToolCollection, ToolDescription, ToolError};

use ai_chain::multitool;
use ai_chain::tools::tools::{
    BashTool, BashToolError, BashToolInput, BashToolOutput, ExitTool, ExitToolError, ExitToolInput,
    ExitToolOutput,
};
use ai_chain::traits::Executor;
use ai_chain::parameters;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use ai_chain::prompt::{ChatMessage, ChatRole, ConversationTemplate, StringTemplate};
use ai_chain::step::Step;

// A simple example generating a prompt with some tools.
multitool!(
    MyMultitool,
    MyMultiToolInput,
    MyMultiToolOutput,
    MyMultitoolError,
    BashTool,
    BashToolInput,
    BashToolOutput,
    BashToolError,
    ExitTool,
    ExitToolInput,
    ExitToolOutput,
    ExitToolError
);

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Create a new ChatGPT executor with the default settings
    env::set_var("OPENAI_API_KEY", "sk-16519115bd424029ad9477a612bf5a9d");

    let exec = executor!(qwen)?;
    let mut tool_collection = ToolCollection::<MyMultitool>::new();
    tool_collection.add_tool(BashTool::new().into());
    tool_collection.add_tool(ExitTool::new().into());
    let tool_prompt = tool_collection.to_prompt_template().unwrap();
    let template = StringTemplate::combine(vec![
        tool_prompt,
        StringTemplate::tera("You may ONLY use one tool at a time. Please perform the following task: {{task}}. Once you have read the IP Address you may trigger ExitTool. -- Do not do this before you know the ip address. do not ask for more tasks."),
    ]);
    let mut prompt = ConversationTemplate::new()
        .with_system_template(
            "You are an automated agent for performing tasks. Your output must always be YAML.",
        )
        .with_user(template);
    let task = "Figure out my IP address";
    for _ in 1..5 {
        let result = Step::for_prompt_template(prompt.clone().into())
            .run(&parameters!("task" => task), &exec)
            .await?;
        let msg = result
            .to_immediate()
            .await?
            .primary_textual_output()
            .unwrap();
        println!("{}", &msg);
        match tool_collection
            .process_chat_input(
                &msg
            )
            .await
        {
            Ok(output) => {
                println!("{}", &output);
                 prompt = ConversationTemplate::new().with_user_template(msg.as_str()).with_user_template(&output);
            }
            Err(e) => println!("Error: {}", e),
        }
    }

    Ok(())
}
