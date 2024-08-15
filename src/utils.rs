use crate::models::User;
use serde_json;

// 요청에서 ID 추출 함수
pub fn get_id(request: &str) -> &str {
    request.split("/").nth(2).unwrap_or_default().split_whitespace().next().unwrap_or_default()
}

// 요청 본문에서 사용자 정보 추출 함수
pub fn get_user_request_body(request: &str) -> Result<User, serde_json::Error> {
    serde_json::from_str(request.split("\r\n\r\n").last().unwrap_or_default())
}
