use reqwest::Client;
use tracing::{error, info};
use uuid::Uuid;

use crate::errors::{AvailError, AvailErrorType, AvailResult};

pub async fn add_text_content_to_lesson(
    module_id: Uuid,
    lesson_id: Uuid,
    text_content_id: Uuid,
    is_local: bool,
) -> Result<(), String> {
    let client = reqwest::Client::new();
    let api = env!("TEST_API_URL");
    let mut url = format!(
        "{}/lessons/update/text/{}/{}",
        api, lesson_id, text_content_id
    );
    if is_local {
        url = format!(
            "http://localhost:8004/lessons/update/text/{}/{}",
            lesson_id, text_content_id
        );
    }
    let res = client.post(&url).send().await;
    match res {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

pub async fn module_exists(m_id: Uuid, is_local: bool) -> AvailResult<bool> {
    let api = env!("TEST_API_URL");

    let client = Client::new();
    let mut req_url = format!("{api}/modules/ifExists/{m_id}");
    if is_local {
        req_url = format!("http://localhost:8004/modules/ifExists/{m_id}");
    }
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

pub async fn lesson_exists(l_id: Uuid, is_local: bool) -> AvailResult<bool> {
    let client = Client::new();
    let api = env!("TEST_API_URL");
    let mut req_url = format!("{api}/lessons/ifExists/{l_id}");
    if is_local {
        req_url = format!("http://localhost:8004/lessons/ifExists/{l_id}");
    }
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

pub async fn add_lesson(m_id: Uuid, l_id: Uuid, is_local: bool) -> AvailResult<()> {
    let client = Client::new();
    let api = env!("TEST_API_URL");
    let mut req_url = format!("{api}/modules/addLesson/{m_id}/{l_id}");
    if is_local {
        req_url = format!("http://localhost:8004/modules/addLesson/{m_id}/{l_id}");
    }
    info!("req_url: {:?}", req_url);
    match client.put(&req_url).send().await {
        Ok(res) => AvailResult::Ok(()),

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
