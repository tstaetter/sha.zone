mod grpc;
mod token;

///
/// Main entry point for SurrealDB backed server
///

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Hello, world!");
    
    Ok(())
}
