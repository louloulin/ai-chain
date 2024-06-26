//! # Tokens Module
//!
//! This module provides utilities for managing tokens in Language Learning Models (LLMs),
//! primarily focusing on measuring the sizes of prompts. This is useful for ensuring that
//! prompts stay within the context window size supported by a given model.

use crate::step::Step;
use crate::{traits, Parameters};
use serde::{Deserialize, Serialize};
use std::cmp::max;
use thiserror::Error;

/// Custom error type for handling prompt token-related errors.
#[derive(Clone, Debug, Error)]
pub enum PromptTokensError {
    /// Indicates that prompt tokens are not accessible for the given step.
    #[error("The prompt tokens are not accessible for this type of step.")]
    NotAvailable,
    /// Indicates that the prompt tokens could not be computed.
    #[error("The prompt tokens could not be computed.")]
    UnableToCompute,
    /// Indicates that the prompt tokens could not be computed because formatting the prompt failed.
    #[error("Formatting prompt failed: {0}")]
    PromptFormatFailed(#[from] crate::prompt::StringTemplateError),
    #[error("Tokenizer error: {0}")]
    TokenizerError(#[from] crate::tokens::TokenizerError),
}

/// An extension trait for the `Executor` trait that provides additional methods for working
/// with token counts.
pub trait ExecutorTokenCountExt: traits::Executor {
    /// Splits a `Parameters` object into multiple smaller `Parameters` objects that fit within
    /// the context window size supported by the given model.
    ///
    /// # Arguments
    /// * `step` - The step that will process the Parameters. Has impact on tokenizer & text splitter used
    /// * `doc` - The parameter object to split into multiple, smaller, parameters
    /// * `chunk_overlap` - The amount of tokens each split part should overlap with previous & next chunk
    ///
    /// # Errors
    ///
    /// Returns a `PromptTokensError` if there is an issue computing the tokens.
    fn split_to_fit(
        &self,
        step: &Step,
        doc: &Parameters,
        base_parameters: &Parameters,
        chunk_overlap: Option<usize>,
    ) -> Result<Vec<Parameters>, PromptTokensError> {
        let splitter = self
            .get_tokenizer(step.options())
            .map_err(|_e| PromptTokensError::UnableToCompute)?;

        let text = doc.get_text().ok_or(PromptTokensError::UnableToCompute)?;

        let prompt = step.format(&base_parameters.combine(&Parameters::new_with_text("")))?;
        let tokens_used = self.tokens_used(step.options(), &prompt)?;
        let chunk_overlap = chunk_overlap.unwrap_or(0);

        let split_params = splitter
            .split_text(
                &text,
                tokens_used.max_tokens as usize - tokens_used.tokens_used as usize,
                chunk_overlap,
            )
            .map_err(|_e| PromptTokensError::UnableToCompute)?
            .into_iter()
            .map(Parameters::new_with_text)
            .collect();
        Ok(split_params)
    }
}

/// Blanket implementation of ExecutorTokenCountExt for all Executors
impl<E: traits::Executor> ExecutorTokenCountExt for E {}

/// Struct representing token count information, including the maximum tokens allowed and the
/// total number of tokens used.
pub struct TokenCount {
    /// The maximum number of tokens allowed.
    max_tokens: i32,
    /// The total number of tokens used.
    tokens_used: i32,
}
impl TokenCount {
    /// Creates a new `TokenCount` instance with the given maximum tokens and tokens used.
    ///
    /// # Arguments
    ///
    /// * `max_tokens` - The maximum number of tokens allowed.
    /// * `tokens_used` - The total number of tokens used.
    pub fn new(max_tokens: i32, tokens_used: i32) -> Self {
        Self {
            max_tokens,
            tokens_used,
        }
    }

    /// Returns the number of tokens that could be added to the context window.
    pub fn tokens_remaining(&self) -> i32 {
        self.max_tokens - self.tokens_used
    }

    /// Returns true if there is still room in the context window.
    pub fn has_tokens_remaining(&self) -> bool {
        self.has_room_for(1)
    }

    /// Returns true if there is room for the given number of tokens.
    ///
    /// # Arguments
    ///
    /// * `tokens` - The number of tokens to check if there is room for.
    ///
    /// # Examples
    ///
    /// ```
    /// use ai_chain::tokens::TokenCount;
    /// let token_count = TokenCount::new(100, 50);
    /// assert!(token_count.has_room_for(49));
    /// ```
    pub fn has_room_for(&self, tokens: i32) -> bool {
        self.tokens_remaining() >= tokens
    }
}

#[derive(Error, Debug, Clone)]
pub enum TokenizerError {
    #[error("Error tokenizing input text")]
    TokenizationError,
    #[error("Error stringifying tokens to text")]
    ToStringError,
    #[error("Error creating tokenizer")]
    TokenizerCreationError,
    #[error("Token Collection type mismatch")]
    TokenCollectionTypeMismatch,
}

pub trait Tokenizer {
    /// Tokenizes a string.
    ///
    /// # Parameters
    ///
    /// * `doc`: The string to tokenize.
    ///
    /// # Returns
    ///
    /// A `Result` containing a vector of tokens, or an error if there was a problem.
    fn tokenize_str(&self, doc: &str) -> Result<TokenCollection, TokenizerError>;

    /// Converts a vector of tokens into a string.
    ///
    /// # Parameters
    ///
    /// * `tokens`: The slice of tokens to convert.
    ///
    /// # Returns
    ///
    /// A `Result` containing a string, or an error if there was a problem.
    fn to_string(&self, tokens: TokenCollection) -> Result<String, TokenizerError>;

    fn split_text(
        &self,
        doc: &str,
        max_tokens_per_chunk: usize,
        chunk_overlap: usize,
    ) -> Result<Vec<String>, TokenizerError> {
        let tokens = self.tokenize_str(doc)?;
        let step_size = max(
            max_tokens_per_chunk.checked_sub(chunk_overlap).unwrap_or(1),
            1,
        );

        debug_assert_ne!(step_size, 0);

        (0..tokens.len())
            .step_by(step_size)
            .map(|start_idx| {
                let end_idx = usize::min(start_idx + max_tokens_per_chunk, tokens.len());
                self.to_string(tokens.slice(start_idx, end_idx))
            })
            .collect()
    }
}
/// Represents a single token.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(transparent)]
pub struct Token(TokenImpl);

#[derive(Serialize, Deserialize, Clone, Debug)]
enum TokenImpl {
    I32(i32),
    USize(usize),
}

impl From<i32> for Token {
    fn from(value: i32) -> Self {
        Token(TokenImpl::I32(value))
    }
}

impl From<usize> for Token {
    fn from(value: usize) -> Self {
        Token(TokenImpl::USize(value))
    }
}

impl Token {
    pub fn to_i32(&self) -> Option<i32> {
        match &self.0 {
            TokenImpl::I32(x) => Some(*x),
            _ => None,
        }
    }

    pub fn to_usize(&self) -> Option<usize> {
        match &self.0 {
            TokenImpl::USize(x) => Some(*x),
            _ => None,
        }
    }
}

/// A type-safe, enum-backed collection of tokens.
///
/// `TokenCollection` can hold a collection of `i32` or `usize` tokens,
/// ensuring type safety and efficient storage.
#[derive(Debug)]
pub struct TokenCollection(TokenCollectionImpl);

/// The internal enum representation of `TokenCollection`.
///
/// This enum holds the actual data for a `TokenCollection` instance,
/// allowing us to differentiate between the two types of collections
/// (`i32` and `usize`) in a type-safe manner.
#[derive(Debug)]
enum TokenCollectionImpl {
    /// A token collection of `i32`
    I32(Vec<i32>),
    /// A token collection of usize, this should be avoided as the size is non-determinate, but is present in some libraries.
    Usize(Vec<usize>),
}

impl TokenCollection {
    /// Converts the `TokenCollection` into a vector of `i32`,
    /// if it contains `i32` values. Returns `None` otherwise.
    pub fn as_i32(self) -> Result<Vec<i32>, TokenizerError> {
        match self.0 {
            TokenCollectionImpl::I32(v) => Ok(v),
            _ => Err(TokenizerError::TokenCollectionTypeMismatch),
        }
    }

    /// Converts the `TokenCollection` into a vector of `usize`,
    /// if it contains `usize` values. Returns `None` otherwise.
    pub fn as_usize(self) -> Result<Vec<usize>, TokenizerError> {
        match self.0 {
            TokenCollectionImpl::Usize(v) => Ok(v),
            _ => Err(TokenizerError::TokenCollectionTypeMismatch),
        }
    }

    /// Returns the number of tokens in the token collection
    pub fn len(&self) -> usize {
        match &self.0 {
            TokenCollectionImpl::I32(x) => x.len(),
            TokenCollectionImpl::Usize(x) => x.len(),
        }
    }

    /// Returns true if the TokenCollection is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Slices the token collection between start and end.
    pub fn slice(&self, start: usize, end: usize) -> Self {
        match &self.0 {
            TokenCollectionImpl::I32(v) => Vec::from(&v[start..end]).into(),
            TokenCollectionImpl::Usize(v) => Vec::from(&v[start..end]).into(),
        }
    }
}

/// Enables the conversion from a vector of `i32` into a `TokenCollection`.
impl From<Vec<i32>> for TokenCollection {
    fn from(v: Vec<i32>) -> Self {
        TokenCollection(TokenCollectionImpl::I32(v))
    }
}

/// Enables the conversion from a vector of `usize` into a `TokenCollection`.
impl From<Vec<usize>> for TokenCollection {
    fn from(v: Vec<usize>) -> Self {
        TokenCollection(TokenCollectionImpl::Usize(v))
    }
}
