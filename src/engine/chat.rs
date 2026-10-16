//! Gemma4 chat-template rendering and chat-request preparation.
//!
//! Renders the model's own chat template (the GGUF's `tokenizer.chat_template`,
//! or a file the user supplies) with minijinja configured to reproduce transformers'
//! `apply_chat_template` (Jinja2 sandbox, `trim_blocks`, `lstrip_blocks`,
//! Python value printing), then tokenizes the text exactly as transformers
//! does (special tokens recognized, no extra BOS). See `docs/chat.md`.
//!
//! Text only: image/audio/video content parts are rejected, not rendered as
//! placeholder tokens without their embeddings.

use super::tokenizer::{self, EncodeOptions, Tokenizer};
use crate::engine::model::Vocabulary;
use minijinja::value::{Value as Jinja, ValueKind};
use minijinja::{Environment, ErrorKind, State};
use serde_json::{Map, Value};
use std::fmt::{self, Write as _};

const MAX_DEPTH: usize = 32;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// The request is malformed or uses something this text-only path does
    /// not support. Nothing is silently dropped or approximated.
    Request(String),
    /// The template rejected the request or needed unsupported Python behavior.
    Template(String),
    Tokenizer(tokenizer::Error),
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Request(message) => write!(f, "invalid chat request: {message}"),
            Self::Template(message) => write!(f, "chat template error: {message}"),
            Self::Tokenizer(error) => write!(f, "{error}"),
        }
    }
}
impl std::error::Error for Error {}
impl From<tokenizer::Error> for Error {
    fn from(error: tokenizer::Error) -> Self {
        Self::Tokenizer(error)
    }
}

fn request(message: impl Into<String>) -> Error {
    Error::Request(message.into())
}

/// What to do with special-token text (`<|turn>`, `<bos>`, `<|"|>`, ...)
/// inside request strings. The reference tokenizes the rendered prompt with
/// special tokens recognized, so such text becomes real control tokens.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpecialText {
    /// Refuse the request (default for OpenAI requests: prevents prompt
    /// structure injection through message content, names or tool schemas).
    Reject,
    /// Reference behavior: the text is rendered and becomes control tokens.
    Allow,
}

/// Template inputs in OpenAI chat-completions shape.
#[derive(Debug, Clone, PartialEq)]
pub struct ChatRequest {
    pub messages: Vec<Value>,
    /// Function tools; empty means none (the template sees `None`).
    pub tools: Vec<Value>,
    pub enable_thinking: bool,
    /// Canonical template only: replay reasoning of earlier tool-calling turns.
    pub preserve_thinking: bool,
    pub add_generation_prompt: bool,
    pub special_text: SpecialText,
}

impl ChatRequest {
    /// Reads the template-relevant subset of a `/v1/chat/completions` body:
    /// `messages`, `tools`, `chat_template_kwargs.{enable_thinking,
    /// preserve_thinking}` and `add_generation_prompt` (default true).
    /// Sampling and server fields are ignored here. `special_text` defaults to
    /// [`SpecialText::Reject`].
    pub fn from_openai_json(body: &Value) -> Result<Self, Error> {
        let body = body
            .as_object()
            .ok_or_else(|| request("body must be an object"))?;
        let messages = match body.get("messages") {
            Some(Value::Array(m)) => m.clone(),
            _ => return Err(request("messages must be an array")),
        };
        let tools = match body.get("tools") {
            None | Some(Value::Null) => Vec::new(),
            Some(Value::Array(t)) => t.clone(),
            Some(_) => return Err(request("tools must be an array")),
        };
        let flag = |map: &Map<String, Value>, key: &str, default: bool| match map.get(key) {
            None | Some(Value::Null) => Ok(default),
            Some(Value::Bool(b)) => Ok(*b),
            Some(_) => Err(request(format!("{key} must be a boolean"))),
        };
        let (mut enable_thinking, mut preserve_thinking) = (false, false);
        match body.get("chat_template_kwargs") {
            None | Some(Value::Null) => {}
            Some(Value::Object(kwargs)) => {
                if let Some(key) = kwargs
                    .keys()
                    .find(|k| !matches!(k.as_str(), "enable_thinking" | "preserve_thinking"))
                {
                    return Err(request(format!(
                        "unsupported chat_template_kwargs key {key:?}"
                    )));
                }
                enable_thinking = flag(kwargs, "enable_thinking", false)?;
                preserve_thinking = flag(kwargs, "preserve_thinking", false)?;
            }
            Some(_) => return Err(request("chat_template_kwargs must be an object")),
        }
        if flag(body, "continue_final_message", false)? {
            return Err(request("continue_final_message is not supported"));
        }
        Ok(Self {
            messages,
            tools,
            enable_thinking,
            preserve_thinking,
            add_generation_prompt: flag(body, "add_generation_prompt", true)?,
            special_text: SpecialText::Reject,
        })
    }
}

/// Rendered prompt and its token IDs (BOS comes from the template's text).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreparedChat {
    pub text: String,
    pub ids: Vec<u32>,
}

pub struct ChatTemplate {
    env: Environment<'static>,
    /// `special_tokens_map` as transformers passes it to the template.
    special_tokens: Vec<(&'static str, String)>,
    special_texts: Vec<String>,
}

impl fmt::Debug for ChatTemplate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ChatTemplate").finish_non_exhaustive()
    }
}

impl ChatTemplate {
    /// Compiles `source` (a Jinja chat template) for rendering.
    pub fn new(source: &str, tokenizer: &Tokenizer) -> Result<Self, Error> {
        let text = |id: u32| {
            tokenizer
                .token(id)
                .map(str::to_owned)
                .ok_or(tokenizer::Error::TokenOutOfRange(id))
        };
        let mask = tokenizer
            .mask()
            .ok_or_else(|| request("tokenizer has no mask token"))?;
        Self::with_tokens(
            source,
            vec![
                ("bos_token", text(tokenizer.bos())?),
                ("eos_token", text(tokenizer.eos())?),
                ("unk_token", text(tokenizer.unknown())?),
                ("pad_token", text(tokenizer.padding())?),
                ("mask_token", text(mask)?),
            ],
            tokenizer
                .special_tokens()
                .map(|(text, _)| text.to_owned())
                .collect(),
        )
    }

    /// As [`Self::new`] for any tokenizer: `special_tokens` is the template's
    /// `special_tokens_map` (`("bos_token", "<s>")`, ...), `special_texts` the
    /// token texts [`SpecialText::Reject`] refuses in requests.
    pub fn with_tokens(
        source: &str,
        special_tokens: Vec<(&'static str, String)>,
        special_texts: Vec<String>,
    ) -> Result<Self, Error> {
        let mut env = Environment::new();
        env.set_trim_blocks(true);
        env.set_lstrip_blocks(true);
        env.set_formatter(|out, _state, value| {
            let text = py_str(value)?;
            out.write_str(&text)
                .map_err(|_| minijinja::Error::from(ErrorKind::WriteFailure))
        });
        env.set_unknown_method_callback(py_method);
        env.add_filter("upper", |value: &Jinja| {
            py_str(value).map(|s| s.to_uppercase())
        });
        env.add_filter("trim", |value: &Jinja| {
            py_str(value).map(|s| s.trim_matches(py_isspace).to_owned())
        });
        env.add_filter("dictsort", py_dictsort);
        // Python Jinja: `none is iterable` is false (minijinja says true).
        env.add_test("iterable", |value: &Jinja| {
            !value.is_none() && value.try_iter().is_ok()
        });
        env.add_function(
            "raise_exception",
            |message: String| -> Result<Jinja, minijinja::Error> {
                Err(minijinja::Error::new(ErrorKind::InvalidOperation, message))
            },
        );
        env.add_template_owned("chat", source.to_owned())
            .map_err(|e| Error::Template(e.to_string()))?;
        Ok(Self {
            env,
            special_tokens,
            special_texts,
        })
    }

    /// The model's own template, stored in the GGUF (`tokenizer.chat_template`).
    pub fn from_gguf(vocabulary: &Vocabulary<'_>, tokenizer: &Tokenizer) -> Result<Self, Error> {
        if vocabulary.chat_template.trim().is_empty() {
            return Err(Error::Template("the GGUF has no chat template".into()));
        }
        Self::new(vocabulary.chat_template, tokenizer)
    }

    /// Validates and normalizes the request, then renders the prompt text.
    pub fn render(&self, chat: &ChatRequest) -> Result<String, Error> {
        let messages = normalize_messages(&chat.messages)?;
        let tools = normalize_tools(&chat.tools)?;
        if chat.special_text == SpecialText::Reject {
            for value in messages.iter().chain(&tools) {
                self.reject_special_text(value)?;
            }
            // Text parts are trimmed and concatenated by the templates. A
            // control token can be assembled across otherwise harmless parts.
            for message in &messages {
                if let Some(parts) = message.get("content").and_then(Value::as_array) {
                    let joined: String = parts
                        .iter()
                        .filter_map(|p| p.get("text").and_then(Value::as_str))
                        .map(|s| s.trim_matches(py_isspace))
                        .collect();
                    self.reject_special_text(&Value::String(joined))?;
                }
            }
        }
        let mut context = Map::new();
        for (key, text) in &self.special_tokens {
            context.insert((*key).into(), Value::String(text.clone()));
        }
        context.insert("messages".into(), Value::Array(messages));
        context.insert(
            "tools".into(),
            if tools.is_empty() {
                Value::Null
            } else {
                Value::Array(tools)
            },
        );
        context.insert("documents".into(), Value::Null);
        context.insert(
            "add_generation_prompt".into(),
            Value::Bool(chat.add_generation_prompt),
        );
        context.insert("enable_thinking".into(), Value::Bool(chat.enable_thinking));
        context.insert(
            "preserve_thinking".into(),
            Value::Bool(chat.preserve_thinking),
        );
        let template = self
            .env
            .get_template("chat")
            .map_err(|e| Error::Template(e.to_string()))?;
        template
            .render(Jinja::from_serialize(&context))
            .map_err(|e| Error::Template(e.to_string()))
    }

    /// Supported text-chat preparation. Tool schemas, tool roles and tool
    /// call/response history are out of the initial engine scope and rejected.
    /// Use `prepare_reference` only for template-oracle research, not serving.
    pub fn prepare(
        &self,
        tokenizer: &Tokenizer,
        chat: &ChatRequest,
    ) -> Result<PreparedChat, Error> {
        if !chat.tools.is_empty()
            || chat.messages.iter().any(|m| {
                m.get("role").and_then(Value::as_str) == Some("tool")
                    || ["tool_calls", "tool_responses", "tool_call_id"]
                        .iter()
                        .any(|key| m.get(*key).is_some())
            })
        {
            return Err(request(
                "tool use and tool history are not supported in text-chat generation",
            ));
        }
        self.prepare_reference(tokenizer, chat)
    }

    /// Oracle-only `render` + tokenization, including the template's
    /// tool-rendering branches. This is NOT supported tool generation/execution.
    /// Special tokens are recognized; no BOS beyond the template's `<bos>`.
    pub fn prepare_reference(
        &self,
        tokenizer: &Tokenizer,
        chat: &ChatRequest,
    ) -> Result<PreparedChat, Error> {
        let text = self.render(chat)?;
        let ids = tokenizer.encode(&text, EncodeOptions::REFERENCE)?;
        Ok(PreparedChat { text, ids })
    }

    fn reject_special_text(&self, value: &Value) -> Result<(), Error> {
        let check = |s: &str| match self.special_texts.iter().find(|t| s.contains(t.as_str())) {
            Some(t) => Err(request(format!(
                "special token text {t:?} in request content"
            ))),
            None => Ok(()),
        };
        match value {
            Value::String(s) => check(s),
            Value::Array(items) => items.iter().try_for_each(|v| self.reject_special_text(v)),
            Value::Object(map) => map.iter().try_for_each(|(k, v)| {
                check(k)?;
                self.reject_special_text(v)
            }),
            _ => Ok(()),
        }
    }
}

// ---- request validation ---------------------------------------------------

fn check_depth(value: &Value, depth: usize) -> Result<(), Error> {
    if depth > MAX_DEPTH {
        return Err(request(format!("value nesting deeper than {MAX_DEPTH}")));
    }
    match value {
        Value::Array(items) => items.iter().try_for_each(|v| check_depth(v, depth + 1)),
        Value::Object(map) => map.values().try_for_each(|v| check_depth(v, depth + 1)),
        _ => Ok(()),
    }
}

fn object<'v>(value: &'v Value, what: &str) -> Result<&'v Map<String, Value>, Error> {
    value
        .as_object()
        .ok_or_else(|| request(format!("{what} must be an object")))
}

fn optional_string(map: &Map<String, Value>, key: &str, what: &str) -> Result<(), Error> {
    match map.get(key) {
        None | Some(Value::Null | Value::String(_)) => Ok(()),
        Some(_) => Err(request(format!("{what}.{key} must be a string"))),
    }
}

/// String, or a list of `{"type": "text", "text": ...}` parts.
fn check_content(content: Option<&Value>, what: &str, nullable: bool) -> Result<(), Error> {
    match content {
        Some(Value::String(_)) => Ok(()),
        None | Some(Value::Null) if nullable => Ok(()),
        Some(Value::Array(parts)) => parts.iter().try_for_each(|part| {
            let part = object(part, &format!("{what} content part"))?;
            match (part.get("type"), part.get("text")) {
                (Some(Value::String(t)), Some(Value::String(_))) if t == "text" => Ok(()),
                (Some(Value::String(t)), _) if t != "text" => Err(request(format!(
                    "{what}: {t:?} content parts are not supported (text-only model path)"
                ))),
                _ => Err(request(format!(
                    "{what}: text content part needs a string \"text\""
                ))),
            }
        }),
        _ => Err(request(format!(
            "{what}: content must be a string or a list of text parts"
        ))),
    }
}

fn normalize_messages(messages: &[Value]) -> Result<Vec<Value>, Error> {
    if messages.is_empty() {
        return Err(request("messages must not be empty"));
    }
    messages
        .iter()
        .enumerate()
        .map(|(index, message)| {
            let what = format!("messages[{index}]");
            check_depth(message, 1)?;
            let mut message = object(message, &what)?.clone();
            let role = message
                .get("role")
                .and_then(Value::as_str)
                .ok_or_else(|| request(format!("{what}.role must be a string")))?;
            match role {
                "system" | "developer" if index == 0 => {
                    check_content(message.get("content"), &what, false)?
                }
                "system" | "developer" => {
                    return Err(request(format!(
                        "{what}: {role} is only supported as the first message"
                    )));
                }
                "user" => check_content(message.get("content"), &what, false)?,
                "tool" => {
                    check_content(message.get("content"), &what, false)?;
                    optional_string(&message, "tool_call_id", &what)?;
                    optional_string(&message, "name", &what)?;
                }
                "assistant" => {
                    check_content(message.get("content"), &what, true)?;
                    optional_string(&message, "reasoning", &what)?;
                    optional_string(&message, "reasoning_content", &what)?;
                    if let Some(calls) = message.get_mut("tool_calls") {
                        normalize_tool_calls(calls, &what)?;
                    }
                    match message.get("tool_responses") {
                        None | Some(Value::Null) => {}
                        Some(Value::Array(responses)) => {
                            for response in responses {
                                optional_string(
                                    object(response, &format!("{what}.tool_responses[]"))?,
                                    "name",
                                    &what,
                                )?;
                            }
                        }
                        Some(_) => {
                            return Err(request(format!("{what}.tool_responses must be an array")));
                        }
                    }
                }
                _ => return Err(request(format!("{what}: unsupported role {role:?}"))),
            }
            Ok(Value::Object(message))
        })
        .collect()
}

/// OpenAI sends `arguments` as a JSON string; both templates need a mapping
/// (the canonical one raises on strings), so strings are parsed here.
fn normalize_tool_calls(calls: &mut Value, what: &str) -> Result<(), Error> {
    let calls = match calls {
        Value::Null => return Ok(()),
        Value::Array(calls) => calls,
        _ => return Err(request(format!("{what}.tool_calls must be an array"))),
    };
    for call in calls {
        let call = call
            .as_object_mut()
            .ok_or_else(|| request(format!("{what}.tool_calls[] must be an object")))?;
        optional_string(call, "id", what)?;
        let function = call
            .get_mut("function")
            .and_then(Value::as_object_mut)
            .ok_or_else(|| request(format!("{what}.tool_calls[].function must be an object")))?;
        if !function.get("name").is_some_and(Value::is_string) {
            return Err(request(format!(
                "{what}.tool_calls[].function.name must be a string"
            )));
        }
        let arguments = function
            .get_mut("arguments")
            .ok_or_else(|| request(format!("{what}.tool_calls[].function.arguments is missing")))?;
        if let Value::String(text) = arguments {
            *arguments = serde_json::from_str(text).map_err(|e| {
                request(format!(
                    "{what}.tool_calls[].function.arguments is not JSON: {e}"
                ))
            })?;
            check_depth(arguments, 4)?;
        }
        if !matches!(arguments, Value::Object(_) | Value::Null) {
            return Err(request(format!(
                "{what}.tool_calls[].function.arguments must be a JSON object"
            )));
        }
    }
    Ok(())
}

fn normalize_tools(tools: &[Value]) -> Result<Vec<Value>, Error> {
    for (index, tool) in tools.iter().enumerate() {
        let what = format!("tools[{index}]");
        check_depth(tool, 1)?;
        let tool = object(tool, &what)?;
        if tool.get("type").and_then(Value::as_str) != Some("function") {
            return Err(request(format!("{what}.type must be \"function\"")));
        }
        let function = object(
            tool.get("function").unwrap_or(&Value::Null),
            &format!("{what}.function"),
        )?;
        if !function.get("name").is_some_and(Value::is_string) {
            return Err(request(format!("{what}.function.name must be a string")));
        }
        optional_string(function, "description", &what)?;
        for key in ["parameters", "response"] {
            if !matches!(
                function.get(key),
                None | Some(Value::Null | Value::Object(_))
            ) {
                return Err(request(format!("{what}.function.{key} must be an object")));
            }
        }
    }
    Ok(tools.to_vec())
}

// ---- Jinja2/Python compatibility -------------------------------------------

fn unsupported(message: impl Into<String>) -> minijinja::Error {
    minijinja::Error::new(ErrorKind::InvalidOperation, message.into())
}

/// Python `str.isspace`, which `str.strip()` uses: Unicode White_Space plus
/// the information separators U+001C..U+001F.
fn py_isspace(c: char) -> bool {
    c.is_whitespace() || ('\u{1c}'..='\u{1f}').contains(&c)
}

/// Python `repr(float)`: shortest round-trip digits, fixed notation for
/// decimal exponents in [-4, 16), otherwise `d.ddde+XX`.
fn py_float(x: f64) -> String {
    if x.is_nan() {
        return "nan".into();
    }
    if x.is_infinite() {
        return if x > 0.0 { "inf" } else { "-inf" }.into();
    }
    if x == 0.0 {
        return if x.is_sign_negative() { "-0.0" } else { "0.0" }.into();
    }
    let sci = format!("{:e}", x.abs());
    let (mantissa, exponent) = sci.split_once('e').unwrap_or((&sci, "0"));
    let exponent: i32 = exponent.parse().unwrap_or(0);
    let digits: String = mantissa.chars().filter(char::is_ascii_digit).collect();
    let sign = if x < 0.0 { "-" } else { "" };
    if (-4..16).contains(&exponent) {
        let point = exponent + 1;
        if point <= 0 {
            format!("{sign}0.{}{digits}", "0".repeat((-point) as usize))
        } else if point as usize >= digits.len() {
            format!(
                "{sign}{digits}{}.0",
                "0".repeat(point as usize - digits.len())
            )
        } else {
            format!(
                "{sign}{}.{}",
                &digits[..point as usize],
                &digits[point as usize..]
            )
        }
    } else {
        let (head, tail) = digits.split_at(1);
        let dot = if tail.is_empty() {
            String::new()
        } else {
            format!(".{tail}")
        };
        let esign = if exponent < 0 { '-' } else { '+' };
        format!("{sign}{head}{dot}e{esign}{:02}", exponent.abs())
    }
}

/// Python `str()` of a template value (`None`, `True`, `1.0`, `1e+20`, ...).
fn py_str(value: &Jinja) -> Result<String, minijinja::Error> {
    match value.kind() {
        ValueKind::String => Ok(value.as_str().unwrap_or_default().to_owned()),
        ValueKind::Seq | ValueKind::Map => py_repr(value),
        _ => py_scalar(value),
    }
}

fn py_scalar(value: &Jinja) -> Result<String, minijinja::Error> {
    match value.kind() {
        ValueKind::Undefined => Ok(String::new()),
        ValueKind::None => Ok("None".into()),
        ValueKind::Bool => Ok(if value.is_true() { "True" } else { "False" }.into()),
        ValueKind::Number if value.is_integer() => Ok(value.to_string()),
        ValueKind::Number => Ok(py_float(f64::try_from(value.clone())?)),
        kind => Err(unsupported(format!("cannot render {kind} as Python text"))),
    }
}

/// Python `repr()` for JSON-shaped values. Non-ASCII strings are refused:
/// their repr depends on Unicode printability tables this module does not carry.
fn py_repr(value: &Jinja) -> Result<String, minijinja::Error> {
    match value.kind() {
        ValueKind::String => {
            let s = value.as_str().unwrap_or_default();
            if !s.is_ascii() {
                return Err(unsupported("Python repr of a non-ASCII string"));
            }
            let quote = if s.contains('\'') && !s.contains('"') {
                '"'
            } else {
                '\''
            };
            let mut out = String::from(quote);
            for c in s.chars() {
                match c {
                    '\\' => out.push_str("\\\\"),
                    '\n' => out.push_str("\\n"),
                    '\r' => out.push_str("\\r"),
                    '\t' => out.push_str("\\t"),
                    c if c == quote => {
                        out.push('\\');
                        out.push(c);
                    }
                    c if (c as u32) < 0x20 || c == '\x7f' => {
                        let _ = write!(out, "\\x{:02x}", c as u32);
                    }
                    c => out.push(c),
                }
            }
            out.push(quote);
            Ok(out)
        }
        ValueKind::Seq => {
            let items: Result<Vec<_>, _> = value.try_iter()?.map(|v| py_repr(&v)).collect();
            Ok(format!("[{}]", items?.join(", ")))
        }
        ValueKind::Map => {
            let mut items = Vec::new();
            for key in value.try_iter()? {
                items.push(format!(
                    "{}: {}",
                    py_repr(&key)?,
                    py_repr(&value.get_item(&key)?)?
                ));
            }
            Ok(format!("{{{}}}", items.join(", ")))
        }
        _ => py_scalar(value),
    }
}

/// Jinja2 `dictsort`: (key, value) pairs, stably sorted by `key.lower()`.
fn py_dictsort(value: &Jinja) -> Result<Jinja, minijinja::Error> {
    if value.kind() != ValueKind::Map {
        return Err(unsupported("dictsort needs a mapping"));
    }
    let mut pairs = Vec::new();
    for key in value.try_iter()? {
        let sort_key = key
            .as_str()
            .map(str::to_lowercase)
            .ok_or_else(|| unsupported("non-string mapping key"))?;
        pairs.push((
            sort_key,
            Jinja::from(vec![key.clone(), value.get_item(&key)?]),
        ));
    }
    pairs.sort_by(|a, b| a.0.cmp(&b.0));
    Ok(Jinja::from(
        pairs.into_iter().map(|(_, pair)| pair).collect::<Vec<_>>(),
    ))
}

/// The Python methods the templates call: `dict.get(key[, default])` and
/// `str.split(sep)`. Anything else is an error.
fn py_method(
    _: &State,
    value: &Jinja,
    method: &str,
    args: &[Jinja],
) -> Result<Jinja, minijinja::Error> {
    match (value.kind(), method, args) {
        (ValueKind::Map, "get", [key, rest @ ..]) if rest.len() <= 1 => {
            let found = value.get_item(key)?;
            Ok(if found.is_undefined() {
                rest.first().cloned().unwrap_or(Jinja::from(()))
            } else {
                found
            })
        }
        (ValueKind::String, "split", [sep]) => {
            let (text, sep) = (
                value.as_str().unwrap_or_default(),
                sep.as_str().unwrap_or_default(),
            );
            if sep.is_empty() {
                return Err(unsupported("str.split needs a non-empty string separator"));
            }
            Ok(Jinja::from(
                text.split(sep).map(Jinja::from).collect::<Vec<_>>(),
            ))
        }
        (ValueKind::String, "startswith" | "endswith", [affix])
            if affix.kind() == ValueKind::String =>
        {
            let (text, affix) = (
                value.as_str().unwrap_or_default(),
                affix.as_str().unwrap_or_default(),
            );
            Ok(Jinja::from(if method == "startswith" {
                text.starts_with(affix)
            } else {
                text.ends_with(affix)
            }))
        }
        // Python str.strip/lstrip/rstrip: whitespace, or any of `chars`.
        (ValueKind::String, "strip" | "lstrip" | "rstrip", chars) if chars.len() <= 1 => {
            let text = value.as_str().unwrap_or_default();
            let set: Option<Vec<char>> = match chars.first() {
                None => None,
                Some(c) if c.is_none() => None,
                Some(c) => Some(
                    c.as_str()
                        .ok_or_else(|| unsupported("str.strip chars must be a string"))?
                        .chars()
                        .collect(),
                ),
            };
            let strip = |c: char| {
                set.as_ref()
                    .map_or_else(|| py_isspace(c), |s| s.contains(&c))
            };
            Ok(Jinja::from(match method {
                "lstrip" => text.trim_start_matches(strip),
                "rstrip" => text.trim_end_matches(strip),
                _ => text.trim_matches(strip),
            }))
        }
        _ => Err(minijinja::Error::new(
            ErrorKind::UnknownMethod,
            format!("{}.{method}", value.kind()),
        )),
    }
}

#[cfg(test)]
mod tests;
