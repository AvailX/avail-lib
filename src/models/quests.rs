use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use snarkvm::prelude::{Field,Network};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Quest {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub display_image: String,
    pub tasks: Vec<Task>,
    pub reward: Reward,
    pub expires_on: DateTime<Utc>,
    pub created_on: DateTime<Utc>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub transaction: bool,
    pub program_id: Option<String>,
    pub function_id: Option<String>,
    pub dapp_url: Option<String>,
    pub points: i32,
}

#[derive(Deserialize, Serialize, Debug,Clone)]
pub struct Reward {
    pub id: Uuid,
    pub collection_name: String,
    pub amount: i32,
}

/* API Request/Response types */
#[derive(Deserialize, Serialize, Debug)]
pub struct QuestsResponse {
    pub quests: Vec<Quest>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(bound = "N: Network")]
pub struct VerifyTaskRequest<N:Network> {
    pub task_id: Uuid,
    pub confirmation_height: u32,
    pub transaction_id: N::TransactionID,
    pub transition_id: N::TransitionID,
    pub tvk : Field<N>
}

#[derive(Deserialize, Serialize, Debug)]
pub struct VerifyTaskResponse {
    pub verified: bool,
}

impl VerifyTaskResponse {
    pub fn new(verified: bool) -> Self {
        Self { verified }
    }
}