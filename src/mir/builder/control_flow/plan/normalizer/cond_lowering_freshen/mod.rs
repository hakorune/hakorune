//! Plan cloning and block ID freshening.

use crate::mir::builder::control_flow::plan::LoweredRecipe;
use crate::mir::builder::MirBuilder;
use crate::mir::{MirType, ValueId};
use std::collections::{BTreeMap, BTreeSet};

mod collector;
mod remapper;
mod utils;
mod verifier;

use collector::collect_definition_value_ids;
use remapper::{remap_plan, remap_plan_blocks};
use verifier::{find_remap_mismatch, find_unremapped_value_id};

pub(super) struct FreshenedPlans {
    pub(super) plans: Vec<LoweredRecipe>,
    pub(super) value_map: BTreeMap<ValueId, ValueId>,
}

#[track_caller]
pub(super) fn clone_plans_with_fresh_loops(
    builder: &mut MirBuilder,
    plans: &[LoweredRecipe],
) -> Result<FreshenedPlans, String> {
    let strict_planner_required = crate::config::env::joinir_dev::strict_enabled()
        && crate::config::env::joinir_dev::planner_required_enabled();
    let fn_name = builder
        .scope_ctx
        .current_function
        .as_ref()
        .map(|f| f.signature.name.clone())
        .unwrap_or_else(|| "<none>".to_string());
    let caller = std::panic::Location::caller();

    // Important: treat `plans` as a single clone unit.
    //
    // If we freshen ValueIds per-plan (old behavior), definitions in an earlier plan (e.g. `Const dst=%X`)
    // can get remapped while later plans that *use* that ValueId (e.g. `BinOp rhs=%X`) won't see the mapping.
    // That creates "one-sided remap" drift (Const remapped, operand not), which then shows up as undefined
    // operands downstream.
    let mut block_map = BTreeMap::new();
    let mut value_map = BTreeMap::new();

    // Phase 1: allocate fresh ValueIds for all definition sites across the whole plan slice.
    let mut definitions = BTreeSet::new();
    for plan in plans {
        definitions.extend(collect_definition_value_ids(plan));
    }
    for def_id in definitions {
        let ty = builder
            .type_ctx
            .get_type(def_id)
            .cloned()
            .unwrap_or(MirType::Unknown);
        let fresh_id = builder.alloc_typed(ty);
        value_map.insert(def_id, fresh_id);

        // Debug-only: preserve origin span across ValueId alpha-renaming so downstream diagnostics
        // can tell whether an ID drift is a remap bug vs an attach/drop bug.
        if crate::config::env::joinir_dev::debug_enabled() {
            let span_opt = builder.metadata_ctx.value_span(def_id);
            if let Some(span) = span_opt {
                builder.metadata_ctx.record_value_span(fresh_id, span);
            }
        }
    }

    if strict_planner_required {
        let reserved: Vec<ValueId> = value_map.values().copied().collect();
        crate::mir::builder::emission::value_lifecycle::verify_reserved_values_not_exposed(
            builder,
            &reserved,
            "cond_freshen:clone_plans_with_fresh_loops",
        )?;
    }

    // Phase 2: remap all plans using the same block/value maps.
    let mut out = Vec::with_capacity(plans.len());
    for plan in plans.iter().cloned() {
        let remapped = remap_plan(builder, &mut block_map, &value_map, plan);
        if strict_planner_required {
            if crate::config::env::joinir_dev::debug_enabled() {
                if let Some((old, new, operand, dst, op)) =
                    find_remap_mismatch(&remapped, &value_map)
                {
                    return Err(format!(
                        "[freeze:contract][cond_freshen/remap_mismatch] fn={} old=%{} new=%{} use_by=CoreEffectPlan::BinOp operand={} dst=%{} op={:?}",
                        fn_name, old.0, new.0, operand, dst.0, op
                    ));
                }
            }
            if let Some((old, new, site)) = find_unremapped_value_id(&remapped, &value_map) {
                return Err(format!(
                    "[freeze:contract][cond_freshen/unremapped_valueid] fn={} old=%{} new=%{} site={} caller={}",
                    fn_name, old.0, new.0, site, caller
                ));
            }
        }
        out.push(remapped);
    }

    Ok(FreshenedPlans {
        plans: out,
        value_map,
    })
}

fn freshen_plan_block_ids(builder: &mut MirBuilder, plan: LoweredRecipe) -> LoweredRecipe {
    // Legacy: only freshens block IDs, no ValueId freshening
    // This is kept for compatibility with internal calls within this module
    let mut block_map = BTreeMap::new();
    remap_plan_blocks(builder, &mut block_map, &plan)
}

/// Freshen both block IDs and ValueIds in a plan (SSA-compliant cloning)
fn freshen_plan_ids(builder: &mut MirBuilder, plan: LoweredRecipe) -> LoweredRecipe {
    let mut block_map = BTreeMap::new();
    let mut value_map = BTreeMap::new();

    // Phase 1: Collect definitions and allocate fresh ValueIds
    let definitions = collect_definition_value_ids(&plan);
    for def_id in definitions {
        let ty = builder
            .type_ctx
            .get_type(def_id)
            .cloned()
            .unwrap_or(MirType::Unknown);
        let fresh_id = builder.alloc_typed(ty);
        value_map.insert(def_id, fresh_id);
    }

    // Phase 2: Remap plan with both block_map and value_map
    remap_plan(builder, &mut block_map, &value_map, plan)
}
