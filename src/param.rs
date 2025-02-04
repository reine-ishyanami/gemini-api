use std::fmt;

use serde::{Deserialize, Serialize};
#[derive(Clone, Default, Serialize, Deserialize)]
pub enum LanguageModel {
    #[serde(rename = "gemini-1.0-pro")]
    Gemini1_0Pro,
    #[serde(rename = "gemini-1.5-pro")]
    Gemini1_5Pro,
    #[serde(rename = "gemini-1.5-flash")]
    Gemini1_5Flash,
    #[serde(rename = "gemini-2.0-flash-exp")]
    #[default]
    Gemini2_0FlashExp,
    Custom(String),
}

impl fmt::Display for LanguageModel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LanguageModel::Gemini1_0Pro => write!(f, "models/gemini-1.0-pro"),
            LanguageModel::Gemini1_5Pro => write!(f, "models/gemini-1.5-pro"),
            LanguageModel::Gemini1_5Flash => write!(f, "models/gemini-1.5-flash"),
            LanguageModel::Gemini2_0FlashExp => write!(f, "models/gemini-2.0-flash-exp"),
            LanguageModel::Custom(s) => write!(f, "{s}"),
        }
    }
}

/// 实现 String 与 LanguageModel 之间的转换
impl From<String> for LanguageModel {
    fn from(val: String) -> Self {
        match val.as_str() {
            "models/gemini-1.0-pro" => LanguageModel::Gemini1_0Pro,
            "models/gemini-1.5-pro" => LanguageModel::Gemini1_5Pro,
            "models/gemini-1.5-flash" => LanguageModel::Gemini1_5Flash,
            "models/gemini-2.0-flash-exp" => LanguageModel::Gemini2_0FlashExp,
            _ => LanguageModel::Custom(val),
        }
    }
}
