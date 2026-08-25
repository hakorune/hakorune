//! SSOT helper for extern call construction.
//!
//! RCL-3-min1:
//! - Stop constructing legacy `MirInstruction::ExternCall` at source sites.
//! - Emit the canonical Call helper with a `Callee::Extern` target.
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

    MirInstruction::call(dst, Callee::Extern(extern_name), args, effects)
}
