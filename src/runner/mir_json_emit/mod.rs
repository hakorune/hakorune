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
            }).collect::<serde_json::Map<String, serde_json::Value>>(),
            "string_corridor_facts": f.metadata.string_corridor_facts.iter().map(|(k, fact)| {
                (k.as_u32().to_string(), json!({
                    "op": fact.op.to_string(),
                    "role": fact.role.to_string(),
                    "carrier": fact.carrier.to_string(),
                    "outcome": fact.outcome.map(|outcome| outcome.to_string()),
                    "objectize": fact.objectize.to_string(),
                    "publish": fact.publish.to_string(),
                    "materialize": fact.materialize.to_string(),
                }))
            }).collect::<serde_json::Map<String, serde_json::Value>>(),
            "string_corridor_relations": f.metadata.string_corridor_relations.iter().map(|(k, relations)| {
                (k.as_u32().to_string(), json!(relations.iter().map(|relation| {
                    json!({
                        "kind": relation.kind.to_string(),
                        "base_value": relation.base_value.as_u32(),
                        "witness_value": relation.witness_value.map(|value| value.as_u32()),
                        "window_contract": relation.window_contract.to_string(),
                        "reason": relation.reason,
                    })
                }).collect::<Vec<_>>()))
            }).collect::<serde_json::Map<String, serde_json::Value>>(),
            "string_corridor_candidates": f.metadata.string_corridor_candidates.iter().map(|(k, candidates)| {
                (k.as_u32().to_string(), json!(candidates.iter().map(|candidate| {
                    json!({
                        "kind": candidate.kind.to_string(),
                        "state": candidate.state.to_string(),
                        "reason": candidate.reason,
                        "plan": candidate.plan.map(|plan| json!({
                            "corridor_root": plan.corridor_root.as_u32(),
                            "source_root": plan.source_root.map(|value| value.as_u32()),
                            "start": plan.start.map(|value| value.as_u32()),
                            "end": plan.end.map(|value| value.as_u32()),
                            "known_length": plan.known_length,
                            "proof": match plan.proof {
                                crate::mir::string_corridor_placement::StringCorridorCandidateProof::BorrowedSlice {
                                    source,
                                    start,
                                    end,
                                } => json!({
                                    "kind": "borrowed_slice",
                                    "source": source.as_u32(),
                                    "start": start.as_u32(),
                                    "end": end.as_u32(),
                                }),
                                crate::mir::string_corridor_placement::StringCorridorCandidateProof::ConcatTriplet {
                                    left_value,
                                    left_source,
                                    left_start,
                                    left_end,
                                    middle,
                                    right_value,
                                    right_source,
                                    right_start,
                                    right_end,
                                    shared_source,
                                } => json!({
                                    "kind": "concat_triplet",
                                    "left_value": left_value.map(|value| value.as_u32()),
                                    "left_source": left_source.as_u32(),
                                    "left_start": left_start.as_u32(),
                                    "left_end": left_end.as_u32(),
                                    "middle": middle.as_u32(),
                                    "right_value": right_value.map(|value| value.as_u32()),
                                    "right_source": right_source.as_u32(),
                                    "right_start": right_start.as_u32(),
                                    "right_end": right_end.as_u32(),
                                    "shared_source": shared_source,
                                }),
                            },
                        })),
                    })
                }).collect::<Vec<_>>()))
            }).collect::<serde_json::Map<String, serde_json::Value>>(),
            "thin_entry_candidates": f.metadata.thin_entry_candidates.iter().map(|candidate| {
                json!({
                    "block": candidate.block.as_u32(),
                    "instruction_index": candidate.instruction_index,
                    "value": candidate.value.map(|value| value.as_u32()),
                    "surface": candidate.surface.to_string(),
                    "subject": candidate.subject,
                    "preferred_entry": candidate.preferred_entry.to_string(),
                    "current_carrier": candidate.current_carrier.to_string(),
                    "value_class": candidate.value_class.to_string(),
                    "reason": candidate.reason,
                })
            }).collect::<Vec<_>>(),
            "thin_entry_selections": f.metadata.thin_entry_selections.iter().map(|selection| {
                json!({
                    "block": selection.block.as_u32(),
                    "instruction_index": selection.instruction_index,
                    "value": selection.value.map(|value| value.as_u32()),
                    "surface": selection.surface.to_string(),
                    "subject": selection.subject,
                    "manifest_row": selection.manifest_row,
                    "selected_entry": selection.selected_entry.to_string(),
                    "state": selection.state.to_string(),
                    "current_carrier": selection.current_carrier.to_string(),
                    "value_class": selection.value_class.to_string(),
                    "reason": selection.reason,
                })
            }).collect::<Vec<_>>(),
            "sum_placement_facts": f.metadata.sum_placement_facts.iter().map(|fact| {
                json!({
                    "block": fact.block.as_u32(),
                    "instruction_index": fact.instruction_index,
                    "value": fact.value.map(|value| value.as_u32()),
                    "surface": fact.surface.to_string(),
                    "subject": fact.subject,
                    "source_sum": fact.source_sum.map(|value| value.as_u32()),
                    "value_class": fact.value_class.to_string(),
                    "state": fact.state.to_string(),
                    "tag_reads": fact.tag_reads,
                    "project_reads": fact.project_reads,
                    "barriers": fact.barriers.iter().map(|barrier| barrier.to_string()).collect::<Vec<_>>(),
                    "reason": fact.reason,
                })
            }).collect::<Vec<_>>(),
            "sum_placement_selections": f.metadata.sum_placement_selections.iter().map(|selection| {
                json!({
                    "block": selection.block.as_u32(),
                    "instruction_index": selection.instruction_index,
                    "value": selection.value.map(|value| value.as_u32()),
                    "surface": selection.surface.to_string(),
                    "subject": selection.subject,
                    "source_sum": selection.source_sum.map(|value| value.as_u32()),
                    "manifest_row": selection.manifest_row,
                    "selected_path": selection.selected_path.to_string(),
                    "reason": selection.reason,
                })
            }).collect::<Vec<_>>(),
            "sum_placement_layouts": f.metadata.sum_placement_layouts.iter().map(|layout| {
                json!({
                    "block": layout.block.as_u32(),
                    "instruction_index": layout.instruction_index,
                    "value": layout.value.map(|value| value.as_u32()),
                    "surface": layout.surface.to_string(),
                    "subject": layout.subject,
                    "source_sum": layout.source_sum.map(|value| value.as_u32()),
                    "layout": layout.layout.to_string(),
                    "reason": layout.reason,
                })
            }).collect::<Vec<_>>()
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

    #[test]
    fn build_mir_json_root_emits_thin_entry_candidates() {
        let mut module = MirModule::new("test".to_string());
        let mut function = make_function("main", true);
        function
            .metadata
            .thin_entry_candidates
            .push(crate::mir::ThinEntryCandidate {
                block: BasicBlockId::new(0),
                instruction_index: 2,
                value: Some(crate::mir::ValueId::new(7)),
                surface: crate::mir::ThinEntrySurface::VariantMake,
                subject: "Option::Some".to_string(),
                preferred_entry: crate::mir::ThinEntryPreferredEntry::ThinInternalEntry,
                current_carrier: crate::mir::ThinEntryCurrentCarrier::CompatBox,
                value_class: crate::mir::ThinEntryValueClass::AggLocal,
                reason: "variant.make stays aggregate-first".to_string(),
            });
        module.functions.insert("main".to_string(), function);

        let root = build_mir_json_root(&module).expect("mir json root");
        let candidates = root["functions"][0]["metadata"]["thin_entry_candidates"]
            .as_array()
            .expect("thin_entry_candidates array");

        assert_eq!(candidates.len(), 1);
        assert_eq!(candidates[0]["surface"], "variant_make");
        assert_eq!(candidates[0]["subject"], "Option::Some");
        assert_eq!(candidates[0]["preferred_entry"], "thin_internal_entry");
        assert_eq!(candidates[0]["current_carrier"], "compat_box");
        assert_eq!(candidates[0]["value_class"], "agg_local");
        assert_eq!(candidates[0]["value"], 7);
    }

    #[test]
    fn build_mir_json_root_emits_string_corridor_facts() {
        let mut module = MirModule::new("test".to_string());
        let mut function = make_function("main", true);
        function.metadata.string_corridor_facts.insert(
            crate::mir::ValueId::new(7),
            crate::mir::StringCorridorFact::str_slice(
                crate::mir::StringCorridorCarrier::MethodCall,
            ),
        );
        module.functions.insert("main".to_string(), function);

        let root = build_mir_json_root(&module).expect("mir json root");
        let facts = root["functions"][0]["metadata"]["string_corridor_facts"]
            .as_object()
            .expect("string_corridor_facts object");

        assert_eq!(facts["7"]["op"], "str.slice");
        assert_eq!(facts["7"]["role"], "borrow_producer");
        assert_eq!(facts["7"]["carrier"], "method_call");
        assert!(facts["7"]["outcome"].is_null());
        assert_eq!(facts["7"]["objectize"], "?");
        assert_eq!(facts["7"]["publish"], "?");
        assert_eq!(facts["7"]["materialize"], "?");
    }

    #[test]
    fn build_mir_json_root_emits_string_corridor_candidates() {
        let mut module = MirModule::new("test".to_string());
        let mut function = make_function("main", true);
        function.metadata.string_corridor_relations.insert(
            crate::mir::ValueId::new(7),
            vec![crate::mir::StringCorridorRelation {
                kind: crate::mir::StringCorridorRelationKind::PhiCarryBase,
                base_value: crate::mir::ValueId::new(6),
                window_contract:
                    crate::mir::StringCorridorWindowContract::PreservePlanWindow,
                witness_value: None,
                reason: "single-input phi continuity keeps the current string corridor lane and preserves the proof-bearing plan window",
            }],
        );
        function.metadata.string_corridor_candidates.insert(
            crate::mir::ValueId::new(8),
            vec![crate::mir::StringCorridorCandidate {
                kind: crate::mir::StringCorridorCandidateKind::DirectKernelEntry,
                state: crate::mir::StringCorridorCandidateState::Candidate,
                reason:
                    "borrowed slice corridor can target a direct kernel entry before publication",
                plan: Some(crate::mir::string_corridor_placement::StringCorridorCandidatePlan {
                    corridor_root: crate::mir::ValueId::new(7),
                    source_root: Some(crate::mir::ValueId::new(1)),
                    start: Some(crate::mir::ValueId::new(2)),
                    end: Some(crate::mir::ValueId::new(3)),
                    known_length: Some(2),
                    proof:
                        crate::mir::string_corridor_placement::StringCorridorCandidateProof::ConcatTriplet {
                            left_value: Some(crate::mir::ValueId::new(4)),
                            left_source: crate::mir::ValueId::new(1),
                            left_start: crate::mir::ValueId::new(4),
                            left_end: crate::mir::ValueId::new(5),
                            middle: crate::mir::ValueId::new(6),
                            right_value: Some(crate::mir::ValueId::new(8)),
                            right_source: crate::mir::ValueId::new(1),
                            right_start: crate::mir::ValueId::new(5),
                            right_end: crate::mir::ValueId::new(9),
                            shared_source: true,
                        },
                }),
            }],
        );
        module.functions.insert("main".to_string(), function);

        let root = build_mir_json_root(&module).expect("mir json root");
        let candidates = root["functions"][0]["metadata"]["string_corridor_candidates"]
            .as_object()
            .expect("string_corridor_candidates object");
        let value_candidates = candidates["8"]
            .as_array()
            .expect("string corridor candidate array");

        assert_eq!(value_candidates.len(), 1);
        assert_eq!(value_candidates[0]["kind"], "direct_kernel_entry");
        assert_eq!(value_candidates[0]["state"], "candidate");
        assert_eq!(
            value_candidates[0]["reason"],
            "borrowed slice corridor can target a direct kernel entry before publication"
        );
        assert_eq!(value_candidates[0]["plan"]["corridor_root"], 7);
        assert_eq!(value_candidates[0]["plan"]["source_root"], 1);
        assert_eq!(value_candidates[0]["plan"]["start"], 2);
        assert_eq!(value_candidates[0]["plan"]["end"], 3);
        assert_eq!(value_candidates[0]["plan"]["known_length"], 2);
        assert_eq!(
            value_candidates[0]["plan"]["proof"]["kind"],
            "concat_triplet"
        );
        assert_eq!(value_candidates[0]["plan"]["proof"]["left_value"], 4);
        assert_eq!(value_candidates[0]["plan"]["proof"]["middle"], 6);
        assert_eq!(value_candidates[0]["plan"]["proof"]["right_value"], 8);
        assert_eq!(value_candidates[0]["plan"]["proof"]["shared_source"], true);

        let relations = root["functions"][0]["metadata"]["string_corridor_relations"]
            .as_object()
            .expect("string_corridor_relations object");
        let value_relations = relations["7"]
            .as_array()
            .expect("string corridor relation array");
        assert_eq!(value_relations[0]["kind"], "phi_carry_base");
        assert_eq!(value_relations[0]["base_value"], 6);
        assert_eq!(value_relations[0]["witness_value"], serde_json::Value::Null);
        assert_eq!(
            value_relations[0]["window_contract"],
            "preserve_plan_window"
        );
    }

    #[test]
    fn build_mir_json_root_emits_thin_entry_selections() {
        let mut module = MirModule::new("test".to_string());
        let mut function = make_function("main", true);
        function
            .metadata
            .thin_entry_selections
            .push(crate::mir::ThinEntrySelection {
                block: BasicBlockId::new(0),
                instruction_index: 3,
                value: Some(crate::mir::ValueId::new(8)),
                surface: crate::mir::ThinEntrySurface::UserBoxFieldGet,
                subject: "Point.x".to_string(),
                manifest_row: "user_box_field_get.inline_scalar",
                selected_entry: crate::mir::ThinEntryPreferredEntry::ThinInternalEntry,
                state: crate::mir::ThinEntrySelectionState::AlreadySatisfied,
                current_carrier: crate::mir::ThinEntryCurrentCarrier::BackendTyped,
                value_class: crate::mir::ThinEntryValueClass::InlineI64,
                reason: "typed field read stays on thin internal scalar lane".to_string(),
            });
        module.functions.insert("main".to_string(), function);

        let root = build_mir_json_root(&module).expect("mir json root");
        let selections = root["functions"][0]["metadata"]["thin_entry_selections"]
            .as_array()
            .expect("thin_entry_selections array");

        assert_eq!(selections.len(), 1);
        assert_eq!(
            selections[0]["manifest_row"],
            "user_box_field_get.inline_scalar"
        );
        assert_eq!(selections[0]["selected_entry"], "thin_internal_entry");
        assert_eq!(selections[0]["state"], "already_satisfied");
        assert_eq!(selections[0]["value"], 8);
    }

    #[test]
    fn build_mir_json_root_emits_sum_placement_facts() {
        let mut module = MirModule::new("test".to_string());
        let mut function = make_function("main", true);
        function
            .metadata
            .sum_placement_facts
            .push(crate::mir::SumPlacementFact {
                block: BasicBlockId::new(0),
                instruction_index: 4,
                value: Some(crate::mir::ValueId::new(9)),
                surface: crate::mir::ThinEntrySurface::VariantMake,
                subject: "Option::Some".to_string(),
                source_sum: None,
                value_class: crate::mir::ThinEntryValueClass::AggLocal,
                state: crate::mir::SumPlacementState::LocalAggregateCandidate,
                tag_reads: 1,
                project_reads: 1,
                barriers: vec![crate::mir::SumObjectizationBarrier::Call],
                reason: "variant value stays local until call boundary".to_string(),
            });
        module.functions.insert("main".to_string(), function);

        let root = build_mir_json_root(&module).expect("mir json root");
        let facts = root["functions"][0]["metadata"]["sum_placement_facts"]
            .as_array()
            .expect("sum_placement_facts array");

        assert_eq!(facts.len(), 1);
        assert_eq!(facts[0]["surface"], "variant_make");
        assert_eq!(facts[0]["state"], "local_agg_candidate");
        assert_eq!(facts[0]["barriers"][0], "call");
        assert_eq!(facts[0]["value"], 9);
    }

    #[test]
    fn build_mir_json_root_emits_sum_placement_selections() {
        let mut module = MirModule::new("test".to_string());
        let mut function = make_function("main", true);
        function
            .metadata
            .sum_placement_selections
            .push(crate::mir::SumPlacementSelection {
                block: BasicBlockId::new(0),
                instruction_index: 5,
                value: Some(crate::mir::ValueId::new(10)),
                surface: crate::mir::ThinEntrySurface::VariantProject,
                subject: "Option::Some".to_string(),
                source_sum: Some(crate::mir::ValueId::new(9)),
                manifest_row: "variant_project.local_aggregate",
                selected_path: crate::mir::SumPlacementPath::LocalAggregate,
                reason: "selected local aggregate projection".to_string(),
            });
        module.functions.insert("main".to_string(), function);

        let root = build_mir_json_root(&module).expect("mir json root");
        let selections = root["functions"][0]["metadata"]["sum_placement_selections"]
            .as_array()
            .expect("sum_placement_selections array");

        assert_eq!(selections.len(), 1);
        assert_eq!(
            selections[0]["manifest_row"],
            "variant_project.local_aggregate"
        );
        assert_eq!(selections[0]["selected_path"], "local_aggregate");
        assert_eq!(selections[0]["source_sum"], 9);
        assert_eq!(selections[0]["value"], 10);
    }

    #[test]
    fn build_mir_json_root_emits_sum_placement_layouts() {
        let mut module = MirModule::new("test".to_string());
        let mut function = make_function("main", true);
        function
            .metadata
            .sum_placement_layouts
            .push(crate::mir::SumPlacementLayout {
                block: BasicBlockId::new(0),
                instruction_index: 6,
                value: Some(crate::mir::ValueId::new(11)),
                surface: crate::mir::ThinEntrySurface::VariantMake,
                subject: "Option::Some".to_string(),
                source_sum: None,
                layout: crate::mir::SumLocalAggregateLayout::TagI64Payload,
                reason: "selected local aggregate uses i64 payload lane".to_string(),
            });
        module.functions.insert("main".to_string(), function);

        let root = build_mir_json_root(&module).expect("mir json root");
        let layouts = root["functions"][0]["metadata"]["sum_placement_layouts"]
            .as_array()
            .expect("sum_placement_layouts array");

        assert_eq!(layouts.len(), 1);
        assert_eq!(layouts[0]["layout"], "tag_i64_payload");
        assert_eq!(layouts[0]["surface"], "variant_make");
        assert_eq!(layouts[0]["value"], 11);
    }
}
