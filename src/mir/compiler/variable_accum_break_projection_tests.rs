use super::issue_variable_accum_break_source_attempt_v1;
use crate::ast::{ASTNode, BinaryOperator, DeclarationAttrs, LiteralValue, Span};
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::compiler::VerifiedResolvedSourceUnitV1;
use crate::mir::loop_recipe_contract::{LoopItemKeyV1, LoopJoinBranchArmV1, LoopJoinEdgeRoleV1};
use crate::mir::loop_structural_facts::{
    VariableAccumBreakObservationCoverageV1, VariableAccumBreakSourceAttemptOutcomeV1,
    VariableAccumBreakSourceDeclineV1, VariableAccumBreakSourceRejectV1,
    VariableAccumBreakSourceUnresolvedV1,
};
use crate::mir::resolved_semantics::CallableSemanticSourceLedgerView;

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

fn binary(operator: BinaryOperator, left: ASTNode, right: ASTNode) -> ASTNode {
    ASTNode::BinaryOp {
        operator,
        left: Box::new(left),
        right: Box::new(right),
        span: Span::unknown(),
    }
}

fn assignment(target: &str, left: &str, right: ASTNode) -> ASTNode {
    ASTNode::Assignment {
        target: Box::new(variable(target)),
        value: Box::new(binary(BinaryOperator::Add, variable(left), right)),
        span: Span::unknown(),
    }
}

fn function(loop_operator: BinaryOperator, explicit_else: bool) -> ASTNode {
    let else_body = explicit_else.then(|| {
        vec![ASTNode::Continue {
            span: Span::unknown(),
        }]
    });
    ASTNode::FunctionDeclaration {
        name: "main".into(),
        params: Vec::new(),
        param_decls: Vec::new(),
        return_type_name: Some("i64".into()),
        body: vec![
            ASTNode::Local {
                variables: vec!["sum".into()],
                initial_values: vec![Some(Box::new(integer(0)))],
                declared_type_names: vec![Some("i64".into())],
                span: Span::unknown(),
            },
            ASTNode::Local {
                variables: vec!["i".into()],
                initial_values: vec![Some(Box::new(integer(0)))],
                declared_type_names: vec![Some("i64".into())],
                span: Span::unknown(),
            },
            ASTNode::Loop {
                condition: Box::new(binary(loop_operator, variable("i"), integer(10))),
                body: vec![
                    ASTNode::If {
                        condition: Box::new(binary(
                            BinaryOperator::Equal,
                            variable("i"),
                            integer(5),
                        )),
                        then_body: vec![
                            assignment("sum", "sum", integer(10)),
                            ASTNode::Break {
                                span: Span::unknown(),
                            },
                        ],
                        else_body,
                        span: Span::unknown(),
                    },
                    assignment("sum", "sum", integer(1)),
                    assignment("i", "i", integer(1)),
                ],
                span: Span::unknown(),
            },
            ASTNode::Return {
                value: Some(Box::new(variable("sum"))),
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

fn input_for(
    loop_operator: BinaryOperator,
    explicit_else: bool,
) -> ResolvedFunctionLoweringInputV1<'static> {
    let unit = Box::leak(Box::new(
        VerifiedResolvedSourceUnitV1::resolve_function(function(loop_operator, explicit_else))
            .expect("break recurrence fixture resolves"),
    ));
    unit.root_function_input().expect("root input")
}

fn candidate(
    input: ResolvedFunctionLoweringInputV1<'_>,
) -> crate::mir::loop_structural_facts::VerifiedVariableAccumBreakFactsV1 {
    let ledger = CallableSemanticSourceLedgerView::from_forest(input.forest(), input.owner())
        .expect("ledger");
    let membership = ledger.only_loop_site().expect("one loop");
    let attempt = issue_variable_accum_break_source_attempt_v1(
        input,
        &ledger,
        membership,
        VariableAccumBreakObservationCoverageV1::Complete,
    );
    let VariableAccumBreakSourceAttemptOutcomeV1::Candidate(facts) = attempt.into_parts().0 else {
        panic!("expected candidate facts");
    };
    facts
}

#[test]
fn break_recurrence_projects_exact_source_contract() {
    let input = input_for(BinaryOperator::Less, false);
    let ledger = CallableSemanticSourceLedgerView::from_forest(input.forest(), input.owner())
        .expect("ledger");
    let membership = ledger.only_loop_site().expect("one loop");
    let attempt = issue_variable_accum_break_source_attempt_v1(
        input,
        &ledger,
        membership,
        VariableAccumBreakObservationCoverageV1::Complete,
    );
    let (outcome, identity, coverage) = attempt.into_parts();
    let facts = match outcome {
        VariableAccumBreakSourceAttemptOutcomeV1::Candidate(facts) => facts,
        other => panic!("unexpected outcome: {other:?}"),
    };
    assert_eq!(coverage, VariableAccumBreakObservationCoverageV1::Complete);
    assert_eq!(facts.bindings().len(), 2);
    assert_eq!(facts.inputs().len(), 2);
    assert_eq!(facts.coverage().root_statement_count(), 4);
    assert_eq!(facts.coverage().operation_roles().len(), 20);
    assert_eq!(facts.loop_condition().bound(), 10);
    assert_eq!(facts.branch_condition().bound(), 5);
    assert!(identity.site().node().segments().len() >= 1);
}

#[test]
fn break_recurrence_producer_emits_existing_recipe_and_join_sig() {
    let facts = candidate(input_for(BinaryOperator::Less, false));
    let product = crate::mir::loop_recipe_contract::produce_variable_accum_break_recipe_v1(facts)
        .expect("existing Recipe/JoinSig/Core product");
    let recipe = product.recipe().as_recipe();
    assert_eq!(recipe.loops.len(), 1);
    assert_eq!(recipe.blocks.len(), 3);
    assert_eq!(recipe.items.len(), 20);
    assert_eq!(recipe.values.len(), 17);
    assert_eq!(recipe.inputs.len(), 2);
    assert_eq!(recipe.exits.len(), 1);
    assert_eq!(product.inputs().rows().len(), 2);
    assert_eq!(product.operations().evidence().len(), 18);
    assert_eq!(product.operations().core().effect_relations().len(), 10);

    let sig = product.operations().core().join_sig().as_sig();
    assert_eq!(sig.branches.len(), 1);
    let branch = &sig.branches[0];
    assert_eq!(branch.if_item, LoopItemKeyV1::new(6));
    assert!(matches!(
        branch.then_arm,
        LoopJoinBranchArmV1::Exit(ref exit) if exit.exit_item == LoopItemKeyV1::new(11)
    ));
    assert!(matches!(
        branch.else_arm,
        LoopJoinBranchArmV1::Fallthrough { .. }
    ));
    let edge_roles = &sig.loops[0]
        .edges
        .iter()
        .map(|edge| edge.role)
        .collect::<Vec<_>>();
    assert!(edge_roles.contains(&LoopJoinEdgeRoleV1::Break));
    assert!(edge_roles.contains(&LoopJoinEdgeRoleV1::Backedge));
    assert!(edge_roles.contains(&LoopJoinEdgeRoleV1::PredicateTrue));
    assert!(edge_roles.contains(&LoopJoinEdgeRoleV1::PredicateFalse));
    assert!(
        product
            .control_source()
            .branch_site()
            .node()
            .segments()
            .len()
            >= 1
    );
    assert!(
        product
            .control_source()
            .break_site()
            .node()
            .segments()
            .len()
            >= 1
    );
}

#[test]
fn break_recurrence_maps_incomplete_observation_to_unresolved() {
    let input = input_for(BinaryOperator::Less, false);
    let ledger = CallableSemanticSourceLedgerView::from_forest(input.forest(), input.owner())
        .expect("ledger");
    let membership = ledger.only_loop_site().expect("one loop");
    let attempt = issue_variable_accum_break_source_attempt_v1(
        input,
        &ledger,
        membership,
        VariableAccumBreakObservationCoverageV1::Incomplete,
    );
    assert!(matches!(
        attempt.outcome(),
        VariableAccumBreakSourceAttemptOutcomeV1::Unresolved(
            VariableAccumBreakSourceUnresolvedV1::IncompleteCoverage
        )
    ));
}

#[test]
fn break_recurrence_declines_unsupported_shape() {
    let input = input_for(BinaryOperator::Greater, false);
    let ledger = CallableSemanticSourceLedgerView::from_forest(input.forest(), input.owner())
        .expect("ledger");
    let membership = ledger.only_loop_site().expect("one loop");
    let attempt = issue_variable_accum_break_source_attempt_v1(
        input,
        &ledger,
        membership,
        VariableAccumBreakObservationCoverageV1::Complete,
    );
    assert!(matches!(
        attempt.outcome(),
        VariableAccumBreakSourceAttemptOutcomeV1::Declined(
            VariableAccumBreakSourceDeclineV1::NotVariableAccumBreakShape
        )
    ));
}

#[test]
fn break_recurrence_declines_explicit_else() {
    let input = input_for(BinaryOperator::Less, true);
    let ledger = CallableSemanticSourceLedgerView::from_forest(input.forest(), input.owner())
        .expect("ledger");
    let membership = ledger.only_loop_site().expect("one loop");
    let attempt = issue_variable_accum_break_source_attempt_v1(
        input,
        &ledger,
        membership,
        VariableAccumBreakObservationCoverageV1::Complete,
    );
    assert!(matches!(
        attempt.outcome(),
        VariableAccumBreakSourceAttemptOutcomeV1::Declined(
            VariableAccumBreakSourceDeclineV1::NotVariableAccumBreakShape
        )
    ));
}

#[test]
fn break_recurrence_rejects_foreign_owner_before_shape() {
    let input = input_for(BinaryOperator::Less, false);
    let foreign = input_for(BinaryOperator::Less, false);
    let ledger = CallableSemanticSourceLedgerView::from_forest(foreign.forest(), foreign.owner())
        .expect("foreign ledger");
    let membership = ledger.only_loop_site().expect("foreign loop");
    let attempt = issue_variable_accum_break_source_attempt_v1(
        input,
        &ledger,
        membership,
        VariableAccumBreakObservationCoverageV1::Complete,
    );
    assert!(matches!(
        attempt.outcome(),
        VariableAccumBreakSourceAttemptOutcomeV1::Rejected(
            VariableAccumBreakSourceRejectV1::ForeignOwner
        )
    ));
}
