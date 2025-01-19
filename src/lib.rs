use actix_web::{get, post, web, HttpResponse};
use askama::Template;
use askama_actix::TemplateToResponse;
use sqlx::{Pool, Row, Sqlite, SqlitePool};
use std::env;

#[derive(Template)]
#[template(path = "todo.html")]
struct TodoTemplate {
    unstarted_tasks: Vec<TaskRegisterd>,
    in_progress_tasks: Vec<TaskRegisterd>,
    completed_tasks: Vec<TaskRegisterd>,
}

#[derive(serde::Deserialize)]
struct Task {
    id: Option<String>,
    task: Option<String>,
}

#[derive(serde::Deserialize)]
struct TaskRegisterd {
    id: i32,
    task: String,
}

pub async fn init_db_pool() -> Pool<Sqlite> {
    // .envの読み込み
    dotenv::dotenv().expect(".envの読み込み失敗");

    // DATABASE_URLの取得
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URLがセットされていません");

    // 接続プールの作成
    SqlitePool::connect(&database_url).await.unwrap()
}

#[get("/")]
pub async fn todo(pool: web::Data<SqlitePool>) -> HttpResponse {
    // 未着手のタスクを取得
    let unstarted_rows = sqlx::query("SELECT id, task FROM tasks WHERE status = 0;")
        .fetch_all(pool.as_ref())
        .await
        .unwrap();
    let unstarted_tasks: Vec<TaskRegisterd> = unstarted_rows
        .iter()
        .map(|row| TaskRegisterd {
            id: row.get::<i32, _>("id"),
            task: row.get::<String, _>("task"),
        })
        .collect();

    // 仕掛かり中のタスクを取得
    let in_progress_rows = sqlx::query("SELECT id, task FROM tasks WHERE status = 1;")
        .fetch_all(pool.as_ref())
        .await
        .unwrap();
    let in_progress_tasks: Vec<TaskRegisterd> = in_progress_rows
        .iter()
        .map(|row| TaskRegisterd {
            id: row.get::<i32, _>("id"),
            task: row.get::<String, _>("task"),
        })
        .collect();

    // 完了タスクを取得
    let completed_rows = sqlx::query("SELECT id, task FROM tasks WHERE status = 9;")
        .fetch_all(pool.as_ref())
        .await
        .unwrap();
    let completed_tasks: Vec<TaskRegisterd> = completed_rows
        .iter()
        .map(|row| TaskRegisterd {
            id: row.get::<i32, _>("id"),
            task: row.get::<String, _>("task"),
        })
        .collect();

    let todo = TodoTemplate {
        unstarted_tasks,
        in_progress_tasks,
        completed_tasks,
    };

    todo.to_response()
}

#[post("/create")]
pub async fn create(pool: web::Data<SqlitePool>, form: web::Form<Task>) -> HttpResponse {
    let task = form.into_inner();

    // 作成ボタン押下時
    match task.task {
        Some(task) if !task.is_empty() => {
            sqlx::query("INSERT INTO tasks (task, status) VALUES (?, 0)")
                .bind(task)
                .execute(pool.as_ref())
                .await
                .unwrap();
        }
        _ => {}
    }

    HttpResponse::Found()
        .append_header(("Location", "/"))
        .finish()
}

#[post("/start")]
pub async fn start(pool: web::Data<SqlitePool>, form: web::Form<Task>) -> HttpResponse {
    let task = form.into_inner();

    // 開始ボタン押下時
    if let Some(id) = task.id {
        sqlx::query("UPDATE tasks SET status = 1, started_at = CURRENT_TIMESTAMP WHERE id = ?")
            .bind(id)
            .execute(pool.as_ref())
            .await
            .unwrap();
    }

    HttpResponse::Found()
        .append_header(("Location", "/"))
        .finish()
}

#[post("/done")]
pub async fn done(pool: web::Data<SqlitePool>, form: web::Form<Task>) -> HttpResponse {
    let task = form.into_inner();

    // 完了ボタン押下時
    if let Some(id) = task.id {
        sqlx::query("UPDATE tasks SET status = 9, done_at = CURRENT_TIMESTAMP WHERE id = ?")
            .bind(id)
            .execute(pool.as_ref())
            .await
            .unwrap();
    }

    HttpResponse::Found()
        .append_header(("Location", "/"))
        .finish()
}

#[post("/delete")]
pub async fn delete(pool: web::Data<SqlitePool>, form: web::Form<Task>) -> HttpResponse {
    let task = form.into_inner();

    // 完了ボタン押下時
    if let Some(id) = task.id {
        sqlx::query("DELETE FROM tasks WHERE id = ?")
            .bind(id)
            .execute(pool.as_ref())
            .await
            .unwrap();
    }

    HttpResponse::Found()
        .append_header(("Location", "/"))
        .finish()
}
