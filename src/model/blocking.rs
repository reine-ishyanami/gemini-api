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

use super::GEMINI_API_URL;

#[derive(Clone, Default)]
pub struct Gemini {
    pub key: String,
    pub model: LanguageModel,
    pub contents: Vec<Content>,
    pub options: GenerationConfig,
    pub system_instruction: Option<String>,
    pub conversation: bool,
    url: String,
    client: Client,
}

impl Gemini {
    /// 创建新实例
    pub fn new(key: String, model: LanguageModel) -> Self {
        let client = Client::new();
        let contents = Vec::new();
        let url = format!("{}{}:generateContent", GEMINI_API_URL, model);
        Self {
            key,
            model,
            contents,
            url,
            client,
            ..Default::default()
        }
    }

    /// 重建实例
    pub fn rebuild(key: String, model: LanguageModel, contents: Vec<Content>, options: GenerationConfig) -> Self {
        let client = Client::new();
        let url = format!("{}{}:generateContent", GEMINI_API_URL, model);
        Self {
            key,
            model,
            contents,
            options,
            url,
            client,
            conversation: true,
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
    #[deprecated(since = "0.5.0", note = "Please use `send_message` instead.")]
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
    #[deprecated(
        since = "0.5.0",
        note = "Please use `start_chat` & `send_message` instead, which supports continuous conversation."
    )]
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

    /// 图片分析
    /// 可传入本地图片路径以及网络图片路径
    #[cfg(feature = "image_analysis")]
    #[deprecated(since = "0.5.0", note = "Please use `send_image_message` instead.")]
    pub fn image_analysis(&self, image_path: String, text: String) -> Result<String> {
        use crate::utils::image::blocking::get_image_type_and_base64_string;

        let (image_type, base64_string) = get_image_type_and_base64_string(image_path).unwrap();
        let url = format!("{}?key={}", self.url, self.key);

        // 请求内容
        let contents = vec![Content {
            role: Some(Role::User),
            parts: vec![
                Part::Text(text),
                Part::InlineData {
                    mime_type: image_type,
                    data: base64_string,
                },
            ],
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

    /// 图片分析
    /// 可传入本地图片路径以及网络图片路径
    #[cfg(feature = "image_analysis")]
    #[deprecated(
        since = "0.5.0",
        note = "Please use `start_chat` & `send_image_message` instead, which supports continuous conversation."
    )]
    pub fn image_analysis_conversation(&mut self, image_path: String, text: String) -> Result<String> {
        use base64::{engine::general_purpose, Engine as _};
        use image::EncodableLayout;
        use std::{fs::File, io::Read};

        use crate::utils::image::guess_image_format;

        let (image_type, base64_string) = if image_path.starts_with("https://") || image_path.starts_with("http://") {
            let response = self.client.get(image_path).send()?;
            if response.status().is_success() {
                let bytes = response.bytes()?; // 读取整个响应体为字节
                let base64_string = general_purpose::STANDARD.encode(&bytes);
                (guess_image_format(bytes.as_bytes()), base64_string)
            } else {
                bail!("Failed to download image, status: {}", response.status());
            }
        } else {
            let mut buffer = Vec::new();
            let mut file = File::open(image_path)?;
            file.read_to_end(&mut buffer)?;
            let base64_string = general_purpose::STANDARD.encode(&buffer);
            (guess_image_format(buffer.as_slice()), base64_string)
        };
        let url = format!("{}?key={}", self.url, self.key);

        // 请求内容
        // 先文本后图片
        self.contents.push(Content {
            role: Some(Role::User),
            parts: vec![
                Part::Text(text),
                Part::InlineData {
                    mime_type: image_type,
                    data: base64_string,
                },
            ],
        });
        let cloned_contents = self.contents.clone();
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
            self.contents.pop();
            let response_text = response.text()?;
            // 解析响应内容
            let response_error: GenerateContentResponseError = serde_json::from_str(&response_text)?;
            let error_message = response_error.error.message;
            bail!(error_message)
        }
    }

    /// 开启历史记录
    pub fn start_chat(&mut self, contents: Vec<Content>) {
        self.contents = contents;
        self.conversation = true;
    }

    /// 发送消息
    pub fn send_message(&mut self, message: Content) -> Result<(String, GenerateContentResponse)> {
        if !self.conversation {
            // 创建一个客户端实例
            let url = format!("{}?key={}", self.url, self.key);
            let contents = vec![message];
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
                match response.candidates[0].content.parts[0].clone().clone() {
                    Part::Text(s) => {
                        self.contents.push(Content {
                            role: Some(Role::Model),
                            parts: vec![Part::Text(s.clone())],
                        });
                        Ok((s, response))
                    }
                    _ => bail!("Unexpected response format"),
                }
            } else {
                let response_text = response.text()?;
                // 解析错误响应内容
                let response_error: GenerateContentResponseError = serde_json::from_str(&response_text)?;
                let error_message = response_error.error.message;
                bail!(error_message)
            }
        } else {
            self.contents.push(message);
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
                        Ok((s, response))
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

    /// 发送简单文本消息
    pub fn send_simple_message(&mut self, message: String) -> Result<(String, GenerateContentResponse)> {
        if !self.conversation {
            // 创建一个客户端实例
            let url = format!("{}?key={}", self.url, self.key);
            let contents = vec![Content {
                parts: vec![Part::Text(message.clone())],
                role: Some(Role::User),
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
                match response.candidates[0].content.parts[0].clone().clone() {
                    Part::Text(s) => {
                        self.contents.push(Content {
                            role: Some(Role::Model),
                            parts: vec![Part::Text(s.clone())],
                        });
                        Ok((s, response))
                    }
                    _ => bail!("Unexpected response format"),
                }
            } else {
                let response_text = response.text()?;
                // 解析错误响应内容
                let response_error: GenerateContentResponseError = serde_json::from_str(&response_text)?;
                let error_message = response_error.error.message;
                bail!(error_message)
            }
        } else {
            self.contents.push(Content {
                parts: vec![Part::Text(message.clone())],
                role: Some(Role::User),
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
                        Ok((s, response))
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

    /// 发送图片文本消息
    #[cfg(feature = "image_analysis")]
    pub fn send_image_message(
        &mut self,
        image_path: String,
        text: String,
    ) -> Result<(String, GenerateContentResponse)> {
        use base64::{engine::general_purpose, Engine as _};
        use image::EncodableLayout;
        use std::{fs::File, io::Read};

        use crate::utils::image::blocking::get_image_type_and_base64_string;
        use crate::utils::image::guess_image_format;
        if !self.conversation {
            let (image_type, base64_string) = get_image_type_and_base64_string(image_path).unwrap();
            let url = format!("{}?key={}", self.url, self.key);

            // 请求内容
            let contents = vec![Content {
                role: Some(Role::User),
                parts: vec![
                    Part::Text(text),
                    Part::InlineData {
                        mime_type: image_type,
                        data: base64_string,
                    },
                ],
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
                    Part::Text(s) => Ok((s, response)),
                    _ => bail!("Unexpected response format"),
                }
            } else {
                let response_text = response.text()?;
                // 解析响应内容
                let response_error: GenerateContentResponseError = serde_json::from_str(&response_text)?;
                let error_message = response_error.error.message;
                bail!(error_message)
            }
        } else {
            let (image_type, base64_string) = if image_path.starts_with("https://") || image_path.starts_with("http://")
            {
                let response = self.client.get(image_path).send()?;
                if response.status().is_success() {
                    let bytes = response.bytes()?; // 读取整个响应体为字节
                    let base64_string = general_purpose::STANDARD.encode(&bytes);
                    (guess_image_format(bytes.as_bytes()), base64_string)
                } else {
                    bail!("Failed to download image, status: {}", response.status());
                }
            } else {
                let mut buffer = Vec::new();
                let mut file = File::open(image_path)?;
                file.read_to_end(&mut buffer)?;
                let base64_string = general_purpose::STANDARD.encode(&buffer);
                (guess_image_format(buffer.as_slice()), base64_string)
            };
            let url = format!("{}?key={}", self.url, self.key);

            // 请求内容
            // 先文本后图片
            self.contents.push(Content {
                role: Some(Role::User),
                parts: vec![
                    Part::Text(text),
                    Part::InlineData {
                        mime_type: image_type,
                        data: base64_string,
                    },
                ],
            });
            let cloned_contents = self.contents.clone();
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
                        Ok((s, response))
                    }
                    _ => bail!("Unexpected response format"),
                }
            } else {
                self.contents.pop();
                let response_text = response.text()?;
                // 解析响应内容
                let response_error: GenerateContentResponseError = serde_json::from_str(&response_text)?;
                let error_message = response_error.error.message;
                bail!(error_message)
            }
        }
    }
}
