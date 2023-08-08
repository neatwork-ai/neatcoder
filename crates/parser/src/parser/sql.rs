use anyhow::anyhow;
use sqlparser::ast::Statement;
use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;
use std::fmt::Write;
use std::{
    fmt,
    ops::{Deref, DerefMut},
};

use super::AsFormat;
use crate::err::ParseError;

/// Trait providing methods for working with SQL code.
pub trait AsSql: AsFormat {
    /// Converts the object to an SQL syntax tree.
    fn as_sql(&self) -> Result<Sql, ParseError>;

    /// Strips the SQL formatting and returns the SQL syntax tree.
    fn strip_sql(&self) -> Result<Sql, ParseError>;

    /// Strips multiple SQL code blocks and returns them as a vector of `Sql` objects.
    fn strip_sqls(&self) -> Result<Vec<Sql>, ParseError>;
}

impl<'a> AsSql for &'a str {
    /// Implementation of converting a string slice to an SQL syntax tree.
    fn as_sql(&self) -> Result<Sql, ParseError> {
        self.as_format(deserialize_sql)
    }

    /// Implementation of stripping SQL code from a string slice.
    fn strip_sql(&self) -> Result<Sql, ParseError> {
        self.strip_format(deserialize_sql, "sql")
    }

    /// Implementation of stripping multiple SQL code blocks from a string slice.
    fn strip_sqls(&self) -> Result<Vec<Sql>, ParseError> {
        self.strip_formats(deserialize_sql, "sql")
    }
}

/// Represents a collection of SQL statements.
#[derive(Debug)]
pub struct Sql(Vec<SqlStatement>);

/// Represents an individual SQL statement, including the raw text and parsed AST.
#[derive(Debug)]
pub struct SqlStatement {
    // TODO: If we make it such that `Statement` can be serialized back into a string
    // with correct identation and clean format, then we can consider removing `raw`
    /// Raw text of the SQL statement
    pub raw: String,
    /// Parsed AST representation
    pub stmt: Statement,
}

impl Sql {
    /// Attempts to convert `Sql` into a single `SqlStatement`. Returns an error if `Sql` is not a singleton.
    pub fn as_stmt(mut self) -> Result<SqlStatement, ParseError> {
        if self.len() != 1 {
            Err(ParseError::from(anyhow!(
                "Failed to convert `Sql` to `SqlStatement` as it's not singleton"
            )))
        } else {
            Ok(self.pop().unwrap())
        }
    }
}

impl AsRef<Vec<SqlStatement>> for Sql {
    fn as_ref(&self) -> &Vec<SqlStatement> {
        &self.0
    }
}

impl Deref for Sql {
    type Target = Vec<SqlStatement>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Sql {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl fmt::Display for Sql {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut writer = String::new();

        for statement in self.iter() {
            writeln!(writer, "{0}\n", statement)?;
        }
        write!(f, "{}", writer.trim_end_matches('\n'))
    }
}

impl fmt::Display for SqlStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.raw)
    }
}

/// Function to deserialize an SQL code string into an `Sql` object.
///
/// # Arguments
/// * `sql_str` - The SQL code string to be deserialized.
///
/// # Returns
/// * A `Result` containing an `Sql` object if successful, or a `ParseError` if an error occurred.
fn deserialize_sql(sql_str: &str) -> Result<Sql, ParseError> {
    // TODO: Support multiple dialects
    let dialect = GenericDialect {};

    let statements: Vec<String> = sql_str
        .split_terminator(";\n")
        .filter_map(|x| {
            if x.trim().is_empty() {
                None
            } else {
                Some(format!("{};\n", x.trim()))
            }
        })
        .collect();

    let mut sql_vec = Vec::new();

    for raw_stmt in statements.iter() {
        let mut syntax_tree = Parser::parse_sql(&dialect, raw_stmt).unwrap();

        if syntax_tree.len() > 1 {
            return Err(ParseError::from(anyhow!(
                "SQL Syntax Tree should contain only one statement as script was already divided by statements"
            )));
        }

        let stmt = syntax_tree.pop().unwrap();

        let sql_stmt = SqlStatement {
            raw: String::from(raw_stmt),
            stmt,
        };

        sql_vec.push(sql_stmt);
    }

    Ok(Sql(sql_vec))
}
