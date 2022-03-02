# Getting started with diesel.rs

Some notes and the corresponding sandbox repo for the
[getting started guide of diesel.rs](https://diesel.rs/guides/getting-started).
This digest assumes that you have postgres set up already.

## Setup

### Installing diesel

Install with postgres feature (by default it will require)

```sh
cargo install diesel_cli --no-default-features --features postgres
```

Dependencies in for Rust library:

```toml
[dependencies]
diesel = { version = "1.4.4", features = ["postgres"] }
dotenv = "0.15.0" # not strictly required, but very helpful
```

### Running Postgres

Clearing dirty repo:

```sh
pg_ctl -D ./data stop
rm -rf data postgres.log
mkdir data
```

Running new DB and following logs:

```sh
initdb ./data
pg_ctl -D ./data -l postgres.log start
tail -F postgres.log
```

### Initializing diesel and database

```sh
diesel setup
```

The diesel setup will

- create a `migrations` directory including the `diesel_initial_setup` migration
- create `diesel.toml`, which sets up the auto-generation of the `src/schema.rs`

To initialize the DB run:

```sh
diesel migration generate init_db
```

This will create another migration called `init_db` at
`migrations/<timestamp>_init_db` with files `up.sql` (performing the migration)
and `down.sql` (undoing the migration). These files are empty and will contain
handcrafted migrations. Apply them using:

```sh
diesel migration run
```

You can double check that your migrations are working by:

```sh
diesel migration redo
```

This will undo the migration, and then re-apply it. If everything goes well, it
means that `up.sql` and `down.sql` are compatible with one another.

## Interacting

### Schemas

This is the easy part. The database schema (in rustland ) will be auto-generated
by diesel.rs based on the migrations. Find it in `schema.rs` (default).

### Models for Querying

These will have to be handcrafted. Given a table creation migration:

```sql
CREATE TABLE posts (
  id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL,
  body TEXT NOT NULL,
  published BOOLEAN NOT NULL DEFAULT 'f'
)
```

This will translate into the following Rust model:

```rs
#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}
```

The derive macro for `Queryable` requires you to define struct fields in the
same order in which you defined table columns. The most accurate resource on
that is the table definition inside `src/schema.rs`.

### Models for Inserting

```rs
use crate::schema::posts;

#[derive(Insertable)]
#[table_name = "posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}

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
```

If you don't need any result, you can replace
`.get_result(connection).expect(...)` by a simple `.execute()`
