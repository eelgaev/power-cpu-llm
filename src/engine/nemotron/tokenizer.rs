//! Byte-level BPE tokenizer of Nemotron 3.5 (GGUF `gpt2`, pre-tokenizer
//! `pixtral`), following its Hugging Face `tokenizer.json`:
//! 1. added tokens (GGUF types CONTROL and USER_DEFINED, e.g. `<|im_end|>`,
//!    `<think>`) are matched in the raw text, leftmost-longest;
//! 2. the rest is split by the pixtral regex (hand-written in [`split`]);
//! 3. each piece's bytes map to GPT-2 byte characters and, unless the whole
//!    piece is a token (`ignore_merges`), are BPE-merged.
//!
//! No BOS is added (`add_bos_token` is false); the chat template writes the
//! prompt's special tokens itself.

use crate::engine::nemotron::Vocabulary;
use crate::engine::tokenizer::{Error, merge};
use std::collections::HashMap;

mod unicode;

const NORMAL: i64 = 1;
const CONTROL: i64 = 3;
const USER_DEFINED: i64 = 4;
/// Reserved padding entries (Qwen's `[PAD...]`): decodable, never merged to.
const UNUSED: i64 = 5;

/// The pre-tokenizer's character classes: the regex's `\p{Lu}\p{Lt}`
/// (Upper), `\p{Ll}` (Lower), `\p{Lm}\p{Lo}` (Letter), `\p{M}` and `\p{N}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Class {
    Upper,
    Lower,
    Letter,
    Mark,
    Number,
    Other,
}

fn class(c: char) -> Class {
    match c {
        'A'..='Z' => Class::Upper,
        'a'..='z' => Class::Lower,
        '0'..='9' => Class::Number,
        '\0'..='\x7f' => Class::Other,
        _ => {
            let cp = c as u32;
            let table = &unicode::RANGES;
            match table.binary_search_by(|&(first, last, _)| {
                if last < cp {
                    std::cmp::Ordering::Less
                } else if first > cp {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Equal
                }
            }) {
                Ok(i) => table[i].2,
                Err(_) => Class::Other,
            }
        }
    }
}

/// `[\p{Lu}\p{Lt}\p{Lm}\p{Lo}\p{M}]`.
fn upper(c: char) -> bool {
    matches!(class(c), Class::Upper | Class::Letter | Class::Mark)
}
/// `[\p{Ll}\p{Lm}\p{Lo}\p{M}]`.
fn lower(c: char) -> bool {
    matches!(class(c), Class::Lower | Class::Letter | Class::Mark)
}
fn letter_or_number(c: char) -> bool {
    matches!(
        class(c),
        Class::Upper | Class::Lower | Class::Letter | Class::Number
    )
}
/// `[^\r\n\p{L}\p{N}]`.
fn prefix(c: char) -> bool {
    c != '\r' && c != '\n' && !letter_or_number(c)
}
/// `[^\s\p{L}\p{N}]`.
fn symbol(c: char) -> bool {
    !c.is_whitespace() && !letter_or_number(c)
}
fn newline(c: char) -> bool {
    c == '\r' || c == '\n'
}

/// The length of the run of `chars[from..]` satisfying `test`.
fn run(chars: &[char], from: usize, test: fn(char) -> bool) -> usize {
    chars[from.min(chars.len())..]
        .iter()
        .take_while(|&&c| test(c))
        .count()
}

/// `U* L+` from `j` (greedy with backtracking): its end, if it matches.
fn lower_tail(chars: &[char], j: usize) -> Option<usize> {
    let m = run(chars, j, upper);
    (0..=m)
        .rev()
        .find(|&k| chars.get(j + k).is_some_and(|&c| lower(c)))
        .map(|k| j + k + run(chars, j + k, lower))
}
/// `U+ L*` from `j`.
fn upper_head(chars: &[char], j: usize) -> Option<usize> {
    let m = run(chars, j, upper);
    (m > 0).then(|| j + m + run(chars, j + m, lower))
}

/// The end of the regex match at `i` (the first alternative that matches):
/// ```text
/// [^\r\n\p{L}\p{N}]?[\p{Lu}\p{Lt}\p{Lm}\p{Lo}\p{M}]*[\p{Ll}\p{Lm}\p{Lo}\p{M}]+
/// |[^\r\n\p{L}\p{N}]?[\p{Lu}\p{Lt}\p{Lm}\p{Lo}\p{M}]+[\p{Ll}\p{Lm}\p{Lo}\p{M}]*
/// |\p{N}| ?[^\s\p{L}\p{N}]+[\r\n/]*|\s*[\r\n]+|\s+(?!\S)|\s+
/// ```
fn match_at(chars: &[char], i: usize) -> usize {
    let with_prefix = |core: fn(&[char], usize) -> Option<usize>| {
        prefix(chars[i])
            .then(|| core(chars, i + 1))
            .flatten()
            .or_else(|| core(chars, i))
    };
    if let Some(end) = with_prefix(lower_tail).or_else(|| with_prefix(upper_head)) {
        return end;
    }
    if class(chars[i]) == Class::Number {
        return i + 1;
    }
    let j = if chars[i] == ' ' { i + 1 } else { i };
    let symbols = run(chars, j, symbol);
    if symbols > 0 {
        let end = j + symbols;
        return end + run(chars, end, |c| newline(c) || c == '/');
    }
    whitespace_end(chars, i)
}

/// `[\p{L}\p{M}]`.
fn letter_or_mark(c: char) -> bool {
    matches!(
        class(c),
        Class::Upper | Class::Lower | Class::Letter | Class::Mark
    )
}
/// `[^\s\p{L}\p{M}\p{N}]`.
fn qwen_symbol(c: char) -> bool {
    symbol(c) && class(c) != Class::Mark
}

/// The end of the `qwen35` regex match at `i`:
/// ```text
/// (?:'[sS]|'[tT]|'[rR][eE]|'[vV][eE]|'[mM]|'[lL][lL]|'[dD])
/// |[^\r\n\p{L}\p{N}]?[\p{L}\p{M}]+|\p{N}| ?[^\s\p{L}\p{M}\p{N}]+[\r\n]*
/// |\s*[\r\n]+|\s+(?!\S)|\s+
/// ```
fn match_qwen35(chars: &[char], i: usize) -> usize {
    if chars[i] == '\'' {
        let lower = |k: usize| chars.get(i + k).map(char::to_ascii_lowercase);
        match (lower(1), lower(2)) {
            (Some('s' | 't' | 'm' | 'd'), _) => return i + 2,
            (Some('r'), Some('e')) | (Some('v'), Some('e')) | (Some('l'), Some('l')) => {
                return i + 3;
            }
            _ => {}
        }
    }
    let letters =
        |j: usize| (run(chars, j, letter_or_mark) > 0).then(|| j + run(chars, j, letter_or_mark));
    if let Some(end) = prefix(chars[i])
        .then(|| letters(i + 1))
        .flatten()
        .or_else(|| letters(i))
    {
        return end;
    }
    if class(chars[i]) == Class::Number {
        return i + 1;
    }
    let j = if chars[i] == ' ' { i + 1 } else { i };
    let symbols = run(chars, j, qwen_symbol);
    if symbols > 0 {
        let end = j + symbols;
        return end + run(chars, end, newline);
    }
    whitespace_end(chars, i)
}

/// The shared whitespace alternatives `\s*[\r\n]+|\s+(?!\S)|\s+` at `i`.
fn whitespace_end(chars: &[char], i: usize) -> usize {
    let spaces = run(chars, i, char::is_whitespace);
    if let Some(last) = (i..i + spaces).rev().find(|&p| newline(chars[p])) {
        return last + 1;
    }
    match spaces {
        // Unreachable (every character matches an alternative); a lone
        // character is the safe fallback.
        0 => i + 1,
        // `\s+(?!\S)` leaves the last space to the following word.
        n if i + n < chars.len() && n > 1 => i + n - 1,
        n => i + n,
    }
}

/// A GGUF `tokenizer.ggml.pre` this tokenizer implements.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pretokenizer {
    /// Nemotron 3.5.
    Pixtral,
    /// Qwen 3.5/3.6.
    Qwen35,
}

/// The pixtral pre-tokenizer: `text` as consecutive regex matches.
pub fn split(text: &str) -> Vec<&str> {
    split_with(Pretokenizer::Pixtral, text)
}

/// `text` as consecutive matches of `pre`'s regex.
pub fn split_with(pre: Pretokenizer, text: &str) -> Vec<&str> {
    let chars: Vec<char> = text.chars().collect();
    let mut offsets: Vec<usize> = text.char_indices().map(|(o, _)| o).collect();
    offsets.push(text.len());
    let mut pieces = Vec::new();
    let mut i = 0;
    while i < chars.len() {
        let end = match pre {
            Pretokenizer::Pixtral => match_at(&chars, i),
            Pretokenizer::Qwen35 => match_qwen35(&chars, i),
        };
        pieces.push(&text[offsets[i]..offsets[end]]);
        i = end;
    }
    pieces
}

/// GPT-2's reversible byte-to-character map: printable Latin-1 bytes map to
/// themselves, the other 68 to U+0100 onward in byte order.
fn byte_chars() -> [char; 256] {
    let mut map = ['\0'; 256];
    let mut next = 256u32;
    for b in 0..=255u8 {
        let printable = matches!(b, b'!'..=b'~' | 0xA1..=0xAC | 0xAE..=0xFF);
        map[b as usize] = if printable {
            char::from(b)
        } else {
            next += 1;
            char::from_u32(next - 1).expect("below the surrogates")
        };
    }
    map
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Kind {
    Normal,
    /// GGUF CONTROL: dropped by `skip_special_tokens`.
    Special,
    /// GGUF USER_DEFINED (`<think>`, `<tool_call>`, ...): matched like special
    /// tokens but kept when decoding.
    Added,
}

#[derive(Debug)]
pub struct Tokenizer {
    tokens: Vec<Box<str>>,
    kinds: Vec<Kind>,
    ids: HashMap<Box<str>, u32>,
    merges: HashMap<(u32, u32), (u32, u32)>,
    byte_ids: [u32; 256],
    char_bytes: HashMap<char, u8>,
    byte_chars: [char; 256],
    /// Longest first, so the first match at a position is the longest.
    added: Vec<(Box<str>, u32)>,
    bos: u32,
    eos: u32,
    pre: Pretokenizer,
}

impl Tokenizer {
    pub fn from_vocabulary(vocabulary: &Vocabulary<'_>) -> Result<Self, Error> {
        let invalid = |m: String| Error::Vocabulary(m);
        let pre = match vocabulary.pre_tokenizer {
            "pixtral" => Pretokenizer::Pixtral,
            "qwen35" => Pretokenizer::Qwen35,
            other => {
                return Err(invalid(format!("unsupported pre-tokenizer {other:?}")));
            }
        };
        let count = vocabulary.tokens.len();
        if count == 0 || count > u32::MAX as usize || vocabulary.token_types.len() != count {
            return Err(invalid("vocabulary size".into()));
        }
        let byte_chars = byte_chars();
        let char_bytes: HashMap<char, u8> =
            (0..=255u8).map(|b| (byte_chars[b as usize], b)).collect();
        let mut ids = HashMap::with_capacity(count);
        let mut kinds = Vec::with_capacity(count);
        let mut added = Vec::new();
        for (id, (token, &kind)) in vocabulary
            .tokens
            .iter()
            .zip(vocabulary.token_types)
            .enumerate()
        {
            let id = id as u32;
            if token.is_empty() || ids.insert(Box::<str>::from(token.as_str()), id).is_some() {
                return Err(invalid(format!("empty or duplicate token {id}")));
            }
            kinds.push(match kind {
                NORMAL | UNUSED if token.chars().all(|c| char_bytes.contains_key(&c)) => {
                    Kind::Normal
                }
                CONTROL | USER_DEFINED => {
                    added.push((Box::<str>::from(token.as_str()), id));
                    if kind == CONTROL {
                        Kind::Special
                    } else {
                        Kind::Added
                    }
                }
                _ => {
                    return Err(invalid(format!(
                        "token {id} {token:?} has unsupported type {kind} or spelling"
                    )));
                }
            });
        }
        let mut byte_ids = [0u32; 256];
        for (b, c) in byte_chars.iter().enumerate() {
            byte_ids[b] = *ids
                .get(c.to_string().as_str())
                .filter(|&&id| kinds[id as usize] == Kind::Normal)
                .ok_or_else(|| invalid(format!("missing byte token {c:?}")))?;
        }
        let mut merges = HashMap::with_capacity(vocabulary.merges.len());
        for (rank, m) in vocabulary.merges.iter().enumerate() {
            let (left, right) = m
                .split_once(' ')
                .filter(|(a, b)| !a.is_empty() && !b.is_empty() && !b.contains(' '))
                .ok_or_else(|| invalid(format!("malformed merge {m:?}")))?;
            let lookup = |s: &str| {
                ids.get(s)
                    .copied()
                    .ok_or_else(|| invalid(format!("merge {m:?} uses unknown {s:?}")))
            };
            let pair = (lookup(left)?, lookup(right)?);
            let merged = lookup(&format!("{left}{right}"))?;
            let rank = u32::try_from(rank).map_err(|_| invalid("too many merges".into()))?;
            if merges.insert(pair, (rank, merged)).is_some() {
                return Err(invalid(format!("duplicate merge {m:?}")));
            }
        }
        for id in [vocabulary.bos, vocabulary.eos] {
            if kinds.get(id as usize) != Some(&Kind::Special) {
                return Err(invalid(format!("BOS/EOS {id} is not a special token")));
            }
        }
        added.sort_by(|a, b| b.0.len().cmp(&a.0.len()).then(a.1.cmp(&b.1)));
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
            char_bytes,
            byte_chars,
            added,
            bos: vocabulary.bos,
            eos: vocabulary.eos,
            pre,
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
    pub fn bos(&self) -> u32 {
        self.bos
    }
    pub fn eos(&self) -> u32 {
        self.eos
    }
    /// The texts matched as added tokens (CONTROL and USER_DEFINED).
    pub fn added_tokens(&self) -> impl Iterator<Item = &str> {
        self.added.iter().map(|(t, _)| &**t)
    }
    /// CONTROL tokens (`<s>`, `<|im_end|>`, ...), not `<think>` and the like.
    pub fn is_special(&self, id: u32) -> bool {
        self.kinds.get(id as usize) == Some(&Kind::Special)
    }

    /// With `recognize_added`, added-token text in `text` becomes those tokens
    /// (the reference default); otherwise it is ordinary text (for untrusted
    /// content spliced into a rendered prompt).
    pub fn encode(&self, text: &str, recognize_added: bool) -> Vec<u32> {
        let mut out = Vec::with_capacity(text.len() / 3 + 2);
        if !recognize_added {
            self.encode_text(text, &mut out);
            return out;
        }
        let (mut start, mut i) = (0, 0);
        let bytes = text.as_bytes();
        while i < bytes.len() {
            // Added tokens are valid UTF-8: a byte-level match is on character
            // boundaries.
            match self
                .added
                .iter()
                .find(|(t, _)| bytes[i..].starts_with(t.as_bytes()))
            {
                Some((t, id)) => {
                    self.encode_text(&text[start..i], &mut out);
                    out.push(*id);
                    i += t.len();
                    start = i;
                }
                None => i += 1,
            }
        }
        self.encode_text(&text[start..], &mut out);
        out
    }

    fn encode_text(&self, text: &str, out: &mut Vec<u32>) {
        let mut word = String::new();
        for piece in split_with(self.pre, text) {
            word.clear();
            word.extend(piece.bytes().map(|b| self.byte_chars[b as usize]));
            match self.ids.get(word.as_str()) {
                Some(&id) if self.kinds[id as usize] == Kind::Normal => out.push(id),
                _ => out.extend(merge(
                    &self.merges,
                    piece.bytes().map(|b| self.byte_ids[b as usize]).collect(),
                )),
            }
        }
    }

    /// Exact bytes of `ids`; added tokens are their text. The result may end
    /// mid-character (see [`crate::engine::tokenizer::Utf8Stream`]).
    pub fn decode_bytes(&self, ids: &[u32], skip_special: bool) -> Result<Vec<u8>, Error> {
        if let Some(&id) = ids.iter().find(|&&id| id as usize >= self.tokens.len()) {
            return Err(Error::TokenOutOfRange(id));
        }
        let mut out = Vec::with_capacity(ids.len() * 4);
        for &id in ids {
            let token = &self.tokens[id as usize];
            match self.kinds[id as usize] {
                Kind::Normal => out.extend(token.chars().map(|c| self.char_bytes[&c])),
                Kind::Special if skip_special => {}
                _ => out.extend_from_slice(token.as_bytes()),
            }
        }
        Ok(out)
    }

    /// [`Self::decode_bytes`] as text; invalid UTF-8 becomes U+FFFD (HF
    /// `ByteLevel` decoding).
    pub fn decode(&self, ids: &[u32], skip_special: bool) -> Result<String, Error> {
        Ok(String::from_utf8_lossy(&self.decode_bytes(ids, skip_special)?).into_owned())
    }
}

#[cfg(test)]
mod tests;
