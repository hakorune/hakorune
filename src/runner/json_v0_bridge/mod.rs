mod ast;
mod lexer;
mod lowering;

use ast::{ProgramV0, StmtV0};
use lowering::lower_program;
use std::collections::BTreeMap;

pub fn parse_json_v0_to_module(json: &str) -> Result<crate::mir::MirModule, String> {
    parse_json_v0_to_module_with_imports(json, BTreeMap::new())
}

pub fn parse_json_v0_to_module_with_imports(
    json: &str,
    mut imports: BTreeMap<String, String>,
) -> Result<crate::mir::MirModule, String> {
    let prog: ProgramV0 =
        serde_json::from_str(json).map_err(|e| format!("invalid JSON v0: {}", e))?;
    if crate::config::env::cli_verbose() {
        let first = prog
            .body
            .get(1)
            .map(|s| match s {
                StmtV0::Try { .. } => "Try",
                StmtV0::FiniReg { .. } => "FiniReg",
                _ => "Other",
            })
            .unwrap_or("<none>");
        crate::runtime::get_global_ring0().log.debug(&format!(
            "[Bridge] JSON v0: body_len={} first_type={}",
            prog.body.len(),
            first
        ));
    }
    if prog.version != 0 || prog.kind != "Program" {
        return Err("unsupported IR: expected {version:0, kind:\"Program\"}".into());
    }
    // Phase 29bq: Merge imports from JSON with passed-in imports
    // JSON imports take precedence over passed-in imports
    for (alias, module_path) in &prog.imports {
        imports.insert(alias.clone(), module_path.clone());
    }

    let mut module = lower_program(prog, imports)?;
    // Keep Program(JSON v0) bridge aligned with runtime preflight callsite contract.
    let _ = crate::mir::passes::callsite_canonicalize::canonicalize_callsites(&mut module);
    crate::mir::refresh_module_thin_entry_candidates(&mut module);
    crate::mir::refresh_module_thin_entry_selections(&mut module);
    crate::mir::refresh_module_sum_placement_facts(&mut module);
    crate::mir::refresh_module_sum_placement_selections(&mut module);
    crate::mir::refresh_module_sum_placement_layouts(&mut module);
    Ok(module)
}

pub fn parse_source_v0_to_module(input: &str) -> Result<crate::mir::MirModule, String> {
    let json = lexer::parse_source_v0_to_json(input)?;
    if std::env::var("NYASH_DUMP_JSON_IR").ok().as_deref() == Some("1") {
        println!("{}", json);
    }
    parse_json_v0_to_module(&json)
}

pub fn maybe_dump_mir(module: &crate::mir::MirModule) {
    lowering::maybe_dump_mir(module)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::{MirInstruction, MirModule, MirType, ValueId};
    use serde_json::json;
    use std::sync::{Mutex, OnceLock};

    fn env_guard() -> &'static Mutex<()> {
        static GUARD: OnceLock<Mutex<()>> = OnceLock::new();
        GUARD.get_or_init(|| Mutex::new(()))
    }

    fn option_enum_decls() -> serde_json::Value {
        json!([
            {
                "name": "Option",
                "type_parameters": [],
                "variants": [
                    { "name": "None", "payload_type": null },
                    { "name": "Some", "payload_type": "Integer" }
                ]
            }
        ])
    }

    fn option_some_ctor(value: i64) -> serde_json::Value {
        json!({
            "type": "EnumCtor",
            "enum": "Option",
            "variant": "Some",
            "payload_type": "Integer",
            "args": [{ "type": "Int", "value": value }]
        })
    }

    fn main_instructions(module: &MirModule) -> Vec<&MirInstruction> {
        let func = module.get_function("main").expect("main exists");
        let mut block_ids: Vec<_> = func.blocks.keys().copied().collect();
        block_ids.sort();
        let mut out = Vec::new();
        for block_id in block_ids {
            let block = func
                .blocks
                .get(&block_id)
                .unwrap_or_else(|| panic!("block {:?} exists", block_id));
            out.extend(block.instructions.iter());
            if let Some(term) = block.terminator.as_ref() {
                out.push(term);
            }
        }
        out
    }

    #[test]
    fn parse_json_v0_to_module_lowers_enum_ctor_to_sum_make() {
        let json = json!({
            "version": 0,
            "kind": "Program",
            "enum_decls": option_enum_decls(),
            "body": [
                {
                    "type": "Return",
                    "expr": option_some_ctor(7)
                }
            ]
        })
        .to_string();

        let module = parse_json_v0_to_module(&json).expect("enum ctor lowers");
        let insts = main_instructions(&module);

        assert!(matches!(
            insts.iter().find(|inst| matches!(inst, MirInstruction::SumMake { .. })),
            Some(MirInstruction::SumMake {
                enum_name,
                variant,
                tag,
                payload: Some(payload),
                payload_type: Some(MirType::Integer),
                ..
            }) if enum_name == "Option"
                && variant == "Some"
                && *tag == 1
                && *payload == ValueId::new(1)
        ));
        assert!(module.metadata.enum_decls.contains_key("Option"));
        assert!(
            !insts.iter().any(|inst| matches!(
                inst,
                MirInstruction::NewBox { .. }
                    | MirInstruction::FieldGet { .. }
                    | MirInstruction::FieldSet { .. }
            )),
            "enum ctor must stay on sum lane, not box/field lane"
        );
    }

    #[test]
    fn parse_json_v0_to_module_lowers_enum_match_to_sum_tag_and_project() {
        let json = json!({
            "version": 0,
            "kind": "Program",
            "enum_decls": option_enum_decls(),
            "body": [
                {
                    "type": "Local",
                    "name": "x",
                    "expr": option_some_ctor(7)
                },
                {
                    "type": "Return",
                    "expr": {
                        "type": "EnumMatch",
                        "enum": "Option",
                        "scrutinee": { "type": "Var", "name": "x" },
                        "arms": [
                            {
                                "variant": "Some",
                                "bind": "value",
                                "payload_type": "Integer",
                                "expr": { "type": "Var", "name": "value" }
                            },
                            {
                                "variant": "None",
                                "expr": { "type": "Int", "value": 0 }
                            }
                        ]
                    }
                }
            ]
        })
        .to_string();

        let module = parse_json_v0_to_module(&json).expect("enum match lowers");
        let insts = main_instructions(&module);

        assert_eq!(
            insts
                .iter()
                .filter(|inst| matches!(inst, MirInstruction::SumTag { .. }))
                .count(),
            1,
            "enum match should read the tag exactly once"
        );
        assert!(matches!(
            insts.iter().find(|inst| matches!(inst, MirInstruction::SumProject { .. })),
            Some(MirInstruction::SumProject {
                enum_name,
                variant,
                tag,
                payload_type: Some(MirType::Integer),
                ..
            }) if enum_name == "Option" && variant == "Some" && *tag == 1
        ));
        assert!(
            insts
                .iter()
                .any(|inst| matches!(inst, MirInstruction::Phi { .. })),
            "enum match should merge arm values through PHI"
        );
        assert!(
            !insts.iter().any(|inst| matches!(
                inst,
                MirInstruction::NewBox { .. }
                    | MirInstruction::FieldGet { .. }
                    | MirInstruction::FieldSet { .. }
            )),
            "enum match lowering must not encode semantics via box/field ops"
        );
    }

    #[test]
    fn parse_json_v0_to_module_lowers_record_enum_payload_through_hidden_box() {
        let payload_box = "__NyEnumPayload_Token_Ident";
        let json = json!({
            "version": 0,
            "kind": "Program",
            "user_box_decls": [
                {
                    "name": payload_box,
                    "fields": ["name"],
                    "field_decls": [
                        { "name": "name", "declared_type": "String", "is_weak": false }
                    ]
                }
            ],
            "enum_decls": [
                {
                    "name": "Token",
                    "type_parameters": [],
                    "variants": [
                        { "name": "Ident", "payload_type": payload_box },
                        { "name": "Eof", "payload_type": null }
                    ]
                }
            ],
            "body": [
                {
                    "type": "Local",
                    "name": "tok",
                    "expr": {
                        "type": "EnumCtor",
                        "enum": "Token",
                        "variant": "Ident",
                        "payload_type": payload_box,
                        "args": [
                            {
                                "type": "New",
                                "class": payload_box,
                                "args": [{ "type": "Str", "value": "hello" }]
                            }
                        ]
                    }
                },
                {
                    "type": "Return",
                    "expr": {
                        "type": "EnumMatch",
                        "enum": "Token",
                        "scrutinee": { "type": "Var", "name": "tok" },
                        "arms": [
                            {
                                "variant": "Ident",
                                "bind": "payload",
                                "payload_type": payload_box,
                                "expr": {
                                    "type": "BlockExpr",
                                    "prelude": [
                                        {
                                            "type": "Local",
                                            "name": "name",
                                            "expr": {
                                                "type": "Field",
                                                "recv": { "type": "Var", "name": "payload" },
                                                "field": "name"
                                            }
                                        }
                                    ],
                                    "tail": {
                                        "type": "Expr",
                                        "expr": { "type": "Var", "name": "name" }
                                    }
                                }
                            },
                            {
                                "variant": "Eof",
                                "expr": { "type": "Str", "value": "eof" }
                            }
                        ]
                    }
                }
            ]
        })
        .to_string();

        let module = parse_json_v0_to_module(&json).expect("record enum lowers");
        let insts = main_instructions(&module);

        assert_eq!(
            module
                .metadata
                .user_box_decls
                .get(payload_box)
                .expect("payload box names"),
            &vec!["name".to_string()]
        );
        assert_eq!(
            module
                .metadata
                .user_box_field_decls
                .get(payload_box)
                .expect("payload box field decls")[0]
                .declared_type_name
                .as_deref(),
            Some("String")
        );
        assert!(
            insts.iter().any(|inst| matches!(
                inst,
                MirInstruction::NewBox { box_type, .. } if box_type == payload_box
            )),
            "record payload should materialize through one hidden payload box"
        );
        assert!(
            insts
                .iter()
                .any(|inst| matches!(inst, MirInstruction::SumMake { .. })),
            "record enum constructor must still stay on sum lane"
        );
        assert!(
            insts
                .iter()
                .any(|inst| matches!(inst, MirInstruction::SumProject { .. })),
            "record enum match must still project through sum lane"
        );
        assert!(
            insts.iter().any(|inst| matches!(
                inst,
                MirInstruction::FieldGet { field, .. } if field == "name"
            )),
            "record enum bindings should lower through field_get on the hidden payload box"
        );
    }

    #[test]
    fn parse_json_v0_to_module_rejects_ctor_with_multiple_payload_args() {
        let json = json!({
            "version": 0,
            "kind": "Program",
            "enum_decls": option_enum_decls(),
            "body": [
                {
                    "type": "Return",
                    "expr": {
                        "type": "EnumCtor",
                        "enum": "Option",
                        "variant": "Some",
                        "payload_type": "Integer",
                        "args": [
                            { "type": "Int", "value": 1 },
                            { "type": "Int", "value": 2 }
                        ]
                    }
                }
            ]
        })
        .to_string();

        let error =
            parse_json_v0_to_module(&json).expect_err("json v0 sum lane stays single-payload");
        assert!(error.contains("[freeze:contract][json_v0][enum_ctor]"));
        assert!(error.contains("expects 1 arg(s), got 2"));
    }

    #[test]
    fn parse_json_v0_to_module_attaches_thin_entry_candidates_for_sum_lane() {
        let json = json!({
            "version": 0,
            "kind": "Program",
            "enum_decls": option_enum_decls(),
            "body": [
                {
                    "type": "Local",
                    "name": "opt",
                    "expr": option_some_ctor(1)
                },
                {
                    "type": "Return",
                    "expr": {
                        "type": "EnumMatch",
                        "enum": "Option",
                        "scrutinee": { "type": "Var", "name": "opt" },
                        "arms": [
                            {
                                "variant": "Some",
                                "bind": "v",
                                "payload_type": "Integer",
                                "expr": { "type": "Var", "name": "v" }
                            },
                            {
                                "variant": "None",
                                "expr": { "type": "Int", "value": 0 }
                            }
                        ]
                    }
                }
            ]
        })
        .to_string();

        let module = parse_json_v0_to_module(&json).expect("sum thin-entry inventory");
        let func = module.get_function("main").expect("main exists");

        assert!(func.metadata.thin_entry_candidates.iter().any(|candidate| {
            candidate.surface == crate::mir::ThinEntrySurface::SumMake
                && candidate.subject == "Option::Some"
                && candidate.preferred_entry
                    == crate::mir::ThinEntryPreferredEntry::ThinInternalEntry
                && candidate.current_carrier == crate::mir::ThinEntryCurrentCarrier::CompatBox
        }));
        assert!(func.metadata.thin_entry_candidates.iter().any(|candidate| {
            candidate.surface == crate::mir::ThinEntrySurface::SumProject
                && candidate.subject == "Option::Some"
                && candidate.value_class == crate::mir::ThinEntryValueClass::InlineI64
        }));
        assert!(func.metadata.thin_entry_selections.iter().any(|selection| {
            selection.surface == crate::mir::ThinEntrySurface::SumMake
                && selection.subject == "Option::Some"
                && selection.manifest_row == "sum_make.aggregate_local"
                && selection.selected_entry
                    == crate::mir::ThinEntryPreferredEntry::ThinInternalEntry
                && selection.state == crate::mir::ThinEntrySelectionState::Candidate
        }));
        assert!(func.metadata.thin_entry_selections.iter().any(|selection| {
            selection.surface == crate::mir::ThinEntrySurface::SumProject
                && selection.subject == "Option::Some"
                && selection.manifest_row == "sum_project.payload_local"
                && selection.selected_entry
                    == crate::mir::ThinEntryPreferredEntry::ThinInternalEntry
        }));
        assert!(func.metadata.sum_placement_facts.iter().any(|fact| {
            fact.surface == crate::mir::ThinEntrySurface::SumMake
                && fact.subject == "Option::Some"
                && fact.state == crate::mir::SumPlacementState::LocalAggregateCandidate
                && fact.tag_reads >= 1
                && fact.project_reads >= 1
        }));
        assert!(func.metadata.sum_placement_facts.iter().any(|fact| {
            fact.surface == crate::mir::ThinEntrySurface::SumProject
                && fact.subject == "Option::Some"
                && fact.source_sum.is_some()
                && fact.state == crate::mir::SumPlacementState::LocalAggregateCandidate
        }));
        assert!(func
            .metadata
            .sum_placement_selections
            .iter()
            .any(|selection| {
                selection.surface == crate::mir::ThinEntrySurface::SumMake
                    && selection.subject == "Option::Some"
                    && selection.manifest_row == "sum_make.local_aggregate"
                    && selection.selected_path == crate::mir::SumPlacementPath::LocalAggregate
            }));
        assert!(func
            .metadata
            .sum_placement_selections
            .iter()
            .any(|selection| {
                selection.surface == crate::mir::ThinEntrySurface::SumProject
                    && selection.subject == "Option::Some"
                    && selection.manifest_row == "sum_project.local_aggregate"
                    && selection.selected_path == crate::mir::SumPlacementPath::LocalAggregate
                    && selection.source_sum.is_some()
            }));
        assert!(func.metadata.sum_placement_layouts.iter().any(|layout| {
            layout.surface == crate::mir::ThinEntrySurface::SumMake
                && layout.subject == "Option::Some"
                && layout.layout == crate::mir::SumLocalAggregateLayout::TagI64Payload
        }));
        assert!(func.metadata.sum_placement_layouts.iter().any(|layout| {
            layout.surface == crate::mir::ThinEntrySurface::SumProject
                && layout.subject == "Option::Some"
                && layout.layout == crate::mir::SumLocalAggregateLayout::TagI64Payload
                && layout.source_sum.is_some()
        }));
    }

    #[test]
    fn sum_ops_survive_mir_json_roundtrip() {
        let _lock = env_guard().lock().expect("env lock");
        std::env::set_var("NYASH_JSON_SCHEMA_V1", "0");

        let json = json!({
            "version": 0,
            "kind": "Program",
            "enum_decls": option_enum_decls(),
            "body": [
                {
                    "type": "Local",
                    "name": "x",
                    "expr": option_some_ctor(7)
                },
                {
                    "type": "Return",
                    "expr": {
                        "type": "EnumMatch",
                        "enum": "Option",
                        "scrutinee": { "type": "Var", "name": "x" },
                        "arms": [
                            {
                                "variant": "Some",
                                "bind": "value",
                                "payload_type": "Integer",
                                "expr": { "type": "Var", "name": "value" }
                            },
                            {
                                "variant": "None",
                                "expr": { "type": "Int", "value": 0 }
                            }
                        ]
                    }
                }
            ]
        })
        .to_string();

        let module = parse_json_v0_to_module(&json).expect("enum module");
        let emitted = crate::runner::mir_json_emit::emit_mir_json_string_for_harness_bin(&module)
            .expect("emit mir json");
        let reparsed =
            crate::runner::mir_json_v0::parse_mir_v0_to_module(&emitted).expect("reparse mir json");
        std::env::remove_var("NYASH_JSON_SCHEMA_V1");

        let insts = main_instructions(&reparsed);
        assert!(
            insts
                .iter()
                .any(|inst| matches!(inst, MirInstruction::SumMake { .. })),
            "sum_make must survive MIR JSON roundtrip"
        );
        assert!(
            insts
                .iter()
                .any(|inst| matches!(inst, MirInstruction::SumTag { .. })),
            "sum_tag must survive MIR JSON roundtrip"
        );
        assert!(
            insts
                .iter()
                .any(|inst| matches!(inst, MirInstruction::SumProject { .. })),
            "sum_project must survive MIR JSON roundtrip"
        );
    }
}
