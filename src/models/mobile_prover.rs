use serde::{Deserialize, Serialize};

use crate::errors::AvailError;
use bincode::{deserialize, serialize};
use snarkvm::prelude::Network;

use super::network::SupportedNetworks;

#[derive(Deserialize, Serialize, Debug, Clone)]
// #[serde(bound = "N: Network")]
pub struct ProverRequest {
    pub address: String,
    pub auth_object_bytes: Vec<u8>,
    pub network: SupportedNetworks,
}

// trait FeeRequest

// impl <N:Network> FeeRequest<N> {
impl ProverRequest {
    pub fn new(address: String, auth_object_bytes: Vec<u8>, network: SupportedNetworks) -> Self {
        Self {
            address,
            auth_object_bytes,
            network,
        }
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, AvailError> {
        let bytes = serialize(&self)?;
        Ok(bytes)
    }

    pub fn from_bytes(bytes: Vec<u8>) -> Result<Self, AvailError> {
        let data = deserialize(&bytes)?;
        Ok(data)
    }

    pub async fn to_bytes_execution_object<N: Network>(
        auth_object: snarkvm::prelude::Authorization<N>,
    ) -> Result<Vec<u8>, AvailError> {
        let bytes = serialize(&auth_object)?;
        Ok(bytes)
    }

    pub async fn from_bytes_execution_object<N: Network>(
        bytes: Vec<u8>,
    ) -> Result<snarkvm::prelude::Authorization<N>, AvailError> {
        let data: snarkvm::prelude::Authorization<N> = deserialize(&bytes)?;
        Ok(data)
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
// #[serde(bound = "N: Network")]
pub struct ProverResponse {
    pub txn_bytes: Vec<u8>,
    pub network: SupportedNetworks,
}

// trait FeeRequest

// impl <N:Network> FeeRequest<N> {
impl ProverResponse {
    pub fn new(txn_bytes: Vec<u8>, network: SupportedNetworks) -> Self {
        Self { txn_bytes, network }
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, AvailError> {
        let bytes = serialize(&self)?;
        Ok(bytes)
    }

    pub fn from_bytes(bytes: Vec<u8>) -> Result<Self, AvailError> {
        let data = deserialize(&bytes)?;
        Ok(data)
    }

    pub async fn to_bytes_execution_object<N: Network>(
        txn_object: snarkvm::prelude::Transaction<N>,
    ) -> Result<Vec<u8>, AvailError> {
        let bytes = serialize(&txn_object)?;
        Ok(bytes)
    }

    pub async fn from_bytes_execution_object<N: Network>(
        bytes: Vec<u8>,
    ) -> Result<snarkvm::prelude::Transaction<N>, AvailError> {
        let data: snarkvm::prelude::Transaction<N> = deserialize(&bytes)?;
        Ok(data)
    }
}
