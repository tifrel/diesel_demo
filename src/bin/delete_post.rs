extern crate diesel;
extern crate diesel_demo;

use diesel_demo::*;

fn main() {
    let title_match = std::env::args()
        .nth(1)
        .expect("I require an argument to match against the post title");
    let connection = establish_connection();

    let title_match = format!("%{}%", title_match);

    let posts = delete_post(&connection, title_match);
    for post in posts {
        println!("Deleted post '{}'", post.title);
    }
}
