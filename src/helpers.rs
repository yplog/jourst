use chrono::{DateTime, Local, NaiveDate};
use colored::Colorize;
use std::{io::{self, Write}, collections::HashMap};

use crate::{
    args::{AddCommandDate, ListCommandDate, ListCommandType},
    models::{Todo, self},
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

pub fn print_table_result(
    todos: Result<Vec<Todo>, diesel::result::Error>,
) -> Result<(), io::Error> {
    println!(
        "{0: <4} | {1: <2} | {2: <24} | {3: <10}",
        "done", "id", "content", "date"
    );

    for todo in todos.unwrap() {
        println!("{}", todo);
    }

    Ok(())
}

pub fn print_result<T, E>(result: Result<T, E>) -> Result<(), io::Error> {
    match result {
        Ok(_) => writeln!(io::stdout(), "{}", "Ok!".green()),
        Err(_) => writeln!(io::stdout(), "{}", "Ok!".red()),
    }
}

// TODO: HTML generating function
pub fn generate_html(group: HashMap<NaiveDate, Vec<models::Todo>>) {
    !unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn test_get_date() {
        // Test case for AddCommandDate::Today
        let today = Local::now().date_naive();
        let result = get_date(&AddCommandDate::Today);
        assert_eq!(result.date_naive(), today, "Expected today's date");

        // Test case for AddCommandDate::Tomorrow
        let tomorrow = Local::now().date_naive() + Duration::days(1);
        let result = get_date(&AddCommandDate::Tomorrow);
        assert_eq!(result.date_naive(), tomorrow, "Expected tomorrow's date");

        // Test case for AddCommandDate::Yesterday
        let yesterday = Local::now().date_naive() - Duration::days(1);
        let result = get_date(&AddCommandDate::Yesterday);
        assert_eq!(result.date_naive(), yesterday, "Expected yesterday's date");
    }

    #[test]
    fn test_get_filter_date() {
        // Test case for ListCommandDate::Today
        let today = Local::now().date_naive();
        let result = get_filter_date(&ListCommandDate::Today)
            .unwrap()
            .date_naive();

        assert_eq!(result, today, "Expected Some(today)");

        // Test case for ListCommandDate::Tomorrow
        let tomorrow = (Local::now() + Duration::days(1)).date_naive();
        let result = get_filter_date(&ListCommandDate::Tomorrow)
            .unwrap()
            .date_naive();

        assert_eq!(result, tomorrow, "Expected Some(tomorrow)");

        // Test case for ListCommandDate::Yesterday
        let yesterday = (Local::now() - Duration::days(1)).date_naive();
        let result = get_filter_date(&ListCommandDate::Yesterday)
            .unwrap()
            .date_naive();
        assert_eq!(result, yesterday, "Expected Some(yesterday)");

        // Test case for ListCommandDate::All
        let result = get_filter_date(&ListCommandDate::All);
        assert_eq!(result, None, "Expected None");
    }

    #[test]
    fn test_get_kind() {
        // Test case for ListCommandType::Done
        let result = get_kind(&ListCommandType::Done);
        assert_eq!(result, Some(true), "Expected Some(true)");

        // Test case for ListCommandType::Undone
        let result = get_kind(&ListCommandType::Undone);
        assert_eq!(result, Some(false), "Expected Some(false)");

        // Test case for ListCommandType::All
        let result = get_kind(&ListCommandType::All);
        assert_eq!(result, None, "Expected None");
    }

    #[test]
    fn test_print_table_result() {
        let todos = Ok(vec![
            Todo {
                id: 1,
                completed: true,
                content: "Task 1".to_string(),
                when_will_it_be_done: Local::now().naive_local().into(),
            },
            Todo {
                id: 2,
                completed: false,
                content: "Task 2".to_string(),
                when_will_it_be_done: Local::now().naive_local().into(),
            },
        ]);

        assert_eq!(print_table_result(todos).is_ok(), true);
    }

    #[test]
    fn test_print_result() {
        let result = Ok::<(), ()>(());
        assert_eq!(print_result(result).is_ok(), true);
    }
}
