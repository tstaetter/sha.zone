use tonic::transport::server::Router;
use tonic::transport::{Server};
use crate::error::ShaResult;
use crate::grpc::authentication_service::AuthenticationServer;
use crate::grpc::file_service::FileServer;

/// Register all services to a gRPC server and return it to
/// be used in the main fn
pub fn grpc_server() -> ShaResult<Router> {
    let auth_svc = AuthenticationServer::default();
    let file_svc = FileServer::default().into();

    Ok(Server::builder()
        .accept_http1(true)
        .add_service(auth_svc)
        .add_service(file_svc))
}
