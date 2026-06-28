use ark_core::server::Info;
use bitcoin::{
    Address, Amount, KnownHrp, Network, ScriptBuf, Sequence, WitnessProgram, WitnessVersion,
};
use std::collections::HashMap;

pub fn get_ark_server_info() -> Info {
    Info {
        version: "".to_string(),
        signer_pk: "fa73c6e4876ffb2dfc961d763cca9abc73d4b88efcb8f5e7ff92dc55e9aa553d"
            .parse()
            .unwrap(),
        forfeit_pk: "dfcaec558c7e78cf3e38b898ba8a43cfb5727266bae32c5c5b3aeb32c558aa0b"
            .parse()
            .unwrap(),
        forfeit_address: Address::from_witness_program(
            WitnessProgram::new(
                WitnessVersion::V0,
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
