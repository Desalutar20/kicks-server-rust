FROM rust:1.93-alpine3.20 as development

RUN apk add --no-cache pkgconfig libc-dev openssl-dev openssl-libs-static
RUN cargo install sqlx-cli --no-default-features --features native-tls,postgres --version 0.8.6

WORKDIR /app

COPY Cargo.toml ./
COPY Cargo.lock ./

RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build

COPY ./src ./src
COPY ./.sqlx ./.sqlx
COPY /configs/development.yaml ./configs/development.yaml
COPY ./migrations ./migrations

ENV SQLX_OFFLINE=true

RUN cargo build

ENV APPLICATION__ENV=development
CMD sh -c "sqlx --no-dotenv migrate run && ./target/debug/kicksapi"


FROM rust:1.93-alpine3.20 as testing

RUN apk add --no-cache musl-dev openssl-dev

WORKDIR /app

COPY . .

ENV SQLX_OFFLINE=true
ENV APPLICATION__ENV=test

CMD ["cargo", "test", "--", "--nocapture"]


FROM rust:1-alpine3.20 AS chef
WORKDIR /app
RUN cargo install --locked cargo-chef
RUN apk add --no-cache musl-dev openssl-dev


FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json


FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --recipe-path recipe.json --release
COPY . .

ENV SQLX_OFFLINE=true
RUN cargo build --release


FROM alpine:3.20 AS production
WORKDIR /app

COPY --from=builder /app/configs/production.yaml ./configs/production.yaml
COPY --from=builder /app/migrations ./migrations
COPY --from=builder /app/target/release/kicksapi .

ENV APPLICATION__ENV=production
CMD ["./kicksapi"]
