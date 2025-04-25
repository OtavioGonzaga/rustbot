use serde::{Deserialize, Serialize};

use super::{Part, Role};

#[derive(Serialize, Deserialize)]
pub struct Contents {
    contents: Vec<Content>,
}

impl Contents {
    pub fn new(contents: Vec<Content>) -> Self {
        Self { contents }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Content {
    parts: Vec<Part>,
    role: Role,
}

impl Content {
    pub fn new(parts: Vec<Part>, role: Role) -> Self {
        Self { parts, role }
    }
}
