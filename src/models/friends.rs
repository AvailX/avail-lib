use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relationship {
    pub user1_id: Uuid,
    pub user2_id: Uuid,
}
