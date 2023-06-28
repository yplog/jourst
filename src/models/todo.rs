use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Todo {
    pub id: u32,
    pub content: String,
    pub date: String,
    pub is_done: bool,
}

impl Todo {
    pub fn new(id: u32, content: String, date: String, is_done: bool) -> Todo {
        Todo {
            id,
            content,
            date,
            is_done,
        }
    }
}