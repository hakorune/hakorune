use super::*;
use crate::mir::string_kernel_plan::{StringKernelPlan, StringKernelPlanPart};

pub(super) fn build_string_kernel_plan_parts_json(
    plan: &StringKernelPlan,
) -> Vec<serde_json::Value> {
    plan.parts()
        .into_iter()
        .map(|part| match part {
            StringKernelPlanPart::Slice {
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
            StringKernelPlanPart::Const {
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

pub(super) fn build_string_kernel_plan_json(plan: &StringKernelPlan) -> serde_json::Value {
    let legality = plan.legality();
    json!({
        "version": plan.version,
        "plan_value": plan.plan_value.as_u32(),
        "family": plan.family.to_string(),
        "corridor_root": plan.corridor_root.as_u32(),
        "source_root": plan.source_root.map(|value| value.as_u32()),
        "borrow_contract": plan.borrow_contract.map(|contract| contract.to_string()),
        "publish_reason": plan.publish_reason.map(|reason| reason.to_string()),
        "publish_repr_policy": plan.publish_repr_policy.map(|repr| repr.to_string()),
        "stable_view_provenance": plan.stable_view_provenance.map(|provenance| provenance.to_string()),
        "parts": build_string_kernel_plan_parts_json(&plan),
        "known_length": plan.known_length,
        "retained_form": plan.retained_form.to_string(),
        "publication_boundary": plan.publication_boundary.map(|boundary| boundary.to_string()),
        "publication_contract": plan.publication_contract.map(|contract| contract.to_string()),
        "barriers": {
            "publication": plan.publication.map(|state| state.to_string()),
            "materialization": plan.materialization.map(|state| state.to_string()),
        },
        "consumer": plan.consumer.map(|consumer| consumer.to_string()),
        "text_consumer": plan.text_consumer.map(|consumer| consumer.to_string()),
        "carrier": plan.carrier.map(|carrier| carrier.to_string()),
        "verifier_owner": plan.verifier_owner.map(|owner| owner.to_string()),
        "direct_kernel_entry": plan.direct_kernel_entry.map(|state| json!({
            "state": state.to_string(),
        })),
        "legality": {
            "byte_exact": legality.byte_exact,
            "no_publish_inside": legality.no_publish_inside,
            "requires_kernel_text_slot": legality.requires_kernel_text_slot,
            "rejects_early_stable_box_now": legality.rejects_early_stable_box_now,
            "rejects_early_fresh_registry_handle": legality.rejects_early_fresh_registry_handle,
            "rejects_registry_backed_carrier": legality.rejects_registry_backed_carrier,
        },
        "read_alias": {
            "same_receiver": plan.read_alias.same_receiver,
            "source_window": plan.read_alias.source_window,
            "followup_substring": plan.read_alias.followup_substring,
            "piecewise_subrange": plan.read_alias.piecewise_subrange,
            "direct_set_consumer": plan.read_alias.direct_set_consumer,
            "shared_receiver": plan.read_alias.shared_receiver,
        },
        "slot_hop_substring": plan.slot_hop_substring.as_ref().map(|route| json!({
            "consumer_value": route.consumer_value.as_u32(),
            "start": route.start.as_u32(),
            "end": route.end.as_u32(),
            "instruction_index": route.instruction_index,
            "copy_instruction_indices": route.copy_instruction_indices,
        })),
        "loop_payload": plan.loop_payload.as_ref().map(|payload| json!({
            "seed_value": payload.seed_value.as_u32(),
            "seed_literal": payload.seed_literal,
            "seed_length": payload.seed_length,
            "loop_bound": payload.loop_bound,
            "split_length": payload.split_length,
        })),
    })
}
