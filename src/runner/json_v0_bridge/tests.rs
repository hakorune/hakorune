use super::*;
use crate::mir::sum_placement::SumPlacementState;
use crate::mir::sum_placement_layout::SumLocalAggregateLayout;
use crate::mir::sum_placement_selection::SumPlacementPath;
use crate::mir::thin_entry::{
    ThinEntryCurrentCarrier, ThinEntryPreferredEntry, ThinEntrySurface, ThinEntryValueClass,
};
use crate::mir::thin_entry_selection::ThinEntrySelectionState;
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
fn parse_json_v0_to_module_lowers_enum_ctor_to_variant_make() {
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
        insts.iter().find(|inst| matches!(inst, MirInstruction::VariantMake { .. })),
        Some(MirInstruction::VariantMake {
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
fn parse_json_v0_to_module_lowers_enum_match_to_variant_tag_and_project() {
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
            .filter(|inst| matches!(inst, MirInstruction::VariantTag { .. }))
            .count(),
        1,
        "enum match should read the tag exactly once"
    );
    assert!(matches!(
        insts.iter().find(|inst| matches!(inst, MirInstruction::VariantProject { .. })),
        Some(MirInstruction::VariantProject {
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
    let payload_box = "__NyVariantPayload_Token_Ident";
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
            .any(|inst| matches!(inst, MirInstruction::VariantMake { .. })),
        "record enum constructor must still stay on sum lane"
    );
    assert!(
        insts
            .iter()
            .any(|inst| matches!(inst, MirInstruction::VariantProject { .. })),
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
fn parse_json_v0_to_module_lowers_tuple_enum_payload_through_hidden_box() {
    let payload_box = "__NyVariantPayload_Pair_Both";
    let json = json!({
        "version": 0,
        "kind": "Program",
        "user_box_decls": [
            {
                "name": payload_box,
                "fields": ["_0", "_1"],
                "field_decls": [
                    { "name": "_0", "declared_type": "Integer", "is_weak": false },
                    { "name": "_1", "declared_type": "Integer", "is_weak": false }
                ]
            }
        ],
        "enum_decls": [
            {
                "name": "Pair",
                "type_parameters": [],
                "variants": [
                    { "name": "Both", "payload_type": payload_box },
                    { "name": "None", "payload_type": null }
                ]
            }
        ],
        "body": [
            {
                "type": "Local",
                "name": "pair",
                "expr": {
                    "type": "EnumCtor",
                    "enum": "Pair",
                    "variant": "Both",
                    "payload_type": payload_box,
                    "args": [
                        {
                            "type": "New",
                            "class": payload_box,
                            "args": [
                                { "type": "Int", "value": 1 },
                                { "type": "Int", "value": 2 }
                            ]
                        }
                    ]
                }
            },
            {
                "type": "Return",
                "expr": {
                    "type": "EnumMatch",
                    "enum": "Pair",
                    "scrutinee": { "type": "Var", "name": "pair" },
                    "arms": [
                        {
                            "variant": "Both",
                            "bind": "payload",
                            "payload_type": payload_box,
                            "expr": {
                                "type": "BlockExpr",
                                "prelude": [
                                    {
                                        "type": "Local",
                                        "name": "left",
                                        "expr": {
                                            "type": "Field",
                                            "recv": { "type": "Var", "name": "payload" },
                                            "field": "_0"
                                        }
                                    },
                                    {
                                        "type": "Local",
                                        "name": "right",
                                        "expr": {
                                            "type": "Field",
                                            "recv": { "type": "Var", "name": "payload" },
                                            "field": "_1"
                                        }
                                    }
                                ],
                                "tail": {
                                    "type": "Expr",
                                    "expr": {
                                        "type": "Binary",
                                        "op": "+",
                                        "lhs": { "type": "Var", "name": "left" },
                                        "rhs": { "type": "Var", "name": "right" }
                                    }
                                }
                            }
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

    let module = parse_json_v0_to_module(&json).expect("tuple enum lowers");
    let insts = main_instructions(&module);

    assert_eq!(
        module
            .metadata
            .user_box_decls
            .get(payload_box)
            .expect("payload box names"),
        &vec!["_0".to_string(), "_1".to_string()]
    );
    assert!(
        insts.iter().any(|inst| matches!(
            inst,
            MirInstruction::NewBox { box_type, .. } if box_type == payload_box
        )),
        "tuple payload should materialize through one hidden payload box"
    );
    assert!(
        insts
            .iter()
            .any(|inst| matches!(inst, MirInstruction::VariantMake { .. })),
        "tuple enum constructor must still stay on sum lane"
    );
    assert!(
        insts
            .iter()
            .any(|inst| matches!(inst, MirInstruction::VariantProject { .. })),
        "tuple enum match must still project through sum lane"
    );
    assert!(
        insts.iter().any(|inst| matches!(
            inst,
            MirInstruction::FieldGet { field, .. } if field == "_0"
        )),
        "tuple enum bindings should lower through field_get on the hidden payload box"
    );
    assert!(
        insts.iter().any(|inst| matches!(
            inst,
            MirInstruction::FieldGet { field, .. } if field == "_1"
        )),
        "tuple enum bindings should lower through field_get on the hidden payload box"
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

    let error = parse_json_v0_to_module(&json).expect_err("json v0 sum lane stays single-payload");
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
        candidate.surface == ThinEntrySurface::VariantMake
            && candidate.subject == "Option::Some"
            && candidate.preferred_entry == ThinEntryPreferredEntry::ThinInternalEntry
            && candidate.current_carrier == ThinEntryCurrentCarrier::CompatBox
    }));
    assert!(func.metadata.thin_entry_candidates.iter().any(|candidate| {
        candidate.surface == ThinEntrySurface::VariantTag
            && candidate.subject == "Option"
            && candidate.value_class == ThinEntryValueClass::InlineI64
    }));
    assert!(func.metadata.thin_entry_candidates.iter().any(|candidate| {
        candidate.surface == ThinEntrySurface::VariantProject
            && candidate.subject == "Option::Some"
            && candidate.value_class == ThinEntryValueClass::InlineI64
    }));
    assert!(func.metadata.thin_entry_selections.iter().any(|selection| {
        selection.surface == ThinEntrySurface::VariantMake
            && selection.subject == "Option::Some"
            && selection.manifest_row == "variant_make.aggregate_local"
            && selection.selected_entry == ThinEntryPreferredEntry::ThinInternalEntry
            && selection.state == ThinEntrySelectionState::Candidate
    }));
    assert!(func.metadata.thin_entry_selections.iter().any(|selection| {
        selection.surface == ThinEntrySurface::VariantTag
            && selection.subject == "Option"
            && selection.manifest_row == "variant_tag.tag_local"
            && selection.selected_entry == ThinEntryPreferredEntry::ThinInternalEntry
    }));
    assert!(func.metadata.thin_entry_selections.iter().any(|selection| {
        selection.surface == ThinEntrySurface::VariantProject
            && selection.subject == "Option::Some"
            && selection.manifest_row == "variant_project.payload_local"
            && selection.selected_entry == ThinEntryPreferredEntry::ThinInternalEntry
    }));
    assert!(func.metadata.sum_placement_facts.iter().any(|fact| {
        fact.surface == ThinEntrySurface::VariantMake
            && fact.subject == "Option::Some"
            && fact.state == SumPlacementState::LocalAggregateCandidate
            && fact.tag_reads >= 1
            && fact.project_reads >= 1
    }));
    assert!(func.metadata.sum_placement_facts.iter().any(|fact| {
        fact.surface == ThinEntrySurface::VariantTag
            && fact.subject == "Option"
            && fact.source_sum.is_some()
            && fact.state == SumPlacementState::LocalAggregateCandidate
    }));
    assert!(func.metadata.sum_placement_facts.iter().any(|fact| {
        fact.surface == ThinEntrySurface::VariantProject
            && fact.subject == "Option::Some"
            && fact.source_sum.is_some()
            && fact.state == SumPlacementState::LocalAggregateCandidate
    }));
    assert!(func
        .metadata
        .sum_placement_selections
        .iter()
        .any(|selection| {
            selection.surface == ThinEntrySurface::VariantMake
                && selection.subject == "Option::Some"
                && selection.manifest_row == "variant_make.local_aggregate"
                && selection.selected_path == SumPlacementPath::LocalAggregate
        }));
    assert!(func
        .metadata
        .sum_placement_selections
        .iter()
        .any(|selection| {
            selection.surface == ThinEntrySurface::VariantTag
                && selection.subject == "Option"
                && selection.manifest_row == "variant_tag.local_aggregate"
                && selection.selected_path == SumPlacementPath::LocalAggregate
                && selection.source_sum.is_some()
        }));
    assert!(func
        .metadata
        .sum_placement_selections
        .iter()
        .any(|selection| {
            selection.surface == ThinEntrySurface::VariantProject
                && selection.subject == "Option::Some"
                && selection.manifest_row == "variant_project.local_aggregate"
                && selection.selected_path == SumPlacementPath::LocalAggregate
                && selection.source_sum.is_some()
        }));
    assert!(func.metadata.sum_placement_layouts.iter().any(|layout| {
        layout.surface == ThinEntrySurface::VariantMake
            && layout.subject == "Option::Some"
            && layout.layout == SumLocalAggregateLayout::TagI64Payload
    }));
    assert!(func.metadata.sum_placement_layouts.iter().any(|layout| {
        layout.surface == ThinEntrySurface::VariantProject
            && layout.subject == "Option::Some"
            && layout.layout == SumLocalAggregateLayout::TagI64Payload
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
            .any(|inst| matches!(inst, MirInstruction::VariantMake { .. })),
        "variant_make must survive MIR JSON roundtrip"
    );
    assert!(
        insts
            .iter()
            .any(|inst| matches!(inst, MirInstruction::VariantTag { .. })),
        "variant_tag must survive MIR JSON roundtrip"
    );
    assert!(
        insts
            .iter()
            .any(|inst| matches!(inst, MirInstruction::VariantProject { .. })),
        "variant_project must survive MIR JSON roundtrip"
    );
}
