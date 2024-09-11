use crate::errors::{AvailError, AvailErrorType, AvailResult};
use reqwest::Client;
use tracing::error;
use uuid::Uuid;

use super::module::Module;

pub async fn module_exists(m_id: Uuid) -> AvailResult<bool> {
    let client = Client::new();
    // let req_url = format!("https://test-api.avail.global/modules/ifExists/{m_id}");
    let req_url = format!("http://localhost:8004/modules/ifExists/{m_id}");
    match client.get(&req_url).send().await {
        Ok(res) => match res.json::<bool>().await {
            Ok(result) => AvailResult::Ok(result),
            Err(e) => Err(AvailError::new(
                AvailErrorType::InvalidData,
                "JSON parsing error".to_string(),
                "JSON parsing error".to_string(),
            )),
        },
        Err(e) => {
            error!("Error sending request: {:?}", e);
            Err(AvailError::new(
                AvailErrorType::NotFound,
                "Module not found in module-service".to_string(),
                "Module not found in module-service".to_string(),
            ))
        }
    }
}

pub async fn lesson_exists(l_id: Uuid) -> AvailResult<bool> {
    let client = Client::new();
    // let req_url = format!("https://test-api.avail.global/lessons/ifExists/{l_id}");
    let req_url = format!("http://localhost:8004/lessons/ifExists/{l_id}");

    match client.get(&req_url).send().await {
        Ok(res) => match res.json::<bool>().await {
            Ok(result) => AvailResult::Ok(result),
            Err(e) => Err(AvailError::new(
                AvailErrorType::InvalidData,
                "JSON parsing error".to_string(),
                "JSON parsing error".to_string(),
            )),
        },
        Err(e) => {
            error!("Error sending request: {:?}", e);
            Err(AvailError::new(
                AvailErrorType::NotFound,
                "Module not found in module-service".to_string(),
                "Module not found in module-service".to_string(),
            ))
        }
    }
}

pub async fn if_lesson_is_of_module(l_id: Uuid, m_id: Uuid) -> AvailResult<bool> {
    let client = Client::new();
    // let req_url = format!("https://test-api.avail.global/modules/get/{l_id}");
    let req_url = format!("http://localhost:8004/modules/get/{m_id}");

    match client.get(&req_url).send().await {
        Ok(res) => match res.json::<Module>().await {
            Ok(result) => {
                if result.lessons.contains(&Some(l_id)) {
                    AvailResult::Ok(true)
                } else {
                    AvailResult::Ok(false)
                }
            }
            Err(e) => Err(AvailError::new(
                AvailErrorType::InvalidData,
                "JSON parsing error".to_string(),
                "JSON parsing error".to_string(),
            )),
        },
        Err(e) => {
            error!("Error sending request: {:?}", e);
            Err(AvailError::new(
                AvailErrorType::NotFound,
                "Module not found in module-service".to_string(),
                "Module not found in module-service".to_string(),
            ))
        }
    }
}
