use std::str::FromStr;

use snarkvm::prelude::{Address, MainnetV0, Network, Signature, TestnetV0};
use tracing::info;

use crate::{
    converters::messages::{field_to_fields, utf8_string_to_bits},
    errors::{AvailError, AvailErrorType, AvailResult},
    models::{academy::web_user::UserVerificationRequest, network::SupportedNetworks},
};

use super::program_manager::network;

pub fn verify_signature(
    verification_object: UserVerificationRequest,
    address: &str,
) -> AvailResult<bool> {
    let network = SupportedNetworks::from_str(&verification_object.network)?;
    println!("Verification Object: {:?}", verification_object);
    info!("Verification Object: {:?}", verification_object);
    let result = match network {
        SupportedNetworks::Mainnet => verify_signature_raw::<MainnetV0>(
            &verification_object.message,
            address,
            &verification_object.sign,
        ),
        SupportedNetworks::Testnet => verify_signature_raw::<TestnetV0>(
            &verification_object.message,
            address,
            &verification_object.sign,
        ),
        _ => Err(AvailError::new(
            AvailErrorType::Network,
            "Incorrect Network".to_string(),
            "Incorrect Network".to_string(),
        )),
    }?;
    Ok(result)
}

fn verify_signature_raw<N: Network>(
    message: &str,
    address: &str,
    signature: &str,
) -> AvailResult<bool> {
    let signature = Signature::<N>::from_str(signature)?;
    let address = Address::<N>::from_str(address)?;
    info!("Signature: {:?}", signature);
    info!("Address: {:?}", address);
    let msg_bits = utf8_string_to_bits(message);
    let msg_field = N::hash_bhp512(&msg_bits)?;
    let msg = field_to_fields(&msg_field)?;

    let result = signature.verify_bytes(&address, message.as_bytes());
    info!("Signature Verification Result: {:?}", result);
    println!("Signature Verification Result: {:?}", result);
    Ok(result)
}

#[cfg(test)]
mod tests {
    use snarkvm::prelude::PrivateKey;

    use super::*;
    use crate::models::academy::web_user::WebUser;

    #[test]
    fn test_signature() {
        let message = "Hello World";
        let address = "aleo1633synkqz94vjcxfwy5yn60kvjz8c7heck8ym8tfahxrt443lqgsd49rqd";
        let PK = PrivateKey::<TestnetV0>::from_str(
            "APrivateKey1zkp3kgfxbSumSen6i5Vyszymi38HqEoUyVJFR9rdEJmNo3z",
        )
        .unwrap();
        let rng = &mut rand::thread_rng();

        let msg = utf8_string_to_bits(message);
        println!("Message: {:?}", msg);
        let msg_field = TestnetV0::hash_bhp512(&msg).unwrap();
        println!("Message Field: {:?}", msg_field);
        let msg = field_to_fields(&msg_field).unwrap();
        println!("Message Fields: {:?}", msg);
        let sign = PK.sign(&msg, rng).unwrap();
        println!("Signature: {:?}", sign);
        println!("Signature: {:?}", sign.to_string());
        let res =
            verify_signature_raw::<TestnetV0>(message, address, sign.to_string().as_str()).unwrap();
        println!("Result: {:?}", res);
    }
    #[test]
    fn test_signature_learnaleo() {
        let message = "HelloWorld";
        let address = "aleo153yrfe7ag2d634dwgm3236y7t0s0py9y7vlnkmdkhle9qcp0puqqlvzr3u";
        let ver_obj = UserVerificationRequest {
            sign: "sign17mxwls3yjhu2sg65m4zahudez84e4sd45u5gp6c9p3g407tpygp4l5mau0wvwv8w3vsm65k4ljvasdtaftx0h8psm5nz0zqd2qlm2qd50rjzj6nkl0ajqcanufunjz3vx3ldxw8z8dwyk86tzm3yc6jjpaj4autvrpf9f697ucd2d9jpmwjplfqq83rra9juz4d739vza9nsvpktw0k".to_string(),
            message: message.to_string(),
            network: "mainnetbeta".to_string(),
        };
        let res = verify_signature(ver_obj, &address).unwrap();
        println!("Result: {:?}", res);
    }
}
