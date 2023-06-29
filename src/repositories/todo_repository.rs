use std::error::Error;

use diesel::prelude::*;

use crate::models::todo::NewTodo;
use crate::models::*;
use crate::schema::todos;

pub struct TodoRepository;

impl TodoRepository {
    pub fn create(c: &mut SqliteConnection, new_todo: NewTodo) -> Result<(), Box<dyn Error>> {
        match diesel::insert_into(todos::table)
            .values(&new_todo)
            .execute(c)
        {
            Ok(_) => Ok(()),
            Err(err) => Err(Box::new(err)),
        }
    }

    pub fn find(c: &mut SqliteConnection, id: i32) -> QueryResult<Todo> {
        todos::table.find(id).get_result(c)
    }

    pub fn find_all(c: &mut SqliteConnection) -> QueryResult<Vec<Todo>> {
        todos::table.load::<Todo>(c)
    }

    pub fn update(c: &mut SqliteConnection, id: i32, todo: Todo) -> Result<(), Box<dyn Error>> {
        match  {
            diesel::update(todos::table.find(id))
                .set((
                    todos::content.eq(todo.content),
                    todos::completed.eq(todo.completed),
                    todos::when_will_it_be_done.eq(todo.when_will_it_be_done),
                ))
                .execute(c)
        } {
            Ok(_) => Ok(()),
            Err(err) => Err(Box::new(err)),
        }
    }

    pub fn delete(c: &mut SqliteConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(todos::table.find(id)).execute(c)
    }
}
