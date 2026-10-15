//! Parser for COMMITTED Gemma4 output token IDs: separates the final answer
//! (content) from the `<|channel>thought\n … <channel|>` reasoning span.
//!
//! Structure is recognized by special-token IDs only (never by decoded text),
//! with marker IDs looked up and checked in the validated vocabulary. Text is
//! decoded per token with the checked tokenizer and streamed through a UTF-8
//! accumulator, so incomplete headers and characters are withheld across
//! commit boundaries. Text-only engine: tool and media structures are
//! rejected, never surfaced as content. See `docs/output.md`.

use crate::engine::tokenizer::{DecodeOptions, Tokenizer};
use std::fmt;

/// Literal channel header after `<|channel>` (model card and both templates).
const THOUGHT_HEADER: &[u8] = b"thought\n";
/// Literal role header after `<|turn>` in the generation prefix (both templates).
const MODEL_HEADER: &[u8] = b"model\n";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Terminator {
    /// `<eos>`
    Eos,
    /// `<turn|>`
    TurnEnd,
}

/// Why parsing stopped without a well-formed answer. After an error the
/// parser accepts no more input; text emitted before the error stays as it was.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseError {
    TokenOutOfRange(u32),
    /// `<eos>` / `<turn|>` while a reasoning span (or its header) is open.
    TerminatorInsideReasoning,
    /// `<|channel>` followed by anything other than `thought\n`.
    UnknownChannelHeader,
    /// A reasoning span opened after content had started.
    ChannelAfterContent,
    /// A control token with no meaning here (`<bos>`, `<pad>`, `<|turn>`, stray `<channel|>`, …).
    UnexpectedControl(u32),
    /// Tool declaration/call/response or `<|"|>` structure: unsupported, never content or executed.
    UnsupportedToolStructure(u32),
    UnsupportedMedia(u32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Incomplete {
    /// Input ended inside a reasoning span or its header (e.g. length limit).
    UnclosedReasoning,
    /// Input ended in content without `<eos>`/`<turn|>`.
    TruncatedContent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Complete(Terminator),
    Incomplete(Incomplete),
    Error(ParseError),
}

/// Incremental output of [`OutputParser::push`]. Concatenating all
/// `Reasoning` / `Content` text equals the final [`Output`] fields.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Update {
    ReasoningStarted,
    Reasoning(String),
    ReasoningEnded,
    Content(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Output {
    pub content: String,
    /// `None` if no reasoning span was opened; `Some("")` for an empty one.
    pub reasoning: Option<String>,
    pub status: Status,
    /// Invalid or unterminated UTF-8 sequences replaced by U+FFFD.
    pub replaced_invalid_utf8: usize,
    /// Text released only by `finish` (an unterminated UTF-8 tail as U+FFFD);
    /// already included in `content`/`reasoning`. Streaming callers append it
    /// after the `push` updates. Identical on repeated `finish` calls.
    pub trailing: Vec<Update>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// A marker is missing from the vocabulary or is not a special token.
    Vocabulary(String),
    /// The prompt does not end in a generation prefix this parser understands.
    UnsupportedPrefix,
    /// `push` after the output already terminated (the result is unchanged).
    InputAfterTermination,
    /// `push` after a parse error (repeats it; the result is unchanged).
    Failed(ParseError),
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for Error {}

/// A rejected [`OutputParser::push`]: `updates` holds the text emitted by the
/// same call BEFORE the error, so callers never lose already-parsed output.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PushError {
    pub error: Error,
    pub updates: Vec<Update>,
}
impl fmt::Display for PushError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.error)
    }
}
impl std::error::Error for PushError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Kind {
    Ordinary,
    ChannelOpen,
    ChannelClose,
    Terminator(Terminator),
    Tool,
    Media,
    OtherControl,
}

/// Marker IDs, looked up by spelling and checked to be special tokens.
#[derive(Debug, Clone)]
pub struct Markers {
    channel_open: u32,
    channel_close: u32,
    turn_open: u32,
    turn_close: u32,
    eos: u32,
    tool: Vec<u32>,
    media: Vec<u32>,
}

impl Markers {
    pub fn new(tokenizer: &Tokenizer) -> Result<Self, Error> {
        let special = |text: &str| match tokenizer.token_id(text) {
            Some(id) if tokenizer.is_special(id) => Ok(id),
            Some(_) => Err(Error::Vocabulary(format!(
                "{text:?} is not a special token"
            ))),
            None => Err(Error::Vocabulary(format!("{text:?} is missing"))),
        };
        let eos = special("<eos>")?;
        if eos != tokenizer.eos() {
            return Err(Error::Vocabulary(
                "<eos> is not the vocabulary's EOS".into(),
            ));
        }
        let all = |texts: &[&str]| {
            texts
                .iter()
                .map(|t| special(t))
                .collect::<Result<Vec<_>, _>>()
        };
        Ok(Self {
            channel_open: special("<|channel>")?,
            channel_close: special("<channel|>")?,
            turn_open: special("<|turn>")?,
            turn_close: special("<turn|>")?,
            eos,
            tool: all(&[
                "<|tool>",
                "<tool|>",
                "<|tool_call>",
                "<tool_call|>",
                "<|tool_response>",
                "<tool_response|>",
                "<|\"|>",
            ])?,
            media: all(&[
                "<|image>",
                "<image|>",
                "<|audio>",
                "<audio|>",
                "<|image|>",
                "<|audio|>",
                "<|video|>",
            ])?,
        })
    }

    fn kind(&self, tokenizer: &Tokenizer, id: u32) -> Kind {
        if !tokenizer.is_special(id) {
            Kind::Ordinary
        } else if id == self.channel_open {
            Kind::ChannelOpen
        } else if id == self.channel_close {
            Kind::ChannelClose
        } else if id == self.eos {
            Kind::Terminator(Terminator::Eos)
        } else if id == self.turn_close {
            Kind::Terminator(Terminator::TurnEnd)
        } else if self.tool.contains(&id) {
            Kind::Tool
        } else if self.media.contains(&id) {
            Kind::Media
        } else {
            Kind::OtherControl
        }
    }
}

/// Where the rendered generation prefix left the model.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InitialContext {
    /// Prompt ends with `<|turn>model\n` (both templates, thinking on or off).
    ModelTurn,
    /// Prompt ends with `<|channel>thought\n` (canonical template after a tool
    /// response with thinking enabled): output starts inside reasoning.
    InsideThought,
    /// Prompt ends with `<|turn>model\n<|channel>thought\n<channel|>` (Gemma 4
    /// QAT GGUF template, thinking disabled): an empty, already-closed
    /// reasoning span; output is the answer.
    AfterEmptyThought,
}

impl InitialContext {
    /// Classifies the END of the prepared prompt IDs (e.g. `PreparedChat::ids`):
    /// the LAST special token must be `<|turn>` or `<|channel>` (by ID) and the
    /// ordinary tokens after it must decode exactly to `model\n` / `thought\n`.
    pub fn from_prompt_ids(
        tokenizer: &Tokenizer,
        markers: &Markers,
        prompt: &[u32],
    ) -> Result<Self, Error> {
        let last = prompt
            .iter()
            .rposition(|&id| tokenizer.is_special(id))
            .ok_or(Error::UnsupportedPrefix)?;
        let tail = tokenizer
            .decode_bytes(&prompt[last + 1..], DecodeOptions::REFERENCE)
            .map_err(|_| Error::UnsupportedPrefix)?;
        match prompt[last] {
            id if id == markers.turn_open && tail == MODEL_HEADER => Ok(Self::ModelTurn),
            id if id == markers.channel_open && tail == THOUGHT_HEADER => Ok(Self::InsideThought),
            id if id == markers.channel_close && tail.is_empty() => {
                // Exactly `<|turn>model\n<|channel>thought\n<channel|>`.
                let before = &prompt[..last];
                let open = before
                    .iter()
                    .rposition(|&id| tokenizer.is_special(id))
                    .ok_or(Error::UnsupportedPrefix)?;
                if Self::from_prompt_ids(tokenizer, markers, before)? == Self::InsideThought
                    && Self::from_prompt_ids(tokenizer, markers, &before[..open])?
                        == Self::ModelTurn
                {
                    Ok(Self::AfterEmptyThought)
                } else {
                    Err(Error::UnsupportedPrefix)
                }
            }
            _ => Err(Error::UnsupportedPrefix),
        }
    }
}

/// Incremental UTF-8 with U+FFFD replacement counting.
#[derive(Debug, Default)]
struct Utf8 {
    pending: Vec<u8>,
    replaced: usize,
}
impl Utf8 {
    fn push(&mut self, bytes: &[u8]) -> String {
        self.pending.extend_from_slice(bytes);
        let mut out = String::new();
        let mut rest: &[u8] = &self.pending;
        loop {
            match std::str::from_utf8(rest) {
                Ok(s) => {
                    out.push_str(s);
                    rest = &[];
                    break;
                }
                Err(e) => {
                    let (valid, tail) = rest.split_at(e.valid_up_to());
                    out.push_str(std::str::from_utf8(valid).unwrap_or_default());
                    match e.error_len() {
                        Some(n) => {
                            out.push('\u{FFFD}');
                            self.replaced += 1;
                            rest = &tail[n..];
                        }
                        None => {
                            rest = tail;
                            break;
                        }
                    }
                }
            }
        }
        self.pending = rest.to_vec();
        out
    }
    /// An unterminated trailing sequence becomes one U+FFFD.
    fn flush(&mut self) -> String {
        if self.pending.is_empty() {
            return String::new();
        }
        self.pending.clear();
        self.replaced += 1;
        "\u{FFFD}".into()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum State {
    /// Model turn open, nothing parsed yet (also between reasoning spans).
    Start,
    /// After `<|channel>`: header bytes withheld (bounded by the header length).
    Header(Vec<u8>),
    /// First ordinary bytes of the turn that are still a prefix of
    /// `thought\n`: withheld. A complete `thought\n` is a reasoning span whose
    /// `<|channel>` opener the model dropped (observed at long context);
    /// anything else is flushed as content.
    ImplicitHeader(Vec<u8>),
    Reasoning,
    Content,
    Done(Terminator),
    Failed(ParseError),
}

/// Streaming parser over committed token IDs. Borrows the checked tokenizer.
#[derive(Debug)]
pub struct OutputParser<'t> {
    tokenizer: &'t Tokenizer,
    markers: Markers,
    state: State,
    content: String,
    reasoning: Option<String>,
    text: Utf8,
    /// Set by the first `finish`: the stream ended; later input is rejected.
    finished: Option<Output>,
    /// Any ID consumed yet (the implicit header applies only at the start).
    started: bool,
}

impl<'t> OutputParser<'t> {
    pub fn new(tokenizer: &'t Tokenizer, context: InitialContext) -> Result<Self, Error> {
        let (state, reasoning) = match context {
            InitialContext::ModelTurn => (State::Start, None),
            InitialContext::InsideThought => (State::Reasoning, Some(String::new())),
            InitialContext::AfterEmptyThought => (State::Start, Some(String::new())),
        };
        Ok(Self {
            tokenizer,
            markers: Markers::new(tokenizer)?,
            state,
            content: String::new(),
            reasoning,
            text: Utf8::default(),
            finished: None,
            // The dropped-opener recovery applies only to a fresh model turn.
            started: context != InitialContext::ModelTurn,
        })
    }

    pub fn markers(&self) -> &Markers {
        &self.markers
    }

    /// Consumes newly COMMITTED IDs (any chunking gives identical results).
    /// On failure, `PushError::updates` carries what this call emitted first.
    pub fn push(&mut self, ids: &[u32]) -> Result<Vec<Update>, PushError> {
        let mut updates = Vec::new();
        let reject = |error, updates| Err(PushError { error, updates });
        if let State::Failed(e) = self.state
            && !ids.is_empty()
        {
            return reject(Error::Failed(e), updates);
        }
        if self.finished.is_some() && !ids.is_empty() {
            return reject(Error::InputAfterTermination, updates);
        }
        for &id in ids {
            if let State::Done(_) = self.state {
                return reject(Error::InputAfterTermination, updates);
            }
            if let Err(e) = self.step(id, &mut updates) {
                self.fail(e, &mut updates);
                return reject(Error::Failed(e), updates);
            }
        }
        Ok(updates)
    }

    fn step(&mut self, id: u32, updates: &mut Vec<Update>) -> Result<(), ParseError> {
        if id as usize >= self.tokenizer.len() {
            return Err(ParseError::TokenOutOfRange(id));
        }
        let kind = self.markers.kind(self.tokenizer, id);
        let first = !std::mem::replace(&mut self.started, true);
        if let State::ImplicitHeader(pending) = &self.state {
            if kind == Kind::Ordinary {
                let mut pending = pending.clone();
                pending.extend_from_slice(&self.bytes(id));
                self.implicit_header(pending, updates);
                return Ok(());
            }
            // Not a header after all: the withheld bytes are content.
            let pending = pending.clone();
            self.state = State::Content;
            self.emit(&pending, updates);
        }
        if first && self.state == State::Start && kind == Kind::Ordinary {
            let bytes = self.bytes(id);
            self.implicit_header(bytes, updates);
            return Ok(());
        }
        match (&self.state, kind) {
            (_, Kind::Tool) => Err(ParseError::UnsupportedToolStructure(id)),
            (_, Kind::Media) => Err(ParseError::UnsupportedMedia(id)),
            (_, Kind::OtherControl) => Err(ParseError::UnexpectedControl(id)),
            (State::Start, Kind::ChannelOpen) => {
                self.state = State::Header(Vec::new());
                Ok(())
            }
            (State::Start | State::Content, Kind::Terminator(t)) => {
                self.flush_text(updates);
                self.state = State::Done(t);
                Ok(())
            }
            (State::Start | State::Content, Kind::Ordinary) => {
                self.state = State::Content;
                let bytes = self.bytes(id);
                self.emit(&bytes, updates);
                Ok(())
            }
            (State::Content, Kind::ChannelOpen) => Err(ParseError::ChannelAfterContent),
            (State::Start | State::Content, Kind::ChannelClose) => {
                Err(ParseError::UnexpectedControl(id))
            }
            (State::Header(_) | State::Reasoning, Kind::Terminator(_)) => {
                Err(ParseError::TerminatorInsideReasoning)
            }
            (State::Header(_), Kind::ChannelOpen | Kind::ChannelClose) => {
                Err(ParseError::UnknownChannelHeader)
            }
            (State::Header(pending), Kind::Ordinary) => {
                let mut pending = pending.clone();
                pending.extend_from_slice(&self.bytes(id));
                let n = pending.len().min(THOUGHT_HEADER.len());
                if pending[..n] != THOUGHT_HEADER[..n] {
                    return Err(ParseError::UnknownChannelHeader);
                }
                if pending.len() < THOUGHT_HEADER.len() {
                    self.state = State::Header(pending); // withheld, at most 7 bytes
                } else {
                    self.state = State::Reasoning;
                    self.reasoning.get_or_insert_with(String::new);
                    updates.push(Update::ReasoningStarted);
                    self.emit(&pending[THOUGHT_HEADER.len()..], updates);
                }
                Ok(())
            }
            (State::Reasoning, Kind::Ordinary) => {
                let bytes = self.bytes(id);
                self.emit(&bytes, updates);
                Ok(())
            }
            (State::Reasoning, Kind::ChannelOpen) => Err(ParseError::UnexpectedControl(id)),
            (State::Reasoning, Kind::ChannelClose) => {
                self.flush_text(updates);
                updates.push(Update::ReasoningEnded);
                self.state = State::Start; // content or another span may follow
                Ok(())
            }
            (State::Done(_) | State::Failed(_) | State::ImplicitHeader(_), _) => {
                unreachable!("checked in push / resolved above")
            }
        }
    }

    /// Resolve withheld leading bytes against `thought\n` (see `ImplicitHeader`).
    fn implicit_header(&mut self, pending: Vec<u8>, updates: &mut Vec<Update>) {
        let n = pending.len().min(THOUGHT_HEADER.len());
        if pending[..n] != THOUGHT_HEADER[..n] {
            self.state = State::Content;
            self.emit(&pending, updates);
        } else if pending.len() < THOUGHT_HEADER.len() {
            self.state = State::ImplicitHeader(pending);
        } else {
            self.state = State::Reasoning;
            self.reasoning.get_or_insert_with(String::new);
            updates.push(Update::ReasoningStarted);
            self.emit(&pending[THOUGHT_HEADER.len()..], updates);
        }
    }

    fn bytes(&self, id: u32) -> Vec<u8> {
        // In range and ordinary: decoding a single valid ID cannot fail.
        self.tokenizer
            .decode_bytes(&[id], DecodeOptions::REFERENCE)
            .unwrap_or_default()
    }

    fn emit(&mut self, bytes: &[u8], updates: &mut Vec<Update>) {
        let text = self.text.push(bytes);
        self.append(text, updates);
    }

    fn flush_text(&mut self, updates: &mut Vec<Update>) {
        let text = self.text.flush();
        self.append(text, updates);
    }

    fn append(&mut self, text: String, updates: &mut Vec<Update>) {
        if text.is_empty() {
            return;
        }
        match self.state {
            State::Reasoning => {
                self.reasoning
                    .get_or_insert_with(String::new)
                    .push_str(&text);
                updates.push(Update::Reasoning(text));
            }
            _ => {
                self.content.push_str(&text);
                updates.push(Update::Content(text));
            }
        }
    }

    fn fail(&mut self, error: ParseError, updates: &mut Vec<Update>) {
        if matches!(self.state, State::Reasoning | State::Content) {
            self.flush_text(updates);
        }
        self.state = State::Failed(error);
    }

    /// Final result; may be called repeatedly. Flushes withheld UTF-8 as
    /// U+FFFD (counted) but never closes a reasoning span or invents a terminator.
    pub fn finish(&mut self) -> Output {
        if let Some(done) = &self.finished {
            return done.clone();
        }
        let mut trailing = Vec::new();
        let status = match self.state.clone() {
            State::Done(t) => Status::Complete(t),
            State::Failed(e) => Status::Error(e),
            State::Header(_) => Status::Incomplete(Incomplete::UnclosedReasoning),
            State::ImplicitHeader(pending) => {
                // Ended while ambiguous: the withheld bytes are content.
                self.state = State::Content;
                self.emit(&pending, &mut trailing);
                self.flush_text(&mut trailing);
                Status::Incomplete(Incomplete::TruncatedContent)
            }
            State::Reasoning => {
                self.flush_text(&mut trailing);
                Status::Incomplete(Incomplete::UnclosedReasoning)
            }
            State::Start | State::Content => {
                self.flush_text(&mut trailing);
                Status::Incomplete(Incomplete::TruncatedContent)
            }
        };
        let output = Output {
            content: self.content.clone(),
            reasoning: self.reasoning.clone(),
            status,
            replaced_invalid_utf8: self.text.replaced,
            trailing,
        };
        self.finished = Some(output.clone());
        output
    }
}

#[cfg(test)]
mod tests;
