//! Exit lowering helpers (Parts).
//!
//! Scope: behavior-preserving extraction of existing lowering logic.
//! SSOT for exit (break/continue/return) lowering.

use crate::ast::ASTNode;
use super::super::steps::effects_to_plans;
use crate::mir::builder::control_flow::plan::normalizer::PlanNormalizer;
use crate::mir::builder::control_flow::plan::recipe_tree::common::ExitKind;
use crate::mir::builder::control_flow::plan::recipes::refs::StmtRef;
use crate::mir::builder::control_flow::plan::recipes::RecipeBody;
use crate::mir::builder::control_flow::plan::{CoreExitPlan, CorePlan, LoweredRecipe};
use crate::mir::builder::MirBuilder;
use std::collections::BTreeMap;

pub(in crate::mir::builder) fn lower_loop_cond_exit_leaf(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    break_phi_dsts: &BTreeMap<String, crate::mir::ValueId>,
    body: &RecipeBody,
    kind: ExitKind,
    stmt_ref: StmtRef,
    error_prefix: &str,
) -> Result<Vec<LoweredRecipe>, String> {
    match kind {
        ExitKind::Break { depth: 1 } => Ok(vec![CorePlan::Exit(
            build_break_with_phi_args(break_phi_dsts, current_bindings, error_prefix)?,
        )]),
        ExitKind::Continue { depth: 1 } => Ok(vec![CorePlan::Exit(
            build_continue_with_phi_args(
                builder,
                carrier_step_phis,
                current_bindings,
                error_prefix,
            )?,
        )]),
        ExitKind::Return => {
            let stmt = body.get_ref(stmt_ref).ok_or_else(|| {
                format!("{error_prefix}: missing stmt idx={}", stmt_ref.index())
            })?;
            let ret_value = match stmt {
                ASTNode::Return { value, .. } => value.as_ref().map(|v| v.as_ref()),
                _ => return Err(format!("{error_prefix}: ExitLeaf::Return expects Return")),
            };
            lower_return_stmt_with_effects(builder, ret_value, current_bindings, error_prefix)
        }
        ExitKind::Break { depth } => Err(format!(
            "[freeze:contract][exit_depth] {error_prefix}: break depth={} unsupported (only depth=1)",
            depth
        )),
        ExitKind::Continue { depth } => Err(format!(
            "[freeze:contract][exit_depth] {error_prefix}: continue depth={} unsupported (only depth=1)",
            depth
        )),
    }
}

/// Lower Return statement with effects (SSOT for Return lowering).
///
/// Handles:
/// - value: Some(expr) → lower expr, emit effects, Return(value_id)
/// - value: None → Return(None)
///
/// This is the canonical entry point for Return lowering in all pipelines.
#[allow(unused_variables)]
pub(in crate::mir::builder) fn lower_return_stmt_with_effects(
    builder: &mut MirBuilder,
    value: Option<&crate::ast::ASTNode>,
    current_bindings: &BTreeMap<String, crate::mir::ValueId>,
    _error_prefix: &str,
) -> Result<Vec<LoweredRecipe>, String> {
    let mut plans = Vec::new();
    let ret_val = match value {
        Some(v) => {
            let (value_id, effects) = PlanNormalizer::lower_value_ast(v, builder, current_bindings)?;
            plans.extend(effects_to_plans(effects));
            Some(value_id)
        },
        None => None,
    };
    plans.push(CorePlan::Exit(CoreExitPlan::Return(ret_val)));
    Ok(plans)
}

#[track_caller]
pub(in crate::mir::builder) fn build_continue_with_phi_args(
    builder: &MirBuilder,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    current_bindings: &BTreeMap<String, crate::mir::ValueId>,
    error_prefix: &str,
) -> Result<CoreExitPlan, String> {
    let mut phi_args = Vec::new();
    let debug = crate::config::env::joinir_dev::strict_planner_required_debug_enabled();
    let fn_name = if debug {
        builder
            .scope_ctx
            .current_function
            .as_ref()
            .map(|f| f.signature.name.as_str())
            .unwrap_or("<none>")
    } else {
        "<none>"
    };
    let pred_bb = if debug { builder.current_block } else { None };
    let caller = if debug {
        Some(std::panic::Location::caller().to_string())
    } else {
        None
    };
    for (name, phi_dst) in carrier_step_phis {
        let Some(&val) = current_bindings.get(name) else {
            return Err(format!("{error_prefix}: step join value {} not found", name));
        };
        if debug {
            let ring0 = crate::runtime::get_global_ring0();
            ring0.log.debug(&format!(
                "[loop_lowering/step_phi_input:add] fn={} origin=plan_build pred_bb={:?} name={} dst=%{} incoming=%{} caller={}",
                fn_name,
                pred_bb,
                name,
                phi_dst.0,
                val.0,
                caller.as_deref().unwrap_or("<unknown>")
            ));
        }
        phi_args.push((*phi_dst, val));
    }
    Ok(CoreExitPlan::ContinueWithPhiArgs {
        depth: 1,
        phi_args,
    })
}

pub(in crate::mir::builder) fn build_break_with_phi_args(
    break_phi_dsts: &BTreeMap<String, crate::mir::ValueId>,
    current_bindings: &BTreeMap<String, crate::mir::ValueId>,
    error_prefix: &str,
) -> Result<CoreExitPlan, String> {
    let mut phi_args = Vec::new();
    for (name, phi_dst) in break_phi_dsts {
        let Some(&val) = current_bindings.get(name) else {
            return Err(format!("{error_prefix}: break merge value {} not found", name));
        };
        phi_args.push((*phi_dst, val));
    }
    Ok(CoreExitPlan::BreakWithPhiArgs {
        depth: 1,
        phi_args,
    })
}
