use anyhow::{anyhow, Result};
use futures_util::TryStreamExt;
use reqwest::{header::HeaderMap, Body, Client};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::path::Path;

use tokio::fs::File as TokioFile;
use tokio_util::codec::{BytesCodec, FramedRead};

use super::FileID;
use crate::print_;
use crate::{
    consts::BASE_BETA_URL,
    models::get_data,
    utils::{delete_api, get_api},
};

#[derive(Serialize, Deserialize, Debug)]
pub struct AgentFile {
    pub id: FileID,
    pub object: String,   // "file",
    pub bytes: u32,       // 120000,
    pub created_at: u32,  // Timestamp.. 1677610602,
    pub filename: String, // "salesOverview.pdf",
    pub purpose: FilePurpose,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum FilePurpose {
    #[serde(rename = "fine-tune")]
    FineTune,
    #[serde(rename = "fine-tune-results")]
    FineTuneResults,
    #[serde(rename = "assistants")]
    Assistants,
    #[serde(rename = "assistants-output")]
    AssistantsOutput,
}

impl FilePurpose {
    pub fn from_str(file_purpose: &str) -> Result<Self> {
        serde_json::from_str(file_purpose).map_err(Into::into)
    }

    pub fn as_str(&self) -> Result<String> {
        serde_plain::to_string(self).map_err(Into::into)
    }
}

impl AgentFile {
    pub async fn list_files(client: &Client, headers: &HeaderMap) -> Result<Vec<AgentFile>> {
        let response_body = get_api(client, headers, "files", None).await?;

        let files_json = get_data(&response_body)?;
        let files: Vec<AgentFile> = serde_json::from_value(files_json.clone())?;

        print_!("Files: {:?}", files);

        Ok(files)
    }

    pub async fn upload_file(
        client: &Client,
        headers: &HeaderMap,
        file_path: &Path,
        purpose: FilePurpose,
    ) -> Result<AgentFile> {
        let file_json = upload_file(client, headers, file_path, &purpose.as_str()?).await?;

        serde_json::from_value(file_json.clone()).map_err(Into::into)
    }

    pub async fn delete_file(client: &Client, headers: &HeaderMap, file_id: &str) -> Result<Value> {
        let payload = json!({
            "file_id": file_id,
        });

        let response_body =
            delete_api(client, headers, &format!("files/{}", file_id), &payload).await?;

        Ok(response_body)
    }

    pub async fn retrieve_file(
        client: &Client,
        headers: &HeaderMap,
        file_id: &str,
    ) -> Result<AgentFile> {
        let payload = json!({
            "file_id": file_id,
        });

        let response_body = get_api(
            client,
            headers,
            &format!("files/{}", file_id),
            Some(&payload),
        )
        .await?;

        let file_data = get_data(&response_body)?;

        let file: AgentFile = serde_json::from_value(file_data.clone())?;

        print_!("File details: {:?}", file);

        Ok(file)
    }

    pub async fn retrieve_file_content(
        client: &Client,
        headers: &HeaderMap,
        file_id: &str,
    ) -> Result<String> {
        let payload = json!({
            "file_id": file_id,
        });

        let route = &format!("files/{}/content", BASE_BETA_URL);

        let response = client
            .get(route)
            .headers(headers.clone())
            .json(&payload)
            .send()
            .await?;

        if response.status().is_success() {
            let file_string = response.text().await?;

            // let json_value = response.json::<Value>().await?;
            print_!("File content: {:?}", file_string);

            Ok(file_string)
        } else {
            print_!("API Err on route {}", route);
            // If not successful, perhaps you want to parse it differently or handle the error
            Err(anyhow!(response.status()))
        }
    }
}

pub async fn upload_file(
    client: &Client,
    headers: &HeaderMap,
    file_path: &Path,
    purpose: &str,
) -> Result<Value> {
    // Open file asynchronously
    let file = TokioFile::open(file_path).await?;
    let filename = file_path.file_name().unwrap().to_str().unwrap();

    let reader = FramedRead::new(file, BytesCodec::new());
    let stream = reader.map_ok(|bytes| bytes.freeze());

    let part =
        reqwest::multipart::Part::stream(Body::wrap_stream(stream)).file_name(filename.to_owned());

    let form = reqwest::multipart::Form::new()
        .text("purpose", purpose.to_string())
        .part("file", part);

    // Create and send the request
    let response = client
        .post("https://api.openai.com/v1/files")
        .headers(headers.clone())
        .multipart(form)
        .send()
        .await?;

    if response.status().is_success() {
        let json_value = response.json::<Value>().await?;
        print_!("JSON Body: {:?}", json_value);

        Ok(json_value)
    } else {
        let status = response.status();
        eprintln!("API Error with status {} on file upload.", status);

        if let Ok(text) = response.text().await {
            eprintln!("Error details: {}", text);
        }

        // If not successful, perhaps you want to parse it differently or handle the error
        Err(anyhow!(status))
    }
}
