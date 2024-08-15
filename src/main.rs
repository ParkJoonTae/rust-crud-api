use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

mod models;
mod handlers;
mod db;
mod utils;

use handlers::{handle_client};
use db::set_database;

// 메인 함수
fn main() {
    // 데이터베이스 설정
    if let Err(e) = set_database() {
        println!("데이터베이스 설정 중 오류 발생: {}", e);
        return;
    }

    // 서버 시작 및 포트 출력
    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();
    println!("서버가 8080 포트에서 시작되었습니다.");

    // 클라이언트 요청 처리
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream);
            }
            Err(e) => {
                println!("클라이언트 연결 중 오류 발생: {}", e);
            }
        }
    }
}
