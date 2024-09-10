use serde::{Deserialize, Serialize};

use super::{request::HarmCategory, Content};

/// Response from the model supporting multiple candidate responses.
/// Safety ratings and content filtering are reported for both prompt in GenerateContentResponse.prompt_feedback and for each candidate in finishReason and in safetyRatings.
/// The API:
///  - Returns either all requested candidates or none of them
///  - Returns no candidates at all only if there was something wrong with the prompt (check promptFeedback)
///  - Reports feedback on each candidate in finishReason and safetyRatings.
#[derive(Serialize, Deserialize)]
pub struct GenerateContentResponse {
    /// Candidate responses from the model.
    pub candidates: Vec<Candidate>,
    /// Returns the prompt's feedback related to the content filters.
    #[serde(rename = "promptFeedback")]
    pub prompt_feedback: Option<PromptFeedback>,
    /// Output only. Metadata on the generation requests' token usage.
    #[serde(rename = "usageMetadata")]
    pub usage_metadata: UsageMetadata,
}

#[derive(Serialize, Deserialize)]
pub struct Candidate {
    /// Output only. Generated content returned from the model.
    pub content: Content,
    /// Optional. Output only. The reason why the model stopped generating tokens.
    /// If empty, the model has not stopped generating tokens.
    #[serde(skip_serializing_if = "Option::is_none", rename = "finishReason")]
    pub finish_reason: Option<FinishReason>,
    /// List of ratings for the safety of a response candidate.
    /// There is at most one rating per category.
    #[serde(rename = "safetyRatings")]
    pub safety_ratings: Vec<SafetyRating>,
    /// Output only. Citation information for model-generated candidate.
    /// This field may be populated with recitation information for any text included in the content.
    /// These are passages that are "recited" from copyrighted material in the foundational LLM's training data.
    #[serde(rename = "citationMetadata")]
    pub citation_metadata: Option<CitationMetadata>,
    /// Output only. Token count for this candidate.
    #[serde(rename = "tokenCount")]
    pub token_count: Option<isize>,
    /// Output only. Attribution information for sources that contributed to a grounded answer.
    /// This field is populated for GenerateAnswer calls.
    #[serde(rename = "groundingAttributions")]
    pub grounding_attributions: Option<Vec<GroundingAttribution>>,
    /// Output only. Index of the candidate in the list of response candidates.
    pub index: isize,
}

/// Defines the reason why the model stopped generating tokens.
#[derive(Serialize, Deserialize)]
pub enum FinishReason {
    /// Default value. This value is unused.
    #[serde(rename = "FINISH_REASON_UNSPECIFIED")]
    FinishReasonUnspecified,
    /// Natural stop point of the model or provided stop sequence.
    #[serde(rename = "STOP")]
    Stop,
    /// The maximum number of tokens as specified in the request was reached.
    #[serde(rename = "MAX_TOKENS")]
    MaxTokens,
    /// The response candidate content was flagged for safety reasons.
    #[serde(rename = "SAFETY")]
    Safety,
    /// The response candidate content was flagged for recitation reasons.
    #[serde(rename = "RECITATION")]
    Recitation,
    /// The response candidate content was flagged for using an unsupported language.
    #[serde(rename = "LANGUAGE")]
    Language,
    /// Unknown reason.
    #[serde(rename = "OTHER")]
    Other,
    /// Token generation stopped because the content contains forbidden terms.
    #[serde(rename = "BLOCKLIST")]
    Blocklist,
    /// Token generation stopped for potentially containing prohibited content.
    #[serde(rename = "PROHIBITED_CONTENT")]
    ProhibitedContent,
    /// Token generation stopped because the content potentially contains Sensitive Personally Identifiable Information (SPII).
    #[serde(rename = "SPII")]
    Spii,
    /// The function call generated by the model is invalid.
    #[serde(rename = "MALFORMED_FUNCTION_CALL")]
    MalformedFunctionCall,
}

/// Safety rating for a piece of content.
/// The safety rating contains the category of harm and the harm probability level in that category for a piece of content.
/// Content is classified for safety across a number of harm categories and the probability of the harm classification is included here.
#[derive(Serialize, Deserialize)]
pub struct SafetyRating {
    /// Required. The category for this rating.
    pub category: HarmCategory,
    /// Required. The probability of harm for this content.
    pub probability: HarmProbability,
    /// Was this content blocked because of this rating?
    pub blocked: Option<bool>,
}

/// The probability that a piece of content is harmful.
/// The classification system gives the probability of the content being unsafe.
/// This does not indicate the severity of harm for a piece of content.
#[derive(Serialize, Deserialize)]
pub enum HarmProbability {
    /// Probability is unspecified.
    #[serde(rename = "HARM_PROBABILITY_UNSPECIFIED")]
    HarmProbabilityUnspecified,
    ///Content has a negligible chance of being unsafe.
    #[serde(rename = "NEGLIGIBLE")]
    Negligible,
    /// Content has a low chance of being unsafe.
    #[serde(rename = "LOW")]
    Low,
    /// Content has a medium chance of being unsafe.
    #[serde(rename = "MEDIUM")]
    Medium,
    /// Content has a high chance of being unsafe.
    #[serde(rename = "HIGH")]
    High,
}

/// Metadata on the generation request's token usage.
#[derive(Serialize, Deserialize)]
pub struct UsageMetadata {
    /// Number of tokens in the prompt. When cachedContent is set, this is still the total effective prompt size meaning this includes the number of tokens in the cached content.
    #[serde(rename = "promptTokenCount")]
    pub prompt_token_count: isize,
    /// Number of tokens in the cached part of the prompt (the cached content)
    #[serde(rename = "cachedContentTokenCount")]
    pub cached_content_token_count: Option<isize>,
    /// Total number of tokens across all the generated response candidates.
    #[serde(rename = "candidatesTokenCount")]
    pub candidates_token_count: isize,
    /// Total token count for the generation request (prompt + response candidates).
    #[serde(rename = "totalTokenCount")]
    pub total_token_count: isize,
}

/// A collection of source attributions for a piece of content.
#[derive(Serialize, Deserialize)]
pub struct CitationMetadata {
    /// Citations to sources for a specific response.
    #[serde(rename = "citationSources")]
    pub citation_sources: Vec<CitationSource>,
}

/// A citation to a source for a portion of a specific response.
#[derive(Serialize, Deserialize)]
pub struct CitationSource {
    /// Optional. Start of segment of the response that is attributed to this source.
    /// Index indicates the start of the segment, measured in bytes.
    #[serde(skip_serializing_if = "Option::is_none", rename = "startIndex")]
    pub start_index: Option<isize>,
    /// Optional. End of the attributed segment, exclusive.
    #[serde(skip_serializing_if = "Option::is_none", rename = "endIndex")]
    pub end_index: Option<isize>,
    /// Optional. URI that is attributed as a source for a portion of the text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    /// Optional. License for the GitHub project that is attributed as a source for segment.
    /// License info is required for code citations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<String>,
}

/// Attribution for a source that contributed to an answer.
#[derive(Serialize, Deserialize)]
pub struct GroundingAttribution {
    /// Output only. Identifier for the source contributing to this attribution.
    #[serde(rename = "sourceId")]
    pub source_id: AttributionSourceId,
    /// Grounding source content that makes up this attribution.
    pub content: Content,
}

/// Identifier for the source contributing to this attribution.
#[derive(Serialize, Deserialize)]
pub struct AttributionSourceId {
    /// Identifier for an inline passage.
    #[serde(rename = "groundingPassage")]
    pub grounding_passage: GroundingPassageId,
    /// Identifier for a Chunk fetched via Semantic Retriever.
    #[serde(rename = "semanticRetrieverChunk")]
    pub semantic_retriever_chunk: SemanticRetrieverChunk,
}

/// Identifier for a part within a GroundingPassage.
#[derive(Serialize, Deserialize)]
pub struct GroundingPassageId {
    /// Output only. ID of the passage matching the GenerateAnswerRequest's GroundingPassage.id.
    #[serde(rename = "passageId")]
    pub passage_id: String,
    /// Output only. Index of the part within the GenerateAnswerRequest's GroundingPassage.content.
    #[serde(rename = "partIndex")]
    pub part_index: isize,
}

/// Identifier for a Chunk retrieved via Semantic Retriever specified in the GenerateAnswerRequest using SemanticRetrieverConfig.
#[derive(Serialize, Deserialize)]
pub struct SemanticRetrieverChunk {
    /// Output only. Name of the source matching the request's SemanticRetrieverConfig.source. Example: corpora/123 or corpora/123/documents/abc
    pub source: String,
    /// Output only. Name of the Chunk containing the attributed text. Example: corpora/123/documents/abc/chunks/xyz
    pub chunk: String,
}

/// A set of the feedback metadata the prompt specified in GenerateContentRequest.content.
#[derive(Serialize, Deserialize)]
pub struct PromptFeedback {
    /// Optional. If set, the prompt was blocked and no candidates are returned. Rephrase the prompt.
    #[serde(skip_serializing_if = "Option::is_none", rename = "blockReason")]
    pub block_reason: Option<BlockReason>,
    /// Ratings for safety of the prompt. There is at most one rating per category.
    #[serde(rename = "safetyRatings")]
    pub safety_ratings: SafetyRating,
}

/// Specifies the reason why the prompt was blocked.
#[derive(Serialize, Deserialize)]
pub enum BlockReason {
    /// Default value. This value is unused.
    #[serde(rename = "BLOCK_REASON_UNSPECIFIED")]
    BlockReasonUnspecified,
    /// Prompt was blocked due to safety reasons. Inspect safetyRatings to understand which safety category blocked it.
    #[serde(rename = "SAFETY")]
    Safety,
    /// Prompt was blocked due to unknown reasons.
    #[serde(rename = "OTHER")]
    Other,
    /// Prompt was blocked due to the terms which are included from the terminology blocklist.
    #[serde(rename = "BLOCKLIST")]
    Blocklist,
    /// Prompt was blocked due to prohibited content.
    #[serde(rename = "PROHIBITED_CONTENT")]
    ProhibitedContent,
}
