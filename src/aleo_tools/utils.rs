use std::str::FromStr;

use snarkvm::prelude::{Address, Network, Signature};

use crate::{
    converters::messages::{field_to_fields, utf8_string_to_bits},
    errors::AvailResult,
};

pub fn verify_signature<N: Network>(
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
