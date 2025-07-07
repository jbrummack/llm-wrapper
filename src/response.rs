use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    id: String,
    provider: Option<String>,
    model: Option<String>,
    object: Option<String>,
    created: Option<i64>,
    choices: Vec<Choice>,
    usage: Option<Usage>,
}

impl Response {
    pub fn get_json(self) -> Option<String> {
        self.choices.get(0).map(|c| c.message.content.clone())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Choice {
    logprobs: Option<serde_json::Value>,
    finish_reason: Option<String>,
    native_finish_reason: Option<String>,
    index: Option<i64>,
    message: Message,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    role: String,
    content: String,
    refusal: Option<serde_json::Value>,
    reasoning: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Usage {
    prompt_tokens: i64,
    completion_tokens: i64,
    total_tokens: i64,
}
