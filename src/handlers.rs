use actix_web::{web, HttpResponse};
use sqlx::PgPool;

use crate::models::{CreateItem, Item};

/// 새로운 아이템을 생성하는 핸들러
pub async fn create_item(
    pool: web::Data<PgPool>,
    item: web::Json<CreateItem>,
) -> HttpResponse {
    let result = sqlx::query!(
        r#"
        INSERT INTO items (name, description) VALUES ($1, $2)
        "#,
        item.name,
        item.description
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Created().json(item.into_inner()),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

/// 모든 아이템을 가져오는 핸들러
pub async fn get_items(pool: web::Data<PgPool>) -> HttpResponse {
    let result = sqlx::query_as::<_, Item>(
        r#"
        SELECT id, name, description FROM items
        "#
    )
    .fetch_all(pool.get_ref())
    .await;

    match result {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

/// 특정 아이템을 가져오는 핸들러
pub async fn get_item(pool: web::Data<PgPool>, item_id: web::Path<i32>) -> HttpResponse {
    let result = sqlx::query_as::<_, Item>(
        r#"
        SELECT id, name, description FROM items WHERE id = $1
        "#,
        *item_id
    )
    .fetch_one(pool.get_ref())
    .await;

    match result {
        Ok(item) => HttpResponse::Ok().json(item),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

/// 특정 아이템을 업데이트하는 핸들러
pub async fn update_item(
    pool: web::Data<PgPool>,
    item_id: web::Path<i32>,
    item: web::Json<CreateItem>,
) -> HttpResponse {
    let result = sqlx::query!(
        r#"
        UPDATE items SET name = $1, description = $2 WHERE id = $3
        "#,
        item.name,
        item.description,
        *item_id
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().json(item.into_inner()),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

/// 특정 아이템을 삭제하는 핸들러
pub async fn delete_item(pool: web::Data<PgPool>, item_id: web::Path<i32>) -> HttpResponse {
    let result = sqlx::query!(
        r#"
        DELETE FROM items WHERE id = $1
        "#,
        *item_id
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}