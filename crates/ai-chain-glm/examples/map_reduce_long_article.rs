use std::env;
use ai_chain::chains::map_reduce::Chain;
use ai_chain::step::Step;
use ai_chain::{executor, parameters, prompt, Parameters};
use ai_chain::options::{ModelRef, Opt, Options};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new ChatGPT executor with the default settings
    env::set_var("OPENAI_API_KEY", "sk-7LVW4lfKX3ZL01Iwuz8H0oZsUaLsEuO7ri9bfRKV36NrTE1A");
    // env::set_var("OPENAI_API_BASE_URL", "https://api.moonshot.cn/v1");
    // let mut builder = Options::builder();
    // builder.add_option(Opt::Model(ModelRef::from_model_name("moonshot-v1-32k")));
    // let option = builder.build();

    let exec = executor!(glm)?;

    // Create the "map" step to summarize an article into bullet points
    let map_prompt = Step::for_prompt_template(prompt!(
        "You are a bot for summarizing wikipedia articles, you are terse and focus on accuracy",
        "Summarize this article into bullet points:\n{{text}}"
    ));

    // Create the "reduce" step to combine multiple summaries into one
    let reduce_prompt = Step::for_prompt_template(prompt!(
        "You are a diligent bot that summarizes text",
        "Please combine the articles below into one summary as bullet points:\n{{text}}"
    ));

    // Create a map-reduce chain with the map and reduce steps
    let chain = Chain::new(map_prompt, reduce_prompt);

    // Load the content of the article to be summarized
    let article = include_str!("article_to_summarize.md");

    // Create a vector with the Parameters object containing the text of the article
    let docs = vec![parameters!(article)];

    // Run the chain with the provided documents and an empty Parameters object for the "reduce" step
    let res = chain.run(docs, Parameters::new(), &exec).await.unwrap();

    // Print the result to the console
    println!("{}", res);
    Ok(())
}
