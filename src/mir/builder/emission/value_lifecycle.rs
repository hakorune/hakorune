//! Value lifecycle contract (typed → defined) fail-fast utilities.
//!
//! Goal: shorten diagnosis distance for SSA/dominator failures by catching
//! "typed but not defined" ValueIds at the function-finalization boundary.
//!
//! Policy:
//! - This is a **contract** check (no rewrite, no workaround).
//! - Enabled only in strict/dev + planner_required to keep release default behavior unchanged.

use crate::mir::builder::joinir_id_remapper::JoinIrIdRemapper;
use crate::mir::builder::MirBuilder;
use crate::mir::verification::utils::compute_def_blocks;
use crate::mir::{MirFunction, MirType, ValueId};
use std::collections::HashSet;

fn strict_or_dev_planner_required() -> bool {
    let strict_or_dev = crate::config::env::joinir_dev::strict_enabled()
        || crate::config::env::joinir_dev_enabled();
    strict_or_dev && crate::config::env::joinir_dev::planner_required_enabled()
}

fn format_value_ids(values: &[ValueId]) -> String {
    let mut out = String::from("[");
    for (idx, v) in values.iter().enumerate() {
        if idx > 0 {
            out.push(',');
        }
        out.push('%');
        out.push_str(&v.0.to_string());
    }
    out.push(']');
    out
}

fn format_varmap_hits(builder: &MirBuilder, v: ValueId) -> String {
    let mut hits: Vec<&str> = builder
        .variable_ctx
        .variable_map
        .iter()
        .filter(|(_, &vid)| vid == v)
        .map(|(name, _)| name.as_str())
        .take(4)
        .collect();
    hits.sort();
    if hits.is_empty() {
        "[]".to_string()
    } else {
        format!("[{}]", hits.join(","))
    }
}

/// Fail-fast if `type_ctx.value_types` contains ValueIds that have no in-function definition.
///
/// This catches temporal coupling bugs early (typed-but-undef) before downstream dominator/SSA errors.
pub(in crate::mir::builder) fn verify_typed_values_are_defined(
    builder: &mut MirBuilder,
    tag: &str,
) -> Result<(), String> {
    if !strict_or_dev_planner_required() {
        return Ok(());
    }
    let Some(func) = builder.scope_ctx.current_function.as_ref() else {
        return Ok(());
    };

    let def_blocks = compute_def_blocks(func);
    let param_set = &func.params;

    let is_defined = |v: &ValueId| -> bool {
        *v != ValueId::INVALID && (def_blocks.contains_key(v) || param_set.iter().any(|p| p == v))
    };

    let mut missing: Vec<(ValueId, MirType)> = builder
        .type_ctx
        .value_types
        .iter()
        .filter(|(v, _ty)| !is_defined(v))
        .map(|(v, ty)| (*v, ty.clone()))
        .collect();

    if missing.is_empty() {
        return Ok(());
    }

    // A small amount of `type_ctx` drift can happen when a reserved ValueId gets typed
    // but the defining instruction is never emitted (and the value is never referenced).
    // This is behavior-neutral, but would create noise for this contract check.
    //
    // Policy: only fail-fast if the missing ValueId is actually referenced by the function
    // (or still present in builder-side pending structures). Otherwise, prune stale entries.
    let referenced = collect_referenced_values(func);
    let pending_phi_dsts: HashSet<ValueId> =
        builder.pending_phis.iter().map(|(_bb, v, _)| *v).collect();

    let is_fatal_missing = |v: &ValueId| -> bool {
        referenced.contains(v)
            || pending_phi_dsts.contains(v)
            || builder.pin_slot_names.contains_key(v)
        // Metadata caller tables are process-global diagnostics and may still carry
        // same-numbered ValueIds from previously lowered functions. They are not
        // a semantic "use" signal for the current function boundary contract.
    };

    let mut stale: Vec<ValueId> = Vec::new();
    missing.retain(|(v, _)| {
        if is_fatal_missing(v) {
            true
        } else {
            stale.push(*v);
            false
        }
    });

    if missing.is_empty() {
        for v in stale {
            builder.type_ctx.value_types.remove(&v);
            builder.type_ctx.value_kinds.remove(&v);
            builder.type_ctx.value_origin_newbox.remove(&v);
        }
        return Ok(());
    }

    missing.sort_by_key(|(v, _)| v.0);
    let missing_count = missing.len();
    let missing_ids: Vec<ValueId> = missing.iter().take(3).map(|(v, _)| *v).collect();
    let missing_list = format_value_ids(&missing_ids);

    let (v0, ty0) = missing[0].clone();
    let span = builder.metadata_ctx.current_span();
    let file = builder
        .metadata_ctx
        .current_source_file()
        .unwrap_or_else(|| "unknown".to_string());
    let value_caller = builder.metadata_ctx.value_caller(v0).unwrap_or("none");
    let pin = builder
        .pin_slot_names
        .get(&v0)
        .map(|s| s.as_str())
        .unwrap_or("none");
    let varmap_hits = format_varmap_hits(builder, v0);

    Err(format!(
        "[freeze:contract][value_lifecycle/typed_without_def] fn={} tag={} missing_count={} missing0=%{} missing0_ty={:?} missing={} typed_count={} def_count={} span={} span_start={} span_end={} file={} value_caller={} pin={} varmap_hits={}",
        func.signature.name,
        tag,
        missing_count,
        v0.0,
        ty0,
        missing_list,
        builder.type_ctx.value_types.len(),
        def_blocks.len(),
        span.location_string(),
        span.start,
        span.end,
        file,
        value_caller,
        pin,
        varmap_hits
    ))
}

/// Fail-fast if reserve-only ValueIds are exposed to variable_map.
///
/// Reserve-only IDs are allowed in internal freshening maps, but must not be
/// published as the current binding of any variable before they are Defined.
#[track_caller]
pub(in crate::mir::builder) fn verify_reserved_values_not_exposed(
    builder: &MirBuilder,
    reserved: &[ValueId],
    tag: &str,
) -> Result<(), String> {
    if !strict_or_dev_planner_required() || reserved.is_empty() {
        return Ok(());
    }
    let Some(func) = builder.scope_ctx.current_function.as_ref() else {
        return Ok(());
    };

    let reserved_set: HashSet<ValueId> = reserved
        .iter()
        .copied()
        .filter(|v| *v != ValueId::INVALID)
        .collect();
    if reserved_set.is_empty() {
        return Ok(());
    }

    let mut hits: Vec<(&str, ValueId)> = builder
        .variable_ctx
        .variable_map
        .iter()
        .filter_map(|(name, &vid)| {
            if reserved_set.contains(&vid) {
                Some((name.as_str(), vid))
            } else {
                None
            }
        })
        .collect();

    if hits.is_empty() {
        return Ok(());
    }

    hits.sort_by(|(a_name, a_vid), (b_name, b_vid)| {
        a_vid.0.cmp(&b_vid.0).then_with(|| a_name.cmp(b_name))
    });
    let (hit_var, hit_vid) = hits[0];

    let mut reserved_ids: Vec<ValueId> = reserved_set.iter().copied().collect();
    reserved_ids.sort_by_key(|v| v.0);
    let reserved_preview: Vec<ValueId> = reserved_ids.iter().take(3).copied().collect();
    let reserved_str = format_value_ids(&reserved_preview);

    let caller = std::panic::Location::caller();
    Err(format!(
        "[freeze:contract][value_lifecycle/reserved_exposed_in_varmap] fn={} tag={} reserved_count={} reserved={} hit_var={} hit_v=%{} caller={}:{}:{}",
        func.signature.name,
        tag,
        reserved_ids.len(),
        reserved_str,
        hit_var,
        hit_vid.0,
        caller.file(),
        caller.line(),
        caller.column()
    ))
}

fn collect_referenced_values(func: &MirFunction) -> HashSet<ValueId> {
    let remapper = JoinIrIdRemapper::new();
    let mut out: HashSet<ValueId> = HashSet::new();
    let reachable = crate::mir::verification::utils::compute_reachable_blocks(func);
    for (block_id, block) in &func.blocks {
        if !reachable.contains(block_id) {
            continue;
        }
        for v in remapper.collect_values_in_block(block) {
            out.insert(v);
        }
    }
    for p in &func.params {
        out.insert(*p);
    }
    out
}
