use actix_web::web;
use sqlx::SqlitePool;
use sqlx::{sqlite::SqliteQueryResult, Pool, Row, Sqlite};

#[derive(serde::Deserialize)]
pub struct TaskRegisterd {
    pub id: i64,
    pub task: String,
    pub created_at: Option<String>,
    pub due_at: Option<String>,
    pub started_at: Option<String>,
    pub done_at: Option<String>,
}

pub async fn init_db_pool(database_url: &str) -> Pool<Sqlite> {
    // 接続プールの作成
    SqlitePool::connect(database_url).await.unwrap()
}

pub async fn get_task_list(pool: &web::Data<SqlitePool>, status: i32) -> Vec<TaskRegisterd> {
    let sql = "SELECT id, task, created_at, due_at, started_at, done_at FROM tasks WHERE status = ? ORDER BY id;";
    let rows = sqlx::query(sql)
        .bind(status)
        .fetch_all(pool.as_ref())
        .await
        .unwrap();
    let tasks: Vec<TaskRegisterd> = rows
        .iter()
        .map(|row| TaskRegisterd {
            id: row.get::<i64, _>("id"),
            task: row.get::<String, _>("task"),
            created_at: Some(row.get::<String, _>("created_at")),
            due_at: Some(row.get::<String, _>("due_at")),
            started_at: Some(row.get::<String, _>("started_at")),
            done_at: Some(row.get::<String, _>("done_at")),
        })
        .collect();
    tasks
}

pub async fn add_task(
    pool: &SqlitePool,
    task_value: String,
    status: i32,
    due_at_value: String,
) -> Result<i64, sqlx::Error> {
    // SQL文の実行
    sqlx::query("INSERT INTO tasks (task, status, due_at) VALUES (?, ?, ?)")
        .bind(task_value)
        .bind(status)
        .bind(due_at_value)
        .execute(pool)
        .await?;

    // last_insert_rowidを取得
    let id = sqlx::query_scalar("SELECT last_insert_rowid()")
        .fetch_one(pool)
        .await?;

    Ok(id)
}

pub async fn start_task(pool: &SqlitePool, id: String, status: i32) -> SqliteQueryResult {
    let sql = "UPDATE tasks SET status = ?, started_at = DATETIME(CURRENT_TIMESTAMP, '+9 hours') WHERE id = ?";
    sqlx::query(sql)
        .bind(status)
        .bind(id)
        .execute(pool)
        .await
        .unwrap()
}

pub async fn done_task(pool: &SqlitePool, id: String, status: i32) -> SqliteQueryResult {
    let sql = "UPDATE tasks SET status = ?, done_at = DATETIME(CURRENT_TIMESTAMP, '+9 hours') WHERE id = ?";
    sqlx::query(sql)
        .bind(status)
        .bind(id)
        .execute(pool)
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
pub async fn undo_task(pool: &SqlitePool, id: String, status: i32) -> SqliteQueryResult {
    let sql = "UPDATE tasks SET status = ?, started_at = NULL WHERE id = ?";
    sqlx::query(sql)
        .bind(status)
        .bind(id)
        .execute(pool)
        .await
        .unwrap()
}

pub async fn doing_task(pool: &SqlitePool, id: String, status: i32) -> SqliteQueryResult {
    let sql = "UPDATE tasks SET status = ?, done_at = NULL WHERE id = ?";
    sqlx::query(sql)
        .bind(status)
        .bind(id)
        .execute(pool)
        .await
        .unwrap()
}

pub async fn remove_task(pool: &SqlitePool, id: String) -> SqliteQueryResult {
    let sql = "DELETE FROM tasks WHERE id = ?";
    sqlx::query(sql).bind(id).execute(pool).await.unwrap()
}
