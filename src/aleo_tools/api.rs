pub mod blocking;

use snarkvm::prelude::*;

use super::*;

/// Aleo API client for interacting with the Aleo Beacon API
#[derive(Clone, Debug)]
pub struct AleoAPIClient<N: Network> {
    client: ureq::Agent,
    base_url: String,
    network_id: String,
    _network: PhantomData<N>,
}

impl<N: Network> AleoAPIClient<N> {
    pub fn new(base_url: &str, chain: &str) -> Result<Self> {
        let client = ureq::Agent::new();
        ensure!(
            base_url.starts_with("http://") || base_url.starts_with("https://"),
            "specified url {base_url} invalid, the base url must start with or https:// (or http:// if doing local development)"
        );
        Ok(AleoAPIClient {
            client,
            base_url: base_url.to_string(),
            network_id: chain.to_string(),
            _network: PhantomData,
        })
    }

    pub fn testnet3() -> Self {
        Self::new("https://api.explorer.aleo.org/v1", "testnet3").unwrap()
    }

    pub fn local_testnet3(port: &str, ip: &str) -> Self {
        Self::new(&format!("http://{}:{}", ip, port), "testnet3").unwrap()
    }

    /// Get base URL
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// Get network ID being interacted with
    pub fn network_id(&self) -> &str {
        &self.network_id
    }
}
