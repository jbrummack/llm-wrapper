use serde::Serialize;

use crate::{message::Message, structure::ResponseFormat};

#[derive(Debug, Serialize)]
pub struct Request {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<ResponseFormat>,
    pub messages: Vec<Message>,
    pub model: &'static str,
}
