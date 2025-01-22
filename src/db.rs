use actix_web::web;
use sqlx::{sqlite::SqliteQueryResult, Row, SqlitePool};

#[derive(serde::Deserialize)]
pub struct TaskRegisterd {
    pub id: i32,
    pub task: String,
}

pub async fn get_task_list(pool: &web::Data<SqlitePool>, status: i32) -> Vec<TaskRegisterd> {
    let sql = "SELECT id, task FROM tasks WHERE status = ? ORDER BY id;";
    let rows = sqlx::query(sql)
        .bind(status)
        .fetch_all(pool.as_ref())
        .await
        .unwrap();
    let tasks: Vec<TaskRegisterd> = rows
        .iter()
        .map(|row| TaskRegisterd {
            id: row.get::<i32, _>("id"),
            task: row.get::<String, _>("task"),
        })
        .collect();
    tasks
}

pub async fn add_task(
    pool: &web::Data<SqlitePool>,
    task_value: String,
    due_at_value: String,
) -> SqliteQueryResult {
    let sql = "INSERT INTO tasks (task, status, due_at) VALUES (?, 0, ?)";
    sqlx::query(sql)
        .bind(task_value)
        .bind(due_at_value)
        .execute(pool.as_ref())
        .await
        .unwrap()
}

pub async fn start_task(pool: &web::Data<SqlitePool>, id: String) -> SqliteQueryResult {
    let sql = "UPDATE tasks SET status = 1, started_at = DATETIME(CURRENT_TIMESTAMP, '+9 hours') WHERE id = ?";
    sqlx::query(sql)
        .bind(id)
        .execute(pool.as_ref())
        .await
        .unwrap()
}

pub async fn done_task(pool: &web::Data<SqlitePool>, id: String) -> SqliteQueryResult {
    let sql = "UPDATE tasks SET status = 9, done_at = DATETIME(CURRENT_TIMESTAMP, '+9 hours') WHERE id = ?";
    sqlx::query(sql)
        .bind(id)
        .execute(pool.as_ref())
        .await
        .unwrap()
}

/// Reverts the status of a task to "not started".
///
/// This function updates the `status` of a task in the `tasks` table to 0 and sets
/// the `started_at` column to NULL, effectively marking the task as not started.
///
/// # Arguments
/// - `pool`: A reference to the `SqlitePool` wrapped in Actix Web's `web::Data`.
/// - `id`: The unique identifier of the task to be undone.
///
/// # Returns
/// Returns a `SqliteQueryResult` representing the result of the SQL query.
///
/// # Errors
/// This function will panic if the SQL query execution fails.
///
/// Note: This function is intended to be used within an Actix Web application,
/// where the `SqlitePool` is properly configured and managed by the framework.
pub async fn undo_task(pool: &web::Data<SqlitePool>, id: String) -> SqliteQueryResult {
    let sql = "UPDATE tasks SET status = 0, started_at = NULL WHERE id = ?";
    sqlx::query(sql)
        .bind(id)
        .execute(pool.as_ref())
        .await
        .unwrap()
}

pub async fn doing_task(pool: &web::Data<SqlitePool>, id: String) -> SqliteQueryResult {
    let sql = "UPDATE tasks SET status = 1, done_at = NULL WHERE id = ?";
    sqlx::query(sql)
        .bind(id)
        .execute(pool.as_ref())
        .await
        .unwrap()
}

pub async fn remove_task(pool: &web::Data<SqlitePool>, id: String) -> SqliteQueryResult {
    let sql = "DELETE FROM tasks WHERE id = ?";
    sqlx::query(sql)
        .bind(id)
        .execute(pool.as_ref())
        .await
        .unwrap()
}
