//! SSOT helper for extern call construction.
//!
//! RCL-3-min1:
//! - Stop constructing legacy `MirInstruction::ExternCall` at source sites.
//! - Emit canonical `MirInstruction::Call { callee: Some(Callee::Extern) }`.
//! - Keep external name as `<iface>.<method>` for runtime dispatch parity.

use crate::mir::{Callee, EffectMask, MirInstruction, ValueId};

/// Build a canonical extern call instruction in a single place.
pub fn extern_call(
    dst: Option<ValueId>,
    iface_name: impl Into<String>,
    method_name: impl Into<String>,
    args: Vec<ValueId>,
    effects: EffectMask,
) -> MirInstruction {
    let iface_name = iface_name.into();
    let method_name = method_name.into();
    let extern_name = if iface_name.is_empty() {
        method_name
    } else {
        format!("{}.{}", iface_name, method_name)
    };

    MirInstruction::Call {
        dst,
        func: ValueId::INVALID,
        callee: Some(Callee::Extern(extern_name)),
        args,
        effects,
    }
}
