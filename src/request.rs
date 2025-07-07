use serde::Serialize;

use crate::{message::Message, structure::ResponseFormat};

#[derive(Debug, Serialize)]
pub struct Request {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<ResponseFormat>,
    pub messages: Vec<Message>,
    pub model: &'static str,
}

pub struct InlineData {
    mime_type: &'static str,
    data: String,
}
pub struct Part {
    inline_data: Option<InlineData>,
    text: Option<String>,
}

pub struct GeminiContents {
    parts: Vec<Part>,
}
/*#[serde(rename_all = "camel_case")]
pub struct GeminiGenerationConfig {
    response_mime_type: &'static str,
    response_schema:
}*/
