use html_escape::encode_text_to_string;
use rust_decimal::Decimal;

pub fn encode_bitcoin_uri(
    address: &String,
    amount: &Decimal,
    label: &String,
    message: &String,
    network: &String,
) -> String {
    let mut escaped_label = "".to_string();
    let mut escaped_msg = "".to_string();
    encode_text_to_string(label, &mut escaped_label);
    encode_text_to_string(message, &mut escaped_msg);
    let is_testnet: bool = network == "testnet" || network == "signet";
    if is_testnet {
        return format!(
            "bitcoin:?tb={}&amount={}&label={}&message={}",
            address, amount, escaped_label, escaped_msg
        );
    }
    format!(
        "bitcoin:{}?amount={}&label={}&message={}",
        address, amount, escaped_label, escaped_msg
    )
}
