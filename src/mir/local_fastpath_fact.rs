/*!
 * LocalFastPathFact aggregation owner.
 *
 * Positive fastpath facts are backend-consumable proof. Producer families may
 * expose route/object evidence, but this module owns the final assignment to
 * `MirFunction.metadata.local_fastpath_facts` so producers cannot clobber each
 * other as the surface grows.
 */

use crate::mir::{map_repr_plan::MapReprPlan, MirFunction};
use crate::object_storage_plan::{
    AliasClassId, LocalFastPathFact, LocalFastPathSiteId, LocalKnownReceiverDirectCallShadowRow,
    LocalPublicationInventoryRow, ObjectBasicBlockId, ObjectInstructionIndex, ObjectStoragePlanId,
    ObjectValueId, PublicationState, RoutePlanId,
};

pub fn refresh_function_local_fastpath_facts(function: &mut MirFunction) {
    function.metadata.local_fastpath_facts =
        build_local_fastpath_facts_from_map_repr_plans(&function.metadata.map_repr_plans);
}

fn build_local_fastpath_facts_from_map_repr_plans(
    plans: &[MapReprPlan],
) -> Vec<LocalFastPathFact> {
    plans
        .iter()
        .enumerate()
        .filter_map(|(index, plan)| {
            if plan.route_id() != "map_repr.generic_hash_runtime" {
                return None;
            }
            if plan.source_route_kind() != "map_load_scalar_i64" {
                return None;
            }
            if plan.publication_policy_tag() != Some("no_publication") {
                return None;
            }
            if plan.return_shape_tag() != Some("scalar_i64_or_missing_zero") {
                return None;
            }
            let inventory = LocalPublicationInventoryRow::new(
                LocalFastPathSiteId(index as u32),
                ObjectBasicBlockId(plan.block().as_u32()),
                ObjectInstructionIndex(plan.instruction_index() as u32),
                ObjectValueId(plan.receiver_value().as_u32()),
                Some(AliasClassId(plan.receiver_value().as_u32())),
                PublicationState::Unpublished,
            );
            LocalKnownReceiverDirectCallShadowRow::new(
                inventory,
                Some("map_repr.generic_hash_runtime"),
                Some(RoutePlanId(index as u32)),
                Some(ObjectStoragePlanId(index as u32)),
            )
            .into_allowed_fact()
        })
        .collect()
}
