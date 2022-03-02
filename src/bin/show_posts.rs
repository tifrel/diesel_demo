extern crate diesel;
extern crate diesel_demo;

use diesel_demo::*;

fn main() {
    let connection = establish_connection();
    let results = read_posts(&connection, 5);

    println!("Displaying {} posts:\n", results.len());
    for post in results {
        println!("{}", post.title);
        println!("{}", "-".repeat(40));
        println!("{}\n", post.body);
    }
}
