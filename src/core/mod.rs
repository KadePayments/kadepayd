use ::bitcoin::XOnlyPublicKey;
use ::bitcoin::base64::Engine;
use ::bitcoin::base64::engine::general_purpose;
use ::bitcoin::bip32::{ChildNumber, Xpub};
use ::bitcoin::key::Secp256k1;
use ::bitcoin::secp256k1::All;
use qrcode::render::svg;
use qrcode::{EcLevel, QrCode};
use rust_decimal::Decimal;
use std::str::FromStr;
use tonic::Status;

pub mod arkade;
pub mod bitcoin;

pub struct KadeHDWallet;
impl KadeHDWallet {
    fn derive_child_key(
        secp: &Secp256k1<All>,
        x_pub_key: String,
        prev_index: u32,
    ) -> Result<XOnlyPublicKey, Status> {
        let account_index = ChildNumber::from_normal_idx(0)
            .map_err(|_| Status::internal("Failed to create account index: 0"))?;
        let child_index = ChildNumber::from_normal_idx(prev_index)
            .map_err(|_| Status::internal(format!("Invalid child number: {}", prev_index)))?;
        let path = [account_index, child_index];

        let parent_xpub = match Xpub::from_str(x_pub_key.as_str()) {
            Ok(x_pub) => x_pub,
            Err(_) => return Err(Status::invalid_argument("Invalid xpubkey")),
        };
        let child_xpub = match parent_xpub.derive_pub(&secp, &path) {
            Ok(xpub) => xpub,
            Err(_) => return Err(Status::internal("Failed to derive child xpubkey")),
        };
        Ok(child_xpub.to_x_only_pub())
    }
}

pub fn svg_qr_code(data: &str, asset_dir: String) -> Result<String, Status> {
    let code = match QrCode::with_error_correction_level(data, EcLevel::H) {
        Ok(code) => code,
        Err(_) => return Err(Status::internal("Failed to generate qr code")),
    };
    let svg_string = code.render::<svg::Color>().min_dimensions(280, 280).build();

    let logo_bytes = match std::fs::read(format!("{asset_dir}/icons/kadepay.png")) {
        Ok(bytes) => bytes,
        Err(_) => return Err(Status::internal("Failed to read QR code logo")),
    };
    let logo_base64 = general_purpose::STANDARD.encode(&logo_bytes);

    let inline_logo_url = format!("data:image/png;base64,{}", logo_base64);

    let logo_size = 12;
    let logo_x_y = (100 - logo_size) / 2;
    let background_margin = 5;
    let background_size = logo_size + (background_margin * 2);
    let background_radius = background_size / 2;
    let center = 50;

    let overlay_tag = format!(
        "<defs>
            <clipPath id=\"circle-clip\">
                <circle cx=\"{}%\" cy=\"{}%\" r=\"{}%\" />
            </clipPath>
        </defs>

        <circle cx=\"{}%\" cy=\"{}%\" r=\"{}%\" fill=\"#FFFFFF\" />

        <image href=\"{}\"
               x=\"{}%\"
               y=\"{}%\"
               width=\"{}%\"
               height=\"{}%\"
               clip-path=\"url(#circle-clip)\" />
        </svg>",
        center,
        center,
        background_radius,
        center,
        center,
        background_radius,
        inline_logo_url,
        logo_x_y,
        logo_x_y,
        logo_size,
        logo_size,
    );

    let final_svg = svg_string.replace("</svg>", &overlay_tag).replace(
        "<rect x=\"0\" y=\"0\"",
        "<rect x=\"0\" y=\"0\" rx=\"3%\" ry=\"3%\"",
    );
    Ok(final_svg)
}

pub fn sats_to_btc(sats: Decimal) -> Option<Decimal> {
    sats.checked_div(100_000_000.into())
}

pub fn sats_from_str(amount: &str) -> Result<Decimal, Status> {
    let sats = match Decimal::from_str(amount) {
        Ok(sats) => sats,
        Err(_) => return Err(Status::invalid_argument("Failed to parse invoice amount")),
    };
    match sats_to_btc(sats) {
        Some(sats) => Ok(sats),
        None => Err(Status::invalid_argument("Failed to parse sat amount")),
    }
}

pub fn format_amount(amount: &str, currency_code: &str) -> Result<String, Status> {
    let amount = if currency_code == "SATS" {
        let sats = match Decimal::from_str(amount) {
            Ok(sats) => sats,
            Err(_) => return Err(Status::invalid_argument("Failed to parse invoice amount")),
        };
        let number_of_digits = count_digits(sats);
        if number_of_digits > 5 {
            let btc = match sats_to_btc(sats) {
                Some(btc) => btc.normalize(),
                None => return Err(Status::invalid_argument("Failed to convert sats into btc")),
            };
            format!("₿{btc}")
        } else {
            format!("{} SATS", sats.to_string())
        }
    } else {
        format!("{amount} {currency_code}")
    };
    Ok(amount)
}

fn count_digits(mut number: Decimal) -> usize {
    number = number.normalize();

    let mantissa = number.mantissa();

    if mantissa == 0 {
        return 1;
    }

    let mut abs_mantissa = mantissa.abs();
    let mut count = 0;

    while abs_mantissa > 0 {
        abs_mantissa /= 10;
        count += 1;
    }

    count
}
