use std::{collections::HashSet, error::Error};
use std::fmt::Display;

use async_trait::async_trait;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use ai_chain_types::log;

#[derive(Serialize, Deserialize)]
pub enum Dialect {
    #[serde(rename = "mysql")]
    MySQL,
    #[serde(rename = "sqlite")]
    SQLite,
    #[serde(rename = "postgresql")]
    PostgreSQL,
}
impl Display for Dialect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Dialect::MySQL => "mysql".to_string(),
            Dialect::SQLite => "sqlite".to_string(),
            Dialect::PostgreSQL => "postgresql".to_string(),
        };
        write!(f, "{}", str)
    }
}

#[async_trait]
pub trait Engine: Send + Sync {
    // Dialect returns the dialect(e.g. mysql, sqlite, postgre) of the database.
    fn dialect(&self) -> Dialect;
    // Query executes the query and returns the columns and results.
    async fn query(&self, query: &str) -> Result<(Vec<String>, Vec<Vec<String>>), Box<dyn Error>>;
    // TableNames returns all the table names of the database.
    async fn table_names(&self) -> Result<Vec<String>, Box<dyn Error>>;
    // TableInfo returns the table information of the database.
    // Typically, it returns the CREATE TABLE statement.
    async fn table_info(&self, tables: &str) -> Result<String, Box<dyn Error>>;
    // Close closes the database.
    fn close(&self) -> Result<(), Box<dyn Error>>;
}
#[derive(Serialize, Deserialize)]
pub struct SQLDatabase {
    pub engine: Box<dyn Engine>,
    pub sample_rows_number: i32,
    pub all_tables: HashSet<String>,
}

impl Serialize for Box<dyn Engine> {
    fn serialize<Ser>(&self, serializer: Ser) -> Result<Ser::Ok, Ser::Error>
        where
            Ser: Serializer,
    {
        // Implement serialization logic here
        // This might involve converting the Engine to a string or some other serializable form
        todo!()
    }
}

impl<'de> Deserialize<'de> for Box<dyn Engine> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        // Implement deserialization logic here
        // This might involve creating an Engine from a string or some other deserializable form
        todo!()
    }
}

#[derive(Serialize, Deserialize)]
pub struct SQLDatabaseBuilder {
    engine: Box<dyn Engine>,
    sample_rows_number: i32,
    ignore_tables: HashSet<String>,
}

impl SQLDatabaseBuilder {
    pub fn new<E>(engine: E) -> Self
    where
        E: Engine + 'static,
    {
        SQLDatabaseBuilder {
            engine: Box::new(engine),
            sample_rows_number: 3, // Default value
            ignore_tables: HashSet::new(),
        }
    }

    // Function to set custom number of sample rows
    pub fn custom_sample_rows_number(mut self, number: i32) -> Self {
        self.sample_rows_number = number;
        self
    }

    // Function to set tables to ignore
    pub fn ignore_tables(mut self, ignore_tables: HashSet<String>) -> Self {
        self.ignore_tables = ignore_tables;
        self
    }

    // Function to build the SQLDatabase instance
    pub async fn build(self) -> Result<SQLDatabase, Box<dyn Error>> {
        let table_names_result = self.engine.table_names().await;

        // Handle potential error from table_names call
        let table_names = match table_names_result {
            Ok(names) => names,
            Err(error) => {
                return Err(error);
            }
        };

        // Filter out ignored tables
        let all_tables: HashSet<String> = table_names
            .into_iter()
            .filter(|name| !self.ignore_tables.contains(name))
            .collect();

        Ok(SQLDatabase {
            engine: self.engine,
            sample_rows_number: self.sample_rows_number,
            all_tables,
        })
    }
}

impl SQLDatabase {
    pub fn dialect(&self) -> Dialect {
        self.engine.dialect()
    }

    pub fn table_names(&self) -> Vec<String> {
        self.all_tables.iter().cloned().collect()
    }

    pub async fn table_info(&self, tables: &[String]) -> Result<String, Box<dyn Error>> {
        let mut tables: HashSet<String> = tables.to_vec().into_iter().collect();
        if tables.len() == 0 {
            tables = self.all_tables.clone();
        }
        let mut info = String::new();
        for table in tables {
            let table_info = self.engine.table_info(&table).await?;
            info.push_str(&table_info);
            info.push_str("\n\n");

            if self.sample_rows_number > 0 {
                let sample_rows = self.sample_rows(&table).await?;
                info.push_str("/*\n");
                info.push_str(&sample_rows);
                info.push_str("*/ \n\n");
            }
        }
        Ok(info)
    }

    pub async fn query(&self, query: &str) -> Result<String, Box<dyn Error>> {
        log::debug!("Query: {}", query);
        let (cols, results) = self.engine.query(query).await?;
        let mut str = cols.join("\t") + "\n";
        for row in results {
            str += &row.join("\t");
            str.push('\n');
        }
        Ok(str)
    }

    pub fn close(&self) -> Result<(), Box<dyn Error>> {
        self.engine.close()
    }

    pub async fn sample_rows(&self, table: &str) -> Result<String, Box<dyn Error>> {
        let query = format!("SELECT * FROM {} LIMIT {}", table, self.sample_rows_number);
        log::debug!("Sample Rows Query: {}", query);
        self.query(&query).await
    }
}
