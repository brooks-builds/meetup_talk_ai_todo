use eyre::{Context, Result};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

use crate::config::Config;

pub enum Command {
    Create,
    Update,
    Get,
    Delete,
}

pub fn ask_ai_what_tool_to_use(user_input: &str, config: &Config) -> Result<Command> {
    println!("asking ai what to do");
    let response = send_to_ai(config, user_input)?;

    todo!()
}

#[derive(Debug, Serialize)]
struct RequestJson {
    pub model: String,
    pub messages: Vec<OllamaMessage>,
    pub stream: bool,
    pub raw: bool,
    pub tools: Vec<Tool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OllamaMessage {
    pub role: String,
    pub content: String,
    pub tool_calls: Option<Vec<Tool>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Tool {
    #[serde(rename = "type")]
    pub tool_type: Option<String>,
    pub function: Function,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Function {
    pub name: String,
    pub description: Option<String>,
}

fn send_to_ai(config: &Config, user_input: &str) -> Result<()> {
    let client = Client::new();
    let body = RequestJson {
        model: config.model.clone(),
        messages: vec![OllamaMessage {
            role: "user".to_owned(),
            content: user_input.to_owned(),
            tool_calls: None,
        }],
        stream: false,
        raw: false,
        tools: vec![
            Tool {
                tool_type: Some("function".to_owned()),
                function: Function {
                    name: "create_task".to_owned(),
                    description: Some(
                        "the user wants to create a new task and insert it into the database."
                            .to_owned(),
                    ),
                },
            },
            Tool {
                tool_type: Some("function".to_owned()),
                function: Function {
                    name: "get_tasks".to_owned(),
                    description: Some(
                        "the user wants to retrieve one or more tasks from the database."
                            .to_owned(),
                    ),
                },
            },
            Tool {
                tool_type: Some("function".to_owned()),
                function: Function {
                    name: "update_task".to_owned(),
                    description: Some(
                        "the user wants to update a task in the database.".to_owned(),
                    ),
                },
            },
            Tool {
                tool_type: Some("function".to_owned()),
                function: Function {
                    name: "delete_task".to_owned(),
                    description: Some(
                        "the user wants to delete a task in the database.".to_owned(),
                    ),
                },
            },
        ],
    };
    let response = client
        .post(config.ollama_url.clone())
        .json(&body)
        .send()
        .context("sending request find what the ai wants to do")?
        .json::<OllamaResponse>()
        .context("converting response to rust")?;

    println!("response from ai: {response:?}");

    todo!()
}

#[derive(Debug, Deserialize)]
pub struct OllamaResponse {
    pub model: String,
    pub message: OllamaMessage,
}
