use ark_core::server::Info;
use ark_grpc::{Client, Error};
use bitcoin::{
    Address, Amount, KnownHrp, Network, ScriptBuf, Sequence, WitnessProgram, WitnessVersion,
};
use std::collections::HashMap;
use tonic::Status;

#[derive(Debug)]
pub struct ArkadeClient {
    client: Client,
}

impl ArkadeClient {
    pub async fn new_connection(url: &str) -> Result<ArkadeClient, Error> {
        let mut client = Client::new(url.to_string());
        client.connect().await?;
        Ok(Self { client })
    }

    pub async fn get_info(&self) -> Result<Info, Status> {
        match self.client.get_info().await {
            Ok(server_info) => Ok(server_info),
            Err(error) => Err(Status::from_error(Box::from(error))),
        }
    }

    // For unit testing
    pub fn get_test_info() -> Info {
        let witness_program =
            hex::decode("15048e41633084bfcae91d03b3c2bb7f6ac78440").expect("failed deserializing");
        let checkpoint_tapscript = hex::decode(
            "03a80040b27520dfcaec558c7e78cf3e38b898ba8a43cfb5727266bae32c5c5b3aeb32c558aa0bac",
        )
        .expect("failed deserializing");
        Info {
            version: "".to_string(),
            signer_pk: "03a19310a999207dbd9a03d20f649e37c7a578a07d75e6fa19aa3f33fc6b15622c"
                .parse()
                .unwrap(),
            forfeit_pk: "03571632039959ffa1724079cbf03522df889d47ebc3085f9589468d765e447d84"
                .parse()
                .unwrap(),
            forfeit_address: Address::from_witness_program(
                WitnessProgram::new(WitnessVersion::V1, &witness_program).unwrap(),
                KnownHrp::Testnets,
            ),
            checkpoint_tapscript: ScriptBuf::from_bytes(checkpoint_tapscript),
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
}
