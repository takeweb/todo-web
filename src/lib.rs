use actix_web::{get, post, web, HttpResponse};
use askama::Template;
use askama_actix::TemplateToResponse;
use chrono::{Duration, Utc};
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
    due_at: Option<String>,
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
    let unstarted_rows = sqlx::query("SELECT id, task FROM tasks WHERE status = 0 ORDER BY id;")
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
    let in_progress_rows = sqlx::query("SELECT id, task FROM tasks WHERE status = 1 ORDER BY id;")
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
    let completed_rows = sqlx::query("SELECT id, task FROM tasks WHERE status = 9 ORDER BY id;")
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
    let default_due_days: i64 = env::var("DEFAULT_DUE_DAYS") // 環境変数を取得
        .unwrap_or_else(|_| "30".to_string()) // デフォルト値を30に設定
        .parse() // 数値に変換
        .unwrap_or(30); // 変換失敗時もデフォルト値を使用

    // 作成ボタン押下時
    match task.task {
        Some(task_value) if !task_value.is_empty() => {
            // due_atのデフォルト値を外部設定値を使って計算
            let due_at_value = task.due_at.unwrap_or_else(|| {
                (Utc::now() + Duration::days(default_due_days))
                    .format("%Y-%m-%d %H:%M:%S")
                    .to_string()
            });
            sqlx::query("INSERT INTO tasks (task, status, due_at) VALUES (?, 0, ?)")
                .bind(task_value)
                .bind(due_at_value)
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

#[post("/undo")]
pub async fn undo(pool: web::Data<SqlitePool>, form: web::Form<Task>) -> HttpResponse {
    let task = form.into_inner();

    // 戻す(仕掛かり中→未着手)ボタン押下時
    if let Some(id) = task.id {
        sqlx::query("UPDATE tasks SET status = 0, started_at = CURRENT_TIMESTAMP WHERE id = ?")
            .bind(id)
            .execute(pool.as_ref())
            .await
            .unwrap();
    }

    HttpResponse::Found()
        .append_header(("Location", "/"))
        .finish()
}

#[post("/doing")]
pub async fn doing(pool: web::Data<SqlitePool>, form: web::Form<Task>) -> HttpResponse {
    let task = form.into_inner();

    // 戻す(完了→仕掛かり中)ボタン押下時
    if let Some(id) = task.id {
        sqlx::query("UPDATE tasks SET status = 1, done_at = NULL WHERE id = ?")
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

    // 削除ボタン押下時
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
