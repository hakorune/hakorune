use crate::mir::builder::control_flow::plan::{CoreEffectPlan, CorePlan, LoweredRecipe};
use crate::mir::builder::MirBuilder;
use crate::mir::{ConstValue, ValueId};
use std::collections::HashSet;

pub(super) fn validate_effects_binop_operands(
    builder: &mut MirBuilder,
    effects: &[CoreEffectPlan],
    path: &'static str,
) -> Result<(), String> {
    let strict_planner_required = crate::config::env::joinir_dev::strict_enabled()
        && crate::config::env::joinir_dev::planner_required_enabled();
    if !strict_planner_required {
        return Ok(());
    }
    let mut defined_values = match builder.scope_ctx.current_function.as_ref() {
        Some(func) => crate::mir::verification::utils::compute_def_blocks(func)
            .keys()
            .copied()
            .collect::<HashSet<ValueId>>(),
        None => return Ok(()),
    };

    for (effect_idx, effect) in effects.iter().enumerate() {
        if let CoreEffectPlan::BinOp { dst, lhs, op, rhs } = effect {
            if !defined_values.contains(lhs) {
                if let Some((def_idx, def_kind)) = find_forward_def(effects, effect_idx + 1, *lhs) {
                    return Err(format!(
                        "[freeze:contract][loop_lowering/effect_forward_ref] fn={} bb={:?} use=%{} use_idx={} def_idx={} def_kind={} use_by=CoreEffectPlan::BinOp dst=%{} op={:?} operand=lhs path={}",
                        builder
                            .scope_ctx
                            .current_function
                            .as_ref()
                            .map(|f| f.signature.name.as_str())
                            .unwrap_or("<unknown>"),
                        builder.current_block,
                        lhs.0,
                        effect_idx,
                        def_idx,
                        def_kind,
                        dst.0,
                        op,
                        path
                    ));
                }
                return Err(format!(
                    "[freeze:contract][loop_lowering/effect_undefined_operand] fn={} bb={:?} use=%{} use_idx={} use_by=CoreEffectPlan::BinOp dst=%{} op={:?} operand=lhs plan_def=none path={}",
                    builder
                        .scope_ctx
                        .current_function
                        .as_ref()
                        .map(|f| f.signature.name.as_str())
                        .unwrap_or("<unknown>"),
                    builder.current_block,
                    lhs.0,
                    effect_idx,
                    dst.0,
                    op,
                    path
                ));
            }
            if !defined_values.contains(rhs) {
                if let Some((def_idx, def_kind)) = find_forward_def(effects, effect_idx + 1, *rhs) {
                    return Err(format!(
                        "[freeze:contract][loop_lowering/effect_forward_ref] fn={} bb={:?} use=%{} use_idx={} def_idx={} def_kind={} use_by=CoreEffectPlan::BinOp dst=%{} op={:?} operand=rhs path={}",
                        builder
                            .scope_ctx
                            .current_function
                            .as_ref()
                            .map(|f| f.signature.name.as_str())
                            .unwrap_or("<unknown>"),
                        builder.current_block,
                        rhs.0,
                        effect_idx,
                        def_idx,
                        def_kind,
                        dst.0,
                        op,
                        path
                    ));
                }
                return Err(format!(
                    "[freeze:contract][loop_lowering/effect_undefined_operand] fn={} bb={:?} use=%{} use_idx={} use_by=CoreEffectPlan::BinOp dst=%{} op={:?} operand=rhs plan_def=none path={}",
                    builder
                        .scope_ctx
                        .current_function
                        .as_ref()
                        .map(|f| f.signature.name.as_str())
                        .unwrap_or("<unknown>"),
                    builder.current_block,
                    rhs.0,
                    effect_idx,
                    dst.0,
                    op,
                    path
                ));
            }
        }
        if let Some((def_value, _)) = effect_defined_value(effect) {
            defined_values.insert(def_value);
        }
    }
    Ok(())
}

pub(super) fn debug_log_literal_plan(
    builder: &MirBuilder,
    path: &'static str,
    dst: ValueId,
    value: &ConstValue,
) {
    if crate::config::env::joinir_dev::strict_planner_required_debug_enabled() {
        let fn_name = super::super::debug_ctx::current_fn_name(builder);
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
            super::super::span_fmt::current_span_location(builder),
            file,
            next_value_id,
            path
        ));
    }
}

pub(super) fn effect_defined_value(effect: &CoreEffectPlan) -> Option<(ValueId, &'static str)> {
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

pub(super) fn find_forward_def(
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

pub(super) fn plan_def_kind(plan: &LoweredRecipe, target: ValueId) -> Option<&'static str> {
    match plan {
        CorePlan::Effect(effect) => effect_defined_value(effect)
            .and_then(|(def_value, def_kind)| (def_value == target).then_some(def_kind)),
        CorePlan::If(plan) => {
            if plan.joins.iter().any(|join| join.dst == target) {
                return Some("IfJoin");
            }
            for nested in &plan.then_plans {
                if let Some(kind) = plan_def_kind(nested, target) {
                    return Some(kind);
                }
            }
            if let Some(else_plans) = &plan.else_plans {
                for nested in else_plans {
                    if let Some(kind) = plan_def_kind(nested, target) {
                        return Some(kind);
                    }
                }
            }
            None
        }
        CorePlan::Loop(plan) => {
            if plan.phis.iter().any(|phi| phi.dst == target) {
                return Some("LoopPhi");
            }
            for nested in &plan.body {
                if let Some(kind) = plan_def_kind(nested, target) {
                    return Some(kind);
                }
            }
            None
        }
        CorePlan::Seq(plans) => {
            for nested in plans {
                if let Some(kind) = plan_def_kind(nested, target) {
                    return Some(kind);
                }
            }
            None
        }
        CorePlan::BranchN(plan) => {
            for arm in &plan.arms {
                for nested in &arm.plans {
                    if let Some(kind) = plan_def_kind(nested, target) {
                        return Some(kind);
                    }
                }
            }
            if let Some(else_plans) = &plan.else_plans {
                for nested in else_plans {
                    if let Some(kind) = plan_def_kind(nested, target) {
                        return Some(kind);
                    }
                }
            }
            None
        }
        CorePlan::Exit(_) => None,
    }
}

pub(super) fn plans_find_def(
    plans: &[LoweredRecipe],
    start_idx: usize,
    target: ValueId,
) -> Option<(usize, &'static str)> {
    for (idx, plan) in plans.iter().enumerate().skip(start_idx) {
        if let Some(def_kind) = plan_def_kind(plan, target) {
            return Some((idx, def_kind));
        }
    }
    None
}
