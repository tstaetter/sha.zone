//! Transit service

use crate::error::ShaResult;
use tracing::info;

mod token;
mod error;

pub mod protobuf {
    tonic::include_proto!("sha.zone.transition_service");
}

#[tokio::main]
async fn main() -> ShaResult<()> {
    // Load ENV vars
    dotenv::dotenv().ok();
    // install global collector configured based on RUST_LOG env var.
    tracing_subscriber::fmt::init();
    
    info!("Transition service running");

    Ok(())
}
