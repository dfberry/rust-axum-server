FROM rust

RUN apt update
RUN apt install -y libpq-dev

#https://www.reddit.com/r/rust/comments/1f0ibyq/rust_diesel_postgres_container/
#RUN cargo install diesel_cli --no-default-features --features postgres

WORKDIR /app

COPY . .

RUN cargo build --release

CMD ["/app/target/release/dfberry-rust-axum-server-source-board"]