pub mod body;
pub mod model;
pub mod param;
pub mod utils;

use anyhow::{bail, Result};
use body::response::{Model, ModelsResponse};
use reqwest::Client;

/// Get a list of available models from Gemini API
pub async fn get_models(key: String) -> Result<Vec<Model>> {
    let url = "https://generativelanguage.googleapis.com/v1beta/models";
    let url = format!("{}?key={}", url, key);
    let client = Client::new();
    let response = client.get(url).send().await?;
    if response.status().is_success() {
        let response_text = response.text().await?;
        let response: ModelsResponse = serde_json::from_str(&response_text)?;
        Ok(response.models)
    } else {
        bail!("Failed to get models")
    }
}

#[cfg(test)]
mod tests {

    use body::{
        request::{GeminiRequestBody, GenerationConfig},
        Content, Part, Role,
    };
    use serde::{Deserialize, Serialize};

    use super::*;

    #[test]
    fn convert_to_json() -> Result<()> {
        let body = GeminiRequestBody {
            contents: vec![Content {
                role: Some(Role::User),
                parts: vec![Part::Text("Hello, world!".into())],
            }],
            generation_config: Some(GenerationConfig::default()),
            ..Default::default()
        };
        let body_json = serde_json::to_string(&body)?;
        assert_eq!(
            body_json,
            r#"{"contents":[{"parts":[{"text":"Hello, world!"}],"role":"user"}],"generationConfig":{"responseMimeType":"text/plain","maxOutputTokens":8192,"temperature":1.0,"topP":0.95,"topK":64}}"#
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_get_models() {
        use std::env;

        let key = env::var("GEMINI_KEY");
        assert!(key.is_ok());
        let models = get_models(key.unwrap()).await.unwrap();
        println!("{:#?}", models);
        assert!(!models.is_empty());
    }

    #[test]
    fn test_enum_serialize() {
        #[derive(Serialize, Deserialize)]
        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
        }
        {
            let msg = Message::Quit;
            let serialized = serde_json::to_string(&msg).unwrap();
            assert_eq!(serialized, r#""Quit""#);
        }
        {
            let msg = Message::Move { x: 10, y: 20 };
            let serialized = serde_json::to_string(&msg).unwrap();
            assert_eq!(serialized, r#"{"Move":{"x":10,"y":20}}"#);
        }
        {
            let msg = Message::Write("HelloWorld".into());
            let serialized = serde_json::to_string(&msg).unwrap();
            assert_eq!(serialized, r#"{"Write":"HelloWorld"}"#);
        }
    }
}
