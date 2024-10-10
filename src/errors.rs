use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("database error: {0}")]
    Database(#[from] postgres::Error),
    #[error("dotenv error: {0}")]
    Dotenv(#[from] dotenv::Error),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("env variable error: {0}")]
    EnvVar(#[from] std::env::VarError),
}
