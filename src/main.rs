mod encryption;
mod model;
mod storage;
mod token;
mod error;
mod grpc;

use tracing::info;
use crate::error::ShaResult;
use crate::grpc::grpc_server;

///
/// Main entry point for server
///

#[tokio::main]
async fn main() -> ShaResult<()> {
    // Load ENV vars
    dotenv::dotenv().ok();
    // install global collector configured based on RUST_LOG env var.
    tracing_subscriber::fmt::init();
    
    let service = grpc_server()?;
    let storage = storage::connection()?;

    info!("Server started");

    Ok(())
}
