use postgres::{Client, NoTls, Error as PostgresError};

// 데이터베이스 URL 상수
pub const DB_URL: &str = env!("DATABASE_URL");

// 데이터베이스 설정 함수
pub fn set_database() -> Result<(), PostgresError> {
    // 데이터베이스에 연결
    let mut client = Client::connect(DB_URL, NoTls)?;

    // 테이블 생성
    client.batch_execute(
        "CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            name VARCHAR NOT NULL,
            email VARCHAR NOT NULL
        )"
    )?;
    Ok(())
}
