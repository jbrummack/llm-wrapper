use serde::Serialize;

use crate::{message::Message, structure::ResponseFormat};

#[derive(Debug, Serialize)]
pub struct Request {
    response_format: Option<ResponseFormat>,
    messages: Vec<Message>,
}
