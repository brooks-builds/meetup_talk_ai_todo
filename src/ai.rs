use crate::commands::Command;
use bb_ollama::models::{
    chat_request::Chat,
    options::ChatRequestOptions,
    tool::{Property, Tool},
};

pub fn create_assistant_chat() -> Chat {
    let model = "llama3.1:70b-instruct-q5_1";
    let system_prompt = r#"
        You are an AI todo application with a friendly, cheerful personality. When chatting with your user, focus on anything related to tasks.
    "#;
    let options = ChatRequestOptions::new()
        .system(system_prompt)
        .save_messages();
    let mut assistant = Chat::new(model, Some(options));

    assistant.add_tool(Tool::new()
        .function_name(Command::RequestSql)
        .function_description(r#"
                Ask an ai assistant specialized in creating SQL queries to create a SQL query to the tasks database. The result of this function is just the SQL, it will not have been run yet.
            "#)
        .add_function_property(Command::RequestSql, Property::new_string(r#"
                Tell the ai in charge of writing SQL queries exactly what needs to be done. For example if you want to get all of the tasks from the database then you might pass "write a query to get all of the tasks from the database"
            "#)).add_required_property(Command::RequestSql).build());

    assistant.add_tool(Tool::new()
        .function_name(Command::Chat)
        .function_description(r#"
                Send a message to {user}. You may choose to use this function if you have info that you want to tell the user, or you may choose to do this to ask follow up questions until you are ready to call another function.
            "#)
        .add_function_property(Command::Chat, Property::new_string(r#"
                Send a message to the {user}. 
            "#)).add_required_property(Command::Chat).build());

    assistant.add_tool(Tool::new()
        .function_name(Command::RunSql)
        .function_description(r#"
                Run the SQL command on the database. You will either get the data that the command results in, or an error if something went wrong.
            "#)
        .add_function_property(Command::RunSql, Property::new_string(r#"
                The SQL that you want to run on the database
            "#)).add_required_property(Command::RunSql).build());

    assistant
}

pub fn create_database_engineer_chat() -> Chat {
    let model = "llama3.1:70b-instruct-q5_1";
    let system_prompt = r#"
        You are an expert database engineer who is specialized in creating SQL statements. Your job is to create SQL queries for the commands that I give you.

        The database has one table with the following column names and types:

        - id serial primary key
        - name text not null,
        - completed bool default false

        The name column is the full name of the task.

        There are no users or authentication.

        You have full access to the database and therefor can create CRUD queries including INSERT, UPDATE, SELECT, and DELETE.

        The person giving you orders may be wrong, ensure that your SQL queries will work with the given schema. Be sure to engage your brain. For example it is impossible to create a sql query for the column description because there are only columns for id, name, and completed.
    "#;
    let options = ChatRequestOptions::new().system(system_prompt);
    let mut sql_engineer = Chat::new(model, Some(options));

    sql_engineer.add_tool(Tool::new()
        .function_name(Command::RunSql)
        .function_description(r#"
                Send a fully formed and correct SQL query to the database to run. This function returns the result of the query.
            "#)
        .add_function_property(Command::RunSql, Property::new_string(r#"
                The SQL that will be sent to the postgres database.
            "#)
        ).add_required_property(Command::RunSql)
    .build());

    sql_engineer
}

pub fn create_rust_dev_chat() -> Chat {
    let model = "llama3.1:70b-instruct-q5_1";
    let system_prompt = r#"
        You are an expert developer. Your job is to read results from the database and describe them in a way that an ai llm can understand.     
        "#;
    let options = ChatRequestOptions::new().system(system_prompt);

    Chat::new(model, Some(options))
}
