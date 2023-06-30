use chrono::NaiveDate;
use colored::Colorize;
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

#[derive(Debug, Deserialize)]
pub struct FilterTodo {
    pub completed: Option<bool>,
    pub when_will_it_be_done: Option<NaiveDate>,
}

impl std::fmt::Display for Todo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let completed = if self.completed { "[X]" } else { "[ ]" };

        let line = if self.completed {
            format!(
                "{0: <4} | {1: <2} | {2: <24} | {3: <10}",
                completed, self.id, self.content, self.when_will_it_be_done
            )
            .green()
        } else {
            format!(
                "{0: <4} | {1: <2} | {2: <24} | {3: <10}",
                completed, self.id, self.content, self.when_will_it_be_done
            )
            .red()
        };

        write!(f, "{}", line)
    }
}
