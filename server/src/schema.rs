use diesel::PgConnection;

use juniper::{Context};

use crate::model::{Ingredient};

pub struct Database {
    pub db_connection: PgConnection
}

impl Context for Database {}

graphql_object!(Ingredient: Database |&self| {
    field id() -> &i32 {
        &self.id
    }

    field name() -> &String {
        &self.name
    }

    field quantity() -> &i32 {
        &self.quantity
    }
});
