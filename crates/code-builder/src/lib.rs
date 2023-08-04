use anyhow::Result;
use std::{
    fs::{read_dir, File},
    io::Read,
    path::Path,
};

use gluon::serde::sql::{AsSql, SqlStatement};

pub mod crates;
pub mod fs;
pub mod schema;
pub mod specs;

pub fn get_sql_statements(path: &Path) -> Result<Vec<SqlStatement>> {
    let mut sql_stmts = Vec::new();

    for entry in read_dir(Path::new(path))? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            let mut file = File::open(&path)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;

            let sql_stmt = contents.as_str().as_sql()?.as_stmt()?;

            sql_stmts.push(sql_stmt);
        }
    }

    Ok(sql_stmts)
}
