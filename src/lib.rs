pub mod ai;
pub mod commands;
pub mod config;
pub mod state;
pub mod tools;
use ai::{create_assistant_chat, create_database_engineer_chat};
use bb_ollama::models::{chat_request::Chat, message::Message};
use commands::Command;
use db::{connect, run_query, Client};
use eyre::{Context, Result};
use state::AppState;
use std::io::stdin;

pub fn run() -> Result<Message> {
    // setup
    let mut personal_assistant = create_assistant_chat();
    let mut app_state = AppState::default();
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
            Command::RequestSql => handle_request_for_sql(value, &mut personal_assistant)
                .context("handling request for sql")?,
            Command::Chat => {
                handle_chat(value, &mut personal_assistant).context("handling chat command")?
            }
            Command::RunSql => handle_run_sql(value, &mut personal_assistant, &mut db_client)
                .context("handling running sql query")?,
        }
    }
    // teardown

    todo!()
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

fn handle_request_for_sql(request: String, personal_assistant: &mut Chat) -> Result<()> {
    #[cfg(feature = "log")]
    println!("handling request for sql: {request}");

    let message = Message::new_user(request);
    let mut db_ai = create_database_engineer_chat();

    db_ai.add_message(message);

    let mut response = db_ai.send().context("Sending message to db ai")?;
    let (_, value) = Command::from_message(&response);
    #[cfg(feature = "log")]
    println!("response from db ai: {value}");

    let message = Message::new_tool(value);
    personal_assistant.add_message(message);
    personal_assistant.add_message(Message::new_user("If you like the sql, go ahead and use the tool to run it, otherwise use the tool to request the db ai to try again"));

    Ok(())
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
