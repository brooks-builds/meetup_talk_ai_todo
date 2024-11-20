use std::{env, fmt::Display};

use eyre::{Context, Result};
use postgres::Row;
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

pub fn insert(db: &mut Client, name: &str) -> Result<DbTask> {
    let result = db
        .query_one("INSERT INTO tasks (name) values ($1) RETURNING *", &[&name])
        .context("Inserting into database")?;

    Ok(result.into())
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

pub fn update(
    db: &mut Client,
    id: i32,
    name: Option<&str>,
    completed: Option<bool>,
) -> Result<Option<DbTask>> {
    let Some(mut task) = get_task_by_id(db, id)? else {
        return Ok(None);
    };
    if let Some(name) = name {
        if !name.is_empty() {
            task.name = name.to_string();
        }
    }

    if let Some(completed) = completed {
        task.completed = completed;
    }

    let row = db
        .query_one(
            "UPDATE tasks SET (name, completed) = ($1, $2) WHERE id = $3 RETURNING *;",
            &[&task.name, &task.completed, &task.id],
        )
        .context("running update")?;

    Ok(Some(row.into()))
}

pub fn delete(db: &mut Client, id: i32) -> Result<u64> {
    db.execute("DELETE FROM tasks WHERE id = $1", &[&id])
        .context("deleting task from database")
}

pub fn erase(db: &mut Client) -> Result<u64> {
    db.execute("DELETE FROM tasks;", &[])
        .context("Erasing the database")
}

#[derive(Debug)]
pub struct DbTask {
    pub id: i32,
    pub name: String,
    pub completed: bool,
}

impl From<Row> for DbTask {
    fn from(row: Row) -> Self {
        Self {
            id: row.get::<_, i32>("id"),
            name: row.get::<_, String>("name"),
            completed: row.get::<_, bool>("completed"),
        }
    }
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
