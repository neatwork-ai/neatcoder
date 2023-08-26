use anyhow::Result;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::models::{interfaces::SchemaFile, state::AppState};

pub async fn handle(
    app_state: Arc<RwLock<AppState>>,
    interface_name: String,
    schema_name: String,
    schema: SchemaFile,
) -> Result<()> {
    let mut app_data = app_state.write().await;

    println!(
        "[INFO] Adding Schema `{}` to `{}`",
        schema_name, interface_name
    );
    app_data.add_schema(interface_name, schema_name, schema)?;
    println!("[INFO] Schema added successfully");

    Ok(())
}
