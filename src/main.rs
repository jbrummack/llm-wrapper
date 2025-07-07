use llm_wrapper::{
    error::LlmError,
    request::Request,
    structure::{JsonSchema, ResponseFormat, Structure},
};
use schemars::JsonSchema as SchemaTrait;
use serde::Deserialize;

#[derive(Debug, Deserialize, SchemaTrait)]
pub struct TestSchema {
    name: String,
    age: u32,
    nesting: TestNested,
}
impl Structure for TestSchema {
    type SelfType = Self;
}

#[derive(Debug, Deserialize, SchemaTrait)]
pub struct TestNested {
    data: Vec<f32>,
}
impl Structure for TestNested {
    type SelfType = Self;
}

fn main() -> Result<(), LlmError> {
    dotenvy::dotenv()?;
    let key = dotenvy::var("OPENROUTER")?;
    let structure = TestSchema::get_schema()?;
    //let json = serde_json::to_string_pretty(&structure)?;
    let json_schema = JsonSchema {
        name: "Request".into(),
        strict: true,
        schema: structure,
    };
    let response_format = ResponseFormat::new(json_schema);
    /*let request = Request {
        response_format: todo!(),
    };*/
    //println!("{json}");
    //println!("{key}");
    Ok(())
}
