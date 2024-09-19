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

pub async fn add_quiz_content_to_lesson(
    module_id: Uuid,
    lesson_id: Uuid,
    quiz_content_id: Uuid,
    is_local: bool,
) -> Result<(), String> {
    let client = reqwest::Client::new();
    let api = env!("TEST_API_URL");
    let mut url = format!(
        "{}/lessons/update/quiz/{}/{}",
        api, lesson_id, quiz_content_id
    );
    if is_local {
        url = format!(
            "http://localhost:8004/lessons/update/quiz/{}/{}",
            lesson_id, quiz_content_id
        );
    }
    let res = client.post(&url).send().await;
    match res {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

pub async fn add_video_content_to_lesson(
    module_id: Uuid,
    lesson_id: Uuid,
    video_content_id: Uuid,
    is_local: bool,
) -> Result<(), String> {
    let client = reqwest::Client::new();
    let api = env!("TEST_API_URL");
    let mut url = format!(
        "{}/lessons/update/video/{}/{}",
        api, lesson_id, video_content_id
    );
    if is_local {
        url = format!(
            "http://localhost:8004/lessons/update/video/{}/{}",
            lesson_id, video_content_id
        );
    }
    let res = client.post(&url).send().await;
    match res {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
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
