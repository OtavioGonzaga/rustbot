use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Part {
    text: String,
}

impl Part {
    pub fn new(text: String) -> Self {
        Self { text }
    }
}

impl From<&str> for Part {
    fn from(text: &str) -> Self {
        Self::new(String::from(text))
    }
}
