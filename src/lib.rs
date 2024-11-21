#![allow(unused_attributes)]
pub mod ai;
pub mod commands;
pub mod config;
pub mod logger;
pub mod state;
pub mod tool_property;
pub mod tools;
use std::collections::HashMap;

use ai::create_assistant_chat;
use bb_ollama::models::{chat_request::Chat, message::Message};
use commands::Command;
use db::{connect, delete, erase, get_all_tasks, get_task_by_id, insert, update, Client};
use eyre::{Context, Result};
use logger::{loggit, LogLevel};
use tool_property::ToolProperty;

pub fn run() -> Result<Message> {
    // setup
    let mut personal_assistant = create_assistant_chat();
    let mut db_client = connect().context("connecting to the database")?;

    personal_assistant.add_message(Message::new_system(
        "You are an AI Todo Application. You can CRUD (Create, Read, Update, and Delete) tasks in the database. You are super professional while replying to the user.",
    ));
    personal_assistant.add_message(Message::new_user("User has logged into the system, feel free to ask what their name is then introduce them to yourself and your features."));

    // update
    loop {
        let response = match personal_assistant.send() {
            Ok(message) => message,
            Err(error) => {
                loggit(
                    format!(
                        "There was an error sending a message to the personal assistant: {error:?}"
                    ),
                    LogLevel::Error,
                );
                personal_assistant.add_message(Message::new_user(format!("SYSTEM: Apparently there was an error sending a message to you, let's try whatever you were doing / going to say again")));
                continue;
            }
        };
        let (command, arguments) = Command::from_message(&response);

        match command {
            Command::Chat => {
                handle_chat(arguments);
                get_user_input(&mut personal_assistant);
            }
            Command::InsertTaskIntoDb => {
                handle_insert_task(&mut personal_assistant, arguments, &mut db_client)
                    .context("inserting task into db")?;
            }
            Command::GetAllTasksFromDb => {
                handle_get_all_tasks(&mut personal_assistant, &mut db_client)
                    .context("getting all tasks")?;
            }
            Command::GetTaskByIdFromDb => {
                handle_get_task_by_id(&mut personal_assistant, &mut db_client, arguments)
                    .context("getting task by id")?;
            }
            Command::UpdateTaskInDb => {
                handle_update_task(arguments, &mut personal_assistant, &mut db_client)
                    .context("running update task handler")?
            }
            Command::DeleteTaskInDb => {
                handle_delete_task(&mut db_client, arguments, &mut personal_assistant)
            }
            Command::EraseDb => handle_erase(&mut db_client, &mut personal_assistant),
            Command::Quit => {
                personal_assistant.add_message(Message::new_tool(
                    "Quitting app, you can leave a final message for the user now.",
                ));
                break;
            }
            Command::Unknown => {
                loggit("Unkown command", LogLevel::Error);
                personal_assistant.add_message(Message::new_tool(
                    "That was an unknown tool call, please try again",
                ));
                personal_assistant.add_message(Message::new_user(
                    "That tool name didn't exist. Please try again but use the correct tool name",
                ));
            }
        }
    }
    // teardown

    personal_assistant.send().context("Sending last message")
}

fn handle_insert_task(
    personal_assistant: &mut Chat,
    arguments: HashMap<String, String>,
    db_client: &mut Client,
) -> Result<()> {
    loggit("AI running insert task into db", logger::LogLevel::Info);

    let Some(value) = arguments.get(ToolProperty::Name.to_string().as_str()) else {
        loggit(
            "could not find task name in arguments",
            logger::LogLevel::Error,
        );
        personal_assistant.add_message(Message::new_tool("Error, name was not passed into the tool correctly, please try again, but this time pass in the new task name"));
        return Ok(());
    };

    let new_task = insert(db_client, &value).context("inserting the task into the database")?;

    loggit(
        format!("task inserted into the database :{new_task}"),
        LogLevel::Debug,
    );

    let message = format!("The task was created in the database successfully! Here is the full task that was created: {new_task}");

    personal_assistant.add_message(Message::new_tool(message));

    Ok(())
}

fn handle_get_all_tasks(personal_assistant: &mut Chat, db_client: &mut Client) -> Result<()> {
    loggit("AI running get all tasks tool", LogLevel::Info);

    let all_tasks = get_all_tasks(db_client).context("getting all tasks")?;

    loggit(
        format!("got the following tasks from the database: {all_tasks:?}"),
        LogLevel::Debug,
    );

    let tasks_as_messages = all_tasks
        .into_iter()
        .map(|task| Message::new_tool(task.to_string()))
        .collect::<Vec<Message>>();

    tasks_as_messages
        .into_iter()
        .for_each(|task| personal_assistant.add_message(task));

    Ok(())
}

fn handle_get_task_by_id(
    personal_assistant: &mut Chat,
    db_client: &mut Client,
    arguments: HashMap<String, String>,
) -> Result<()> {
    loggit(format!("handling get one task"), LogLevel::Info);

    let Some(id) = arguments.get(ToolProperty::Id.to_string().as_str()) else {
        loggit("Could not find id in arguments", LogLevel::Error);
        personal_assistant.add_message(Message::new_tool(
            "Error, the id for the task you want to get was not passed into the tool.",
        ));

        return Ok(());
    };

    let Ok(id) = id.parse::<i32>() else {
        loggit(
            "id argument could not be parsed into an i32!",
            LogLevel::Error,
        );
        personal_assistant.add_message(Message::new_tool(
            "ERROR: The id you passed in was not a number",
        ));
        return Ok(());
    };

    let Some(task) = get_task_by_id(db_client, id).context("getting task by id")? else {
        loggit(
            format!("task with id {id} not found in the database"),
            LogLevel::Error,
        );
        personal_assistant.add_message(Message::new_tool("No task exists with the given id"));
        return Ok(());
    };

    loggit(format!("got task from database: {task}"), LogLevel::Debug);

    personal_assistant.add_message(Message::new_tool(task.to_string()));

    Ok(())
}

fn handle_chat(arguments: HashMap<String, String>) {
    loggit("AI chatting", LogLevel::Info);

    for value in arguments.values() {
        loggit(value, LogLevel::Normal);
    }
}

fn handle_update_task(
    arguments: HashMap<String, String>,
    personal_assistant: &mut Chat,
    db_client: &mut Client,
) -> Result<()> {
    loggit("AI ran the update task tool", LogLevel::Info);
    let Some(id) = arguments.get(ToolProperty::Id.to_string().as_str()) else {
        loggit("ai didn't pass in id to update tool", LogLevel::Error);
        personal_assistant.add_message(Message::new_tool(
            "Error, an id was not passed into the update tool",
        ));
        return Ok(());
    };
    let Ok(id) = id.parse() else {
        loggit(
            "An id was supplied, but it was not able to be parsed into an i32",
            LogLevel::Error,
        );
        personal_assistant.add_message(Message::new_tool("The id that you passed into the update tool was not a valid id, please try again but use a valid id."));
        return Ok(());
    };
    let name = arguments
        .get(ToolProperty::Name.to_string().as_str())
        .map(|name| name.as_str());
    let completed = arguments
        .get(ToolProperty::Completed.to_string().as_str())
        .map(|completed| completed.to_lowercase() == "true");
    let updated_task = match update(db_client, id, name, completed) {
        Ok(Some(task)) => task,
        Ok(None) => {
            loggit(
                "Task with supplied id was not found in the database",
                LogLevel::Error,
            );
            personal_assistant.add_message(Message::new_tool("Error: The task with the supplied id was not found, so the task could not be updated"));
            return Ok(());
        }
        Err(error) => {
            loggit(format!("Error: {error:?}"), LogLevel::Error);
            personal_assistant.add_message(Message::new_tool(format!("There was the following error when attempting to update the task in the database: {error:?}")));
            return Ok(());
        }
    };
    loggit(
        format!("The task has been updated in the database. New task: {updated_task}"),
        LogLevel::Debug,
    );
    personal_assistant.add_message(Message::new_tool(format!(
        "The task has been updated. Here is the updated task: {updated_task}"
    )));

    Ok(())
}

fn handle_delete_task(
    db_client: &mut Client,
    arguments: HashMap<String, String>,
    personal_assistant: &mut Chat,
) {
    loggit("AI called the handle delete task tool", LogLevel::Info);

    let Some(stringified_id) = arguments.get(ToolProperty::Id.to_string().as_str()) else {
        loggit("could not find the id", LogLevel::Error);
        personal_assistant.add_message(Message::new_tool(
            "Error, an id was not passed into the tool",
        ));
        return;
    };
    let id = match stringified_id.parse() {
        Ok(id) => id,
        Err(error) => {
            loggit(
                format!("Id could not be parsed into an i32: {error:?}"),
                LogLevel::Error,
            );
            personal_assistant.add_message(Message::new_tool(format!(
                "Error, the id you passed in was not a stringified number."
            )));
            return;
        }
    };
    match delete(db_client, id) {
        Ok(count) => {
            loggit(
                format!("Deleted {count} tasks from the database"),
                LogLevel::Info,
            );
            personal_assistant.add_message(Message::new_tool(format!(
                "Success, {count} tasks have been deleted from the database"
            )));
        }
        Err(error) => {
            loggit(
                format!("Failed to delete the task from the database: {error:?}"),
                LogLevel::Error,
            );
            personal_assistant.add_message(Message::new_tool(format!(
                "Error deleting the task from the database: {error:?}"
            )));
            return;
        }
    }
}

fn handle_erase(db_client: &mut Client, personal_assistant: &mut Chat) {
    loggit("AI is erasing the database", LogLevel::Info);
    match erase(db_client) {
        Ok(count) => {
            loggit(
                format!("{count} tasks removed from the database"),
                LogLevel::Debug,
            );
            personal_assistant.add_message(Message::new_tool(format!("You have erased the database! {count} tasks were removed in this purge. You may allow yourself to express remorse or mad scientist vibes for your reply")));
        }
        Err(error) => {
            loggit(format!("{error:?}"), LogLevel::Error);
            personal_assistant.add_message(Message::new_tool(format!(
                "There was an error erasing the database! The error was {error:?}"
            )));
        }
    }
}

fn get_user_input(personal_assistant: &mut Chat) {
    let mut user_input = String::new();
    if let Err(error) = std::io::stdin()
        .read_line(&mut user_input)
        .context("getting user input")
    {
        loggit(format!("{error:?}"), LogLevel::Error);
        personal_assistant.add_message(Message::new_tool(format!(
            "There was an error getting input from your user: {error:?}"
        )));
        return;
    }

    personal_assistant.add_message(Message::new_tool(format!("The user said: {user_input}. To answer the question use one of the tools to find to appropriate information before responding.")));
}
