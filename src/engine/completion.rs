use crate::engine::{
    chat::PreparedChat,
    generation::{self, Event, FinishReason, Usage},
    output::{self, InitialContext, Markers, OutputParser, Status},
    tokenizer::Tokenizer,
};
use serde_json::{Value, json};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompletionError {
    Parse(output::ParseError),
    UnclosedReasoning,
    UnexpectedTermination,
    Cancelled,
    Failed,
    Unfinished,
    CommitOrder,
    InputAfterTermination,
    CommitmentMismatch,
    UsageOverflow,
}

impl fmt::Display for CompletionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{self:?}")
    }
}
impl std::error::Error for CompletionError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompletionFinish {
    Stop,
    Length,
    Error(CompletionError),
}

pub fn map_finish(reason: Option<FinishReason>, status: Status) -> CompletionFinish {
    match reason {
        Some(FinishReason::Cancelled) => CompletionFinish::Error(CompletionError::Cancelled),
        Some(FinishReason::Failed) => CompletionFinish::Error(CompletionError::Failed),
        None => CompletionFinish::Error(CompletionError::Unfinished),
        Some(FinishReason::Eos | FinishReason::Length) => match status {
            Status::Error(error) => CompletionFinish::Error(CompletionError::Parse(error)),
            Status::Incomplete(output::Incomplete::UnclosedReasoning) => {
                CompletionFinish::Error(CompletionError::UnclosedReasoning)
            }
            Status::Complete(_) if reason == Some(FinishReason::Eos) => CompletionFinish::Stop,
            Status::Incomplete(output::Incomplete::TruncatedContent)
                if reason == Some(FinishReason::Length) =>
            {
                CompletionFinish::Length
            }
            _ => CompletionFinish::Error(CompletionError::UnexpectedTermination),
        },
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Completion {
    pub content: String,
    pub reasoning: Option<String>,
    pub finish: CompletionFinish,
    pub parse_status: Status,
    pub usage: Usage,
    pub committed_ids: Vec<u32>,
    pub generation_finish: Option<FinishReason>,
    pub replaced_invalid_utf8: usize,
}

impl Completion {
    pub fn to_json(&self) -> Value {
        let total = self
            .usage
            .prompt_tokens
            .checked_add(self.usage.completion_tokens);
        let error = match &self.finish {
            CompletionFinish::Error(error) => Some(error.clone()),
            _ if total.is_none() => Some(CompletionError::UsageOverflow),
            _ => None,
        };
        if let Some(error) = error {
            return json!({"error": {
                "type": "completion_error",
                "message": error.to_string(),
                "code": format!("{error:?}"),
            }});
        }
        let mut message = json!({"role": "assistant", "content": self.content});
        if let Some(reasoning) = &self.reasoning {
            message["reasoning_content"] = json!(reasoning);
        }
        json!({
            "object": "chat.completion",
            "choices": [{
                "index": 0,
                "message": message,
                "finish_reason": match self.finish {
                    CompletionFinish::Stop => "stop",
                    CompletionFinish::Length => "length",
                    CompletionFinish::Error(_) => unreachable!(),
                },
            }],
            "usage": {
                "prompt_tokens": self.usage.prompt_tokens,
                "completion_tokens": self.usage.completion_tokens,
                "total_tokens": total.unwrap(),
            },
        })
    }
}

pub struct CompletionAssembler<'tokenizer> {
    parser: OutputParser<'tokenizer>,
    committed_ids: Vec<u32>,
    next_canvas: usize,
    terminal_commit: bool,
    error: Option<CompletionError>,
}

impl<'tokenizer> CompletionAssembler<'tokenizer> {
    pub fn new(
        tokenizer: &'tokenizer Tokenizer,
        prepared: &PreparedChat,
    ) -> Result<Self, output::Error> {
        let markers = Markers::new(tokenizer)?;
        let context = InitialContext::from_prompt_ids(tokenizer, &markers, &prepared.ids)?;
        Self::with_context(tokenizer, context)
    }

    fn with_context(
        tokenizer: &'tokenizer Tokenizer,
        context: InitialContext,
    ) -> Result<Self, output::Error> {
        Ok(Self {
            parser: OutputParser::new(tokenizer, context)?,
            committed_ids: Vec::new(),
            next_canvas: 0,
            terminal_commit: false,
            error: None,
        })
    }

    pub fn observe(&mut self, event: &Event) {
        let Event::Commit {
            canvas_index,
            tokens,
            finish_reason,
        } = event
        else {
            return;
        };
        self.committed_ids.extend_from_slice(tokens);
        if self.error.is_some() {
            return;
        }
        if *canvas_index != self.next_canvas || self.terminal_commit {
            self.error = Some(CompletionError::CommitOrder);
            return;
        }
        self.next_canvas += 1;
        self.terminal_commit = finish_reason.is_some();
        if let Err(error) = self.parser.push(tokens) {
            self.error = Some(match error.error {
                output::Error::Failed(error) => CompletionError::Parse(error),
                _ => CompletionError::InputAfterTermination,
            });
        }
    }

    pub fn finish(mut self, result: &generation::Output, include_reasoning: bool) -> Completion {
        let parsed = self.parser.finish();
        let mismatch = self.committed_ids != result.tokens
            || result.usage.completion_tokens != result.tokens.len();
        let finish = if mismatch {
            CompletionFinish::Error(CompletionError::CommitmentMismatch)
        } else if let Some(error) = self.error {
            CompletionFinish::Error(error)
        } else {
            map_finish(result.finish_reason, parsed.status)
        };
        Completion {
            content: parsed.content,
            reasoning: if include_reasoning {
                parsed.reasoning
            } else {
                None
            },
            finish,
            parse_status: parsed.status,
            usage: result.usage.clone(),
            committed_ids: result.tokens.clone(),
            generation_finish: result.finish_reason,
            replaced_invalid_utf8: parsed.replaced_invalid_utf8,
        }
    }
}

#[cfg(test)]
mod tests;
