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

pub fn emit_mir_json_string_for_harness_bin(
    module: &crate::mir::MirModule,
) -> Result<String, String> {
    let root = build_mir_json_root(module)?;
    serialize_mir_json_root(&root)
}

fn emit_mir_json(module: &crate::mir::MirModule, path: &std::path::Path) -> Result<(), String> {
    let root = build_mir_json_root(module)?;
    write_mir_json_root(path, &root)
}

fn build_mir_json_root(module: &crate::mir::MirModule) -> Result<serde_json::Value, String> {
    use crate::mir::MirType;

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
            }).collect::<serde_json::Map<String, serde_json::Value>>(),
            "storage_classes": f.metadata.value_storage_classes.iter().map(|(k, v)| {
                (k.as_u32().to_string(), json!(v.to_string()))
            }).collect::<serde_json::Map<String, serde_json::Value>>()
        });
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
    let cfg_info = nyash_rust::mir::extract_cfg_info(module);

    // Phase 285LLVM-1.1: Extract user box declarations for LLVM harness
    let user_box_decls = collect_sorted_user_box_decl_values(module);
    let enum_decls = collect_sorted_enum_decl_values(module);

    let root = if use_v1_schema {
        let mut root = helpers::create_json_v1_root(json!(funs));
        // Add CFG data and user box declarations to v1 schema
        if let Some(obj) = root.as_object_mut() {
            obj.insert("cfg".to_string(), cfg_info);
            obj.insert("user_box_decls".to_string(), json!(user_box_decls)); // Phase 285LLVM-1.1
            obj.insert("enum_decls".to_string(), json!(enum_decls));
        }
        root
    } else {
        // v0 legacy format - also add CFG and user_box_decls
        json!({
            "functions": funs,
            "cfg": cfg_info,
            "user_box_decls": user_box_decls,  // Phase 285LLVM-1.1
            "enum_decls": enum_decls
        })
    };

    // NOTE: numeric_core strict validation is applied on the AotPrep output
    // (tools/hakorune_emit_mir.sh) rather than at raw MIR emit time. This keeps
    // pre-AotPrep MIR emission usable even when BoxCall(MatI64, mul_naive) is
    // still present.
    Ok(root)
}

fn ordered_harness_functions<'a>(
    module: &'a crate::mir::MirModule,
) -> Vec<(&'a String, &'a crate::mir::MirFunction)> {
    let mut functions: Vec<_> = module.functions.iter().collect();
    functions.sort_by(|(lhs_name, lhs_func), (rhs_name, rhs_func)| {
        harness_entry_priority(lhs_name, lhs_func)
            .cmp(&harness_entry_priority(rhs_name, rhs_func))
            .then_with(|| lhs_name.cmp(rhs_name))
    });
    functions
}

fn harness_entry_priority(name: &str, func: &crate::mir::MirFunction) -> (u8, u8) {
    if func.metadata.is_entry_point {
        return (0, 0);
    }
    if name == "main" {
        return (0, 1);
    }
    if name == "ny_main" {
        return (0, 2);
    }
    if name.ends_with(".main/0") || name.ends_with(".main/1") {
        return (1, 0);
    }
    (2, 0)
}

fn serialize_mir_json_root(root: &serde_json::Value) -> Result<String, String> {
    serde_json::to_string(root).map_err(|e| format!("write mir json: {}", e))
}

fn collect_sorted_user_box_decl_values(module: &crate::mir::MirModule) -> Vec<serde_json::Value> {
    let mut names = std::collections::BTreeSet::new();
    names.extend(module.metadata.user_box_decls.keys().cloned());
    names.extend(module.metadata.user_box_field_decls.keys().cloned());

    names
        .into_iter()
        .map(|name| {
            let field_decls = module
                .metadata
                .user_box_field_decls
                .get(&name)
                .cloned()
                .unwrap_or_default();
            let fields = module
                .metadata
                .user_box_decls
                .get(&name)
                .cloned()
                .unwrap_or_else(|| field_decls.iter().map(|decl| decl.name.clone()).collect());

            json!({
                "name": name,
                "fields": fields,
                "field_decls": field_decls.into_iter().map(|decl| json!({
                    "name": decl.name,
                    "declared_type": decl.declared_type_name,
                    "is_weak": decl.is_weak,
                })).collect::<Vec<_>>(),
            })
        })
        .collect()
}

fn collect_sorted_enum_decl_values(module: &crate::mir::MirModule) -> Vec<serde_json::Value> {
    module
        .metadata
        .enum_decls
        .iter()
        .map(|(name, decl)| {
            json!({
                "name": name,
                "type_parameters": decl.type_parameters,
                "variants": decl.variants.iter().map(|variant| json!({
                    "name": variant.name,
                    "payload_type": variant.payload_type_name,
                })).collect::<Vec<_>>(),
            })
        })
        .collect()
}

fn write_mir_json_root(path: &std::path::Path, root: &serde_json::Value) -> Result<(), String> {
    let file = std::fs::File::create(path).map_err(|e| format!("write mir json: {}", e))?;
    let mut writer = std::io::BufWriter::new(file);
    serde_json::to_writer_pretty(&mut writer, &root)
        .map_err(|e| format!("write mir json: {}", e))?;
    writer
        .write_all(b"\n")
        .map_err(|e| format!("write mir json: {}", e))?;
    writer.flush().map_err(|e| format!("write mir json: {}", e))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::RuneAttr;
    use crate::mir::{
        BasicBlockId, EffectMask, FunctionSignature, MirFunction, MirModule, MirType,
    };

    fn make_function(name: &str, is_entry_point: bool) -> MirFunction {
        let signature = FunctionSignature {
            name: name.to_string(),
            params: vec![],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        };
        let mut function = MirFunction::new(signature, BasicBlockId::new(0));
        function.metadata.is_entry_point = is_entry_point;
        function
    }

    #[test]
    fn collect_sorted_user_box_decl_values_sorts_by_box_name() {
        let mut module = crate::mir::MirModule::new("test".to_string());
        module
            .metadata
            .user_box_decls
            .insert("Stage1ProgramResultValidationBox".to_string(), Vec::new());
        module
            .metadata
            .user_box_decls
            .insert("Main".to_string(), Vec::new());
        module
            .metadata
            .user_box_decls
            .insert("Stage1InputContractBox".to_string(), Vec::new());

        let decls = collect_sorted_user_box_decl_values(&module);
        let names: Vec<_> = decls
            .iter()
            .map(|decl| {
                decl.get("name")
                    .and_then(serde_json::Value::as_str)
                    .unwrap_or("")
                    .to_string()
            })
            .collect();

        assert_eq!(
            names,
            vec![
                "Main".to_string(),
                "Stage1InputContractBox".to_string(),
                "Stage1ProgramResultValidationBox".to_string(),
            ]
        );
    }

    #[test]
    fn collect_sorted_user_box_decl_values_includes_typed_field_decls() {
        let mut module = crate::mir::MirModule::new("test".to_string());
        module
            .metadata
            .user_box_decls
            .insert("Point".to_string(), vec!["x".to_string(), "y".to_string()]);
        module.metadata.user_box_field_decls.insert(
            "Point".to_string(),
            vec![
                crate::mir::UserBoxFieldDecl {
                    name: "x".to_string(),
                    declared_type_name: Some("IntegerBox".to_string()),
                    is_weak: false,
                },
                crate::mir::UserBoxFieldDecl {
                    name: "y".to_string(),
                    declared_type_name: Some("IntegerBox".to_string()),
                    is_weak: true,
                },
            ],
        );

        let decls = collect_sorted_user_box_decl_values(&module);
        let point = decls
            .iter()
            .find(|decl| decl.get("name").and_then(serde_json::Value::as_str) == Some("Point"))
            .expect("Point decl");
        let field_decls = point
            .get("field_decls")
            .and_then(serde_json::Value::as_array)
            .expect("field_decls array");

        assert_eq!(field_decls.len(), 2);
        assert_eq!(
            field_decls[0]
                .get("name")
                .and_then(serde_json::Value::as_str),
            Some("x")
        );
        assert_eq!(
            field_decls[0]
                .get("declared_type")
                .and_then(serde_json::Value::as_str),
            Some("IntegerBox")
        );
        assert_eq!(
            field_decls[1]
                .get("is_weak")
                .and_then(serde_json::Value::as_bool),
            Some(true)
        );
    }

    #[test]
    fn collect_sorted_enum_decl_values_preserves_variant_inventory() {
        let mut module = crate::mir::MirModule::new("test".to_string());
        module.metadata.enum_decls.insert(
            "Option".to_string(),
            crate::mir::MirEnumDecl {
                type_parameters: vec!["T".to_string()],
                variants: vec![
                    crate::mir::MirEnumVariantDecl {
                        name: "None".to_string(),
                        payload_type_name: None,
                    },
                    crate::mir::MirEnumVariantDecl {
                        name: "Some".to_string(),
                        payload_type_name: Some("T".to_string()),
                    },
                ],
            },
        );

        let decls = collect_sorted_enum_decl_values(&module);
        assert_eq!(decls.len(), 1);
        assert_eq!(decls[0]["name"], "Option");
        assert_eq!(decls[0]["type_parameters"], json!(["T"]));
        assert_eq!(decls[0]["variants"][1]["name"], "Some");
        assert_eq!(decls[0]["variants"][1]["payload_type"], "T");
    }

    #[test]
    fn ordered_harness_functions_puts_entry_main_first() {
        let mut module = MirModule::new("test".to_string());
        module.functions.insert(
            "Main.equals/1".to_string(),
            make_function("Main.equals/1", false),
        );
        module.functions.insert(
            "condition_fn".to_string(),
            make_function("condition_fn", false),
        );
        module
            .functions
            .insert("main".to_string(), make_function("main", true));

        let ordered: Vec<_> = ordered_harness_functions(&module)
            .into_iter()
            .map(|(name, _)| name.as_str())
            .collect();

        assert_eq!(ordered[0], "main");
        assert_eq!(ordered[1], "Main.equals/1");
        assert_eq!(ordered[2], "condition_fn");
    }

    #[test]
    fn build_mir_json_root_emits_function_runes_as_attrs() {
        let mut module = MirModule::new("test".to_string());
        let mut function = make_function("main", true);
        function.metadata.runes = vec![
            RuneAttr {
                name: "Symbol".to_string(),
                args: vec!["main_sym".to_string()],
            },
            RuneAttr {
                name: "CallConv".to_string(),
                args: vec!["c".to_string()],
            },
        ];
        module.functions.insert("main".to_string(), function);

        let root = build_mir_json_root(&module).expect("mir json root");
        let runes = root["functions"][0]["attrs"]["runes"]
            .as_array()
            .expect("attrs.runes array");
        assert_eq!(runes.len(), 2);
        assert_eq!(runes[0]["name"], "Symbol");
        assert_eq!(runes[0]["args"], serde_json::json!(["main_sym"]));
        assert_eq!(runes[1]["name"], "CallConv");
        assert_eq!(runes[1]["args"], serde_json::json!(["c"]));
    }
}
