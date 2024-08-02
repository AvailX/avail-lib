use serde::{Deserialize, Serialize};
use serde_json::json;
use snarkvm::console::program::Network;

use crate::{
    errors::{AvailError, AvailErrorType, AvailResult},
    models::mobile_prover::ProverRequest,
    service_clients::{get_marlin_client, get_prover_client_with_session},
};
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SecretAuth {
    pub auth: serde_json::Value,
    pub fee_auth: serde_json::Value,
}
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MarlinRequest {
    pub public: String,
    pub secret: SecretAuth,
}

pub async fn delegate_execution(request: ProverRequest) -> AvailResult<String> {
    let res = get_prover_client_with_session(reqwest::Method::POST, "delegateProving")?
        .json(&request)
        .send()
        .await
        .unwrap();
    println!("Prover Response{:?}", res);
    if res.status() == 200 {
        println!("sss");
        Ok(res.text().await.unwrap())
    } else {
        Err(AvailError::new(
            AvailErrorType::External,
            "Error delegating execution".to_string(),
            "Error delegating execution".to_string(),
        ))
    }
}

pub async fn delegate_execution_marlin(request: MarlinRequest) -> AvailResult<String> {
    let res = get_marlin_client(reqwest::Method::POST, "proveTx")?
        .json(&request)
        .send()
        .await
        .unwrap();
    println!("Prover Response{:?}", res);
    if res.status() == 200 {
        println!("sss");
        Ok(res.text().await.unwrap())
    } else {
        Err(AvailError::new(
            AvailErrorType::External,
            "Error delegating execution".to_string(),
            "Error delegating execution".to_string(),
        ))
    }
}
