use std::str::FromStr;

use snarkvm::prelude::{Address, MainnetV0, Network, Signature, TestnetV0};

use crate::{
    converters::messages::{field_to_fields, utf8_string_to_bits},
    errors::{AvailError, AvailErrorType, AvailResult},
    models::{network::SupportedNetworks, web_user::UserVerificationRequest},
};

use super::program_manager::network;

pub fn verify_signature(
    verification_object: UserVerificationRequest,
    address: &str,
) -> AvailResult<bool> {
    let network = SupportedNetworks::from_str(&verification_object.network)?;
    let result = match network {
        // SupportedNetworks::Mainnet => verify_signature_raw::<MainnetV0>(
        //     &verification_object.message,
        //     address,
        //     &verification_object.sign,
        // ),
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

    let msg_bits = utf8_string_to_bits(message);
    let msg_field = N::hash_bhp512(&msg_bits)?;
    let msg = field_to_fields(&msg_field)?;

    let result = signature.verify(&address, &msg);

    Ok(result)
}
