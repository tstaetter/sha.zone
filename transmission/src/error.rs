//! Project-wide used errors and Result types 
#[derive(thiserror::Error, Debug)]
pub enum TransitError {
    #[error("Environment variable error: {0}")]
    Env(#[from] std::env::VarError),
    #[error("Error parsing address: {0}")]
    AddrParse(#[from] std::net::AddrParseError),
    #[error("Tonic transport error: {0}")]
    TonicTransport(#[from] tonic::transport::Error),
    #[error("Tonic metadata error: {0}")]
    TonicMetadata(#[from] tonic::metadata::errors::ToStrError),
}

pub type TransitResult<T> = Result<T, TransitError>;
