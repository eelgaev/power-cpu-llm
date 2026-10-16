use super::*;

#[test]
fn split_follows_the_pixtral_regex() {
    let cases: &[(&str, &[&str])] = &[
        ("Hello world", &["Hello", " world"]),
        ("HELLOworld foo", &["HELLOworld", " foo"]),
        ("HELLO", &["HELLO"]),
        ("I'm", &["I", "'m"]),
        ("abc123", &["abc", "1", "2", "3"]),
        ("a  b", &["a", " ", " b"]),
        ("x \n\n y", &["x", " \n\n", " y"]),
        ("end.  ", &["end", ".", "  "]),
        ("path/to\n", &["path", "/to", "\n"]),
        ("!!!\n\nx", &["!!!\n\n", "x"]),
        (" (x)", &[" (", "x", ")"]),
        ("Ünïcödé café", &["Ünïcödé", " café"]),
        ("日本語", &["日本語"]),
        ("e\u{301}t", &["e\u{301}t"]),
        ("😀x", &["😀x"]),
        ("１２", &["１", "２"]),
        ("\t\tif", &["\t", "\tif"]),
        ("", &[]),
    ];
    for (text, expected) in cases {
        assert_eq!(split(text), *expected, "{text:?}");
    }
}

#[test]
fn split_follows_the_qwen35_regex() {
    let cases: &[(&str, &[&str])] = &[
        ("Hello world", &["Hello", " world"]),
        // Contractions split off (case-insensitive); letters then run.
        ("I'm don't WE'LL", &["I", "'m", " don", "'t", " WE", "'LL"]),
        ("'sun", &["'s", "un"]),
        ("HELLOworld", &["HELLOworld"]),
        ("abc123", &["abc", "1", "2", "3"]),
        ("path/to\n", &["path", "/to", "\n"]),
        ("x;/\n\ny", &["x", ";/\n\n", "y"]),
        ("e\u{301}t", &["e\u{301}t"]),
        (" (x)", &[" (", "x", ")"]),
        ("a  b", &["a", " ", " b"]),
    ];
    for (text, expected) in cases {
        assert_eq!(
            split_with(Pretokenizer::Qwen35, text),
            *expected,
            "{text:?}"
        );
    }
}

#[test]
fn byte_characters_are_gpt2s() {
    let map = byte_chars();
    assert_eq!((map[b' ' as usize], map[b'\n' as usize]), ('Ġ', 'Ċ'));
    assert_eq!((map[b'A' as usize], map[0x7f], map[0xAD]), ('A', 'ġ', 'Ń'));
    let mut sorted = map.to_vec();
    sorted.sort_unstable();
    sorted.dedup();
    assert_eq!(sorted.len(), 256);
}

fn tokenizer() -> Tokenizer {
    let map = byte_chars();
    let mut tokens: Vec<String> = vec!["<s>".into(), "<|im_end|>".into(), "<think>".into()];
    let mut types = vec![CONTROL, CONTROL, USER_DEFINED];
    tokens.extend(map.iter().map(|c| c.to_string()));
    for t in ["Ġw", "or", "ld", "Ġwor", "Ġworld", "ab"] {
        tokens.push(t.into());
    }
    types.resize(tokens.len(), NORMAL);
    let merges: Vec<String> = ["Ġ w", "o r", "l d", "Ġw or", "Ġwor ld"]
        .map(String::from)
        .into();
    // Leaked: the vocabulary borrows, as from a mapped GGUF.
    let vocabulary = Vocabulary {
        tokens: Box::leak(tokens.into_boxed_slice()),
        token_types: Box::leak(types.into_boxed_slice()),
        merges: Box::leak(merges.into_boxed_slice()),
        pre_tokenizer: "pixtral",
        chat_template: "",
        bos: 0,
        eos: 1,
        padding: 0,
        unknown: Some(0),
        add_bos: false,
    };
    Tokenizer::from_vocabulary(&vocabulary).unwrap()
}

#[test]
fn encodes_added_tokens_merges_and_whole_words() {
    let t = tokenizer();
    let id = |s: &str| t.token_id(s).unwrap();
    // " worlds" is not a token: merged to Ġworld + s.
    assert_eq!(
        t.encode(" worlds<think>ab<|im_end|>", true),
        [
            id("Ġworld"),
            id("s"),
            id("<think>"),
            id("ab"),
            id("<|im_end|>")
        ]
    );
    // "ab" has no merge but is a token (ignore_merges); plain mode spells
    // the added-token text out.
    assert_eq!(t.encode("<s>", false), [id("<"), id("s"), id(">")]);
    let ids = t.encode(" worlds<think>é\n<|im_end|>", true);
    assert_eq!(
        t.decode(&ids, false).unwrap(),
        " worlds<think>é\n<|im_end|>"
    );
    assert_eq!(t.decode(&ids, true).unwrap(), " worlds<think>é\n");
    // A split character decodes to U+FFFD; its bytes stay exact.
    let e = t.encode("é", true);
    assert_eq!(e.len(), 2);
    assert_eq!(t.decode(&e[..1], true).unwrap(), "\u{FFFD}");
    assert_eq!(t.decode_bytes(&e[..1], true).unwrap(), [0xC3]);
    assert!(matches!(
        t.decode(&[9999], true),
        Err(Error::TokenOutOfRange(9999))
    ));
}
