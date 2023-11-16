use anyhow::{anyhow, Result};
use reqwest::{header::HeaderMap, Client};
use serde_json::Value;

use crate::consts::BASE_BETA_URL;

pub async fn post_api(
    client: &Client,
    headers: &HeaderMap,
    route: &str,
    payload: &Value,
) -> Result<Value> {
    println!("CALLING: {}/{}", BASE_BETA_URL, route);
    println!("PAYLOAD: {}", payload);

    let response = client
        .post(&format!("{}/{}", BASE_BETA_URL, route))
        .headers(headers.clone())
        .json(payload)
        .send()
        .await?;

    if response.status().is_success() {
        let json_value = response.json::<Value>().await?;
        println!("JSON Body: {:?}", json_value);

        Ok(json_value)
    } else {
        println!("API Error on route {}", route);
        // If not successful, perhaps you want to parse it differently or handle the error
        Err(anyhow!(response.status()))
    }
}

pub async fn delete_api(
    client: &Client,
    headers: &HeaderMap,
    route: &str,
    payload: &Value,
) -> Result<Value> {
    println!("CALLING: {}/{}", BASE_BETA_URL, route);
    println!("PAYLOAD: {}", payload);

    let response = client
        .delete(&format!("{}/{}", BASE_BETA_URL, route))
        .headers(headers.clone())
        .json(payload)
        .send()
        .await?;

    if response.status().is_success() {
        let json_value = response.json::<Value>().await?;
        println!("JSON Body: {:?}", json_value);

        Ok(json_value)
    } else {
        println!("API Error on route {}", route);
        // If not successful, perhaps you want to parse it differently or handle the error
        Err(anyhow!(response.status()))
    }
}

pub async fn get_api(
    client: &Client,
    headers: &HeaderMap,
    route: &str,
    payload: Option<&Value>,
) -> Result<Value> {
    let req = client
        .get(&format!("{}/{}", BASE_BETA_URL, route))
        .headers(headers.clone());

    let response = if let Some(payload) = payload {
        req.json(payload).send().await?
    } else {
        req.send().await?
    };

    if response.status().is_success() {
        let json_value = response.json::<Value>().await?;
        println!("JSON Body: {:?}", json_value);

        Ok(json_value)
    } else {
        println!("API Err on route {}", route);
        // If not successful, perhaps you want to parse it differently or handle the error
        Err(anyhow!(response.status()))
    }
}

// #[macro_export]
// macro_rules! println {
//     ($($arg:tt)*) => {
//         {
//             use std::io::Write;

//             // Print to the console
//             println!($($arg)*);

//             let log_file_name = &$crate::LOG_FILE_NAME;

//             let mut file = std::fs::OpenOptions::new()
//                 .create(true)
//                 .append(true)
//                 .open(log_file_name.as_str())
//                 .unwrap();

//             writeln!(file, $($arg)*).unwrap();
//         }
//     };
// }
