use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct User {
    pub username: Option<String>,
    pub address: String,
    pub tag: Option<u32>,
    pub backup: bool,
}

impl User {
    pub fn new(username: Option<String>, address: String, tag: Option<u32>, backup: bool) -> Self {
        Self {
            username,
            address,
            tag,
            backup,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateBackupRequest {
    pub backup: bool,
}

impl UpdateBackupRequest {
    pub fn new(backup: bool) -> Self {
        Self { backup }
    }
}
