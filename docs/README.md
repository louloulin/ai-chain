# ai-chain 🚀
> ai-chain fork llm-chain with extensions

`ai-chain` is a collection of Rust crates designed to help you create advanced LLM applications such as chatbots, agents, and more. As a comprehensive LLM-Ops platform we have strong support for both cloud and locally-hosted LLMs. We also provide robust support for prompt templates and chaining together prompts in multi-step chains, enabling complex tasks that LLMs can't handle in a single step. We also provide vector store integrations making it easy to give your model long-term memory and subject matter knowledge. This empowers you to build sophisticated applications.


### roadmap



### changelog 2024-05-22 0.14.4
* fix ai-chain executor costume bug
* add qwen model support
* other bug fix

### changelog 2024-05-21 0.14.3
* improve openai-compatible api
* add glm llm
* upgrade moonshot llm


## Examples 💡

To help you get started, here is an example demonstrating how to use `ai-chain`. You can find more examples in the [examples folder](/crates/ai-chain-model-provider/ai-chain-openai/examples) in the repository.

```rust
let exec = executor!()?;
let res = prompt!(
    "You are a robot assistant for making personalized greetings",
    "Make a personalized greeting for Joe"
)
.run(parameters()!, &exec)
.await?;
println!("{}", res);
```

[➡️ **tutorial: get started with ai-chain**](https://github.com/godlinchong/ai-chain/docs/getting-started-tutorial/index)
[➡️ **quick-start**: Create project based on our template](https://github.com/godlinchong/ai-chain-template/generate)

## Features 🌟

- **Prompt templates**: Create reusable and easily customizable prompt templates for consistent and structured interactions with LLMs.
- **Chains**: Build powerful chains of prompts that allow you to execute more complex tasks, step by step, leveraging the full potential of LLMs.
- **ChatGPT support**: Supports ChatGPT models, with plans to add OpenAI's other models in the future.
- **LLaMa support**: Provides seamless integration with LLaMa models, enabling natural language understanding and generation tasks with Facebook's research models.
- **Alpaca support**: Incorporates support for Stanford's Alpaca models, expanding the range of available language models for advanced AI applications.
- **`llm.rs` support**: Use llms in rust without dependencies on C++ code with our support for `llm.rs`
- **Tools**: Enhance your AI agents' capabilities by giving them access to various tools, such as running Bash commands, executing Python scripts, or performing web searches, enabling more complex and powerful interactions.
- **Extensibility**: Designed with extensibility in mind, making it easy to integrate additional LLMs as the ecosystem grows.
- **Community-driven**: We welcome and encourage contributions from the community to help improve and expand the capabilities of `ai-chain`.

## Getting Started 🚀

To start using `ai-chain`, add it as a dependency in your `Cargo.toml` (you need Rust 1.65.0 or newer):


ai-chain-openai

* cargo dependencies

```toml
[dependencies]
ai-chain = "0.14.2"
ai-chain-openai = "0.14.2"
```

* coding

```rust
env::set_var("OPENAI_API_KEY", "sk-7LVW4lfKX3ZL01Iwuz8H0oZsUaLsEuO7ri9bfRKV36NrTE1A");
let exec = executor!()?;
let res = prompt!(
"You are a robot assistant for making personalized greetings",
"Make a personalized greeting for Joe"
)
.run(parameters()!, &exec)
.await?;
println!("{}", res);
```




ai-chain-moonshot

 * cargo dependencies

```toml
[dependencies]
ai-chain = "0.14.2"
ai-chain-moonshot = "0.14.2"
```

* coding

```rust
env::set_var("OPENAI_API_KEY", "sk-7LVW4lfKX3ZL01Iwuz8H0oZsUaLsEuO7ri9bfRKV36NrTE1A");
let exec = executor!(mooonshot)?;
let res = prompt!(
    "You are a robot assistant for making personalized greetings",
    "Make a personalized greeting for Joe"
).run(parameters()!, &exec)
.await?;
println!("{}", res);
```


ai-chain-glm


* cargo dependencies

```toml
[dependencies]
ai-chain = "0.14.2"
ai-chain-glm = "0.14.2"
```

* coding

```
env::set_var("OPENAI_API_KEY", "sk-7LVW4lfKX3ZL01Iwuz8H0oZsUaLsEuO7ri9bfRKV36NrTE1A");
let exec = executor!(glm)?;
let res = prompt!(
    "You are a robot assistant for making personalized greetings",
    "Make a personalized greeting for Joe"
).run(parameters()!, &exec)
.await?;
println!("{}", res);
```


ai-chain-qwen


* cargo dependencies

```toml
[dependencies]
ai-chain = "0.14.2"
ai-chain-qwen = "0.14.2"
```

* coding

```rust
env::set_var("OPENAI_API_KEY", "sk-7LVW4lfKX3ZL01Iwuz8H0oZsUaLsEuO7ri9bfRKV36NrTE1A");
let exec = executor!(qwen)?;
let res = prompt!(
    "You are a robot assistant for making personalized greetings",
    "Make a personalized greeting for Joe"
)
.run(parameters()!, &exec)
.await?;
println!("{}", res);

```

The examples for `ai-chain-openai` or `ai-chain-moonshot` or others llms require you to set the `OPENAI_API_KEY` environment variable which you can do like this:

```bash
export OPENAI_API_KEY="sk-YOUR_OPEN_AI_KEY_HERE"

```

* support custom llm

```rust
env::set_var("OPENAI_API_KEY", "sk-7LVW4lfKX3ZL01Iwuz8H0oZsUaLsEuO7ri9bfRKV36NrTE1A");
let exec = executor!(custom,ai_chain_qwen)?;
let res = prompt!(
    "You are a robot assistant for making personalized greetings",
    "Make a personalized greeting for Joe"
)
.run(parameters()!, &exec)
.await?;
println!("{}", res);
```

## Contributing 🤝

**We warmly welcome contributions from everyone!** If you're interested in helping improve `ai-chain`, please check out our [`CONTRIBUTING.md`](/docs/CONTRIBUTING.md) file for guidelines and best practices.

## License 📄

`ai-chain` is licensed under the [MIT License](/LICENSE).

## Connect with Us 🌐