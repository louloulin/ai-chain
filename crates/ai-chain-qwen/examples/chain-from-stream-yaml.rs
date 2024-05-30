use std::env;
use ai_chain::chains::sequential::Chain;
use ai_chain::serialization::StorableEntity;
use ai_chain::step::Step;
use ai_chain::traits::Executor as ExecutorTrait;
use ai_chain::{executor, options, parameters, prompt};
use ai_chain::options::{ModelRef, Opt, Options, options_from_env};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env::set_var("OPENAI_API_KEY", "sk-16519115bd424029ad9477a612bf5a9d");
    let flag: bool = true;
    env::set_var("ai_chain_STREAM", flag.to_string());
    let opts = options_from_env()?;
    let exec = executor!(qwen, opts)?;
    let mut path = env::temp_dir();
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
    println!("{}", res.to_immediate().await?);
    Ok(())
}
