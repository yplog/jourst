// Example: jourst <ACTION> <PATTERN> <PATH>
// Example: jourst add "My first journal" "This is my first journal."

use clap::{ValueEnum, Parser, Subcommand, Args};

#[derive(ValueEnum, Clone, PartialEq, Debug)]
pub enum Action {
    Add,
    Remove,
    List,
    Done,
}

#[derive(ValueEnum, Clone, PartialEq, Debug)]
pub enum AddCommandDate {
    Today,
    Tomorrow,
    Yesterday,
}

#[derive(ValueEnum, Clone, PartialEq, Debug)]
pub enum ListCommandDate {
    Today,
    Tomorrow,
    Yesterday,
    All,
}

#[derive(ValueEnum, Clone, PartialEq, Debug)]
pub enum ListCommandType {
    All,
    Done,
    Undone,
}

#[derive(Parser, Debug)]
#[clap(author, about, version)]
pub struct Cli {
    #[clap(subcommand)]
    pub action: ActionType,
}

#[derive(Debug, Subcommand)]
pub enum ActionType {
    /// Add a new todo.
    Add(AddCommand),

    /// Remove a todo.
    Remove(RemoveCommand),

    /// List all todos.
    List(ListCommand),

    /// Mark a todo as done.
    Done(DoneCommand),
}

#[derive(Debug, Args)]
pub struct AddCommand {
    /// The title of the todo.
    pub content: String,

    /// The date of the todo.
    pub date: AddCommandDate,
}

#[derive(Debug, Args)]
pub struct RemoveCommand {
    /// The id of the todo.
    pub id: u32,
}

#[derive(Debug, Args)]
pub struct ListCommand {
    /// The type of the todo.
    pub kind: ListCommandType,

    /// The date of the todo.
    pub date: ListCommandDate,
}

#[derive(Debug, Args)]
pub struct DoneCommand {
    /// The id of the todo.
    pub id: u32,
}