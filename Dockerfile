FROM rust:1.75

WORKDIR /usr/src/audiobooks

RUN apt-get update
RUN apt-get install -y postgresql-client zip build-essential autoconf libtool pkg-config protobuf-compiler libprotobuf-dev

ENV SQLX_OFFLINE=true

COPY . .

EXPOSE 8000

RUN cargo install --path .

CMD ["audiobooks"]