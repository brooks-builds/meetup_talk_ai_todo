#![allow(unused_attributes)]
pub mod ai;
pub mod commands;
pub mod config;
pub mod state;
pub mod tools;
use core::panic;

use ai::create_assistant_chat;
use bb_ollama::models::{chat_request::Chat, message::Message};
use commands::Command;
use db::{connect, insert, run_query, Client};
use eyre::{Context, Result};

pub fn run() -> Result<Message> {
    // setup
    let mut personal_assistant = create_assistant_chat();
    let mut db_client = connect().context("connecting to the database")?;

    personal_assistant.add_message(Message::new_user(
        "User has connected and is ready to work with you",
    ));

    // update
    loop {
        let response = personal_assistant
            .send()
            .context("Sending message to personal assistant")?;
        let (command, value) = Command::from_message(&response);

        match command {
            Command::Chat => {
                handle_chat(value, &mut personal_assistant).context("handling chat command")?
            }
            Command::RunSql => handle_run_sql(value, &mut personal_assistant, &mut db_client)
                .context("handling running sql query")?,
            Command::InsertTaskIntoDb => {
                handle_insert_task(&mut personal_assistant, value, &mut db_client)
                    .context("inserting task into db")?;
            }
            Command::GetAllTasksFromDb => todo!(),
            Command::GetTaskByIdFromDb => todo!(),
            Command::UpdateTaskInDb => todo!(),
            Command::DeleteTaskInDb => todo!(),
            Command::EraseDb => todo!(),
            Command::Quit => break,
        }
    }
    // teardown

    todo!()
}

fn handle_insert_task(
    personal_assistant: &mut Chat,
    value: String,
    db_client: &mut Client,
) -> Result<()> {
    #[cfg(feature = "log")]
    println!("handling insert task: {value}");

    let created_id = insert(db_client, &value).context("inserting the task into the database")?;
    let message = format!("The task was created with the id {created_id}");

    personal_assistant.add_message(Message::new_tool(message));

    Ok(())
}

fn handle_chat(value: String, personal_assistant: &mut Chat) -> Result<()> {
    println!("{value}");
    let user_input = get_user_input().context("handle chat")?;
    let message = Message::new_user(user_input);
    personal_assistant.add_message(message);

    Ok(())
}

fn get_user_input() -> Result<String> {
    let mut user_input = String::new();
    std::io::stdin()
        .read_line(&mut user_input)
        .context("getting user input")?;

    Ok(user_input.trim().to_owned())
}

fn handle_run_sql(
    request: String,
    personal_assistant: &mut Chat,
    db_client: &mut Client,
) -> Result<()> {
    match run_query(db_client, &request) {
        Ok(result) => personal_assistant.add_message(Message::new_tool(result)),
        Err(error) => personal_assistant.add_message(Message::new_tool(format!(
            "There was an error attempting to run the database query: {error}"
        ))),
    }

    Ok(())
}
