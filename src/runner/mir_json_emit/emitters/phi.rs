use serde_json::json;

use crate::mir::{MirInstruction, MirType};

pub(crate) fn emit_phi(
    inst: &MirInstruction,
    value_types: &std::collections::BTreeMap<crate::mir::ValueId, MirType>,
) -> Option<serde_json::Value> {
    let MirInstruction::Phi { dst, inputs, .. } = inst else {
        return None;
    };
    let incoming: Vec<_> = inputs
        .iter()
        .map(|(b, v)| json!([v.as_u32(), b.as_u32()]))
        .collect();
    // Phase 131-11-F: Add dst_type hint from metadata for all PHI instructions
    let mut phi_inst = json!({"op":"phi","dst": dst.as_u32(), "incoming": incoming});
    if let Some(dst_type) = value_types.get(dst) {
        let type_json = match dst_type {
            MirType::Integer => json!("i64"),
            MirType::Float => json!("f64"), // Phase 275 P0: Float PHI type
            MirType::String => json!({"kind": "string"}),
            MirType::Box(bt) => json!({"kind": "handle", "box_type": bt}),
            MirType::Bool => json!("i1"),
            MirType::Void => json!("void"),
            _ => json!(null),
        };
        if !type_json.is_null() {
            phi_inst["dst_type"] = type_json;
        }
    }
    Some(phi_inst)
}
