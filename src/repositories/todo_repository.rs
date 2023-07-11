use chrono::Local;
use diesel::prelude::*;
use std::error::Error;

use crate::models::todo::{FilterTodo, NewTodo};
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

    pub fn find_all(c: &mut SqliteConnection, filter: FilterTodo) -> QueryResult<Vec<Todo>> {
        let mut query = todos::table.into_boxed();

        if let Some(completed) = filter.completed {
            query = query.filter(todos::completed.eq(completed));
        }

        if let Some(when_will_it_be_done) = filter.when_will_it_be_done {
            query = query.filter(todos::when_will_it_be_done.eq(when_will_it_be_done));
        }

        query.load::<Todo>(c)
    }

    pub fn done(c: &mut SqliteConnection, id: i32) -> Result<(), Box<dyn Error>> {
        match diesel::update(todos::table.find(id))
            .set(todos::completed.eq(true))
            .execute(c)
        {
            Ok(_) => Ok(()),
            Err(err) => Err(Box::new(err)),
        }
    }

    pub fn delete(c: &mut SqliteConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(todos::table.find(id)).execute(c)
    }

    pub fn sync(c: &mut SqliteConnection) -> Result<(), Box<dyn Error>> {
        let today = Local::now().naive_local().date();

        match diesel::update(todos::table)
            .filter(
                todos::when_will_it_be_done
                    .lt(today)
                    .and(todos::completed.eq(false)),
            )
            .set(todos::when_will_it_be_done.eq(today))
            .execute(c)
        {
            Ok(_) => Ok(()),
            Err(err) => Err(Box::new(err)),
        }
    }

    pub fn export_command(c: &mut SqliteConnection) -> QueryResult<Vec<Todo>> {
        // TODO: Implement
        !unimplemented!()
    }
}
