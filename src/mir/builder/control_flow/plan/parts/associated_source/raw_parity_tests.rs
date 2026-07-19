//! RAW0 parity proof for the associated-source block driver.

use std::collections::BTreeMap;

use crate::ast::{ASTNode, LiteralValue, Span};
use crate::mir::builder::control_flow::facts::canon::cond_block_view::CondBlockView;
use crate::mir::builder::control_flow::plan::parts::dispatch::block::BoxedLowerStmtFn;
use crate::mir::builder::control_flow::plan::parts::dispatch::{
    lower_block_internal, plans_exit_on_all_paths, BlockKindInternal,
};
use crate::mir::builder::control_flow::plan::recipe_tree::{
    ExitKind, IfContractKind, IfMode, RecipeBlock, RecipeBodies, RecipeItem,
};
use crate::mir::builder::control_flow::plan::{CoreExitPlan, CorePlan, LoweredRecipe};
use crate::mir::builder::control_flow::recipes::{refs::StmtRef, RecipeBody};
use crate::mir::builder::stmts::variable_stmt::build_local_statement;
use crate::mir::builder::vars::lexical_scope::LexicalScopeGuard;
use crate::mir::builder::MirBuilder;
use crate::mir::{MirType, ValueId};

use super::super::entry;

#[derive(Debug, PartialEq, Eq)]
struct NormalizedRawLoweringV1 {
    plans: String,
    current_bindings: BTreeMap<String, ValueId>,
    variable_map: BTreeMap<String, ValueId>,
    value_types: BTreeMap<ValueId, MirType>,
    exits_on_all_paths: bool,
}

fn normalized(
    builder: &MirBuilder,
    current_bindings: &BTreeMap<String, ValueId>,
    plans: &[LoweredRecipe],
) -> NormalizedRawLoweringV1 {
    NormalizedRawLoweringV1 {
        plans: format!("{plans:#?}"),
        current_bindings: current_bindings.clone(),
        variable_map: builder.function_state.variable_ctx.variable_map.clone(),
        value_types: builder.function_state.type_ctx.value_types.clone(),
        exits_on_all_paths: plans_exit_on_all_paths(plans),
    }
}

fn literal_int(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: Span::unknown(),
    }
}

fn literal_bool(value: bool) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Bool(value),
        span: Span::unknown(),
    }
}

fn variable(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.to_string(),
        span: Span::unknown(),
    }
}

fn assignment(name: &str, value: i64) -> ASTNode {
    ASTNode::Assignment {
        target: Box::new(variable(name)),
        value: Box::new(literal_int(value)),
        span: Span::unknown(),
    }
}

fn local(name: &str, value: i64) -> ASTNode {
    ASTNode::Local {
        variables: vec![name.to_string()],
        initial_values: vec![Some(Box::new(literal_int(value)))],
        declared_type_names: Vec::new(),
        span: Span::unknown(),
    }
}

fn return_value(value: i64) -> ASTNode {
    ASTNode::Return {
        value: Some(Box::new(literal_int(value))),
        span: Span::unknown(),
    }
}

fn exit_only_fixture() -> (RecipeBodies, RecipeBlock) {
    let returning = return_value(7);
    let mut arena = RecipeBodies::new();
    let body = arena.register(RecipeBody::new(vec![returning]));
    let block = RecipeBlock::new(
        body,
        vec![RecipeItem::Exit {
            kind: ExitKind::Return,
            stmt: StmtRef::new(0),
        }],
    );
    (arena, block)
}

fn exit_allowed_fixture() -> (RecipeBodies, RecipeBlock) {
    let condition = literal_bool(true);
    let returning = return_value(11);
    let source_if = ASTNode::If {
        condition: Box::new(condition.clone()),
        then_body: vec![returning.clone()],
        else_body: Some(Vec::new()),
        span: Span::unknown(),
    };

    let mut arena = RecipeBodies::new();
    let then_body = arena.register(RecipeBody::new(vec![returning]));
    let then_block = RecipeBlock::new(
        then_body,
        vec![RecipeItem::Exit {
            kind: ExitKind::Return,
            stmt: StmtRef::new(0),
        }],
    );
    let else_body = arena.register(RecipeBody::new(Vec::new()));
    let else_block = RecipeBlock::new(else_body, Vec::new());
    let root_body = arena.register(RecipeBody::new(vec![source_if]));
    let root = RecipeBlock::new(
        root_body,
        vec![RecipeItem::IfV2 {
            if_stmt: StmtRef::new(0),
            cond_view: CondBlockView::from_expr(&condition),
            contract: IfContractKind::ExitAllowed {
                mode: IfMode::ThenOnlyExit,
            },
            then_block: Box::new(then_block),
            else_block: Some(Box::new(else_block)),
        }],
    );
    (arena, root)
}

fn no_exit_join_fixture() -> (RecipeBodies, RecipeBlock) {
    let condition = literal_bool(true);
    let then_assignment = assignment("value", 1);
    let else_assignment = assignment("value", 2);
    let source_if = ASTNode::If {
        condition: Box::new(condition.clone()),
        then_body: vec![then_assignment.clone()],
        else_body: Some(vec![else_assignment.clone()]),
        span: Span::unknown(),
    };

    let mut arena = RecipeBodies::new();
    let then_body = arena.register(RecipeBody::new(vec![then_assignment]));
    let then_block = RecipeBlock::new(then_body, vec![RecipeItem::Stmt(StmtRef::new(0))]);
    let else_body = arena.register(RecipeBody::new(vec![else_assignment]));
    let else_block = RecipeBlock::new(else_body, vec![RecipeItem::Stmt(StmtRef::new(0))]);
    let root_body = arena.register(RecipeBody::new(vec![source_if]));
    let root = RecipeBlock::new(
        root_body,
        vec![RecipeItem::IfV2 {
            if_stmt: StmtRef::new(0),
            cond_view: CondBlockView::from_expr(&condition),
            contract: IfContractKind::Join,
            then_block: Box::new(then_block),
            else_block: Some(Box::new(else_block)),
        }],
    );
    (arena, root)
}

fn stmt_only_fixture() -> (RecipeBodies, RecipeBlock) {
    let mut arena = RecipeBodies::new();
    let body = arena.register(RecipeBody::new(vec![local("item", 5)]));
    let block = RecipeBlock::new(body, vec![RecipeItem::Stmt(StmtRef::new(0))]);
    (arena, block)
}

fn fresh_builder(name: &str) -> MirBuilder {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test(name.to_string());
    builder
}

#[test]
fn raw_exit_only_facade_matches_associated_block_driver() {
    let (arena, block) = exit_only_fixture();
    let empty = BTreeMap::new();

    let mut facade_builder = fresh_builder("raw_exit_only_facade/0");
    let mut facade_bindings = BTreeMap::new();
    let verified = entry::verify_exit_only_block_with_pre(
        &arena,
        &block,
        "raw_exit_only_facade",
        Some(&facade_bindings),
    )
    .expect("exit-only fixture verifies");
    let facade_plans = entry::lower_exit_only_block_verified(
        &mut facade_builder,
        &mut facade_bindings,
        &empty,
        &empty,
        verified,
        "raw_exit_only_facade",
    )
    .expect("facade lowers exit-only fixture");

    let mut driver_builder = fresh_builder("raw_exit_only_facade/0");
    let mut driver_bindings = BTreeMap::new();
    let driver_plans = lower_block_internal(
        &mut driver_builder,
        &mut driver_bindings,
        &empty,
        &arena,
        &block,
        "raw_exit_only_facade",
        BlockKindInternal::ExitOnly {
            break_phi_dsts: &empty,
        },
    )
    .expect("associated driver lowers exit-only fixture");

    assert_eq!(
        normalized(&facade_builder, &facade_bindings, &facade_plans),
        normalized(&driver_builder, &driver_bindings, &driver_plans)
    );
    assert!(plans_exit_on_all_paths(&driver_plans));
    assert!(matches!(
        driver_plans.last(),
        Some(CorePlan::Exit(CoreExitPlan::Return(Some(_))))
    ));
    assert!(driver_bindings.is_empty());
    assert!(driver_builder
        .function_state
        .variable_ctx
        .variable_map
        .is_empty());
}

#[test]
fn raw_exit_allowed_facade_matches_associated_block_driver() {
    let (arena, block) = exit_allowed_fixture();
    let empty = BTreeMap::new();

    let mut facade_builder = fresh_builder("raw_exit_allowed_facade/0");
    let mut facade_bindings = BTreeMap::new();
    let facade_plans = entry::lower_exit_allowed_block(
        &mut facade_builder,
        &mut facade_bindings,
        &empty,
        &empty,
        &arena,
        &block,
        "raw_exit_allowed_facade",
    )
    .expect("facade lowers exit-allowed fixture");

    let mut driver_builder = fresh_builder("raw_exit_allowed_facade/0");
    let mut driver_bindings = BTreeMap::new();
    let driver_plans = lower_block_internal(
        &mut driver_builder,
        &mut driver_bindings,
        &empty,
        &arena,
        &block,
        "raw_exit_allowed_facade",
        BlockKindInternal::ExitAllowed {
            break_phi_dsts: &empty,
        },
    )
    .expect("associated driver lowers exit-allowed fixture");

    assert_eq!(
        normalized(&facade_builder, &facade_bindings, &facade_plans),
        normalized(&driver_builder, &driver_bindings, &driver_plans)
    );
    assert!(!plans_exit_on_all_paths(&driver_plans));
    let if_plan = driver_plans
        .iter()
        .find_map(|plan| match plan {
            CorePlan::If(plan) => Some(plan),
            _ => None,
        })
        .expect("exit-allowed golden shape contains one If");
    assert!(matches!(
        if_plan.then_plans.last(),
        Some(CorePlan::Exit(CoreExitPlan::Return(Some(_))))
    ));
    assert!(if_plan
        .else_plans
        .as_ref()
        .is_some_and(|plans| !plans_exit_on_all_paths(plans)));
    assert!(if_plan.joins.is_empty());
    assert!(driver_bindings.is_empty());
    assert!(driver_builder
        .function_state
        .variable_ctx
        .variable_map
        .is_empty());
}

#[test]
fn raw_no_exit_join_facade_matches_associated_block_driver() {
    let (arena, block) = no_exit_join_fixture();
    let empty = BTreeMap::new();

    let mut facade_builder = fresh_builder("raw_no_exit_join_facade/0");
    let _facade_scope = LexicalScopeGuard::new(&mut facade_builder);
    build_local_statement(
        &mut facade_builder,
        vec!["value".to_string()],
        vec![Some(Box::new(literal_int(0)))],
        Vec::new(),
    )
    .expect("seed facade binding");
    let mut facade_bindings = facade_builder
        .function_state
        .variable_ctx
        .variable_map
        .clone();
    let facade_plans = entry::lower_no_exit_block(
        &mut facade_builder,
        &mut facade_bindings,
        &empty,
        None,
        &arena,
        &block,
        "raw_no_exit_join_facade",
    )
    .expect("facade lowers no-exit Join fixture");

    let mut driver_builder = fresh_builder("raw_no_exit_join_facade/0");
    let _driver_scope = LexicalScopeGuard::new(&mut driver_builder);
    build_local_statement(
        &mut driver_builder,
        vec!["value".to_string()],
        vec![Some(Box::new(literal_int(0)))],
        Vec::new(),
    )
    .expect("seed driver binding");
    let mut driver_bindings = driver_builder
        .function_state
        .variable_ctx
        .variable_map
        .clone();
    let mut make_lower_stmt = || -> BoxedLowerStmtFn<'_> {
        Box::new(
            |builder, bindings, carrier_step_phis, break_phi_dsts, stmt, error_prefix| {
                super::super::stmt::lower_return_prelude_stmt(
                    builder,
                    bindings,
                    carrier_step_phis,
                    break_phi_dsts,
                    stmt,
                    error_prefix,
                )
            },
        )
    };
    let should_update_binding =
        |name: &str, bindings: &BTreeMap<String, ValueId>| bindings.contains_key(name);
    let driver_plans = lower_block_internal(
        &mut driver_builder,
        &mut driver_bindings,
        &empty,
        &arena,
        &block,
        "raw_no_exit_join_facade",
        BlockKindInternal::NoExit {
            break_phi_dsts: None,
            make_lower_stmt: &mut make_lower_stmt,
            should_update_binding: &should_update_binding,
        },
    )
    .expect("associated driver lowers no-exit Join fixture");

    assert_eq!(
        normalized(&facade_builder, &facade_bindings, &facade_plans),
        normalized(&driver_builder, &driver_bindings, &driver_plans)
    );
    assert!(!plans_exit_on_all_paths(&driver_plans));
    let if_plan = driver_plans
        .iter()
        .find_map(|plan| match plan {
            CorePlan::If(plan) => Some(plan),
            _ => None,
        })
        .expect("no-exit golden shape contains one If");
    assert_eq!(if_plan.joins.len(), 1);
    assert_eq!(if_plan.joins[0].name, "value");
    assert_eq!(driver_bindings.get("value"), Some(&if_plan.joins[0].dst));
    assert_eq!(
        driver_builder
            .function_state
            .variable_ctx
            .variable_map
            .get("value"),
        Some(&if_plan.joins[0].dst)
    );
}

#[test]
fn raw_stmt_only_facade_matches_associated_block_driver_and_golden_state() {
    let (arena, block) = stmt_only_fixture();
    let empty = BTreeMap::new();

    let mut facade_builder = fresh_builder("raw_stmt_only_facade/0");
    let _facade_scope = LexicalScopeGuard::new(&mut facade_builder);
    let mut facade_bindings = BTreeMap::new();
    let verified = entry::verify_stmt_only_block_with_pre(
        &arena,
        &block,
        "raw_stmt_only_facade",
        Some(&facade_bindings),
    )
    .expect("stmt-only fixture verifies");
    let facade_plans = entry::lower_stmt_only_block_verified(
        &mut facade_builder,
        &mut facade_bindings,
        &empty,
        None,
        verified,
        "raw_stmt_only_facade",
        super::super::stmt::lower_return_prelude_stmt,
    )
    .expect("facade lowers stmt-only fixture");

    let mut driver_builder = fresh_builder("raw_stmt_only_facade/0");
    let _driver_scope = LexicalScopeGuard::new(&mut driver_builder);
    let mut driver_bindings = BTreeMap::new();
    let mut lower_stmt = super::super::stmt::lower_return_prelude_stmt;
    let driver_plans = lower_block_internal(
        &mut driver_builder,
        &mut driver_bindings,
        &empty,
        &arena,
        &block,
        "raw_stmt_only_facade",
        BlockKindInternal::StmtOnly {
            break_phi_dsts: None,
            lower_stmt: &mut lower_stmt,
        },
    )
    .expect("associated driver lowers stmt-only fixture");

    assert_eq!(
        normalized(&facade_builder, &facade_bindings, &facade_plans),
        normalized(&driver_builder, &driver_bindings, &driver_plans)
    );
    assert!(!plans_exit_on_all_paths(&driver_plans));
    let item = driver_bindings
        .get("item")
        .copied()
        .expect("stmt-only golden state publishes local binding");
    assert_eq!(
        driver_builder
            .function_state
            .variable_ctx
            .variable_map
            .get("item"),
        Some(&item)
    );
    assert_eq!(
        driver_builder
            .function_state
            .type_ctx
            .value_types
            .get(&item),
        Some(&MirType::Integer)
    );
}
