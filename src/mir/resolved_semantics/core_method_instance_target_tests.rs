use super::*;
use crate::mir::core_method_op::{CoreMethodLoweringTier, CoreMethodOp};
use crate::mir::core_method_result_kind::{
    issue_core_method_manifest_row_ref_for_test, issue_core_method_manifest_row_ref_v1,
    issue_core_method_manifest_test_row_ref, CoreMethodContractResultRowV1, CoreMethodEffectV1,
    CoreMethodResultKindV1, CORE_METHOD_MANIFEST_BRAND_V1,
};

fn issuer() -> CoreMethodInstanceTargetIssuerV1 {
    CoreMethodInstanceTargetIssuerV1::string_box_text(CORE_METHOD_MANIFEST_BRAND_V1)
        .expect("generated manifest brand should issue")
}

#[test]
fn string_len_issues_exact_typed_home_target() {
    let row = issue_core_method_manifest_row_ref_v1(CoreMethodOp::StringLen, 0)
        .expect("generated StringLen/0 row");
    let mut issuer = issuer();
    let target = issuer.issue(row).expect("StringLen/0 target");

    assert_eq!(target.schema(), CoreMethodHomeSchemaV1::StringBoxText);
    assert_eq!(
        target.receiver(),
        CoreMethodHomeReceiverRelationV1::StringBoxReceiver
    );
    assert_eq!(target.parameters(), &[]);
    assert_eq!(target.result(), CoreMethodHomeResultRelationV1::I64ToCaller);
    assert_eq!(
        target.abi_profile(),
        CoreMethodHomeAbiProfileV1::StringBoxTextV1
    );
    assert_eq!(
        target.execution_policy(),
        CoreMethodHomeExecutionPolicyV1::NonSuspendingNonControl
    );
    assert_eq!(target.row().arity(), 0);
}

#[test]
fn string_substring_specializes_union_row_to_exact_arity_two() {
    let row = issue_core_method_manifest_row_ref_v1(CoreMethodOp::StringSubstring, 2)
        .expect("generated StringSubstring/2 row");
    let mut issuer = issuer();
    let target = issuer.issue(row).expect("StringSubstring/2 target");

    assert_eq!(
        target.parameters(),
        &[CoreMethodHomeParameterRelationV1::I64Parameter; 2]
    );
    assert_eq!(
        target.result(),
        CoreMethodHomeResultRelationV1::TextToCaller
    );
    assert_eq!(target.row().arity(), 2);
}

#[test]
fn target_issuer_rejects_union_arity_one_and_duplicate_target() {
    let row = issue_core_method_manifest_row_ref_v1(CoreMethodOp::StringSubstring, 1)
        .expect("generated union row should still be observable");
    let mut issuer = issuer();
    assert!(matches!(
        issuer.issue(row),
        Err(CoreMethodInstanceTargetRejectV1::UnsupportedOperation {
            op: CoreMethodOp::StringSubstring,
            arity: 1,
        })
    ));

    let row = issue_core_method_manifest_row_ref_v1(CoreMethodOp::StringLen, 0)
        .expect("generated StringLen/0 row");
    issuer.issue(row).expect("first target");
    assert!(matches!(
        issuer.issue(row),
        Err(CoreMethodInstanceTargetRejectV1::DuplicateTarget {
            op: CoreMethodOp::StringLen,
            arity: 0,
        })
    ));
}

#[test]
fn target_issuer_rejects_foreign_brand_and_wrong_receiver() {
    let foreign = issue_core_method_manifest_row_ref_for_test(CoreMethodOp::StringLen, 0, true)
        .expect("foreign test row");
    assert!(matches!(
        issuer().issue(foreign),
        Err(CoreMethodInstanceTargetRejectV1::ManifestBrandMismatch)
    ));

    static ARRAY_ROW: CoreMethodContractResultRowV1 = CoreMethodContractResultRowV1 {
        receiver_box: "ArrayBox",
        canonical: "length",
        aliases: &[],
        arities: &[0],
        op: CoreMethodOp::StringLen,
        result_kind: CoreMethodResultKindV1::I64Value,
        effect: CoreMethodEffectV1::PureRead,
        lowering_tier: CoreMethodLoweringTier::WarmDirectAbi,
    };
    let wrong_receiver = issue_core_method_manifest_test_row_ref(&ARRAY_ROW, 0, false);
    assert!(matches!(
        issuer().issue(wrong_receiver),
        Err(CoreMethodInstanceTargetRejectV1::ReceiverMismatch)
    ));
}

#[test]
fn target_issuer_rejects_design_only_row_before_home_effects() {
    static DESIGN_ONLY_ROW: CoreMethodContractResultRowV1 = CoreMethodContractResultRowV1 {
        receiver_box: "StringBox",
        canonical: "design_only",
        aliases: &[],
        arities: &[0],
        op: CoreMethodOp::StringLen,
        result_kind: CoreMethodResultKindV1::I64Value,
        effect: CoreMethodEffectV1::PureRead,
        lowering_tier: CoreMethodLoweringTier::DesignOnly,
    };
    let design_only = issue_core_method_manifest_test_row_ref(&DESIGN_ONLY_ROW, 0, false);
    assert!(matches!(
        issuer().issue(design_only),
        Err(CoreMethodInstanceTargetRejectV1::DesignOnlyRow)
    ));
}

#[test]
fn target_issuer_rejects_generated_string_equals_row_before_home_effects() {
    let row = issue_core_method_manifest_row_ref_v1(CoreMethodOp::StringEquals, 1)
        .expect("generated StringEquals/1 row");
    assert!(matches!(
        issuer().issue(row),
        Err(CoreMethodInstanceTargetRejectV1::DesignOnlyRow)
    ));
}

#[test]
fn target_issuer_rejects_wrong_effect_and_result() {
    static MUTATING_ROW: CoreMethodContractResultRowV1 = CoreMethodContractResultRowV1 {
        receiver_box: "StringBox",
        canonical: "length",
        aliases: &[],
        arities: &[0],
        op: CoreMethodOp::StringLen,
        result_kind: CoreMethodResultKindV1::I64Value,
        effect: CoreMethodEffectV1::MutatesSlot,
        lowering_tier: CoreMethodLoweringTier::WarmDirectAbi,
    };
    let wrong_effect = issue_core_method_manifest_test_row_ref(&MUTATING_ROW, 0, false);
    assert!(matches!(
        issuer().issue(wrong_effect),
        Err(CoreMethodInstanceTargetRejectV1::EffectMismatch)
    ));

    static STRING_RESULT_ROW: CoreMethodContractResultRowV1 = CoreMethodContractResultRowV1 {
        receiver_box: "StringBox",
        canonical: "length",
        aliases: &[],
        arities: &[0],
        op: CoreMethodOp::StringLen,
        result_kind: CoreMethodResultKindV1::StringValue,
        effect: CoreMethodEffectV1::PureRead,
        lowering_tier: CoreMethodLoweringTier::WarmDirectAbi,
    };
    let wrong_result = issue_core_method_manifest_test_row_ref(&STRING_RESULT_ROW, 0, false);
    assert!(matches!(
        issuer().issue(wrong_result),
        Err(CoreMethodInstanceTargetRejectV1::ResultMismatch)
    ));
}
