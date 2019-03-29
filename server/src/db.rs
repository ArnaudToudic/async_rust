use diesel::PgConnection;
use diesel::r2d2;
use diesel::r2d2::ConnectionManager;

use dotenv::dotenv;

use std::ops::Deref;
use std::env;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn init_pool() -> Pool {
    dotenv().expect("No .env file found.");

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);

    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}

pub struct DbConn(r2d2::PooledConnection<ConnectionManager<PgConnection>>);

pub fn establish_connection(pool: &Pool) -> Option<DbConn> {
    match pool.get() {
        Ok(conn) => Some(DbConn(conn)),
        Err(_) => None
    }
}

impl Deref for DbConn {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
