//! Gemma4 text tokenizer for the bound DiffusionGemma vocabulary.
//!
//! Reproduces the pinned Hugging Face `tokenizer.json` pipeline (see
//! `docs/tokenizer.md`): special added tokens are matched in the raw text
//! (leftmost-longest), every remaining segment has U+0020 replaced by U+2581
//! and is BPE-merged as ONE word (no pre-splitting), characters missing from
//! the vocabulary fall back to `<0xXX>` byte tokens. No BOS/EOS is added
//! unless requested. Text only: no chat-template rendering.

use crate::engine::model::Vocabulary;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::fmt;

const METASPACE: char = '\u{2581}';
// GGUF/SentencePiece token types.
const NORMAL: i64 = 1;
const CONTROL: i64 = 3;
const USER_DEFINED: i64 = 4;
const BYTE: i64 = 6;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// The vocabulary cannot be tokenized faithfully; nothing is approximated.
    Vocabulary(String),
    TokenOutOfRange(u32),
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Vocabulary(message) => write!(f, "unsupported tokenizer vocabulary: {message}"),
            Self::TokenOutOfRange(id) => write!(f, "token ID out of range: {id}"),
        }
    }
}
impl std::error::Error for Error {}

fn invalid(message: impl Into<String>) -> Error {
    Error::Vocabulary(message.into())
}

/// How special-token text (`<bos>`, `<|turn>`, ...) in the input is treated.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpecialTokens {
    /// Special-token text is ordinary text and is BPE-encoded character by
    /// character. Use for untrusted user content. (HF `encode_special_tokens = True`.)
    Plain,
    /// All 24 special tokens (GGUF types CONTROL and USER_DEFINED) are matched
    /// in the raw text, leftmost-longest. (HF default.)
    Recognize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EncodeOptions {
    /// Prepend exactly one BOS. Text containing `<bos>` is not deduplicated.
    pub add_bos: bool,
    pub special_tokens: SpecialTokens,
}
impl EncodeOptions {
    /// The pinned reference default (`tokenizers` and `transformers`): no BOS,
    /// special tokens recognized. The chat template writes `<bos>` itself.
    /// GGUF `add_bos_token = true` is llama.cpp policy and is NOT applied here.
    pub const REFERENCE: Self = Self {
        add_bos: false,
        special_tokens: SpecialTokens::Recognize,
    };
    /// Untrusted text spliced into an already-rendered prompt.
    pub const PLAIN_TEXT: Self = Self {
        add_bos: false,
        special_tokens: SpecialTokens::Plain,
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DecodeOptions {
    /// Drop special tokens (GGUF types CONTROL and USER_DEFINED) before decoding.
    pub skip_special_tokens: bool,
}
impl DecodeOptions {
    /// Reference default: special tokens are rendered as their text.
    pub const REFERENCE: Self = Self {
        skip_special_tokens: false,
    };
    pub const SKIP_SPECIAL: Self = Self {
        skip_special_tokens: true,
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Kind {
    Normal,
    Special,
    Byte(u8),
}

#[derive(Debug)]
pub struct Tokenizer {
    tokens: Vec<Box<str>>,
    kinds: Vec<Kind>,
    ids: HashMap<Box<str>, u32>,
    /// (left, right) -> (rank, merged).
    merges: HashMap<(u32, u32), (u32, u32)>,
    byte_ids: [u32; 256],
    /// Longest first, so the first match at a position is the longest.
    specials: Vec<(Box<str>, u32)>,
    special_first_bytes: [bool; 256],
    bos: u32,
    eos: u32,
    padding: u32,
    unknown: u32,
    mask: Option<u32>,
}

fn byte_token_value(token: &str) -> Option<u8> {
    let hex = token.strip_prefix("<0x")?.strip_suffix('>')?;
    (hex.len() == 2
        && hex
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'A'..=b'F').contains(&b)))
    .then(|| u8::from_str_radix(hex, 16).ok())
    .flatten()
}

impl Tokenizer {
    /// Validates the whole vocabulary up front. Copies what it needs, so the
    /// tokenizer does not borrow the model.
    pub fn from_vocabulary(vocabulary: &Vocabulary<'_>) -> Result<Self, Error> {
        let count = vocabulary.tokens.len();
        if count == 0 || count > u32::MAX as usize {
            return Err(invalid("vocabulary size"));
        }
        if vocabulary.token_types.len() != count || vocabulary.scores.len() != count {
            return Err(invalid("token/type/score array lengths differ"));
        }
        if vocabulary.add_space_prefix {
            return Err(invalid(
                "add_space_prefix is not part of the Gemma4 pipeline",
            ));
        }
        let mut ids = HashMap::with_capacity(count);
        let mut kinds = Vec::with_capacity(count);
        let mut byte_ids = [u32::MAX; 256];
        let mut specials = Vec::new();
        for (id, (token, &kind)) in vocabulary
            .tokens
            .iter()
            .zip(vocabulary.token_types)
            .enumerate()
        {
            let id = id as u32;
            if token.is_empty() {
                return Err(invalid(format!("empty token {id}")));
            }
            if ids.insert(Box::<str>::from(token.as_str()), id).is_some() {
                return Err(invalid(format!("duplicate token {token:?}")));
            }
            // The reference decoder recognizes byte tokens by spelling, so the
            // spelling and the type must agree.
            let byte = byte_token_value(token);
            kinds.push(match (kind, byte) {
                (BYTE, Some(b)) => {
                    byte_ids[b as usize] = id;
                    Kind::Byte(b)
                }
                (NORMAL, None) => Kind::Normal,
                (CONTROL | USER_DEFINED, None) => {
                    specials.push((Box::<str>::from(token.as_str()), id));
                    Kind::Special
                }
                _ => {
                    return Err(invalid(format!(
                        "token {id} {token:?} has unsupported type {kind}"
                    )));
                }
            });
        }
        if byte_ids.contains(&u32::MAX) {
            return Err(invalid("missing byte-fallback tokens"));
        }
        let mut merges = HashMap::with_capacity(vocabulary.merges.len());
        for (rank, merge) in vocabulary.merges.iter().enumerate() {
            let parts = merge
                .split_once(' ')
                .filter(|(a, b)| !a.is_empty() && !b.is_empty() && !b.contains(' '));
            let (left, right) =
                parts.ok_or_else(|| invalid(format!("malformed merge {merge:?}")))?;
            let lookup = |s: &str| {
                ids.get(s)
                    .copied()
                    .ok_or_else(|| invalid(format!("merge {merge:?} uses unknown {s:?}")))
            };
            let pair = (lookup(left)?, lookup(right)?);
            let merged = lookup(&format!("{left}{right}"))?;
            let rank = u32::try_from(rank).map_err(|_| invalid("too many merges"))?;
            if merges.insert(pair, (rank, merged)).is_some() {
                return Err(invalid(format!("duplicate merge {merge:?}")));
            }
        }
        let special_ids = [
            vocabulary.bos,
            vocabulary.eos,
            vocabulary.padding,
            vocabulary.unknown,
        ];
        for id in special_ids.into_iter().chain(vocabulary.mask) {
            if kinds.get(id as usize) != Some(&Kind::Special) {
                return Err(invalid(format!(
                    "configured special ID {id} is not a special token"
                )));
            }
        }
        specials.sort_by(|a, b| b.0.len().cmp(&a.0.len()).then(a.1.cmp(&b.1)));
        let mut special_first_bytes = [false; 256];
        for (text, _) in &specials {
            special_first_bytes[text.as_bytes()[0] as usize] = true;
        }
        Ok(Self {
            tokens: vocabulary
                .tokens
                .iter()
                .map(|t| Box::from(t.as_str()))
                .collect(),
            kinds,
            ids,
            merges,
            byte_ids,
            specials,
            special_first_bytes,
            bos: vocabulary.bos,
            eos: vocabulary.eos,
            padding: vocabulary.padding,
            unknown: vocabulary.unknown,
            mask: vocabulary.mask,
        })
    }

    pub fn len(&self) -> usize {
        self.tokens.len()
    }
    pub fn is_empty(&self) -> bool {
        self.tokens.is_empty()
    }
    pub fn token(&self, id: u32) -> Option<&str> {
        self.tokens.get(id as usize).map(|t| &**t)
    }
    pub fn token_id(&self, token: &str) -> Option<u32> {
        self.ids.get(token).copied()
    }
    /// The special tokens recognized by [`SpecialTokens::Recognize`].
    pub fn special_tokens(&self) -> impl Iterator<Item = (&str, u32)> {
        self.specials.iter().map(|(text, id)| (&**text, *id))
    }
    pub fn is_special(&self, id: u32) -> bool {
        self.kinds.get(id as usize) == Some(&Kind::Special)
    }
    pub fn bos(&self) -> u32 {
        self.bos
    }
    pub fn eos(&self) -> u32 {
        self.eos
    }
    pub fn padding(&self) -> u32 {
        self.padding
    }
    pub fn unknown(&self) -> u32 {
        self.unknown
    }
    pub fn mask(&self) -> Option<u32> {
        self.mask
    }

    /// Never appends EOS. Currently infallible for a validated vocabulary;
    /// `Result` is kept for future input limits.
    pub fn encode(&self, text: &str, options: EncodeOptions) -> Result<Vec<u32>, Error> {
        let mut out = Vec::with_capacity(text.len() / 3 + 2);
        if options.add_bos {
            out.push(self.bos);
        }
        match options.special_tokens {
            SpecialTokens::Plain => self.encode_segment(text, &mut out),
            SpecialTokens::Recognize => {
                let mut start = 0;
                let mut i = 0;
                let bytes = text.as_bytes();
                while i < bytes.len() {
                    // Special tokens are valid UTF-8, so a byte-level match
                    // always starts and ends on character boundaries.
                    match self.special_at(&bytes[i..]) {
                        Some((len, id)) => {
                            self.encode_segment(&text[start..i], &mut out);
                            out.push(id);
                            i += len;
                            start = i;
                        }
                        None => i += 1,
                    }
                }
                self.encode_segment(&text[start..], &mut out);
            }
        }
        Ok(out)
    }

    fn special_at(&self, rest: &[u8]) -> Option<(usize, u32)> {
        if !self.special_first_bytes[rest[0] as usize] {
            return None;
        }
        self.specials
            .iter()
            .find(|(text, _)| rest.starts_with(text.as_bytes()))
            .map(|(text, id)| (text.len(), *id))
    }

    /// One normalized word: every char is a symbol (or its bytes), then
    /// repeatedly apply the lowest-rank adjacent merge, leftmost first.
    fn encode_segment(&self, segment: &str, out: &mut Vec<u32>) {
        if segment.is_empty() {
            return;
        }
        let mut ids = Vec::with_capacity(segment.len());
        let mut buf = [0u8; 4];
        for c in segment.chars() {
            let c = if c == ' ' { METASPACE } else { c };
            let s = c.encode_utf8(&mut buf);
            match self.ids.get(&*s) {
                Some(&id) => ids.push(id),
                None => ids.extend(s.bytes().map(|b| self.byte_ids[b as usize])),
            }
        }
        out.extend(self.merge(ids));
    }

    fn merge(&self, mut ids: Vec<u32>) -> Vec<u32> {
        const NONE: usize = usize::MAX;
        let n = ids.len();
        let mut prev: Vec<usize> = (0..n).map(|i| i.wrapping_sub(1)).collect();
        let mut next: Vec<usize> = (1..=n).map(|i| if i == n { NONE } else { i }).collect();
        let mut alive = vec![true; n];
        // Min-heap on (rank, left position); stale entries are skipped.
        let mut heap = BinaryHeap::new();
        let push = |heap: &mut BinaryHeap<_>, ids: &[u32], left: usize, right: usize| {
            if let Some(&(rank, merged)) = self.merges.get(&(ids[left], ids[right])) {
                heap.push(Reverse((rank, left, merged)));
            }
        };
        for i in 1..n {
            push(&mut heap, &ids, i - 1, i);
        }
        while let Some(Reverse((rank, left, merged))) = heap.pop() {
            let right = next[left];
            if !alive[left]
                || right == NONE
                || self.merges.get(&(ids[left], ids[right])) != Some(&(rank, merged))
            {
                continue;
            }
            ids[left] = merged;
            alive[right] = false;
            next[left] = next[right];
            if next[left] != NONE {
                prev[next[left]] = left;
                push(&mut heap, &ids, left, next[left]);
            }
            if prev[left] != NONE {
                push(&mut heap, &ids, prev[left], left);
            }
        }
        ids.into_iter()
            .zip(alive)
            .filter_map(|(id, alive)| alive.then_some(id))
            .collect()
    }

    /// Rejects the whole call if any ID is out of range, before any output.
    fn visible<'s>(
        &'s self,
        ids: &'s [u32],
        options: DecodeOptions,
    ) -> Result<impl Iterator<Item = (u32, Kind)> + 's, Error> {
        if let Some(&id) = ids.iter().find(|&&id| id as usize >= self.tokens.len()) {
            return Err(Error::TokenOutOfRange(id));
        }
        Ok(ids
            .iter()
            .map(|&id| (id, self.kinds[id as usize]))
            .filter(move |&(_, kind)| !(options.skip_special_tokens && kind == Kind::Special)))
    }

    fn push_text(&self, id: u32, out: &mut String) {
        for c in self.tokens[id as usize].chars() {
            out.push(if c == METASPACE { ' ' } else { c });
        }
    }

    /// Exact bytes: byte tokens become raw bytes, U+2581 becomes a space.
    /// The result may be invalid or end mid-character; use [`Utf8Stream`]
    /// to stream it.
    pub fn decode_bytes(&self, ids: &[u32], options: DecodeOptions) -> Result<Vec<u8>, Error> {
        let mut out = Vec::with_capacity(ids.len() * 4);
        let mut text = String::new();
        for (id, kind) in self.visible(ids, options)? {
            match kind {
                Kind::Byte(b) => out.push(b),
                _ => {
                    text.clear();
                    self.push_text(id, &mut text);
                    out.extend_from_slice(text.as_bytes());
                }
            }
        }
        Ok(out)
    }

    /// Reference text decoding (HF `Replace`, `ByteFallback`, `Fuse`): each
    /// maximal run of consecutive byte tokens becomes its UTF-8 text if the
    /// WHOLE run is valid, otherwise one U+FFFD per byte token in the run.
    /// This intentionally differs from lossy decoding of `decode_bytes`.
    pub fn decode(&self, ids: &[u32], options: DecodeOptions) -> Result<String, Error> {
        let mut out = String::with_capacity(ids.len() * 4);
        let mut run = Vec::new();
        let flush = |run: &mut Vec<u8>, out: &mut String| {
            match std::str::from_utf8(run) {
                Ok(s) => out.push_str(s),
                Err(_) => out.extend(std::iter::repeat_n('\u{FFFD}', run.len())),
            }
            run.clear();
        };
        for (id, kind) in self.visible(ids, options)? {
            match kind {
                Kind::Byte(b) => run.push(b),
                _ => {
                    flush(&mut run, &mut out);
                    self.push_text(id, &mut out);
                }
            }
        }
        flush(&mut run, &mut out);
        Ok(out)
    }
}

/// Incremental UTF-8 for streamed `decode_bytes` output: complete characters
/// are released, an incomplete trailing sequence (at most 3 bytes) is held
/// until more bytes arrive, invalid sequences become U+FFFD as in
/// `String::from_utf8_lossy`.
#[derive(Debug, Default, Clone)]
pub struct Utf8Stream {
    pending: Vec<u8>,
}
impl Utf8Stream {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn pending(&self) -> &[u8] {
        &self.pending
    }
    pub fn push(&mut self, bytes: &[u8]) -> String {
        self.pending.extend_from_slice(bytes);
        let mut out = String::new();
        let mut rest = &self.pending[..];
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
                            rest = &tail[n..];
                        }
                        None => {
                            rest = tail; // incomplete: wait for more bytes
                            break;
                        }
                    }
                }
            }
        }
        self.pending = rest.to_vec();
        out
    }
    /// End of stream: an incomplete trailing sequence becomes U+FFFD.
    pub fn finish(&mut self) -> String {
        let out = String::from_utf8_lossy(&self.pending).into_owned();
        self.pending.clear();
        out
    }
}

#[cfg(test)]
mod tests;
