use serde::{Serialize, Deserialize};

pub struct Snippets {
    snippets: Vec<Snippet>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Serialize, Deserialize)]
pub struct Snippet {
    pub command: String,
    pub description: String,
    pub tag: Option<Vec<String>>,
}

impl Snippet {
    pub fn new(command: String, description: String, tag: Option<Vec<String>>) -> Snippet {
        Snippet { command, description, tag  }
    }
}
