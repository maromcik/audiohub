FROM rust:1.75

WORKDIR /usr/src/audiobooks

RUN apt-get update
RUN apt-get install -y postgresql-client zip build-essential autoconf libtool pkg-config protobuf-compiler libprotobuf-dev

COPY . .


RUN cargo install --path .

CMD ["audiobooks"]