fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    tonic_build::configure()
        .build_client(false)
        .compile(
            &[
                "proto/authentication_service.proto",
                "proto/file_service.proto",
                "proto/transition_service.proto",
            ],
            &["proto"],
        )
        .unwrap();

    Ok(())
}
