use anyhow::{anyhow, Result};
use gluon::serde::sql::{AsSql, SqlStatement};
use sqlparser::ast::Statement;
use std::fmt::Write;
use std::{
    fmt::{self},
    ops::{Deref, DerefMut},
};

#[derive(Debug)]
pub struct DataModel(Vec<Ddl>);

impl AsRef<Vec<Ddl>> for DataModel {
    fn as_ref(&self) -> &Vec<Ddl> {
        &self.0
    }
}

impl Deref for DataModel {
    type Target = Vec<Ddl>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for DataModel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug)]
pub struct Ddl(SqlStatement);

impl AsRef<SqlStatement> for Ddl {
    fn as_ref(&self) -> &SqlStatement {
        &self.0
    }
}

impl Deref for Ddl {
    type Target = SqlStatement;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Ddl {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl fmt::Display for DataModel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut writer = String::new();

        for schema in self.iter() {
            writeln!(writer, "{0}", schema)?;
        }
        write!(f, "{}", writer.trim_end_matches('\n'))
    }
}

impl fmt::Display for Ddl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Ddl {
    fn new(stmt: SqlStatement) -> Option<Self> {
        match stmt.stmt {
            Statement::CreateTable { .. } => Some(Ddl(stmt)),
            _ => None,
        }
    }

    pub fn name(&self) -> String {
        match &self.stmt {
            Statement::CreateTable { name, .. } => format!("{}", name),
            _ => panic!("Unexpected statement type"),
        }
    }
}

impl DataModel {
    pub fn from_sql(raw: &str) -> Result<DataModel> {
        let mut sql = raw.strip_sql().unwrap();

        if sql.len() == 0 {
            return Err(anyhow!("Expected at least one SQL statement."));
        }

        let mut schemas = vec![];

        // Assert that sql is a CREATE DDL statement
        for stmt in sql.drain(..) {
            if let Some(ddl) = Ddl::new(stmt) {
                schemas.push(ddl);
            }
        }

        Ok(Self(schemas))
    }
}

// pub fn generate_api_ideas(data_model: &DataModel) {}

mod tests {
    #[cfg(test)]
    use super::DataModel;
    #[cfg(test)]
    use anyhow::Result;

    #[test]
    fn test_from_sql() -> Result<()> {
        let sql_str = "CREATE TABLE Albums (
    -- An integer column that serves as the primary key. The AUTO_INCREMENT keyword means that each new album gets a unique ID that's one greater than the previous album's ID.
    AlbumID INT PRIMARY KEY AUTO_INCREMENT,
    -- A foreign key that references the ArtistID primary key in the Artists table. This creates a relationship between the two tables, linking each album to an artist.
    ArtistID INT,
    -- A variable-length string that can be up to 255 characters long. The NOT NULL constraint means that this column can't be empty.
    AlbumName VARCHAR(255) NOT NULL,
    -- A column to store the year when the album was released.
    ReleaseYear YEAR,
    FOREIGN KEY (ArtistID) REFERENCES Artists(ArtistID)
);";
        let data_model = DataModel::from_sql(sql_str)?;

        println!("{:?}", data_model);

        Ok(())
    }
}
