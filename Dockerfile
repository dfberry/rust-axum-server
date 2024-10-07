FROM rust as builder

RUN apt update && apt install -y libpq-dev

#https://www.reddit.com/r/rust/comments/1f0ibyq/rust_diesel_postgres_container/
#RUN cargo install diesel_cli --no-default-features --features postgres

WORKDIR /app

COPY . .
RUN ls -la

RUN cargo build --release
#---------------------------------------------
FROM rust as server

RUN apt update && apt install -y libpq-dev

WORKDIR /app

# Copy the built application from the first stage
COPY --from=builder /app/target/release/source-board-server /app/source-board-server
COPY --from=builder /app/Cargo.toml /app/Cargo.toml

RUN ls -la

CMD ["/app/source-board-server"]