use ark_core::server::Info;
use bitcoin::{
    Address, Amount, KnownHrp, Network, ScriptBuf, Sequence, WitnessProgram, WitnessVersion,
};
use kadepayd::core::KadeHDWallet;
use std::collections::{HashMap, HashSet};

#[tokio::test]
async fn should_generate_new_offchain_payment_address_on_mutinynet_successfully() {
    let server_info = get_ark_server_info();
    let server_pub_key = server_info.signer_pk.x_only_public_key().0;
    let exit_delay = server_info.unilateral_exit_delay;
    let network = server_info.network;
    let address = KadeHDWallet::new_offchain_payment_address(
        "tpubDDneEXG899zhkpQt6bqo7fmaSVVi7ErfjNSs82gmTKJHJM5dfzT6f4er8dqgt85z3TYZYzJ7FZeTzKSkX1KKs8ejtXGg4FudTA9TR55ntaF".to_string(),
        server_pub_key,
        exit_delay,
        0,
        network
    ).expect("failed to create address");

    assert_eq!(
        address.to_string(),
        "tark1qzsexy9fnys8m0v6q0fq7ey7xlr6279q0467d7se4gln8lrtz43zcksgp8pqgml42pxjjhgv50q3ruepn355r5kypk8j7jg4d04fle9kpuect8"
    );
}

#[tokio::test]
async fn should_generate_new_offchain_payment_address_for_every_index_successfully() {
    let server_info = get_ark_server_info();
    let server_pub_key = server_info.signer_pk.x_only_public_key().0;
    let exit_delay = server_info.unilateral_exit_delay;
    let network = server_info.network;

    let mut prev_address = "".to_string();
    let mut seen_addresses: HashSet<String> = HashSet::new();
    for index in 0..16 {
        let address_result = KadeHDWallet::new_offchain_payment_address(
            "tpubDDneEXG899zhkpQt6bqo7fmaSVVi7ErfjNSs82gmTKJHJM5dfzT6f4er8dqgt85z3TYZYzJ7FZeTzKSkX1KKs8ejtXGg4FudTA9TR55ntaF".to_string(),
            server_pub_key,
            exit_delay,
            index,
            network
        );
        assert!(address_result.is_ok());
        let address = address_result.unwrap().to_string();
        assert!(!address.is_empty());
        assert_ne!(address, prev_address);
        assert!(seen_addresses.insert(address.clone()));

        prev_address = address;
    }
}

pub fn get_ark_server_info() -> Info {
    Info {
        version: "".to_string(),
        signer_pk: "03a19310a999207dbd9a03d20f649e37c7a578a07d75e6fa19aa3f33fc6b15622c"
            .parse()
            .unwrap(),
        forfeit_pk: "03571632039959ffa1724079cbf03522df889d47ebc3085f9589468d765e447d84"
            .parse()
            .unwrap(),
        forfeit_address: Address::from_witness_program(
            WitnessProgram::new(
                WitnessVersion::V1,
                "15048e41633084bfcae91d03b3c2bb7f6ac78440".as_bytes(),
            )
            .unwrap(),
            KnownHrp::Testnets,
        ),
        checkpoint_tapscript: ScriptBuf::from_bytes(Vec::from(
            "03a80040b27520dfcaec558c7e78cf3e38b898ba8a43cfb5727266bae32c5c5b3aeb32c558aa0bac"
                .as_bytes(),
        )),
        network: Network::Testnet,
        session_duration: 1,
        unilateral_exit_delay: Sequence(2),
        boarding_exit_delay: Sequence(180),
        utxo_min_amount: Some(Amount::from_sat(330)),
        utxo_max_amount: Some(Amount::from_sat(21_000_000)),
        vtxo_min_amount: Some(Amount::from_sat(1)),
        vtxo_max_amount: Some(Amount::from_sat(21_000_000)),
        dust: Amount::from_sat(330),
        fees: None,
        scheduled_session: None,
        deprecated_signers: Vec::new(),
        service_status: HashMap::new(),
        digest: "50da3e81cba4844be3559638cf7104a64e30c616bd5862e86b3903222ece0994".to_string(),
        max_tx_weight: 40000,
        max_op_return_outputs: 3,
    }
}
