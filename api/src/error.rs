//! Project-wide used errors and Result types 
#[derive(thiserror::Error, Debug)]
pub enum ShaError {
    #[error("Error with SurrealDB: {0}")]
    Surreal(#[from] surrealdb::Error),
    #[error("Environment variable error: {0}")]
    Env(#[from] std::env::VarError),
    #[error("Error parsing address: {0}")]
    AddrParse(#[from] std::net::AddrParseError),
    #[error("Tonic transport error: {0}")]
    TonicTransport(#[from] tonic::transport::Error),
    #[error("Tonic metadata error: {0}")]
    TonicMetadata(#[from] tonic::metadata::errors::ToStrError),
    #[error("Error handling token: {0}")]
    JwtToken(#[from] jwt::Error),
    #[error("Error reading encryption key: {0}")]
    Sha2(#[from] sha2::digest::InvalidLength),
}

pub type ShaResult<T> = Result<T, ShaError>;
