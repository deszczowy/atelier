use rusqlite::{Connection, Result};
use rusqlite::NO_PARAMS;

pub struct Database {
    connection: Connection
}

pub trait Queryable {
    fn new(file: String) -> Database;
}

impl Queryable for Database {
    fn new(file: String) -> Database {
        Database {
            connection: Connection::open(file).unwrap()
        }
    }
}