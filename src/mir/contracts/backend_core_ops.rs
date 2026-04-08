use crate::mir::{Callee, MirInstruction};

/// Canonical instruction tag used by contract/fail-fast diagnostics.
pub fn instruction_tag(inst: &MirInstruction) -> &'static str {
    match inst {
        MirInstruction::Const { .. } => "Const",
        MirInstruction::BinOp { .. } => "BinOp",
        MirInstruction::UnaryOp { .. } => "UnaryOp",
        MirInstruction::Compare { .. } => "Compare",
        MirInstruction::FieldGet { .. } => "FieldGet",
        MirInstruction::FieldSet { .. } => "FieldSet",
        MirInstruction::Load { .. } => "Load",
        MirInstruction::Store { .. } => "Store",
        MirInstruction::Call { .. } => "Call",
        MirInstruction::NewClosure { .. } => "NewClosure",
        MirInstruction::Branch { .. } => "Branch",
        MirInstruction::Jump { .. } => "Jump",
        MirInstruction::Return { .. } => "Return",
        MirInstruction::Phi { .. } => "Phi",
        MirInstruction::NewBox { .. } => "NewBox",
        MirInstruction::TypeOp { .. } => "TypeOp",
        MirInstruction::Copy { .. } => "Copy",
        MirInstruction::Debug { .. } => "Debug",
        MirInstruction::KeepAlive { .. } => "KeepAlive",
        MirInstruction::ReleaseStrong { .. } => "ReleaseStrong",
        MirInstruction::Throw { .. } => "Throw",
        MirInstruction::Catch { .. } => "Catch",
        MirInstruction::Safepoint => "Safepoint",
        MirInstruction::RefNew { .. } => "RefNew",
        MirInstruction::WeakRef { .. } => "WeakRef",
        MirInstruction::Barrier { .. } => "Barrier",
        MirInstruction::FutureNew { .. } => "FutureNew",
        MirInstruction::FutureSet { .. } => "FutureSet",
        MirInstruction::Await { .. } => "Await",
        MirInstruction::Select { .. } => "Select",
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstructionDietCohort {
    Kept,
    LoweredAway,
    Removed,
}

pub const MIR_INSTRUCTION_KEPT_TAGS: &[&str] = &[
    "Await",
    "Barrier",
    "BinOp",
    "Branch",
    "Call",
    "Catch",
    "Compare",
    "Const",
    "Copy",
    "Debug",
    "FutureNew",
    "FutureSet",
    "FieldGet",
    "FieldSet",
    "Jump",
    "KeepAlive",
    "Load",
    "NewBox",
    "NewClosure",
    "Phi",
    "RefNew",
    "ReleaseStrong",
    "Return",
    "Safepoint",
    "Select",
    "Store",
    "Throw",
    "TypeOp",
    "UnaryOp",
    "WeakRef",
];

pub const MIR_INSTRUCTION_LOWERED_AWAY_TAGS: &[&str] = &[];

pub const MIR_INSTRUCTION_REMOVED_TAGS: &[&str] = &[
    "ArrayGet",
    "ArraySet",
    "BarrierRead",
    "BarrierWrite",
    "BoxCall",
    "Cast",
    "DebugLog",
    "ExternCall",
    "Nop",
    "Print",
    "PluginInvoke",
    "RefGet",
    "RefSet",
    "TypeCheck",
    "WeakLoad",
    "WeakNew",
];

pub const MIR_INSTRUCTION_VOCABULARY_COUNT: usize = MIR_INSTRUCTION_KEPT_TAGS.len()
    + MIR_INSTRUCTION_LOWERED_AWAY_TAGS.len()
    + MIR_INSTRUCTION_REMOVED_TAGS.len();

pub fn instruction_diet_cohort(inst: &MirInstruction) -> InstructionDietCohort {
    match inst {
        MirInstruction::Await { .. }
        | MirInstruction::Barrier { .. }
        | MirInstruction::BinOp { .. }
        | MirInstruction::Branch { .. }
        | MirInstruction::Call { .. }
        | MirInstruction::Catch { .. }
        | MirInstruction::Compare { .. }
        | MirInstruction::Const { .. }
        | MirInstruction::Copy { .. }
        | MirInstruction::Debug { .. }
        | MirInstruction::FutureNew { .. }
        | MirInstruction::FutureSet { .. }
        | MirInstruction::FieldGet { .. }
        | MirInstruction::FieldSet { .. }
        | MirInstruction::Jump { .. }
        | MirInstruction::KeepAlive { .. }
        | MirInstruction::Load { .. }
        | MirInstruction::NewBox { .. }
        | MirInstruction::NewClosure { .. }
        | MirInstruction::Phi { .. }
        | MirInstruction::RefNew { .. }
        | MirInstruction::ReleaseStrong { .. }
        | MirInstruction::Return { .. }
        | MirInstruction::Safepoint
        | MirInstruction::Select { .. }
        | MirInstruction::Store { .. }
        | MirInstruction::Throw { .. }
        | MirInstruction::TypeOp { .. }
        | MirInstruction::UnaryOp { .. }
        | MirInstruction::WeakRef { .. } => InstructionDietCohort::Kept,
    }
}

pub fn lowered_away_tag(inst: &MirInstruction) -> Option<&'static str> {
    if instruction_diet_cohort(inst) == InstructionDietCohort::LoweredAway {
        Some(instruction_tag(inst))
    } else {
        None
    }
}

/// Reject codes for legacy call-site shapes that must not cross backend boundaries.
///
/// RCL-3 contract:
/// - `Call { callee: None }` is rejected.
/// NCL-0 contract:
/// - `Call { callee: Some(Callee::Closure{..}), dst=Some(_), args=[] }` is rejected as
///   `call-closure-not-canonical` (must be canonicalized to `NewClosure` first).
/// NCL-2 contract:
/// - other closure-call shapes are rejected with shape-specific reason codes.
pub fn legacy_callsite_reject_code(inst: &MirInstruction) -> Option<&'static str> {
    match inst {
        MirInstruction::Call { callee: None, .. } => Some("call-missing-callee"),
        MirInstruction::Call {
            dst,
            callee: Some(Callee::Closure { .. }),
            args,
            ..
        } => Some(crate::mir::ssot::closure_call::closure_call_reject_code(
            crate::mir::ssot::closure_call::classify_closure_call_shape(*dst, args),
        )),
        _ => None,
    }
}

/// Allowlist for MIR -> JSON instruction emission (non-terminator payload).
pub fn is_supported_mir_json_instruction(inst: &MirInstruction) -> bool {
    if legacy_callsite_reject_code(inst).is_some() {
        return false;
    }
    matches!(
        inst,
        MirInstruction::Copy { .. }
            | MirInstruction::UnaryOp { .. }
            | MirInstruction::Const { .. }
            | MirInstruction::TypeOp { .. }
            | MirInstruction::BinOp { .. }
            | MirInstruction::Compare { .. }
            | MirInstruction::Select { .. }
            | MirInstruction::FieldGet { .. }
            | MirInstruction::FieldSet { .. }
            | MirInstruction::Call { .. }
            | MirInstruction::NewBox { .. }
            | MirInstruction::NewClosure { .. }
            | MirInstruction::Branch { .. }
            | MirInstruction::Jump { .. }
            | MirInstruction::Return { .. }
            | MirInstruction::WeakRef { .. }
            | MirInstruction::KeepAlive { .. }
            | MirInstruction::ReleaseStrong { .. }
            | MirInstruction::Phi { .. }
    )
}

/// Allowlist for MIR terminator emission in MIR JSON.
pub fn is_supported_mir_json_terminator(inst: &MirInstruction) -> bool {
    matches!(
        inst,
        MirInstruction::Return { .. } | MirInstruction::Jump { .. } | MirInstruction::Branch { .. }
    )
}

/// Allowlist for MIR interpreter dispatch.
pub fn is_supported_vm_instruction(inst: &MirInstruction) -> bool {
    if legacy_callsite_reject_code(inst).is_some() {
        return false;
    }
    matches!(
        inst,
        MirInstruction::Const { .. }
            | MirInstruction::NewBox { .. }
            | MirInstruction::BinOp { .. }
            | MirInstruction::UnaryOp { .. }
            | MirInstruction::Compare { .. }
            | MirInstruction::TypeOp { .. }
            | MirInstruction::Copy { .. }
            | MirInstruction::FieldGet { .. }
            | MirInstruction::FieldSet { .. }
            | MirInstruction::Load { .. }
            | MirInstruction::Store { .. }
            | MirInstruction::Call { .. }
            | MirInstruction::Debug { .. }
            | MirInstruction::Select { .. }
            | MirInstruction::WeakRef { .. }
            | MirInstruction::Barrier { .. }
            | MirInstruction::Safepoint
            | MirInstruction::FutureNew { .. }
            | MirInstruction::FutureSet { .. }
            | MirInstruction::Await { .. }
            | MirInstruction::KeepAlive { .. }
            | MirInstruction::ReleaseStrong { .. }
    )
}

/// Allowlist for MIR interpreter block terminators.
pub fn is_supported_vm_terminator(inst: &MirInstruction) -> bool {
    matches!(
        inst,
        MirInstruction::Return { .. } | MirInstruction::Jump { .. } | MirInstruction::Branch { .. }
    )
}

/// MIR instruction -> LLVM JSON opcode candidates.
pub fn llvm_json_ops_for_instruction(inst: &MirInstruction) -> &'static [&'static str] {
    match inst {
        MirInstruction::Const { .. } => &["const"],
        MirInstruction::BinOp { .. } => &["binop"],
        MirInstruction::UnaryOp { .. } => &["unop"],
        MirInstruction::Compare { .. } => &["compare"],
        MirInstruction::FieldGet { .. } => &["field_get"],
        MirInstruction::FieldSet { .. } => &["field_set"],
        MirInstruction::Call { .. } => &["mir_call", "call", "boxcall", "externcall"],
        MirInstruction::Branch { .. } => &["branch"],
        MirInstruction::Jump { .. } => &["jump"],
        MirInstruction::Return { .. } => &["ret"],
        MirInstruction::Phi { .. } => &["phi"],
        MirInstruction::NewBox { .. } => &["newbox"],
        MirInstruction::TypeOp { .. } => &["typeop"],
        MirInstruction::Copy { .. } => &["copy"],
        MirInstruction::KeepAlive { .. } => &["keepalive"],
        MirInstruction::ReleaseStrong { .. } => &["release_strong"],
        MirInstruction::Safepoint => &["safepoint"],
        MirInstruction::WeakRef { .. } => &["weak_new", "weak_load"],
        MirInstruction::Barrier { .. } => &["barrier"],
        MirInstruction::Select { .. } => &["select"],

        MirInstruction::Load { .. }
        | MirInstruction::Store { .. }
        | MirInstruction::NewClosure { .. }
        | MirInstruction::Debug { .. }
        | MirInstruction::Throw { .. }
        | MirInstruction::Catch { .. }
        | MirInstruction::RefNew { .. }
        | MirInstruction::FutureNew { .. }
        | MirInstruction::FutureSet { .. }
        | MirInstruction::Await { .. } => &[],
    }
}

/// Canonical LLVM JSON opcode allowlist (Python lowerer frontend contract).
pub const LLVM_SUPPORTED_JSON_OPS: &[&str] = &[
    "const",
    "binop",
    "jump",
    "copy",
    "branch",
    "ret",
    "phi",
    "compare",
    "field_get",
    "field_set",
    "unop",
    "mir_call",
    "call",
    "boxcall",
    "externcall",
    "newbox",
    "typeop",
    "safepoint",
    "barrier",
    "keepalive",
    "release_strong",
    "select",
    "weak_new",
    "weak_load",
    "while",
];

/// Canonical LLVM JSON opcode allowlist (Python lowerer frontend contract).
pub fn is_supported_llvm_json_op(op: &str) -> bool {
    LLVM_SUPPORTED_JSON_OPS.contains(&op)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::{ConstValue, EffectMask, ValueId};
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
    fn instruction_diet_ledger_counts_match_ssot() {
        assert_eq!(MIR_INSTRUCTION_KEPT_TAGS.len(), 28);
        assert_eq!(MIR_INSTRUCTION_LOWERED_AWAY_TAGS.len(), 0);
        assert_eq!(MIR_INSTRUCTION_REMOVED_TAGS.len(), 16);
        assert_eq!(MIR_INSTRUCTION_VOCABULARY_COUNT, 44);
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
}
