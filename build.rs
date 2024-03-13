use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let out_dir = PathBuf::from(std::env::var("PB_OUT").unwrap());

    tonic_build::configure()
        .build_client(false)
        .file_descriptor_set_path(out_dir.join("sha_zone.bin"))
        .compile(
            &[
                "proto/encryption_service.proto",
                "proto/file_service.proto",
                "proto/storage_service.proto",
                "proto/authentication_service.proto",
            ],
            &["proto"])
        .unwrap();

    Ok(())
}
