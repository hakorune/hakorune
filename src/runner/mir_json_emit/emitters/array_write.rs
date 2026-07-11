use crate::mir::{ArrayElementWriteKind, ArrayWriteSiteId, ValueId};
use serde_json::json;

pub(super) fn emit(
    site_id: ArrayWriteSiteId,
    dst: Option<ValueId>,
    kind: ArrayElementWriteKind,
    receiver: ValueId,
    index: Option<ValueId>,
    value: ValueId,
) -> serde_json::Value {
    json!({
        "op": "array_element_write",
        "site_id": site_id.0,
        "dst": dst.map(|value| value.as_u32()),
        "kind": kind.as_str(),
        "receiver": receiver.as_u32(),
        "index": index.map(|value| value.as_u32()),
        "value": value.as_u32(),
    })
}
