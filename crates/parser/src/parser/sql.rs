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

pub trait AsSql: AsFormat {
    fn as_sql(&self) -> Result<Sql, ParseError>;
    fn strip_sql(&self) -> Result<Sql, ParseError>;
    fn strip_sqls(&self) -> Result<Vec<Sql>, ParseError>;
}

impl<'a> AsSql for &'a str {
    fn as_sql(&self) -> Result<Sql, ParseError> {
        self.as_format(deserialize_sql)
    }

    fn strip_sql(&self) -> Result<Sql, ParseError> {
        self.strip_format(deserialize_sql, "sql")
    }

    fn strip_sqls(&self) -> Result<Vec<Sql>, ParseError> {
        self.strip_formats(deserialize_sql, "sql")
    }
}

#[derive(Debug)]
pub struct Sql(Vec<SqlStatement>);

#[derive(Debug)]
pub struct SqlStatement {
    // If we make it such that `Statement` can be serialized back into a string
    // with correct identation and clean format, then we can consider removing `raw`
    pub raw: String,
    pub stmt: Statement,
}

impl Sql {
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
