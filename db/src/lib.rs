use std::env;

use eyre::{Context, Result};
use postgres::{Client, GenericClient, NoTls};

pub fn connect() -> Result<Client> {
    dotenvy::dotenv().ok();

    let database_url =
        env::var("DATABASE_URL").context("Getting database url from environment variable")?;
    Client::connect(&database_url, NoTls).context("connecting to postgres database")
}

pub fn run_query(db: &mut Client, sql: &str) -> Result<String> {
    let result = db.simple_query(sql).context("running query")?;

    println!("\n***The result of the query is: '{result:?}'***\n");

    Ok(format!("{result:?}"))
}
