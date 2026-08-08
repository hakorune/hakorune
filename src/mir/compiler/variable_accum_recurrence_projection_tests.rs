use super::function_input::ResolvedFunctionLoweringInputV1;
use super::variable_accum_recurrence_projection::{
    issue_variable_accum_recurrence_facts_from_membership_v1,
    issue_variable_accum_recurrence_source_attempt_v1, VariableAccumRecurrenceProjectionRejectV1,
};
use crate::ast::{ASTNode, BinaryOperator, DeclarationAttrs, LiteralValue, Span};
use crate::mir::compiler::normal_source_plan::{
    NormalSourcePlanClassifierV1, PreparedNormalSourcePlanInputV1, SealedNormalScalarRootV1,
    SealedNormalSourcePlanV1,
};
use crate::mir::loop_structural_facts::{
    VariableAccumRecurrenceObservationCoverageV1, VariableAccumRecurrenceSourceAttemptOutcomeV1,
};
use crate::mir::resolved_semantics::CallableSemanticSourceLedgerView;
use crate::mir::resolved_semantics::SourcePathSegmentV1;
use std::collections::HashMap;

fn variable(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.into(),
        span: Span::unknown(),
    }
}

fn integer(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: Span::unknown(),
    }
}

fn assignment(target: &str, left: &str, right: ASTNode) -> ASTNode {
    ASTNode::Assignment {
        target: Box::new(variable(target)),
        value: Box::new(ASTNode::BinaryOp {
            operator: BinaryOperator::Add,
            left: Box::new(variable(left)),
            right: Box::new(right),
            span: Span::unknown(),
        }),
        span: Span::unknown(),
    }
}

fn function(condition: BinaryOperator) -> ASTNode {
    ASTNode::FunctionDeclaration {
        name: "main".into(),
        params: Vec::new(),
        param_decls: Vec::new(),
        return_type_name: Some("i64".into()),
        body: vec![
            ASTNode::Local {
                variables: vec!["i".into()],
                initial_values: vec![Some(Box::new(integer(0)))],
                declared_type_names: vec![Some("i64".into())],
                span: Span::unknown(),
            },
            ASTNode::Local {
                variables: vec!["acc".into()],
                initial_values: vec![Some(Box::new(integer(0)))],
                declared_type_names: vec![Some("i64".into())],
                span: Span::unknown(),
            },
            ASTNode::Loop {
                condition: Box::new(ASTNode::BinaryOp {
                    operator: condition,
                    left: Box::new(variable("i")),
                    right: Box::new(integer(4)),
                    span: Span::unknown(),
                }),
                body: vec![
                    assignment("acc", "acc", variable("i")),
                    assignment("i", "i", integer(1)),
                ],
                span: Span::unknown(),
            },
            ASTNode::Print {
                expression: Box::new(variable("acc")),
                span: Span::unknown(),
            },
            ASTNode::Return {
                value: Some(Box::new(integer(0))),
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

fn normal_main_program(condition: BinaryOperator) -> ASTNode {
    let function = function(condition);
    let ASTNode::FunctionDeclaration { name, .. } = &function else {
        unreachable!()
    };
    let mut methods = HashMap::new();
    methods.insert(name.clone(), function);
    ASTNode::Program {
        statements: vec![ASTNode::BoxDeclaration {
            name: "Main".into(),
            fields: vec!["retained".into()],
            field_decls: Vec::new(),
            public_fields: Vec::new(),
            private_fields: Vec::new(),
            methods,
            constructors: HashMap::new(),
            init_fields: Vec::new(),
            weak_fields: Vec::new(),
            delegates: Vec::new(),
            invariants: Vec::new(),
            transitions: Vec::new(),
            is_interface: false,
            is_sync: false,
            is_record: false,
            type_parameters: Vec::new(),
            extends: Vec::new(),
            implements: Vec::new(),
            is_static: true,
            static_init: None,
            attrs: DeclarationAttrs::default(),
            span: Span::unknown(),
        }],
        span: Span::unknown(),
    }
}

fn input_for(condition: BinaryOperator) -> ResolvedFunctionLoweringInputV1<'static> {
    let unit = Box::leak(Box::new(
        crate::mir::compiler::VerifiedResolvedSourceUnitV1::resolve_function(function(condition))
            .expect("recurrence fixture resolves"),
    ));
    unit.root_function_input().expect("root input")
}

fn normal_main_input_for(condition: BinaryOperator) -> ResolvedFunctionLoweringInputV1<'static> {
    let prepared = PreparedNormalSourcePlanInputV1::new(
        normal_main_program(condition),
        "variable-accum-recurrence-main-test",
    );
    let plan = NormalSourcePlanClassifierV1::seal(prepared).expect("Main0 source");
    let SealedNormalSourcePlanV1::ScalarRoot(SealedNormalScalarRootV1::Main0(main)) = plan else {
        panic!("expected Main0 source");
    };
    let resolved = main
        .prepare_function_source()
        .expect("Main source")
        .prepare_embedded_resolved_main()
        .expect("embedded Main resolver");
    let resolved = Box::leak(Box::new(resolved));
    resolved.borrow_function_input().expect("Main input")
}

#[test]
fn variable_recurrence_projects_two_bindings_and_eleven_roles() {
    let input = input_for(BinaryOperator::Less);
    let ledger = CallableSemanticSourceLedgerView::from_forest(input.forest(), input.owner())
        .expect("ledger");
    let membership = ledger.only_loop_site().expect("one loop");
    let facts =
        issue_variable_accum_recurrence_facts_from_membership_v1(input, &ledger, membership)
            .expect("candidate facts");

    assert_eq!(facts.bindings().len(), 2);
    assert_eq!(facts.inputs().len(), 2);
    assert_eq!(facts.coverage().operation_roles().len(), 11);
    assert_eq!(facts.condition().bound(), 4);
    assert_eq!(facts.induction_step().delta(), 1);
}

#[test]
fn variable_recurrence_rejects_non_less_condition_before_recipe() {
    let input = input_for(BinaryOperator::Greater);
    let ledger = CallableSemanticSourceLedgerView::from_forest(input.forest(), input.owner())
        .expect("ledger");
    let membership = ledger.only_loop_site().expect("one loop");
    let error =
        issue_variable_accum_recurrence_facts_from_membership_v1(input, &ledger, membership)
            .expect_err("unsupported condition");
    assert_eq!(
        error,
        VariableAccumRecurrenceProjectionRejectV1::ConditionShape
    );
}

#[test]
fn variable_recurrence_normal_main_uses_program_owned_resolver_ingress() {
    let input = normal_main_input_for(BinaryOperator::Less);
    let ledger = CallableSemanticSourceLedgerView::from_forest(input.forest(), input.owner())
        .expect("ledger");
    let membership = ledger.only_loop_site().expect("one loop");
    let attempt = issue_variable_accum_recurrence_source_attempt_v1(
        input,
        &ledger,
        membership,
        VariableAccumRecurrenceObservationCoverageV1::Complete,
    );
    let (outcome, identity, coverage) = attempt.into_parts();
    assert!(matches!(
        outcome,
        VariableAccumRecurrenceSourceAttemptOutcomeV1::Candidate(_)
    ));
    assert_eq!(
        coverage,
        VariableAccumRecurrenceObservationCoverageV1::Complete
    );
    assert!(matches!(
        identity.site().node().segments().last(),
        Some(SourcePathSegmentV1::Body(2))
    ));
}

#[test]
fn variable_recurrence_attempt_maps_incomplete_observation_to_unresolved() {
    let input = normal_main_input_for(BinaryOperator::Less);
    let ledger = CallableSemanticSourceLedgerView::from_forest(input.forest(), input.owner())
        .expect("ledger");
    let membership = ledger.only_loop_site().expect("one loop");
    let attempt = issue_variable_accum_recurrence_source_attempt_v1(
        input,
        &ledger,
        membership,
        VariableAccumRecurrenceObservationCoverageV1::Incomplete,
    );
    assert!(matches!(
        attempt.outcome(),
        VariableAccumRecurrenceSourceAttemptOutcomeV1::Unresolved(_)
    ));
}

#[test]
fn variable_recurrence_attempt_rejects_foreign_owner_before_shape() {
    let input = normal_main_input_for(BinaryOperator::Less);
    let foreign_input = normal_main_input_for(BinaryOperator::Less);
    let foreign_ledger = CallableSemanticSourceLedgerView::from_forest(
        foreign_input.forest(),
        foreign_input.owner(),
    )
    .expect("foreign ledger");
    let membership = foreign_ledger.only_loop_site().expect("foreign loop");
    let attempt = issue_variable_accum_recurrence_source_attempt_v1(
        input,
        &foreign_ledger,
        membership,
        VariableAccumRecurrenceObservationCoverageV1::Complete,
    );
    assert!(matches!(
        attempt.outcome(),
        VariableAccumRecurrenceSourceAttemptOutcomeV1::Rejected(_)
    ));
}

#[test]
fn variable_recurrence_producer_seals_existing_recipe_core() {
    let input = input_for(BinaryOperator::Less);
    let ledger = CallableSemanticSourceLedgerView::from_forest(input.forest(), input.owner())
        .expect("ledger");
    let membership = ledger.only_loop_site().expect("one loop");
    let facts =
        issue_variable_accum_recurrence_facts_from_membership_v1(input, &ledger, membership)
            .expect("candidate facts");
    let product =
        crate::mir::loop_recipe_contract::produce_variable_accum_recurrence_recipe_v1(facts)
            .expect("recipe product");

    assert_eq!(product.inputs().rows().len(), 2);
    assert_eq!(product.operations().evidence().len(), 11);
    assert_eq!(product.operations().core().binding_relations().len(), 2);
    assert_eq!(product.operations().core().effect_relations().len(), 8);
    assert_eq!(
        product.operations().core().recipe().as_recipe().items.len(),
        11
    );
}
