use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Token {
    pub symbol: String,
    pub image_url: String,
}

impl Token {
    pub fn new(symbol: String, image_url: String) -> Self {
        Self { symbol, image_url }
    }
}
