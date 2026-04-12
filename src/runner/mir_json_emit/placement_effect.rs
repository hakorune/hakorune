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
