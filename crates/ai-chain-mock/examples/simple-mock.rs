use ai_chain::executor;
use ai_chain::options::Options;
use std::{env::args, error::Error};

use ai_chain::{prompt::Data, traits::Executor};

extern crate ai_chain_mock;

/// This example demonstrates how to use the ai-chain-mock crate to generate text using a mock model.
///
/// Usage: cargo run --release --package ai-chain-mock --example simple
///
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    let raw_args: Vec<String> = args().collect();
    let prompt = match &raw_args.len() {
        1 => "Rust is a cool programming language because",
        2 => raw_args[1].as_str(),
        _ => panic!("Usage: cargo run --release --example simple"),
    };

    let exec = executor!(mock)?;
    let res = exec
        .execute(Options::empty(), &Data::Text(String::from(prompt)))
        .await?;

    println!("{}", res);
    Ok(())
}
