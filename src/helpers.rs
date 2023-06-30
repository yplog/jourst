use chrono::{DateTime, Local};
use colored::Colorize;

use crate::{
    args::{AddCommandDate, ListCommandDate, ListCommandType},
    models::Todo,
};

pub fn get_date(date: &AddCommandDate) -> DateTime<Local> {
    match date {
        AddCommandDate::Today => Local::now(),
        AddCommandDate::Tomorrow => Local::now()
            .checked_add_signed(chrono::Duration::days(1))
            .unwrap(),
        AddCommandDate::Yesterday => Local::now()
            .checked_sub_signed(chrono::Duration::days(1))
            .unwrap(),
    }
}

pub fn get_filter_date(date: &ListCommandDate) -> Option<DateTime<Local>> {
    match date {
        ListCommandDate::Today => Some(Local::now()),
        ListCommandDate::Tomorrow => Some(
            Local::now()
                .checked_add_signed(chrono::Duration::days(1))
                .unwrap(),
        ),
        ListCommandDate::Yesterday => Some(
            Local::now()
                .checked_sub_signed(chrono::Duration::days(1))
                .unwrap(),
        ),
        ListCommandDate::All => None,
    }
}

pub fn get_kind(kind: &ListCommandType) -> Option<bool> {
    match kind {
        ListCommandType::Done => Some(true),
        ListCommandType::Undone => Some(false),
        ListCommandType::All => None,
    }
}

pub fn print_table_result(todos: Result<Vec<Todo>, diesel::result::Error>) {
    println!(
        "{0: <4} | {1: <2} | {2: <24} | {3: <10}",
        "done", "id", "content", "date"
    );

    for todo in todos.unwrap() {
        println!("{}", todo);
    }
}

pub fn print_result<T, E>(result: Result<T, E>) {
    match result {
        Ok(_) => println!("{}", "Ok!".green()),
        Err(_) => println!("{}", "Error!".red()),
    }
}
