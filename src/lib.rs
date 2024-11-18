pub mod ai;
pub mod config;
pub mod tools;

use core::panic;
use std::io::stdin;

use ai::ask_ai_what_tool_to_use;
use bb_ollama::models::{
    chat_request::Chat,
    message::Message,
    options::ChatRequestOptions,
    tool::{Property, Tool},
};
use config::Config;
use db::connect;
use eyre::{Context, OptionExt, Result};

pub fn run(config: Config) -> Result<()> {
    let options = ChatRequestOptions::new().system("you are a todo application, greet the user when you don't have any history of todos. Just write as if you were talking to a real person who has walked up to the todo counter. Your todo app name is AI todo, and the user is user. When creating sql to manage the tasks, you can insert, select, update, or delete. Deleting is for when tasks are completed. As a senior sql developer, you know to use id's when mutating the database");
    let mut app = Chat::new(config.model, Some(options));
    let initial_message = "hello";
    let mut db_client = connect()?;

    app.add_message(Message::new_user(initial_message));

    let app_greeting = app.send().context("Sending initial message")?;

    println!("{app_greeting}");

    app.add_message(app_greeting);

    loop {
        let user_input = get_user_input().context("getting user input")?;

        // println!("user input: {user_input}");

        let user_message = Message::new_user(user_input);

        app.add_message(user_message);

        app.add_tool(Tool::new().function_name("run_sql").function_description("Run a SQL command for a postgres database that has a single table named tasks. The table has the following columns: id: int, name: text. The id is auto generated when creating, please try your best to let the database set the id when creating").add_function_property("sql", Property::new_string("The SQL to be run on the database, make sure that you include the ; otherwise it won't run")).add_required_property("sql").build().ok_or_eyre("Adding tool")?);

        let tool_result = app.send().context("getting sql")?;

        println!("\n***result of command: {:?}***\n", tool_result.tool_calls);

        let tool_calls = &tool_result.tool_calls.unwrap();
        let arguments = &tool_calls.first().unwrap().function.arguments;
        let sql = arguments.get("sql").unwrap();

        let result = db::run_query(&mut db_client, sql).context("running query from ai")?;
        let result_message = Message::new_tool(format!("Here is the result of the SQL query. It is in Rust code so you will need to parse the relevant info out: {result}"));

        app.add_message(result_message);
        app.add_message(Message::new_user(
            "convert the previous tool call result into a format that makes sense",
        ));

        app.tools.clear();
        let reform_message = app.send().context("having ai reform the message")?;

        println!("***\nAfter reforming message: {reform_message}***\n");

        app.add_message(reform_message);

        let after_tool_response = app
            .send()
            .context("asking ai for response after sql was run")?;

        println!("{after_tool_response}");

        println!("\n")
    }

    // Ok(())
}

fn get_user_input() -> Result<String> {
    let mut user_input = String::new();

    stdin()
        .read_line(&mut user_input)
        .context("Reading user input")?;

    Ok(user_input.trim().to_owned())
}
