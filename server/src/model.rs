use crate::schema::ingredients;

#[derive(Queryable)]
pub struct Ingredient {
    pub id: i32,
    pub name: String,
    pub quantity: i32
}

#[derive(Insertable)]
#[table_name="ingredients"]
pub struct NewIngredient<'a> {
    pub name: &'a str,
    pub quantity: i32
}
