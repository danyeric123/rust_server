use std::env;
use std::fs;

fn main() {
    // Set the DATABASE_URL environment variable for the build process
    env::set_var("DATABASE_URL", "postgres://user:password@db/rust_db");

    // Touch the .env file to ensure it exists
    fs::write(".env", "DATABASE_URL=postgres://user:password@db/rust_db").unwrap();
}