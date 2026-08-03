use super::nested_predicate_projection::{
    issue_nested_predicate_source_projection_v1, NestedChildBodyRoleV1,
    NestedObservedRecurrenceOwnerV1, NestedPredicateProjectionRejectV1, NestedRootBodyRoleV1,
};
use crate::ast::{ASTNode, BinaryOperator, DeclarationAttrs, LiteralValue, Span};
use crate::mir::compiler::VerifiedResolvedSourceUnitV1;
use crate::mir::resolved_semantics::ScopeKindV1;

fn variable(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.to_owned(),
        span: Span::unknown(),
    }
}

fn integer(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: Span::unknown(),
    }
}

fn increment(name: &str) -> ASTNode {
    ASTNode::Assignment {
        target: Box::new(variable(name)),
        value: Box::new(ASTNode::BinaryOp {
            operator: BinaryOperator::Add,
            left: Box::new(variable(name)),
            right: Box::new(integer(1)),
            span: Span::unknown(),
        }),
        span: Span::unknown(),
    }
}

fn assign(name: &str, value: i64) -> ASTNode {
    ASTNode::Assignment {
        target: Box::new(variable(name)),
        value: Box::new(integer(value)),
        span: Span::unknown(),
    }
}

fn nested_function() -> ASTNode {
    let child = ASTNode::Loop {
        condition: Box::new(ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left: Box::new(variable("j")),
            right: Box::new(integer(3)),
            span: Span::unknown(),
        }),
        body: vec![increment("sum"), increment("j")],
        span: Span::unknown(),
    };
    ASTNode::FunctionDeclaration {
        name: "nested_loop_minimal".into(),
        params: Vec::new(),
        param_decls: Vec::new(),
        return_type_name: None,
        body: vec![
            ASTNode::Local {
                variables: vec!["i".into(), "sum".into()],
                initial_values: vec![Some(Box::new(integer(0))), Some(Box::new(integer(0)))],
                declared_type_names: vec![None, None],
                span: Span::unknown(),
            },
            ASTNode::Loop {
                condition: Box::new(ASTNode::BinaryOp {
                    operator: BinaryOperator::Less,
                    left: Box::new(variable("i")),
                    right: Box::new(integer(3)),
                    span: Span::unknown(),
                }),
                body: vec![
                    ASTNode::Local {
                        variables: vec!["j".into()],
                        initial_values: vec![None],
                        declared_type_names: vec![None],
                        span: Span::unknown(),
                    },
                    assign("j", 0),
                    child,
                    increment("i"),
                ],
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

fn root_input_and_loop(
    unit: &VerifiedResolvedSourceUnitV1,
) -> (
    crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
    crate::mir::compiler::located::LocatedStmtV1<'_>,
) {
    let input = unit.root_function_input().expect("root function input");
    let body = input.source().root_body().expect("function body");
    let root = input
        .source()
        .body_stmt(&body, 1)
        .expect("root loop statement");
    (input, root)
}

#[test]
fn nested_projection_seals_source_shape_and_lexical_recurrence_boundary() {
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(nested_function()).unwrap();
    let (input, root) = root_input_and_loop(&unit);
    let projection =
        issue_nested_predicate_source_projection_v1(input, &root).expect("source projection");
    let shape = projection.shape();
    assert_eq!(projection.forest_binding().members().len(), 2);
    assert_eq!(shape.root_condition.bound, 3);
    assert_eq!(shape.child_condition.bound, 3);
    assert_eq!(shape.initialize_child.delta, 0);
    assert_eq!(shape.increment_root.delta, 1);
    assert_eq!(shape.increment_ancestor.delta, 1);
    assert_eq!(shape.increment_child.delta, 1);
    assert_eq!(
        shape.root_body_roles,
        [
            NestedRootBodyRoleV1::LocalJ,
            NestedRootBodyRoleV1::InitializeJ,
            NestedRootBodyRoleV1::ChildLoop,
            NestedRootBodyRoleV1::IncrementRoot,
        ]
    );
    assert_eq!(
        shape.child_body_roles,
        [
            NestedChildBodyRoleV1::IncrementAncestor,
            NestedChildBodyRoleV1::IncrementChild,
        ]
    );
    assert_eq!(
        shape.bindings[0].recurrence_owner,
        NestedObservedRecurrenceOwnerV1::Root
    );
    assert_eq!(
        input
            .function()
            .scope(shape.bindings[0].lexical_scope)
            .unwrap()
            .kind(),
        ScopeKindV1::LexicalBlock
    );
    assert!(shape.bindings[0].parent_visible);
    assert_eq!(
        shape.bindings[1].recurrence_owner,
        NestedObservedRecurrenceOwnerV1::Root
    );
    assert_eq!(
        input
            .function()
            .scope(shape.bindings[1].lexical_scope)
            .unwrap()
            .kind(),
        ScopeKindV1::LexicalBlock
    );
    assert!(shape.bindings[1].parent_visible);
    assert_eq!(
        shape.bindings[2].recurrence_owner,
        NestedObservedRecurrenceOwnerV1::Child
    );
    assert_eq!(
        input
            .function()
            .scope(shape.bindings[2].lexical_scope)
            .unwrap()
            .kind(),
        ScopeKindV1::LoopBody
    );
    assert!(!shape.bindings[2].parent_visible);
}

#[test]
fn nested_projection_rejects_foreign_located_loop_before_forest_issue() {
    let first = VerifiedResolvedSourceUnitV1::resolve_function(nested_function()).unwrap();
    let second = VerifiedResolvedSourceUnitV1::resolve_function(nested_function()).unwrap();
    let (input, _) = root_input_and_loop(&first);
    let (_, foreign_root) = root_input_and_loop(&second);
    assert_eq!(
        issue_nested_predicate_source_projection_v1(input, &foreign_root),
        Err(NestedPredicateProjectionRejectV1::ForeignOwner)
    );
}

#[test]
fn nested_projection_rejects_extra_child_statement() {
    let mut tree = nested_function();
    let ASTNode::FunctionDeclaration { body, .. } = &mut tree else {
        unreachable!();
    };
    let ASTNode::Loop { body: outer, .. } = &mut body[1] else {
        unreachable!();
    };
    let ASTNode::Loop { body: inner, .. } = &mut outer[2] else {
        unreachable!();
    };
    inner.push(assign("sum", 0));
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(tree).unwrap();
    let (input, root) = root_input_and_loop(&unit);
    assert_eq!(
        issue_nested_predicate_source_projection_v1(input, &root),
        Err(NestedPredicateProjectionRejectV1::ChildBodySchedule)
    );
}
