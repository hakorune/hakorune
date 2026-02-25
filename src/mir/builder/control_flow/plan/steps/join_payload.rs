//! Step: Join payload generation (If-merge PHI construction).
//! (plan::steps SSOT)
//!
//! Input: pre_if_map, then_map, else_map (3-map diff)
//! Output: Result<Vec<CoreIfJoin>, String> with pre-allocated dst values
//! Fail-Fast (strict/dev + planner_required):
//! - pre var missing in both branches → contract freeze
//! Note: Type resolution fallback → Unknown (挙動不変)

use crate::mir::builder::control_flow::plan::planner::Freeze;
use crate::mir::builder::control_flow::plan::CoreIfJoin;
use crate::mir::builder::MirBuilder;
use crate::mir::{MirType, ValueId};
use crate::config::env::joinir_dev;
use std::collections::BTreeMap;

#[track_caller]
fn build_join_for_name(
    builder: &mut MirBuilder,
    name: &str,
    pre_val: ValueId,
    then_val: ValueId,
    else_val: ValueId,
) -> Option<CoreIfJoin> {
    // Skip if no change from pre-if value (3-map diff)
    if then_val == pre_val && else_val == pre_val {
        return None;
    }

    // Resolve type (prefer then, fallback else, fallback Unknown)
    // NOTE: Unknown fallback is intentional for 挙動不変
    let ty = builder
        .type_ctx
        .get_type(then_val)
        .cloned()
        .or_else(|| builder.type_ctx.get_type(else_val).cloned())
        .unwrap_or(MirType::Unknown);

    let dst = builder.alloc_typed(ty);

    if joinir_dev::strict_planner_required_debug_enabled() {
        let fn_name = builder
            .scope_ctx
            .current_function
            .as_ref()
            .map(|f| f.signature.name.as_str())
            .unwrap_or("<none>");
        let span = builder.metadata_ctx.current_span();
        let file = builder
            .metadata_ctx
            .current_source_file()
            .unwrap_or_else(|| "unknown".to_string());
        let pre_span = builder
            .metadata_ctx
            .value_span(pre_val)
            .map(|s| s.to_string())
            .unwrap_or_else(|| "unknown".to_string());
        let then_span = builder
            .metadata_ctx
            .value_span(then_val)
            .map(|s| s.to_string())
            .unwrap_or_else(|| "unknown".to_string());
        let else_span = builder
            .metadata_ctx
            .value_span(else_val)
            .map(|s| s.to_string())
            .unwrap_or_else(|| "unknown".to_string());
        let caller = std::panic::Location::caller();
        let ring0 = crate::runtime::get_global_ring0();
        ring0.log.debug(&format!(
            "[if_join/payload] fn={} name={} dst=%{} pre=%{} then=%{} else=%{} span={} file={} pre_span={} then_span={} else_span={} caller={}:{}:{}",
            fn_name,
            name,
            dst.0,
            pre_val.0,
            then_val.0,
            else_val.0,
            span,
            file,
            pre_span,
            then_span,
            else_span,
            caller.file(),
            caller.line(),
            caller.column()
        ));
    }

    Some(CoreIfJoin {
        name: name.to_string(),
        dst,
        pre_val: Some(pre_val),
        then_val,
        else_val,
    })
}

/// Build join payload from 3-map diff.
///
/// # Contract
/// - Creates joins only for variables that differ between branches
/// - Allocates dst ValueId for each join
/// - Does NOT update variable_map (caller's responsibility)
/// - Returns empty Vec if no variables differ (not an error)
/// - Type fallback: then → else → Unknown (挙動不変のため)
pub fn build_join_payload(
    builder: &mut MirBuilder,
    pre_if_map: &BTreeMap<String, ValueId>,
    then_map: &BTreeMap<String, ValueId>,
    else_map: &BTreeMap<String, ValueId>,
) -> Result<Vec<CoreIfJoin>, String> {
    let strict_or_dev = joinir_dev::strict_enabled() || crate::config::env::joinir_dev_enabled();
    let planner_required = strict_or_dev && joinir_dev::planner_required_enabled();
    if planner_required {
        for name in pre_if_map.keys() {
            if !then_map.contains_key(name) && !else_map.contains_key(name) {
                let freeze =
                    Freeze::contract("if_join: pre var missing in both branches");
                return Err(freeze.to_string());
            }
        }
    }
    let mut joins = Vec::new();

    for (name, pre_val) in pre_if_map {
        let then_val = then_map.get(name).copied().unwrap_or(*pre_val);
        let else_val = else_map.get(name).copied().unwrap_or(*pre_val);
        if let Some(join) = build_join_for_name(builder, name, *pre_val, then_val, else_val) {
            joins.push(join);
        }
    }

    Ok(joins)
}

/// Build join payload filtered to specific variables.
///
/// Only creates joins for variables in `filter_vars`.
/// Does NOT update variable_map (caller's responsibility).
pub fn build_join_payload_filtered<'a, I>(
    builder: &mut MirBuilder,
    pre_if_map: &BTreeMap<String, ValueId>,
    then_map: &BTreeMap<String, ValueId>,
    else_map: &BTreeMap<String, ValueId>,
    filter_vars: I,
) -> Vec<CoreIfJoin>
where
    I: IntoIterator<Item = &'a String>,
{
    let mut joins = Vec::new();

    for name in filter_vars {
        let Some(&pre_val) = pre_if_map.get(name) else {
            continue; // Variable not in pre-map, skip
        };

        let then_val = then_map.get(name).copied().unwrap_or(pre_val);
        let else_val = else_map.get(name).copied().unwrap_or(pre_val);
        if let Some(join) = build_join_for_name(builder, name, pre_val, then_val, else_val) {
            joins.push(join);
        }
    }

    joins
}
