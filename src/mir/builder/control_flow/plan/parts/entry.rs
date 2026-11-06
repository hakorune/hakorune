//! Verified entrypoints for lowering RecipeBlocks (SSOT).
//!
//! Goal:
//! - Make verification always-on (release builds included).
//! - Keep a single acceptance gate: `verify -> VerifiedRecipeBlock -> dispatch lower`.
//!
//! Notes:
//! - This module is intended to become the only place that creates `VerifiedRecipeBlock`.

use crate::mir::builder::control_flow::plan::recipe_tree::{
    verify_block_contract_with_pre, verify_port_sig_obligations_if_enabled, BlockContractKind, RecipeBlock, RecipeBodies, VerifiedRecipeBlock,
};
use crate::mir::builder::control_flow::plan::{CoreEffectPlan, CorePlan, LoweredRecipe};
use crate::mir::builder::MirBuilder;
use crate::mir::ConstValue;
use std::collections::BTreeMap;

use super::dispatch;
pub(in crate::mir::builder) use super::dispatch::{
    lower_if_join_with_branch_lowerers,
};

pub(in crate::mir::builder) fn lower_cond_prelude_stmt_as_plan(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    stmt: &crate::ast::ASTNode,
    error_prefix: &str,
) -> Result<Vec<LoweredRecipe>, String> {
    let empty_carrier_step_phis = BTreeMap::new();
    super::stmt::lower_return_prelude_stmt(
        builder,
        current_bindings,
        &empty_carrier_step_phis,
        None,
        stmt,
        error_prefix,
    )
}

pub(in crate::mir::builder) fn verify_exit_only_block_with_pre<'a>(
    arena: &'a RecipeBodies,
    block: &'a RecipeBlock,
    error_prefix: &str,
    pre_bindings: Option<&BTreeMap<String, crate::mir::ValueId>>,
) -> Result<VerifiedRecipeBlock<'a>, String> {
    verify_block_contract_with_pre(
        arena,
        block,
        BlockContractKind::ExitOnly,
        error_prefix,
        pre_bindings,
    )
}

pub(in crate::mir::builder) fn verify_stmt_only_block_with_pre<'a>(
    arena: &'a RecipeBodies,
    block: &'a RecipeBlock,
    error_prefix: &str,
    pre_bindings: Option<&BTreeMap<String, crate::mir::ValueId>>,
) -> Result<VerifiedRecipeBlock<'a>, String> {
    verify_block_contract_with_pre(
        arena,
        block,
        BlockContractKind::StmtOnly,
        error_prefix,
        pre_bindings,
    )
}

pub(in crate::mir::builder) fn verify_exit_allowed_block_with_pre<'a>(
    arena: &'a RecipeBodies,
    block: &'a RecipeBlock,
    error_prefix: &str,
    pre_bindings: Option<&BTreeMap<String, crate::mir::ValueId>>,
) -> Result<VerifiedRecipeBlock<'a>, String> {
    verify_block_contract_with_pre(
        arena,
        block,
        BlockContractKind::ExitAllowed,
        error_prefix,
        pre_bindings,
    )
}

pub(in crate::mir::builder) fn verify_no_exit_block_with_pre<'a>(
    arena: &'a RecipeBodies,
    block: &'a RecipeBlock,
    error_prefix: &str,
    pre_bindings: Option<&BTreeMap<String, crate::mir::ValueId>>,
) -> Result<VerifiedRecipeBlock<'a>, String> {
    verify_block_contract_with_pre(
        arena,
        block,
        BlockContractKind::NoExit,
        error_prefix,
        pre_bindings,
    )
}
pub(in crate::mir::builder) fn lower_exit_only_block_verified(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    break_phi_dsts: &BTreeMap<String, crate::mir::ValueId>,
    verified: VerifiedRecipeBlock<'_>,
    error_prefix: &str,
) -> Result<Vec<LoweredRecipe>, String> {
    verify_port_sig_obligations_if_enabled(&verified, error_prefix)?;
    let plans = dispatch::lower_exit_only_block_verified(
        builder,
        current_bindings,
        carrier_step_phis,
        break_phi_dsts,
        verified,
        error_prefix,
    )?;
    debug_log_entry_plans_lit3_origin(builder, &plans, "exit_only");
    Ok(plans)
}

pub(in crate::mir::builder) fn lower_stmt_only_block_verified<LowerStmt>(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    break_phi_dsts: Option<&BTreeMap<String, crate::mir::ValueId>>,
    verified: VerifiedRecipeBlock<'_>,
    error_prefix: &str,
    lower_stmt: LowerStmt,
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
    verify_port_sig_obligations_if_enabled(&verified, error_prefix)?;
    let plans = dispatch::lower_stmt_only_block(
        builder,
        current_bindings,
        carrier_step_phis,
        break_phi_dsts,
        verified.arena(),
        verified.block(),
        error_prefix,
        lower_stmt,
    )?;
    debug_log_entry_plans_lit3_origin(builder, &plans, "stmt_only");
    Ok(plans)
}

pub(in crate::mir::builder) fn lower_exit_allowed_block_verified(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    break_phi_dsts: &BTreeMap<String, crate::mir::ValueId>,
    verified: VerifiedRecipeBlock<'_>,
    error_prefix: &str,
) -> Result<Vec<LoweredRecipe>, String> {
    verify_port_sig_obligations_if_enabled(&verified, error_prefix)?;
    let plans = dispatch::lower_exit_allowed_block_verified(
        builder,
        current_bindings,
        carrier_step_phis,
        break_phi_dsts,
        verified,
        error_prefix,
    )?;
    debug_log_entry_plans_lit3_origin(builder, &plans, "exit_allowed");
    Ok(plans)
}

pub(in crate::mir::builder) fn lower_exit_allowed_block(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    break_phi_dsts: &BTreeMap<String, crate::mir::ValueId>,
    arena: &RecipeBodies,
    block: &RecipeBlock,
    error_prefix: &str,
) -> Result<Vec<LoweredRecipe>, String> {
    let verified =
        verify_exit_allowed_block_with_pre(arena, block, error_prefix, Some(current_bindings))?;
    lower_exit_allowed_block_verified(
        builder,
        current_bindings,
        carrier_step_phis,
        break_phi_dsts,
        verified,
        error_prefix,
    )
}

pub(in crate::mir::builder) fn lower_no_exit_block_verified(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    break_phi_dsts: Option<&BTreeMap<String, crate::mir::ValueId>>,
    verified: VerifiedRecipeBlock<'_>,
    error_prefix: &str,
) -> Result<Vec<LoweredRecipe>, String> {
    verify_port_sig_obligations_if_enabled(&verified, error_prefix)?;
    let plans = dispatch::lower_no_exit_block_verified(
        builder,
        current_bindings,
        carrier_step_phis,
        break_phi_dsts,
        verified,
        error_prefix,
    )?;
    debug_log_entry_plans_lit3_origin(builder, &plans, "no_exit");
    Ok(plans)
}

pub(in crate::mir::builder) fn lower_no_exit_block(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    break_phi_dsts: Option<&BTreeMap<String, crate::mir::ValueId>>,
    arena: &RecipeBodies,
    block: &RecipeBlock,
    error_prefix: &str,
) -> Result<Vec<LoweredRecipe>, String> {
    let verified =
        verify_no_exit_block_with_pre(arena, block, error_prefix, Some(current_bindings))?;
    lower_no_exit_block_verified(
        builder,
        current_bindings,
        carrier_step_phis,
        break_phi_dsts,
        verified,
        error_prefix,
    )
}

fn debug_log_entry_plans_lit3_origin(
    builder: &MirBuilder,
    plans: &[LoweredRecipe],
    kind: &'static str,
) {
    if !crate::config::env::joinir_dev::strict_planner_required_debug_enabled() {
        return;
    }

    let mut lit3_dsts = Vec::new();
    let mut lit3_spans = Vec::new();
    for plan in plans {
        if let CorePlan::Effect(CoreEffectPlan::Const { dst, value }) = plan {
            if matches!(value, ConstValue::Integer(3)) {
                if let Some(span) = builder.value_origin_span(*dst) {
                    lit3_dsts.push(*dst);
                    lit3_spans.push(span.to_string());
                }
            }
        }
    }

    if lit3_dsts.is_empty() {
        return;
    }

    let fn_name = builder
        .function_state
        .current_function
        .as_ref()
        .map(|f| f.signature.name.as_str())
        .unwrap_or("<none>");
    let const_int3_dsts = lit3_dsts
        .iter()
        .map(|v| format!("%{}", v.0))
        .collect::<Vec<_>>()
        .join(",");
    let origin_spans = lit3_spans.join(",");
    let ring0 = crate::runtime::get_global_ring0();
    ring0.log.debug(&format!(
        "[entry/block_plans:lit3_origin] fn={} kind={} bb={:?} plans_len={} const_int3_dsts=[{}] origin_spans=[{}]",
        fn_name,
        kind,
        builder.function_state.current_block,
        plans.len(),
        const_int3_dsts,
        origin_spans
    ));
}
