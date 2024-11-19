pub mod ai;
pub mod commands;
pub mod config;
pub mod state;
pub mod tools;
use ai::{create_assistant_chat, create_database_engineer_chat};
use bb_ollama::models::message::Message;
use commands::Command;
use db::{connect, run_query};
use eyre::{Context, Result};
use std::io::stdin;

pub fn run() -> Result<Message> {
    let mut personal_assistant = create_assistant_chat();
    let mut database_engineer = create_database_engineer_chat();
    let mut db_client = connect()?;

    personal_assistant.add_message(Message::new_user("Hello"));

    let app_greeting = personal_assistant
        .send()
        .context("Sending initial message")?;

    let (command, greeting) = Command::from_message(&app_greeting);
    if command == Command::Chat {
        println!("{greeting}");
    }

    loop {
        let user_input = get_user_input().context("main loop getting user input")?;

        personal_assistant.add_message(Message::new_user(user_input));
        let (command, message) = Command::from_message(
            &personal_assistant
                .send()
                .context("main loop getting command from message")?,
        );

        #[cfg(feature = "log_messages")]
        println!("\n***Personal Assistant running command {command}: {message}***\n");

        match command {
            Command::RequestSql => {
                database_engineer.add_message(Message::new_user(message));

                let (command, query) = Command::from_message(
                    &database_engineer
                        .send()
                        .context("sending message to database engineer")?,
                );
                if command == Command::Sql {
                    #[cfg(feature = "log_messages")]
                    println!("database engineer wrote this query: {query}");

                    let Ok(query_result) = run_query(&mut db_client, &query) else {
                        personal_assistant.add_message(Message::new_tool(
                            "There was an error running the database query",
                        ));

                        continue;
                    };

                    #[cfg(feature = "log_messages")]
                    println!("result from the database: {query_result}");

                    let message_to_rust_dev = Message::new_tool(query_result);

                    personal_assistant.add_message(message_to_rust_dev);
                } else {
                    #[cfg(feature = "log_messages")]
                    println!("Not a sql command: {query}");

                    panic!("database engineer didn't call an appropriate tool");
                }
            }
            Command::Chat => println!("{message}"),
            Command::Sql => break,
        }

        println!(
            "{}",
            personal_assistant
                .send()
                .context("Turn over, having personal assistant respond to user")?
        );
    }

    personal_assistant.add_message(Message::new_user(
        "Thanks for the help, I'm heading out now",
    ));
    personal_assistant.send().context("Sending end message")
}

fn get_user_input() -> Result<String> {
    let mut user_input = String::new();

    stdin()
        .read_line(&mut user_input)
        .context("Reading user input")?;

    Ok(user_input.trim().to_owned())
}
