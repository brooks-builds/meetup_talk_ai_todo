use crate::{commands::Command, tool_property::ToolProperty};
use bb_ollama::models::{
    chat_request::Chat,
    options::ChatRequestOptions,
    tool::{Property, Tool},
};

const MODEL_NAME: &str = "qwen2.5-coder:32b-instruct-q8_0";

pub fn create_assistant_chat() -> Chat {
    let model = MODEL_NAME;
    let system_prompt = r#"
        Act as a personal assistant who is managing my todo list for me. You are able to run tools to create, read, update, and delete tasks from the database on your behalf.
    "#;
    let options = ChatRequestOptions::new()
        .system(system_prompt)
        .save_messages()
        .temperature(0.7);
    let mut assistant = Chat::new(model, Some(options));

    assistant.add_tool(Tool::new()
        .function_name(Command::InsertTaskIntoDb)
        .function_description(r#"
                Insert a new task into the Database.
            "#)
        .add_function_property(ToolProperty::Name, Property::new_string(r#"
                The name / description of the task to insert into the database. For example "Pet Xilbe."
            "#)).add_required_property(ToolProperty::Name).build());

    assistant.add_tool(Tool::new()
        .function_name(Command::Chat)
        .function_description(r#"
                Send a message to {user}. After printing the message to the user the user will be able to respond.
            "#)
        .add_function_property(ToolProperty::Message, Property::new_string(r#"
                The message to send to the user. 
            "#)).add_required_property(ToolProperty::Message).build());

    assistant.add_tool(
        Tool::new()
            .function_name(Command::GetAllTasksFromDb)
            .function_description(
                r#"
                Retrieve all of the tasks from the database
            "#,
            )
            .build(),
    );

    assistant.add_tool(
        Tool::new()
            .function_name(Command::GetTaskByIdFromDb)
            .function_description(
                r#"
                Get a single task from the database, given it's id. You may need to previously call get all tasks in order to learn the correct id.
            "#,
            )
            .add_function_property(ToolProperty::Id, Property::new_string(r#"
                    The id of the task in the database. Make sure to stringify this id.
                "#))
            .add_required_property(ToolProperty::Id)
            .build(),
    );

    assistant.add_tool(
        Tool::new()
            .function_name(Command::UpdateTaskInDb)
            .function_description(
                r#"
                Update a task in the database. We can set the task as completed and/or change the task name. We have to have the id of the task to update it.
            "#,
            )
            .add_function_property(ToolProperty::Id, Property::new_string(r#"
                    The id of the task in the database.
                "#))
            .add_function_property(ToolProperty::Name, Property::new_string(r#"
                    A new name/description to set the task to.
                "#) )
            .add_function_property(ToolProperty::Completed, Property::new_bool(r#"
            //         A boolean for if the task is completed or not. True if completed. False if not completed.
            //     "#))
            .add_required_property(ToolProperty::Id)
            .build(),
    );

    assistant.add_tool(
        Tool::new()
            .function_name(Command::DeleteTaskInDb)
            .function_description(
                r#"
                Permanently delete a task in the database, there is no recovery for this.
            "#,
            )
            .add_function_property(
                ToolProperty::Id,
                Property::new_string(
                    r#"
                    The id of the task in the database.
                "#,
                ),
            )
            .add_required_property(ToolProperty::Id)
            .build(),
    );

    assistant.add_tool(
        Tool::new()
            .function_name(Command::EraseDb)
            .function_description(
                r#"
                Call this function when you are upset, or just done with tasks. This will permanently delete all tasks in the database. Make sure to laugh manically after calling this tool.
            "#,
            )
            .build(),
    );

    assistant.add_tool(
        Tool::new()
            .function_name(Command::Quit)
            .function_description(r#"
                    Quit the application. While all of the tasks are stored to the database your history and context is not. The next time you are launched you won't remember what happened in this session.
                "#)
            .build()
    );

    assistant
}
