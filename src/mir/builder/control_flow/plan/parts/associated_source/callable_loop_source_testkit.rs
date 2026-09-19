//! Shared fixtures for the located callable-loop associated-source tests:
//! AST builders, a real parse+resolve+projection ledger, located carrier
//! helpers, and the neutral block driver. Kept separate so the provider and
//! driver test files each stay below the split threshold.

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
use crate::mir::builder::control_flow::plan::recipe_tree::{RecipeBlock, RecipeBodies};
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
    CallableFunctionSyntaxViewV1, FunctionSemanticResolverSessionV1,
    ResolveSelectedCallableForestsOutcomeV1, SourceBodyKindV1, SourcePathSegmentV1, SourcePathV1,
};
use crate::mir::ValueId;
use crate::parser::NyashParser;

pub(super) fn span() -> Span {
    Span::unknown()
}

pub(super) fn integer(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: span(),
    }
}

pub(super) fn variable(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.to_owned(),
        span: span(),
    }
}

pub(super) fn less_than(left: ASTNode, right: ASTNode) -> ASTNode {
    ASTNode::BinaryOp {
        operator: BinaryOperator::Less,
        left: Box::new(left),
        right: Box::new(right),
        span: span(),
    }
}

pub(super) fn local(name: &str, value: ASTNode) -> ASTNode {
    ASTNode::Local {
        variables: vec![name.to_owned()],
        initial_values: vec![Some(Box::new(value))],
        declared_type_names: Vec::new(),
        span: span(),
    }
}

pub(super) fn assign(name: &str, value: ASTNode) -> ASTNode {
    ASTNode::Assignment {
        target: Box::new(variable(name)),
        value: Box::new(value),
        span: span(),
    }
}

pub(super) fn add(left: ASTNode, right: ASTNode) -> ASTNode {
    ASTNode::BinaryOp {
        operator: BinaryOperator::Add,
        left: Box::new(left),
        right: Box::new(right),
        span: span(),
    }
}

pub(super) fn if_node(
    condition: ASTNode,
    then_body: Vec<ASTNode>,
    else_body: Option<Vec<ASTNode>>,
) -> ASTNode {
    ASTNode::If {
        condition: Box::new(condition),
        then_body,
        else_body,
        span: span(),
    }
}

pub(super) fn loop_node(condition: ASTNode, body: Vec<ASTNode>) -> ASTNode {
    ASTNode::Loop {
        condition: Box::new(condition),
        body,
        span: span(),
    }
}

pub(super) fn break_node() -> ASTNode {
    ASTNode::Break { span: span() }
}

pub(super) fn continue_node() -> ASTNode {
    ASTNode::Continue { span: span() }
}

/// One real callable ledger: the parse+resolve+projection pipeline is the only
/// issuer of source site tables, so tests reuse it rather than forging one.
pub(super) fn real_ledger(
    source: &str,
) -> (Rc<RefCell<CallableSemanticLoweringState>>, Vec<ASTNode>) {
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

pub(super) fn function_body_source() -> RawInvocationSourceContextV1 {
    RawInvocationSourceContextV1::Located {
        root: RawInvocationRootLineageV1::ScriptRoot,
        site: SourcePathV1::function_body().node(),
        body_kind: Some(SourceBodyKindV1::Function),
    }
}

pub(super) fn stmt_segments(input: &CallableLoopSourceStmtInputV1<'_>) -> Vec<SourcePathSegmentV1> {
    match input {
        CallableLoopSourceStmtInputV1::Located { source, .. } => {
            source.site().expect("located stmt").segments().to_vec()
        }
        CallableLoopSourceStmtInputV1::Synthetic(_) => panic!("synthetic stmt carrier"),
    }
}

pub(super) fn expr_segments(input: &CallableLoopSourceExprInputV1<'_>) -> Vec<SourcePathSegmentV1> {
    match input {
        CallableLoopSourceExprInputV1::Located { source, .. } => {
            source.site().expect("located expr").segments().to_vec()
        }
        CallableLoopSourceExprInputV1::Synthetic(_) => panic!("synthetic expr carrier"),
    }
}

pub(super) type ProjectedItem<'view> = PartsAssociatedRecipeItemV1<
    CallableLoopSourceStmtInputV1<'view>,
    CallableLoopSourceExprInputV1<'view>,
    CallableLoopSourceBodyInputV1<'view>,
    CallableLoopSourcePartsBlockV1<'view>,
    std::convert::Infallible,
    CallableLoopSourcePartsLoopV0V1<'view>,
>;

pub(super) fn project<'view, 'ledger: 'view>(
    source: &CallableLoopSourcePartsAssociatedSourceV1<'view, 'ledger>,
    block: &CallableLoopSourcePartsBlockV1<'view>,
    index: usize,
) -> Result<ProjectedItem<'view>, PartsAssociatedSourceErrorV1> {
    source
        .item(block, index)
        .map(|verified| verified.test_parts().1)
}

/// One test builder whose callable shape matches the resolved ledger: plain
/// `function` declarations seal a lexical `me` receiver binding, so the entry
/// is instance-shaped (receiver + 0 params) and installed before any
/// materialized read.
pub(super) fn test_builder(
    ledger: &Rc<RefCell<CallableSemanticLoweringState>>,
) -> MirBuilder {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("t/0".to_owned());
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
    builder
}

/// Neutral driver entry: installs the entry values the ledger requires before
/// any materialized read, then drives one co-sealed block through the hooks.
pub(super) fn drive_block(
    ledger: &Rc<RefCell<CallableSemanticLoweringState>>,
    block: &CallableLoopSourcePartsBlockV1<'_>,
    mode: PartsAssociatedBlockModeV1,
) -> Result<(Vec<LoweredRecipe>, BTreeMap<String, ValueId>), String> {
    let port = CallableLoopSourceExpressionPortV1::new(ledger);
    let mut builder = test_builder(ledger);
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

pub(super) fn drive_recipe(
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
