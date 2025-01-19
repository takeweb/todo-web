use actix_files as fs;
use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = todo_web::init_db_pool().await;
    HttpServer::new(move || {
        App::new()
            .service(fs::Files::new("/static", "./static").show_files_listing())
            .service(todo_web::todo)
            .service(todo_web::create)
            .service(todo_web::start)
            .service(todo_web::done)
            .service(todo_web::delete)
            .app_data(web::Data::new(pool.clone()))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
