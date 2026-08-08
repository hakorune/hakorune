use super::NyashParser;
use crate::tokenizer::NyashTokenizer;

#[test]
fn parser_session_owns_fresh_brand_and_top_level_cursor() {
    let left = NyashParser::new(Vec::new());
    let right = NyashParser::new(Vec::new());

    assert_ne!(
        left.source_invocation_brand(),
        right.source_invocation_brand()
    );
    assert_eq!(left.next_source_statement_ordinal, 0);
    assert_eq!(left.active_source_statement_ordinal(), None);
}

#[test]
fn parser_session_advances_top_level_cursor_once_per_statement() {
    let mut tokenizer = NyashTokenizer::new("box First {}\nbox Second {}\n");
    let tokens = tokenizer.tokenize().unwrap();
    let mut parser = NyashParser::new(tokens);

    parser.parse().unwrap();

    assert_eq!(parser.next_source_statement_ordinal, 2);
    assert_eq!(parser.active_source_statement_ordinal(), None);
}
