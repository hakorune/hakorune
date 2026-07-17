use crate::mir::resolved_semantics::SourcePathSegmentV1;

use super::super::{VerifiedCallableResultDispositionV1, VerifiedCallableResultEvidenceV1};
use super::support::{
    declarations, extend_current_owner_targets, key, qualified_targets, seal_with_targets, site,
    CallSiteSpecV1,
};

fn exact(requirements: &[u32]) -> VerifiedCallableResultDispositionV1 {
    VerifiedCallableResultDispositionV1::ExactI64 {
        required_i64_arguments: requirements.into(),
    }
}

#[test]
fn actual_string_helpers_and_parser_wrapper_share_one_complete_result_catalog() {
    let source = format!(
        "{}\n{}",
        include_str!(concat!(
            "../../../../lang/src/shared/common/",
            "string_helpers.hako"
        )),
        include_str!(concat!(
            "../../../../lang/src/compiler/parser/scan/",
            "parser_string_utils_box.hako"
        )),
    );
    let declarations = declarations(&source);
    let wrapper = key(&declarations, "ParserStringUtilsBox", "skip_ws", 2);
    let to_i64 = key(&declarations, "StringHelpers", "to_i64", 1);
    let wrapper_site = site(vec![
        SourcePathSegmentV1::Body(0),
        SourcePathSegmentV1::Value,
    ]);
    let digit_site = site(vec![
        SourcePathSegmentV1::Body(12),
        SourcePathSegmentV1::LoopBody(2),
        SourcePathSegmentV1::Initializer(0),
    ]);
    let length_site = site(vec![
        SourcePathSegmentV1::Body(2),
        SourcePathSegmentV1::Initializer(0),
    ]);

    let targets = qualified_targets(
        &declarations,
        &[("StringHelpers", "StringHelpers")],
        &[CallSiteSpecV1 {
            caller_owner: "ParserStringUtilsBox",
            caller_name: "skip_ws",
            caller_arity: 2,
            site: wrapper_site.clone(),
        }],
    );
    let targets = extend_current_owner_targets(
        targets,
        &declarations,
        &[CallSiteSpecV1 {
            caller_owner: "StringHelpers",
            caller_name: "to_i64",
            caller_arity: 1,
            site: digit_site.clone(),
        }],
    );
    let results = seal_with_targets(&declarations, &targets);

    for (owner, name, arity, expected) in [
        ("StringHelpers", "skip_ws", 2, exact(&[1])),
        ("ParserStringUtilsBox", "skip_ws", 2, exact(&[1])),
        ("StringHelpers", "to_i64", 1, exact(&[])),
        ("StringHelpers", "_digit_value", 1, exact(&[])),
    ] {
        assert_eq!(
            results.disposition(&key(&declarations, owner, name, arity)),
            Some(&expected),
            "unexpected result row for {owner}.{name}/{arity}",
        );
    }

    let wrapper_row = results.call_result(&wrapper, &wrapper_site).unwrap();
    assert_eq!(
        wrapper_row.static_target_key().unwrap().owner(),
        "StringHelpers"
    );
    assert_eq!(wrapper_row.required_i64_arguments(), &[1]);

    let digit_row = results.call_result(&to_i64, &digit_site).unwrap();
    assert_eq!(
        digit_row.static_target_key().unwrap().name(),
        "_digit_value"
    );

    let length_row = results.call_result(&to_i64, &length_site).unwrap();
    match length_row.evidence() {
        VerifiedCallableResultEvidenceV1::CoreStringMethod { contract, .. } => {
            assert_eq!(contract.receiver_box, "StringBox");
            assert_eq!(contract.canonical, "length");
        }
        VerifiedCallableResultEvidenceV1::SameModuleStatic { .. } => {
            panic!("expected Core String result evidence")
        }
    }
}
