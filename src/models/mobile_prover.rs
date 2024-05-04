use core::fmt;
use std::fmt::Display;

use serde::{Deserialize, Serialize};
use snarkvm::ledger::query::Query;
use snarkvm::ledger::store::helpers::memory::BlockMemory;
use snarkvm::ledger::store::BlockStorage;
use snarkvm::ledger::store::ConsensusStorage;
use snarkvm::synthesizer::vm;
use snarkvm::synthesizer::Authorization;
use snarkvm::synthesizer::VM;

use super::network::SupportedNetworks;
use crate::errors::AvailError;
use bincode::{deserialize, serialize};
use snarkvm::prelude::CryptoRng;
use snarkvm::prelude::Network;
use snarkvm::prelude::Rng;

// #[derive(Deserialize, Serialize, Debug, Clone)]
#[derive(Clone)]
// #[serde(bound = "N: Network")]
pub struct ProverRequest<N: Network> {
    // pub struct ProverRequest {
    pub address: String,
    pub auth: Authorization<N>,
    pub network: SupportedNetworks,
    pub fee_auth: Option<Authorization<N>>,
    pub rng: Vec<u8>,                    // maybe add rng
    pub query: Query<N, BlockMemory<N>>, // pub vm: VM<N, C>,
}

impl<N: Network> ProverRequest<N> {
    // impl ProverRequest {

    pub fn new(
        address: String,
        auth: Authorization<N>,
        network: SupportedNetworks,
        fee_auth: Option<Authorization<N>>,
        rng: Vec<u8>,
        query: Query<N, BlockMemory<N>>,
        // vm: VM<N, C>,
    ) -> Self {
        Self {
            address,
            auth,
            network,
            fee_auth,
            rng,
            query, // vm,
        }
    }

    // pub fn to_bytes(&self) -> Result<Vec<u8>, AvailError> {
    //     let bytes = serialize(&self)?;
    //     Ok(bytes)
    // }

    // pub fn from_bytes(bytes: Vec<u8>) -> Result<Self, AvailError> {
    //     let data = deserialize(&bytes)?;
    //     Ok(data)
    // }

    // pub fn to_bytes_auth_object(
    //     auth_object: snarkvm::prelude::Authorization<N>,
    // ) -> Result<Vec<u8>, AvailError> {
    //     let bytes = serialize(&auth_object)?;
    //     Ok(bytes)
    // }

    // pub fn from_bytes_auth_object(
    //     bytes: Vec<u8>,
    // ) -> Result<snarkvm::prelude::Authorization<N>, AvailError> {
    //     let data: snarkvm::prelude::Authorization<N> = deserialize(&bytes)?;
    //     Ok(data)
    // }

    // pub fn to_bytes_rng<R: Rng + CryptoRng>(rng: &mut R) -> Result<Vec<u8>, AvailError> {
    //     let bytes = serialize(&rng)?;
    //     Ok(bytes)
    // }

    // pub fn from_bytes_rng<N: Network, C: ConsensusStorage<N>>(
    //     bytes: Vec<u8>,
    // ) -> Result<snarkvm::prelude::Authorization<N>, AvailError> {
    //     let data: snarkvm::prelude::Authorization<N> = deserialize(&bytes)?;
    //     Ok(data)
    // }
}

impl<N: Network> Display for ProverRequest<N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ProverRequest: address: {}, network: {:?}",
            self.address, self.network
        )
    }
}

// #[derive(Deserialize, Serialize, Debug, Clone)]
// // #[serde(bound = "N: Network")]
// pub struct ProverResponse {
//     pub txn_bytes: Vec<u8>,
//     pub network: SupportedNetworks,
// }

// impl ProverResponse {
//     pub fn new(txn_bytes: Vec<u8>, network: SupportedNetworks) -> Self {
//         Self { txn_bytes, network }
//     }

//     pub fn to_bytes(&self) -> Result<Vec<u8>, AvailError> {
//         let bytes = serialize(&self)?;
//         Ok(bytes)
//     }

//     pub fn from_bytes(bytes: Vec<u8>) -> Result<Self, AvailError> {
//         let data = deserialize(&bytes)?;
//         Ok(data)
//     }

//     pub async fn to_bytes_txn_object<N: Network>(
//         txn_object: snarkvm::prelude::Transaction<N>,
//     ) -> Result<Vec<u8>, AvailError> {
//         let bytes = serialize(&txn_object)?;
//         Ok(bytes)
//     }

//     pub async fn from_bytes_txn_object<N: Network>(
//         bytes: Vec<u8>,
//     ) -> Result<snarkvm::prelude::Transaction<N>, AvailError> {
//         let data: snarkvm::prelude::Transaction<N> = deserialize(&bytes)?;
//         Ok(data)
//     }
// }
