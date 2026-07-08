fn main() {
    let result = tonic_prost_build::configure()
        .build_client(false)
        .build_server(true)
        .build_transport(true)
        .compile_protos(
            &[
                "protos/kadepay/v1/services/invoice/invoice.proto",
                "protos/kadepay/v1/services/wallet/wallet.proto",
            ],
            &["protos"],
        );
    match result {
        Ok(_) => {}
        Err(error) => panic!("Failed to compile protos {:?}", error),
    }
}
