use std::env;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

pub fn establish_connection() -> SqliteConnection {
    let db_url = env::var("DATABASE_URL").expect("Please set DATABASE_URL");
    SqliteConnection::establish(&db_url).expect(&format!("Error connecting to {}", db_url))
}
