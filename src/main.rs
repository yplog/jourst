use args::Cli;
use clap::Parser;

mod models;
mod args;
mod schema;
mod db;
mod repositories;

fn main() {
    let mut connection = db::establish_connection();

    db::run_migrations(&mut connection).unwrap();

    let todos = repositories::todo_repository::TodoRepository::find_all(&mut connection);

    println!("{:?}", todos);

    let matches = Cli::parse();
    println!("{:?}", matches);
}
