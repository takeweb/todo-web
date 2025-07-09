use actix_web::{get, post, web, HttpResponse};
use askama::Template;
use askama_actix::TemplateToResponse;
use chrono::{Duration, Local};
use db::TaskRegisterd;
use sqlx::{Pool, Sqlite, SqlitePool};
use std::env;
pub mod db;

#[derive(serde::Deserialize)]
struct Task {
    id: Option<i64>,
    task: Option<String>,
    due_at: Option<String>,
}

#[derive(Template)]
#[template(path = "todo.html")]
struct TodoTemplate {
    unstarted_tasks: Vec<TaskRegisterd>,
    in_progress_tasks: Vec<TaskRegisterd>,
    completed_tasks: Vec<TaskRegisterd>,
}

#[derive(Debug, PartialEq, Eq, sqlx::Type)]
#[repr(i32)]
enum TaskStatus {
    NotStarted = 0, // 未着手
    InProgress = 1, // 仕掛かり中
    Completed = 9,  // 完了
}
impl From<TaskStatus> for i32 {
    fn from(status: TaskStatus) -> Self {
        status as i32
    }
}

pub async fn init_db_pool() -> Pool<Sqlite> {
    // .envの読み込み
    dotenv::dotenv().expect(".envの読み込み失敗");

    // DATABASE_URLの取得
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URLがセットされていません");

    // 接続プールの作成
    db::init_db_pool(&database_url).await
}

#[get("/")]
pub async fn todo(pool: web::Data<SqlitePool>) -> HttpResponse {
    // 未着手のタスクを取得
    let unstarted_tasks = db::get_task_list(&pool, TaskStatus::NotStarted.into()).await;

    // 仕掛かり中のタスクを取得
    let in_progress_tasks = db::get_task_list(&pool, TaskStatus::InProgress.into()).await;

    // 完了タスクを取得
    let completed_tasks = db::get_task_list(&pool, TaskStatus::Completed.into()).await;

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
                (Local::now() + Duration::days(default_due_days))
                    .format("%Y-%m-%d %H:%M:%S")
                    .to_string()
            });
            let _id = db::add_task(pool.as_ref(), task_value, due_at_value).await;
        }
        _ => {}
    }

    HttpResponse::Found()
        .append_header(("Location", "/todo_new/"))
        .finish()
}

#[post("/start")]
pub async fn start(pool: web::Data<SqlitePool>, form: web::Form<Task>) -> HttpResponse {
    let task = form.into_inner();

    // 開始ボタン押下時
    if let Some(id) = task.id {
        db::start_task(pool.as_ref(), id).await;
    }

    HttpResponse::Found()
        .append_header(("Location", "/todo_new/"))
        .finish()
}

#[post("/done")]
pub async fn done(pool: web::Data<SqlitePool>, form: web::Form<Task>) -> HttpResponse {
    let task = form.into_inner();

    // 完了ボタン押下時
    if let Some(id) = task.id {
        db::done_task(&pool, id).await;
    }

    HttpResponse::Found()
        .append_header(("Location", "/todo_new/"))
        .finish()
}

#[post("/undo")]
pub async fn undo(pool: web::Data<SqlitePool>, form: web::Form<Task>) -> HttpResponse {
    let task = form.into_inner();

    // 戻す(仕掛かり中→未着手)ボタン押下時
    if let Some(id) = task.id {
        db::undo_task(&pool, id).await;
    }

    HttpResponse::Found()
        .append_header(("Location", "/todo_new/"))
        .finish()
}

#[post("/doing")]
pub async fn doing(pool: web::Data<SqlitePool>, form: web::Form<Task>) -> HttpResponse {
    let task = form.into_inner();

    // 戻す(完了→仕掛かり中)ボタン押下時
    if let Some(id) = task.id {
        db::doing_task(&pool, id).await;
    }

    HttpResponse::Found()
        .append_header(("Location", "/todo_new/"))
        .finish()
}

#[post("/delete")]
pub async fn delete(pool: web::Data<SqlitePool>, form: web::Form<Task>) -> HttpResponse {
    let task = form.into_inner();

    // 削除ボタン押下時
    if let Some(id) = task.id {
        db::remove_task(&pool, id).await;
    }

    HttpResponse::Found()
        .append_header(("Location", "/todo_new/"))
        .finish()
}
