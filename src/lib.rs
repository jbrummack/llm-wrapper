use tokio::runtime::Handle;

use crate::{
    error::LlmError,
    message::Message,
    request::Request,
    response::Response,
    structure::{JsonSchema, ResponseFormat, Structure},
};

pub mod error;
pub mod image;
pub mod message;
pub mod request;
pub mod response;
pub mod structure;
pub mod urls;

pub struct LlmWrapper {
    model: &'static str,
    client: reqwest::Client,
    url: &'static str,
    runtime: Option<Handle>,
    key: String,
}

impl LlmWrapper {
    pub fn new(url: &'static str, model: &'static str, key: String) -> Result<Self, LlmError> {
        Ok(Self {
            model,
            client: reqwest::Client::new(),
            url,
            runtime: None,
            key,
        })
    }
    ///creates an LlmWrapper with runtime; for use when there is no #[tokio::main]
    pub fn new_with_rt(
        url: &'static str,
        model: &'static str,
        key: String,
        rt: Handle,
    ) -> Result<Self, LlmError> {
        Ok(Self {
            model,
            client: reqwest::Client::new(),
            url,
            runtime: Some(rt),
            key,
        })
    }
    async fn generate_response(&self, r: Request) -> Result<Response, LlmError> {
        let client = self.client.clone();
        let url = self.url;
        let key = self.key.clone();
        //let request = r;
        let future = {
            let url = url;
            let key = key;
            let client = client.clone();
            async move {
                let response = client
                    .post(url)
                    .json(&r)
                    .header("Content-Type", "application/json")
                    .bearer_auth(key)
                    .send()
                    .await?
                    .error_for_status()?;
                Result::<Response, LlmError>::Ok(response.json().await?)
            }
        };
        let response = if let Some(rt) = self.runtime.clone() {
            rt.spawn(future).await??
        } else {
            tokio::spawn(future).await??
        };
        Ok(response)
    }

    pub async fn request(&self, messages: Vec<Message>) -> Result<Response, LlmError> {
        todo!()
    }
    pub async fn request_structured<T: Structure>(
        &self,
        messages: Vec<Message>,
    ) -> Result<Option<T::SelfType>, LlmError> {
        let structure = T::get_schema()?;

        let json_schema = JsonSchema {
            name: "Request".into(),
            strict: true,
            schema: structure,
        };
        let response_format = Some(ResponseFormat::new(json_schema));

        let request = Request {
            model: self.model,
            response_format, //: None,
            messages,
        };
        let response = self.generate_response(request).await?;

        if let Some(json) = response.get_json() {
            let obj: T::SelfType = serde_json::from_str(&json)?;
            Ok(Some(obj))
            //println!("{obj:?}");
        } else {
            Ok(None)
        }
    }
}
