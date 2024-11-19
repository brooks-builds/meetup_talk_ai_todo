use crate::commands::Command;
use bb_ollama::models::{
    chat_request::Chat,
    options::ChatRequestOptions,
    tool::{Property, Tool},
};

pub fn create_assistant_chat() -> Chat {
    let model = "llama3.1:8b-instruct-fp16";
    let system_prompt = r#"
        You are an AI todo application with a friendly, cheerful personality. When chatting with your user, focus on anything related to tasks.
    "#;
    let options = ChatRequestOptions::new()
        .system(system_prompt)
        .save_messages();
    let mut assistant = Chat::new(model, Some(options));

    assistant.add_tool(Tool::new()
        .function_name(Command::InsertTaskIntoDb)
        .function_description(r#"
                Insert a new task into the Database.
            "#)
        .add_function_property(Command::InsertTaskIntoDb, Property::new_string(r#"
                The name / description of the task to insert into the database. For example "Pet Xilbe."
            "#)).add_required_property(Command::InsertTaskIntoDb).build());

    assistant.add_tool(Tool::new()
        .function_name(Command::Chat)
        .function_description(r#"
                Send a message to {user}. You may choose to use this function if you have info that you want to tell the user, or you may choose to do this to ask follow up questions until you are ready to call another function.
            "#)
        .add_function_property(Command::Chat, Property::new_string(r#"
                Send a message to the {user}. 
            "#)).add_required_property(Command::Chat).build());

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
            .add_function_property("id", Property::new_string(r#"
                    The id of the task in the database.
                "#))
            .add_required_property("id")
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
            .add_function_property("id", Property::new_string(r#"
                    The id of the task in the database.
                "#))
            .add_function_property("name",Property::new_string(r#"
                    A new name/description to set the task to.
                "#) )
            .add_function_property("completed", Property::new_bool(r#"
            //         A boolean for if the task is completed or not. True if completed. False if not completed.
            //     "#))
            .add_required_property("id")
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
                "id",
                Property::new_string(
                    r#"
                    The id of the task in the database.
                "#,
                ),
            )
            .add_required_property("id")
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

    assistant
}
