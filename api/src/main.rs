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

    let service_addr = std::env::var("SVC_ADDR").unwrap_or(String::from(""));
    let service_addr = service_addr.parse()?;
    let service = grpc_server()?;
    // let _storage = storage::connection().await?;

    info!("Server starting");

    service.serve(service_addr).await?;
    
    Ok(())
}
