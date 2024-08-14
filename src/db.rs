use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::env;

/// 데이터베이스 풀을 생성하고 반환합니다.
pub async fn create_pool() -> PgPool {
    dotenv::dotenv().ok(); // .env 파일에서 환경 변수 로드

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL 환경 변수를 설정해야 합니다.");
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("데이터베이스에 연결할 수 없습니다.")
}