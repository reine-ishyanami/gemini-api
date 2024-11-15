use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GenerateContentResponseError {
    pub error: Error,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Error {
    pub code: i16,
    pub message: String,
    pub status: Option<String>,
    pub details: Option<Vec<Detail>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Detail {
    #[serde(rename = "@type")]
    pub type0: String,
    pub reason: Option<String>,
    pub domain: Option<String>,
    pub metadata: Option<Metadata>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Metadata {
    pub service: String,
}
