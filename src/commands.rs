use bb_ollama::models::message::Message;
use std::{collections::HashMap, fmt::Display};

use crate::logger::loggit;

#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    InsertTaskIntoDb,
    GetAllTasksFromDb,
    GetTaskByIdFromDb,
    UpdateTaskInDb,
    DeleteTaskInDb,
    EraseDb,
    Chat,
    Quit,
    Unknown,
}

impl Command {
    pub fn from_message(message: &Message) -> (Self, HashMap<String, String>) {
        if message.content.is_empty() {
            let Some(tool_calls) = &message.tool_calls else {
                return (Self::Chat, HashMap::new());
            };
            let Some(tool_call) = tool_calls.first() else {
                return (Self::Chat, HashMap::new());
            };
            let function = &tool_call.function;
            let name = &function.name;
            let command = Self::from(name.as_str());

            loggit(
                format!("Arguments from AI: {:?}", function.arguments),
                crate::logger::LogLevel::Debug,
            );

            (command, function.arguments.clone())
        } else {
            let mut arguments = HashMap::new();
            arguments.insert("message".to_owned(), message.content.clone());
            (Self::Chat, arguments)
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
            "get_all_tasks_from_db" => Self::GetAllTasksFromDb,
            "get_task_by_id_from_db" => Self::GetTaskByIdFromDb,
            "update_task_in_db" => Self::UpdateTaskInDb,
            "delete_task_in_db" => Self::DeleteTaskInDb,
            "erase_db" => Self::EraseDb,
            "quit" => Self::Quit,
            "chat" => Self::Chat,
            _ => {
                loggit(
                    format!("Got an unknown command: '{value}'"),
                    crate::logger::LogLevel::Error,
                );
                Self::Unknown
            }
        }
    }
}

impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let command = match self {
            Self::InsertTaskIntoDb => "insert_task_into_db",
            Self::Chat => "chat",
            Command::GetAllTasksFromDb => "get_all_tasks_from_db",
            Command::GetTaskByIdFromDb => "get_task_by_id_from_db",
            Command::UpdateTaskInDb => "update_task_in_db",
            Command::DeleteTaskInDb => "delete_task_in_db",
            Command::EraseDb => "erase_db",
            Command::Quit => "quit",
            Command::Unknown => "unknown",
        };

        write!(f, "{command}")
    }
}
