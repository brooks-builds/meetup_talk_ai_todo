use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ChatRequestOptions {
    pub system: Option<String>,
    pub seed: Option<u32>,
}

impl ChatRequestOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn seed(mut self, seed: u32) -> Self {
        self.seed = Some(seed);

        self
    }
}
