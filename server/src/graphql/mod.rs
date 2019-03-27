use diesel::prelude::*;
use diesel::PgConnection;

use juniper::{FieldResult, RootNode};

use super::model::{Ingredient, NewIngredient};

pub struct Context {
    pub db_conn: PgConnection
}

impl juniper::Context for Context {}

pub struct QueryRoot;

graphql_object!(QueryRoot: Context |&self| {
    field ingredients(&executor) -> FieldResult<Vec<Ingredient>> {
        use super::schema::ingredients::dsl;

        dsl::ingredients
            .order(ingredients::id.desc())
            .load::<Ingredient>(&executor.context().db_conn)
            .unwrap()
    }
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
