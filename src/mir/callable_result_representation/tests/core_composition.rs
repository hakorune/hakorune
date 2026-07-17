use super::super::{
    CallableResultUnavailableReasonV1, VerifiedCallableResultDispositionV1,
    VerifiedCallableResultEvidenceV1,
};
use super::support::{declarations, disposition, key, qualified_targets, seal_with_targets, site};
use crate::mir::resolved_semantics::SourcePathSegmentV1;

fn exact() -> VerifiedCallableResultDispositionV1 {
    VerifiedCallableResultDispositionV1::ExactI64 {
        required_i64_arguments: Box::new([]),
    }
}

#[test]
fn string_length_aliases_share_the_generated_exact_result_contract() {
    let source = r#"
        static box CoreStringV1 {
            length(value) { local text = "" + value return text.length() }
            len(value) { local text = "" + value return text.len() }
            size(value) { local text = "" + value return text.size() }
        }
    "#;
    let declarations = declarations(source);
    let targets = qualified_targets(&declarations, &[], &[]);
    let results = seal_with_targets(&declarations, &targets);
    let call_site = site(vec![
        SourcePathSegmentV1::Body(1),
        SourcePathSegmentV1::Value,
    ]);

    for name in ["length", "len", "size"] {
        let caller = key(&declarations, "CoreStringV1", name, 1);
        assert_eq!(results.disposition(&caller), Some(&exact()));
        match results.call_result(&caller, &call_site).unwrap().evidence() {
            VerifiedCallableResultEvidenceV1::CoreStringMethod { contract, .. } => {
                assert_eq!(contract.canonical, "length");
            }
            VerifiedCallableResultEvidenceV1::SameModuleStatic { .. } => {
                panic!("expected Core String evidence")
            }
        }
    }
}

#[test]
fn unsupported_receiver_spelling_arity_and_non_i64_results_remain_unavailable() {
    let source = r#"
        static box CoreNegativeV1 {
            receiver(value) { return value.length() }
            spelling(value) { local text = "" + value return text.missing() }
            arity(value) { local text = "" + value return text.length(1) }
            substring(value) { local text = "" + value return text.substring(0) }
            contains(value) { local text = "" + value return text.contains("x") }
        }
    "#;
    let target_unavailable = VerifiedCallableResultDispositionV1::Unavailable(
        CallableResultUnavailableReasonV1::StaticCallTargetAuthorityUnavailable,
    );
    let non_i64 = VerifiedCallableResultDispositionV1::Unavailable(
        CallableResultUnavailableReasonV1::KnownNonI64Return,
    );

    for name in ["receiver", "spelling", "arity"] {
        assert_eq!(
            disposition(source, "CoreNegativeV1", name, 1),
            target_unavailable,
        );
    }
    for name in ["substring", "contains"] {
        assert_eq!(disposition(source, "CoreNegativeV1", name, 1), non_i64,);
    }
}
