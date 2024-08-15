use serde::{Serialize, Deserialize};

// 모델: User 구조체 (id, name, email)
#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: Option<i32>,
    pub name: String,
    pub email: String,
}
