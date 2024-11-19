use std::{env, fmt::Display};

use eyre::{Context, Result};
use postgres::GenericClient;
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

pub fn get_all_tasks(db: &mut Client) -> Result<Vec<DbTask>> {
    let results = db
        .query("SELECT * FROM tasks;", &[])
        .context("running query")?;
    Ok(results
        .into_iter()
        .map(|row| DbTask {
            id: row.get::<_, i32>("id"),
            name: row.get::<_, String>("name"),
            completed: row.get::<_, bool>("completed"),
        })
        .collect())
}

pub fn get_task_by_id(db: &mut Client, id: i32) -> Result<Option<DbTask>> {
    let Some(row) = db
        .query_opt("SELECT * FROM tasks WHERE id = $1;", &[&id])
        .context("running query")?
    else {
        return Ok(None);
    };

    let task = DbTask {
        id: row.get::<_, i32>("id"),
        name: row.get::<_, String>("name"),
        completed: row.get::<_, bool>("completed"),
    };
    Ok(Some(task))
}

pub struct DbTask {
    pub id: i32,
    pub name: String,
    pub completed: bool,
}

impl Display for DbTask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "id: {}, name: {}, completed: {}",
            self.id, self.name, self.completed
        )
    }
}
