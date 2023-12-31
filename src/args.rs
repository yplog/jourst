use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(ValueEnum, Clone, PartialEq, Debug)]
pub enum Action {
    Add,
    Remove,
    List,
    Done,
    Sync,
    Export,
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

#[derive(ValueEnum, Clone, PartialEq, Debug)]
pub enum ExportCommandType {
    Markdown,
    Html,
}

#[derive(ValueEnum, Clone, PartialEq, Debug)]
pub enum RemoveCommandType {
    ID,
    ALL,
}

#[derive(Parser, Debug)]
#[clap(author, about = "Simple command line task manager", version)]
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

    /// Export todos
    Export(ExportCommand),

    /// Carries the to do that were not done on time to today.
    Sync,
}

#[derive(Debug, Args)]
pub struct AddCommand {
    /// The content of the todo.
    #[clap(short = 'c', long = "content")]
    pub content: String,

    /// The date of the todo.
    #[clap(short = 'd', long = "date", default_value = "today")]
    pub date: AddCommandDate,
}

#[derive(Debug, Args)]
pub struct RemoveCommand {
    /// Remove Command Type
    #[clap(short = 't', long = "type")]
    pub kind: RemoveCommandType,

    /// The id of the todo.
    #[clap(short = 'i', long = "id", default_value = "-1")]
    pub id: i32,
}

#[derive(Debug, Args)]
pub struct ListCommand {
    /// The type of the todo.
    #[clap(short = 'k', long = "kind", default_value = "all")]
    pub kind: ListCommandType,

    /// The date of the todo.
    #[clap(short = 'd', long = "date", default_value = "today")]
    pub date: ListCommandDate,
}

#[derive(Debug, Args)]
pub struct ExportCommand {
    #[clap(short = 't', long = "type", default_value = "html")]
    pub export_type: ExportCommandType,
}

#[derive(Debug, Args)]
pub struct DoneCommand {
    /// The id of the todo.
    #[clap(short = 'i', long = "id")]
    pub id: i32,
}
