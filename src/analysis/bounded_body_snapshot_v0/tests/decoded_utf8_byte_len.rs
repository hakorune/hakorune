use super::*;

#[test]
fn decoded_utf8_byte_len_v0_matches_the_declared_unicode_contract() {
    for (value, expected) in [
        ("abc", 3),
        ("猫", 3),
        ("😸", 4),
        ("猫😸", 7),
        ("e\u{0301}", 3),
        ("é", 2),
        ("\0", 1),
        ("a\0b", 3),
    ] {
        assert_eq!(DecodedUtf8ByteLenV0::count(value), expected, "{value:?}");
    }
}

#[test]
fn decoded_utf8_byte_len_v0_does_not_normalize_text() {
    assert_ne!("e\u{0301}", "é");
    assert_eq!(DecodedUtf8ByteLenV0::count("e\u{0301}"), 3);
    assert_eq!(DecodedUtf8ByteLenV0::count("é"), 2);
}

#[test]
fn validated_text_constructor_derives_the_only_byte_witness() {
    let atom = ValidatedTextV0::from_decoded("猫😸", TextClassV0::Atom);
    assert_eq!(atom.value(), "猫😸");
    assert_eq!(atom.utf8_byte_len(), 7);
    assert_eq!(atom.class(), TextClassV0::Atom);

    let literal = ValidatedTextV0::from_decoded("e\u{0301}", TextClassV0::Literal);
    assert_eq!(literal.utf8_byte_len(), 3);
    assert_eq!(literal.class(), TextClassV0::Literal);
}

#[test]
fn validated_text_and_budget_consume_the_decoded_utf8_leaf() {
    let input = r#"{"version":0,"kind":"Program","body":[{"type":"Local","name":"猫😸","expr":{"type":"Str","value":"猫😸"}}]}"#;
    let view = read_program_v0_body(input).expect("strict input");
    let node = view.body_node(0).expect("local node");
    let atoms = node.atoms();
    assert!(matches!(
        atoms.as_slice(),
        [(AtomKeyV0::Name, ValidatedAtomValueV0::Text(text))]
            if text.utf8_byte_len() == 7 && text.class() == TextClassV0::Atom
    ));

    let mut budget = BoundedBodyBudgetV0::default();
    assert_eq!(budget.observe_atom("猫😸"), Ok(()));
    assert_eq!(budget.observe_literal("猫😸"), Ok(()));
    assert_eq!(budget.total_text_bytes(), 14);
}
