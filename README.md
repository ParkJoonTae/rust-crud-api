# Rust CRUD API 서버

이 프로젝트는 Rust로 간단한 CRUD API 서버를 구현한 것입니다. PostgreSQL 데이터베이스와 연동되며, Docker Compose를 이용해 컨테이너화된 환경에서 실행할 수 있도록 구성하였습니다.

## 1. 프로젝트 구조

프로젝트의 기본 디렉토리 구조는 아래와 같습니다:

```plaintext
rust-crud-api/
├── src/
│   ├── main.rs
│   ├── handlers.rs
│   ├── models.rs
│   ├── utils.rs
│   └── db.rs
├── Cargo.toml
├── Dockerfile
└── docker-compose.yml
```

`Cargo.toml`은 프로젝트의 의존성을 관리하는 설정 파일입니다. 

- `postgres`: PostgreSQL 데이터베이스와 통신하기 위한 라이브러리입니다.
- `serde`, `serde_json`: Rust 구조체를 JSON으로 직렬화하고 역직렬화하는 데 사용됩니다.
- `serde_derive`: `Serialize`와 `Deserialize` 매크로를 자동으로 생성해주는 라이브러리입니다.

`main.rs`는 프로그램의 진입점으로, 서버를 초기화하고 클라이언트 요청을 처리하는 역할을 합니다. 여기서 데이터베이스 설정과 서버 초기화, 그리고 클라이언트 요청 대기를 설정하였습니다.

`handlers.rs`는 클라이언트 요청을 처리하는 로직이 구현된 파일입니다. CRUD와 health check API가 구현되어 있습니다.

`models.rs`는 `User`라는 구조체를 정의하여 사용자 정보를 관리합니다.

`utils.rs`는 API 요청을 처리하는 데 필요한 유틸리티 함수를 지원합니다.

`db.rs`는 DB설정을 위한 경로, 연결, 테이블 생성 들을 처리합니다.

## 2 Dockerfile

`Dockerfile` 파일을 사용하여 애플리케이션을 이미지로 빌드합니다. 여기서 DATABASE_URL 변수를 설정하기 위해 docker compose로 빌드합니다.

```# 빌드 단계
FROM rust:1.70-buster as builder

WORKDIR /app

ARG DATABASE_URL
ENV DATABASE_URL=$DATABASE_URL

COPY . . 

RUN cargo build --release

# 실행 단계
FROM debian:buster-slim

WORKDIR /usr/local/bin

COPY --from=builder /app/target/release/rust-crud-api .

CMD ["./rust-crud-api"]
```

## 2 docker-compose.yaml

`docker-compose.yaml` 파일을 사용하여 애플리케이션과 데이터베이스를 한 번에 관리하고 빌드할 수 있게 구성했습니다. `rustapi`는 `rustapi-db`에 의존하도록 설정되어 있습니다. 

```plaintext
version: '3'

services:
  rustapi:
    container_name: rustapi
    image: ksi05298/rustapi:1.0.1
    build:
      context: .
      dockerfile: Dockerfile
      args:
        DATABASE_URL: postgres://postgres:postgres@rustapi-db:5432/postgres
    ports:
      - '8080:8080'
    depends_on:
      - rustapi-db
  rustapi-db:
    container_name: rustapi-db
    image: postgres:12
    environment:
      POSTGRES_USER: postgres
      POSTGRES_PASSWORD: postgres
      POSTGRES_DB: postgres
    ports:
      - '5432:5432'
    volumes:
      - pgdata:/var/lib/postgresql/data

volumes:
  pgdata: {}
```

## 3. 컨테이너 빌드 및 실행

다음과 같은 명령어로 컨테이너를 빌드하고 실행할 수 있습니다:

```bash
docker compose up rustapi-db
docker compose build
docker compose up rustapi
```

## 4. API 테스트

서버가 정상적으로 실행되면 CURL을 사용하여 API를 테스트할 수 있습니다. 예를 들어, 다음과 같은 명령어로 사용자를 생성할 수 있습니다:

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
