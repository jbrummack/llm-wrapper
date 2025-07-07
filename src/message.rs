use serde::Serialize;
use serde_json::Value;

use crate::image::ImageUrl;

#[derive(Debug, Serialize)]
pub struct Message {
    role: &'static str,
    content: Vec<MessageContent>,
}

#[derive(Debug, Serialize)]
//#[serde(tag = "r#type", rename_all = "snake_case")]
//#[serde(rename_all = "snake_case")]
#[serde(untagged)]
pub enum MessageContent {
    Image {
        r#type: &'static str,
        image_url: ImageUrl,
    },
    Text {
        r#type: &'static str,
        text: String,
    },
}

impl Message {
    pub fn user(content: Vec<MessageContent>) -> Self {
        Self {
            role: "user",
            content,
        }
    }
    pub fn system(content: Vec<MessageContent>) -> Self {
        Self {
            role: "system",
            content,
        }
    }
}

impl MessageContent {
    pub fn text<T: Into<String>>(text: T) -> Self {
        Self::Text {
            r#type: "text",
            text: text.into(),
        }
    }
}

pub const ROLE_USER: &'static str = "user";
pub const TYPE_IMAGE_URL: &'static str = "image_url";

/*pub struct WireMessage {
    pub role: &'static str,
    pub content: Vec<Box<dyn MessageContent>>,
}*/
/*pub trait MessageContent {
    fn get_value(&self) -> Result<Value, LlmError>;
}*/
