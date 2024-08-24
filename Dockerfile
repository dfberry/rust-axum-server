#--------------------------------------------
# Use the official Rust image as the base image
FROM mcr.microsoft.com/devcontainers/rust:bookworm as builder

RUN apt-get update 

# Set the working directory inside the container
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock files to the container
COPY Cargo.toml Cargo.lock ./

# Copy the source code to the container
COPY src ./src

# Build the Rust project
RUN cargo build --release

# Expose port 3000
EXPOSE 3000

# Set the entrypoint to the built binary
ENTRYPOINT ["/app"]