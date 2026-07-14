use super::*;
use crate::mir::instruction::{FastMemRegionId, MemOpKind};
use crate::mir::{Callee, ConstValue, EffectMask, MirInstruction, ValueId};
use std::collections::BTreeSet;
use std::fs;

const MIR_INSTRUCTION_DOC_SSOT_PATH: &str = "docs/reference/mir/INSTRUCTION_SET.md";

fn read_doc_ssot() -> String {
    fs::read_to_string(MIR_INSTRUCTION_DOC_SSOT_PATH).unwrap_or_else(|err| {
        panic!(
            "failed to read MIR instruction SSOT doc '{}': {}",
            MIR_INSTRUCTION_DOC_SSOT_PATH, err
        )
    })
}

const MIR_JSON_SCHEMA_PATH: &str = "docs/reference/mir/json_v0.schema.json";

fn read_json_schema() -> serde_json::Value {
    let text = fs::read_to_string(MIR_JSON_SCHEMA_PATH).unwrap_or_else(|err| {
        panic!(
            "failed to read MIR JSON schema '{}': {}",
            MIR_JSON_SCHEMA_PATH, err
        )
    });
    serde_json::from_str(&text).unwrap_or_else(|err| {
        panic!(
            "failed to parse MIR JSON schema '{}': {}",
            MIR_JSON_SCHEMA_PATH, err
        )
    })
}

fn schema_instruction_op_enum(schema: &serde_json::Value) -> BTreeSet<&str> {
    let op_enum = schema
        .pointer("/definitions/instruction/properties/op/enum")
        .unwrap_or_else(|| panic!("schema_op_enum_missing: {}", MIR_JSON_SCHEMA_PATH))
        .as_array()
        .unwrap_or_else(|| panic!("schema_op_enum_not_array: {}", MIR_JSON_SCHEMA_PATH));
    op_enum
        .iter()
        .map(|value| {
            value
                .as_str()
                .unwrap_or_else(|| panic!("schema_op_enum_non_string: {}", value))
        })
        .collect()
}

fn parse_doc_sync_count(doc: &str, key: &str) -> usize {
    let prefix = format!("{}=", key);
    let raw = doc
        .lines()
        .map(str::trim)
        .find_map(|line| line.strip_prefix(&prefix))
        .unwrap_or_else(|| panic!("missing '{}' in {}", key, MIR_INSTRUCTION_DOC_SSOT_PATH));
    raw.parse::<usize>().unwrap_or_else(|err| {
        panic!(
            "failed to parse '{}' value '{}' in {}: {}",
            key, raw, MIR_INSTRUCTION_DOC_SSOT_PATH, err
        )
    })
}

#[test]
fn mir_json_allowlist_rejects_throw() {
    let inst = MirInstruction::Throw {
        exception: ValueId::new(1),
        effects: crate::mir::EffectMask::PANIC,
    };
    assert!(!is_supported_mir_json_instruction(&inst));
    assert_eq!(instruction_tag(&inst), "Throw");
}

#[test]
fn mir_json_allowlist_rejects_legacy_callsite_shapes() {
    let missing_callee = MirInstruction::Call {
        dst: Some(ValueId::new(0)),
        func: ValueId::new(1),
        callee: None,
        args: vec![],
        effects: EffectMask::PURE,
    };
    assert_eq!(
        legacy_callsite_reject_code(&missing_callee),
        Some("call-missing-callee")
    );
    assert!(!is_supported_mir_json_instruction(&missing_callee));
}

#[test]
fn vm_allowlist_rejects_call_without_callee() {
    let inst = MirInstruction::Call {
        dst: Some(ValueId::new(0)),
        func: ValueId::new(1),
        callee: None,
        args: vec![],
        effects: EffectMask::PURE,
    };
    assert_eq!(
        legacy_callsite_reject_code(&inst),
        Some("call-missing-callee")
    );
    assert!(!is_supported_vm_instruction(&inst));
}

#[test]
fn callsite_rejects_call_with_closure_callee() {
    let inst = MirInstruction::Call {
        dst: Some(ValueId::new(0)),
        func: ValueId::INVALID,
        callee: Some(Callee::Closure {
            params: vec!["x".to_string()],
            captures: vec![("outer".to_string(), ValueId::new(7))],
            me_capture: None,
        }),
        args: vec![],
        effects: EffectMask::PURE,
    };
    assert_eq!(
        legacy_callsite_reject_code(&inst),
        Some("call-closure-not-canonical")
    );
    assert!(!is_supported_mir_json_instruction(&inst));
    assert!(!is_supported_vm_instruction(&inst));
}

#[test]
fn callsite_rejects_closure_call_without_dst() {
    let inst = MirInstruction::Call {
        dst: None,
        func: ValueId::INVALID,
        callee: Some(Callee::Closure {
            params: vec!["x".to_string()],
            captures: vec![],
            me_capture: None,
        }),
        args: vec![],
        effects: EffectMask::PURE,
    };
    assert_eq!(
        legacy_callsite_reject_code(&inst),
        Some("call-closure-missing-dst")
    );
    assert!(!is_supported_mir_json_instruction(&inst));
    assert!(!is_supported_vm_instruction(&inst));
}

#[test]
fn callsite_rejects_closure_call_with_runtime_args() {
    let inst = MirInstruction::Call {
        dst: Some(ValueId::new(0)),
        func: ValueId::INVALID,
        callee: Some(Callee::Closure {
            params: vec!["x".to_string()],
            captures: vec![],
            me_capture: None,
        }),
        args: vec![ValueId::new(9)],
        effects: EffectMask::PURE,
    };
    assert_eq!(
        legacy_callsite_reject_code(&inst),
        Some("call-closure-runtime-args")
    );
    assert!(!is_supported_mir_json_instruction(&inst));
    assert!(!is_supported_vm_instruction(&inst));
}

#[test]
fn vm_allowlist_accepts_typeop() {
    let inst = MirInstruction::TypeOp {
        dst: ValueId::new(0),
        op: crate::mir::TypeOpKind::Check,
        value: ValueId::new(1),
        ty: crate::mir::MirType::Integer,
    };
    assert!(is_supported_vm_instruction(&inst));
}

#[test]
fn vm_terminator_allowlist_rejects_throw() {
    let inst = MirInstruction::Throw {
        exception: ValueId::new(1),
        effects: crate::mir::EffectMask::PANIC,
    };
    assert!(!is_supported_vm_terminator(&inst));
}

#[test]
fn llvm_opcode_allowlist_rejects_unknown() {
    assert!(is_supported_llvm_json_op("mir_call"));
    assert!(!is_supported_llvm_json_op("debug"));
}

#[test]
fn mir_json_allowlist_accepts_const() {
    let inst = MirInstruction::Const {
        dst: ValueId::new(0),
        value: ConstValue::Integer(42),
    };
    assert!(is_supported_mir_json_instruction(&inst));
}

#[test]
fn mir_json_allowlist_accepts_new_closure() {
    let inst = MirInstruction::NewClosure {
        dst: ValueId::new(0),
        params: vec!["x".to_string()],
        body_id: None,
        body: vec![],
        captures: vec![("outer".to_string(), ValueId::new(7))],
        me: None,
    };
    assert!(is_supported_mir_json_instruction(&inst));
}

#[test]
fn mir_json_allowlist_accepts_select() {
    let inst = MirInstruction::Select {
        dst: ValueId::new(0),
        cond: ValueId::new(1),
        then_val: ValueId::new(2),
        else_val: ValueId::new(3),
    };
    assert!(is_supported_mir_json_instruction(&inst));
}

#[test]
fn mir_json_allowlist_accepts_sum_lane_ops() {
    let make = MirInstruction::VariantMake {
        dst: ValueId::new(0),
        enum_name: "Option".to_string(),
        variant: "Some".to_string(),
        tag: 1,
        payload: Some(ValueId::new(1)),
        payload_type: Some(crate::mir::MirType::Integer),
    };
    let tag = MirInstruction::VariantTag {
        dst: ValueId::new(2),
        value: ValueId::new(0),
        enum_name: "Option".to_string(),
    };
    let project = MirInstruction::VariantProject {
        dst: ValueId::new(3),
        value: ValueId::new(0),
        enum_name: "Option".to_string(),
        variant: "Some".to_string(),
        tag: 1,
        payload_type: Some(crate::mir::MirType::Integer),
    };
    assert!(is_supported_mir_json_instruction(&make));
    assert!(is_supported_mir_json_instruction(&tag));
    assert!(is_supported_mir_json_instruction(&project));
    assert!(is_supported_vm_instruction(&make));
    assert!(is_supported_vm_instruction(&tag));
    assert!(is_supported_vm_instruction(&project));
    assert_eq!(instruction_tag(&project), "VariantProject");
}

#[test]
fn memop_v0_dialect_is_json_and_llvm_supported() {
    for kind in MemOpKind::ALL {
        let inst = MirInstruction::MemOp {
            region: FastMemRegionId::new(1),
            kind: *kind,
            dst: Some(ValueId::new(10)),
            operands: vec![ValueId::new(1)],
            access: None,
            effects: EffectMask::READ,
        };
        assert_eq!(instruction_tag(&inst), "MemOp");
        assert_eq!(instruction_diet_cohort(&inst), InstructionDietCohort::Kept);
        assert!(is_supported_mir_json_instruction(&inst));
        assert!(!is_supported_vm_instruction(&inst));
        assert_eq!(llvm_json_ops_for_instruction(&inst), &["memop"]);
        assert!(is_supported_llvm_json_op("memop"));
    }
}

#[test]
fn instruction_diet_ledger_counts_match_ssot() {
    assert_eq!(MIR_INSTRUCTION_KEPT_TAGS.len(), 43);
    assert_eq!(MIR_INSTRUCTION_LOWERED_AWAY_TAGS.len(), 0);
    assert_eq!(MIR_INSTRUCTION_REMOVED_TAGS.len(), 16);
    assert_eq!(MIR_INSTRUCTION_VOCABULARY_COUNT, 59);
}

#[test]
fn ownership_ssa_vocabulary_has_rust_and_exact_llvm_py_handler_support() {
    let copy = MirInstruction::CopyOwned {
        dst: ValueId::new(2),
        src: ValueId::new(1),
    };
    let destroy = MirInstruction::DestroyOwned {
        value: ValueId::new(2),
    };

    for (instruction, tag, op) in [
        (copy, "CopyOwned", "copy_owned"),
        (destroy, "DestroyOwned", "destroy_owned"),
    ] {
        assert_eq!(instruction_tag(&instruction), tag);
        assert_eq!(
            instruction_diet_cohort(&instruction),
            InstructionDietCohort::Kept
        );
        assert!(is_supported_mir_json_instruction(&instruction));
        assert!(is_supported_vm_instruction(&instruction));
        assert_eq!(llvm_json_ops_for_instruction(&instruction), &[op]);
        assert!(!MIR_JSON_TRANSPORT_ONLY_OPS.contains(&op));
        assert!(is_supported_llvm_json_op(op));
    }
}

#[test]
fn instruction_diet_ledger_counts_match_docs_ssot() {
    let doc = read_doc_ssot();

    let kept = parse_doc_sync_count(&doc, "DOC_SYNC_MIR_KEPT_COUNT");
    let lowered = parse_doc_sync_count(&doc, "DOC_SYNC_MIR_LOWERED_AWAY_COUNT");
    let removed = parse_doc_sync_count(&doc, "DOC_SYNC_MIR_REMOVED_COUNT");
    let vocab = parse_doc_sync_count(&doc, "DOC_SYNC_MIR_VOCABULARY_COUNT");
    let core26 = parse_doc_sync_count(&doc, "DOC_SYNC_CORE26_COUNT");
    let mir14 = parse_doc_sync_count(&doc, "DOC_SYNC_MIR14_COUNT");

    assert_eq!(MIR_INSTRUCTION_KEPT_TAGS.len(), kept);
    assert_eq!(MIR_INSTRUCTION_LOWERED_AWAY_TAGS.len(), lowered);
    assert_eq!(MIR_INSTRUCTION_REMOVED_TAGS.len(), removed);
    assert_eq!(MIR_INSTRUCTION_VOCABULARY_COUNT, vocab);
    assert_eq!(
        crate::mir::instruction_introspection::mir14_instruction_names().len(),
        mir14
    );
    assert_eq!(core26, 26, "Core-26 profile contract changed");
}

#[test]
fn mir_json_schema_op_enum_matches_lowerable_and_transport_only_ops() {
    let schema = read_json_schema();
    let schema_ops = schema_instruction_op_enum(&schema);
    let expected_ops: BTreeSet<_> = LLVM_SUPPORTED_JSON_OPS
        .iter()
        .chain(MIR_JSON_TRANSPORT_ONLY_OPS.iter())
        .copied()
        .collect();
    assert_eq!(
        schema_ops, expected_ops,
        "MIR JSON schema op enum must cover lowerable and explicit transport-only ops"
    );
    assert!(!is_supported_llvm_json_op("local_contract_write"));
}

#[test]
fn instruction_diet_ledger_cohorts_are_disjoint() {
    let kept: BTreeSet<_> = MIR_INSTRUCTION_KEPT_TAGS.iter().copied().collect();
    let lowered: BTreeSet<_> = MIR_INSTRUCTION_LOWERED_AWAY_TAGS.iter().copied().collect();
    let removed: BTreeSet<_> = MIR_INSTRUCTION_REMOVED_TAGS.iter().copied().collect();
    assert_eq!(kept.intersection(&lowered).count(), 0);
    assert_eq!(kept.intersection(&removed).count(), 0);
    assert_eq!(lowered.intersection(&removed).count(), 0);
}

#[test]
fn removed_tags_include_legacy_and_retired_ops() {
    assert!(MIR_INSTRUCTION_REMOVED_TAGS.contains(&"ArrayGet"));
    assert!(MIR_INSTRUCTION_REMOVED_TAGS.contains(&"ArraySet"));
    assert!(MIR_INSTRUCTION_REMOVED_TAGS.contains(&"BarrierRead"));
    assert!(MIR_INSTRUCTION_REMOVED_TAGS.contains(&"BarrierWrite"));
    assert!(MIR_INSTRUCTION_REMOVED_TAGS.contains(&"BoxCall"));
    assert!(MIR_INSTRUCTION_REMOVED_TAGS.contains(&"DebugLog"));
    assert!(MIR_INSTRUCTION_REMOVED_TAGS.contains(&"ExternCall"));
    assert!(MIR_INSTRUCTION_REMOVED_TAGS.contains(&"Nop"));
    assert!(MIR_INSTRUCTION_REMOVED_TAGS.contains(&"RefGet"));
    assert!(MIR_INSTRUCTION_REMOVED_TAGS.contains(&"RefSet"));
    assert!(MIR_INSTRUCTION_REMOVED_TAGS.contains(&"Print"));
    assert!(MIR_INSTRUCTION_REMOVED_TAGS.contains(&"PluginInvoke"));
    assert!(MIR_INSTRUCTION_REMOVED_TAGS.contains(&"WeakLoad"));
    assert!(MIR_INSTRUCTION_REMOVED_TAGS.contains(&"WeakNew"));
}

#[test]
fn lowered_away_tag_ignores_kept_const() {
    let inst = MirInstruction::Const {
        dst: ValueId::new(0),
        value: ConstValue::Integer(7),
    };
    assert_eq!(lowered_away_tag(&inst), None);
    assert_eq!(instruction_diet_cohort(&inst), InstructionDietCohort::Kept);
}
