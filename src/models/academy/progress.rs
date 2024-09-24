use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use snarkvm::circuit::modules;
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug)]
pub struct TotalProgress {
    pub user_id: Uuid,
    pub address: String,
    pub progress_percentage: i32,
    pub modules_completed: Vec<Option<Uuid>>,
    pub lessons_completed: Vec<Option<Uuid>>,
    pub modules_in_progress: Vec<Option<Uuid>>,
}

impl TotalProgress {
    pub fn new(
        user_id: Uuid,
        address: String,
        progress_percentage: i32,
        modules_completed: Vec<Option<Uuid>>,
        lessons_completed: Vec<Option<Uuid>>,
        modules_in_progress: Vec<Option<Uuid>>,
    ) -> Self {
        Self {
            user_id,
            address,
            progress_percentage,
            modules_completed,
            lessons_completed,
            modules_in_progress,
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ModuleProgress {
    pub user_id: Uuid,
    pub address: String,
    pub module_id: Uuid,
    pub progress_percentage: i32,
    pub is_completed: bool,
    pub score: i32,
    pub time_taken: i64,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub lessons_completed: Vec<Option<Uuid>>,
}

impl ModuleProgress {
    pub fn new(
        user_id: Uuid,
        address: String,
        module_id: Uuid,
        progress_percentage: i32,
        is_completed: bool,
        score: i32,
        time_taken: i64,
        started_at: Option<DateTime<Utc>>,
        completed_at: Option<DateTime<Utc>>,
        lessons_completed: Vec<Option<Uuid>>,
    ) -> Self {
        Self {
            user_id,
            address,
            module_id,
            progress_percentage,
            is_completed,
            score,
            time_taken,
            started_at,
            completed_at,
            lessons_completed,
        }
    }
    pub fn calculate_time_taken(&self) -> i64 {
        let time_taken = self.completed_at.unwrap() - self.started_at.unwrap();
        time_taken.num_hours()
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TotalProgressCreationRequest {
    pub user_id: Uuid,
    pub address: String,
    pub progress_percentage: i32,
    pub modules_completed: Vec<Option<Uuid>>,
    pub lessons_completed: Vec<Option<Uuid>>,
    pub modules_in_progress: Vec<Option<Uuid>>,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct ModuleProgressCreationRequest {
    pub user_id: Uuid,
    pub address: String,
    pub module_id: Uuid,
    pub progress_percentage: i32,
    pub is_completed: bool,
    pub score: i32,
    pub time_taken: i64,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub lessons_completed: Vec<Option<Uuid>>,
}
