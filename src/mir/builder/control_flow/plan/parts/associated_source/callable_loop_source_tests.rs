//! Focused tests for the located callable-loop associated-source provider and
//! its lowering hooks: carrier pairing, foreign/mismatched co-seal rejections,
//! and the hooks coverage matrix driven through the neutral block driver.

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

use super::callable_loop_source::{
    CallableLoopSourcePartsAssociatedSourceV1, CallableLoopSourcePartsBlockV1,
    CallableLoopSourcePartsLoopV0V1,
};
use super::callable_loop_source_lowering::lower_callable_loop_source_parts_block;
use super::dispatch::PartsAssociatedBlockModeV1;
use super::{PartsAssociatedRecipeItemV1, PartsAssociatedSourceErrorV1, PartsAssociatedSourceV1};
use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};
use crate::mir::builder::control_flow::facts::canon::cond_block_view::CondBlockView;
use crate::mir::builder::control_flow::facts::no_exit_block::try_build_no_exit_block_recipe;
use crate::mir::builder::control_flow::plan::expression_port::LoopPlanExpressionPortV1;
use crate::mir::builder::control_flow::plan::facts::exit_only_block::try_build_exit_allowed_block_recipe;
use crate::mir::builder::control_flow::plan::recipe_tree::{
    ExitKind, IfContractKind, IfMode, RecipeBlock, RecipeBodies, RecipeItem,
};
use crate::mir::builder::control_flow::plan::LoweredRecipe;
use crate::mir::builder::normal_callable_binding_materialization_port::PreparedCallableEntryValuesV1;
use crate::mir::builder::normal_callable_loop_source_port::{
    CallableLoopSourceBodyInputV1, CallableLoopSourceExprInputV1,
    CallableLoopSourceExpressionPortV1, CallableLoopSourceStmtInputV1,
};
use crate::mir::builder::normal_callable_semantic_lowering_state::CallableSemanticLoweringState;
use crate::mir::builder::raw_invocation_source_transport::{
    RawInvocationRootLineageV1, RawInvocationSourceContextV1,
};
use crate::mir::builder::vars::lexical_scope::LexicalScopeGuard;
use crate::mir::builder::MirBuilder;
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::compiler::source_projection::VerifiedSourceProjectionV1;
use crate::mir::resolved_semantics::{
    BodyChildRoleV1, CallableFunctionSyntaxViewV1, FunctionSemanticResolverSessionV1,
    ResolveSelectedCallableForestsOutcomeV1, SourceBodyKindV1, SourcePathSegmentV1, SourcePathV1,
};
use crate::mir::ValueId;
use crate::parser::NyashParser;

fn span() -> Span {
    Span::unknown()
}

fn integer(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: span(),
    }
}

fn variable(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.to_owned(),
        span: span(),
    }
}

fn less_than(left: ASTNode, right: ASTNode) -> ASTNode {
    ASTNode::BinaryOp {
        operator: BinaryOperator::Less,
        left: Box::new(left),
        right: Box::new(right),
        span: span(),
    }
}

fn local(name: &str, value: ASTNode) -> ASTNode {
    ASTNode::Local {
        variables: vec![name.to_owned()],
        initial_values: vec![Some(Box::new(value))],
        declared_type_names: Vec::new(),
        span: span(),
    }
}

fn assign(name: &str, value: ASTNode) -> ASTNode {
    ASTNode::Assignment {
        target: Box::new(variable(name)),
        value: Box::new(value),
        span: span(),
    }
}

fn add(left: ASTNode, right: ASTNode) -> ASTNode {
    ASTNode::BinaryOp {
        operator: BinaryOperator::Add,
        left: Box::new(left),
        right: Box::new(right),
        span: span(),
    }
}

fn if_node(condition: ASTNode, then_body: Vec<ASTNode>, else_body: Option<Vec<ASTNode>>) -> ASTNode {
    ASTNode::If {
        condition: Box::new(condition),
        then_body,
        else_body,
        span: span(),
    }
}

fn loop_node(condition: ASTNode, body: Vec<ASTNode>) -> ASTNode {
    ASTNode::Loop {
        condition: Box::new(condition),
        body,
        span: span(),
    }
}

fn break_node() -> ASTNode {
    ASTNode::Break { span: span() }
}

fn continue_node() -> ASTNode {
    ASTNode::Continue { span: span() }
}

/// One real callable ledger: the parse+resolve+projection pipeline is the only
/// issuer of source site tables, so tests reuse it rather than forging one.
fn real_ledger(source: &str) -> (Rc<RefCell<CallableSemanticLoweringState>>, Vec<ASTNode>) {
    let program = NyashParser::parse_from_string(source).expect("fixture parses");
    let ASTNode::Program { mut statements, .. } = program else {
        panic!("fixture must be a program")
    };
    let function = statements.remove(0);
    let body = match &function {
        ASTNode::FunctionDeclaration { body, .. } => body.clone(),
        _ => panic!("fixture must be a function"),
    };
    let syntax = CallableFunctionSyntaxViewV1::from_function_ast(&function).expect("callable syntax");
    let mut resolver = FunctionSemanticResolverSessionV1::new(7701).expect("resolver");
    let ResolveSelectedCallableForestsOutcomeV1::Complete(forests) = resolver
        .resolve_selected_callable_forests(&[syntax.function()])
        .expect("source forest")
    else {
        panic!("fixture forest unexpectedly deferred")
    };
    let forest = forests.into_vec().pop().expect("source forest root");
    let projection = VerifiedSourceProjectionV1::seal_with_root_profile(
        &function,
        &forest,
        syntax.function().root_profile(),
    )
    .expect("source projection");
    let input = ResolvedFunctionLoweringInputV1::from_exact_parts_without_callable(
        &function,
        &forest,
        &projection,
    )
    .expect("source lowering input");
    let state = CallableSemanticLoweringState::from_exact_source(input).expect("callable ledger");
    (Rc::new(RefCell::new(state)), body)
}

fn function_body_source() -> RawInvocationSourceContextV1 {
    RawInvocationSourceContextV1::Located {
        root: RawInvocationRootLineageV1::ScriptRoot,
        site: SourcePathV1::function_body().node(),
        body_kind: Some(SourceBodyKindV1::Function),
    }
}

fn stmt_segments(input: &CallableLoopSourceStmtInputV1<'_>) -> Vec<SourcePathSegmentV1> {
    match input {
        CallableLoopSourceStmtInputV1::Located { source, .. } => {
            source.site().expect("located stmt").segments().to_vec()
        }
        CallableLoopSourceStmtInputV1::Synthetic(_) => panic!("synthetic stmt carrier"),
    }
}

fn expr_segments(input: &CallableLoopSourceExprInputV1<'_>) -> Vec<SourcePathSegmentV1> {
    match input {
        CallableLoopSourceExprInputV1::Located { source, .. } => {
            source.site().expect("located expr").segments().to_vec()
        }
        CallableLoopSourceExprInputV1::Synthetic(_) => panic!("synthetic expr carrier"),
    }
}

type ProjectedItem<'view> = PartsAssociatedRecipeItemV1<
    CallableLoopSourceStmtInputV1<'view>,
    CallableLoopSourceExprInputV1<'view>,
    CallableLoopSourceBodyInputV1<'view>,
    CallableLoopSourcePartsBlockV1<'view>,
    std::convert::Infallible,
    CallableLoopSourcePartsLoopV0V1<'view>,
>;

fn project<'view, 'ledger: 'view>(
    source: &CallableLoopSourcePartsAssociatedSourceV1<'view, 'ledger>,
    block: &CallableLoopSourcePartsBlockV1<'view>,
    index: usize,
) -> Result<ProjectedItem<'view>, PartsAssociatedSourceErrorV1> {
    source
        .item(block, index)
        .map(|verified| verified.test_parts().1)
}

/// Neutral driver entry: installs the entry values the ledger requires before
/// any materialized read, then drives one co-sealed block through the hooks.
fn drive_block(
    ledger: &Rc<RefCell<CallableSemanticLoweringState>>,
    block: &CallableLoopSourcePartsBlockV1<'_>,
    mode: PartsAssociatedBlockModeV1,
) -> Result<(Vec<LoweredRecipe>, BTreeMap<String, ValueId>), String> {
    let port = CallableLoopSourceExpressionPortV1::new(ledger);
    crate::runtime::ring0::ensure_global_ring0_initialized();
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("t/0".to_owned());
    // Plain `function` declarations seal a lexical `me` receiver binding, so
    // the ledger expects an instance-shaped entry (receiver + 0 params).
    let receiver = builder.alloc_value_for_test();
    builder
        .function_state
        .current_function
        .as_mut()
        .expect("current function")
        .params
        .push(receiver);
    let entry =
        PreparedCallableEntryValuesV1::instance_method(&builder, 0).expect("instance entry values");
    ledger
        .borrow_mut()
        .install_entry_values(&entry)
        .expect("entry install");
    let _scope = LexicalScopeGuard::new(&mut builder);
    let mut bindings = BTreeMap::new();
    let mut carrier_updates = BTreeMap::new();
    let empty = BTreeMap::new();
    let plans = lower_callable_loop_source_parts_block(
        port,
        block,
        mode,
        &mut builder,
        &mut bindings,
        &empty,
        &empty,
        &empty,
        &mut carrier_updates,
        "callable-loop-parts/test",
    )?;
    Ok((plans, bindings))
}

fn drive_recipe(
    ledger: &Rc<RefCell<CallableSemanticLoweringState>>,
    body: &[ASTNode],
    recipe_arena: &RecipeBodies,
    recipe_block: &RecipeBlock,
    mode: PartsAssociatedBlockModeV1,
) -> Result<(Vec<LoweredRecipe>, BTreeMap<String, ValueId>), String> {
    let port = CallableLoopSourceExpressionPortV1::new(ledger);
    let carrier = port
        .body(body, &function_body_source())
        .expect("located body");
    let block = CallableLoopSourcePartsBlockV1::located_body(
        recipe_arena,
        recipe_block,
        carrier,
        &port,
    )
    .expect("co-sealed block");
    drive_block(ledger, &block, mode)
}

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

#[test]
fn driver_lowers_no_exit_stmt_block_through_the_source_port() {
    let (ledger, body) = real_ledger("function t() { local tmp = 0; tmp = tmp + 1 }");
    let recipe = try_build_no_exit_block_recipe(&body, true).expect("no-exit recipe");
    let (plans, bindings) = drive_recipe(
        &ledger,
        &body,
        &recipe.arena,
        &recipe.block,
        PartsAssociatedBlockModeV1::NoExit,
    )
    .expect("no-exit block lowers");
    assert!(!plans.is_empty());
    assert!(bindings.contains_key("tmp"));
}

#[test]
fn driver_lowers_join_if_through_the_source_port() {
    let (ledger, body) = real_ledger(
        "function t() { local tmp = 0; if tmp == 0 { tmp = 1 } else { tmp = 2 } }",
    );
    let recipe = try_build_no_exit_block_recipe(&body, true).expect("no-exit recipe");
    assert!(matches!(
        recipe.block.items[1],
        RecipeItem::IfV2 {
            contract: IfContractKind::Join,
            ..
        }
    ));
    let (plans, bindings) = drive_recipe(
        &ledger,
        &body,
        &recipe.arena,
        &recipe.block,
        PartsAssociatedBlockModeV1::NoExit,
    )
    .expect("join if lowers");
    assert!(!plans.is_empty());
    assert!(bindings.contains_key("tmp"));
}

#[test]
fn driver_lowers_exit_if_under_exit_allowed_mode() {
    // `break` only resolves inside a loop body, so the fixture hosts the
    // exit-if inside `loop(true)` and the carrier is projected through the
    // located `LoopBody` child exactly like production does.
    let (ledger, body) =
        real_ledger("function t() { loop(true) { local tmp = 0; if tmp == 0 { break } } }");
    let port = CallableLoopSourceExpressionPortV1::new(&ledger);
    let function_carrier = port
        .body(&body, &function_body_source())
        .expect("located body");
    let loop_stmt = port
        .body_stmt(&function_carrier, 0)
        .expect("located loop stmt");
    let carrier = port
        .child_body_from_stmt(&loop_stmt, BodyChildRoleV1::LoopBody)
        .expect("located loop body");
    let ASTNode::Loop { body: loop_body, .. } = &body[0] else {
        panic!("fixture loop")
    };
    let recipe = try_build_exit_allowed_block_recipe(loop_body, true).expect("exit-allowed recipe");
    assert!(matches!(
        recipe.block.items[1],
        RecipeItem::IfV2 {
            contract: IfContractKind::ExitOnly {
                mode: IfMode::ExitIf
            },
            ..
        }
    ));
    let block = CallableLoopSourcePartsBlockV1::located_body(
        &recipe.arena,
        &recipe.block,
        carrier,
        &port,
    )
    .expect("co-sealed block");
    let (plans, bindings) = drive_block(
        &ledger,
        &block,
        PartsAssociatedBlockModeV1::ExitAllowed,
    )
    .expect("exit-if lowers");
    assert!(!plans.is_empty());
    assert!(bindings.contains_key("tmp"));
}

#[test]
fn driver_lowers_an_opaque_if_container_via_the_singleton_recipe() {
    let (ledger, body) = real_ledger("function t() { local tmp = 0; if tmp == 0 { tmp = 1 } }");
    // A join-bearing if with no else degrades to an opaque `Stmt` item under
    // the exit-allowed builder; the hooks re-derive the singleton container
    // recipe exactly like the raw return-prelude arm.
    let recipe = try_build_exit_allowed_block_recipe(&body, true).expect("exit-allowed recipe");
    assert!(matches!(recipe.block.items[1], RecipeItem::Stmt(_)));
    let (_, bindings) = drive_recipe(
        &ledger,
        &body,
        &recipe.arena,
        &recipe.block,
        PartsAssociatedBlockModeV1::ExitAllowed,
    )
    .expect("opaque if container lowers through the singleton recipe");
    assert!(bindings.contains_key("tmp"));
}

#[test]
fn driver_rejects_loop_v0_with_a_named_missing_error() {
    let (ledger, body) =
        real_ledger("function t() { local tmp = 0; loop(tmp < 3) { tmp = tmp + 1 } }");
    let recipe = try_build_no_exit_block_recipe(&body, true).expect("no-exit recipe");
    assert!(matches!(
        recipe.block.items[1],
        RecipeItem::LoopV0 { .. }
    ));
    let error = drive_recipe(
        &ledger,
        &body,
        &recipe.arena,
        &recipe.block,
        PartsAssociatedBlockModeV1::NoExit,
    )
    .expect_err("LoopV0 must stay a named reject in this slice");
    assert!(
        error.contains("loop-v0-source-lowering-missing"),
        "{error}"
    );
}
