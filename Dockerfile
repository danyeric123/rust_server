# Use the latest official Rust image as the base image
FROM rust:latest as builder

# Set the working directory inside the container
WORKDIR /usr/src/app

# Install required dependencies
RUN apt-get update && apt-get install -y libpq-dev

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Set the DATABASE_URL environment variable for the build process
ENV DATABASE_URL=postgres://postgres:postgres@localhost/rust_db
ENV POSTGRES_USER=postgres
ENV POSTGRES_PASSWORD=postgres
ENV POSTGRES_DB=rust_db
ENV POSTGRES_HOST=localhost

ENV SQLX_OFFLINE=true

# Now copy the real source code
COPY src ./src

# Run cargo sqlx prepare to generate sqlx-data.json
RUN cargo install sqlx-cli && cargo sqlx prepare

# Build the application
RUN cargo build --release

# Use a different base image for the final image
FROM ubuntu:20.04

# Install required dependencies
RUN apt-get update && \ 
    apt-get install -y libpq-dev && \
    apt-get install -y libpq5 ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Set the working directory inside the container
WORKDIR /usr/src/app

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/app/target/release/rust_server .

# Expose the port that the application will run on
EXPOSE 8000

# Set the command to run the application
CMD ["./rust_server"]