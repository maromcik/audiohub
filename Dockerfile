FROM rust:1.75

WORKDIR /usr/src/audiobooks

RUN apt-get update
RUN apt-get install -y postgresql-client

COPY . .
RUN mkdir media

RUN cargo install --path .

CMD ["audiobooks"]