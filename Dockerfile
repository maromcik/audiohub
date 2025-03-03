FROM rust:1.85 as base

RUN apt-get update
RUN apt-get install -y postgresql-client zip build-essential autoconf libtool pkg-config protobuf-compiler libprotobuf-dev

RUN cargo install cargo-chef --version 0.1.68

FROM base AS planner

WORKDIR /usr/src/audiobooks

COPY ./.env ./.env
COPY ./.sqlx ./.sqlx
COPY ./migrations ./migrations
COPY ./src ./src
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./proto ./proto
COPY ./templates ./templates
COPY ./build.rs ./build.rs
RUN cargo chef prepare --recipe-path recipe.json


FROM base as builder
WORKDIR /usr/src/audiobooks

RUN mkdir ./media
ENV SQLX_OFFLINE=true

COPY --from=planner /usr/src/audiobooks/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

COPY ./.env ./.env
COPY ./.sqlx ./.sqlx
COPY ./migrations ./migrations
COPY ./src ./src
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./proto ./proto
COPY ./templates ./templates
COPY ./build.rs ./build.rs

RUN cargo build --release --bin audiobooks

FROM debian:bookworm-slim AS runtime
RUN apt-get update
RUN apt-get install -y postgresql-client zip build-essential autoconf libtool pkg-config protobuf-compiler libprotobuf-dev

WORKDIR /usr/src/audiobooks
COPY --from=builder /usr/src/audiobooks/target/release/audiobooks /usr/local/bin

COPY ./.env ./.env
COPY ./media ./media
COPY ./static ./static

EXPOSE 8000

ENTRYPOINT ["/usr/local/bin/audiobooks"]