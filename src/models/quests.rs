use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use snarkvm::prelude::{Field, Network};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Campaign {
    pub id: Uuid,
    pub title: String,
    pub subtitle: String,
    pub description: CampaignDescription,
    pub inner_description: String,
    pub box_image: String,
    pub bg_image: String,
    pub profile_image: String,
    pub color: String,
    pub points_image: String,
    pub project_name: String,
    pub owner_id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CampaignDescription {
    pub part1: String,
    pub main: String,
    pub part2: String,
}

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
    pub campaign_id: Uuid,
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

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Reward {
    pub id: Uuid,
    pub collection_name: String,
    pub amount: i32,
    pub method: RewardMethodCommon,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum RewardMethodCommon {
    LuckyDraw,
    LeaderBoard,
    FCFS,
}

/* API Request/Response types */
#[derive(Deserialize, Serialize, Debug)]
pub struct QuestsResponse {
    pub quests: Vec<Quest>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(bound = "N: Network")]
pub struct VerifyTaskRequest<N: Network> {
    pub task_id: Uuid,
    pub confirmation_height: u32,
    pub transaction_id: N::TransactionID,
    pub transition_id: N::TransitionID,
    pub tvk: Field<N>,
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

#[derive(Deserialize, Serialize, Debug)]
pub struct WhitelistResponse {
    pub collection_name: String,
    pub amount: i32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PointsResponse {
    pub points: i32,
    pub img_src: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Collection {
    pub id: Uuid,
    pub name: String,
    pub whitelist_img: String,
    pub description: String,
    pub inner_img: Option<String>,
    pub twitter_link: Option<String>,
    pub discord_link: Option<String>,
}
