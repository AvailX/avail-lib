use aes_gcm::aes::cipher::typenum::Less;
use chrono::{DateTime, Utc};
use diesel::sql_types::Date;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug)]
pub struct TextContent {
    pub id: Uuid,
    pub module_id: Uuid,
    pub lesson_id: Uuid,
    pub title: String,
    pub author: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub content: Vec<Option<String>>,
    pub slug: Option<String>,
}

impl TextContent {
    pub fn new(
        id: Uuid,
        module_id: Uuid,
        lesson_id: Uuid,
        title: String,
        author: Option<String>,
        created_at: Option<DateTime<Utc>>,
        updated_at: Option<DateTime<Utc>>,
        content: Vec<Option<String>>,
        slug: Option<String>,
    ) -> Self {
        Self {
            id,
            module_id,
            lesson_id,
            title,
            author,
            created_at,
            updated_at,
            content,
            slug,
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TextContentRequest {
    pub module_id: Uuid,
    pub lesson_id: Uuid,
    pub title: String,
    pub author: Option<String>,
    pub content: Vec<Option<String>>,
    pub slug: Option<String>,
}
