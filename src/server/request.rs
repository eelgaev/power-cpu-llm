use crate::engine::{chat::ChatRequest, generation::GenerationConfig};
use crate::server::ApiError;
use serde_json::Value;

pub struct ChatInput {
    pub chat: ChatRequest,
    pub config: GenerationConfig,
    pub stream: bool,
    pub include_reasoning: bool,
}
impl ChatInput {
    /// `sampling`: the served model is autoregressive, so `temperature`,
    /// `top_p` and `top_k` are honoured (diffusion models reject them).
    pub fn parse(body: Value, model: &str, sampling: bool) -> Result<Self, ApiError> {
        let object = body
            .as_object()
            .ok_or_else(|| ApiError::invalid("body must be an object", "body"))?;
        for key in object.keys() {
            let sampling_key =
                sampling && ["temperature", "top_p", "top_k"].contains(&key.as_str());
            if !sampling_key
                && ![
                    "model",
                    "messages",
                    "max_completion_tokens",
                    "max_tokens",
                    "seed",
                    "stream",
                    "include_reasoning",
                    "n",
                    "chat_template_kwargs",
                ]
                .contains(&key.as_str())
            {
                return Err(ApiError::invalid(format!("unsupported field {key}"), key));
            }
        }
        let mut sampler = crate::engine::generation::Sampling::default();
        if let Some(value) = object.get("temperature") {
            sampler.temperature = value
                .as_f64()
                .filter(|t| (0.0..=2.0).contains(t))
                .ok_or_else(|| ApiError::invalid("temperature must be in [0, 2]", "temperature"))?
                as f32;
        }
        if let Some(value) = object.get("top_p") {
            sampler.top_p = value
                .as_f64()
                .filter(|p| *p > 0.0 && *p <= 1.0)
                .ok_or_else(|| ApiError::invalid("top_p must be in (0, 1]", "top_p"))?
                as f32;
        }
        if let Some(value) = object.get("top_k") {
            sampler.top_k = usize::try_from(value.as_u64().ok_or_else(|| {
                ApiError::invalid("top_k must be a non-negative integer", "top_k")
            })?)
            .map_err(|_| ApiError::invalid("top_k overflow", "top_k"))?;
        }
        if let Some(value) = object.get("model")
            && value.as_str() != Some(model)
        {
            return Err(ApiError::invalid("unknown model", "model"));
        }
        if object
            .get("n")
            .is_some_and(|value| value.as_u64() != Some(1))
        {
            return Err(ApiError::invalid("only n=1 is supported", "n"));
        }
        if object.contains_key("max_tokens") && object.contains_key("max_completion_tokens") {
            return Err(ApiError::invalid(
                "use only one token limit",
                "max_completion_tokens",
            ));
        }
        let limit = object
            .get("max_completion_tokens")
            .or_else(|| object.get("max_tokens"));
        let max_new_tokens = match limit {
            Some(value) => usize::try_from(value.as_u64().ok_or_else(|| {
                ApiError::invalid(
                    "token limit must be a positive integer",
                    "max_completion_tokens",
                )
            })?)
            .map_err(|_| ApiError::invalid("token limit overflow", "max_completion_tokens"))?,
            None => 256,
        };
        if max_new_tokens == 0 {
            return Err(ApiError::invalid(
                "token limit must be positive",
                "max_completion_tokens",
            ));
        }
        let seed = match object.get("seed") {
            Some(value) => value
                .as_u64()
                .ok_or_else(|| ApiError::invalid("seed must be an unsigned integer", "seed"))?,
            None => 7,
        };
        let boolean = |key: &str| match object.get(key) {
            Some(value) => value
                .as_bool()
                .ok_or_else(|| ApiError::invalid("must be a boolean", key)),
            None => Ok(false),
        };
        let messages = object
            .get("messages")
            .and_then(Value::as_array)
            .ok_or_else(|| ApiError::invalid("messages must be an array", "messages"))?;
        for message in messages {
            let message = message
                .as_object()
                .ok_or_else(|| ApiError::invalid("message must be an object", "messages"))?;
            for key in message.keys() {
                if !["role", "content"].contains(&key.as_str()) {
                    return Err(ApiError::invalid(
                        format!("unsupported message field {key}"),
                        "messages",
                    ));
                }
            }
            if !matches!(
                message.get("role").and_then(Value::as_str),
                Some("system" | "user" | "assistant")
            ) {
                return Err(ApiError::invalid("unsupported role", "messages"));
            }
            if let Some(parts) = message.get("content").and_then(Value::as_array) {
                for part in parts {
                    let part = part
                        .as_object()
                        .ok_or_else(|| ApiError::invalid("invalid text part", "messages"))?;
                    if part.get("type").and_then(Value::as_str) != Some("text")
                        || part
                            .keys()
                            .any(|key| !["type", "text"].contains(&key.as_str()))
                    {
                        return Err(ApiError::invalid(
                            "only text parts are supported",
                            "messages",
                        ));
                    }
                }
            }
        }
        let chat = ChatRequest::from_openai_json(&body)
            .map_err(|error| ApiError::invalid(error.to_string(), "messages"))?;
        Ok(Self {
            chat,
            config: GenerationConfig {
                max_new_tokens,
                seed,
                sampling: sampler,
                ..Default::default()
            },
            stream: boolean("stream")?,
            include_reasoning: boolean("include_reasoning")?,
        })
    }
}
