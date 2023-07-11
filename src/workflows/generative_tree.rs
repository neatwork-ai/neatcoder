use crate::{
    ai::openai::{
        client::OpenAI,
        input::{GptRole, Message},
    },
    utils::tasks::Tasks,
    Sample,
};
use anyhow::Result;

pub async fn generate_tree(client: &OpenAI) -> Result<()> {
    let sys_msg = Message {
        role: GptRole::System,
        content: String::from("You are a technical entrepreneur on steroids."),
    };

    let context = r#"
        You are creating a new startup called Promptify,
        whose objective is to create a marketplace for prompt templates.\n
    "#;

    let task = r#"
        List the items that your team will have to work on in 
        order to build the product\n
    "#;

    let output_schema = format!(
        "Use the following schema as a format for your response: {}",
        serde_json::to_string(&Tasks::sample())?
    );

    // let output_schema = r#"Format the output in csv format with two columns:
    // - Item (i.e. Item to work on)
    // - Role (i.e. Person/Role to work on it)"#;

    let msg = context.to_string() + task + output_schema.as_str();

    let user_msg = Message {
        role: GptRole::User,
        content: msg,
    };

    let resp = client.chat(&[sys_msg, user_msg], &[], &[]).await?;

    let json = resp.choices.first().unwrap().message.content.as_str();
    println!("{}", json);

    // Implement Fallback logic
    let task: Tasks = serde_json::from_str(json)?;

    // println!("Tasks: {:?}", task);

    Ok(())
}
