use eyre::{Context, Result};
use reqwest::Url;

pub struct Config {
    pub model: String,
    pub ollama_url: Url,
}

impl Config {
    pub fn new() -> Result<Self> {
        let model = "llama3.1:8b-instruct-fp16".to_owned();
        let ollama_url =
            Url::parse("http://localhost:11434/api/chat").context("creating ollama url")?;

        Ok(Self { model, ollama_url })
    }
}
