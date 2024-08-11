#--------------------------------------------
# Use the official Rust image as the base image
FROM rust:latest as builder

# Set the working directory inside the container
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock files to the container
COPY Cargo.toml Cargo.lock ./

# Copy the source code to the container
COPY src ./src

# Build the Rust project
RUN cargo build --release

#--------------------------------------------
# Use a minimal base image for the runtime
FROM gcr.io/distroless/cc AS runtime

# Set the working directory inside the container
WORKDIR /app

# Copy the built binary from the builder stage
COPY --from=builder /app/target/release/dfberry-rust-axum-server-source-board /usr/local/bin/app

# Set the entrypoint to the built binary
ENTRYPOINT ["/usr/local/bin/app"]