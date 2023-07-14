use std::{
    collections::HashMap,
    fs,
    io::{self, Write},
};

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

            let _ = helpers::print_result(result);
        }
        ActionType::Remove(remove_command) => {
            let result = TodoRepository::delete(&mut connection, remove_command.id);

            let _ = helpers::print_result(result);
        }
        ActionType::List(list_command) => {
            let new_date: Option<NaiveDate> =
                helpers::get_filter_date(&list_command.date).map(|date| date.naive_local().into());

            let filter = models::todo::FilterTodo {
                completed: helpers::get_kind(&list_command.kind),
                when_will_it_be_done: new_date,
            };

            let todos = TodoRepository::find_all(&mut connection, filter);

            let _ = helpers::print_table_result(todos);
        }
        ActionType::Done(done_command) => {
            let result = TodoRepository::done(&mut connection, done_command.id);

            let _ = helpers::print_result(result);
        }
        ActionType::Sync => {
            let result = TodoRepository::sync(&mut connection);

            let _ = helpers::print_result(result);
        }
        ActionType::Export(export_command) => {
            let _command = export_command;
            let mut groups: HashMap<NaiveDate, Vec<models::Todo>> = HashMap::new();

            let filter = models::todo::FilterTodo {
                completed: None,
                when_will_it_be_done: None,
            };

            let todos = TodoRepository::find_all(&mut connection, filter);

            match todos {
                Err(e) => println!("{:?}", e),
                Ok(result) => {
                    result.into_iter().for_each(|todo| {
                        let group = groups.entry(todo.when_will_it_be_done).or_insert(vec![]);
                        group.push(todo);
                    });
                }
            }

            let content = helpers::generate_html(groups);

            fs::write("index.html", content).expect("Unable to write file!");

            let _ = writeln!(io::stdout(), "{}", "Ok!".green());
        }
    }
}
