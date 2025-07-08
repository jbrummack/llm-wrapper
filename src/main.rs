use llm_wrapper::error::LlmError;
use schemars::JsonSchema as SchemaTrait;
use serde::Deserialize;
#[derive(Debug, Deserialize, SchemaTrait)]
pub struct ImageRecognition {
    kcal_per_100g: f32,
    fat_per_100g: f32,
    carbs_per_100g: f32,
    protein_per_100g: f32,
}

#[derive(Debug, Deserialize, SchemaTrait)]
pub struct TestSchema {
    name: String,
    age: u32,
    nesting: TestNested,
}

#[derive(Debug, Deserialize, SchemaTrait)]
pub struct TestNested {
    data: Vec<f32>,
}

/*impl Structure for TestNested {
    type SelfType = Self;
}*/

#[tokio::main]
async fn main() -> Result<(), LlmError> {
    dotenvy::dotenv()?;
    let key = dotenvy::var("OPENROUTER")?;
    Ok(())
}

/*#[tokio::main]
async fn main() -> Result<(), LlmError> {
    dotenvy::dotenv()?;
    let key = dotenvy::var("OPENROUTER")?;
    //let key = std::env::var("GEMINI_API_KEY").unwrap();
    let structure = ImageRecognition::get_schema()?; //TestSchema::get_schema()?;
    //let json = serde_json::to_string_pretty(&structure)?;
    let json_schema = JsonSchema {
        name: "Request".into(),
        strict: true,
        schema: structure,
    };
    let response_format = Some(ResponseFormat::new(json_schema));
    let request = Request {
        //model: "google/gemini-2.5-flash-lite-preview-06-17",
        //model: "gemini-2.0-flash-lite",
        model: "google/gemini-2.0-flash-lite-001",
        //model: "openai/gpt-4.1-nano",
        //model: "google/gemma-3-27b-it:free",
        //model: "mistralai/mistral-small-3.2-24b-instruct",
        //model: "meta-llama/llama-4-scout:free",
        //model: "qwen/qwen2.5-vl-72b-instruct:free",
        //
        response_format, //: None,
        messages: vec![Message::user(vec![
            MessageContent::image_data(
                include_bytes!("test_img.png"),
                Some(llm_wrapper::image::ImageMime::WebP),
            )?,
            MessageContent::text("Extract the values from the picture into the schema."),
        ])],
    };
    let req_data = serde_json::to_string_pretty(&request)?;
    println!("{req_data}");
    //let url = "https://generativelanguage.googleapis.com/v1beta/openai/chat/completions";
    let url = "https://openrouter.ai/api/v1/chat/completions";
    let client = reqwest::Client::new();
    let response = client
        .post(url)
        .body(req_data)
        .header("Content-Type", "application/json")
        .bearer_auth(key)
        .send()
        .await?
        .error_for_status()?;
    println!("{response:?}");
    //.error_for_status()?;
    let text = response.text().await?;
    println!("{text}");
    let res: Response = serde_json::from_str(&text)?;
    if let Some(json) = res.get_json() {
        let obj: ImageRecognition = serde_json::from_str(&json)?;
        println!("{obj:?}");
    }
    //println!("{res:#?}");

    //let obj: TestSchema = serde_json::from
    //println!("{json}");
    //println!("{key}");
    Ok(())
}*/
