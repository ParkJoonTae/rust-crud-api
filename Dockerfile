# 빌드 단계
FROM rust:1.70 as builder

WORKDIR /usr/src/rust_pg_api
COPY . .

RUN cargo build --release

# 실행 단계
FROM debian:buster-slim

WORKDIR /usr/src/rust_pg_api
COPY --from=builder /usr/src/rust_pg_api/target/release/rust_pg_api .
COPY --from=builder /usr/src/rust_pg_api/.env .

CMD ["./rust_pg_api"]