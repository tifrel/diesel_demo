extern crate diesel;
extern crate diesel_demo;

use diesel_demo::*;

fn main() {
    let id = std::env::args()
        .nth(1)
        .expect("I require a post ID")
        .parse()
        .expect("ID must be an i32");
    let connection = establish_connection();

    let post = publish_post(&connection, id);
    println!("Published post '{}'", post.title);
}
