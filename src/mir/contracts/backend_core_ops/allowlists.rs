use crate::mir::{Callee, MirInstruction};

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
    if let MirInstruction::MemOp { kind, .. } = inst {
        return crate::mir::contracts::fastmem_ops::is_supported_memop_kind(
            crate::mir::contracts::fastmem_ops::FastMemBackend::MirJson,
            *kind,
        );
    }
    matches!(
        inst,
        MirInstruction::Copy { .. }
            | MirInstruction::ArrayElementWrite { .. }
            | MirInstruction::ArrayStateContractClaim { .. }
            | MirInstruction::LocalContractWrite { .. }
            | MirInstruction::RecordFieldContractCheck { .. }
            | MirInstruction::RecordValuePublish { .. }
            | MirInstruction::UnaryOp { .. }
            | MirInstruction::Const { .. }
            | MirInstruction::TypeOp { .. }
            | MirInstruction::BinOp { .. }
            | MirInstruction::Compare { .. }
            | MirInstruction::StaticDataLoad { .. }
            | MirInstruction::Debug { .. }
            | MirInstruction::Select { .. }
            | MirInstruction::FieldGet { .. }
            | MirInstruction::FieldSet { .. }
            | MirInstruction::WeakFieldWrite { .. }
            | MirInstruction::VariantMake { .. }
            | MirInstruction::VariantTag { .. }
            | MirInstruction::VariantProject { .. }
            | MirInstruction::Call { .. }
            | MirInstruction::NewBox { .. }
            | MirInstruction::NewClosure { .. }
            | MirInstruction::Branch { .. }
            | MirInstruction::Jump { .. }
            | MirInstruction::Return { .. }
            | MirInstruction::WeakRef { .. }
            | MirInstruction::KeepAlive { .. }
            | MirInstruction::ReleaseStrong { .. }
            | MirInstruction::Safepoint
            | MirInstruction::FutureNew { .. }
            | MirInstruction::FutureSet { .. }
            | MirInstruction::Await { .. }
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
            | MirInstruction::ArrayElementWrite { .. }
            | MirInstruction::ArrayStateContractClaim { .. }
            | MirInstruction::NewBox { .. }
            | MirInstruction::BinOp { .. }
            | MirInstruction::UnaryOp { .. }
            | MirInstruction::Compare { .. }
            | MirInstruction::StaticDataLoad { .. }
            | MirInstruction::TypeOp { .. }
            | MirInstruction::Copy { .. }
            | MirInstruction::LocalContractWrite { .. }
            | MirInstruction::RecordFieldContractCheck { .. }
            | MirInstruction::RecordValuePublish { .. }
            | MirInstruction::FieldGet { .. }
            | MirInstruction::FieldSet { .. }
            | MirInstruction::VariantMake { .. }
            | MirInstruction::VariantTag { .. }
            | MirInstruction::VariantProject { .. }
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
        MirInstruction::StaticDataLoad { .. } => &["static_data_load"],
        MirInstruction::ArrayElementWrite { .. } => &["array_element_write"],
        MirInstruction::ArrayStateContractClaim { .. } => &[],
        MirInstruction::FieldGet { .. } => &["field_get"],
        MirInstruction::FieldSet { .. } => &["field_set"],
        MirInstruction::WeakFieldWrite { .. } => &["weak_field_write"],
        MirInstruction::VariantMake { .. } => &["variant_make"],
        MirInstruction::VariantTag { .. } => &["variant_tag"],
        MirInstruction::VariantProject { .. } => &["variant_project"],
        MirInstruction::Call { .. } => &["mir_call", "call", "boxcall", "externcall"],
        MirInstruction::Branch { .. } => &["branch"],
        MirInstruction::Jump { .. } => &["jump"],
        MirInstruction::Return { .. } => &["ret"],
        MirInstruction::Phi { .. } => &["phi"],
        MirInstruction::NewBox { .. } => &["newbox"],
        MirInstruction::TypeOp { .. } => &["typeop"],
        MirInstruction::Copy { .. } => &["copy"],
        MirInstruction::LocalContractWrite { .. } => &[],
        MirInstruction::RecordFieldContractCheck { .. }
        | MirInstruction::RecordValuePublish { .. } => &[],
        MirInstruction::KeepAlive { .. } => &["keepalive"],
        MirInstruction::ReleaseStrong { .. } => &["release_strong"],
        MirInstruction::Safepoint => &["safepoint"],
        MirInstruction::WeakRef { .. } => &["weak_new", "weak_load"],
        MirInstruction::Barrier { .. } => &["barrier"],
        MirInstruction::Select { .. } => &["select"],
        MirInstruction::MemOp { kind, .. } => {
            if crate::mir::contracts::fastmem_ops::is_supported_memop_kind(
                crate::mir::contracts::fastmem_ops::FastMemBackend::LlvmJson,
                *kind,
            ) {
                &["memop"]
            } else {
                &[]
            }
        }

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
    "static_data_load",
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
    "memop",
];

/// MIR JSON operations retained for typed transport but deliberately rejected
/// by LLVM/backend lowering until their capability owner is implemented.
pub const MIR_JSON_TRANSPORT_ONLY_OPS: &[&str] = &[
    "array_state_contract_claim",
    "array_element_write",
    "local_contract_write",
    "record_field_contract_check",
    "record_value_publish",
];

/// Canonical LLVM JSON opcode allowlist (Python lowerer frontend contract).
pub fn is_supported_llvm_json_op(op: &str) -> bool {
    LLVM_SUPPORTED_JSON_OPS.contains(&op)
}
