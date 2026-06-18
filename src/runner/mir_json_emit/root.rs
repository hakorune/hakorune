use super::decls::{
    collect_array_record_autouse_eligibility_plan_values,
    collect_array_record_materialization_boundary_plan_values,
    collect_array_record_packed_autouse_pilot_plan_values,
    collect_array_record_storage_plan_values, collect_direct_state_plan_values,
    collect_hako_alloc_aligned_small_packed_store_pilot_plan_values,
    collect_hako_alloc_huge_page_packed_store_pilot_plan_values,
    collect_object_storage_plan_values, collect_record_layout_plan_values,
    collect_record_state_residence_plan_values, collect_sorted_enum_decl_values,
    collect_sorted_record_decl_values, collect_sorted_user_box_decl_values,
    collect_source_packed_array_autouse_pilot_plan_values,
    collect_source_packed_array_direct_read_consumption_plan_values,
    collect_static_data_plan_values, collect_typed_object_plan_values,
};
use super::emitters;
use super::helpers;
use super::metadata::build_function_metadata_json;
use super::order::ordered_harness_functions;
use crate::runner::mir_json_export_model;
use serde_json::json;

fn insert_root_metadata(
    root: &mut serde_json::Value,
    entries: Vec<(&'static str, serde_json::Value)>,
) {
    if let Some(obj) = root.as_object_mut() {
        for (key, value) in entries {
            obj.insert(key.to_string(), value);
        }
    }
}

pub(super) fn build_mir_json_root(
    module: &crate::mir::MirModule,
) -> Result<serde_json::Value, String> {
    let mut funs = Vec::new();
    for (name, f) in ordered_harness_functions(module) {
        let mut blocks = Vec::new();
        let mut ids: Vec<_> = f.blocks.keys().copied().collect();
        ids.sort();
        for bid in ids {
            if let Some(bb) = f.blocks.get(&bid) {
                let mut insts = Vec::new();
                // Phase 131-13: Emit all instructions in MIR order (SSOT principle)
                // No reordering except PHI consolidation at block start (LLVM constraint)

                // Step 1: Emit all PHI instructions first (LLVM requirement)
                insts.extend(emitters::emit_phi_instructions(f, bb));

                // Step 2: Emit all non-PHI instructions in MIR order (no reordering!)
                emitters::emit_non_phi_instructions(f, bb, &mut insts)?;

                // Phase 131-13: Terminator emitted inline (no delayed copies)
                if let Some(term) = emitters::emit_terminator(&bb.terminator)? {
                    insts.push(term);
                }
                blocks.push(json!({"id": bid.as_u32(), "instructions": insts}));
            }
        }
        // Export parameter value-ids so a VM can bind arguments
        let params: Vec<_> = f.params.iter().map(|v| v.as_u32()).collect();

        // Phase 131-11-F: Build metadata JSON from MIR metadata (SSOT)
        let metadata_json = build_function_metadata_json(f);
        let attrs_json = json!({
            "runes": f
                .metadata
                .runes
                .iter()
                .map(|rune| json!({"name": rune.name, "args": rune.args}))
                .collect::<Vec<_>>()
        });
        funs.push(json!({
            "name": name,
            "params": params,
            "blocks": blocks,
            "metadata": metadata_json,
            "attrs": attrs_json
        }));
    }

    // Phase 15.5: JSON v1 schema with environment variable control
    let use_v1_schema = std::env::var("NYASH_JSON_SCHEMA_V1").unwrap_or_default() == "1"
        || match std::env::var("NYASH_MIR_UNIFIED_CALL")
            .ok()
            .as_deref()
            .map(|s| s.to_ascii_lowercase())
        {
            Some(s) if s == "0" || s == "false" || s == "off" => false,
            _ => true,
        };

    // Phase 155: Extract CFG information for hako_check
    let cfg_info = nyash_rust::mir::cfg_extractor::extract_cfg_info(module);

    // Phase 285LLVM-1.1+: shared root metadata for both JSON v1 and legacy v0.
    // Keep this list as the single insertion point when a new top-level plan
    // surface is added.
    let root_metadata = vec![
        ("cfg", cfg_info),
        (
            "user_box_decls",
            json!(collect_sorted_user_box_decl_values(module)),
        ),
        (
            "record_decls",
            json!(collect_sorted_record_decl_values(module)),
        ),
        (
            "typed_object_plans",
            json!(collect_typed_object_plan_values(module)),
        ),
        (
            "object_storage_plans",
            json!(collect_object_storage_plan_values(module)),
        ),
        (
            "direct_state_plans",
            json!(collect_direct_state_plan_values(module)),
        ),
        (
            "record_state_residence_plans",
            json!(collect_record_state_residence_plan_values(module)),
        ),
        (
            "record_layout_plans",
            json!(collect_record_layout_plan_values(module)),
        ),
        (
            "array_record_storage_plans",
            json!(collect_array_record_storage_plan_values(module)),
        ),
        (
            "array_record_autouse_eligibility_plans",
            json!(collect_array_record_autouse_eligibility_plan_values(module)),
        ),
        (
            "array_record_materialization_boundary_plans",
            json!(collect_array_record_materialization_boundary_plan_values(
                module
            )),
        ),
        (
            "array_record_packed_autouse_pilot_plans",
            json!(collect_array_record_packed_autouse_pilot_plan_values(
                module
            )),
        ),
        (
            "source_packed_array_autouse_pilot_plans",
            json!(collect_source_packed_array_autouse_pilot_plan_values(
                module
            )),
        ),
        (
            "source_packed_array_direct_read_consumption_plans",
            json!(collect_source_packed_array_direct_read_consumption_plan_values(module)),
        ),
        (
            "hako_alloc_aligned_small_packed_store_pilot_plans",
            json!(collect_hako_alloc_aligned_small_packed_store_pilot_plan_values(module)),
        ),
        (
            "hako_alloc_huge_page_packed_store_pilot_plans",
            json!(collect_hako_alloc_huge_page_packed_store_pilot_plan_values(
                module
            )),
        ),
        (
            "static_data_plans",
            json!(collect_static_data_plan_values(module)),
        ),
        ("enum_decls", json!(collect_sorted_enum_decl_values(module))),
    ];
    let export_summary =
        mir_json_export_model::summarize_root(use_v1_schema, funs.len(), root_metadata.len());
    debug_assert_eq!(export_summary.function_count, funs.len());
    debug_assert_eq!(
        export_summary.root_metadata_entry_count,
        root_metadata.len()
    );

    let mut root = if use_v1_schema {
        helpers::create_json_v1_root(json!(funs))
    } else {
        json!({ "functions": funs })
    };
    insert_root_metadata(&mut root, root_metadata);

    // NOTE: numeric_core strict validation is applied on the AotPrep output
    // (tools/hakorune_emit_mir.sh) rather than at raw MIR emit time. This keeps
    // pre-AotPrep MIR emission usable even when BoxCall(MatI64, mul_naive) is
    // still present.
    Ok(root)
}
