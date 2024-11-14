use serde::{Deserialize, Serialize};

use super::tool_call::ToolCall;

#[derive(Debug, Default, Serialize, Deserialize)]
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
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    #[default]
    User,
    Assistant,
    Tool,
}
