use std::env;

use eyre::{Context, Result};
pub use postgres::{Client, NoTls};

pub fn connect() -> Result<Client> {
    dotenvy::dotenv().ok();

    let database_url =
        env::var("DATABASE_URL").context("Getting database url from environment variable")?;
    Client::connect(&database_url, NoTls).context("connecting to postgres database")
}

pub fn run_query(db: &mut Client, sql: &str) -> Result<String> {
    let result = db.query(sql, &[]).context("running query")?;

    Ok(format!(
        "The SQL query ```{sql}``` resulted in ```{result:?}``` from the database"
    ))
}

pub fn insert(db: &mut Client, name: &str) -> Result<i32> {
    let result = db
        .query_one(
            "INSERT INTO tasks (name) values ($1) RETURNING id",
            &[&name],
        )
        .context("Inserting into database")?;
    let created_id = result.get::<_, i32>("id");

    Ok(created_id)
}
