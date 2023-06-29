use diesel::prelude::*;

use crate::schema::todos;
use crate::models::*;

pub struct TodoRepository;

impl TodoRepository {
    pub fn create() {
        unimplemented!()
    }

    pub fn find(c: &mut SqliteConnection, id: i32) -> QueryResult<Todo> {
        todos::table.find(id).get_result(c)
    }

    pub fn find_all(c: &mut SqliteConnection) -> QueryResult<Vec<Todo>> {
        todos::table.load::<Todo>(c)
    }

    pub fn update() {
        unimplemented!()
    }

    pub fn delete(c: &mut SqliteConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(todos::table.find(id)).execute(c)
    }
}