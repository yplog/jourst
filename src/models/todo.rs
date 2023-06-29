use chrono::{NaiveDateTime, NaiveDate};
use diesel::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Todo {
    pub id: i32,

    pub content: String,

    #[serde(skip_deserializing)]
    pub date: NaiveDate,
    
    pub completed: bool,

    #[serde(skip_deserializing)]
    pub created_at: NaiveDateTime,
    
    #[serde(skip_deserializing)]
    pub updated_at: NaiveDateTime,
}