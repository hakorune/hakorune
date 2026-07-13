#[cfg(feature = "vm-reference")]
use crate::ast::BinaryOperator;
use crate::ast::{ASTNode, DeclarationAttrs, LiteralValue, Span};
use crate::mir::compiler::capability::CanonicalLoweringPreflightV1;
use crate::mir::compiler::source_view::ExprChildRoleV1;
use crate::mir::compiler::VerifiedResolvedSourceUnitV1;
use crate::mir::resolved_semantics::ResolvedScopeRegionLookupErrorV1;
use crate::mir::{ConstValue, MirInstruction};

use super::identity::ResolvedIdentityStateV1;
use super::scope::ResolvedScopeStateV1;
use super::MirBuilder;

fn int(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: Span::unknown(),
    }
}

#[cfg(feature = "vm-reference")]
fn var(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.into(),
        span: Span::unknown(),
    }
}

#[cfg(feature = "vm-reference")]
fn add(left: ASTNode, right: ASTNode) -> ASTNode {
    ASTNode::BinaryOp {
        operator: BinaryOperator::Add,
        left: Box::new(left),
        right: Box::new(right),
        span: Span::unknown(),
    }
}

#[cfg(feature = "vm-reference")]
fn local(name: &str, initial: ASTNode) -> ASTNode {
    ASTNode::Local {
        variables: vec![name.into()],
        initial_values: vec![Some(Box::new(initial))],
        declared_type_names: vec![None],
        span: Span::unknown(),
    }
}

#[cfg(feature = "vm-reference")]
fn assign(name: &str, value: ASTNode) -> ASTNode {
    ASTNode::Assignment {
        target: Box::new(var(name)),
        value: Box::new(value),
        span: Span::unknown(),
    }
}

fn block(prelude: Vec<ASTNode>, tail: ASTNode) -> ASTNode {
    ASTNode::BlockExpr {
        prelude_stmts: prelude,
        tail_expr: Box::new(tail),
        span: Span::unknown(),
    }
}

fn function(name: &str, body: Vec<ASTNode>) -> ASTNode {
    ASTNode::FunctionDeclaration {
        name: name.into(),
        params: Vec::new(),
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

fn returning(name: &str, expression: ASTNode) -> ASTNode {
    function(
        name,
        vec![ASTNode::Return {
            value: Some(Box::new(expression)),
            span: Span::unknown(),
        }],
    )
}

fn build(root: ASTNode) -> crate::mir::MirModule {
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(root).unwrap();
    let plan = CanonicalLoweringPreflightV1::verify(&unit).unwrap();
    MirBuilder::new()
        .build_resolved_function_module(plan)
        .unwrap()
}

#[cfg(feature = "vm-reference")]
fn execute_integer(root: ASTNode, function_name: &str) -> i64 {
    let module = build(root);
    let value = crate::backend::MirInterpreter::new()
        .execute_function_with_args(&module, function_name, &[])
        .unwrap();
    let crate::backend::VMValue::Integer(value) = value else {
        panic!("expected Integer result")
    };
    value
}

#[test]
fn empty_prelude_tail_is_lowered_exactly_once() {
    let module = build(returning("empty_block", block(Vec::new(), int(7))));
    let function = &module.functions["empty_block/0"];
    let tail_constants = function
        .blocks
        .values()
        .flat_map(|block| &block.instructions)
        .filter(|instruction| {
            matches!(
                instruction,
                MirInstruction::Const {
                    value: ConstValue::Integer(7),
                    ..
                }
            )
        })
        .count();
    assert_eq!(tail_constants, 1);
}

#[cfg(feature = "vm-reference")]
#[test]
fn shadow_restores_outer_and_outer_rebind_survives_scope_leave() {
    let root = function(
        "shadow_and_rebind",
        vec![
            local("x", int(1)),
            local(
                "shadow_value",
                block(vec![local("x", add(var("x"), int(1)))], var("x")),
            ),
            block(vec![assign("x", add(var("x"), int(2)))], var("x")),
            ASTNode::Return {
                value: Some(Box::new(add(var("shadow_value"), var("x")))),
                span: Span::unknown(),
            },
        ],
    );
    assert_eq!(execute_integer(root, "shadow_and_rebind/0"), 5);
}

#[cfg(feature = "vm-reference")]
#[test]
fn nested_pairs_keep_tail_value_after_each_leave() {
    let root = returning(
        "nested_blocks",
        block(
            vec![local("outer", int(4))],
            block(
                vec![local("inner", add(var("outer"), int(1)))],
                add(var("inner"), var("outer")),
            ),
        ),
    );
    assert_eq!(execute_integer(root, "nested_blocks/0"), 9);
}

#[test]
fn exact_pair_query_rejects_wrong_and_foreign_sites() {
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(returning(
        "pair_query",
        block(Vec::new(), int(1)),
    ))
    .unwrap();
    let input = unit.root_function_input().unwrap();
    let body = input.source().root_body().unwrap();
    let statement = input.source().body_stmt(&body, 0).unwrap();
    let expression = input
        .source()
        .child_expr_from_stmt(&statement, ExprChildRoleV1::ReturnValue)
        .unwrap();
    let pair = input
        .function()
        .block_expr_scope_region_pair(expression.owner(), expression.site())
        .unwrap();
    assert_eq!(
        input.function().scope(pair.scope()).unwrap().owner_region(),
        pair.region()
    );

    let tail = input
        .source()
        .child_expr_from_expr(&expression, ExprChildRoleV1::BlockExprTail)
        .unwrap();
    assert!(input
        .function()
        .block_expr_scope_region_pair(tail.owner(), tail.site())
        .is_err());

    let foreign = VerifiedResolvedSourceUnitV1::resolve_function(returning(
        "foreign_pair",
        block(Vec::new(), int(2)),
    ))
    .unwrap();
    assert_eq!(
        input.function().block_expr_scope_region_pair(
            foreign.root_function_input().unwrap().owner(),
            expression.site(),
        ),
        Err(ResolvedScopeRegionLookupErrorV1::ForeignOwner)
    );
}

#[test]
fn error_close_balances_scope_and_reconsumption_is_rejected() {
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(returning(
        "error_balance",
        block(Vec::new(), int(1)),
    ))
    .unwrap();
    let input = unit.root_function_input().unwrap();
    let body = input.source().root_body().unwrap();
    let statement = input.source().body_stmt(&body, 0).unwrap();
    let expression = input
        .source()
        .child_expr_from_stmt(&statement, ExprChildRoleV1::ReturnValue)
        .unwrap();
    let pair = input
        .function()
        .block_expr_scope_region_pair(expression.owner(), expression.site())
        .unwrap();
    let mut scopes = ResolvedScopeStateV1::new(input.function());
    let mut identity = ResolvedIdentityStateV1::new(input.function());
    let session = scopes.enter(input.function(), pair).unwrap();
    scopes.close_error(session, &mut identity).unwrap();
    scopes.finish().unwrap();
    assert!(scopes.enter(input.function(), pair).is_err());
}
