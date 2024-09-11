use aes_gcm::aes::cipher::typenum::Less;
use chrono::{DateTime, Utc};
use diesel::sql_types::Date;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::errors::AvailResult;

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

#[derive(Deserialize, Serialize, Debug)]
pub struct VideoContent {
    pub id: Uuid,
    pub module_id: Uuid,
    pub lesson_id: Uuid,
    pub title: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub yt_link: String,
}

impl VideoContent {
    pub fn new(
        id: Uuid,
        module_id: Uuid,
        lesson_id: Uuid,
        title: String,
        created_at: Option<DateTime<Utc>>,
        updated_at: Option<DateTime<Utc>>,
        yt_link: String,
    ) -> Self {
        Self {
            id,
            module_id,
            lesson_id,
            title,
            created_at,
            updated_at,
            yt_link,
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct VideoContentRequest {
    pub module_id: Uuid,
    pub lesson_id: Uuid,
    pub title: String,
    pub yt_link: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct QuizContent {
    pub id: Uuid,
    pub module_id: Uuid,
    pub lesson_id: Uuid,
    pub title: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub question: String,
    pub options: Vec<Option<String>>,
    pub answers: Vec<Option<String>>,
    pub score: Option<i32>,
}

impl QuizContent {
    pub fn new(
        id: Uuid,
        module_id: Uuid,
        lesson_id: Uuid,
        title: String,
        created_at: Option<DateTime<Utc>>,
        updated_at: Option<DateTime<Utc>>,
        question: String,
        options: Vec<Option<String>>,
        answers: Vec<Option<String>>,
        score: Option<i32>,
    ) -> Self {
        Self {
            id,
            module_id,
            lesson_id,
            title,
            created_at,
            updated_at,
            question,
            options,
            answers,
            score,
        }
    }

    pub fn evaluate(&self, answers: Vec<String>) -> AvailResult<(i32, Vec<bool>)> {
        let mut score = 0;
        let mut correct_answers = vec![];
        for (i, ans) in answers.iter().enumerate() {
            if let Some(answer) = &self.answers[i] {
                if answer == ans {
                    score += self.score.unwrap_or(0);
                    correct_answers.push(true);
                } else {
                    correct_answers.push(false);
                }
            }
        }
        Ok((score, correct_answers))
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct QuizContentRequest {
    pub module_id: Uuid,
    pub lesson_id: Uuid,
    pub title: String,
    pub question: String,
    pub options: Vec<Option<String>>,
    pub answers: Vec<Option<String>>,
    pub score: Option<i32>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AddQuizQuestionRequest {
    pub q_id: Uuid,
    pub question: String,
    pub options: Vec<Option<String>>,
    pub answers: Vec<Option<String>>,
    pub score: Option<i32>,
}
