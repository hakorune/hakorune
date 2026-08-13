//! JSON projection of the Rust-owned Dynamic V2 AOT metadata projection.
//!
//! This encoder is intentionally not connected to the ordinary metadata
//! builder yet.  The canonical physical session will supply the projection
//! when the full activation cell is ready.

use crate::abi::text_scan_aot_export_facts::{TextScanLeaseCapabilityV1, TextScanValueLaneV1};
use crate::box_callable::provider_admission::DynamicV2AotCallMetadataProjectionV1;
use crate::mir::a_prime_i64_physical_receipt::APrimeI64LaneV1;
use serde_json::json;

pub(crate) fn insert_dynamic_v2_aot_call_admission_json(
    obj: &mut serde_json::Map<String, serde_json::Value>,
    projection: &DynamicV2AotCallMetadataProjectionV1,
) {
    let stamp = projection.plan_stamp();
    let value = json!({
        "schema_version": projection.schema_version(),
        "contract_id": projection.contract_id(),
        "profile": projection.profile(),
        "abi_revision": projection.abi_revision(),
        "wire_revision": projection.wire_revision(),
        "registry_generation": projection.registry_generation(),
        "return_type": "i64",
        "return_lane": a_prime_lane_name(projection.return_lane()),
        "function_effects": projection.function_effects().bits(),
        "formal_parameters": projection.formal_parameters().iter().map(|formal| json!({
            "role": formal.role().as_str(),
            "value_id": formal.value_id().as_u32(),
            "lane": a_prime_lane_name(formal.lane()),
        })).collect::<Vec<_>>(),
        "plan_stamp": {
            "compiler_domain": stamp.compiler_domain().get(),
            "invocation_ordinal": stamp.invocation_ordinal().get(),
        },
        "calls": projection.calls().iter().map(|call| {
            let entry = call.entry();
            json!({
                "role": call.role().as_str(),
                "site_id": call.site_id().0,
                "entry_id": entry.entry() as u32,
                "symbol": entry.symbol(),
                "abi_revision": projection.abi_revision(),
                "wire_revision": entry.call_abi().out_wire_revision,
                "receiver_lane": lane_name(entry.receiver_lane()),
                "argument_lanes": entry
                    .argument_lanes()
                    .iter()
                    .copied()
                    .map(lane_name)
                    .collect::<Vec<_>>(),
                "result_lane": lane_name(entry.result_lane()),
                "lease": lease_name(entry.lease()),
                "normal_shape": match call.normal_shape() {
                    crate::mir::checked_callout::CheckedCallOutNormalShapeV1::EndAuthorizedHandle { .. } => "end_authorized_handle",
                    crate::mir::checked_callout::CheckedCallOutNormalShapeV1::ImmediateI64 => "immediate_i64",
                },
                "outcome_slot": call.outcome_slot().0,
                "normal_result_dst": call.normal_result_dst().as_u32(),
                "effects": call.effects().bits(),
            })
        }).collect::<Vec<_>>(),
    });
    obj.insert("dynamic_v2_aot_call_admission_v2".to_string(), value);
}

fn lane_name(lane: TextScanValueLaneV1) -> &'static str {
    match lane {
        TextScanValueLaneV1::HostHandle => "opaque_handle",
        TextScanValueLaneV1::ImmediateI64 => "immediate_i64",
    }
}

fn a_prime_lane_name(lane: APrimeI64LaneV1) -> &'static str {
    match lane {
        APrimeI64LaneV1::OpaqueHandle => "opaque_handle",
        APrimeI64LaneV1::ImmediateI64 => "immediate_i64",
    }
}

fn lease_name(lease: TextScanLeaseCapabilityV1) -> &'static str {
    match lease {
        TextScanLeaseCapabilityV1::None => "none",
        TextScanLeaseCapabilityV1::EndAuthorized => "end_authorized",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transport_lane_names_match_python_projection() {
        assert_eq!(lane_name(TextScanValueLaneV1::HostHandle), "opaque_handle");
        assert_eq!(
            lane_name(TextScanValueLaneV1::ImmediateI64),
            "immediate_i64"
        );
        assert_eq!(lease_name(TextScanLeaseCapabilityV1::None), "none");
        assert_eq!(
            lease_name(TextScanLeaseCapabilityV1::EndAuthorized),
            "end_authorized"
        );
    }
}
