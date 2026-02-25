//! Block lowering - core dispatch logic for RecipeBlocks.
//!
//! Contains:
//! - Type aliases for stmt lowering functions
//! - BlockKindInternal enum for dispatch
//! - Core lower_block_internal function
//! - Block-level lowering entry points (exit-only, exit-allowed, stmt-only, no-exit)

use crate::mir::builder::control_flow::plan::recipe_tree::{
    IfContractKind, RecipeBodies, RecipeBlock, RecipeItem,
};
use crate::mir::builder::control_flow::plan::recipe_tree::verified::VerifiedRecipeBlock;
use crate::mir::builder::control_flow::plan::{CorePlan, LoweredRecipe};
use crate::mir::builder::MirBuilder;
use super::super::stmt as parts_stmt;
#[cfg(debug_assertions)]
use super::super::verify;
use std::collections::BTreeMap;

use super::if_exit_only::lower_exit_only_item;
use super::if_join::lower_if_join_with_stmt_lowerer;

// ============================================================================
// Type aliases
// ============================================================================

pub(super) type LowerStmtFn<'a> = dyn FnMut(
    &mut MirBuilder,
    &mut BTreeMap<String, crate::mir::ValueId>,
    &BTreeMap<String, crate::mir::ValueId>,
    Option<&BTreeMap<String, crate::mir::ValueId>>,
    &crate::ast::ASTNode,
    &str,
) -> Result<Vec<LoweredRecipe>, String>
    + 'a;

pub(super) type BoxedLowerStmtFn<'a> = Box<LowerStmtFn<'a>>;

// ============================================================================
// BlockKindInternal - dispatch enum
// ============================================================================

pub(super) enum BlockKindInternal<'a> {
    ExitOnly {
        break_phi_dsts: &'a BTreeMap<String, crate::mir::ValueId>,
    },
    StmtOnly {
        break_phi_dsts: Option<&'a BTreeMap<String, crate::mir::ValueId>>,
        lower_stmt: &'a mut LowerStmtFn<'a>,
    },
    NoExit {
        break_phi_dsts: Option<&'a BTreeMap<String, crate::mir::ValueId>>,
        make_lower_stmt: &'a mut dyn FnMut() -> BoxedLowerStmtFn<'a>,
        should_update_binding: &'a dyn Fn(&str, &BTreeMap<String, crate::mir::ValueId>) -> bool,
    },
}

// ============================================================================
// Core block lowering
// ============================================================================

pub(super) fn lower_block_internal<'a>(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    arena: &RecipeBodies,
    block: &RecipeBlock,
    error_prefix: &str,
    kind: BlockKindInternal<'a>,
) -> Result<Vec<LoweredRecipe>, String> {
    arena.get(block.body_id).ok_or_else(|| {
        format!("[freeze:contract][recipe] invalid_body_id: ctx={}", error_prefix)
    })?;

    match kind {
        BlockKindInternal::ExitOnly { break_phi_dsts } => {
            #[cfg(debug_assertions)]
            verify::debug_check_block_contract(arena, block, error_prefix)?;

            let mut plans = Vec::new();
            for item in &block.items {
                plans.extend(lower_exit_only_item(
                    builder,
                    current_bindings,
                    carrier_step_phis,
                    break_phi_dsts,
                    arena,
                    block.body_id,
                    item,
                    error_prefix,
                )?);
            }

            if !plans_exit_on_all_paths(&plans) {
                return Err(format!(
                    "[freeze:contract][recipe] exit_only_block_must_end_with_exit: ctx={}",
                    error_prefix
                ));
            }

            Ok(plans)
        }
        BlockKindInternal::StmtOnly {
            break_phi_dsts,
            lower_stmt,
        } => {
            #[cfg(debug_assertions)]
            verify::debug_check_stmt_only_block_contract(arena, block, error_prefix)?;

            let body = arena.get(block.body_id).ok_or_else(|| {
                format!("[freeze:contract][recipe] invalid_body_id: ctx={}", error_prefix)
            })?;

            let mut plans = Vec::new();
            for item in &block.items {
                let RecipeItem::Stmt(stmt_ref) = item else {
                    return Err(format!(
                        "[freeze:contract][recipe] stmt_only_block_contains_non_stmt_item: ctx={}",
                        error_prefix
                    ));
                };
                let stmt = body.get_ref(*stmt_ref).ok_or_else(|| {
                    format!("{}: missing stmt idx={}", error_prefix, stmt_ref.index())
                })?;
                plans.extend(lower_stmt(
                    builder,
                    current_bindings,
                    carrier_step_phis,
                    break_phi_dsts,
                    stmt,
                    error_prefix,
                )?);
            }

            Ok(plans)
        }
        BlockKindInternal::NoExit {
            break_phi_dsts,
            make_lower_stmt,
            should_update_binding,
        } => {
            #[cfg(debug_assertions)]
            verify::debug_check_no_exit_block_contract(arena, block, error_prefix)?;

            let body = arena.get(block.body_id).ok_or_else(|| {
                format!("[freeze:contract][recipe] invalid_body_id: ctx={}", error_prefix)
            })?;

            let mut lower_stmt_outer = make_lower_stmt();

            let mut plans = Vec::new();
            for item in &block.items {
                #[allow(unreachable_patterns)]
                match item {
                    RecipeItem::Stmt(stmt_ref) => {
                        let stmt = body.get_ref(*stmt_ref).ok_or_else(|| {
                            format!("{}: missing stmt idx={}", error_prefix, stmt_ref.index())
                        })?;
                        plans.extend(lower_stmt_outer(
                            builder,
                            current_bindings,
                            carrier_step_phis,
                            break_phi_dsts,
                            stmt,
                            error_prefix,
                        )?);
                    }
                    RecipeItem::IfV2 {
                        if_stmt: _,
                        cond_view,
                        contract,
                        then_block,
                        else_block,
                    } => match contract {
                        IfContractKind::Join => {
                            plans.extend(lower_if_join_with_stmt_lowerer(
                                builder,
                                current_bindings,
                                carrier_step_phis,
                                break_phi_dsts,
                                arena,
                                cond_view,
                                then_block,
                                else_block.as_deref(),
                                error_prefix,
                                make_lower_stmt,
                                should_update_binding,
                            )?);
                        }
                        _ => {
                            return Err(format!(
                                "[freeze:contract][recipe] dispatch_saw_unsupported_item: ctx={}",
                                error_prefix
                            ));
                        }
                    },
                    RecipeItem::LoopV0 {
                        cond_view,
                        body_block,
                        body_contract,
                        ..
                    } => {
                        let plan = super::super::loop_::lower_loop_v0(
                            builder,
                            current_bindings,
                            cond_view,
                            *body_contract,
                            arena,
                            body_block,
                            error_prefix,
                        )?;
                        plans.push(plan);
                    }
                    _ => {
                        return Err(format!(
                            "[freeze:contract][recipe] dispatch_saw_unsupported_item: ctx={}",
                            error_prefix
                        ));
                    }
                }
            }

            Ok(plans)
        }
    }
}

// ============================================================================
// Exit path helpers
// ============================================================================

pub(super) fn plans_exit_on_all_paths(plans: &[LoweredRecipe]) -> bool {
    plans.last().is_some_and(core_plan_exits_on_all_paths)
}

fn core_plan_exits_on_all_paths(plan: &LoweredRecipe) -> bool {
    match plan {
        CorePlan::Exit(_) => true,
        CorePlan::If(if_plan) => {
            plans_exit_on_all_paths(&if_plan.then_plans)
                && if_plan
                    .else_plans
                    .as_ref()
                    .is_some_and(|p| plans_exit_on_all_paths(p))
        }
        CorePlan::BranchN(branch) => {
            branch
                .arms
                .iter()
                .all(|arm| plans_exit_on_all_paths(&arm.plans))
                && branch
                    .else_plans
                    .as_ref()
                    .is_some_and(|p| plans_exit_on_all_paths(p))
        }
        CorePlan::Seq(inner) => plans_exit_on_all_paths(inner),
        CorePlan::Effect(_) | CorePlan::Loop(_) => false,
    }
}

// ============================================================================
// Block lowering entry points
// ============================================================================

/// Lower exit-only RecipeBlock via arena.
pub(super) fn lower_exit_only_block(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    break_phi_dsts: &BTreeMap<String, crate::mir::ValueId>,
    arena: &RecipeBodies,
    block: &RecipeBlock,
    error_prefix: &str,
) -> Result<Vec<LoweredRecipe>, String> {
    lower_block_internal(
        builder,
        current_bindings,
        carrier_step_phis,
        arena,
        block,
        error_prefix,
        BlockKindInternal::ExitOnly { break_phi_dsts },
    )
}

/// Lower an already-verified exit-only block (Verifier-gated entry).
pub(in crate::mir::builder) fn lower_exit_only_block_verified(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    break_phi_dsts: &BTreeMap<String, crate::mir::ValueId>,
    verified: VerifiedRecipeBlock<'_>,
    error_prefix: &str,
) -> Result<Vec<LoweredRecipe>, String> {
    lower_exit_only_block(
        builder,
        current_bindings,
        carrier_step_phis,
        break_phi_dsts,
        verified.arena(),
        verified.block(),
        error_prefix,
    )
}

/// Lower a block that is allowed to contain exit items, but does not require
/// "exit on all paths".
pub(super) fn lower_exit_allowed_block(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    break_phi_dsts: &BTreeMap<String, crate::mir::ValueId>,
    arena: &RecipeBodies,
    block: &RecipeBlock,
    error_prefix: &str,
) -> Result<Vec<LoweredRecipe>, String> {
    arena.get(block.body_id).ok_or_else(|| {
        format!("[freeze:contract][recipe] invalid_body_id: ctx={}", error_prefix)
    })?;

    #[cfg(debug_assertions)]
    verify::debug_check_block_contract(arena, block, error_prefix)?;

    let mut plans = Vec::new();
    for item in &block.items {
        plans.extend(lower_exit_only_item(
            builder,
            current_bindings,
            carrier_step_phis,
            break_phi_dsts,
            arena,
            block.body_id,
            item,
            error_prefix,
        )?);
    }

    Ok(plans)
}

/// Lower an already-verified exit-allowed block (Verifier-gated entry).
pub(in crate::mir::builder) fn lower_exit_allowed_block_verified(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    break_phi_dsts: &BTreeMap<String, crate::mir::ValueId>,
    verified: VerifiedRecipeBlock<'_>,
    error_prefix: &str,
) -> Result<Vec<LoweredRecipe>, String> {
    lower_exit_allowed_block(
        builder,
        current_bindings,
        carrier_step_phis,
        break_phi_dsts,
        verified.arena(),
        verified.block(),
        error_prefix,
    )
}

/// Lower statement-only RecipeBlock via arena.
///
/// Contract:
/// - Only `RecipeItem::Stmt` is allowed.
/// - No "must end with Exit" condition is enforced here.
pub(in crate::mir::builder) fn lower_stmt_only_block<LowerStmt>(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    break_phi_dsts: Option<&BTreeMap<String, crate::mir::ValueId>>,
    arena: &RecipeBodies,
    block: &RecipeBlock,
    error_prefix: &str,
    mut lower_stmt: LowerStmt,
) -> Result<Vec<LoweredRecipe>, String>
where
    LowerStmt: FnMut(
        &mut MirBuilder,
        &mut BTreeMap<String, crate::mir::ValueId>,
        &BTreeMap<String, crate::mir::ValueId>,
        Option<&BTreeMap<String, crate::mir::ValueId>>,
        &crate::ast::ASTNode,
        &str,
    ) -> Result<Vec<LoweredRecipe>, String>,
{
    lower_block_internal(
        builder,
        current_bindings,
        carrier_step_phis,
        arena,
        block,
        error_prefix,
        BlockKindInternal::StmtOnly {
            break_phi_dsts,
            lower_stmt: &mut lower_stmt,
        },
    )
}

/// Lower non-exit RecipeBlock (Stmt / IfV2{Join} only).
///
/// Contract:
/// - Only `RecipeItem::Stmt` and `RecipeItem::IfV2{ contract: Join, .. }` are allowed.
/// - Join-bearing if branches are lowered via `lower_stmt_only_block` and joined with PHI payload.
fn lower_no_exit_block(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    break_phi_dsts: Option<&BTreeMap<String, crate::mir::ValueId>>,
    arena: &RecipeBodies,
    block: &RecipeBlock,
    error_prefix: &str,
) -> Result<Vec<LoweredRecipe>, String> {
    lower_no_exit_block_with_stmt_lowerer(
        builder,
        current_bindings,
        carrier_step_phis,
        break_phi_dsts,
        arena,
        block,
        error_prefix,
        || {
            |builder, bindings, carrier_step_phis, break_phi_dsts, stmt, error_prefix| {
                parts_stmt::lower_return_prelude_stmt(
                    builder,
                    bindings,
                    carrier_step_phis,
                    break_phi_dsts,
                    stmt,
                    error_prefix,
                )
            }
        },
        |name, bindings| bindings.contains_key(name),
    )
}

/// Lower an already-verified non-exit block (Verifier-gated entry).
pub(in crate::mir::builder) fn lower_no_exit_block_verified(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    break_phi_dsts: Option<&BTreeMap<String, crate::mir::ValueId>>,
    verified: VerifiedRecipeBlock<'_>,
    error_prefix: &str,
) -> Result<Vec<LoweredRecipe>, String> {
    lower_no_exit_block(
        builder,
        current_bindings,
        carrier_step_phis,
        break_phi_dsts,
        verified.arena(),
        verified.block(),
        error_prefix,
    )
}

/// Lower non-exit RecipeBlock (Stmt / IfJoin only), with injected lowering and join-update policy.
///
/// - `make_lower_stmt` is called once per block lowering context (outer, then, else) to allow
///   block-local state (e.g., carrier_updates map) without cross-branch leakage.
/// - `should_update_binding` controls which joins update `current_bindings`.
fn lower_no_exit_block_with_stmt_lowerer<
    MakeLowerStmt,
    LowerStmt,
    ShouldUpdateBinding,
>(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    break_phi_dsts: Option<&BTreeMap<String, crate::mir::ValueId>>,
    arena: &RecipeBodies,
    block: &RecipeBlock,
    error_prefix: &str,
    mut make_lower_stmt: MakeLowerStmt,
    should_update_binding: ShouldUpdateBinding,
) -> Result<Vec<LoweredRecipe>, String>
where
    MakeLowerStmt: FnMut() -> LowerStmt,
    LowerStmt: FnMut(
        &mut MirBuilder,
        &mut BTreeMap<String, crate::mir::ValueId>,
        &BTreeMap<String, crate::mir::ValueId>,
        Option<&BTreeMap<String, crate::mir::ValueId>>,
        &crate::ast::ASTNode,
        &str,
    ) -> Result<Vec<LoweredRecipe>, String>,
    ShouldUpdateBinding: Fn(&str, &BTreeMap<String, crate::mir::ValueId>) -> bool,
{
    let mut make_lower_stmt_boxed = || -> BoxedLowerStmtFn<'_> { Box::new(make_lower_stmt()) };

    lower_block_internal(
        builder,
        current_bindings,
        carrier_step_phis,
        arena,
        block,
        error_prefix,
        BlockKindInternal::NoExit {
            break_phi_dsts,
            make_lower_stmt: &mut make_lower_stmt_boxed,
            should_update_binding: &should_update_binding,
        },
    )
}

/// Lower an already-verified non-exit block with injected lowering (Verifier-gated entry).
pub(in crate::mir::builder) fn lower_no_exit_block_with_stmt_lowerer_verified<
    MakeLowerStmt,
    LowerStmt,
    ShouldUpdateBinding,
>(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    break_phi_dsts: Option<&BTreeMap<String, crate::mir::ValueId>>,
    verified: VerifiedRecipeBlock<'_>,
    error_prefix: &str,
    make_lower_stmt: MakeLowerStmt,
    should_update_binding: ShouldUpdateBinding,
) -> Result<Vec<LoweredRecipe>, String>
where
    MakeLowerStmt: FnMut() -> LowerStmt,
    LowerStmt: FnMut(
        &mut MirBuilder,
        &mut BTreeMap<String, crate::mir::ValueId>,
        &BTreeMap<String, crate::mir::ValueId>,
        Option<&BTreeMap<String, crate::mir::ValueId>>,
        &crate::ast::ASTNode,
        &str,
    ) -> Result<Vec<LoweredRecipe>, String>,
    ShouldUpdateBinding: Fn(&str, &BTreeMap<String, crate::mir::ValueId>) -> bool,
{
    lower_no_exit_block_with_stmt_lowerer(
        builder,
        current_bindings,
        carrier_step_phis,
        break_phi_dsts,
        verified.arena(),
        verified.block(),
        error_prefix,
        make_lower_stmt,
        should_update_binding,
    )
}
