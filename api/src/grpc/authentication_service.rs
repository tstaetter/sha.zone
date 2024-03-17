use tonic::{Request, Response, Status};
use sha_zone_protobuf::protobuf::{authentication_service_server, LoginRequest, RegisterRequest, TokenRequest, TokenResponse};

#[derive(Default)]
pub struct AuthenticationServer {}

#[tonic::async_trait]
impl authentication_service_server::AuthenticationService for AuthenticationServer {
    async fn register(&self, _request: Request<RegisterRequest>) -> Result<Response<TokenResponse>, Status> {
        todo!()
    }

    async fn login(&self, _request: Request<LoginRequest>) -> Result<Response<TokenResponse>, Status> {
        todo!()
    }

    async fn validate(&self, _request: Request<TokenRequest>) -> Result<Response<TokenResponse>, Status> {
        todo!()
    }
}

