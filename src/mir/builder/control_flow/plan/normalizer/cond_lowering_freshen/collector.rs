use crate::mir::builder::control_flow::plan::{CoreEffectPlan, CorePlan, LoweredRecipe};
use crate::mir::ValueId;
use std::collections::BTreeSet;

/// Collect all ValueId definitions (dst fields ONLY) from a plan.
///
/// Only collects dst fields (definition sites) that need fresh allocation:
/// - CoreIfJoin.dst
/// - CorePhiInfo.dst
/// - CoreEffectPlan dst fields (Const, Copy, NewBox, BinOp, Compare, Select)
/// - CoreEffectPlan Option<dst> (MethodCall, GlobalCall, ValueCall, ExternCall)
///
/// Does NOT collect (these are uses, remapped via value_map):
/// - CoreLoopPlan.final_values (references to outer scope, not definitions)
/// - condition values, phi inputs, args
pub(crate) fn collect_definition_value_ids(plan: &LoweredRecipe) -> BTreeSet<ValueId> {
    let mut defs = BTreeSet::new();
    collect_definition_value_ids_recursive(plan, &mut defs);
    defs
}

fn collect_definition_value_ids_recursive(plan: &LoweredRecipe, defs: &mut BTreeSet<ValueId>) {
    match plan {
        CorePlan::Seq(plans) => {
            for p in plans {
                collect_definition_value_ids_recursive(p, defs);
            }
        }
        CorePlan::If(if_plan) => {
            // Collect CoreIfJoin.dst values
            for join in &if_plan.joins {
                defs.insert(join.dst);
            }
            // Recurse into branches
            for p in &if_plan.then_plans {
                collect_definition_value_ids_recursive(p, defs);
            }
            if let Some(else_plans) = &if_plan.else_plans {
                for p in else_plans {
                    collect_definition_value_ids_recursive(p, defs);
                }
            }
        }
        CorePlan::Loop(loop_plan) => {
            // Collect PHI dsts
            for phi in &loop_plan.phis {
                defs.insert(phi.dst);
            }
            // Recurse into body
            for p in &loop_plan.body {
                collect_definition_value_ids_recursive(p, defs);
            }
            // Collect block_effects definitions
            for (_, effects) in &loop_plan.block_effects {
                for effect in effects {
                    collect_effect_definitions(effect, defs);
                }
            }
            // Note: final_values are NOT definitions (they're references to outer scope)
        }
        CorePlan::BranchN(branch_n) => {
            for arm in &branch_n.arms {
                for p in &arm.plans {
                    collect_definition_value_ids_recursive(p, defs);
                }
            }
            if let Some(else_plans) = &branch_n.else_plans {
                for p in else_plans {
                    collect_definition_value_ids_recursive(p, defs);
                }
            }
        }
        CorePlan::Effect(effect) => {
            collect_effect_definitions(effect, defs);
        }
        CorePlan::Exit(_) => {} // No definitions in exit plans
    }
}

fn collect_effect_definitions(effect: &CoreEffectPlan, defs: &mut BTreeSet<ValueId>) {
    match effect {
        CoreEffectPlan::MethodCall { dst, .. } => {
            if let Some(d) = dst {
                defs.insert(*d);
            }
        }
        CoreEffectPlan::GlobalCall { dst, .. } => {
            if let Some(d) = dst {
                defs.insert(*d);
            }
        }
        CoreEffectPlan::ValueCall { dst, .. } => {
            if let Some(d) = dst {
                defs.insert(*d);
            }
        }
        CoreEffectPlan::ExternCall { dst, .. } => {
            if let Some(d) = dst {
                defs.insert(*d);
            }
        }
        CoreEffectPlan::NewBox { dst, .. } => {
            defs.insert(*dst);
        }
        CoreEffectPlan::BinOp { dst, .. } => {
            defs.insert(*dst);
        }
        CoreEffectPlan::Compare { dst, .. } => {
            defs.insert(*dst);
        }
        CoreEffectPlan::Select { dst, .. } => {
            defs.insert(*dst);
        }
        CoreEffectPlan::Const { dst, .. } => {
            defs.insert(*dst);
        }
        CoreEffectPlan::Copy { dst, .. } => {
            defs.insert(*dst);
        }
        CoreEffectPlan::ExitIf { .. } | CoreEffectPlan::IfEffect { .. } => {} // No dst
    }
}
