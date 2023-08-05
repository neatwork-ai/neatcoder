use anyhow::{anyhow, Result};
use reqwest;
use serde_json::Value;

static AGENT: &'static str = "MyBot/1.0 (me@example.com)";

pub async fn get_lib(lib: &str) -> Result<()> {
    // Construct the URL to fetch the crate data
    let url = format!("https://crates.io/api/v1/crates/{}", lib);

    println!("Getting there..");
    // Make a GET request to the crates.io API
    // Create a client
    let client = reqwest::Client::new();

    let response = client
        .get(&url)
        .header(reqwest::header::USER_AGENT, AGENT)
        .send()
        .await?
        .text()
        .await?;

    // Parse the JSON response
    let v: Value = serde_json::from_str(&response)?;

    // Print the versions
    let versions = v["versions"].as_array();
    println!("{:?}", versions);

    if versions.is_none() {
        return Err(anyhow!(format!(
            "Failed to fetch versions for crate {}",
            lib
        )));
    }

    Ok(())
}
