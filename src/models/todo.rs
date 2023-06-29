use chrono::NaiveDate;
use diesel::prelude::*;
use serde::Deserialize;

use crate::schema::todos;

#[derive(Queryable, Selectable, Debug, PartialEq)]
#[diesel(table_name = todos)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Todo {
    pub id: i32,
    pub content: String,
    pub completed: bool,
    pub when_will_it_be_done: NaiveDate,
}

#[derive(Insertable, Debug, Deserialize)]
#[diesel(table_name = todos)]
pub struct NewTodo {
    pub content: String,
    pub when_will_it_be_done: NaiveDate,
}
