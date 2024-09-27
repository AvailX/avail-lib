use std::os::unix::process;

use reqwest::Client;
use snarkvm::cli;
use tracing::{error, info};
use uuid::Uuid;

use crate::{
    errors::{AvailError, AvailErrorType, AvailResult},
    models::academy::{module::Module, progress::ModuleProgress},
};

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

pub async fn increment_xp(address: String, xp: i64, is_local: bool) -> AvailResult<()> {
    let client = Client::new();
    let api = env!("TEST_API_URL");
    let mut req_url = format!("{api}/learn/incrementXP/{address}/{xp}");
    if is_local {
        req_url = format!("http://localhost:8004/learn/incrementXP/{address}/{xp}");
    }
    info!("req_url: {:?}", req_url);
    match client.put(&req_url).send().await {
        Ok(res) => AvailResult::Ok(()),

        Err(e) => {
            error!("Error sending request: {:?}", e);
            Err(AvailError::new(
                AvailErrorType::NotFound,
                "User not found in user-service".to_string(),
                "User not found in user-service".to_string(),
            ))
        }
    }
}

pub async fn check_if_module_completion(
    address: String,
    m_id: Uuid,
    is_local: bool,
) -> AvailResult<bool> {
    let client = Client::new();
    let api = env!("TEST_API_URL");
    let mut module_progress_url = format!("{api}/moduleProgress/progress/{address}/{m_id}");
    if is_local {
        module_progress_url =
            format!("http://localhost:8004/moduleProgress/progress/{address}/{m_id}");
    }
    let module = fetch_module(m_id, is_local).await?;

    info!("module: {:?}", module);

    // FETCH MODULE PROGRESS OF USER
    info!("req_url: {:?}", module_progress_url);

    match client.get(&module_progress_url).send().await {
        Ok(res) => match res.json::<ModuleProgress>().await {
            Ok(result) => {
                if result.is_completed {
                    return AvailResult::Ok(true);
                }
                if result.lessons_completed.len() != module.lessons.len() {
                    return AvailResult::Ok(false);
                }
                if check_if_lesson_present(module.clone().lessons, result.lessons_completed) {
                    process_module_completion(address, m_id, Some(module.clone()), is_local)
                        .await?;
                    AvailResult::Ok(true)
                } else {
                    AvailResult::Ok(false)
                }
            }
            Err(e) => Err(AvailError::new(
                AvailErrorType::InvalidData,
                e.to_string(),
                e.to_string(),
            )),
        },
        Err(e) => {
            error!("Error sending request: {:?}", e);
            Err(AvailError::new(
                AvailErrorType::NotFound,
                "User not found in user-service".to_string(),
                "User not found in user-service".to_string(),
            ))
        }
    }
}

pub async fn fetch_module(m_id: Uuid, is_local: bool) -> AvailResult<Module> {
    let client = Client::new();
    let api = env!("TEST_API_URL");
    let mut req_url = format!("{api}/modules/get/{m_id}");
    if is_local {
        req_url = format!("http://localhost:8004/modules/get/{m_id}");
    }
    info!("req_url: {:?}", req_url);
    match client.get(&req_url).send().await {
        Ok(res) => match res.json::<Module>().await {
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

pub fn check_if_lesson_present(
    module_lessons: Vec<Option<Uuid>>,
    lessons_completed: Vec<Option<Uuid>>,
) -> bool {
    let mut present = true;
    for lesson in module_lessons {
        if !lessons_completed.contains(&lesson) {
            present = false;
            break;
        }
    }
    present
}

pub async fn process_module_completion(
    address: String,
    m_id: Uuid,
    module: Option<Module>,
    is_local: bool,
) -> AvailResult<()> {
    // checks if module is present
    let client = Client::new();
    let api = env!("TEST_API_URL");
    // if module.is_none() {
    //     let module = fetch_module(m_id, is_local).await?;
    //     match check_if_module_completion(address, m_id, is_local).await {
    //         Ok(result) => {
    //             if result {
    //                 match increment_xp(address, module.clone().xp, is_local).await {
    //                     Ok(_) => {}
    //                     Err(e) => {
    //                         return Err(e);
    //                     }
    //                 };

    //                 match update_user_progress(address, m_id, is_local).await {
    //                     Ok(_) => {}
    //                     Err(e) => {
    //                         return Err(e);
    //                     }
    //                 };
    //             } else {
    //                 AvailResult::Ok(());
    //             }
    //         }
    //         Err(e) => Err(e),
    //     }

    //     //
    // }

    match increment_xp(address.clone(), module.clone().unwrap().xp, is_local).await {
        Ok(_) => {}
        Err(e) => {
            return Err(e);
        }
    };

    match update_user_progress(address.clone(), m_id, is_local).await {
        Ok(_) => {}
        Err(e) => {
            return Err(e);
        }
    };

    Ok(())
}

pub async fn update_user_progress(address: String, m_id: Uuid, is_local: bool) -> AvailResult<()> {
    let client = Client::new();
    let api = env!("TEST_API_URL");
    let ts = chrono::Utc::now().to_string();
    let mut req_url = format!("{api}/moduleProgress/updateCompletedAt/{address}/{m_id}/{ts}");
    if is_local {
        req_url =
            format!("http://localhost:8004/moduleProgress/updateCompletedAt/{address}/{m_id}/{ts}");
    }
    info!("req_url: {:?}", req_url);
    match client.put(&req_url).send().await {
        Ok(res) => AvailResult::Ok(()),

        Err(e) => {
            error!("Error sending request: {:?}", e);
            Err(AvailError::new(
                AvailErrorType::NotFound,
                "User not found in user-service".to_string(),
                "User not found in user-service".to_string(),
            ))
        }
    }
}

// pub async fn update_total_progress(address: String, m_id: Uuid, is_local: bool) -> AvailResult<()> {
//     let client = Client::new();
//     let api = env!("TEST_API_URL");
//     let mut req_url = format!("{api}/totalProgress/totalProgress/{address}");
//     if is_local {
//         req_url = format!("http://localhost:8004/learn/totalProgress/totalProgress/{address}");
//     }
//     info!("req_url: {:?}", req_url);
//     match client.put(&req_url).send().await {
//         Ok(res) => AvailResult::Ok(()),

//         Err(e) => {
//             error!("Error sending request: {:?}", e);
//             Err(AvailError::new(
//                 AvailErrorType::NotFound,
//                 "User not found in user-service".to_string(),
//                 "User not found in user-service".to_string(),
//             ))
//         }
//     }
// }

pub async fn calculate_total_progress(is_local: bool, modules_completed: i32) -> AvailResult<i32> {
    let client = Client::new();
    let api = env!("TEST_API_URL");
    let mut req_url = format!("{api}/modules/all");
    if is_local {
        req_url = format!("http://localhost:8004/learn/modules/all");
    }
    info!("req_url: {:?}", req_url);
    match client.get(&req_url).send().await {
        Ok(res) => match res.json::<Vec<Module>>().await {
            Ok(result) => {
                let total_modules: i32 = result.len() as i32;
                let progress: i32 = if total_modules > 0 {
                    ((modules_completed as f64 / total_modules as f64) * 100.0).round() as i32
                } else {
                    0 // Handle the case where total_modules is zero to avoid division by zero
                };
                info!(
                    "{:?} // {:?}  ---> progress: {:?}",
                    modules_completed, total_modules, progress
                );
                AvailResult::Ok(progress)
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
                "User not found in user-service".to_string(),
                "User not found in user-service".to_string(),
            ))
        }
    }
}
