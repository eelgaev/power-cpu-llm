use super::*;

/// Tiny Gemma4-shaped vocabulary. Expected IDs below are derived by hand
/// from the merge ranks; real-vocabulary parity is in ../tests/tokenizer_contract.rs (outside the repository).
struct Owned {
    tokens: Vec<String>,
    scores: Vec<f64>,
    types: Vec<i64>,
    merges: Vec<String>,
    add_space_prefix: bool,
    bos: u32,
}
const SPECIALS: [(&str, i64); 8] = [
    ("<pad>", CONTROL),
    ("<eos>", CONTROL),
    ("<bos>", CONTROL),
    ("<unk>", CONTROL),
    ("<mask>", CONTROL),
    ("<|turn>", CONTROL),
    ("<|tu", CONTROL),
    ("<|\"|>", USER_DEFINED),
];
const BYTE0: u32 = SPECIALS.len() as u32;
const NORMALS: [&str; 18] = [
    "▁", "a", "b", "c", "ab", "abc", "bc", "aa", "▁a", "<", "|", "t", "u", "r", "n", ">", "é", "x",
];
fn id(token: &str) -> u32 {
    BYTE0 + 256 + NORMALS.iter().position(|&t| t == token).unwrap() as u32
}
fn byte(b: u8) -> u32 {
    BYTE0 + b as u32
}

impl Owned {
    fn new() -> Self {
        let mut tokens: Vec<String> = SPECIALS.iter().map(|(t, _)| t.to_string()).collect();
        let mut types: Vec<i64> = SPECIALS.iter().map(|(_, k)| *k).collect();
        tokens.extend((0..=255u8).map(|b| format!("<0x{b:02X}>")));
        types.extend([BYTE; 256]);
        tokens.extend(NORMALS.iter().map(|t| t.to_string()));
        types.extend([NORMAL; NORMALS.len()]);
        let merges = ["a b", "b c", "ab c", "a a", "▁ a"]
            .map(String::from)
            .to_vec();
        Self {
            scores: vec![0.0; tokens.len()],
            tokens,
            types,
            merges,
            add_space_prefix: false,
            bos: 2,
        }
    }
    fn vocabulary(&self) -> Vocabulary<'_> {
        Vocabulary {
            tokens: &self.tokens,
            scores: &self.scores,
            token_types: &self.types,
            merges: &self.merges,
            chat_template: "",
            bos: self.bos,
            eos: 1,
            padding: 0,
            unknown: 3,
            mask: Some(4),
            add_bos: true,
            add_space_prefix: self.add_space_prefix,
        }
    }
    fn tokenizer(&self) -> Tokenizer {
        Tokenizer::from_vocabulary(&self.vocabulary()).unwrap()
    }
}

#[test]
fn merges_apply_lowest_rank_first_then_leftmost() {
    let t = Owned::new().tokenizer();
    let enc = |s| t.encode(s, EncodeOptions::REFERENCE).unwrap();
    assert_eq!(enc("abc"), [id("abc")]); // (a,b) rank 0 before (b,c) rank 1
    assert_eq!(enc("bc"), [id("bc")]);
    assert_eq!(enc("aaa"), [id("aa"), id("a")]);
    assert_eq!(enc("aaaa"), [id("aa"), id("aa")]);
    assert_eq!(enc("bc a"), [id("bc"), id("▁a")]);
    // No word splitting: merges cross what would be word boundaries elsewhere.
    assert_eq!(enc(" a  a "), [id("▁a"), id("▁"), id("▁a"), id("▁")]);
    assert_eq!(enc(""), Vec::<u32>::new());
    // A literal U+2581 in the input is indistinguishable from a space.
    assert_eq!(enc("▁a"), enc(" a"));
}

#[test]
fn missing_characters_fall_back_to_bytes() {
    let t = Owned::new().tokenizer();
    let enc = |s| t.encode(s, EncodeOptions::REFERENCE).unwrap();
    assert_eq!(enc("é"), [id("é")]);
    assert_eq!(enc("ü"), [byte(0xC3), byte(0xBC)]);
    assert_eq!(enc("a\0b"), [id("a"), byte(0), id("b")]);
    assert_eq!(
        enc("z😀"),
        [byte(b'z'), byte(0xF0), byte(0x9F), byte(0x98), byte(0x80)]
    );
    assert_eq!(enc("\t"), [byte(9)]);
}

#[test]
fn special_token_policies_and_bos() {
    let t = Owned::new().tokenizer();
    let text = "<bos>a<|turn>x<|tux<|\"|>";
    let recognized = [2, id("a"), 5, id("x"), 6, id("x"), 7];
    assert_eq!(
        t.encode(text, EncodeOptions::REFERENCE).unwrap(),
        recognized
    );
    let with_bos = EncodeOptions {
        add_bos: true,
        ..EncodeOptions::REFERENCE
    };
    assert_eq!(t.encode(text, with_bos).unwrap()[..2], [2, 2]); // inserted once, no dedup
    assert_eq!(t.encode("", with_bos).unwrap(), [2]);
    let plain = t.encode("<|turn>", EncodeOptions::PLAIN_TEXT).unwrap();
    assert_eq!(plain, ["<", "|", "t", "u", "r", "n", ">"].map(id));
    assert!(
        t.encode("<bos>", EncodeOptions::PLAIN_TEXT)
            .unwrap()
            .iter()
            .all(|&i| !t.is_special(i))
    );
    assert_eq!(
        t.encode("<|t", EncodeOptions::REFERENCE).unwrap(),
        ["<", "|", "t"].map(id)
    );
}

#[test]
fn decode_matches_reference_byte_fallback_rules() {
    let t = Owned::new().tokenizer();
    let all = DecodeOptions::REFERENCE;
    assert_eq!(t.decode(&[byte(0xC3), byte(0xA9)], all).unwrap(), "é");
    // A run is all-or-nothing: one U+FFFD per byte token if any part is invalid.
    assert_eq!(
        t.decode(&[byte(0xE2), byte(0x82), byte(0xAC), byte(0xC3)], all)
            .unwrap(),
        "\u{FFFD}".repeat(4)
    );
    assert_eq!(
        t.decode(&[byte(0xC3), id("a"), byte(0xA9)], all).unwrap(),
        "\u{FFFD}a\u{FFFD}"
    );
    assert_eq!(t.decode(&[byte(0)], all).unwrap(), "\0");
    assert_eq!(
        t.decode(&[2, id("▁a"), id("▁"), 1], all).unwrap(),
        "<bos> a <eos>"
    );
    assert_eq!(
        t.decode(&[2, id("▁a"), 6, 1], DecodeOptions::SKIP_SPECIAL)
            .unwrap(),
        " a"
    );
    // Skipping specials first can join byte runs, as in the reference.
    assert_eq!(
        t.decode(&[byte(0xC3), 5, byte(0xA9)], DecodeOptions::SKIP_SPECIAL)
            .unwrap(),
        "é"
    );
    assert_eq!(
        t.decode_bytes(&[byte(0xE2), byte(0x82), id("▁a")], all)
            .unwrap(),
        b"\xE2\x82 a"
    );
    assert_eq!(t.decode_bytes(&[], all).unwrap(), b"");
    let max = t.len() as u32;
    assert_eq!(
        t.decode(&[id("a"), max], all),
        Err(Error::TokenOutOfRange(max))
    );
    assert_eq!(
        t.decode_bytes(&[u32::MAX], all),
        Err(Error::TokenOutOfRange(u32::MAX))
    );
}

#[test]
fn round_trip_without_specials() {
    let t = Owned::new().tokenizer();
    for text in ["", " a b  c ", "abc\0\t\r\n😀 é ü", "<bos> stays text"] {
        let ids = t.encode(text, EncodeOptions::PLAIN_TEXT).unwrap();
        assert_eq!(t.decode(&ids, DecodeOptions::REFERENCE).unwrap(), text);
        assert_eq!(
            t.decode_bytes(&ids, DecodeOptions::REFERENCE).unwrap(),
            text.as_bytes()
        );
    }
}

#[test]
fn utf8_stream_holds_incomplete_sequences() {
    let mut s = Utf8Stream::new();
    assert_eq!(s.push(b"a\xE2\x82"), "a");
    assert_eq!(s.pending(), b"\xE2\x82");
    assert_eq!(s.push(b"\xAC b\xF0"), "\u{20AC} b");
    assert_eq!(s.push(b"\x9F\x98\x80"), "😀");
    assert_eq!(s.push(b"\xFFx\xC3"), "\u{FFFD}x");
    assert_eq!(s.finish(), "\u{FFFD}");
    assert_eq!(s.pending(), b"");
    assert_eq!(s.push(b"\xE2\x28"), "\u{FFFD}("); // invalid continuation is not held
}

#[test]
fn accessors_report_special_ids() {
    let t = Owned::new().tokenizer();
    assert_eq!(
        (t.bos(), t.eos(), t.padding(), t.unknown(), t.mask()),
        (2, 1, 0, 3, Some(4))
    );
    assert!(
        t.is_special(6)
            && !t.is_special(byte(0))
            && !t.is_special(id("a"))
            && !t.is_special(u32::MAX)
    );
    assert_eq!(t.token(byte(0x41)), Some("<0x41>"));
    assert_eq!(t.token_id("abc"), Some(id("abc")));
}

#[test]
fn unsupported_vocabularies_are_rejected() {
    let reject = |edit: &dyn Fn(&mut Owned)| {
        let mut v = Owned::new();
        edit(&mut v);
        assert!(matches!(
            Tokenizer::from_vocabulary(&v.vocabulary()),
            Err(Error::Vocabulary(_))
        ));
    };
    reject(&|v| v.add_space_prefix = true);
    reject(&|v| v.tokens[id("x") as usize] = "a".into()); // duplicate
    reject(&|v| v.tokens[id("x") as usize] = String::new());
    reject(&|v| v.types[id("a") as usize] = 2); // UNKNOWN type: semantics unverified
    reject(&|v| v.types[id("a") as usize] = 5); // UNUSED type
    reject(&|v| v.types[byte(7) as usize] = NORMAL); // byte spelling, wrong type
    reject(&|v| v.types[id("x") as usize] = BYTE); // byte type, wrong spelling
    reject(&|v| v.tokens[byte(0xFF) as usize] = "<0xff>".into()); // lowercase is not a byte token
    reject(&|v| v.merges.push("a".into()));
    reject(&|v| v.merges.push("a b c".into()));
    reject(&|v| v.merges.push("c c".into())); // result not in vocabulary
    reject(&|v| v.merges.push("a b".into())); // duplicate pair
    reject(&|v| v.bos = id("a"));
    reject(&|v| {
        v.scores.pop();
    });
    reject(&|v| {
        v.tokens.clear();
        v.types.clear();
        v.scores.clear();
    });
}
