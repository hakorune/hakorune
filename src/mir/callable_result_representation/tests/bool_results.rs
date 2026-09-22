use super::super::VerifiedCallableResultDispositionV1 as Disposition;
use super::support::disposition;

#[test]
fn bool_results_preserve_operator_and_merge_boundaries() {
    let source = r#"static box Predicates {
        literal() { return true }
        equals(x) { return x == " " }
        logic(x) { return x == " " || x == "\t" }
        negated(x) { return !(x == " ") }
        mixed_string(x) { if x == 0 { return true } return "x" }
        mixed_integer(x) { if x == 0 { return false } return 1 }
        arithmetic(x) { return x + 1 }
        bitwise(x) { return x & 1 }
        shift(x) { return x << 1 }
    }"#;
    for (method, arity) in [("literal", 0), ("equals", 1), ("logic", 1), ("negated", 1)] {
        assert_eq!(
            disposition(source, "Predicates", method, arity),
            Disposition::ExactBool,
            "{method}"
        );
    }
    for method in [
        "mixed_string",
        "mixed_integer",
        "arithmetic",
        "bitwise",
        "shift",
    ] {
        assert_ne!(
            disposition(source, "Predicates", method, 1),
            Disposition::ExactBool,
            "{method}"
        );
    }
    assert_eq!(
        disposition(
            include_str!("../../../../lang/src/shared/common/string_helpers.hako"),
            "StringHelpers",
            "is_space",
            1
        ),
        Disposition::ExactBool
    );
}

#[test]
fn bool_proof_preserves_logical_and_not_call_coverage() {
    use super::super::{
        StaticCallResultPublicationTakeV1, VerifiedStaticCallResultPublicationOwnerV1,
    };
    use super::support::{declarations, key, seal_with_targets};
    use crate::mir::source_call_target::{
        VerifiedStaticImportAliasViewV1, VerifiedWholeSourceStaticCallTargetInventoryV1,
    };
    let declarations = declarations(
        r#"static box Predicates {
        value() { return true }
        negated() { return !me.value() }
        logical() { return false && me.value() }
    }"#,
    );
    let imports = VerifiedStaticImportAliasViewV1::seal(
        &declarations,
        std::iter::empty::<(String, String)>(),
    )
    .unwrap();
    let targets = VerifiedWholeSourceStaticCallTargetInventoryV1::verify(&declarations, &imports)
        .unwrap()
        .into_targets();
    let results = seal_with_targets(&declarations, &targets);
    let mut owner =
        VerifiedStaticCallResultPublicationOwnerV1::issue(&declarations, &targets, &results)
            .unwrap();
    let mut sites = 0;
    for ((caller, site), _) in targets.rows() {
        assert_eq!(results.disposition(caller), Some(&Disposition::ExactBool));
        assert!(results.call_result(caller, site).is_some());
        assert!(matches!(
            owner.take_for_source(&declarations, caller, site).unwrap(),
            StaticCallResultPublicationTakeV1::Selected(_)
        ));
        sites += 1;
    }
    assert_eq!(sites, 2);
    assert_eq!(
        results.disposition(&key(&declarations, "Predicates", "value", 0)),
        Some(&Disposition::ExactBool)
    );
    owner.finish_empty().unwrap();
}
