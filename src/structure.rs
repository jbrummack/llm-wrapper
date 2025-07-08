use schemars::{
    SchemaGenerator,
    r#gen::SchemaSettings,
    visit::{Visitor, visit_schema_object},
};
use serde::Serialize;
use serde_json::Value;

use crate::error::LlmError;
pub const TYPE_JSON_SCHEMA: &'static str = "json_schema";

#[derive(Debug, Serialize)]
pub struct ResponseFormat {
    r#type: &'static str,
    json_schema: JsonSchema,
}
//Needed for ChatGPT compatibility
#[derive(Debug, Clone)]
pub struct AdditionalProperties;

impl Visitor for AdditionalProperties {
    fn visit_schema_object(&mut self, schema: &mut schemars::schema::SchemaObject) {
        if schema.has_type(schemars::schema::InstanceType::Object) {
            schema
                .extensions
                .insert("additionalProperties".to_string(), serde_json::json!(false));
        }
        visit_schema_object(self, schema);
    }
}
impl ResponseFormat {
    pub fn new(json_schema: JsonSchema) -> Self {
        let mut json_schema = json_schema;
        if let Some(obj) = json_schema.schema.as_object_mut() {
            obj.insert(String::from("additionalProperties"), Value::Bool(false));
        }
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
        //;
        let mut gene = SchemaGenerator::new(settings.with_visitor(AdditionalProperties));
        let schema = gene.root_schema_for::<Self>();

        let response_schema = serde_json::to_value(&schema.schema)?;
        Ok(response_schema)
    }
    fn decode(candidate: &str) -> Result<Self::SelfType, LlmError> {
        let r = serde_json::from_str(candidate)?;
        Ok(r)
    }
}

impl<T: schemars::JsonSchema + serde::de::DeserializeOwned> Structure for T {
    type SelfType = T;
}
