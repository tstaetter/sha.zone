use tonic::transport::server::Router;
use tonic::transport::{Server};
use crate::error::ShaResult;
use crate::grpc::authentication_service::AuthenticationServer;
use crate::grpc::file_service::FileServer;
use crate::protobuf::file_service_server::FileServiceServer;
use crate::protobuf::authentication_service_server::AuthenticationServiceServer;

/// Register all services to a gRPC server and return it to
/// be used in the main fn
pub fn grpc_server() -> ShaResult<Router> {
    let auth_svc = AuthenticationServiceServer::new(AuthenticationServer::default());
    let file_svc = FileServiceServer::new(FileServer::default());

    Ok(Server::builder()
        .accept_http1(true)
        .add_service(auth_svc)
        .add_service(file_svc))
}
