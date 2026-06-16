use super::ids::{
    AliasClassId, LocalFastPathSiteId, ObjectBasicBlockId, ObjectInstructionIndex,
    ObjectStoragePlanId, ObjectValueId, RoutePlanId,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LocalFastPathFallbackReason {
    OpenWorld,
    AliasUnknown,
    PublishedBeforeSite,
    MaybePublishedBeforeSite,
    DynamicRoute,
    GenericStorage,
    BackendMissing,
    UnknownCall,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LocalFastPathKind {
    KnownReceiverDirectCall,
    LocalFieldAccess,
    LocalStorageAccess,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalFastPathFact {
    pub site_id: LocalFastPathSiteId,
    pub block_id: ObjectBasicBlockId,
    pub instruction_index: ObjectInstructionIndex,
    pub object_id: ObjectValueId,
    pub alias_class: AliasClassId,
    pub route_plan: RoutePlanId,
    pub storage_plan: ObjectStoragePlanId,
    pub valid_until_publication: bool,
    pub backend_kind: LocalFastPathKind,
}

impl LocalFastPathFact {
    pub fn known_receiver_direct_call(
        site_id: LocalFastPathSiteId,
        block_id: ObjectBasicBlockId,
        instruction_index: ObjectInstructionIndex,
        object_id: ObjectValueId,
        alias_class: AliasClassId,
        route_plan: RoutePlanId,
        storage_plan: ObjectStoragePlanId,
    ) -> Self {
        Self {
            site_id,
            block_id,
            instruction_index,
            object_id,
            alias_class,
            route_plan,
            storage_plan,
            valid_until_publication: true,
            backend_kind: LocalFastPathKind::KnownReceiverDirectCall,
        }
    }
}
