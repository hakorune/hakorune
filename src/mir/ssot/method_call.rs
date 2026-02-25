//! SSOT helper for canonical method-call construction.
//!
//! RCL-3-min2:
//! - Stop constructing legacy `MirInstruction::BoxCall` at emit sites.
//! - Emit canonical `MirInstruction::Call { callee: Some(Callee::Method { .. }) }`.

use crate::mir::definitions::call_unified::{CalleeBoxKind, TypeCertainty};
use crate::mir::{Callee, EffectMask, MirInstruction, ValueId};

/// Build a canonical method call instruction in a single place.
pub fn method_call(
    dst: Option<ValueId>,
    receiver: ValueId,
    box_name: impl Into<String>,
    method: impl Into<String>,
    args: Vec<ValueId>,
    effects: EffectMask,
    certainty: TypeCertainty,
    box_kind: CalleeBoxKind,
) -> MirInstruction {
    MirInstruction::Call {
        dst,
        func: ValueId::INVALID,
        callee: Some(Callee::Method {
            box_name: box_name.into(),
            method: method.into(),
            receiver: Some(receiver),
            certainty,
            box_kind,
        }),
        args,
        effects,
    }
}

/// Runtime-dispatch method call helper with conservative box_kind.
pub fn runtime_method_call(
    dst: Option<ValueId>,
    receiver: ValueId,
    box_name: impl Into<String>,
    method: impl Into<String>,
    args: Vec<ValueId>,
    effects: EffectMask,
    certainty: TypeCertainty,
) -> MirInstruction {
    method_call(
        dst,
        receiver,
        box_name,
        method,
        args,
        effects,
        certainty,
        CalleeBoxKind::RuntimeData,
    )
}
