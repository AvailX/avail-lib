use serde::{Deserialize, Serialize};
use snarkvm::{
    console::network::Network,
    prelude::{Address, Ciphertext, Group, Literal, Plaintext, Scalar, StringType, ViewKey},
    utilities::Uniform,
};

use crate::errors::AvailResult;

pub trait Encryptable {
    fn encrypt_for<N: Network>(&self, address: Address<N>) -> AvailResult<EncryptedStruct<N>>;
    fn encrypt_for_multi<N: Network>(
        &self,
        addresses: Vec<Address<N>>,
    ) -> AvailResult<Vec<EncryptedStruct<N>>>;
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(bound = "N: Network")]
pub struct EncryptedStruct<N: Network> {
    pub cipher_text: Ciphertext<N>,
    pub encrypted_for: Address<N>,
    pub nonce: Group<N>,
}

impl<T: Serialize> Encryptable for T {
    fn encrypt_for<N: Network>(&self, address: Address<N>) -> AvailResult<EncryptedStruct<N>> {
        let bytes = bincode::serialize(&self)?;

        let hex_string = hex::encode(bytes);

        // if hex string is larger then 255 characters then we need to split it into multiple strings
        // and encrypt each one separately

        //split hex string into an array of strings of max length 255
        let mut plaintext_array: Vec<Plaintext<N>> = Vec::new();
        let mut hex_string_clone = hex_string.clone();

        if hex_string_clone.len() < 255 {
            let snarkvm_string = StringType::<N>::new(hex_string_clone.as_str());
            let plaintext = Plaintext::Literal(
                Literal::String(snarkvm_string),
                once_cell::sync::OnceCell::new(),
            );
            plaintext_array.push(plaintext);
        } else {
            while hex_string_clone.len() > 255 {
                let (first, second) = hex_string_clone.split_at(255);
                let snarkvm_string = StringType::<N>::new(first);

                let plaintext = Plaintext::Literal(
                    Literal::String(snarkvm_string),
                    once_cell::sync::OnceCell::new(),
                );
                plaintext_array.push(plaintext);
                hex_string_clone = second.to_string();
            }

            let snarkvm_string = StringType::<N>::new(hex_string_clone.as_str());
            let plaintext = Plaintext::Literal(
                Literal::String(snarkvm_string),
                once_cell::sync::OnceCell::new(),
            );
            plaintext_array.push(plaintext);
        }

        let plaintext = Plaintext::Array(plaintext_array, once_cell::sync::OnceCell::new());

        let rng = &mut rand::thread_rng();
        let scalar = Scalar::<N>::rand(rng);
        let nonce = N::g_scalar_multiply(&scalar);

        let cipher_text = plaintext.encrypt(&address, scalar)?;

        Ok(EncryptedStruct {
            cipher_text,
            encrypted_for: address,
            nonce,
        })
    }

    fn encrypt_for_multi<N: Network>(
        &self,
        addresses: Vec<Address<N>>,
    ) -> AvailResult<Vec<EncryptedStruct<N>>> {
        addresses
            .into_iter()
            .map(|address| self.encrypt_for(address))
            .collect()
    }
}

impl<N: Network> EncryptedStruct<N> {
    pub fn new(ciphertext: Ciphertext<N>, encrypted_for: Address<N>, nonce: Group<N>) -> Self {
        Self {
            cipher_text: ciphertext,
            encrypted_for,
            nonce,
        }
    }

    pub fn decrypt<T: serde::de::DeserializeOwned>(&self, view_key: ViewKey<N>) -> AvailResult<T> {
        let plaintext = self.cipher_text.decrypt(view_key, self.nonce)?;

        let hex_string = plaintext
            .to_string()
            .replace(['[', ']', '\"', ',', '\n', ' '], "");

        let bytes = hex::decode(hex_string)?;

        let deserialized: T = bincode::deserialize(&bytes)?;

        Ok(deserialized)
    }
}

#[cfg(test)]
mod tests {

    use serde::Deserialize;
    use snarkvm::prelude::Testnet3;
    use std::str::FromStr;

    use super::*;

    #[derive(Serialize, Deserialize)]
    struct Person {
        pub name: String,
        pub age: u8,
    }

    #[test]
    fn test_encryption() {
        let public_key = "aleo15z3mag4mtdcyh0upephc4dcawfe22znnfkgtxmx3y5xx36q4fvqq93cnff";
        let vk = "AViewKey1tBryiVGTEnJEfVGxa1spRKLfiwPqc7nTnkv62izdSZcC";

        let address = Address::<Testnet3>::from_str(public_key).unwrap();
        let vk = ViewKey::<Testnet3>::from_str(vk).unwrap();

        let p = Person {
            name: String::from("John"),
            age: 32,
        };

        let encrypted = p.encrypt_for(address).unwrap();

        let decrypted_person: Person = encrypted.decrypt(vk).unwrap();

        assert_eq!(decrypted_person.name, "John");
        assert_eq!(decrypted_person.age, 32);
    }

    #[derive(Serialize, Deserialize)]
    struct Contract {
        header: String,
        footer: String,
        title: String,
        party1: String,
        party2: String,
        effective_date: String,
        contract_date: String,
        clause1: String,
        clause2: String,
        clause3: String,
        clause4: String,
        clause5: String,
        clause6: String,
    }

    #[test]
    fn test_c_encryption() {
        let public_key = "aleo15z3mag4mtdcyh0upephc4dcawfe22znnfkgtxmx3y5xx36q4fvqq93cnff";
        let vk = "AViewKey1tBryiVGTEnJEfVGxa1spRKLfiwPqc7nTnkv62izdSZcC";

        let address = Address::<Testnet3>::from_str(public_key).unwrap();
        let vk = ViewKey::<Testnet3>::from_str(vk).unwrap();

        let contract = Contract {
            header: "Header Content".to_string(),
            footer: "Footer Content".to_string(),
            title: "Contract Title".to_string(),
            party1: "Party One".to_string(),
            party2: "Party Two".to_string(),
            effective_date: "2023-01-01".to_string(),
            contract_date: "2023-01-10".to_string(),
            clause1: "Clause 1 Content".to_string(),
            clause2: "Clause 2 Content".to_string(),
            clause3: "Clause 3 Content".to_string(),
            clause4: "Clause 4 Content".to_string(),
            clause5: "Clause 5 Content".to_string(),
            clause6: "Clause 6 Content".to_string(),
        };

        let encrypted = contract.encrypt_for(address).unwrap();

        let decrypted_contract: Contract = encrypted.decrypt(vk).unwrap();

        assert_eq!(decrypted_contract.header, "Header Content");
        assert_eq!(decrypted_contract.footer, "Footer Content");
        assert_eq!(decrypted_contract.title, "Contract Title");
        assert_eq!(decrypted_contract.party1, "Party One");
        assert_eq!(decrypted_contract.party2, "Party Two");
        assert_eq!(decrypted_contract.effective_date, "2023-01-01");
        assert_eq!(decrypted_contract.contract_date, "2023-01-10");
        assert_eq!(decrypted_contract.clause1, "Clause 1 Content");
        assert_eq!(decrypted_contract.clause2, "Clause 2 Content");
        assert_eq!(decrypted_contract.clause3, "Clause 3 Content");
        assert_eq!(decrypted_contract.clause4, "Clause 4 Content");
        assert_eq!(decrypted_contract.clause5, "Clause 5 Content");
        assert_eq!(decrypted_contract.clause6, "Clause 6 Content");
    }
}
