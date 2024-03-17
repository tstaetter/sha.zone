use surrealdb::engine::local::{Db, Mem};
use surrealdb::Surreal;
use crate::error::ShaResult;

/// Create connection to in-memory SurrealDB
pub async fn connection() -> ShaResult<Surreal<Db>> {
    let conn = Surreal::new::<Mem>(()).await?;
    let ns = std::env::var("SURREAL_NS").unwrap_or(String::from("sha_zone"));
    let db = std::env::var("SURREAL_DB").unwrap_or(String::from("general"));

    conn.use_ns(ns).use_db(db).await?;
    
    Ok(conn)
}
