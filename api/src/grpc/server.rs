use tonic::transport::server::Router;
use tonic::transport::{Server};
use crate::error::ShaResult;
use crate::grpc::authentication_service::AuthenticationServer;
use crate::grpc::file_service::FileServer;
use sha_zone_protobuf::protobuf::file_service_server::FileServiceServer;
use sha_zone_protobuf::protobuf::authentication_service_server::AuthenticationServiceServer;
use crate::token::request_verification;

/// Register all services to a gRPC server and return it to
/// be used in the main fn
pub fn grpc_server() -> ShaResult<Router> {
    let auth_svc = AuthenticationServiceServer::with_interceptor(AuthenticationServer::default(), request_verification);
    let file_svc = FileServiceServer::with_interceptor(FileServer::default(), request_verification);

    Ok(Server::builder()
        .accept_http1(true)
        .add_service(auth_svc)
        .add_service(file_svc))
}
