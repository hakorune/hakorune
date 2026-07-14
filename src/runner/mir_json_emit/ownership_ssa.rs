use crate::mir::ownership_ssa::{MirOwnershipKindV1, OwnershipOperationKindV1};
use crate::mir::MirFunction;
use serde_json::{json, Map, Value};

pub(super) fn build_ownership_ssa_json(function: &MirFunction) -> Option<Value> {
    let witness = function.metadata.ownership_ssa_v1.as_ref()?;
    let value_kinds = witness
        .kinds()
        .map(|(value, kind)| (value.as_u32().to_string(), json!(kind_name(kind))))
        .collect::<Map<_, _>>();
    let operations = witness
        .operations()
        .iter()
        .map(|operation| match operation.kind() {
            OwnershipOperationKindV1::CopyOwned { dst, src } => json!({
                "block": operation.block().as_u32(),
                "instruction_index": operation.instruction_index(),
                "op": "copy_owned",
                "dst": dst.as_u32(),
                "src": src.as_u32(),
            }),
            OwnershipOperationKindV1::DestroyOwned { value } => json!({
                "block": operation.block().as_u32(),
                "instruction_index": operation.instruction_index(),
                "op": "destroy_owned",
                "value": value.as_u32(),
            }),
        })
        .collect::<Vec<_>>();
    Some(json!({
        "schema": "VerifiedOwnershipSsaV1",
        "producer": "rust_ownership_ssa_verifier_v1",
        "owner": witness.owner().as_u64(),
        "backend": "llvm_py",
        "provider": "nyash_kernel",
        "value_kinds": value_kinds,
        "operations": operations,
    }))
}

const fn kind_name(kind: MirOwnershipKindV1) -> &'static str {
    match kind {
        MirOwnershipKindV1::None => "none",
        MirOwnershipKindV1::Borrowed => "borrowed",
        MirOwnershipKindV1::Owned => "owned",
    }
}
