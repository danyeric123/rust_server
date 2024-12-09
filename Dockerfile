# Use the latest official Rust image as the base image
FROM rust:latest as builder

# Set the working directory inside the container
WORKDIR /usr/src/app

# Install required dependencies
RUN apt-get update && apt-get install -y libpq-dev

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Set the DATABASE_URL environment variable for the build process
ENV DATABASE_URL=postgres://user:password@db/rust_db

# Copy the build script
COPY build.rs ./

# Create dummy src/main.rs to build dependencies
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    # Build dependencies - this is cached
    cargo build --release && \
    # Remove the dummy file
    rm -rf src

# Now copy the real source code
COPY src ./src


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