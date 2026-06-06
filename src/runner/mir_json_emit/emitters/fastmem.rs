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
        "kind": kind.as_json_name(),
        "dst": dst.map(|value| value.as_u32()),
        "operands": operands.iter().map(|value| value.as_u32()).collect::<Vec<_>>(),
        "effects": format!("{:?}", effects),
    })
}
