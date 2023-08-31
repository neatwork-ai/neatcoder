use crate::models::state::AppState;
use crate::prelude::*;
use std::sync::Arc;
use tokio::sync::RwLock;

pub async fn handle(dot_neat: &mut AppState, update_with: AppState) {
    if dot_neat.specs.is_some() {
        warn!("Initializing .neat even though specs already exist");
    }

    let AppState {
        specs,
        scaffold,
        interfaces,
        codebase,
        jobs,
    } = update_with;

    dot_neat.specs = specs;
    dot_neat.scaffold = scaffold;
    dot_neat.interfaces = interfaces;
    dot_neat.codebase = codebase;
    dot_neat.jobs = jobs;
}
