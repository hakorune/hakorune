//! Exit-branch helper (prelude + exit lowering). No AST rewrite.
//!
//! Note: Core implementations moved to parts::exit_branch (M5i).
//! Functions here delegate to parts for backward compatibility.
#![allow(dead_code)]

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::parts::entry as parts_entry;
use crate::mir::builder::control_flow::plan::parts::exit as parts_exit;
use crate::mir::builder::control_flow::plan::parts::exit_branch as parts_exit_branch;
use crate::mir::builder::control_flow::plan::parts::stmt as parts_stmt;
use crate::mir::builder::control_flow::plan::{CoreExitPlan, CorePlan, LoweredRecipe};
use crate::mir::builder::MirBuilder;
use std::collections::BTreeMap;

/// Delegate to parts::exit_branch (SSOT).
pub(in crate::mir::builder) fn split_exit_branch<'a>(
    body: &'a [ASTNode],
    error_prefix: &str,
) -> Result<(Vec<&'a ASTNode>, &'a ASTNode, bool), String> {
    parts_exit_branch::split_exit_branch(body, error_prefix)
}

/// Delegate to parts::exit_branch (SSOT).
pub(in crate::mir::builder) fn lower_exit_branch_with_prelude(
    builder: &mut MirBuilder,
    current_bindings: &BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    prelude: &[&ASTNode],
    exit_stmt: &ASTNode,
    error_prefix: &str,
) -> Result<Vec<LoweredRecipe>, String> {
    parts_exit_branch::lower_exit_branch_with_prelude(
        builder,
        current_bindings,
        carrier_step_phis,
        prelude,
        exit_stmt,
        error_prefix,
    )
}

/// Delegate to parts::exit_branch (SSOT).
pub(in crate::mir::builder) fn lower_exit_branch_with_prelude_with_break_phi_args(
    builder: &mut MirBuilder,
    current_bindings: &BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    break_phi_dsts: &BTreeMap<String, crate::mir::ValueId>,
    prelude: &[&ASTNode],
    exit_stmt: &ASTNode,
    error_prefix: &str,
) -> Result<Vec<LoweredRecipe>, String> {
    parts_exit_branch::lower_exit_branch_with_prelude_with_break_phi_args(
        builder,
        current_bindings,
        carrier_step_phis,
        break_phi_dsts,
        prelude,
        exit_stmt,
        error_prefix,
    )
}

/// Delegate to parts::exit (SSOT for continue PHI args).
pub(in crate::mir::builder) fn build_continue_with_phi_args(
    builder: &MirBuilder,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    current_bindings: &BTreeMap<String, crate::mir::ValueId>,
    error_prefix: &str,
) -> Result<CoreExitPlan, String> {
    parts_exit::build_continue_with_phi_args(
        builder,
        carrier_step_phis,
        current_bindings,
        error_prefix,
    )
}

/// Delegate to parts::exit (SSOT for break PHI args).
pub(in crate::mir::builder) fn build_break_with_phi_args(
    break_phi_dsts: &BTreeMap<String, crate::mir::ValueId>,
    current_bindings: &BTreeMap<String, crate::mir::ValueId>,
    error_prefix: &str,
) -> Result<CoreExitPlan, String> {
    parts_exit::build_break_with_phi_args(break_phi_dsts, current_bindings, error_prefix)
}

/// Delegate to parts::stmt (SSOT for return prelude lowering).
pub(in crate::mir::builder) fn lower_return_prelude_stmt(
    builder: &mut MirBuilder,
    branch_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    break_phi_dsts: Option<&BTreeMap<String, crate::mir::ValueId>>,
    stmt: &ASTNode,
    error_prefix: &str,
) -> Result<Vec<LoweredRecipe>, String> {
    parts_stmt::lower_return_prelude_stmt(
        builder,
        branch_bindings,
        carrier_step_phis,
        break_phi_dsts,
        stmt,
        error_prefix,
    )
}

/// Build a CoreExitPlan::Return (for ExitIf / low-level usage).
///
/// Consolidates Return generation to a single location (BoxShape).
pub(in crate::mir::builder) fn build_return_exit_plan(
    value_id: crate::mir::ValueId,
) -> CoreExitPlan {
    CoreExitPlan::Return(Some(value_id))
}

/// Build a return-only exit plan (no prelude).
///
/// Consolidates Return generation to a single location (BoxShape).
pub(in crate::mir::builder) fn build_return_only(
    value_id: crate::mir::ValueId,
) -> LoweredRecipe {
    CorePlan::Exit(build_return_exit_plan(value_id))
}

/// Build a CoreExitPlan::Break (for ExitIf / low-level usage).
pub(in crate::mir::builder) fn build_break_exit_plan(depth: usize) -> CoreExitPlan {
    CoreExitPlan::Break(depth)
}

/// Build a break-only exit plan.
///
/// Consolidates Break generation to a single location (BoxShape).
pub(in crate::mir::builder) fn build_break_only(depth: usize) -> LoweredRecipe {
    CorePlan::Exit(build_break_exit_plan(depth))
}

/// Build a CoreExitPlan::Continue (for ExitIf / low-level usage).
pub(in crate::mir::builder) fn build_continue_exit_plan(depth: usize) -> CoreExitPlan {
    CoreExitPlan::Continue(depth)
}

/// Build a continue-only exit plan.
///
/// Consolidates Continue generation to a single location (BoxShape).
pub(in crate::mir::builder) fn build_continue_only(depth: usize) -> LoweredRecipe {
    CorePlan::Exit(build_continue_exit_plan(depth))
}

/// Build a CoreExitPlan::Return with Option<ValueId> (for loop_cond_* pipelines).
///
/// Consolidates Return(Option<ValueId>) generation to a single location (BoxShape).
pub(in crate::mir::builder) fn build_return_exit_plan_opt(
    ret_val: Option<crate::mir::ValueId>,
) -> CoreExitPlan {
    CoreExitPlan::Return(ret_val)
}

/// Delegate to parts::entry (SSOT for Return lowering).
pub(in crate::mir::builder) fn lower_return_stmt_with_effects(
    builder: &mut MirBuilder,
    value: Option<&crate::ast::ASTNode>,
    current_bindings: &BTreeMap<String, crate::mir::ValueId>,
    error_prefix: &str,
) -> Result<Vec<LoweredRecipe>, String> {
    parts_entry::lower_return_with_effects(builder, value, current_bindings, error_prefix)
}
