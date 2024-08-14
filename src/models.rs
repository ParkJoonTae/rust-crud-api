use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// 데이터 모델: items 테이블과 매핑되는 구조체입니다.
#[derive(Serialize, Deserialize, FromRow)]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub description: String,
}

/// 데이터 입력 모델: 새로운 아이템을 생성할 때 사용됩니다.
#[derive(Serialize, Deserialize)]
pub struct CreateItem {
    pub name: String,
    pub description: String,
}