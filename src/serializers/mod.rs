use serde_derive::{Deserialize, Serialize};


#[derive(Deserialize, Serialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}


#[derive(Deserialize)]
pub struct Choices {
    pub message: Message,
    pub finish_reason: String,
    pub index: u32
}


#[derive(Deserialize)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32
}

#[derive(Deserialize)]
pub struct Response {
    pub id: String,
    pub object: String,
    pub created: u32,
    pub model: String,
    pub usage: Usage,
    pub choices: Vec<Choices>
}


#[derive(Serialize)]
pub struct Request {
    pub model: String,
    pub messages: Vec<Message>,
    pub temperature: f32
}

