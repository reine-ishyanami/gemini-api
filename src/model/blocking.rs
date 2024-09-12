use anyhow::{bail, Result};
use reqwest::blocking::Client;
use serde_json;

use crate::{
    body::{
        error::GenerateContentResponseError,
        request::{GeminiRequestBody, GenerationConfig},
        response::GenerateContentResponse,
        Content, Part, Role,
    },
    param::LanguageModel,
};

#[derive(Clone, Default)]
pub struct Gemini {
    pub key: String,
    pub url: String,
    pub contents: Vec<Content>,
    pub options: GenerationConfig,
    pub system_instruction: Option<String>,
    client: Client,
}

impl Gemini {
    /// 创建新实例
    pub fn new(key: String, model: LanguageModel) -> Self {
        let client = Client::new();
        let contents = Vec::new();
        let url = format!("{}{}:generateContent", super::GEMINI_API_URL, model);
        Self {
            key,
            url,
            contents,
            client,
            ..Default::default()
        }
    }

    /// 重建实例
    pub fn rebuild(key: String, url: String, contents: Vec<Content>, options: GenerationConfig) -> Self {
        let client = Client::new();
        Self {
            key,
            url,
            contents,
            client,
            options,
            ..Default::default()
        }
    }

    /// 配置系统指令
    pub fn set_system_instruction(&mut self, instruction: String) {
        self.system_instruction = Some(instruction);
    }

    /// 参数配置
    pub fn set_options(&mut self, options: GenerationConfig) {
        self.options = options;
    }

    /// 构建请求体
    fn build_request_body(&self, contents: Vec<Content>) -> GeminiRequestBody {
        GeminiRequestBody {
            contents,
            generation_config: Some(self.options.clone()),
            system_instruction: self.system_instruction.as_ref().map(|s| Content {
                parts: vec![Part::Text(s.clone())],
                role: None,
            }),
            ..Default::default()
        }
    }

    /// 同步单次对话
    pub fn chat_once(&self, content: String) -> Result<String> {
        // 创建一个客户端实例
        let url = format!("{}?key={}", self.url, self.key);
        // 请求内容
        let contents = vec![Content {
            role: Some(Role::User),
            parts: vec![Part::Text(content)],
        }];
        let body = self.build_request_body(contents);
        let body_json = serde_json::to_string(&body)?;
        // 发送 GET 请求，并添加自定义头部
        let response = self
            .client
            .post(url)
            .header("Content-Type", "application/json")
            .body(body_json)
            .send()?;
        if response.status().is_success() {
            let response_text = response.text()?;
            // 解析响应内容
            let response: GenerateContentResponse = serde_json::from_str(&response_text)?;
            match response.candidates[0].content.parts[0].clone() {
                Part::Text(s) => Ok(s),
                _ => bail!("Unexpected response format"),
            }
        } else {
            let response_text = response.text()?;
            // 解析响应内容
            let response_error: GenerateContentResponseError = serde_json::from_str(&response_text)?;
            let error_message = response_error.error.message;
            bail!(error_message)
        }
    }

    /// 同步连续对话
    pub fn chat_conversation(&mut self, content: String) -> Result<String> {
        self.contents.push(Content {
            role: Some(Role::User),
            parts: vec![Part::Text(content)],
        });
        let cloned_contents = self.contents.clone();
        let url = format!("{}?key={}", self.url, self.key);
        let body = self.build_request_body(cloned_contents);
        let body_json = serde_json::to_string(&body)?;
        // 发送 GET 请求，并添加自定义头部
        let response = self
            .client
            .post(url)
            .header("Content-Type", "application/json")
            .body(body_json)
            .send()?;

        if response.status().is_success() {
            let response_text = response.text()?;
            // 解析响应内容
            let response: GenerateContentResponse = serde_json::from_str(&response_text)?;
            match response.candidates[0].content.parts[0].clone().clone() {
                Part::Text(s) => {
                    self.contents.push(Content {
                        role: Some(Role::Model),
                        parts: vec![Part::Text(s.clone())],
                    });
                    Ok(s)
                }
                _ => bail!("Unexpected response format"),
            }
        } else {
            // 如果响应失败，则移除最后发送的那次用户请求
            self.contents.pop();
            let response_text = response.text()?;
            // 解析错误响应内容
            let response_error: GenerateContentResponseError = serde_json::from_str(&response_text)?;
            let error_message = response_error.error.message;
            bail!(error_message)
        }
    }
}

#[cfg(test)]
mod test {
    use std::env;

    use super::*;

    #[test]
    fn test_chat_once() {
        let key = env::var("GEMINI_KEY");
        assert!(key.is_ok());
        let client = Gemini::new(key.unwrap(), LanguageModel::Gemini1_5Flash);
        let req1 = "My Name is Reine".to_owned();
        let resp1 = client.chat_once(req1.clone());
        assert!(resp1.is_ok());
        println!("{}: {}", req1, resp1.unwrap());
    }

    #[test]
    fn test_chat_conversation() {
        let key = env::var("GEMINI_KEY");
        assert!(key.is_ok());
        let mut client = Gemini::new(key.unwrap(), LanguageModel::Gemini1_5Flash);
        let req1 = "My Name is Reine".to_owned();
        let resp1 = client.chat_conversation(req1.clone());
        assert!(resp1.is_ok());
        println!("{}: {}", req1, resp1.unwrap());
        let req2 = "Who am I".to_owned();
        let resp2 = client.chat_conversation(req2.clone());
        assert!(resp2.is_ok());
        println!("{}: {}", req2, resp2.unwrap());
    }

    #[test]
    fn test_chat_with_system_instruction() -> Result<()> {
        let key = env::var("GEMINI_KEY");
        assert!(key.is_ok());
        let mut client = Gemini::new(key.unwrap(), LanguageModel::Gemini1_5Flash);
        client.set_system_instruction("你是一只猫娘，你每次说话都会在句尾加上喵~ ".to_owned());
        let req = "你好".to_owned();
        let resp = client.chat_once(req)?;
        println!("{}", resp);
        assert!(!resp.is_empty());
        assert!(resp.contains("喵~ "));
        Ok(())
    }

    #[test]
    fn test_chat_conversation_with_system_instruction() -> Result<()> {
        let key = env::var("GEMINI_KEY");
        assert!(key.is_ok());
        let mut client = Gemini::new(key.unwrap(), LanguageModel::Gemini1_5Flash);
        client.set_system_instruction("你是一只猫娘，你每次说话都会在句尾加上喵~ ".to_owned());
        let req1 = "My Name is Reine".to_owned();
        let resp1 = client.chat_conversation(req1.clone())?;
        assert!(!resp1.is_empty());
        assert!(resp1.contains("喵~ "));
        println!("{}: {}", req1, resp1);
        let req2 = "Who am I".to_owned();
        let resp2 = client.chat_conversation(req2.clone())?;
        assert!(!resp2.is_empty());
        assert!(resp2.contains("喵~ "));
        println!("{}: {}", req2, resp2);
        Ok(())
    }
}
