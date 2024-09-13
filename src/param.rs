use std::fmt;

use serde::{Deserialize, Serialize};
#[derive(Clone, Default, Serialize, Deserialize)]
pub enum LanguageModel {
    #[serde(rename = "gemini-1.0-pro")]
    Gemini1_0Pro,
    #[serde(rename = "gemini-1.5-pro")]
    Gemini1_5Pro,
    #[serde(rename = "gemini-1.5-flash")]
    #[default]
    Gemini1_5Flash,
    Custom(String),
}

impl fmt::Display for LanguageModel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LanguageModel::Gemini1_0Pro => write!(f, "models/gemini-1.0-pro"),
            LanguageModel::Gemini1_5Pro => write!(f, "models/gemini-1.5-pro"),
            LanguageModel::Gemini1_5Flash => write!(f, "models/gemini-1.5-flash"),
            LanguageModel::Custom(s) => write!(f, "{s}"),
        }
    }
}
