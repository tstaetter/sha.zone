use tonic::transport::server::Router;
use tonic::transport::Server;

/// Register all services to a gRPC server and return it to
/// be used in the main fn
pub async fn grpc_server() -> anyhow::Result<Router> {
    let server = Server::builder()
        .accept_http1(true)
        .add_service(tonic_web::enable(frontend_handler));

    Ok(server)
}
