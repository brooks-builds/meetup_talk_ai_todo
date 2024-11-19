use bb_ollama::models::message::Message;
use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    InsertTaskIntoDb,
    GetAllTasksFromDb,
    GetTaskByIdFromDb,
    UpdateTaskInDb,
    DeleteTaskInDb,
    EraseDb,
    Chat,
    RunSql,
    Quit,
}

impl Command {
    pub fn from_message(message: &Message) -> (Self, String) {
        if message.content.is_empty() {
            let Some(tool_calls) = &message.tool_calls else {
                return (
                    Self::Chat,
                    "I'm sorry, but I didn't understand what you said, please try again".to_owned(),
                );
            };
            let Some(tool_call) = tool_calls.first() else {
                return (
                    Self::Chat,
                    "I'm sorry, but I didn't understand what you said, please try again".to_owned(),
                );
            };
            let function = &tool_call.function;
            let name = &function.name;
            let command = Self::from(name.as_str());
            let argument = function
                .arguments
                .get(name)
                .cloned()
                .unwrap_or("".to_owned());

            (command, argument.to_owned())
        } else {
            (Self::Chat, message.content.clone())
        }
    }
}

impl Into<String> for Command {
    fn into(self) -> String {
        self.to_string()
    }
}

impl From<&str> for Command {
    fn from(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "insert_task_into_db" => Self::InsertTaskIntoDb,
            "run_sql" => Self::RunSql,
            "get_all_tasks_from_db" => Self::GetAllTasksFromDb,
            "get_task_by_id_from_db" => Self::GetTaskByIdFromDb,
            "update_task_in_db" => Self::UpdateTaskInDb,
            "delete_task_in_db" => Self::DeleteTaskInDb,
            "erase_db" => Self::EraseDb,
            "quit" => Self::Quit,
            _ => Self::Chat,
        }
    }
}

impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let command = match self {
            Self::InsertTaskIntoDb => "insert_task_into_db",
            Self::Chat => "chat",
            Self::RunSql => "run_sql",
            Command::GetAllTasksFromDb => "get_all_tasks_from_db",
            Command::GetTaskByIdFromDb => "get_task_by_id_from_db",
            Command::UpdateTaskInDb => "update_task_in_db",
            Command::DeleteTaskInDb => "delete_task_in_db",
            Command::EraseDb => "erase_db",
            Command::Quit => "quit",
        };

        write!(f, "{command}")
    }
}
