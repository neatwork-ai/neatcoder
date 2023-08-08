use anyhow::Result;
use gluon::ai::openai::{client::OpenAI, job::OpenAIJob, msg::{OpenAIMsg, GptRole}};

pub async fn generate_db_schema(client: &OpenAI, job: &OpenAIJob) -> Result<(String, String)> {
    let sys_msg = OpenAIMsg {
        role: GptRole::System,
        content: String::from("You are a entrepreneur with product management background."),
    };

    let user_msg = OpenAIMsg {
        role: GptRole::User,
        content: String::from("Generate a random idea for a company. The first word in your responde should be the company name."),
    };

    let resp = client.chat(job, &[&sys_msg, &user_msg], &[], &[]).await?;
    let answer = resp.choices.first().unwrap().message.content.as_str();

    let company_name = get_first_word(answer);

    let sys_msg_2 = OpenAIMsg {
        role: GptRole::System,
        content: String::from("You are a data engineer hired to work on this project."),
    };

    let user_msg_2 = OpenAIMsg {
        role: GptRole::User,
        content: String::from(
            "Based on the above project description, produce a database schema in form of SQL DDL.",
        ),
    };

    let resp = client
        .chat(
            job,
            &[&sys_msg, &user_msg, &sys_msg_2, &user_msg_2],
            &[],
            &[],
        )
        .await?;
    let answer = resp.choices.first().unwrap().message.content.as_str();

    Ok((String::from(company_name), String::from(answer)))
}

fn get_first_word(input: &str) -> &str {
    let mut words = input.split_whitespace();
    words.next().unwrap_or("")
}
