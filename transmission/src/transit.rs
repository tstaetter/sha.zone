//! Transit service

use crate::error::TransitResult;
use tracing::info;

mod error;

#[tokio::main]
async fn main() -> TransitResult<()> {
    // Load ENV vars
    dotenv::dotenv().ok();
    // install global collector configured based on RUST_LOG env var.
    tracing_subscriber::fmt::init();
    
    info!("Transition service running");

    Ok(())
}
