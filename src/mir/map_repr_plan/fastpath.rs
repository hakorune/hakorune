use super::plans::MapReprPlan;
use crate::object_storage_plan::{
    AliasClassId, LocalFastPathFact, LocalFastPathSiteId, LocalKnownReceiverDirectCallShadowRow,
    LocalPublicationInventoryRow, ObjectBasicBlockId, ObjectInstructionIndex, ObjectStoragePlanId,
    ObjectValueId, PublicationState, RoutePlanId,
};

pub(super) fn build_local_fastpath_facts_from_map_repr_plans(
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
                Some(RoutePlanId(index as u32)),
                Some(ObjectStoragePlanId(index as u32)),
            )
            .into_allowed_fact()
        })
        .collect()
}
