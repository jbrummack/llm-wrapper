use schemars::{SchemaGenerator, r#gen::SchemaSettings};
use serde::Serialize;
use serde_json::Value;

use crate::error::LlmError;
pub const TYPE_JSON_SCHEMA: &'static str = "json_schema";

#[derive(Debug, Serialize)]
pub struct ResponseFormat {
    r#type: &'static str,
    json_schema: JsonSchema,
}
impl ResponseFormat {
    pub fn new(json_schema: JsonSchema) -> Self {
        Self {
            r#type: TYPE_JSON_SCHEMA,
            json_schema,
        }
    }
}
#[derive(Debug, Serialize)]
pub struct JsonSchema {
    pub name: String,
    pub strict: bool,
    pub schema: Value,
}

pub trait Structure: schemars::JsonSchema + serde::de::DeserializeOwned {
    type SelfType: serde::de::DeserializeOwned;
    fn get_schema() -> Result<Value, LlmError> {
        let mut settings = SchemaSettings::openapi3();
        settings.inline_subschemas = true;
        let mut gene = SchemaGenerator::new(settings);
        let schema = gene.root_schema_for::<Self>();

        let response_schema = serde_json::to_value(&schema.schema)?;
        Ok(response_schema)
    }
    fn decode(candidate: &str) -> Result<Self::SelfType, LlmError> {
        let r = serde_json::from_str(candidate)?;
        Ok(r)
    }
}
/*pub impl JsonSchema {
    fn new<T: schemars::JsonSchema>(format: T)
}*/
