use super::fastpath::{LocalFastPathFact, LocalFastPathFallbackReason};
use super::ids::{
    AliasClassId, LocalFastPathSiteId, ObjectBasicBlockId, ObjectInstructionIndex,
    ObjectSiteLocation, ObjectStoragePlanId, ObjectValueId, RoutePlanId,
};
use super::publication::PublicationState;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LocalPublicationInventoryRow {
    pub site_id: LocalFastPathSiteId,
    pub block_id: ObjectBasicBlockId,
    pub instruction_index: ObjectInstructionIndex,
    pub value_id: ObjectValueId,
    pub alias_class: Option<AliasClassId>,
    pub publication_state: PublicationState,
    pub fallback_reason: Option<LocalFastPathFallbackReason>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalKnownReceiverDirectCallShadowRow {
    pub inventory: LocalPublicationInventoryRow,
    pub route_plan: Option<RoutePlanId>,
    pub storage_plan: Option<ObjectStoragePlanId>,
    pub candidate_fact: Option<LocalFastPathFact>,
    pub fallback_reason: Option<LocalFastPathFallbackReason>,
}

impl LocalPublicationInventoryRow {
    #[inline]
    pub const fn location(&self) -> ObjectSiteLocation {
        ObjectSiteLocation::new(self.block_id, self.instruction_index)
    }

    pub fn new(
        site_id: LocalFastPathSiteId,
        block_id: ObjectBasicBlockId,
        instruction_index: ObjectInstructionIndex,
        value_id: ObjectValueId,
        alias_class: Option<AliasClassId>,
        publication_state: PublicationState,
    ) -> Self {
        let fallback_reason = match (alias_class, publication_state.fallback_reason()) {
            (None, _) => Some(LocalFastPathFallbackReason::AliasUnknown),
            (Some(_), reason) => reason,
        };
        Self {
            site_id,
            block_id,
            instruction_index,
            value_id,
            alias_class,
            publication_state,
            fallback_reason,
        }
    }

    #[inline]
    pub fn can_feed_fastpath_eligibility(&self) -> bool {
        self.alias_class.is_some()
            && self.publication_state.permits_local_fast_path()
            && self.fallback_reason.is_none()
    }
}

impl LocalKnownReceiverDirectCallShadowRow {
    pub fn new(
        inventory: LocalPublicationInventoryRow,
        route_plan: Option<RoutePlanId>,
        storage_plan: Option<ObjectStoragePlanId>,
    ) -> Self {
        let fallback_reason = inventory
            .fallback_reason
            .or_else(|| {
                route_plan
                    .is_none()
                    .then_some(LocalFastPathFallbackReason::DynamicRoute)
            })
            .or_else(|| {
                storage_plan
                    .is_none()
                    .then_some(LocalFastPathFallbackReason::GenericStorage)
            });

        let candidate_fact = match (
            inventory.can_feed_fastpath_eligibility(),
            inventory.alias_class,
            route_plan,
            storage_plan,
            fallback_reason,
        ) {
            (true, Some(alias_class), Some(route_plan), Some(storage_plan), None) => {
                Some(LocalFastPathFact::known_receiver_direct_call(
                    inventory.site_id,
                    inventory.block_id,
                    inventory.instruction_index,
                    inventory.value_id,
                    alias_class,
                    route_plan,
                    storage_plan,
                ))
            }
            _ => None,
        };

        Self {
            inventory,
            route_plan,
            storage_plan,
            candidate_fact,
            fallback_reason,
        }
    }
}
