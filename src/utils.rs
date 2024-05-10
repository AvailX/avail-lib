use snarkvm::console::program::Network;

use crate::{
    errors::{AvailError, AvailErrorType, AvailResult},
    models::mobile_prover::ProverRequest,
    service_clients::get_prover_client_with_session,
};

pub async fn delegate_execution(request: ProverRequest) -> AvailResult<String> {
    let res = get_prover_client_with_session(
        tauri_plugin_http::reqwest::Method::POST,
        "delegateProving",
    )?
    .json(&request)
    .send()
    .await
    .unwrap();
    println!("Prover Response{:?}", res);
    if res.status() == 200 {
        Ok(res.text().await.unwrap())
    } else {
        Err(AvailError::new(
            AvailErrorType::External,
            "Error delegating execution".to_string(),
            "Error delegating execution".to_string(),
        ))
    }
}
