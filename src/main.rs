mod db;
mod handlers;
mod models;

use actix_web::{web, App, HttpServer};
use db::create_pool;
use handlers::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // PostgreSQL 연결 풀을 생성합니다.
    let pool = create_pool().await;

    // HttpServer를 설정하고, 라우팅을 구성합니다.
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone())) // 데이터베이스 풀을 애플리케이션 데이터로 설정합니다.
            .service(
                web::scope("/api")
                    .route("/items", web::post().to(create_item)) // POST /api/items
                    .route("/items", web::get().to(get_items))    // GET /api/items
                    .route("/items/{id}", web::get().to(get_item)) // GET /api/items/{id}
                    .route("/items/{id}", web::put().to(update_item)) // PUT /api/items/{id}
                    .route("/items/{id}", web::delete().to(delete_item)) // DELETE /api/items/{id}
            )
    })
    .bind("0.0.0.0:8080")? // 서버를 8080 포트에 바인딩합니다.
    .run()
    .await
}