use super::function_input::ResolvedFunctionLoweringInputV1;
use super::variable_accum_recurrence_projection::{
    issue_variable_accum_recurrence_facts_from_membership_v1,
    VariableAccumRecurrenceProjectionRejectV1,
};
use crate::ast::{ASTNode, BinaryOperator, DeclarationAttrs, LiteralValue, Span};
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

fn input_for(condition: BinaryOperator) -> ResolvedFunctionLoweringInputV1<'static> {
    let unit = Box::leak(Box::new(
        crate::mir::compiler::VerifiedResolvedSourceUnitV1::resolve_function(function(condition))
            .expect("recurrence fixture resolves"),
    ));
    unit.root_function_input().expect("root input")
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
