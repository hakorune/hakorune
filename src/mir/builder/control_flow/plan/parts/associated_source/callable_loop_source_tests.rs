//! Provider-level tests for the located callable-loop associated-source
//! provider: carrier pairing, foreign/mismatched co-seal rejections, and item
//! projection sites. Hook driver coverage lives in
//! `callable_loop_source_driver_tests`; shared fixtures live in
//! `callable_loop_source_testkit`.

use super::callable_loop_source::{
    CallableLoopSourcePartsAssociatedSourceV1, CallableLoopSourcePartsBlockV1,
};
use super::callable_loop_source_testkit::{
    add, assign, break_node, continue_node, expr_segments, function_body_source, if_node, integer,
    less_than, local, loop_node, project, real_ledger, span, stmt_segments, variable,
};
use super::{PartsAssociatedRecipeItemV1, PartsAssociatedSourceErrorV1, PartsAssociatedSourceV1};
use crate::ast::ASTNode;
use crate::mir::builder::control_flow::facts::canon::cond_block_view::CondBlockView;
use crate::mir::builder::control_flow::facts::no_exit_block::try_build_no_exit_block_recipe;
use crate::mir::builder::control_flow::plan::expression_port::LoopPlanExpressionPortV1;
use crate::mir::builder::control_flow::plan::facts::exit_only_block::try_build_exit_allowed_block_recipe;
use crate::mir::builder::control_flow::plan::recipe_tree::{
    ExitKind, IfContractKind, IfMode, RecipeItem,
};
use crate::mir::builder::normal_callable_loop_source_port::{
    CallableLoopSourceBodyInputV1, CallableLoopSourceExpressionPortV1,
    CallableLoopSourceStmtInputV1,
};
use crate::mir::resolved_semantics::SourcePathSegmentV1;

#[test]
fn projects_stmt_and_exit_items_with_located_sites() {
    let (ledger, _) = real_ledger("function t() { local x = 1 }");
    let port = CallableLoopSourceExpressionPortV1::new(&ledger);
    let body = vec![local("x", integer(1)), break_node()];
    let recipe = try_build_exit_allowed_block_recipe(&body, true).expect("exit-allowed recipe");
    let carrier = port
        .body(&body, &function_body_source())
        .expect("located body");
    let block =
        CallableLoopSourcePartsBlockV1::located_body(&recipe.arena, &recipe.block, carrier, &port)
            .expect("co-sealed block");
    let source = CallableLoopSourcePartsAssociatedSourceV1::for_block(&block, port);

    assert_eq!(source.block_len(&block), Ok(2));
    let first = project(&source, &block, 0).expect("stmt item");
    let PartsAssociatedRecipeItemV1::OpaqueStmt { source: stmt } = first else {
        panic!("expected OpaqueStmt")
    };
    assert_eq!(port.stmt_syntax(&stmt), &body[0]);
    assert_eq!(stmt_segments(&stmt), [SourcePathSegmentV1::Body(0)]);

    let second = project(&source, &block, 1).expect("exit item");
    let PartsAssociatedRecipeItemV1::OpaqueExit { source: stmt, kind } = second else {
        panic!("expected OpaqueExit")
    };
    assert!(matches!(kind, ExitKind::Break { depth: 1 }));
    assert_eq!(port.stmt_syntax(&stmt), &body[1]);
    assert_eq!(stmt_segments(&stmt), [SourcePathSegmentV1::Body(1)]);
}

#[test]
fn projects_explicit_if_with_exact_child_carriers() {
    let (ledger, _) = real_ledger("function t() { local x = 1 }");
    let port = CallableLoopSourceExpressionPortV1::new(&ledger);
    let condition = less_than(variable("i"), integer(3));
    let body = vec![if_node(
        condition.clone(),
        vec![break_node()],
        Some(vec![continue_node()]),
    )];
    let recipe = try_build_exit_allowed_block_recipe(&body, true).expect("exit-allowed recipe");
    let carrier = port
        .body(&body, &function_body_source())
        .expect("located body");
    let block =
        CallableLoopSourcePartsBlockV1::located_body(&recipe.arena, &recipe.block, carrier, &port)
            .expect("co-sealed block");
    let source = CallableLoopSourcePartsAssociatedSourceV1::for_block(&block, port);

    let item = project(&source, &block, 0).expect("if item");
    let PartsAssociatedRecipeItemV1::ExplicitIfV2 {
        source: stmt,
        condition: cond,
        then_body,
        else_body,
        contract,
        then_block,
        else_block,
    } = item
    else {
        panic!("expected ExplicitIfV2")
    };
    assert_eq!(port.expr_syntax(&cond), &condition);
    assert_eq!(
        expr_segments(&cond),
        [
            SourcePathSegmentV1::Body(0),
            SourcePathSegmentV1::IfCondition
        ]
    );
    assert_eq!(port.body_statements(&then_body), [break_node()]);
    assert_eq!(
        port.body_statements(&else_body.expect("else carrier")),
        [continue_node()]
    );
    assert!(matches!(
        contract,
        IfContractKind::ExitOnly {
            mode: IfMode::ExitAll
        }
    ));
    assert_eq!(source.block_len(&then_block), Ok(1));
    assert_eq!(
        else_block
            .as_ref()
            .map(|block| source.block_len(block)),
        Some(Ok(1))
    );
    assert_eq!(port.stmt_syntax(&stmt), &body[0]);
}

#[test]
fn projects_loop_v0_with_located_body_block() {
    let (ledger, _) = real_ledger("function t() { local x = 1 }");
    let port = CallableLoopSourceExpressionPortV1::new(&ledger);
    let condition = less_than(variable("i"), integer(3));
    let body = vec![loop_node(
        condition.clone(),
        vec![assign("i", add(variable("i"), integer(1)))],
    )];
    let recipe = try_build_exit_allowed_block_recipe(&body, true).expect("exit-allowed recipe");
    let carrier = port
        .body(&body, &function_body_source())
        .expect("located body");
    let block =
        CallableLoopSourcePartsBlockV1::located_body(&recipe.arena, &recipe.block, carrier, &port)
            .expect("co-sealed block");
    let source = CallableLoopSourcePartsAssociatedSourceV1::for_block(&block, port);

    let item = project(&source, &block, 0).expect("loop item");
    let PartsAssociatedRecipeItemV1::RawLoopV0 { loop_input } = item else {
        panic!("expected RawLoopV0")
    };
    assert_eq!(port.expr_syntax(&loop_input.condition), &condition);
    assert_eq!(
        stmt_segments(&loop_input.source),
        [SourcePathSegmentV1::Body(0)]
    );
    assert_eq!(source.block_len(&loop_input.body_block), Ok(1));
}

#[test]
fn rejects_block_from_a_foreign_arena() {
    let (ledger, _) = real_ledger("function t() { local x = 1 }");
    let port = CallableLoopSourceExpressionPortV1::new(&ledger);
    let body_a = vec![local("x", integer(1))];
    let body_b = vec![local("y", integer(2))];
    let recipe_a = try_build_exit_allowed_block_recipe(&body_a, true).expect("recipe a");
    let recipe_b = try_build_exit_allowed_block_recipe(&body_b, true).expect("recipe b");
    let carrier = port
        .body(&body_a, &function_body_source())
        .expect("located body");
    let block = CallableLoopSourcePartsBlockV1::located_body(
        &recipe_a.arena,
        &recipe_a.block,
        carrier,
        &port,
    )
    .expect("co-sealed block");

    let foreign = CallableLoopSourcePartsAssociatedSourceV1::new(&recipe_b.arena, port);
    assert_eq!(
        foreign.block_len(&block),
        Err(PartsAssociatedSourceErrorV1::ForeignRawBlock)
    );
    assert_eq!(
        project(&foreign, &block, 0).map(|_| ()),
        Err(PartsAssociatedSourceErrorV1::ForeignRawBlock)
    );
}

#[test]
fn rejects_body_carrier_that_disagrees_with_the_recipe_body() {
    let (ledger, _) = real_ledger("function t() { local x = 1 }");
    let port = CallableLoopSourceExpressionPortV1::new(&ledger);
    let recipe_body = vec![local("x", integer(1))];
    let other_body = vec![local("y", integer(1)), local("z", integer(2))];
    let recipe = try_build_exit_allowed_block_recipe(&recipe_body, true).expect("recipe");
    let mismatched = port
        .body(&other_body, &function_body_source())
        .expect("located body");
    assert_eq!(
        CallableLoopSourcePartsBlockV1::located_body(
            &recipe.arena,
            &recipe.block,
            mismatched,
            &port,
        )
        .map(|_| ()),
        Err(PartsAssociatedSourceErrorV1::RecipeBodyMismatch)
    );
}

#[test]
fn rejects_statement_carrier_that_disagrees_with_the_recipe_body() {
    let (ledger, _) = real_ledger("function t() { local x = 1 }");
    let port = CallableLoopSourceExpressionPortV1::new(&ledger);
    let recipe_body = vec![local("x", integer(1))];
    let carrier_body = vec![local("y", integer(1))];
    let recipe = try_build_exit_allowed_block_recipe(&recipe_body, true).expect("recipe");
    let carrier = port
        .body(&carrier_body, &function_body_source())
        .expect("located body");
    let block =
        CallableLoopSourcePartsBlockV1::located_body(&recipe.arena, &recipe.block, carrier, &port)
            .expect("same length co-seal");
    let source = CallableLoopSourcePartsAssociatedSourceV1::for_block(&block, port);
    assert_eq!(
        project(&source, &block, 0).map(|_| ()),
        Err(PartsAssociatedSourceErrorV1::RecipeBodyMismatch)
    );
}

#[test]
fn rejects_synthetic_carriers_on_the_located_spine() {
    let (ledger, _) = real_ledger("function t() { local x = 1 }");
    let port = CallableLoopSourceExpressionPortV1::new(&ledger);
    let body = vec![local("x", integer(1))];
    let recipe = try_build_exit_allowed_block_recipe(&body, true).expect("recipe");
    assert_eq!(
        CallableLoopSourcePartsBlockV1::located_body(
            &recipe.arena,
            &recipe.block,
            CallableLoopSourceBodyInputV1::Synthetic(&body),
            &port,
        )
        .map(|_| ()),
        Err(PartsAssociatedSourceErrorV1::SyntheticCarrier)
    );
    assert_eq!(
        CallableLoopSourcePartsBlockV1::singleton(
            &recipe.arena,
            &recipe.block,
            CallableLoopSourceStmtInputV1::Synthetic(&body[0]),
            &port,
        )
        .map(|_| ()),
        Err(PartsAssociatedSourceErrorV1::SyntheticCarrier)
    );
}

#[test]
fn singleton_requires_the_exact_statement() {
    let (ledger, _) = real_ledger("function t() { local x = 1 }");
    let port = CallableLoopSourceExpressionPortV1::new(&ledger);
    let stmt = local("x", integer(1));
    let recipe = try_build_no_exit_block_recipe(std::slice::from_ref(&stmt), true)
        .expect("no-exit recipe");
    let carrier_body = vec![stmt.clone()];
    let carrier = port
        .body(&carrier_body, &function_body_source())
        .expect("located body");
    let located = port.body_stmt(&carrier, 0).expect("located stmt");
    assert!(CallableLoopSourcePartsBlockV1::singleton(
        &recipe.arena,
        &recipe.block,
        located,
        &port
    )
    .is_ok());

    let foreign_body = vec![local("y", integer(1))];
    let foreign_carrier = port
        .body(&foreign_body, &function_body_source())
        .expect("located body");
    let foreign = port.body_stmt(&foreign_carrier, 0).expect("located stmt");
    assert_eq!(
        CallableLoopSourcePartsBlockV1::singleton(
            &recipe.arena,
            &recipe.block,
            foreign,
            &port
        )
        .map(|_| ()),
        Err(PartsAssociatedSourceErrorV1::RecipeBodyMismatch)
    );
}

#[test]
fn rejects_a_condition_view_that_disagrees_with_the_ast() {
    let (ledger, _) = real_ledger("function t() { local x = 1 }");
    let port = CallableLoopSourceExpressionPortV1::new(&ledger);
    let body = vec![if_node(
        less_than(variable("i"), integer(3)),
        vec![break_node()],
        None,
    )];
    let mut recipe = try_build_exit_allowed_block_recipe(&body, true).expect("recipe");
    let Some(RecipeItem::IfV2 { cond_view, .. }) = recipe.block.items.first_mut() else {
        panic!("expected IfV2")
    };
    *cond_view = CondBlockView::from_expr(&less_than(variable("j"), integer(9)));
    let carrier = port
        .body(&body, &function_body_source())
        .expect("located body");
    let block =
        CallableLoopSourcePartsBlockV1::located_body(&recipe.arena, &recipe.block, carrier, &port)
            .expect("co-sealed block");
    let source = CallableLoopSourcePartsAssociatedSourceV1::for_block(&block, port);
    assert_eq!(
        project(&source, &block, 0).map(|_| ()),
        Err(PartsAssociatedSourceErrorV1::ConditionViewMismatch)
    );
}

#[test]
fn rejects_unlocated_statement_vocab_as_source_projection_error() {
    let (ledger, _) = real_ledger("function t() { local x = 1 }");
    let port = CallableLoopSourceExpressionPortV1::new(&ledger);
    let call = ASTNode::FunctionCall {
        name: "helper".to_owned(),
        arguments: vec![],
        span: span(),
    };
    let body = vec![call];
    let recipe = try_build_exit_allowed_block_recipe(&body, true).expect("recipe");
    let carrier = port
        .body(&body, &function_body_source())
        .expect("located body");
    let block =
        CallableLoopSourcePartsBlockV1::located_body(&recipe.arena, &recipe.block, carrier, &port)
            .expect("co-sealed block");
    let source = CallableLoopSourcePartsAssociatedSourceV1::for_block(&block, port);
    assert!(matches!(
        project(&source, &block, 0),
        Err(PartsAssociatedSourceErrorV1::SourcePortProjection(_))
    ));
}
