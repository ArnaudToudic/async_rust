use diesel::prelude::*;
use diesel::PgConnection;

use dotenv::dotenv;

use std::env;
use std::error::Error;

pub fn establish_connection() -> Option<PgConnection> {
    dotenv().expect("No .env file found.");

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    match PgConnection::establish(&database_url) {
        Ok(connection) => Some(connection),
        Err(error) => {
            error!("Error connecting to database: {}", error.description());
            None
        }
    }
}
