# Rust CRUD API 서버

이 프로젝트는 Rust로 간단한 CRUD API 서버를 구현한 것입니다. PostgreSQL 데이터베이스와 연동되며, Docker와 Docker Compose를 이용해 쉽게 컨테이너화된 환경에서 실행할 수 있도록 구성하였습니다.

## 1. 프로젝트 구조

프로젝트의 기본 디렉토리 구조는 아래와 같습니다:

```plaintext
rust-crud-api/
├── src/
│   ├── main.rs
│   ├── handlers.rs
│   ├── models.rs
│   └── utils.rs
├── Cargo.toml
├── Dockerfile
└── docker-compose.yml
```

## 2. Cargo.toml

`Cargo.toml`은 프로젝트의 의존성을 관리하는 설정 파일입니다. 주요 의존성으로는 다음이 포함됩니다:

- `postgres`: PostgreSQL 데이터베이스와 통신하기 위한 라이브러리입니다.
- `serde`, `serde_json`: Rust 구조체를 JSON으로 직렬화하고 역직렬화하는 데 사용됩니다.
- `serde_derive`: `Serialize`와 `Deserialize` 매크로를 자동으로 생성해주는 라이브러리입니다.

## 3. src/main.rs

`main.rs`는 프로그램의 진입점으로, 서버를 초기화하고 클라이언트 요청을 처리하는 역할을 합니다. 여기서 데이터베이스 설정과 서버 초기화, 그리고 클라이언트 요청 대기를 설정하였습니다.

## 4. src/handlers.rs

클라이언트 요청을 처리하는 로직이 구현된 파일입니다. 사용자의 생성, 조회, 수정, 삭제를 처리하는 API가 여기에 구현되어 있으며, 서버의 상태를 확인할 수 있는 헬스 체크 API도 추가되었습니다.

## 5. src/models.rs

`models.rs`는 애플리케이션에서 사용하는 데이터 모델을 정의합니다. 이 프로젝트에서는 `User`라는 구조체를 정의하여 사용자 정보를 관리합니다.

## 6. src/utils.rs

`utils.rs`는 요청을 파싱하거나 데이터를 처리하는 유틸리티 함수들을 포함하고 있습니다. API 요청을 처리하는 데 필요한 다양한 기능을 이 파일에서 지원합니다.

## 7. Docker 설정

### 7.1 Dockerfile

Dockerfile을 통해 Rust 애플리케이션을 빌드하고 최종 이미지를 생성합니다.

### 7.2 docker-compose.yaml

`docker-compose.yaml` 파일을 사용하여 애플리케이션과 데이터베이스를 한 번에 관리할 수 있도록 구성했습니다. `rustapi`와 `db` 두 개의 서비스가 정의되어 있으며, `rustapi`는 `db`에 의존하도록 설정되어 있습니다.

## 8. 컨테이너 빌드 및 실행

준비된 파일들을 통해 다음과 같은 명령어로 컨테이너를 빌드하고 실행할 수 있습니다:

```bash
docker compose up db
docker compose build
docker compose up rustapi
```

이 명령어들은 `docker-compose.yaml` 파일에 정의된 서비스를 빌드하고 실행합니다.

## 9. API 테스트

서버가 정상적으로 실행되면 Postman 또는 CURL을 사용하여 API를 테스트할 수 있습니다. 예를 들어, 다음과 같은 명령어로 사용자를 생성할 수 있습니다:

```bash
curl -X POST http://localhost:8080/users -H "Content-Type: application/json" -d '{"name":"John Doe","email":"johndoe@example.com"}'

curl -X GET http://localhost:8080/users

curl -X GET http://localhost:8080/users/1

curl -X PUT http://localhost:8080/users/1 \
-H "Content-Type: application/json" \
-d '{"name":"Jane Doe","email":"janedoe@example.com"}'

curl -X DELETE http://localhost:8080/users/1

curl -X GET http://localhost:8080/health
```
