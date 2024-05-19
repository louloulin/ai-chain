//! This module implements chains for the ChatGPT model from OpenAI.
mod error;
mod model;
mod prompt;
mod config;
mod client;

pub use model::ModelTrait;
