//! Implementation for all available JWT tokens

use hmac::{Hmac, Mac};
use jwt::{AlgorithmType, Header, SignWithKey, Token, VerifyWithKey};
use jwt::token::Signed;
use sha2::Sha384;
use serde_derive::{Deserialize, Serialize};
use tonic::{Request, Status};
use tracing::error;
use crate::error::ShaResult;

/// All supported token types
pub enum TokenType {
    /// Used when a registered user requests a file from a 3rd party
    /// being sent to his account
    FileRequest,
    /// Used for refreshing UserTokens
    Refresh,
    /// Used while uploading files
    Transition,
    /// Common JWT token for user identification
    User,
}

#[derive(Debug, Deserialize, Serialize, Default, PartialEq)]
pub struct JwtClaim {}

/// Create a new JWT token
pub fn create(_ty: TokenType, _claim: JwtClaim) -> ShaResult<Token<Header, JwtClaim, Signed>> {
    let key = std::env::var("JWT_SECRET")?;
    let key: Hmac<Sha384> = Hmac::new_from_slice(key.as_bytes())?;
    let header = Header {
        algorithm: AlgorithmType::Hs384,
        ..Default::default()
    };
    let claims = JwtClaim::default();
    let token = Token::new(header, claims).sign_with_key(&key)?;

    Ok(token)
}

pub fn request_verification(req: Request<()>) -> Result<Request<()>, Status> {
    match req.metadata().get("authorization") {
        Some(token) if verify(token.to_str().unwrap_or("")).unwrap_or(false) => Ok(req),
        _ => Err(Status::unauthenticated("No valid auth token")),
    }
}

/// Verify token represented by its token string
pub fn verify(token_str: &str) -> ShaResult<bool> {
    let key: Hmac<Sha384> = Hmac::new_from_slice(std::env::var("JWT_SECRET")?.as_bytes())?;
    let token: Token<Header, JwtClaim, _> = VerifyWithKey::verify_with_key(token_str, &key)?;
    let header = token.header();
    let claims = token.claims();
    let ref_claim = JwtClaim::default();
    
    assert_eq!(header.algorithm, AlgorithmType::Hs384);
    assert_eq!(claims, &ref_claim);
    
    Ok(true)
}
