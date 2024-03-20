FROM rust:1.75

WORKDIR /usr/src/audiobooks

RUN apt-get update
RUN apt-get install -y postgresql-client zip build-essential autoconf libtool pkg-config protobuf-compiler libprotobuf-dev

COPY . .

EXPOSE 8000
RUN cargo install sqlx-cli
RUN sqlx database create
RUN sqlx migrate run
RUN cargo install --path .

CMD ["audiobooks"]