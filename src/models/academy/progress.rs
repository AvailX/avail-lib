use chrono::{DateTime, Utc};
use diesel::sql_types::Date;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug)]
pub struct TotalProgress {
    user_id: Uuid,
    address: String,
    progress_percentage: i32,
    modules_completed: Vec<Uuid>,
    lessons_completed: Vec<Uuid>,
}

impl TotalProgress {
    pub fn new(
        user_id: Uuid,
        address: String,
        progress_percentage: i32,
        modules_completed: Vec<Uuid>,
        lessons_completed: Vec<Uuid>,
    ) -> Self {
        Self {
            user_id,
            address,
            progress_percentage,
            modules_completed,
            lessons_completed,
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ModuleProgress {
    user_id: Uuid,
    address: String,
    module_id: Uuid,
    progress_percentage: i32,
    is_completed: bool,
    score: i32,
    time_taken: i64,
    started_at: DateTime<Utc>,
    completed_at: DateTime<Utc>,
    lessons_completed: Vec<Uuid>,
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
        started_at: DateTime<Utc>,
        completed_at: DateTime<Utc>,
        lessons_completed: Vec<Uuid>,
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
        let time_taken = self.completed_at - self.started_at;
        time_taken.num_hours()
    }
}
