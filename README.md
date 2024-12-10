# Rust HTTP Server

This project is a simple HTTP server implemented in Rust using the Axum framework. It is designed to help you learn Rust and understand how to build and deploy a web server using Docker and Docker Compose.

## Features

- Health check endpoint
- User management (create and fetch users)
- PostgreSQL database integration
- Tracing and logging
- Docker and Docker Compose setup for easy deployment

## Endpoints

- GET /healthz: Health check endpoint
- GET /users/:id: Fetch a user by ID
- POST /users: Create a new user