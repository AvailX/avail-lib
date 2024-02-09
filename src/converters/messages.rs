use snarkvm::prelude::*;

use crate::errors::AvailResult;

pub fn utf8_string_to_bits(str_in: &str) -> Vec<bool> {
    let mut result = Vec::<bool>::new();
    let bytes_in = str_in.as_bytes();

    for one_ch in bytes_in {
        for bitpos in (0..8).rev() {
            let bit = *one_ch & (1 << bitpos) != 0;
            result.push(bit);
        }
    }

    result
}

pub fn field_to_fields<N: Network>(fld: &Field<N>) -> AvailResult<Vec<Field<N>>> {
    Ok(Value::<N>::from(Literal::Field(*fld)).to_fields()?)
}
