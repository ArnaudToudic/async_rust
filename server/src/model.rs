#[derive(Queryable)]
pub struct Ingredient {
    pub id: i32,
    pub name: String,
    pub quantity: i32
}
