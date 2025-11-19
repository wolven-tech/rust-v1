fn main() {
    // Only compile protobuf if grpc feature is enabled
    #[cfg(feature = "grpc")]
    {
        tonic_build::configure()
            .build_server(true)
            .build_client(true)
            .out_dir("src/generated")
            .compile(&["proto/api.proto"], &["proto"])
            .unwrap_or_else(|e| panic!("Failed to compile protos: {}", e));
    }
}
