use super::utils::{map_block_id, remap_value_id, remap_value_ids};
use crate::mir::builder::control_flow::plan::edgecfg_facade::Frag;
use crate::mir::builder::control_flow::plan::features::edgecfg_stubs;
use crate::mir::builder::control_flow::plan::{
    CoreEffectPlan, CoreExitPlan, CoreIfJoin, CoreIfPlan, CoreLoopPlan, CorePlan, LoweredRecipe,
};
use crate::mir::builder::MirBuilder;
use crate::mir::BasicBlockId;
use crate::mir::{EdgeArgs, ValueId};
use std::collections::BTreeMap;

/// Remap a plan with block ID and ValueId freshening
pub(crate) fn remap_plan(
    builder: &mut MirBuilder,
    block_map: &mut BTreeMap<BasicBlockId, BasicBlockId>,
    value_map: &BTreeMap<ValueId, ValueId>,
    plan: LoweredRecipe,
) -> LoweredRecipe {
    match plan {
        CorePlan::Seq(plans) => CorePlan::Seq(
            plans
                .into_iter()
                .map(|p| remap_plan(builder, block_map, value_map, p))
                .collect(),
        ),
        CorePlan::If(mut if_plan) => {
            if_plan.condition = remap_value_id(value_map, if_plan.condition);
            if_plan.then_plans = if_plan
                .then_plans
                .into_iter()
                .map(|p| remap_plan(builder, block_map, value_map, p))
                .collect();
            if_plan.else_plans = if_plan.else_plans.map(|plans| {
                plans
                    .into_iter()
                    .map(|p| remap_plan(builder, block_map, value_map, p))
                    .collect()
            });
            if_plan.joins = if_plan
                .joins
                .into_iter()
                .map(|join| CoreIfJoin {
                    name: join.name.clone(),
                    dst: remap_value_id(value_map, join.dst),
                    pre_val: join.pre_val.map(|v| remap_value_id(value_map, v)),
                    then_val: remap_value_id(value_map, join.then_val),
                    else_val: remap_value_id(value_map, join.else_val),
                })
                .collect();
            CorePlan::If(if_plan)
        }
        CorePlan::Loop(loop_plan) => remap_loop_plan(builder, block_map, value_map, loop_plan),
        CorePlan::BranchN(mut branch_plan) => {
            branch_plan.arms = branch_plan
                .arms
                .into_iter()
                .map(|mut arm| {
                    arm.condition = remap_value_id(value_map, arm.condition);
                    arm.plans = arm
                        .plans
                        .into_iter()
                        .map(|p| remap_plan(builder, block_map, value_map, p))
                        .collect();
                    arm
                })
                .collect();
            branch_plan.else_plans = branch_plan.else_plans.map(|plans| {
                plans
                    .into_iter()
                    .map(|p| remap_plan(builder, block_map, value_map, p))
                    .collect()
            });
            CorePlan::BranchN(branch_plan)
        }
        CorePlan::Effect(mut effect) => {
            remap_effect_in_place(value_map, &mut effect);
            CorePlan::Effect(effect)
        }
        CorePlan::Exit(exit) => CorePlan::Exit(remap_exit(value_map, exit)),
    }
}

/// Remap plan with only block ID freshening (legacy compatibility)
pub(crate) fn remap_plan_blocks(
    builder: &mut MirBuilder,
    block_map: &mut BTreeMap<BasicBlockId, BasicBlockId>,
    plan: &LoweredRecipe,
) -> LoweredRecipe {
    match plan {
        CorePlan::Seq(plans) => CorePlan::Seq(
            plans
                .iter()
                .map(|p| remap_plan_blocks(builder, block_map, p))
                .collect(),
        ),
        CorePlan::If(if_plan) => CorePlan::If(CoreIfPlan {
            condition: if_plan.condition,
            then_plans: if_plan
                .then_plans
                .iter()
                .map(|p| remap_plan_blocks(builder, block_map, p))
                .collect(),
            else_plans: if_plan.else_plans.as_ref().map(|plans| {
                plans
                    .iter()
                    .map(|p| remap_plan_blocks(builder, block_map, p))
                    .collect()
            }),
            joins: if_plan.joins.clone(),
        }),
        CorePlan::Loop(loop_plan) => {
            // Legacy: remap blocks only, no ValueId freshening
            let mut block_map_local = BTreeMap::new();
            freshen_loop_block_ids_in_place(builder, &mut block_map_local, loop_plan.clone())
        }
        CorePlan::BranchN(_) | CorePlan::Effect(_) | CorePlan::Exit(_) => plan.clone(),
    }
}

pub(crate) fn freshen_loop_block_ids(
    builder: &mut MirBuilder,
    mut loop_plan: CoreLoopPlan,
) -> LoweredRecipe {
    let mut block_map = BTreeMap::new();

    loop_plan.preheader_bb = map_block_id(builder, &mut block_map, loop_plan.preheader_bb);
    loop_plan.preheader_is_fresh = true;
    loop_plan.header_bb = map_block_id(builder, &mut block_map, loop_plan.header_bb);
    loop_plan.body_bb = map_block_id(builder, &mut block_map, loop_plan.body_bb);
    loop_plan.step_bb = map_block_id(builder, &mut block_map, loop_plan.step_bb);
    loop_plan.continue_target = map_block_id(builder, &mut block_map, loop_plan.continue_target);
    loop_plan.after_bb = map_block_id(builder, &mut block_map, loop_plan.after_bb);
    loop_plan.found_bb = map_block_id(builder, &mut block_map, loop_plan.found_bb);

    loop_plan.block_effects = loop_plan
        .block_effects
        .into_iter()
        .map(|(block_id, effects)| (map_block_id(builder, &mut block_map, block_id), effects))
        .collect();

    loop_plan.phis = loop_plan
        .phis
        .into_iter()
        .map(|mut phi| {
            phi.block = map_block_id(builder, &mut block_map, phi.block);
            phi.inputs = phi
                .inputs
                .into_iter()
                .map(|(pred, value)| (map_block_id(builder, &mut block_map, pred), value))
                .collect();
            phi
        })
        .collect();

    loop_plan.frag = remap_frag(builder, &mut block_map, loop_plan.frag);

    CorePlan::Loop(loop_plan)
}

/// Freshen loop block IDs in place (legacy compatibility for remap_plan_blocks)
/// Does NOT freshen ValueIds - only remaps block IDs
fn freshen_loop_block_ids_in_place(
    builder: &mut MirBuilder,
    block_map: &mut BTreeMap<BasicBlockId, BasicBlockId>,
    mut loop_plan: CoreLoopPlan,
) -> LoweredRecipe {
    // Freshen block IDs only (inline the logic from freshen_loop_block_ids)
    loop_plan.preheader_bb = map_block_id(builder, block_map, loop_plan.preheader_bb);
    loop_plan.preheader_is_fresh = true;
    loop_plan.header_bb = map_block_id(builder, block_map, loop_plan.header_bb);
    loop_plan.body_bb = map_block_id(builder, block_map, loop_plan.body_bb);
    loop_plan.step_bb = map_block_id(builder, block_map, loop_plan.step_bb);
    loop_plan.continue_target = map_block_id(builder, block_map, loop_plan.continue_target);
    loop_plan.after_bb = map_block_id(builder, block_map, loop_plan.after_bb);
    loop_plan.found_bb = map_block_id(builder, block_map, loop_plan.found_bb);

    loop_plan.block_effects = loop_plan
        .block_effects
        .into_iter()
        .map(|(block_id, effects)| (map_block_id(builder, block_map, block_id), effects))
        .collect();

    loop_plan.phis = loop_plan
        .phis
        .into_iter()
        .map(|mut phi| {
            phi.block = map_block_id(builder, block_map, phi.block);
            phi.inputs = phi
                .inputs
                .into_iter()
                .map(|(pred, value)| (map_block_id(builder, block_map, pred), value))
                .collect();
            phi
        })
        .collect();

    loop_plan.frag = remap_frag(builder, block_map, loop_plan.frag);

    CorePlan::Loop(loop_plan)
}

fn remap_frag(
    builder: &mut MirBuilder,
    block_map: &mut BTreeMap<BasicBlockId, BasicBlockId>,
    frag: Frag,
) -> Frag {
    let entry = map_block_id(builder, block_map, frag.entry);
    let block_params = frag
        .block_params
        .into_iter()
        .map(|(block_id, params)| (map_block_id(builder, block_map, block_id), params))
        .collect();
    let exits = frag
        .exits
        .into_iter()
        .map(|(kind, stubs)| {
            let mapped = stubs
                .into_iter()
                .map(|stub| {
                    edgecfg_stubs::build_edge_stub(
                        map_block_id(builder, block_map, stub.from),
                        stub.kind,
                        stub.target
                            .map(|target| map_block_id(builder, block_map, target)),
                        stub.args,
                    )
                })
                .collect();
            (kind, mapped)
        })
        .collect();
    let wires = frag
        .wires
        .into_iter()
        .map(|stub| {
            edgecfg_stubs::build_edge_stub(
                map_block_id(builder, block_map, stub.from),
                stub.kind,
                stub.target
                    .map(|target| map_block_id(builder, block_map, target)),
                stub.args,
            )
        })
        .collect();
    let branches = frag
        .branches
        .into_iter()
        .map(|branch| {
            edgecfg_stubs::build_branch_stub(
                map_block_id(builder, block_map, branch.from),
                branch.cond,
                map_block_id(builder, block_map, branch.then_target),
                branch.then_args,
                map_block_id(builder, block_map, branch.else_target),
                branch.else_args,
            )
        })
        .collect();
    Frag {
        entry,
        block_params,
        exits,
        wires,
        branches,
    }
}

/// Remap a LoopPlan with both block and ValueId freshening
fn remap_loop_plan(
    builder: &mut MirBuilder,
    block_map: &mut BTreeMap<BasicBlockId, BasicBlockId>,
    value_map: &BTreeMap<ValueId, ValueId>,
    mut loop_plan: CoreLoopPlan,
) -> LoweredRecipe {
    // Remap block IDs (existing logic)
    loop_plan.preheader_bb = map_block_id(builder, block_map, loop_plan.preheader_bb);
    loop_plan.preheader_is_fresh = true;
    loop_plan.header_bb = map_block_id(builder, block_map, loop_plan.header_bb);
    loop_plan.body_bb = map_block_id(builder, block_map, loop_plan.body_bb);
    loop_plan.step_bb = map_block_id(builder, block_map, loop_plan.step_bb);
    loop_plan.continue_target = map_block_id(builder, block_map, loop_plan.continue_target);
    loop_plan.after_bb = map_block_id(builder, block_map, loop_plan.after_bb);
    loop_plan.found_bb = map_block_id(builder, block_map, loop_plan.found_bb);

    // Remap condition values
    loop_plan.cond_loop = remap_value_id(value_map, loop_plan.cond_loop);
    loop_plan.cond_match = remap_value_id(value_map, loop_plan.cond_match);

    // Remap body plans
    loop_plan.body = loop_plan
        .body
        .into_iter()
        .map(|p| remap_plan(builder, block_map, value_map, p))
        .collect();

    // Remap phis
    loop_plan.phis = loop_plan
        .phis
        .into_iter()
        .map(|mut phi| {
            phi.block = map_block_id(builder, block_map, phi.block);
            phi.dst = remap_value_id(value_map, phi.dst);
            phi.inputs = phi
                .inputs
                .into_iter()
                .map(|(pred, value)| {
                    (
                        map_block_id(builder, block_map, pred),
                        remap_value_id(value_map, value),
                    )
                })
                .collect();
            phi
        })
        .collect();

    // Remap block_effects
    loop_plan.block_effects = loop_plan
        .block_effects
        .into_iter()
        .map(|(block_id, effects)| {
            let mapped_block = map_block_id(builder, block_map, block_id);
            let mapped_effects = effects
                .into_iter()
                .map(|mut e| {
                    remap_effect_in_place(value_map, &mut e);
                    e
                })
                .collect();
            (mapped_block, mapped_effects)
        })
        .collect();

    // Remap final_values (references to outer scope, remap only)
    loop_plan.final_values = loop_plan
        .final_values
        .into_iter()
        .map(|(k, v)| (k, remap_value_id(value_map, v)))
        .collect();

    // Remap frag
    loop_plan.frag = remap_frag_with_values(builder, block_map, value_map, loop_plan.frag);

    CorePlan::Loop(loop_plan)
}

/// Remap an effect plan in place
fn remap_effect_in_place(value_map: &BTreeMap<ValueId, ValueId>, effect: &mut CoreEffectPlan) {
    match effect {
        CoreEffectPlan::MethodCall {
            dst,
            object,
            method: _,
            args,
            effects: _,
        } => {
            *dst = dst.map(|d| remap_value_id(value_map, d));
            *object = remap_value_id(value_map, *object);
            *args = remap_value_ids(value_map, args);
        }
        CoreEffectPlan::GlobalCall { dst, func: _, args } => {
            *dst = dst.map(|d| remap_value_id(value_map, d));
            *args = remap_value_ids(value_map, args);
        }
        CoreEffectPlan::ValueCall { dst, callee, args } => {
            *dst = dst.map(|d| remap_value_id(value_map, d));
            *callee = remap_value_id(value_map, *callee);
            *args = remap_value_ids(value_map, args);
        }
        CoreEffectPlan::ExternCall {
            dst,
            iface_name: _,
            method_name: _,
            args,
            effects: _,
        } => {
            *dst = dst.map(|d| remap_value_id(value_map, d));
            *args = remap_value_ids(value_map, args);
        }
        CoreEffectPlan::NewBox {
            dst,
            box_type: _,
            args,
        } => {
            *dst = remap_value_id(value_map, *dst);
            *args = remap_value_ids(value_map, args);
        }
        CoreEffectPlan::FieldGet {
            dst,
            base,
            field: _,
            declared_type: _,
        } => {
            *dst = remap_value_id(value_map, *dst);
            *base = remap_value_id(value_map, *base);
        }
        CoreEffectPlan::FieldSet {
            base,
            field: _,
            value,
            declared_type: _,
        } => {
            *base = remap_value_id(value_map, *base);
            *value = remap_value_id(value_map, *value);
        }
        CoreEffectPlan::BinOp {
            dst,
            lhs,
            op: _,
            rhs,
        } => {
            *dst = remap_value_id(value_map, *dst);
            *lhs = remap_value_id(value_map, *lhs);
            *rhs = remap_value_id(value_map, *rhs);
        }
        CoreEffectPlan::Compare {
            dst,
            lhs,
            op: _,
            rhs,
        } => {
            *dst = remap_value_id(value_map, *dst);
            *lhs = remap_value_id(value_map, *lhs);
            *rhs = remap_value_id(value_map, *rhs);
        }
        CoreEffectPlan::Select {
            dst,
            cond,
            then_val,
            else_val,
        } => {
            *dst = remap_value_id(value_map, *dst);
            *cond = remap_value_id(value_map, *cond);
            *then_val = remap_value_id(value_map, *then_val);
            *else_val = remap_value_id(value_map, *else_val);
        }
        CoreEffectPlan::Const { dst, value: _ } => {
            *dst = remap_value_id(value_map, *dst);
        }
        CoreEffectPlan::Copy { dst, src } => {
            *dst = remap_value_id(value_map, *dst);
            *src = remap_value_id(value_map, *src);
        }
        CoreEffectPlan::ExitIf { cond, exit } => {
            *cond = remap_value_id(value_map, *cond);
            *exit = remap_exit(value_map, exit.clone());
        }
        CoreEffectPlan::IfEffect {
            cond,
            then_effects,
            else_effects,
        } => {
            *cond = remap_value_id(value_map, *cond);
            for e in then_effects {
                remap_effect_in_place(value_map, e);
            }
            if let Some(effects) = else_effects {
                for e in effects {
                    remap_effect_in_place(value_map, e);
                }
            }
        }
    }
}

/// Remap an exit plan
fn remap_exit(value_map: &BTreeMap<ValueId, ValueId>, exit: CoreExitPlan) -> CoreExitPlan {
    match exit {
        CoreExitPlan::Return(value) => {
            CoreExitPlan::Return(value.map(|v| remap_value_id(value_map, v)))
        }
        CoreExitPlan::Break(depth) => CoreExitPlan::Break(depth),
        CoreExitPlan::BreakWithPhiArgs { depth, phi_args } => {
            let remapped_args = phi_args
                .into_iter()
                .map(|(dst, val)| {
                    (
                        remap_value_id(value_map, dst),
                        remap_value_id(value_map, val),
                    )
                })
                .collect();
            CoreExitPlan::BreakWithPhiArgs {
                depth,
                phi_args: remapped_args,
            }
        }
        CoreExitPlan::Continue(depth) => CoreExitPlan::Continue(depth),
        CoreExitPlan::ContinueWithPhiArgs { depth, phi_args } => {
            let remapped_args = phi_args
                .into_iter()
                .map(|(dst, val)| {
                    (
                        remap_value_id(value_map, dst),
                        remap_value_id(value_map, val),
                    )
                })
                .collect();
            CoreExitPlan::ContinueWithPhiArgs {
                depth,
                phi_args: remapped_args,
            }
        }
    }
}

/// Remap a Frag with ValueId freshening (extends remap_frag)
fn remap_frag_with_values(
    builder: &mut MirBuilder,
    block_map: &mut BTreeMap<BasicBlockId, BasicBlockId>,
    value_map: &BTreeMap<ValueId, ValueId>,
    frag: Frag,
) -> Frag {
    let entry = map_block_id(builder, block_map, frag.entry);
    let block_params = frag
        .block_params
        .into_iter()
        .map(|(block_id, params)| {
            let mapped_block = map_block_id(builder, block_map, block_id);
            let remapped_params = params
                .params
                .iter()
                .map(|v| remap_value_id(value_map, *v))
                .collect();
            (
                mapped_block,
                crate::mir::builder::control_flow::plan::edgecfg_facade::BlockParams {
                    params: remapped_params,
                    layout: params.layout,
                },
            )
        })
        .collect();

    let exits = frag
        .exits
        .into_iter()
        .map(|(kind, stubs)| {
            let mapped = stubs
                .into_iter()
                .map(|stub| {
                    edgecfg_stubs::build_edge_stub(
                        map_block_id(builder, block_map, stub.from),
                        stub.kind,
                        stub.target.map(|t| map_block_id(builder, block_map, t)),
                        EdgeArgs {
                            layout: stub.args.layout,
                            values: remap_value_ids(value_map, &stub.args.values),
                        },
                    )
                })
                .collect();
            (kind, mapped)
        })
        .collect();

    let wires = frag
        .wires
        .into_iter()
        .map(|stub| {
            edgecfg_stubs::build_edge_stub(
                map_block_id(builder, block_map, stub.from),
                stub.kind,
                stub.target.map(|t| map_block_id(builder, block_map, t)),
                EdgeArgs {
                    layout: stub.args.layout,
                    values: remap_value_ids(value_map, &stub.args.values),
                },
            )
        })
        .collect();

    let branches = frag
        .branches
        .into_iter()
        .map(|branch| {
            edgecfg_stubs::build_branch_stub(
                map_block_id(builder, block_map, branch.from),
                remap_value_id(value_map, branch.cond),
                map_block_id(builder, block_map, branch.then_target),
                EdgeArgs {
                    layout: branch.then_args.layout,
                    values: remap_value_ids(value_map, &branch.then_args.values),
                },
                map_block_id(builder, block_map, branch.else_target),
                EdgeArgs {
                    layout: branch.else_args.layout,
                    values: remap_value_ids(value_map, &branch.else_args.values),
                },
            )
        })
        .collect();

    Frag {
        entry,
        block_params,
        exits,
        wires,
        branches,
    }
}
