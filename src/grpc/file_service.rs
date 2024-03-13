use tonic::{Request, Response, Status, Streaming};
use crate::protobuf::{file_service_server, FileRequest, FileUpsertRequest, ServiceResponse};

#[derive(Default)]
pub struct FileServer {}

#[tonic::async_trait]
impl file_service_server::FileService for FileServer {
    async fn upsert(&self, request: Request<Streaming<FileUpsertRequest>>) -> Result<Response<ServiceResponse>, Status> {
        todo!()
    }

    async fn delete(&self, request: Request<FileRequest>) -> Result<Response<ServiceResponse>, Status> {
        todo!()
    }

    type PreviewStream = ();

    async fn preview(&self, request: Request<FileRequest>) -> Result<Response<Self::PreviewStream>, Status> {
        todo!()
    }
}

