use serde::{Deserialize, Serialize};

use super::Todo;

#[derive(Serialize, Deserialize, Debug)]
pub struct File {
    pub title: String,
    pub description: String,
    pub todos: Vec<Todo>,
}

impl File {
    pub fn new(title: String, description: String, todos: Vec<Todo>) -> File {
        File {
            title,
            description,
            todos,
        }
    }
}