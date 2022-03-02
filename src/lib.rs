#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub mod models;
use models::{NewPost, Post};
pub mod schema;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

// CREATE
pub fn create_post<'a>(
    connection: &PgConnection,
    title: &'a str,
    body: &'a str,
) -> Post {
    use schema::posts;

    let new_post = NewPost { title, body };
    diesel::insert_into(posts::table) // specify target table
        .values(&new_post) // specify values to insert
        .get_result(connection) // add `RETURNING *` to the query
        .expect("Error saving new post")
}

// READ
pub fn read_posts(connection: &PgConnection, limit: i64) -> Vec<Post> {
    use schema::posts::dsl::{posts, published};

    posts
        .filter(published.eq(true))
        .limit(limit)
        .load::<Post>(connection)
        .expect("Error loading posts")
}

// UPDATE
pub fn publish_post(connection: &PgConnection, id: i32) -> Post {
    use schema::posts::dsl::{posts, published};

    diesel::update(posts.find(id))
        .set(published.eq(true))
        .get_result::<Post>(connection)
        .expect(&format!("Unable fo find post {}", id))
}

// DELETE
pub fn delete_post(
    connection: &PgConnection,
    title_match: String,
) -> Vec<Post> {
    use schema::posts::dsl::{posts, title};

    // If I used `get_result`, I would only get the first matching post,
    //  but all matching posts would have been deleted
    // If I used `execute`, I would get the number of deleted posts
    diesel::delete(posts.filter(title.like(title_match)))
        .get_results(connection)
        .expect("Error deleting posts")
}
