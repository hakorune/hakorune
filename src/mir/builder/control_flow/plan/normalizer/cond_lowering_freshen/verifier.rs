use crate::mir::builder::control_flow::edgecfg::api::Frag;
use crate::mir::builder::control_flow::plan::{
    CoreEffectPlan, CoreExitPlan, CorePlan, LoweredRecipe,
};
use crate::mir::ValueId;
use std::collections::BTreeMap;

pub(crate) fn find_unremapped_value_id(
    plan: &LoweredRecipe,
    value_map: &BTreeMap<ValueId, ValueId>,
) -> Option<(ValueId, ValueId, &'static str)> {
    // Check: if a ValueId is a key of value_map, it was freshened and MUST NOT remain.
    find_unremapped_value_id_plan(plan, value_map)
}

pub(crate) fn find_remap_mismatch(
    plan: &LoweredRecipe,
    value_map: &BTreeMap<ValueId, ValueId>,
) -> Option<(
    ValueId,
    ValueId,
    &'static str,
    ValueId,
    crate::mir::BinaryOp,
)> {
    match plan {
        CorePlan::Seq(plans) => {
            for p in plans {
                if let Some(hit) = find_remap_mismatch(p, value_map) {
                    return Some(hit);
                }
            }
            None
        }
        CorePlan::If(if_plan) => {
            for p in &if_plan.then_plans {
                if let Some(hit) = find_remap_mismatch(p, value_map) {
                    return Some(hit);
                }
            }
            if let Some(else_plans) = &if_plan.else_plans {
                for p in else_plans {
                    if let Some(hit) = find_remap_mismatch(p, value_map) {
                        return Some(hit);
                    }
                }
            }
            None
        }
        CorePlan::Loop(loop_plan) => {
            for p in &loop_plan.body {
                if let Some(hit) = find_remap_mismatch(p, value_map) {
                    return Some(hit);
                }
            }
            for (_bb, effects) in &loop_plan.block_effects {
                if let Some(hit) = find_remap_mismatch_effects(effects, value_map) {
                    return Some(hit);
                }
            }
            None
        }
        CorePlan::BranchN(branch_n) => {
            for arm in &branch_n.arms {
                for p in &arm.plans {
                    if let Some(hit) = find_remap_mismatch(p, value_map) {
                        return Some(hit);
                    }
                }
            }
            if let Some(else_plans) = &branch_n.else_plans {
                for p in else_plans {
                    if let Some(hit) = find_remap_mismatch(p, value_map) {
                        return Some(hit);
                    }
                }
            }
            None
        }
        CorePlan::Effect(effect) => find_remap_mismatch_effect(effect, value_map),
        CorePlan::Exit(_) => None,
    }
}

fn find_remap_mismatch_effects(
    effects: &[CoreEffectPlan],
    value_map: &BTreeMap<ValueId, ValueId>,
) -> Option<(
    ValueId,
    ValueId,
    &'static str,
    ValueId,
    crate::mir::BinaryOp,
)> {
    for effect in effects {
        if let Some(hit) = find_remap_mismatch_effect(effect, value_map) {
            return Some(hit);
        }
    }
    None
}

fn find_remap_mismatch_effect(
    effect: &CoreEffectPlan,
    value_map: &BTreeMap<ValueId, ValueId>,
) -> Option<(
    ValueId,
    ValueId,
    &'static str,
    ValueId,
    crate::mir::BinaryOp,
)> {
    match effect {
        CoreEffectPlan::BinOp { dst, lhs, op, rhs } => {
            if let Some(&new) = value_map.get(lhs) {
                return Some((*lhs, new, "lhs", *dst, *op));
            }
            if let Some(&new) = value_map.get(rhs) {
                return Some((*rhs, new, "rhs", *dst, *op));
            }
            None
        }
        CoreEffectPlan::IfEffect {
            then_effects,
            else_effects,
            ..
        } => {
            if let Some(hit) = find_remap_mismatch_effects(then_effects, value_map) {
                return Some(hit);
            }
            if let Some(else_effects) = else_effects {
                if let Some(hit) = find_remap_mismatch_effects(else_effects, value_map) {
                    return Some(hit);
                }
            }
            None
        }
        _ => None,
    }
}

fn find_unremapped_value_id_plan(
    plan: &LoweredRecipe,
    value_map: &BTreeMap<ValueId, ValueId>,
) -> Option<(ValueId, ValueId, &'static str)> {
    match plan {
        CorePlan::Seq(plans) => {
            for p in plans {
                if let Some(hit) = find_unremapped_value_id_plan(p, value_map) {
                    return Some(hit);
                }
            }
            None
        }
        CorePlan::If(if_plan) => {
            if let Some(&new) = value_map.get(&if_plan.condition) {
                return Some((if_plan.condition, new, "If.condition"));
            }
            for join in &if_plan.joins {
                if let Some(&new) = value_map.get(&join.dst) {
                    return Some((join.dst, new, "If.join.dst"));
                }
                if let Some(&new) = value_map.get(&join.then_val) {
                    return Some((join.then_val, new, "If.join.then_val"));
                }
                if let Some(&new) = value_map.get(&join.else_val) {
                    return Some((join.else_val, new, "If.join.else_val"));
                }
            }
            for p in &if_plan.then_plans {
                if let Some(hit) = find_unremapped_value_id_plan(p, value_map) {
                    return Some(hit);
                }
            }
            if let Some(else_plans) = &if_plan.else_plans {
                for p in else_plans {
                    if let Some(hit) = find_unremapped_value_id_plan(p, value_map) {
                        return Some(hit);
                    }
                }
            }
            None
        }
        CorePlan::Loop(loop_plan) => {
            if let Some(&new) = value_map.get(&loop_plan.cond_loop) {
                return Some((loop_plan.cond_loop, new, "Loop.cond_loop"));
            }
            if let Some(&new) = value_map.get(&loop_plan.cond_match) {
                return Some((loop_plan.cond_match, new, "Loop.cond_match"));
            }
            for phi in &loop_plan.phis {
                if let Some(&new) = value_map.get(&phi.dst) {
                    return Some((phi.dst, new, "Loop.phi.dst"));
                }
                for (_pred, v) in &phi.inputs {
                    if let Some(&new) = value_map.get(v) {
                        return Some((*v, new, "Loop.phi.input"));
                    }
                }
            }
            for p in &loop_plan.body {
                if let Some(hit) = find_unremapped_value_id_plan(p, value_map) {
                    return Some(hit);
                }
            }
            for (_bb, effects) in &loop_plan.block_effects {
                if let Some(hit) =
                    find_unremapped_value_id_effects(effects, value_map, "Loop.block_effects")
                {
                    return Some(hit);
                }
            }
            for (_k, v) in &loop_plan.final_values {
                if let Some(&new) = value_map.get(v) {
                    return Some((*v, new, "Loop.final_values"));
                }
            }
            find_unremapped_value_id_frag(&loop_plan.frag, value_map)
        }
        CorePlan::BranchN(branch_n) => {
            for arm in &branch_n.arms {
                if let Some(&new) = value_map.get(&arm.condition) {
                    return Some((arm.condition, new, "BranchN.arm.condition"));
                }
                for p in &arm.plans {
                    if let Some(hit) = find_unremapped_value_id_plan(p, value_map) {
                        return Some(hit);
                    }
                }
            }
            if let Some(else_plans) = &branch_n.else_plans {
                for p in else_plans {
                    if let Some(hit) = find_unremapped_value_id_plan(p, value_map) {
                        return Some(hit);
                    }
                }
            }
            None
        }
        CorePlan::Effect(effect) => {
            find_unremapped_value_id_effect(effect, value_map, "Plan.effect")
        }
        CorePlan::Exit(exit) => match exit {
            CoreExitPlan::Return(v) => v.and_then(|v| {
                value_map
                    .get(&v)
                    .copied()
                    .map(|new| (v, new, "Exit.return"))
            }),
            CoreExitPlan::Break(_) | CoreExitPlan::Continue(_) => None,
            CoreExitPlan::BreakWithPhiArgs { phi_args, .. }
            | CoreExitPlan::ContinueWithPhiArgs { phi_args, .. } => {
                for (dst, val) in phi_args {
                    if let Some(&new) = value_map.get(dst) {
                        return Some((*dst, new, "Exit.phi_args.dst"));
                    }
                    if let Some(&new) = value_map.get(val) {
                        return Some((*val, new, "Exit.phi_args.val"));
                    }
                }
                None
            }
        },
    }
}

fn find_unremapped_value_id_effects(
    effects: &[CoreEffectPlan],
    value_map: &BTreeMap<ValueId, ValueId>,
    _site: &'static str,
) -> Option<(ValueId, ValueId, &'static str)> {
    for effect in effects {
        if let Some(hit) = find_unremapped_value_id_effect(effect, value_map, "EffectList.effect") {
            return Some(hit);
        }
    }
    None
}

fn find_unremapped_value_id_effect(
    effect: &CoreEffectPlan,
    value_map: &BTreeMap<ValueId, ValueId>,
    _site: &'static str,
) -> Option<(ValueId, ValueId, &'static str)> {
    match effect {
        CoreEffectPlan::Const { dst, .. } => value_map
            .get(dst)
            .copied()
            .map(|new| (*dst, new, "Effect::Const.dst")),
        CoreEffectPlan::Copy { dst, src } => {
            if let Some(&new) = value_map.get(dst) {
                return Some((*dst, new, "Effect::Copy.dst"));
            }
            value_map
                .get(src)
                .copied()
                .map(|new| (*src, new, "Effect::Copy.src"))
        }
        CoreEffectPlan::BinOp { dst, lhs, rhs, .. } => {
            if let Some(&new) = value_map.get(dst) {
                return Some((*dst, new, "Effect::BinOp.dst"));
            }
            if let Some(&new) = value_map.get(lhs) {
                return Some((*lhs, new, "Effect::BinOp.lhs"));
            }
            value_map
                .get(rhs)
                .copied()
                .map(|new| (*rhs, new, "Effect::BinOp.rhs"))
        }
        CoreEffectPlan::Compare { dst, lhs, rhs, .. } => {
            if let Some(&new) = value_map.get(dst) {
                return Some((*dst, new, "Effect::Compare.dst"));
            }
            if let Some(&new) = value_map.get(lhs) {
                return Some((*lhs, new, "Effect::Compare.lhs"));
            }
            value_map
                .get(rhs)
                .copied()
                .map(|new| (*rhs, new, "Effect::Compare.rhs"))
        }
        CoreEffectPlan::Select {
            dst,
            cond,
            then_val,
            else_val,
        } => {
            if let Some(&new) = value_map.get(dst) {
                return Some((*dst, new, "Effect::Select.dst"));
            }
            if let Some(&new) = value_map.get(cond) {
                return Some((*cond, new, "Effect::Select.cond"));
            }
            if let Some(&new) = value_map.get(then_val) {
                return Some((*then_val, new, "Effect::Select.then_val"));
            }
            value_map
                .get(else_val)
                .copied()
                .map(|new| (*else_val, new, "Effect::Select.else_val"))
        }
        CoreEffectPlan::NewBox { dst, args, .. } => {
            if let Some(&new) = value_map.get(dst) {
                return Some((*dst, new, "Effect::NewBox.dst"));
            }
            for a in args {
                if let Some(&new) = value_map.get(a) {
                    return Some((*a, new, "Effect::NewBox.arg"));
                }
            }
            None
        }
        CoreEffectPlan::FieldGet { dst, base, .. } => {
            if let Some(&new) = value_map.get(dst) {
                return Some((*dst, new, "Effect::FieldGet.dst"));
            }
            value_map
                .get(base)
                .copied()
                .map(|new| (*base, new, "Effect::FieldGet.base"))
        }
        CoreEffectPlan::FieldSet { base, value, .. } => {
            if let Some(&new) = value_map.get(base) {
                return Some((*base, new, "Effect::FieldSet.base"));
            }
            value_map
                .get(value)
                .copied()
                .map(|new| (*value, new, "Effect::FieldSet.value"))
        }
        CoreEffectPlan::MethodCall {
            dst, object, args, ..
        } => {
            if let Some(d) = dst {
                if let Some(&new) = value_map.get(d) {
                    return Some((*d, new, "Effect::MethodCall.dst"));
                }
            }
            if let Some(&new) = value_map.get(object) {
                return Some((*object, new, "Effect::MethodCall.object"));
            }
            for a in args {
                if let Some(&new) = value_map.get(a) {
                    return Some((*a, new, "Effect::MethodCall.arg"));
                }
            }
            None
        }
        CoreEffectPlan::GlobalCall { dst, args, .. } => {
            if let Some(d) = dst {
                if let Some(&new) = value_map.get(d) {
                    return Some((*d, new, "Effect::GlobalCall.dst"));
                }
            }
            for a in args {
                if let Some(&new) = value_map.get(a) {
                    return Some((*a, new, "Effect::GlobalCall.arg"));
                }
            }
            None
        }
        CoreEffectPlan::ValueCall { dst, callee, args } => {
            if let Some(d) = dst {
                if let Some(&new) = value_map.get(d) {
                    return Some((*d, new, "Effect::ValueCall.dst"));
                }
            }
            if let Some(&new) = value_map.get(callee) {
                return Some((*callee, new, "Effect::ValueCall.callee"));
            }
            for a in args {
                if let Some(&new) = value_map.get(a) {
                    return Some((*a, new, "Effect::ValueCall.arg"));
                }
            }
            None
        }
        CoreEffectPlan::ExternCall { dst, args, .. } => {
            if let Some(d) = dst {
                if let Some(&new) = value_map.get(d) {
                    return Some((*d, new, "Effect::ExternCall.dst"));
                }
            }
            for a in args {
                if let Some(&new) = value_map.get(a) {
                    return Some((*a, new, "Effect::ExternCall.arg"));
                }
            }
            None
        }
        CoreEffectPlan::ExitIf { cond, exit } => {
            if let Some(&new) = value_map.get(cond) {
                return Some((*cond, new, "Effect::ExitIf.cond"));
            }
            let exit_plan: LoweredRecipe = CorePlan::Exit(exit.clone());
            find_unremapped_value_id_plan(&exit_plan, value_map)
        }
        CoreEffectPlan::IfEffect {
            cond,
            then_effects,
            else_effects,
        } => {
            if let Some(&new) = value_map.get(cond) {
                return Some((*cond, new, "Effect::IfEffect.cond"));
            }
            if let Some(hit) =
                find_unremapped_value_id_effects(then_effects, value_map, "IfEffect.then")
            {
                return Some(hit);
            }
            if let Some(else_effects) = else_effects {
                if let Some(hit) =
                    find_unremapped_value_id_effects(else_effects, value_map, "IfEffect.else")
                {
                    return Some(hit);
                }
            }
            None
        }
    }
}

fn find_unremapped_value_id_frag(
    frag: &Frag,
    value_map: &BTreeMap<ValueId, ValueId>,
) -> Option<(ValueId, ValueId, &'static str)> {
    for (_bb, params) in &frag.block_params {
        for v in &params.params {
            if let Some(&new) = value_map.get(v) {
                return Some((*v, new, "Frag.block_params.param"));
            }
        }
    }
    for (_kind, stubs) in &frag.exits {
        for stub in stubs {
            for v in &stub.args.values {
                if let Some(&new) = value_map.get(v) {
                    return Some((*v, new, "Frag.exits.args"));
                }
            }
        }
    }
    for stub in &frag.wires {
        for v in &stub.args.values {
            if let Some(&new) = value_map.get(v) {
                return Some((*v, new, "Frag.wires.args"));
            }
        }
    }
    for br in &frag.branches {
        if let Some(&new) = value_map.get(&br.cond) {
            return Some((br.cond, new, "Frag.branch.cond"));
        }
        for v in &br.then_args.values {
            if let Some(&new) = value_map.get(v) {
                return Some((*v, new, "Frag.branch.then_args"));
            }
        }
        for v in &br.else_args.values {
            if let Some(&new) = value_map.get(v) {
                return Some((*v, new, "Frag.branch.else_args"));
            }
        }
    }
    None
}
