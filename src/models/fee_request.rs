use serde::{Deserialize, Serialize};

use crate::errors::AvailError;
use bincode::{deserialize, serialize};
use snarkvm::prelude::Network;

use super::network::SupportedNetworks;

#[derive(Deserialize, Serialize, Debug, Clone)]
// #[serde(bound = "N: Network")]
pub struct FeeRequest {
    pub execution_object: Vec<u8>,
    pub program_id: String,
    pub function_id: String,
    pub network: SupportedNetworks,
}

// trait FeeRequest

// impl <N:Network> FeeRequest<N> {
impl FeeRequest {
    pub fn new(
        execution_object: Vec<u8>,
        program_id: String,
        function_id: String,
        network: SupportedNetworks,
    ) -> Self {
        Self {
            execution_object,
            program_id,
            function_id,
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
        execution_object: snarkvm::prelude::Execution<N>,
    ) -> Result<Vec<u8>, AvailError> {
        let bytes = serialize(&execution_object)?;
        Ok(bytes)
    }

    pub async fn from_bytes_execution_object<N: Network>(
        bytes: Vec<u8>,
    ) -> Result<snarkvm::prelude::Execution<N>, AvailError> {
        let data: snarkvm::prelude::Execution<N> = deserialize(&bytes)?;
        Ok(data)
    }
}
