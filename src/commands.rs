use bb_ollama::models::message::Message;
use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    RequestSql,
    Chat,
    RunSql,
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
            "request_sql" => Self::RequestSql,
            "run_sql" => Self::RunSql,
            _ => Self::Chat,
        }
    }
}

impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let command = match self {
            Self::RequestSql => "request_sql",
            Self::Chat => "chat",
            Self::RunSql => "run_sql",
        };

        write!(f, "{command}")
    }
}
