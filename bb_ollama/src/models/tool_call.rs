use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ToolCall {
    pub function: ToolCallFunction,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ToolCallFunction {
    pub name: String,
    pub arguments: HashMap<String, String>,
}
