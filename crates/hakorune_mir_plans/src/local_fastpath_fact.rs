//! Local fastpath fact aggregation helpers.
//!
//! This module stays pure: it reads passive plan vocabulary and produces
//! backend-consumable `LocalFastPathFact` values. Main-crate MIR code owns the
//! final assignment into `MirFunction.metadata`.

use crate::map_repr_plan::MapReprPlan;
use crate::object_storage_plan::{
    AliasClassId, LocalFastPathFact, LocalFastPathSiteId, LocalKnownReceiverDirectCallShadowRow,
    LocalPublicationInventoryRow, ObjectBasicBlockId, ObjectInstructionIndex, ObjectStoragePlanId,
    ObjectValueId, PublicationState, RoutePlanId,
};

pub fn build_local_fastpath_facts_from_map_repr_plans(
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
            .map(|fact| fact.with_storage_plan(ObjectStoragePlanId(index as u32)))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::map_repr_plan::MapReprKind;
    use hakorune_mir_core::{BasicBlockId, ValueId};

    #[test]
    fn map_repr_generic_hash_runtime_produces_local_fastpath_fact() {
        let plan = map_scalar_i64_plan("map_repr.generic_hash_runtime", "map_load_scalar_i64");

        let facts = build_local_fastpath_facts_from_map_repr_plans(&[plan]);

        assert_eq!(facts.len(), 1);
        assert_eq!(facts[0].site_id, LocalFastPathSiteId(0));
        assert_eq!(facts[0].block_id(), ObjectBasicBlockId(3));
        assert_eq!(facts[0].instruction_index(), ObjectInstructionIndex(7));
        assert_eq!(facts[0].object_id, ObjectValueId(10));
        assert_eq!(facts[0].alias_class, AliasClassId(10));
        assert_eq!(facts[0].route_plan_label, "map_repr.generic_hash_runtime");
        assert_eq!(facts[0].route_plan, RoutePlanId(0));
        assert_eq!(facts[0].storage_plan, Some(ObjectStoragePlanId(0)));
    }

    #[test]
    fn non_scalar_map_repr_plan_does_not_produce_fastpath_fact() {
        let plan = map_scalar_i64_plan("map_repr.generic_hash_runtime", "map_has_scalar_i64");

        let facts = build_local_fastpath_facts_from_map_repr_plans(&[plan]);

        assert!(facts.is_empty());
    }

    fn map_scalar_i64_plan(route_id: &'static str, source_route_kind: &'static str) -> MapReprPlan {
        MapReprPlan::new(
            BasicBlockId::new(3),
            7,
            route_id,
            MapReprKind::GenericHashRuntime,
            "generic_method.get",
            source_route_kind,
            "nyash.map.scalar_load_hi",
            "MapBox".to_string(),
            Some("MapBox".to_string()),
            "get".to_string(),
            ValueId::new(10),
            Some(ValueId::new(11)),
            Some(ValueId::new(12)),
            Some("i64_const"),
            Some("scalar_i64_or_missing_zero"),
            "scalar_i64",
            Some("no_publication"),
            "scalar_i64_get",
            Some("exact"),
        )
    }
}
