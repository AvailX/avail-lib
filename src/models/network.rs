use crate::errors::{AvailError, AvailErrorType, AvailResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum SupportedNetworks {
    Testnet3,
}

use std::str::FromStr;

impl FromStr for SupportedNetworks {
    type Err = AvailError;

    fn from_str(network_str: &str) -> AvailResult<Self> {
        Self::parse_network(network_str)
    }
}

impl SupportedNetworks {
    fn parse_network(network_str: &str) -> AvailResult<Self> {
        let supported_network = match network_str {
            "testnet3" => SupportedNetworks::Testnet3,
            _ => {
                return Err(AvailError::new(
                    AvailErrorType::Network,
                    "Unsupported network".to_string(),
                    "Unsupported network".to_string(),
                ))
            }
        };

        Ok(supported_network)
    }
}
