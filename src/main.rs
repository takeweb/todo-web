use actix_files as fs;
use actix_web::{get, post, web, App, HttpResponse, HttpServer};
use askama::Template;
use askama_actix::TemplateToResponse;
use sqlx::{Pool, Row, Sqlite, SqlitePool};
use std::env;

#[derive(Template)]
#[template(path = "todo.html")]
struct TodoTemplate {
    tasks: Vec<String>,
}

#[derive(serde::Deserialize)]
struct Task {
    id: Option<String>,
    task: Option<String>,
}

#[get("/")]
async fn todo(pool: web::Data<SqlitePool>) -> HttpResponse {
    let rows = sqlx::query("SELECT task FROM tasks;")
        .fetch_all(pool.as_ref())
        .await
        .unwrap();
    let tasks: Vec<String> = rows
        .iter()
        .map(|row| row.get::<String, _>("task"))
        .collect();
    let todo = TodoTemplate { tasks };

    todo.to_response()
}

#[post("/update")]
async fn update(pool: web::Data<SqlitePool>, form: web::Form<Task>) -> HttpResponse {
    let task = form.into_inner();

    // Doneボタン押下時
    if let Some(id) = task.id {
        sqlx::query("DELETE FROM tasks WHERE task = ?")
            .bind(id)
            .execute(pool.as_ref())
            .await
            .unwrap();
    }

    // 作成ボタン押下時
    match task.task {
        Some(task) if !task.is_empty() => {
            sqlx::query("INSERT INTO tasks (task) VALUES (?)")
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

async fn init_db_pool() -> Pool<Sqlite> {
    // .envの読み込み
    dotenv::dotenv().expect(".envの読み込み失敗");

    // DATABASE_URLの取得
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URLがセットされていません");

    // 接続プールの作成
    SqlitePool::connect(&database_url).await.unwrap()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = init_db_pool().await;
    HttpServer::new(move || {
        App::new()
            .service(fs::Files::new("/static", "./static").show_files_listing())
            .service(todo)
            .service(update)
            .app_data(web::Data::new(pool.clone()))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
