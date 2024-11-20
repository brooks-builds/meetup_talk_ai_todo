use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ChatRequestOptions {
    pub system: Option<String>,
    pub seed: Option<u32>,
    pub save_messages: bool,
    pub temperature: f32,
}

impl ChatRequestOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn seed(mut self, seed: u32) -> Self {
        self.seed = Some(seed);

        self
    }

    pub fn system(mut self, system: impl Into<String>) -> Self {
        self.system = Some(system.into());

        self
    }

    pub fn save_messages(mut self) -> Self {
        self.save_messages = true;

        self
    }

    pub fn temperature(mut self, temperature: f32) -> Self {
        self.temperature = temperature;
        self
    }
}
