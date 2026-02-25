//! Loop Lowering Validation Utilities
//!
//! This module contains helper functions and validation logic for loop lowering.
//! Phase 29bq+: Extracted from loop_lowering.rs for better modularity.

use crate::mir::builder::control_flow::plan::CoreEffectPlan;
use crate::mir::builder::MirBuilder;
use crate::mir::{ConstValue, ValueId};

/// Debug logging for literal plan effects (strict/dev+planner_required debug-only)
pub fn debug_log_literal_plan(builder: &MirBuilder, path: &'static str, dst: ValueId, value: &ConstValue) {
    if crate::config::env::joinir_dev::strict_planner_required_debug_enabled() {
        let fn_name = super::debug_ctx::current_fn_name(builder);
        let next_value_id = builder
            .scope_ctx
            .current_function
            .as_ref()
            .map(|f| f.next_value_id)
            .unwrap_or(0);
        let file = builder
            .metadata_ctx
            .current_source_file()
            .unwrap_or_else(|| "unknown".to_string());
        let ring0 = crate::runtime::get_global_ring0();
        ring0.log.debug(&format!(
            "[lit/lower:plan] fn={} bb={:?} v=%{} lit={:?} span={} file={} next={} path={} emit=plan_effect",
            fn_name,
            builder.current_block,
            dst.0,
            value,
            super::span_fmt::current_span_location(builder),
            file,
            next_value_id,
            path
        ));
    }
}

/// Extract the defined value (if any) from a CoreEffectPlan
///
/// Returns Some((value_id, kind_str)) if the effect defines a value, None otherwise.
pub fn effect_defined_value(effect: &CoreEffectPlan) -> Option<(ValueId, &'static str)> {
    match effect {
        CoreEffectPlan::MethodCall { dst: Some(v), .. } => Some((*v, "MethodCall")),
        CoreEffectPlan::GlobalCall { dst: Some(v), .. } => Some((*v, "GlobalCall")),
        CoreEffectPlan::ValueCall { dst: Some(v), .. } => Some((*v, "ValueCall")),
        CoreEffectPlan::ExternCall { dst: Some(v), .. } => Some((*v, "ExternCall")),
        CoreEffectPlan::NewBox { dst, .. } => Some((*dst, "NewBox")),
        CoreEffectPlan::BinOp { dst, .. } => Some((*dst, "BinOp")),
        CoreEffectPlan::Compare { dst, .. } => Some((*dst, "Compare")),
        CoreEffectPlan::Select { dst, .. } => Some((*dst, "Select")),
        CoreEffectPlan::Const { dst, .. } => Some((*dst, "Const")),
        CoreEffectPlan::Copy { dst, .. } => Some((*dst, "Copy")),
        _ => None,
    }
}

/// Search for a forward definition of a ValueId in effects
///
/// Looks ahead in the effects list (starting from start_idx) for an effect that
/// defines the target value. Returns Some((index, kind_str)) if found, None otherwise.
pub fn find_forward_def(
    effects: &[CoreEffectPlan],
    start_idx: usize,
    target: ValueId,
) -> Option<(usize, &'static str)> {
    for (idx, effect) in effects.iter().enumerate().skip(start_idx) {
        if let Some((def_value, def_kind)) = effect_defined_value(effect) {
            if def_value == target {
                return Some((idx, def_kind));
            }
        }
    }
    None
}
