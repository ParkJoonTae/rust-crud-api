#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};
    use sqlx::{PgPool, Executor};
    use std::env;

    async fn setup_test_db() -> PgPool {
        dotenv::dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL 환경 변수를 설정해야 합니다.");
        let pool = PgPool::connect(&database_url).await.expect("테스트 데이터베이스에 연결할 수 없습니다.");

        pool.execute("TRUNCATE TABLE items RESTART IDENTITY").await.unwrap();

        pool
    }

    #[actix_web::test]
    async fn test_create_item() {
        let pool = setup_test_db().await;
        let mut app = test::init_service(App::new().app_data(web::Data::new(pool.clone())).service(
            web::scope("/api").route("/items", web::post().to(create_item))
        )).await;

        let item = CreateItem {
            name: String::from("Test Item"),
            description: String::from("Test Description"),
        };

        let req = test::TestRequest::post()
            .uri("/api/items")
            .set_json(&item)
            .to_request();

        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_success());

        let result: Item = test::read_body_json(resp).await;
        assert_eq!(result.name, "Test Item");
        assert_eq!(result.description, "Test Description");
    }

    // 추가적인 테스트들 (GET, PUT, DELETE)도 유사한 방식으로 작성할 수 있습니다.
}