use crate::{
    ai::openai::{
        client::OpenAI,
        input::{GptRole, Message},
    },
    utils::tasks::{Task, Tasks},
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
        Tasks::sample_json()?
    );

    // let output_schema = r#"
    //     Follow the instructions below when writing the list:
    //         - Skip any introduction or conclusion, only provide the list of items;
    //         - Provide a non-enumerated list of items
    //         - Use the symbol '-' at the start of each item
    //         - For each item, follow with a '-->' and assign who should work on the item.
    // "#;

    let msg = context.to_string() + task + output_schema.as_str();

    let user_msg = Message {
        role: GptRole::User,
        content: msg,
    };

    let resp = client.chat(&[sys_msg, user_msg], &[], &[]).await?;

    let json = resp.choices.first().unwrap().message.content.as_str();

    // Implement Fallback logic
    let task: Tasks = serde_json::from_str(json)?;

    println!("Tasks: {:?}", task);

    Ok(())
}
