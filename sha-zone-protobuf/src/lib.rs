pub mod protobuf {
    tonic::include_proto!("sha.zone.authentication_service");
    tonic::include_proto!("sha.zone.file_service");
    tonic::include_proto!("sha.zone.transition_service");
}


#[cfg(test)]
mod tests {
}
