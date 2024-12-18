FROM rust:1.83

WORKDIR /usr/src/audiobooks

RUN apt-get update
RUN apt-get install -y postgresql-client zip build-essential autoconf libtool pkg-config protobuf-compiler libprotobuf-dev

ENV SQLX_OFFLINE=true
COPY ./.env ./.env
COPY ./.sqlx ./.sqlx
COPY ./media ./media
COPY ./migrations ./migrations
COPY ./proto ./proto
COPY ./src ./src
COPY ./templates ./templates
COPY ./build.rs ./build.rs
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml


EXPOSE 8000

RUN cargo install --path .

CMD ["audiobooks"]