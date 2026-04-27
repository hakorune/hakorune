use crate::mir::agg_local_scalarization::{AggLocalScalarizationKind, AggLocalScalarizationRoute};

pub(super) fn build_agg_local_scalarization_routes_json(
    routes: &[AggLocalScalarizationRoute],
) -> Vec<serde_json::Value> {
    routes
        .iter()
        .map(|route| match route.kind {
            AggLocalScalarizationKind::SumLocalLayout(layout) => serde_json::json!({
                "kind": "sum_local_layout",
                "block": route.block.map(|block| block.as_u32()),
                "instruction_index": route.instruction_index,
                "value": route.value.map(|value| value.as_u32()),
                "subject": route.subject,
                "layout": layout.to_string(),
                "reason": route.reason,
            }),
            AggLocalScalarizationKind::UserBoxLocalBody(value_class) => {
                serde_json::json!({
                    "kind": "user_box_local_body",
                    "block": route.block.map(|block| block.as_u32()),
                    "instruction_index": route.instruction_index,
                    "value": route.value.map(|value| value.as_u32()),
                    "subject": route.subject,
                    "value_class": value_class.to_string(),
                    "reason": route.reason,
                })
            }
            AggLocalScalarizationKind::TypedSlotStorage(storage_class) => {
                serde_json::json!({
                    "kind": "typed_slot_storage",
                    "block": route.block.map(|block| block.as_u32()),
                    "instruction_index": route.instruction_index,
                    "value": route.value.map(|value| value.as_u32()),
                    "subject": route.subject,
                    "storage_class": storage_class.to_string(),
                    "reason": route.reason,
                })
            }
        })
        .collect()
}
