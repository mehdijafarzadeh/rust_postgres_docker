mod errors;

use std::collections::HashMap;
use crate::errors::AppError;
use dotenv::dotenv;
use postgres::{Client, NoTls};
use std::env;
use rust_postgres::Users;

fn main() -> Result<(), AppError> {
    // Load environment variables from .env file
    dotenv().map_err(AppError::Dotenv)?;

    // Get the DATABASE_URL from the environment
    let url = env::var("DATABASE_URL")?;
    println!("Connecting to: {}", url);

    // Connect to PostgreSQL using the URL from the environment
    let mut client = Client::connect(&url, NoTls).map_err(|err| {
        eprintln!("Error connecting to database: {}", err);
        AppError::Database(err)
    })?;

    // Create 'users' table if it does not exist
    client
        .batch_execute(
            "
        CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            name TEXT NOT NULL,
            country TEXT NOT NULL
        )
    ",
        )
        .map_err(|err| {
            eprintln!("Error creating 'users' table: {}", err);
            AppError::Database(err)
        })?;

    // Create 'posts' table if it does not exist, with foreign key referencing 'users'
    client
        .batch_execute(
            "
        CREATE TABLE IF NOT EXISTS posts (
            id SERIAL PRIMARY KEY,
            title TEXT NOT NULL,
            user_id INTEGER REFERENCES users(id)
        )
    ",
        )
        .map_err(|err| {
            eprintln!("Error creating 'posts' table: {}", err);
            AppError::Database(err)
        })?;

    println!("Tables created successfully or already exist.");

    let mut users = HashMap::new();
    users.insert( String::from("Kani"), "Kurdiya");
    users.insert(String::from("Jane"), "Japan");
    users.insert(String::from("Bob"), "USA");

    for (name, country) in &users {
        let user = Users{
            _id: 0,
            name: name.to_string(),
            country: country.to_string(),
        };

        client.execute(
            "INSERT INTO users (name, country) VALUES ($1, $2)",
            &[&user.name, &user.country],
        )?;

        println!("Inserted user: {:?}", user);

        for row in client.query("SELECT * FROM users", &[])? {
            let user = Users{
                _id: row.get(0),
                name: row.get(1),
                country: row.get(2),
            };
            println!("Retrieved user: {:?}, {:?}", user.name, user.country);

        }
    }


    Ok(())
}

