use args::Cli;
use chrono::NaiveDate;
use clap::Parser;
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
    println!("{:?}", matches);

    match matches.action {
        ActionType::Add(add_command) => {
            let date = helpers::get_date(&add_command.date);
            println!("Add command: {:?}", add_command);
            println!("Date: {:?}", date);

            let todo = NewTodo {
                content: add_command.content,
                when_will_it_be_done: date.naive_local().into(),
            };

            let _ = TodoRepository::create(&mut connection, todo);
        }
        ActionType::Remove(remove_command) => {
            println!("Remove command: {:?}", remove_command);

            let _ = TodoRepository::delete(&mut connection, remove_command.id);
        }
        ActionType::List(list_command) => {
            let new_date: Option<NaiveDate> =
                helpers::get_filter_date(&list_command.date).map(|date| date.naive_local().into());

            let filter = models::todo::FilterTodo {
                completed: helpers::get_kind(&list_command.kind),
                when_will_it_be_done: new_date,
            };

            let todos = TodoRepository::find_all(
                &mut connection,
                filter,
            );

            println!("TODOS: {:?}", todos);
        }
        ActionType::Done(done_command) => {
            println!("Done command: {:?}", done_command);

            let _ = TodoRepository::done(&mut connection, done_command.id);
        }
    }
}
