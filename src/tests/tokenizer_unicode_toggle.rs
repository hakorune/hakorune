use crate::tokenizer::{NyashTokenizer, TokenType};
use std::sync::{Mutex, OnceLock};

fn env_guard() -> &'static Mutex<()> {
    static GUARD: OnceLock<Mutex<()>> = OnceLock::new();
    GUARD.get_or_init(|| Mutex::new(()))
}

fn clear_unicode_toggle_env() {
    std::env::remove_var("NYASH_PARSER_DECODE_UNICODE");
    std::env::remove_var("HAKO_PARSER_DECODE_UNICODE");
}

fn collect_string_token(src: &str) -> String {
    let mut t = NyashTokenizer::new(src);
    let tokens = t.tokenize().expect("tokenize");
    // Expect first non-EOF token to be STRING
    for tok in tokens {
        if let TokenType::STRING(s) = tok.token_type {
            return s;
        }
    }
    panic!("no STRING token found");
}

#[test]
fn unicode_decode_toggle_off_keeps_literal() {
    // OFF by default (guarded to avoid test-order races)
    let _lock = env_guard().lock().unwrap();
    clear_unicode_toggle_env();

    let s = collect_string_token("\"\\u0041\"");
    assert_eq!(s, "\\u0041");

    // cleanup
    clear_unicode_toggle_env();
}

#[test]
fn unicode_decode_toggle_on_decodes_basic_and_surrogate() {
    // ON: enable decode (guarded to avoid leaking env to other tests)
    let _lock = env_guard().lock().unwrap();
    clear_unicode_toggle_env();
    std::env::set_var("NYASH_PARSER_DECODE_UNICODE", "1");

    let s = collect_string_token("\"\\u0041\"");
    assert_eq!(s, "A");

    let s2 = collect_string_token("\"\\uD83D\\uDE00\"");
    // Expect surrogate pair to decode into one char (😀)
    assert_eq!(s2.chars().count(), 1);

    // cleanup
    clear_unicode_toggle_env();
}
