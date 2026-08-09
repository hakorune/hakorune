use super::*;

#[test]
fn body_transaction_emits_ast_free_ordered_body_source() {
    let transaction = NyashParser::parse_from_string_with_resolver_body_source(
        "box TextLike { @rune CallableContract(query) length(): i64 { return 0 } }",
        ParserBuildConfig::default(),
    )
    .expect("bounded direct Box body transaction should parse");
    let (handoff, envelope) = transaction
        .into_parts()
        .expect("body transaction should decompose exactly once");

    assert_eq!(handoff.boxes().len(), 1);
    envelope.consume_with(|_, rows| {
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].source_site().box_statement_ordinal(), 0);
        assert_eq!(rows[0].source_site().member_ordinal(), 0);
        assert_eq!(rows[0].name(), "length");
        assert_eq!(rows[0].body_item_ordinals(), &[0]);
    });
}
