use args::Cli;
use chrono::NaiveDate;
use clap::Parser;
use colored::Colorize;
use repositories::todo_repository::TodoRepository;

use crate::{args::ActionType, models::todo::NewTodo};

mod args;
mod db;
mod helpers;
mod models;
mod repositories;
mod schema;

fn main() {
    let mut connection = db::establish_connection();

    db::run_migrations(&mut connection).unwrap();

    let matches = Cli::parse();

    match matches.action {
        ActionType::Add(add_command) => {
            let date = helpers::get_date(&add_command.date);

            let todo = NewTodo {
                content: add_command.content,
                when_will_it_be_done: date.naive_local().into(),
            };

            let result = TodoRepository::create(&mut connection, todo);

            match result {
                Ok(_) => println!("{}", "Ok!".green()),
                Err(_) => println!("{}", "Error!".red()),
            }
        }
        ActionType::Remove(remove_command) => {
            let result = TodoRepository::delete(&mut connection, remove_command.id);

            match result {
                Ok(_) => println!("{}", "Ok!".green()),
                Err(_) => println!("{}", "Error!".red()),
            }
        }
        ActionType::List(list_command) => {
            let new_date: Option<NaiveDate> =
                helpers::get_filter_date(&list_command.date).map(|date| date.naive_local().into());

            let filter = models::todo::FilterTodo {
                completed: helpers::get_kind(&list_command.kind),
                when_will_it_be_done: new_date,
            };

            let todos = TodoRepository::find_all(&mut connection, filter);

            println!(
                "{0: <4} | {1: <2} | {2: <24} | {3: <10}",
                "done", "id", "content", "date"
            );

            for todo in todos.unwrap() {
                println!("{}", todo);
            }
        }
        ActionType::Done(done_command) => {
            let result = TodoRepository::done(&mut connection, done_command.id);

            match result {
                Ok(_) => println!("{}", "Ok!".green()),
                Err(_) => println!("{}", "Error!".red()),
            }
        }
    }
}
