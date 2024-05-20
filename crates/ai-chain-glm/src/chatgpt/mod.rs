//! This module implements chains for the ChatGPT model from OpenAI.
mod executor;
mod model;

pub use executor::{Error, Executor};
pub use model::Model;
