use super::*;

pub(super) fn build_string_kernel_plan_parts_json(
    plan: &crate::mir::StringKernelPlan,
) -> Vec<serde_json::Value> {
    plan.parts()
        .into_iter()
        .map(|part| match part {
            crate::mir::StringKernelPlanPart::Slice {
                value,
                source,
                start,
                end,
            } => json!({
                "kind": "slice",
                "value": value.map(|value| value.as_u32()),
                "source": source.as_u32(),
                "start": start.as_u32(),
                "end": end.as_u32(),
            }),
            crate::mir::StringKernelPlanPart::Const {
                value,
                known_length,
                literal,
            } => json!({
                "kind": "const",
                "value": value.as_u32(),
                "known_length": known_length,
                "literal": literal,
            }),
        })
        .collect()
}

pub(super) fn build_string_kernel_plan_json(
    function: &crate::mir::MirFunction,
    candidates: &[crate::mir::StringCorridorCandidate],
) -> Option<serde_json::Value> {
    let plan = crate::mir::derive_string_kernel_plan(function, candidates)?;
    let legality = plan.legality();
    Some(json!({
        "version": plan.version,
        "family": plan.family.to_string(),
        "corridor_root": plan.corridor_root.as_u32(),
        "source_root": plan.source_root.map(|value| value.as_u32()),
        "parts": build_string_kernel_plan_parts_json(&plan),
        "known_length": plan.known_length,
        "retained_form": plan.retained_form.to_string(),
        "barriers": {
            "publication": plan.publication.map(|state| state.to_string()),
            "materialization": plan.materialization.map(|state| state.to_string()),
        },
        "consumer": plan.consumer.map(|consumer| consumer.to_string()),
        "direct_kernel_entry": plan.direct_kernel_entry.map(|state| json!({
            "state": state.to_string(),
        })),
        "legality": {
            "byte_exact": legality.byte_exact,
            "no_publish_inside": legality.no_publish_inside,
        },
        "loop_payload": plan.loop_payload.as_ref().map(|payload| json!({
            "seed_value": payload.seed_value.as_u32(),
            "seed_literal": payload.seed_literal,
            "seed_length": payload.seed_length,
            "loop_bound": payload.loop_bound,
            "split_length": payload.split_length,
        })),
    }))
}
