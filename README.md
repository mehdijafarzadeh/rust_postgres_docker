# PostgreSQL Rust Application with Docker

This Rust application connects to a PostgreSQL database and creates two tables: `users` and `posts`. It uses the `dotenv` crate to manage environment variables and establishes a foreign key relationship between `posts` and `users`.

## Features

- Connects to PostgreSQL database
- Creates `users` and `posts` tables if they don't exist
- Demonstrates use of `dotenv` for managing environment variables
- Compatible with Docker and Docker Compose

## Table of Contents

- [Installation](#installation)
- [Docker Setup](#docker-setup)
- [Usage](#usage)
- [Environment Variables](#environment-variables)
- [Errors](#errors)
- [Contributing](#contributing)
- [License](#license)

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/) (version 1.56 or higher recommended)
- [Docker](https://www.docker.com/) and [Docker Compose](https://docs.docker.com/compose/)

### Step 1: Clone the repository

```bash
git clone https://github.com/yourusername/your-repo-name.git
cd your-repo-name


### Notes:
- Ensure that the PostgreSQL Docker image uses a correct and up-to-date version (e.g., `postgres:latest`).
- The `.env` file should match the credentials and setup in the `docker-compose.yml` file.
