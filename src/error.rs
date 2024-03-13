//! Project-wide used errors and Result types 
#[derive(thiserror::Error, Debug)]
pub enum ShaError {
    #[error="Error with surreal DB: {0}"]
    Surreal(#[from] surrealdb::Error),
    #[error="Error reading env var: {0}"]
    Env(#[from] std::env::VarError),
}

pub type ShaResult<T> = Result<T, ShaError>;