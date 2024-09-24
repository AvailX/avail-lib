use aes_gcm::aes::cipher::typenum::Less;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug)]
pub struct Module {
    pub id: Uuid,
    pub module_name: String,
    pub lesson_count: i32,
    pub lessons: Vec<Option<Uuid>>,
    pub m_desc: String,
    pub xp: i64,
}

impl Module {
    pub fn new(
        id: Uuid,
        module_name: String,
        lesson_count: i32,
        lessons: Vec<Option<Uuid>>,
        m_desc: String,
        xp: i64,
    ) -> Self {
        Self {
            id,
            module_name,
            lesson_count,
            lessons,
            m_desc,
            xp,
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Lesson {
    pub id: Uuid,
    pub lesson_name: String,
    pub module_id: Uuid,
    pub text_content: Uuid,
    pub lesson_score: Option<i32>,
    pub quiz_content: Vec<Option<Uuid>>,
    pub video_content: Vec<Option<Uuid>>,
}

impl Lesson {
    pub fn new(
        id: Uuid,
        lesson_name: String,
        module_id: Uuid,
        text_content: Uuid,
        lesson_score: Option<i32>,
        quiz_content: Vec<Option<Uuid>>,
        video_content: Vec<Option<Uuid>>,
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
    pub m_desc: String,
    pub xp: i64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LessonCreationRequest {
    pub lesson_name: String,
    pub module_id: Uuid,
    pub text_content: Uuid,
    pub lesson_score: Option<i32>,
    pub quiz_content: Vec<Option<Uuid>>,
    pub video_content: Vec<Option<Uuid>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ContentUpdateRequest {
    pub text_content: Uuid,
    pub video_content: Vec<Option<Uuid>>,
    pub quiz_content: Vec<Option<Uuid>>,
}
