use crate::ai::openai::{
    client::OpenAI,
    job::OpenAIJob,
    msg::{GptRole, OpenAIMsg},
};
use anyhow::Result;

pub async fn gen_project_scaffold(
    client: &OpenAI,
    job: &OpenAIJob,
    api_description: &str,
) -> Result<String> {
    let mut prompts = Vec::new();

    prompts.push(OpenAIMsg {
        role: GptRole::System,
        content: String::from(
            "You are a software engineer who is specialised in building APIs in Rust.",
        ),
    });

    let main_prompt = format!("
You are a Rust engineer tasked with creating an API in Rust based on the following project description:\n{}\n
The API should retrieve the relevant data from a MySQL database.

Based on the information provided write the project's folder structure, starting from `src`.

Answer in JSON format (Do not forget to start with ```json). For each file provide a brief description included in the json", api_description);

    prompts.push(OpenAIMsg {
        role: GptRole::User,
        content: main_prompt,
    });

    let prompts = prompts.iter().map(|x| x).collect::<Vec<&OpenAIMsg>>();

    let resp = client.chat(job, &prompts, &[], &[]).await?;
    let answer = resp.choices.first().unwrap().message.content.as_str();

    Ok(String::from(answer))
}

pub async fn gen_work_schedule(
    client: &OpenAI,
    job: &OpenAIJob,
    api_description: &str,
    data_model: &Vec<String>,
    project_scaffold: &str,
) -> Result<String> {
    let mut prompts = Vec::new();

    prompts.push(OpenAIMsg {
        role: GptRole::System,
        content: String::from(
            "You are a software engineer who is specialised in building APIs in Rust.",
        ),
    });

    for model in data_model.iter() {
        prompts.push(OpenAIMsg {
            role: GptRole::User,
            content: model.clone(),
        });
    }

    prompts.push(OpenAIMsg {
        role: GptRole::User,
        content: api_description.to_string(),
    });

    let main_prompt = format!("
You are a Rust engineer tasked with creating an API in Rust.
You are assigned to build the API based on the project folder structure. Your current task is to order the files in accordance to the order of work that best fits the file dependencies.
The project scaffold is the following:\n{}\n

Answer in JSON format (Do not forget to start with ```json). Define the order by adding the file names to an ordered list.
Use the following schema:

{{'order': [...]}}
", project_scaffold);

    prompts.push(OpenAIMsg {
        role: GptRole::User,
        content: main_prompt,
    });

    let prompts = prompts.iter().map(|x| x).collect::<Vec<&OpenAIMsg>>();

    let resp = client.chat(job, &prompts, &[], &[]).await?;
    let answer = resp.choices.first().unwrap().message.content.as_str();

    println!("{}", answer);

    Ok(String::from(answer))
}

pub async fn gen_code(
    client: &OpenAI,
    job: &OpenAIJob,
    api_description: String,
    data_model: &Vec<String>,
    project_scaffold: String,
    prior_code: &Vec<String>,
    filename: &str,
) -> Result<String> {
    let mut prompts = Vec::new();

    prompts.push(OpenAIMsg {
        role: GptRole::System,
        content: String::from(
            "You are a software engineer who is specialised in building APIs in Rust.",
        ),
    });

    for model in data_model.iter() {
        prompts.push(OpenAIMsg {
            role: GptRole::User,
            content: model.clone(),
        });
    }

    prompts.push(OpenAIMsg {
        role: GptRole::User,
        content: api_description,
    });

    for code in prior_code.iter() {
        prompts.push(OpenAIMsg {
            role: GptRole::User,
            content: code.clone(),
        });
    }

    prompts.push(OpenAIMsg {
        role: GptRole::User,
        content: project_scaffold,
    });

    let main_prompt = format!(
        "
You are a Rust engineer tasked with creating an API in Rust.
You are assigned to build the API based on the project folder structure
Your current task is to write the module `{}.rs
",
        filename
    );

    prompts.push(OpenAIMsg {
        role: GptRole::User,
        content: main_prompt,
    });

    let prompts = prompts.iter().map(|x| x).collect::<Vec<&OpenAIMsg>>();

    let resp = client.chat(job, &prompts, &[], &[]).await?;
    let answer = resp.choices.first().unwrap().message.content.as_str();

    println!("{}", answer);

    Ok(String::from(answer))
}
