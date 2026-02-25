use crate::parser::NyashParser;

#[test]
fn outbox_duplicate_binding_is_fail_fast() {
    let src = r#"
    outbox payload, payload
    "#;
    let err = NyashParser::parse_from_string(src).expect_err("parse should fail");
    assert!(
        err.to_string()
            .contains("[freeze:contract][moved/outbox_duplicate]"),
        "unexpected error: {}",
        err
    );
}
