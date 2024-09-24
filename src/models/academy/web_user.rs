use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct WebUser {
    pub username: Option<String>,
    pub address: String,
    pub tag: Option<u32>,
    pub created_at: DateTime<Utc>,
    pub streak: i32,
    pub last_login: DateTime<Utc>,
}

impl WebUser {
    pub fn new(
        username: Option<String>,
        address: String,
        tag: Option<u32>,
        created_at: DateTime<Utc>,
        streak: i32,
        last_login: DateTime<Utc>,
    ) -> Self {
        Self {
            username,
            address,
            tag,
            created_at,
            streak,
            last_login,
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UserResponse {
    pub user: Option<WebUser>,
    pub exists: bool,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UserVerificationRequest {
    pub sign: String,
    pub message: String,
    pub network: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CreationUserObject {
    pub username: Option<String>,
    pub address: String,
    pub tag: Option<u32>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UserCreationRequest {
    pub user: CreationUserObject,
    pub verification_object: UserVerificationRequest,
}

// #[derive(Debug, Deserialize, Serialize)]
// pub struct UpdateBackupRequest {
//     pub backup: bool,
// }

// impl UpdateBackupRequest {
//     pub fn new(backup: bool) -> Self {
//         Self { backup }
//     }
// }
// ToDo - Add Level struct with enum for level type
// ToDo - Add Socials struct with String of Social URL's
// ToDo - Add Profile struct with User, Level, and Socials and progress
