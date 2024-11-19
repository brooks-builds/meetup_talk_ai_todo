use std::fmt::Display;

use serde::{Deserialize, Serialize};

use super::tool_call::ToolCall;

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Message {
    pub role: Role,
    pub content: String,
    pub tool_calls: Option<Vec<ToolCall>>,
}

impl Message {
    pub fn new_user(content: impl Into<String>) -> Self {
        let role = Role::User;
        let tool_calls = None;

        Self {
            role,
            content: content.into(),
            tool_calls,
        }
    }

    pub fn new_tool(content: impl Into<String>) -> Self {
        let role = Role::Tool;
        let tool_calls = None;

        Self {
            role,
            content: content.into(),
            tool_calls,
        }
    }

    pub fn prepend_content(&mut self, message: &str) {
        self.content.insert_str(0, message);
    }
}

impl Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.content)
    }
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    #[default]
    User,
    Assistant,
    Tool,
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn should_prepend_message_to_content() {
        let mut message = Message::new_tool("world");

        message.prepend_content("hello ");

        assert_eq!(message.content, "hello world");
    }
}
