use serde_json::json;
use std::io::Write;

mod emitters;
mod helpers;

/// Emit MIR JSON for Python harness/PyVM.
/// The JSON schema matches tools/llvmlite_harness.py expectations and is
/// intentionally minimal for initial scaffolding.
///
/// Phase 15.5: Supports both v0 (legacy separate ops) and v1 (unified mir_call) formats
pub fn emit_mir_json_for_harness(
    module: &nyash_rust::mir::MirModule,
    path: &std::path::Path,
) -> Result<(), String> {
    emit_mir_json(module, path)
}

/// Variant for the bin crate's local MIR type
pub fn emit_mir_json_for_harness_bin(
    module: &crate::mir::MirModule,
    path: &std::path::Path,
) -> Result<(), String> {
    emit_mir_json(module, path)
}

fn emit_mir_json(module: &crate::mir::MirModule, path: &std::path::Path) -> Result<(), String> {
    use crate::mir::MirType;

    let mut funs = Vec::new();
    for (name, f) in &module.functions {
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
        let metadata_json = json!({
            "value_types": f.metadata.value_types.iter().map(|(k, v)| {
                let type_str = match v {
                    MirType::Integer => json!("i64"),
                    MirType::Float => json!("f64"),  // Phase 275 P0: Float type annotation
                    MirType::String => json!({"kind": "string"}),
                    MirType::Box(bt) => json!({"kind": "handle", "box_type": bt}),
                    MirType::Bool => json!("i1"),
                    MirType::Void => json!("void"),
                    MirType::Unknown => json!(null),
                    _ => json!(null),
                };
                (k.as_u32().to_string(), type_str)
            }).collect::<serde_json::Map<String, serde_json::Value>>()
        });

        funs.push(json!({
            "name": name,
            "params": params,
            "blocks": blocks,
            "metadata": metadata_json
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
    let cfg_info = nyash_rust::mir::extract_cfg_info(module);

    // Phase 285LLVM-1.1: Extract user box declarations for LLVM harness
    let user_box_decls: Vec<serde_json::Value> = module
        .metadata
        .user_box_decls
        .iter()
        .map(|(name, fields)| {
            json!({
                "name": name,
                "fields": fields
            })
        })
        .collect();

    let root = if use_v1_schema {
        let mut root = helpers::create_json_v1_root(json!(funs));
        // Add CFG data and user box declarations to v1 schema
        if let Some(obj) = root.as_object_mut() {
            obj.insert("cfg".to_string(), cfg_info);
            obj.insert("user_box_decls".to_string(), json!(user_box_decls)); // Phase 285LLVM-1.1
        }
        root
    } else {
        // v0 legacy format - also add CFG and user_box_decls
        json!({
            "functions": funs,
            "cfg": cfg_info,
            "user_box_decls": user_box_decls  // Phase 285LLVM-1.1
        })
    };

    // NOTE: numeric_core strict validation is applied on the AotPrep output
    // (tools/hakorune_emit_mir.sh) rather than at raw MIR emit time. This keeps
    // pre-AotPrep MIR emission usable even when BoxCall(MatI64, mul_naive) is
    // still present.

    let file = std::fs::File::create(path).map_err(|e| format!("write mir json: {}", e))?;
    let mut writer = std::io::BufWriter::new(file);
    serde_json::to_writer_pretty(&mut writer, &root)
        .map_err(|e| format!("write mir json: {}", e))?;
    writer
        .write_all(b"\n")
        .map_err(|e| format!("write mir json: {}", e))?;
    writer.flush().map_err(|e| format!("write mir json: {}", e))
}
