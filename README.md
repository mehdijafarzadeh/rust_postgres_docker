# PostgreSQL Rust Application with Docker

This Rust application connects to a PostgreSQL database and creates two tables: `users` and `posts`. It uses the `dotenv` crate to manage environment variables and establishes a foreign key relationship between `posts` and `users`.

## Features

- Connects to PostgreSQL database
- Creates `users` and `posts` tables if they don't exist
- Demonstrates use of `dotenv` for managing environment variables
- Compatible with Docker and Docker Compose



### Step 1: Clone the repository

```bash
git clonehttps://github.com/mehdijafarzadeh/rust_postgres_docker.git
cd rust_postgres_docker



### Notes:
- Ensure that the PostgreSQL Docker image uses a correct and up-to-date version (e.g., `postgres:latest`).
- The `.env` file should match the credentials and setup in the `docker-compose.yml` file.
