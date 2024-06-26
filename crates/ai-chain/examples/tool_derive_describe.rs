use ai_chain::tools::Describe;
use ai_chain::tools::{Format, FormatPart};
use ai_chain_macros::Describe;

#[derive(Describe)]
struct MyToolInput {
    #[purpose("Person's name")]
    #[allow(dead_code)]
    name: String,

    #[purpose("Person's age")]
    #[allow(dead_code)]
    age: u8,
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    println!("{:#?}", MyToolInput::describe());
}
