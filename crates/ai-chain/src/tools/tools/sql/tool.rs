use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::tools::{Describe, Tool, ToolDescription, ToolError};
use crate::tools::tools::sql::SQLDatabase;

// 假设 SQLDatabase 是之前定义的，并且已经实现了 Engine trait

#[derive(Debug, Error)]
pub enum SQLToolError {
    #[error(transparent)]
    YamlError(#[from] serde_yaml::Error),
    #[error("Database error: {0}")]
    DatabaseError(#[from] Box<dyn std::error::Error>),
}

impl ToolError for SQLToolError {}

#[derive(Serialize, Deserialize)]
pub struct SQLTool {
    db: SQLDatabase,
}

impl SQLTool {
    pub fn new(db: SQLDatabase) -> Self {
        Self { db }
    }
}

#[derive(Serialize, Deserialize)]
pub struct SQLQueryInput {
    pub query: String,
}

impl From<&str> for SQLQueryInput {
    fn from(value: &str) -> Self {
        Self {
            query: value.into(),
        }
    }
}

impl Describe for SQLQueryInput {
    fn describe() -> crate::tools::Format {
        vec![("query", "SQL query to execute").into()].into()
    }
}

#[derive(Serialize, Deserialize)]
pub struct SQLQueryOutput {
    pub result: String,
}

impl From<String> for SQLQueryOutput {
    fn from(value: String) -> Self {
        Self { result: value }
    }
}

impl Describe for SQLQueryOutput {
    fn describe() -> crate::tools::Format {
        vec![("result", "Result of the SQL query execution").into()].into()
    }
}

#[async_trait]
impl Tool for SQLTool {
    type Input = SQLQueryInput;
    type Output = SQLQueryOutput;
    type Error = SQLToolError;

    async fn invoke_typed(&self, input: &Self::Input) -> Result<Self::Output, Self::Error> {
        let result = self.db.query(&input.query).await?;
        Ok(result.into())
    }

    fn description(&self) -> ToolDescription {
        ToolDescription::new(
            "SQL Query",
            "Executes a given SQL query on the connected database.",
            "Use this to retrieve or manipulate data from the database.",
            SQLQueryInput::describe(),
            SQLQueryOutput::describe(),
        )
    }
}