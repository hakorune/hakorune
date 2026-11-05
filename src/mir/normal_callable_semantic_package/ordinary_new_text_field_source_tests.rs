//! The Text field source is lent for a declared Text slot only when every original writer stores
//! Text, and it rechecks against its own field name and its own ledger.
use super::TextFieldValueSourceV1;
use crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog;

#[test]
fn declared_text_slot_spellings_are_admitted_and_other_declarations_are_not() {
    for (declared, admitted) in [
        (None, true), (Some("String"), true), (Some("StringBox"), true), (Some("String?"), false),
        (Some("i64"), false), (Some("ArrayBox"), false), (Some("Leaf"), false), (Some("String[]"), false),
    ] {
        assert_eq!(TextFieldValueSourceV1::declaration_admits_v1(declared), admitted, "{declared:?}");
    }
}

#[test]
fn typed_string_field_lends_text_only_for_a_closed_text_writer_set_and_rechecks_exactly() {
    for (ty, replacement, expected) in [
        ("String", "\"雪\"", true), ("StringBox", "\"雪\"", true), ("String?", "\"雪\"", false),
        ("String", "7", false), ("String?", "null", false),
    ] {
        let text = format!("box Page {{ value: {ty} birth() {{ me.value = \"\" }}
            change() {{ me.value = {replacement} }} }}
            static box Main {{ main() {{ local page = new Page() return 0 }} }}");
        let package = issue_with_brand_catalog(&text).unwrap();
        let ledger = &package.ordinary_new_claim_ledger;
        let field = ledger.field_value_source("Page", "value").unwrap().as_ref().unwrap();
        let ingress = ledger.borrowed_formal_source.as_ref().unwrap().as_ref().unwrap();
        let source = TextFieldValueSourceV1::lend(ingress, &ledger.field_write_claims, field);
        assert_eq!(source.is_some(), expected, "{ty} / {replacement}");
        if let Some(source) = source {
            assert!(source.recheck(ingress, &ledger.field_write_claims, "value"));
            assert!(!source.recheck(ingress, &ledger.field_write_claims, "other"), "site drift");
            let foreign = issue_with_brand_catalog(&text).unwrap();
            let other = &foreign.ordinary_new_claim_ledger;
            assert!(!source.recheck(ingress, &other.field_write_claims, "value"), "owner drift");
        }
    }
}
