use std::fs::create_dir;
use std::path::Path;

fn main() {
    //let out_dir = Path::new("./generated");

    /*if !out_dir.exists() {
        match create_dir("./generated") {
            Ok(_) => (),
            Err(e) => panic!("{:?}", e),
        }
    }*/

    let result = tonic_prost_build::configure()
        .build_client(false)
        .build_server(true)
        .build_transport(true)
        // .out_dir(out_dir)
        .compile_protos(
            &[
                "protos/kadepay/v1/services/invoice.proto",
                "protos/kadepay/v1/services/wallet.proto",
            ],
            &["proto"],
        );
    match result {
        Ok(_) => {}
        Err(error) => panic!("Failed to compile protos {:?}", error),
    }
}
