use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = hodies)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct hodies {
    id: i32,
    name: String,
    description: String,
    price: f64,
    image_url: Option<String>,
    brand: String,
    category: String,
}

#[derive(Insertable)]
#[diesel(table_name = hodies)]
pub struct NewHody<'a> {
    pub name: &'a str,
    pub description: &'a str,
}