use crate::ast::{ASTNode, BinaryOperator, DeclarationAttrs, LiteralValue, Span};
use crate::mir::compiler::{
    capability::CanonicalLoweringPreflightV1, VerifiedResolvedSourceUnitV1,
};
use crate::mir::if_recipe_contract::{IfRecipeNormalizerV1, IfSourcePathStepV1};
use crate::mir::resolved_control_flow::if_control::verify_resolved_function_if_control_v1;
use crate::mir::resolved_control_flow::verify_function_completion_v1;
use crate::mir::resolved_semantics::SourcePathV1;

use super::coverage::TrivialProfileDraftV1;
use super::error::{TrivialProfileContractErrorV1, TrivialProfileStopReasonV1};
use super::product::{TrivialRepresentationV1, TrivialTerminalProfileV1};
use super::{analyze_trivial_canonical_owner_v1, TrivialCanonicalOwnerAnalysisV1};

fn literal(value: LiteralValue) -> ASTNode {
    ASTNode::Literal {
        value,
        span: Span::unknown(),
    }
}

fn int(value: i64) -> ASTNode {
    literal(LiteralValue::Integer(value))
}

fn bool_(value: bool) -> ASTNode {
    literal(LiteralValue::Bool(value))
}

fn float(value: f64) -> ASTNode {
    literal(LiteralValue::Float(value))
}

fn null() -> ASTNode {
    literal(LiteralValue::Null)
}

fn void() -> ASTNode {
    literal(LiteralValue::Void)
}

fn variable(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.into(),
        span: Span::unknown(),
    }
}

fn binary(operator: BinaryOperator, left: ASTNode, right: ASTNode) -> ASTNode {
    ASTNode::BinaryOp {
        operator,
        left: Box::new(left),
        right: Box::new(right),
        span: Span::unknown(),
    }
}

fn local(name: &str, value: Option<ASTNode>) -> ASTNode {
    ASTNode::Local {
        variables: vec![name.into()],
        initial_values: vec![value.map(Box::new)],
        declared_type_names: vec![None],
        span: Span::unknown(),
    }
}

fn assignment(name: &str, value: ASTNode) -> ASTNode {
    ASTNode::Assignment {
        target: Box::new(variable(name)),
        value: Box::new(value),
        span: Span::unknown(),
    }
}

fn if_(condition: ASTNode, then_body: Vec<ASTNode>, else_body: Option<Vec<ASTNode>>) -> ASTNode {
    ASTNode::If {
        condition: Box::new(condition),
        then_body,
        else_body,
        span: Span::unknown(),
    }
}

fn return_(value: Option<ASTNode>) -> ASTNode {
    ASTNode::Return {
        value: value.map(Box::new),
        span: Span::unknown(),
    }
}

fn block_expr(prelude: Vec<ASTNode>, tail: ASTNode) -> ASTNode {
    ASTNode::BlockExpr {
        prelude_stmts: prelude,
        tail_expr: Box::new(tail),
        span: Span::unknown(),
    }
}

fn function_with_params(params: Vec<String>, body: Vec<ASTNode>) -> ASTNode {
    ASTNode::FunctionDeclaration {
        name: "trivial_profile_fixture".into(),
        params,
        param_decls: Vec::new(),
        return_type_name: None,
        body,
        uses: Vec::new(),
        contracts: Vec::new(),
        is_static: true,
        is_override: false,
        attrs: DeclarationAttrs::default(),
        span: Span::unknown(),
    }
}

fn function(body: Vec<ASTNode>) -> ASTNode {
    function_with_params(Vec::new(), body)
}

fn analyze(ast: ASTNode) -> TrivialCanonicalOwnerAnalysisV1 {
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(ast).unwrap();
    let input = unit.root_function_input().unwrap();
    let completion = verify_function_completion_v1(input).unwrap();
    let if_control = verify_resolved_function_if_control_v1(input, &completion).unwrap();
    analyze_trivial_canonical_owner_v1(input, &completion, &if_control).unwrap()
}

fn admitted(ast: ASTNode) -> super::product::VerifiedTrivialCanonicalOwnerV1 {
    let TrivialCanonicalOwnerAnalysisV1::Admitted(product) = analyze(ast) else {
        panic!("expected admitted trivial owner")
    };
    product
}

fn assert_stop(ast: ASTNode, expected: TrivialProfileStopReasonV1) {
    let TrivialCanonicalOwnerAnalysisV1::NotAdmitted(stop) = analyze(ast) else {
        panic!("expected typed trivial-profile stop")
    };
    assert_eq!(stop.reason(), expected);
}

#[test]
fn exact_literals_binary_and_value_return_seal() {
    let product = admitted(function(vec![
        local("i", Some(int(1))),
        local("b", Some(bool_(true))),
        local("f", Some(float(1.5))),
        return_(Some(binary(BinaryOperator::Add, variable("i"), int(2)))),
    ]));

    assert!(product
        .values()
        .iter()
        .any(|row| row.representation() == TrivialRepresentationV1::InlineI64));
    assert!(product
        .values()
        .iter()
        .any(|row| row.representation() == TrivialRepresentationV1::InlineBool));
    assert!(product
        .values()
        .iter()
        .any(|row| row.representation() == TrivialRepresentationV1::InlineF64));
    assert!(matches!(
        product.terminal(),
        TrivialTerminalProfileV1::ExplicitValue {
            representation: TrivialRepresentationV1::InlineI64,
            ..
        }
    ));
}

#[test]
fn local_assignment_and_blockexpr_tail_preserve_exact_profile() {
    let product = admitted(function(vec![
        local("x", Some(int(1))),
        assignment("x", int(2)),
        return_(Some(block_expr(
            vec![local("y", Some(int(3)))],
            binary(BinaryOperator::Add, variable("x"), variable("y")),
        ))),
    ]));

    assert_eq!(product.definitions().len(), 3);
    assert!(product
        .definitions()
        .iter()
        .all(|row| row.representation() == TrivialRepresentationV1::InlineI64));
    assert!(!product.coverage().ordered_subjects().is_empty());
}

#[test]
fn homogeneous_if_merge_seals_and_mixed_merge_rejects() {
    let product = admitted(function(vec![
        local("x", Some(int(0))),
        if_(
            bool_(true),
            vec![assignment("x", int(1))],
            Some(vec![assignment("x", int(2))]),
        ),
        return_(Some(variable("x"))),
    ]));
    assert_eq!(product.merge_profiles().len(), 1);
    assert_eq!(
        product.merge_profiles()[0].representation(),
        TrivialRepresentationV1::InlineI64
    );

    assert_stop(
        function(vec![
            local("x", Some(int(0))),
            if_(
                bool_(true),
                vec![assignment("x", bool_(false))],
                Some(vec![assignment("x", int(2))]),
            ),
            return_(Some(variable("x"))),
        ]),
        TrivialProfileStopReasonV1::IfMergeProfileNotHomogeneous,
    );
}

#[test]
fn same_pass_if_recipe_facts_capture_explicit_else_shape() {
    let product = admitted(function(vec![
        local("x", Some(int(0))),
        if_(
            binary(BinaryOperator::Less, variable("x"), int(1)),
            vec![assignment("x", int(1))],
            Some(vec![assignment("x", int(2))]),
        ),
        return_(Some(variable("x"))),
    ]));

    let facts = product
        .recipe_facts()
        .expect("selected explicit-else shape must emit same-pass facts");
    assert!(facts.has_explicit_else());
    assert_eq!(facts.then_assignment_count(), 1);
    assert_eq!(facts.else_assignment_count(), 1);
    assert!(facts.continuation_read().is_some());
    assert_eq!(
        facts
            .entry_witness()
            .expect("pre-If entry witness")
            .representation(),
        TrivialRepresentationV1::InlineI64
    );
    assert!(facts.expression_count() >= 7);
}

#[test]
fn same_pass_if_recipe_facts_decline_implicit_else_shape() {
    let product = admitted(function(vec![
        local("x", Some(int(0))),
        if_(
            binary(BinaryOperator::Less, variable("x"), int(1)),
            vec![assignment("x", int(1))],
            None,
        ),
        return_(Some(variable("x"))),
    ]));

    assert!(product.recipe_facts().is_none());
}

#[test]
fn same_pass_if_recipe_maps_to_verified_portable_artifact() {
    let root = function(vec![
        local("x", Some(int(0))),
        if_(
            binary(BinaryOperator::Less, variable("x"), int(1)),
            vec![assignment("x", int(1))],
            Some(vec![assignment("x", int(2))]),
        ),
        return_(Some(variable("x"))),
    ]);
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(root).unwrap();
    let input = unit.root_function_input().unwrap();
    let completion = verify_function_completion_v1(input).unwrap();
    let if_control = verify_resolved_function_if_control_v1(input, &completion).unwrap();
    let product = match analyze_trivial_canonical_owner_v1(input, &completion, &if_control).unwrap()
    {
        TrivialCanonicalOwnerAnalysisV1::Admitted(product) => product,
        TrivialCanonicalOwnerAnalysisV1::NotAdmitted(_) => panic!("expected admitted profile"),
    };
    let verified = super::map_trivial_if_recipe_v1(&product, input.function())
        .expect("same-pass facts map to portable artifact");
    let semantic = IfRecipeNormalizerV1::normalize_semantic(verified.recipe())
        .expect("semantic normalization");
    assert!(semantic.contains("explicit"));
    assert_eq!(verified.recipe().as_recipe().joins.len(), 1);
    assert!(!semantic.contains("compilation_unit_ordinal"));
    assert_eq!(
        verified.source_binding().as_source_binding().claims[2]
            .path
            .steps,
        vec![
            IfSourcePathStepV1::BodyItem { index: 1 },
            IfSourcePathStepV1::IfThenItem { index: 0 },
        ]
    );
}

#[test]
fn null_sentinel_flows_locally_and_compares_to_bool() {
    let product = admitted(function(vec![
        local("x", Some(null())),
        if_(
            bool_(true),
            vec![assignment("x", null())],
            Some(vec![assignment("x", null())]),
        ),
        local(
            "same",
            Some(binary(BinaryOperator::Equal, variable("x"), null())),
        ),
        return_(Some(variable("same"))),
    ]));

    assert!(product
        .values()
        .iter()
        .any(|row| row.representation() == TrivialRepresentationV1::NullSentinel));
    assert!(product
        .definitions()
        .iter()
        .any(|row| row.representation() == TrivialRepresentationV1::NullSentinel));
    assert_eq!(
        product.merge_profiles()[0].representation(),
        TrivialRepresentationV1::NullSentinel
    );
    assert!(matches!(
        product.terminal(),
        TrivialTerminalProfileV1::ExplicitValue {
            representation: TrivialRepresentationV1::InlineBool,
            ..
        }
    ));
}

#[test]
fn explicit_void_value_flows_and_terminal_stays_distinct() {
    let product = admitted(function(vec![
        local("x", Some(void())),
        if_(
            bool_(true),
            vec![assignment("x", void())],
            Some(vec![assignment("x", void())]),
        ),
        return_(Some(variable("x"))),
    ]));

    assert!(product
        .values()
        .iter()
        .any(|row| { row.representation() == TrivialRepresentationV1::ExplicitVoidValue }));
    assert!(product
        .definitions()
        .iter()
        .any(|row| { row.representation() == TrivialRepresentationV1::ExplicitVoidValue }));
    assert_eq!(
        product.merge_profiles()[0].representation(),
        TrivialRepresentationV1::ExplicitVoidValue
    );
    assert!(matches!(
        product.terminal(),
        TrivialTerminalProfileV1::ExplicitValue {
            representation: TrivialRepresentationV1::ExplicitVoidValue,
            ..
        }
    ));

    let comparison = admitted(function(vec![return_(Some(binary(
        BinaryOperator::NotEqual,
        void(),
        void(),
    )))]));
    assert!(matches!(
        comparison.terminal(),
        TrivialTerminalProfileV1::ExplicitValue {
            representation: TrivialRepresentationV1::InlineBool,
            ..
        }
    ));

    assert_stop(
        function(vec![
            local("x", Some(void())),
            if_(
                bool_(true),
                vec![assignment("x", null())],
                Some(vec![assignment("x", void())]),
            ),
            return_(Some(variable("x"))),
        ]),
        TrivialProfileStopReasonV1::IfMergeProfileNotHomogeneous,
    );
}

#[test]
fn if_condition_must_be_exact_bool() {
    assert_stop(
        function(vec![if_(int(1), Vec::new(), None)]),
        TrivialProfileStopReasonV1::IfConditionNotBool,
    );
}

#[test]
fn explicit_empty_return_and_implicit_fallthrough_are_distinct() {
    let explicit = admitted(function(vec![return_(None)]));
    let implicit = admitted(function(vec![local("x", Some(int(1)))]));

    assert!(matches!(
        explicit.terminal(),
        TrivialTerminalProfileV1::ExplicitNoValue { .. }
    ));
    assert!(matches!(
        implicit.terminal(),
        TrivialTerminalProfileV1::ImplicitNoValue { body_end: 1, .. }
    ));
}

#[test]
fn parameter_outbox_and_missing_initializer_are_typed_stops() {
    assert_stop(
        function_with_params(vec!["arg".into()], vec![return_(Some(variable("arg")))]),
        TrivialProfileStopReasonV1::ParameterRepresentationUnavailable,
    );
    assert_stop(
        function(vec![ASTNode::Outbox {
            variables: vec!["result".into()],
            initial_values: vec![None],
            span: Span::unknown(),
        }]),
        TrivialProfileStopReasonV1::OutboxRepresentationUnavailable,
    );
    assert_stop(
        function(vec![local("x", None)]),
        TrivialProfileStopReasonV1::MissingLocalInitializer,
    );
}

#[test]
fn string_value_remains_a_typed_stop() {
    assert_stop(
        function(vec![return_(Some(literal(LiteralValue::String(
            "text".into(),
        ))))]),
        TrivialProfileStopReasonV1::StringRepresentationUnavailable,
    );
}

#[test]
fn null_terminal_and_mixed_merge_remain_typed_stops() {
    assert_stop(
        function(vec![return_(Some(null()))]),
        TrivialProfileStopReasonV1::NullRepresentationUnavailable,
    );
    assert_stop(
        function(vec![
            local("x", Some(null())),
            if_(
                bool_(true),
                vec![assignment("x", int(1))],
                Some(vec![assignment("x", null())]),
            ),
            return_(Some(binary(BinaryOperator::Equal, variable("x"), null()))),
        ]),
        TrivialProfileStopReasonV1::IfMergeProfileNotHomogeneous,
    );
}

#[test]
fn mixed_binary_and_short_circuit_are_typed_stops() {
    assert_stop(
        function(vec![return_(Some(binary(
            BinaryOperator::Add,
            int(1),
            float(2.0),
        )))]),
        TrivialProfileStopReasonV1::BinaryOperandsNotExact,
    );
    assert_stop(
        function(vec![return_(Some(binary(
            BinaryOperator::And,
            bool_(true),
            bool_(false),
        )))]),
        TrivialProfileStopReasonV1::BinaryOperatorOutsideProfile,
    );
}

#[test]
fn duplicate_coverage_and_foreign_if_control_cannot_seal() {
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(function(vec![int(1)])).unwrap();
    let input = unit.root_function_input().unwrap();
    let mut draft = TrivialProfileDraftV1::new(input.owner());
    let site = SourcePathV1::root_body(0).expr();
    draft
        .record_value(site.clone(), TrivialRepresentationV1::InlineI64)
        .unwrap();
    assert!(matches!(
        draft.record_value(site, TrivialRepresentationV1::InlineI64),
        Err(TrivialProfileContractErrorV1::DuplicateCoverage { .. })
    ));

    let other = VerifiedResolvedSourceUnitV1::resolve_function(function(vec![int(2)])).unwrap();
    let other_input = other.root_function_input().unwrap();
    let other_completion = verify_function_completion_v1(other_input).unwrap();
    let other_if_control =
        verify_resolved_function_if_control_v1(other_input, &other_completion).unwrap();
    let completion = verify_function_completion_v1(input).unwrap();
    assert!(matches!(
        analyze_trivial_canonical_owner_v1(input, &completion, &other_if_control),
        Err(TrivialProfileContractErrorV1::IfControlOwnerMismatch)
    ));
}

#[test]
fn current_a_plus_acceptance_is_not_narrowed_by_disconnected_profile() {
    let ast = function_with_params(
        vec!["arg".into()],
        vec![
            ASTNode::Outbox {
                variables: vec!["result".into()],
                initial_values: vec![None],
                span: Span::unknown(),
            },
            return_(Some(variable("arg"))),
        ],
    );
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(ast).unwrap();
    assert!(CanonicalLoweringPreflightV1::verify(&unit).is_ok());
}
