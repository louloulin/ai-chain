//! This module implements chains for the ChatGPT model from OpenAI.
mod error;
mod executor;
mod model;
mod prompt;
mod Config;

pub use executor::*;
pub use model::Model;
