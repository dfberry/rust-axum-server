FROM rust

RUN apt update
RUN apt install -y libpq-dev

RUN cargo install diesel_cli --no-default-features --features postgres

WORKDIR /app

COPY . ./app

RUN cargo build --release

CMD ["/app/target/release/dfberry-rust-axum-server-source-board"]