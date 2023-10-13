FROM rust:1-slim-bullseye AS builder
WORKDIR /code

RUN USER=root cargo init
COPY Cargo.toml Cargo.toml
RUN cargo fetch

COPY src src

RUN cargo build --release

FROM debian:bullseye-slim
WORKDIR /app

COPY --from=builder /code/target/release/RustActixGQL_Template RustActixGQL_Template

USER 1001

EXPOSE 8080

CMD [ "/app/RustActixGQL_Template" ]
