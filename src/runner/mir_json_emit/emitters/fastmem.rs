use crate::mir::instruction::{FastMemRegionId, MemOpKind};
use crate::mir::{EffectMask, ValueId};
use serde_json::json;

pub(crate) fn emit_memop(
    region: &FastMemRegionId,
    kind: &MemOpKind,
    dst: &Option<ValueId>,
    operands: &[ValueId],
    effects: &EffectMask,
) -> serde_json::Value {
    json!({
        "op": "memop",
        "region": region.0,
        "kind": memop_kind_json(*kind),
        "dst": dst.map(|value| value.as_u32()),
        "operands": operands.iter().map(|value| value.as_u32()).collect::<Vec<_>>(),
        "effects": format!("{:?}", effects),
    })
}

fn memop_kind_json(kind: MemOpKind) -> &'static str {
    match kind {
        MemOpKind::AddrOf => "addr_of",
        MemOpKind::LogicalShr => "logical_shr",
        MemOpKind::BitAnd => "bit_and",
        MemOpKind::Add => "add",
        MemOpKind::Sub => "sub",
        MemOpKind::TableIndex => "table_index",
        MemOpKind::FieldLoad => "field_load",
        MemOpKind::FieldStore => "field_store",
        MemOpKind::CurrentAllocOwnerId => "current_alloc_owner_id",
        MemOpKind::OwnerEq => "owner_eq",
    }
}
