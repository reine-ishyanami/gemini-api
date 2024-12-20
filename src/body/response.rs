use serde::{Deserialize, Serialize};

use super::{request::HarmCategory, Content};

/// Response from the model supporting multiple candidate responses.
///
/// Safety ratings and content filtering are reported for both prompt in GenerateContentResponse.prompt_feedback and for
/// each candidate in finishReason and in safetyRatings.
/// The API:
///  - Returns either all requested candidates or none of them
///  - Returns no candidates at all only if there was something wrong with the prompt (check promptFeedback)
///  - Reports feedback on each candidate in finishReason and safetyRatings.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerateContentResponse {
    /// Candidate responses from the model.
    pub candidates: Vec<Candidate>,
    /// Returns the prompt's feedback related to the content filters.
    pub prompt_feedback: Option<PromptFeedback>,
    /// Output only. Metadata on the generation requests' token usage.
    pub usage_metadata: UsageMetadata,
}

/// A response candidate generated from the model.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Candidate {
    /// Output only. Generated content returned from the model.
    pub content: Content,
    /// Optional. Output only. The reason why the model stopped generating tokens.
    /// If empty, the model has not stopped generating tokens.
    pub finish_reason: Option<FinishReason>,
    /// List of ratings for the safety of a response candidate.
    /// There is at most one rating per category.
    pub safety_ratings: Option<Vec<SafetyRating>>,
    /// Output only. Citation information for model-generated candidate.
    /// This field may be populated with recitation information for any text included in the content.
    /// These are passages that are "recited" from copyrighted material in the foundational LLM's training data.
    pub citation_metadata: Option<CitationMetadata>,
    /// Output only. Token count for this candidate.
    pub token_count: Option<isize>,
    /// Output only. Attribution information for sources that contributed to a grounded answer.
    /// This field is populated for GenerateAnswer calls.
    #[deprecated(since = "1.0.0")]
    pub grounding_attributions: Option<Vec<GroundingAttribution>>,
    /// Output only. Index of the candidate in the list of response candidates.
    pub index: Option<isize>,
    /// Output only.
    pub avg_logprobs: Option<f64>,
    /// Output only. Log-likelihood scores for the response tokens and top tokens
    pub logprobs_result: Option<LogprobsResult>,
}

/// Logprobs Result
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogprobsResult {
    /// Length = total number of decoding steps.
    pub top_candidates: Vec<TopCandidates>,
    /// Length = total number of decoding steps. The chosen candidates may or may not be in topCandidates.
    pub chosen_candidates: Vec<Candidate1>,
}

/// Candidates with top log probabilities at each decoding step.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TopCandidates {
    /// Sorted by log probability in descending order.
    pub candidates: Vec<Candidate1>,
}

/// Candidate for the logprobs token and score.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename = "candidate", rename_all = "camelCase")]
pub struct Candidate1 {
    /// The candidate’s token string value.
    pub token: Option<String>,
    /// The candidate’s token id value.
    pub token_id: Option<isize>,
    /// The candidate's log probability.
    pub log_probability: Option<f64>,
}

/// Defines the reason why the model stopped generating tokens.
#[derive(Clone, Debug, Serialize, Deserialize)]
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
    /// Token generation stopped because the content potentially contains Sensitive Personally Identifiable Information
    /// (SPII).
    #[serde(rename = "SPII")]
    Spii,
    /// The function call generated by the model is invalid.
    #[serde(rename = "MALFORMED_FUNCTION_CALL")]
    MalformedFunctionCall,
}

/// Safety rating for a piece of content.
///
/// The safety rating contains the category of harm and the harm probability level in that category for a piece of
/// content. Content is classified for safety across a number of harm categories and the probability of the harm
/// classification is included here.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SafetyRating {
    /// Required. The category for this rating.
    pub category: HarmCategory,
    /// Required. The probability of harm for this content.
    pub probability: HarmProbability,
    /// Was this content blocked because of this rating?
    pub blocked: Option<bool>,
}

/// The probability that a piece of content is harmful.
///
/// The classification system gives the probability of the content being unsafe.
/// This does not indicate the severity of harm for a piece of content.
#[derive(Clone, Debug, Serialize, Deserialize)]
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
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UsageMetadata {
    /// Number of tokens in the prompt. When cachedContent is set, this is still the total effective prompt size
    /// meaning this includes the number of tokens in the cached content.
    pub prompt_token_count: isize,
    /// Number of tokens in the cached part of the prompt (the cached content)
    pub cached_content_token_count: Option<isize>,
    /// Total number of tokens across all the generated response candidates.
    pub candidates_token_count: isize,
    /// Total token count for the generation request (prompt + response candidates).
    pub total_token_count: isize,
}

/// A collection of source attributions for a piece of content.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CitationMetadata {
    /// Citations to sources for a specific response.
    pub citation_sources: Vec<CitationSource>,
}

/// A citation to a source for a portion of a specific response.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CitationSource {
    /// Optional. Start of segment of the response that is attributed to this source.
    /// Index indicates the start of the segment, measured in bytes.
    pub start_index: Option<isize>,
    /// Optional. End of the attributed segment, exclusive.
    pub end_index: Option<isize>,
    /// Optional. URI that is attributed as a source for a portion of the text.
    pub uri: Option<String>,
    /// Optional. License for the GitHub project that is attributed as a source for segment.
    /// License info is required for code citations.
    pub license: Option<String>,
}

/// Attribution for a source that contributed to an answer.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroundingAttribution {
    /// Output only. Identifier for the source contributing to this attribution.
    pub source_id: AttributionSourceId,
    /// Grounding source content that makes up this attribution.
    pub content: Content,
}

/// Identifier for the source contributing to this attribution.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttributionSourceId {
    /// Identifier for an inline passage.
    pub grounding_passage: GroundingPassageId,
    /// Identifier for a Chunk fetched via Semantic Retriever.
    pub semantic_retriever_chunk: SemanticRetrieverChunk,
}

/// Identifier for a part within a GroundingPassage.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroundingPassageId {
    /// Output only. ID of the passage matching the GenerateAnswerRequest's GroundingPassage.id.
    pub passage_id: String,
    /// Output only. Index of the part within the GenerateAnswerRequest's GroundingPassage.content.
    pub part_index: isize,
}

/// Identifier for a Chunk retrieved via Semantic Retriever specified in the GenerateAnswerRequest using
/// SemanticRetrieverConfig.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SemanticRetrieverChunk {
    /// Output only. Name of the source matching the request's SemanticRetrieverConfig.source. Example: corpora/123 or
    /// corpora/123/documents/abc
    pub source: String,
    /// Output only. Name of the Chunk containing the attributed text. Example: corpora/123/documents/abc/chunks/xyz
    pub chunk: String,
}

/// A set of the feedback metadata the prompt specified in GenerateContentRequest.content.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PromptFeedback {
    /// Optional. If set, the prompt was blocked and no candidates are returned. Rephrase the prompt.
    pub block_reason: Option<BlockReason>,
    /// Ratings for safety of the prompt. There is at most one rating per category.
    pub safety_ratings: SafetyRating,
}

/// Specifies the reason why the prompt was blocked.
#[derive(Clone, Debug, Serialize, Deserialize)]
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

/// Response from ListModel containing a paginated list of Models.
///
/// If successful, the response body contains data with the following structure
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModelsResponse {
    /// The returned Models.
    pub models: Vec<Model>,
    /// A token, which can be sent as pageToken to retrieve the next page.
    /// If this field is omitted, there are no more pages.
    pub next_page_token: Option<String>,
}

/// Information about a Generative Language Model.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Model {
    /// Required. The resource name of the Model. Refer to Model variants for all allowed values.
    pub name: String,
    /// Required. The name of the base model, pass this to the generation request.
    pub base_model_id: Option<String>,
    /// Required. The version number of the model.
    /// This represents the major version (1.0 or 1.5)
    pub version: String,
    /// The human-readable name of the model. E.g. "Gemini 1.5 Flash".
    /// The name can be up to 128 characters long and can consist of any UTF-8 characters.
    pub display_name: String,
    /// A short description of the model.
    pub description: String,
    /// Maximum number of input tokens allowed for this model.
    pub input_token_limit: isize,
    /// Maximum number of output tokens available for this model.
    pub output_token_limit: isize,
    /// The model's supported generation methods.
    /// The corresponding API method names are defined as Pascal case strings, such as generateMessage and
    /// generateContent.
    pub supported_generation_methods: Vec<String>,
    /// Controls the randomness of the output.
    /// Values can range over [0.0,maxTemperature], inclusive. A higher value will produce responses that are more
    /// varied, while a value closer to 0.0 will typically result in less surprising responses from the model. This
    /// value specifies default to be used by the backend while making the call to the model.
    pub temperature: Option<f64>,
    /// The maximum temperature this model can use.
    pub max_temperature: Option<f64>,
    /// For Nucleus sampling.
    /// Nucleus sampling considers the smallest set of tokens whose probability sum is at least topP. This value
    /// specifies default to be used by the backend while making the call to the model.
    pub top_p: Option<f64>,
    /// For Top-k sampling.
    /// Top-k sampling considers the set of topK most probable tokens. This value specifies default to be used by the
    /// backend while making the call to the model. If empty, indicates the model doesn't use top-k sampling, and topK
    /// isn't allowed as a generation parameter.
    pub top_k: Option<isize>,
}
