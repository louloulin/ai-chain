use std::env;
use ai_chain::chains::sequential::Chain;
use ai_chain::serialization::StorableEntity;
use ai_chain::step::Step;
use ai_chain::traits::Executor as ExecutorTrait;
use ai_chain::{executor, parameters, prompt};


#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env::set_var("OPENAI_API_KEY", "sk-7LVW4lfKX3ZL01Iwuz8H0oZsUaLsEuO7ri9bfRKV36NrTE1A");
    // env::set_var("OPENAI_API_BASE_URL", "https://api.moonshot.cn/v1");
    // let mut builder = Options::builder();
    // builder.add_option(Opt::Model(ModelRef::from_model_name("moonshot-v1-32k")));
    // let option = builder.build();

    let exec = executor!(glm)?;
    let mut path = std::env::temp_dir();
    path.push("chain-from-yaml.yaml");
    let path = path.to_str().unwrap();

    let chain_to_write: Chain = Step::for_prompt_template(prompt!(
        "使用rust实现actor mailbox"
    ))
    .to_chain();
    chain_to_write.write_file_sync(path)?;
    println!("Wrote chain to {}", path);

    let chain = Chain::read_file_sync(path).unwrap();
    let res = chain.run(parameters!(), &exec).await.unwrap();
    println!("{}", res);
    Ok(())
}
