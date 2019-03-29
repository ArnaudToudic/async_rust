use diesel::prelude::*;
use diesel::PgConnection;

use juniper::{FieldResult, RootNode};

use crate::db::DbConn;
use crate::model::{Ingredient};

pub struct Context {
    pub db_conn: DbConn
}

impl juniper::Context for Context {}

pub struct QueryRoot;

graphql_object!(QueryRoot: Context |&self| {
    /*field ingredients(&executor) -> FieldResult<Vec<Ingredient>> {
        use crate::schema::ingredients;
        use crate::schema::ingredients::dsl;

        let ingredients = dsl::ingredients
            .order(ingredients::id.desc())
            .load::<Ingredient>(&executor.context().db_conn)
            .unwrap();

        Ok(ingredients)
    }*/
});

pub struct MutationRoot;

graphql_object!(MutationRoot: Context |&self| {
    /*field createIngredient(&executor, new_ingredient: NewIngredient) -> FieldResult<Ingredient> {
        use super::schema::ingredients::dsl;
    }*/
});

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {})
}
