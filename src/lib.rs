pub mod models;
pub mod schema;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use self::models::{NewHody, Hodies};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_post(conn: &mut PgConnection, name: &str, description: &str) -> Post {
    use crate::schema::posts;

    let new_hoody = NewHody { name, description };

    diesel::insert_into(hodies::table)
        .values(&new_hoody)
        .returning(Post::as_returning())
        .get_result(conn)
        .expect("Error saving new hoody")
}