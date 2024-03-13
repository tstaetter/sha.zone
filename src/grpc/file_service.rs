use std::pin::Pin;
use tokio_stream::Stream;
use tonic::{Request, Response, Status, Streaming};
use crate::protobuf::{file_service_server, FileRequest, FileResponse, FileUpsertRequest, ServiceResponse};

#[derive(Default)]
pub struct FileServer {}

#[tonic::async_trait]
impl file_service_server::FileService for FileServer {
    async fn upsert(&self, _request: Request<Streaming<FileUpsertRequest>>) -> Result<Response<ServiceResponse>, Status> {
        todo!()
    }

    async fn delete(&self, _request: Request<FileRequest>) -> Result<Response<ServiceResponse>, Status> {
        todo!()
    }

    type PreviewStream = Pin<Box<dyn Stream<Item = Result<FileResponse, Status>> + Send>>;

    async fn preview(&self, _request: Request<FileRequest>) -> Result<Response<Self::PreviewStream>, Status> {
        todo!()
    }
}

