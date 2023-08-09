use crate::{
    ai::openai::{
        client::OpenAI,
        job::OpenAIJob,
        msg::{GptRole, OpenAIMsg},
    },
    output::tasks::Tasks,
    Sample,
};
use anyhow::Result;

pub async fn generate_tree(client: &OpenAI, job: &OpenAIJob) -> Result<()> {
    let sys_msg = OpenAIMsg {
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

    let user_msg = OpenAIMsg {
        role: GptRole::User,
        content: msg,
    };

    let resp = client
        .chat_raw(job, &[&sys_msg, &user_msg], &[], &[])
        .await?;
    let json = resp.choices.first().unwrap().message.content.as_str();
    println!("{}", json);

    // Implement Fallback logic
    let tasks: Tasks = serde_json::from_str(json)?;

    println!("Tasks: {:?}", tasks);

    distribute_tasks(
        client,
        job,
        &tasks,
        "
    You are part of a group of people creating a new startup called Promptify,
    whose objective is to create a marketplace for prompt templates.\n
    ",
    )
    .await?;

    Ok(())
}

pub async fn distribute_tasks(
    client: &OpenAI,
    job: &OpenAIJob,
    tasks: &Tasks,
    context: &str,
) -> Result<()> {
    for (_idx, task) in tasks.iter() {
        let role = format!("You work in {}", task.role.as_ref().unwrap());

        let sys_msg = OpenAIMsg {
            role: GptRole::System,
            content: role,
        };

        let msg = format!(
            "{} \nDevelop the the following item: {}",
            context, task.task
        );

        let user_msg = OpenAIMsg {
            role: GptRole::User,
            content: msg,
        };

        let resp = client
            .chat_raw(job, &[&sys_msg, &user_msg], &[], &[])
            .await?;

        let json = resp.choices.first().unwrap().message.content.as_str();
        println!("{}", json);
    }

    Ok(())
}
