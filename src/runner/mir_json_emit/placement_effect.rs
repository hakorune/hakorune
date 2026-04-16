pub(super) fn build_placement_effect_routes_json(
    routes: &[crate::mir::PlacementEffectRoute],
) -> Vec<serde_json::Value> {
    routes
        .iter()
        .map(|route| {
            serde_json::json!({
                "block": route.block.map(|block| block.as_u32()),
                "instruction_index": route.instruction_index,
                "value": route.value.map(|value| value.as_u32()),
                "source_value": route.source_value.map(|value| value.as_u32()),
                "window_start": route.window_start.map(|value| value.as_u32()),
                "window_end": route.window_end.map(|value| value.as_u32()),
                "string_proof": build_string_proof_json(route.string_proof),
                "publication_boundary": route.publication_boundary.map(|boundary| boundary.to_string()),
                "source": route.source.to_string(),
                "subject": route.subject,
                "decision": route.decision.to_string(),
                "state": route.state.to_string(),
                "detail": route.detail,
                "reason": route.reason,
            })
        })
        .collect()
}

fn build_string_proof_json(
    proof: Option<crate::mir::PlacementEffectStringProof>,
) -> serde_json::Value {
    match proof {
        Some(crate::mir::PlacementEffectStringProof::BorrowedSlice { source, start, end }) => {
            serde_json::json!({
                "kind": "borrowed_slice",
                "source": source.as_u32(),
                "start": start.as_u32(),
                "end": end.as_u32(),
            })
        }
        Some(crate::mir::PlacementEffectStringProof::ConcatTriplet {
            left_value,
            left_source,
            left_start,
            left_end,
            middle,
            right_value,
            right_source,
            right_start,
            right_end,
            shared_source,
        }) => serde_json::json!({
            "kind": "concat_triplet",
            "left_value": left_value.map(|value| value.as_u32()),
            "left_source": left_source.as_u32(),
            "left_start": left_start.as_u32(),
            "left_end": left_end.as_u32(),
            "middle": middle.as_u32(),
            "right_value": right_value.map(|value| value.as_u32()),
            "right_source": right_source.as_u32(),
            "right_start": right_start.as_u32(),
            "right_end": right_end.as_u32(),
            "shared_source": shared_source,
        }),
        None => serde_json::Value::Null,
    }
}
