use super::*;
use crate::ast::ASTNode;
use super::super::{
    FunctionSemanticResolverSessionV1, FunctionSyntaxViewV1,
    ResolverCoreMethodCallableContractIssuerV1, ResolverCoreMethodCallableContractRejectV1,
};
use crate::mir::core_method_op::{CoreMethodLoweringTier, CoreMethodOp};
use crate::mir::core_method_result_kind::{
    issue_core_method_manifest_row_ref_for_test, issue_core_method_manifest_row_ref_v2,
    issue_core_method_manifest_test_row_ref, CoreMethodContractRowV2, CoreMethodEffectV1,
    CoreMethodManifestRowRefV2, CoreMethodResultKindV1, CoreMethodSemanticLawV2,
    CORE_METHOD_MANIFEST_BRAND_V2,
};

fn issuer() -> CoreMethodInstanceTargetIssuerV1 {
    CoreMethodInstanceTargetIssuerV1::string_box_text(CORE_METHOD_MANIFEST_BRAND_V2)
        .expect("generated manifest brand should issue")
}

#[test]
fn string_len_issues_exact_typed_home_target() {
    let row = issue_core_method_manifest_row_ref_v2(CoreMethodOp::StringLen, 0)
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
    assert_eq!(target.checked_code_point_law_v1(), Some(CoreMethodSemanticLawV2::CodePointCount));
    let keys = CoreMethodInstanceTargetIssuerV1::map_text_keys(CORE_METHOD_MANIFEST_BRAND_V2).unwrap()
        .issue(issue_core_method_manifest_row_ref_v2(CoreMethodOp::MapKeys, 0).unwrap()).unwrap();
    assert!(keys.checked_text_keys_law_v1());
    assert_eq!(keys.result(), CoreMethodHomeResultRelationV1::TextArrayToCaller);
    assert_eq!(keys.expression_result_class_v1(), None, "snapshot lends no primitive or nominal class");
    let count = CoreMethodInstanceTargetIssuerV1::array_count(CORE_METHOD_MANIFEST_BRAND_V2).unwrap()
        .issue(issue_core_method_manifest_row_ref_v2(CoreMethodOp::ArrayLen, 0).unwrap()).unwrap();
    assert!(count.checked_array_count_law_v1());
    assert_eq!(count.result(), CoreMethodHomeResultRelationV1::I64ToCaller);
}

#[test]
fn string_substring_specializes_union_row_to_exact_arity_two() {
    let row = issue_core_method_manifest_row_ref_v2(CoreMethodOp::StringSubstring, 2)
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
    assert_eq!(target.checked_code_point_law_v1(), Some(CoreMethodSemanticLawV2::CodePointHalfOpenClamped));
    let index_row = issue_core_method_manifest_row_ref_v2(CoreMethodOp::StringIndexOf, 1).unwrap();
    let mut index_target = issuer.issue(index_row).unwrap();
    assert_eq!(index_target.checked_code_point_law_v1(), Some(CoreMethodSemanticLawV2::CodePointSearchFromZero));
    // An ABI family is not enough: a different arity or missing law cannot
    // authorize the checked one-argument CP operation.
    index_target.row = issue_core_method_manifest_row_ref_v2(CoreMethodOp::StringIndexOf, 2).unwrap();
    assert_eq!(index_target.checked_code_point_law_v1(), None);
    static UNPROJECTED_SEARCH: CoreMethodContractRowV2 = CoreMethodContractRowV2 {
        receiver_box: "StringBox", canonical: "indexOf", aliases: &[], arities: &[1],
        semantic_law: &[(1, CoreMethodSemanticLawV2::Unprojected)], op: CoreMethodOp::StringIndexOf,
        result_kind: CoreMethodResultKindV1::I64Value, effect: CoreMethodEffectV1::PureRead,
        lowering_tier: CoreMethodLoweringTier::WarmDirectAbi,
    };
    let unprojected = CoreMethodInstanceTargetIssuerV1::string_box_text(CORE_METHOD_MANIFEST_BRAND_V2).unwrap()
        .issue(issue_core_method_manifest_test_row_ref(&UNPROJECTED_SEARCH, 1, false)).unwrap();
    assert_eq!(unprojected.abi_profile(), CoreMethodHomeAbiProfileV1::StringBoxTextV1);
    assert_eq!(unprojected.checked_code_point_law_v1(), None);

}

#[test]
fn target_issuer_rejects_union_arity_one_and_duplicate_target() {
    let row = issue_core_method_manifest_row_ref_v2(CoreMethodOp::StringSubstring, 1)
        .expect("generated union row should still be observable");
    let mut issuer = issuer();
    assert!(matches!(
        issuer.issue(row),
        Err(CoreMethodInstanceTargetRejectV1::UnsupportedOperation {
            op: CoreMethodOp::StringSubstring,
            arity: 1,
        })
    ));

    let row = issue_core_method_manifest_row_ref_v2(CoreMethodOp::StringLen, 0)
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

    static ARRAY_ROW: CoreMethodContractRowV2 = CoreMethodContractRowV2 {
        receiver_box: "ArrayBox",
        canonical: "length",
        aliases: &[],
        arities: &[0],
        semantic_law: &[(0, CoreMethodSemanticLawV2::Unprojected)],
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
    static DESIGN_ONLY_ROW: CoreMethodContractRowV2 = CoreMethodContractRowV2 {
        receiver_box: "StringBox",
        canonical: "design_only",
        aliases: &[],
        arities: &[0],
        semantic_law: &[(0, CoreMethodSemanticLawV2::Unprojected)],
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
    let row = issue_core_method_manifest_row_ref_v2(CoreMethodOp::StringEquals, 1)
        .expect("generated StringEquals/1 row");
    assert!(matches!(
        issuer().issue(row),
        Err(CoreMethodInstanceTargetRejectV1::DesignOnlyRow)
    ));
}

#[test]
fn target_issuer_rejects_wrong_effect_and_result() {
    static MUTATING_ROW: CoreMethodContractRowV2 = CoreMethodContractRowV2 {
        receiver_box: "StringBox",
        canonical: "length",
        aliases: &[],
        arities: &[0],
        semantic_law: &[(0, CoreMethodSemanticLawV2::Unprojected)],
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

    static STRING_RESULT_ROW: CoreMethodContractRowV2 = CoreMethodContractRowV2 {
        receiver_box: "StringBox",
        canonical: "length",
        aliases: &[],
        arities: &[0],
        semantic_law: &[(0, CoreMethodSemanticLawV2::Unprojected)],
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

use CoreMethodHomeParameterRelationV1 as Param;
use CoreMethodHomeResultRelationV1 as ResultRelation;

/// (op, arity, selector, schema, parameters, result, abi, law, result kind)
type MapContract = (
    CoreMethodOp,
    u32,
    &'static str,
    CoreMethodHomeSchemaV1,
    &'static [Param],
    ResultRelation,
    CoreMethodHomeAbiProfileV1,
    CoreMethodSemanticLawV2,
    CoreMethodResultKindV1,
);

const MAP_CONTRACTS: [MapContract; 4] = [
    (
        CoreMethodOp::MapGet,
        1,
        "get",
        CoreMethodHomeSchemaV1::MapTextGet,
        &[Param::TextParameter],
        ResultRelation::DynamicToCaller,
        CoreMethodHomeAbiProfileV1::MapTextGetV1,
        CoreMethodSemanticLawV2::MapTextGetPresentOrFault,
        CoreMethodResultKindV1::Dynamic,
    ),
    (
        CoreMethodOp::MapSet,
        2,
        "set",
        CoreMethodHomeSchemaV1::MapTextSet,
        &[Param::TextParameter, Param::ValueRetainedByReceiver],
        ResultRelation::DynamicToCaller,
        CoreMethodHomeAbiProfileV1::MapTextSetV1,
        CoreMethodSemanticLawV2::MapTextSetReplaceOrInsert,
        CoreMethodResultKindV1::Dynamic,
    ),
    (
        CoreMethodOp::MapDelete,
        1,
        "delete",
        CoreMethodHomeSchemaV1::MapTextDelete,
        &[Param::TextParameter],
        ResultRelation::BoolToCaller,
        CoreMethodHomeAbiProfileV1::MapTextDeleteV1,
        CoreMethodSemanticLawV2::MapTextDeleteReportsPresence,
        CoreMethodResultKindV1::BoolValue,
    ),
    (
        CoreMethodOp::MapLen,
        0,
        "size",
        CoreMethodHomeSchemaV1::MapEntryCount,
        &[],
        ResultRelation::I64ToCaller,
        CoreMethodHomeAbiProfileV1::MapEntryCountV1,
        CoreMethodSemanticLawV2::MapEntryCount,
        CoreMethodResultKindV1::I64Value,
    ),
];

fn map_issuer(schema: CoreMethodHomeSchemaV1) -> CoreMethodInstanceTargetIssuerV1 {
    let brand = CORE_METHOD_MANIFEST_BRAND_V2;
    match schema {
        CoreMethodHomeSchemaV1::MapTextGet => CoreMethodInstanceTargetIssuerV1::map_text_get(brand),
        CoreMethodHomeSchemaV1::MapTextSet => CoreMethodInstanceTargetIssuerV1::map_text_set(brand),
        CoreMethodHomeSchemaV1::MapTextDelete => {
            CoreMethodInstanceTargetIssuerV1::map_text_delete(brand)
        }
        _ => CoreMethodInstanceTargetIssuerV1::map_entry_count(brand),
    }
    .expect("generated manifest brand should issue")
}

#[test]
fn map_contract_rows_issue_their_exact_targets_and_laws() {
    for (op, arity, _, schema, params, result, abi, law, _) in MAP_CONTRACTS {
        let row = issue_core_method_manifest_row_ref_v2(op, arity).expect("generated Map row");
        assert_eq!(row.semantic_law_for_arity(), Some(law), "{op:?}");
        let target = map_issuer(schema).issue(row).expect("Map contract target");
        assert_eq!(target.schema(), schema);
        assert_eq!(
            target.receiver(),
            CoreMethodHomeReceiverRelationV1::MapReceiver
        );
        assert_eq!(target.parameters(), params);
        assert_eq!(target.result(), result);
        assert_eq!(target.abi_profile(), abi);
    }
    // Delete reports presence as a Bool, per the language reference.
    let delete = issue_core_method_manifest_row_ref_v2(CoreMethodOp::MapDelete, 1).unwrap();
    assert_eq!(delete.row().result_kind, CoreMethodResultKindV1::BoolValue);
    // Another Map operation's row is not issuable under a different schema.
    let get = issue_core_method_manifest_row_ref_v2(CoreMethodOp::MapGet, 1).unwrap();
    assert!(matches!(
        map_issuer(CoreMethodHomeSchemaV1::MapEntryCount).issue(get),
        Err(CoreMethodInstanceTargetRejectV1::UnsupportedOperation { .. })
    ));
}

#[test]
fn map_contract_issuer_rejects_a_row_without_the_exact_law_or_result() {
    macro_rules! forged {
        ($op:expr, $name:literal, $arity:literal, $law:expr, $kind:expr, $effect:expr) => {{
            static ROW: CoreMethodContractRowV2 = CoreMethodContractRowV2 {
                receiver_box: "MapBox",
                canonical: $name,
                aliases: &[],
                arities: &[$arity],
                semantic_law: &[($arity, $law)],
                op: $op,
                result_kind: $kind,
                effect: $effect,
                lowering_tier: CoreMethodLoweringTier::ColdFallback,
            };
            issue_core_method_manifest_test_row_ref(&ROW, $arity, false)
        }};
    }
    use CoreMethodSemanticLawV2 as Law;
    let cases = [
        (
            CoreMethodHomeSchemaV1::MapTextGet,
            forged!(
                CoreMethodOp::MapGet,
                "get",
                1,
                Law::Unprojected,
                CoreMethodResultKindV1::Dynamic,
                CoreMethodEffectV1::PureRead
            ),
        ),
        (
            CoreMethodHomeSchemaV1::MapTextSet,
            forged!(
                CoreMethodOp::MapSet,
                "set",
                2,
                Law::MapKeyPresence,
                CoreMethodResultKindV1::Dynamic,
                CoreMethodEffectV1::MutatesSlot
            ),
        ),
        (
            CoreMethodHomeSchemaV1::MapTextDelete,
            forged!(
                CoreMethodOp::MapDelete,
                "delete",
                1,
                Law::MapTextDeleteReportsPresence,
                CoreMethodResultKindV1::Dynamic,
                CoreMethodEffectV1::MutatesShape
            ),
        ),
        (
            CoreMethodHomeSchemaV1::MapEntryCount,
            forged!(
                CoreMethodOp::MapLen,
                "size",
                0,
                Law::MapEntryCount,
                CoreMethodResultKindV1::BoolValue,
                CoreMethodEffectV1::PureRead
            ),
        ),
    ];
    for (schema, row) in cases {
        assert!(matches!(
            map_issuer(schema).issue(row),
            Err(CoreMethodInstanceTargetRejectV1::ResultMismatch)
        ));
    }
}

fn map_call_tree(selector: &str, arguments: Vec<ASTNode>) -> ASTNode {
    use crate::ast::{DeclarationAttrs, LiteralValue, Span};
    let integer = |value| ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: Span::unknown(),
    };
    ASTNode::FunctionDeclaration {
        name: "map_contract_fixture".into(),
        params: Vec::new(),
        param_decls: Vec::new(),
        return_type_name: None,
        body: vec![
            ASTNode::Local {
                variables: vec!["m".into()],
                initial_values: vec![Some(Box::new(integer(7)))],
                declared_type_names: vec![None],
                span: Span::unknown(),
            },
            ASTNode::Return {
                value: Some(Box::new(ASTNode::MethodCall {
                    object: Box::new(ASTNode::Variable {
                        name: "m".into(),
                        span: Span::unknown(),
                    }),
                    method: selector.into(),
                    arguments,
                    span: Span::unknown(),
                })),
                span: Span::unknown(),
            },
        ],
        uses: Vec::new(),
        contracts: Vec::new(),
        is_static: true,
        is_override: false,
        attrs: DeclarationAttrs::default(),
        span: Span::unknown(),
    }
}

/// The same operation under a row that carries no Map contract law.
fn unprojected_map_row(op: CoreMethodOp) -> CoreMethodManifestRowRefV2 {
    macro_rules! row {
        ($op:expr, $name:literal, $arity:literal, $kind:expr, $effect:expr) => {{
            static ROW: CoreMethodContractRowV2 = CoreMethodContractRowV2 {
                receiver_box: "MapBox",
                canonical: $name,
                aliases: &[],
                arities: &[$arity],
                semantic_law: &[($arity, CoreMethodSemanticLawV2::Unprojected)],
                op: $op,
                result_kind: $kind,
                effect: $effect,
                lowering_tier: CoreMethodLoweringTier::ColdFallback,
            };
            issue_core_method_manifest_test_row_ref(&ROW, $arity, false)
        }};
    }
    match op {
        CoreMethodOp::MapGet => row!(CoreMethodOp::MapGet,
            "get",
            1,
            CoreMethodResultKindV1::Dynamic,
            CoreMethodEffectV1::PureRead
        ),
        CoreMethodOp::MapSet => row!(CoreMethodOp::MapSet,
            "set",
            2,
            CoreMethodResultKindV1::Dynamic,
            CoreMethodEffectV1::MutatesSlot
        ),
        CoreMethodOp::MapDelete => row!(CoreMethodOp::MapDelete,
            "delete",
            1,
            CoreMethodResultKindV1::BoolValue,
            CoreMethodEffectV1::MutatesShape
        ),
        _ => row!(
            CoreMethodOp::MapLen,
            "size",
            0,
            CoreMethodResultKindV1::I64Value,
            CoreMethodEffectV1::PureRead
        ),
    }
}

/// The resolver-side verifier reads the generated row itself. A target whose
/// recorded row, parameters or ABI drift from the Map contract is refused,
/// even though the issuer-side checks are never consulted again.
#[test]
fn map_contract_verifier_independently_rejects_drifted_targets() {
    use crate::ast::{LiteralValue, Span};
    let key = || ASTNode::Literal {
        value: LiteralValue::String("k".into()),
        span: Span::unknown(),
    };
    let value = || ASTNode::Literal {
        value: LiteralValue::Integer(5),
        span: Span::unknown(),
    };
    for (op, arity, selector, schema, _, _, _, _, _) in MAP_CONTRACTS {
        let arguments = match arity {
            0 => vec![],
            1 => vec![key()],
            _ => vec![key(), value()],
        };
        let tree = map_call_tree(selector, arguments);
        let mut session = FunctionSemanticResolverSessionV1::new(0).unwrap();
        let forest = session
            .resolve_forest(FunctionSyntaxViewV1::from_ast(&tree).unwrap())
            .unwrap();
        let view = forest.callable_source_ledger(forest.roots()[0]).unwrap();
        let (_, call) = view.method_calls().next().expect("one method call");
        let row = issue_core_method_manifest_row_ref_v2(op, arity).unwrap();
        let issue = |mutate: fn(&mut VerifiedCoreMethodInstanceTargetV1)| {
            let mut target = map_issuer(schema).issue(row).unwrap();
            mutate(&mut target);
            ResolverCoreMethodCallableContractIssuerV1::issue_ordinary_source(&view, call, target)
                .map(|_| ())
        };
        assert_eq!(issue(|_| {}), Ok(()), "{op:?}");
        assert_eq!(
            issue(|target| target.parameters = Box::new([Param::I64Parameter])),
            Err(ResolverCoreMethodCallableContractRejectV1::TargetParameterMismatch),
            "{op:?} parameters"
        );
        assert!(
            matches!(
                issue(|target| target.row = unprojected_map_row(target.row.row().op)),
                Err(ResolverCoreMethodCallableContractRejectV1::TargetOperationMismatch { .. })
            ),
            "{op:?} law"
        );
        assert_eq!(
            issue(|target| target.abi_profile = CoreMethodHomeAbiProfileV1::MapTextKeysV1),
            Err(ResolverCoreMethodCallableContractRejectV1::TargetSchemaMismatch),
            "{op:?} abi"
        );
    }
}
