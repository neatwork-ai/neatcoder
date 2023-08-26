use anyhow::Result;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::models::{interfaces::Interface, state::AppState};

pub async fn handle(app_state: Arc<RwLock<AppState>>, interface: Interface) -> Result<()> {
    let mut app_data = app_state.write().await;

    println!("[INFO] Adding Interface `{}``", interface.name());

    app_data.add_interface(interface)?;
    println!("[INFO] Interface added successfully");

    Ok(())
}
