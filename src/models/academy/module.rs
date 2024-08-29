use aes_gcm::aes::cipher::typenum::Less;
use chrono::{DateTime, Utc};
use diesel::sql_types::Date;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug)]
pub struct Module {
    pub id: Uuid,
    pub module_name: String,
    pub lesson_count: i32,
    pub lessons: Vec<Option<Uuid>>,
}

impl Module {
    pub fn new(
        id: Uuid,
        module_name: String,
        lesson_count: i32,
        lessons: Vec<Option<Uuid>>,
    ) -> Self {
        Self {
            id,
            module_name,
            lesson_count,
            lessons,
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Lesson {
    pub id: Uuid,
    pub lesson_name: String,
    pub module_id: Uuid,
    pub text_content: Uuid,
    pub video_content: Uuid,
    pub quiz_content: Uuid,
    pub lesson_score: Option<i32>,
}

impl Lesson {
    pub fn new(
        id: Uuid,
        lesson_name: String,
        module_id: Uuid,
        text_content: Uuid,
        video_content: Uuid,
        quiz_content: Uuid,
        lesson_score: Option<i32>,
    ) -> Self {
        Self {
            id,
            lesson_name,
            module_id,
            text_content,
            video_content,
            quiz_content,
            lesson_score,
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ModuleCreationRequest {
    pub module_name: String,
    pub lesson_count: i32,
    pub lesson_list: Vec<Option<Uuid>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LessonCreationRequest {
    pub lesson_name: String,
    pub module_id: Uuid,
    pub text_content: Uuid,
    pub video_content: Uuid,
    pub quiz_content: Uuid,
    pub lesson_score: Option<i32>,
}
