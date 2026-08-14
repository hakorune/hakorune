use crate::mir::resolved_semantics::FunctionSemanticResolverSessionV1;
use crate::parser::{NyashParser, ParserBuildConfig};

use super::{
    issue_resolved_callable_semantic_batch_v1, issue_s6c_typed_input_relation_v1, S6CBinaryRoleV1,
    S6CLogicalValueClassV1, S6CTypedInputRelationRejectV1, S6CTypedInputRoleV1,
    VerifiedResolvedCallableSemanticBatchV1,
};

fn batch(source: &str) -> VerifiedResolvedCallableSemanticBatchV1 {
    let parsed = NyashParser::parse_normal_callable_program_with_build_config(
        source,
        ParserBuildConfig::default(),
    )
    .expect("normal callable source");
    let source = crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        let transformed = crate::r#macro::transform_normal_callable_program_v1(parsed)
            .expect("exact callable transform");
        let crate::r#macro::NormalCallableTransformOutcomeV1::SourceBacked(source) = transformed
        else {
            panic!("fixture must remain source-backed")
        };
        source
    });
    let mut resolver = FunctionSemanticResolverSessionV1::new(73).unwrap();
    issue_resolved_callable_semantic_batch_v1(&mut resolver, source).unwrap()
}

fn issue_first(
    source: &str,
) -> Result<super::VerifiedS6CTypedInputRelationV1, S6CTypedInputRelationRejectV1> {
    let batch = batch(source);
    batch
        .with_declaration_semantics(|view| {
            let row = &view.declarations()[0];
            let loop_site = row.function().only_loop_site().expect("one Loop source");
            issue_s6c_typed_input_relation_v1(row, &loop_site)
        })
        .expect("resolved declaration semantics")
}

#[test]
fn explicit_stringbox_and_i64_source_seal_one_typed_input_frame() {
    let relation = issue_first(include_str!(
        "../../../apps/tests/scan_with_init_typed_ok_min.hako"
    ))
    .expect("explicit S6C typed input");

    assert_eq!(
        relation
            .inputs()
            .iter()
            .map(|input| (input.role(), input.class()))
            .collect::<Vec<_>>(),
        [
            (S6CTypedInputRoleV1::Subject, S6CLogicalValueClassV1::Text),
            (S6CTypedInputRoleV1::Needle, S6CLogicalValueClassV1::Text),
            (S6CTypedInputRoleV1::Index, S6CLogicalValueClassV1::I64),
        ]
    );
    assert_eq!(relation.initializer().declared_type_name(), Some("i64"));
    assert_eq!(
        relation
            .binaries()
            .iter()
            .map(|binary| (binary.role(), binary.placement(), binary.result_class()))
            .collect::<Vec<_>>(),
        [
            (
                S6CBinaryRoleV1::LoopConditionLess,
                crate::mir::resolved_semantics::ResolvedLoopPlacementV1::Condition,
                S6CLogicalValueClassV1::Bool,
            ),
            (
                S6CBinaryRoleV1::SliceEndAdd,
                crate::mir::resolved_semantics::ResolvedLoopPlacementV1::Body,
                S6CLogicalValueClassV1::I64,
            ),
            (
                S6CBinaryRoleV1::TextEqual,
                crate::mir::resolved_semantics::ResolvedLoopPlacementV1::Body,
                S6CLogicalValueClassV1::Bool,
            ),
            (
                S6CBinaryRoleV1::StepAdd,
                crate::mir::resolved_semantics::ResolvedLoopPlacementV1::Body,
                S6CLogicalValueClassV1::I64,
            ),
        ]
    );

    let less = relation
        .binaries()
        .iter()
        .find(|binary| binary.role() == S6CBinaryRoleV1::LoopConditionLess)
        .expect("condition Less relation");
    let equal = relation
        .binaries()
        .iter()
        .find(|binary| binary.role() == S6CBinaryRoleV1::TextEqual)
        .expect("body TextEqual relation");
    relation.with_call_sites(|calls| {
        assert_eq!(calls.length_site(), less.source().rhs());
        assert_eq!(
            calls.length_placement(),
            crate::mir::resolved_semantics::ResolvedLoopPlacementV1::Condition
        );
        assert_eq!(calls.substring_site(), equal.source().lhs());
        assert_eq!(
            calls.substring_placement(),
            crate::mir::resolved_semantics::ResolvedLoopPlacementV1::Body
        );
    });
}

#[test]
fn unannotated_fixture_remains_missing_type_evidence() {
    assert!(matches!(
        issue_first(include_str!(
            "../../../apps/tests/scan_with_init_ok_min.hako"
        )),
        Err(S6CTypedInputRelationRejectV1::MissingTypeEvidence(
            S6CTypedInputRoleV1::Subject
        ))
    ));
}

#[test]
fn wrong_local_type_rejects_before_source_bound_relation() {
    let source = include_str!("../../../apps/tests/scan_with_init_typed_ok_min.hako")
        .replace("local i: i64 = 0", "local i: f64 = 0");
    assert!(matches!(
        issue_first(&source),
        Err(S6CTypedInputRelationRejectV1::WrongInitializerType(Some(actual)))
            if actual.as_ref() == "f64"
    ));
}

#[test]
fn missing_local_type_rejects_before_source_bound_relation() {
    let source = include_str!("../../../apps/tests/scan_with_init_typed_ok_min.hako")
        .replace("local i: i64 = 0", "local i = 0");
    assert_eq!(
        issue_first(&source).unwrap_err(),
        S6CTypedInputRelationRejectV1::WrongInitializerType(None)
    );
}

#[test]
fn wrong_initializer_literal_rejects_before_source_bound_relation() {
    let source = include_str!("../../../apps/tests/scan_with_init_typed_ok_min.hako")
        .replace("local i: i64 = 0", "local i: i64 = 1");
    assert_eq!(
        issue_first(&source).unwrap_err(),
        S6CTypedInputRelationRejectV1::WrongInitializerLiteral
    );
}

#[test]
fn wrong_condition_operator_rejects_exact_binary_role() {
    let source = include_str!("../../../apps/tests/scan_with_init_typed_ok_min.hako")
        .replace("i < s.length()", "i <= s.length()");
    assert_eq!(
        issue_first(&source).unwrap_err(),
        S6CTypedInputRelationRejectV1::BinaryRoleCoverage(S6CBinaryRoleV1::LoopConditionLess)
    );
}

#[test]
fn swapped_subject_receiver_rejects_exact_call_shape() {
    let source = include_str!("../../../apps/tests/scan_with_init_typed_ok_min.hako")
        .replace("s.substring(i, i + 1)", "ch.substring(i, i + 1)");
    assert_eq!(
        issue_first(&source).unwrap_err(),
        S6CTypedInputRelationRejectV1::MethodCallShape("substring")
    );
}
