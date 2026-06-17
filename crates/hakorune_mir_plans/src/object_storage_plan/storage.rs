use super::ids::ObjectValueId;
use super::ids::{FieldId, LayoutId};
use super::publication::ObjectPublicationSite;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldScalarPlan {
    pub field_id: FieldId,
    pub layout_id: LayoutId,
    pub scalar_type: ScalarStorageType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FlattenedNestedFieldPlan {
    pub owner_field_id: FieldId,
    pub nested_field_id: FieldId,
    pub flattened_field_id: FieldId,
    pub nested_layout_id: LayoutId,
    pub scalar_type: ScalarStorageType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ScalarStorageType {
    I64,
    U64,
    Usize,
    Bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GenericBoxReason {
    MissingTypeProof,
    MissingLayoutProof,
    DynamicNyashBoxApiRequired,
    UnknownDropOrFiniSemantics,
    UnsupportedBoundary,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EscapeReason {
    HostHandlePublicationRequired,
    PluginOrExternBoundary,
    ArrayOrMapDynamicStorage,
    ReturnEscapeUnplanned,
    SyncChannelFutureContextBoundary,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DynamicReason {
    ArcDynBoxCarrierRequired,
    TraitObjectDowncastRequired,
    RuntimeTypeIdentityRequired,
    PluginLifecycleRequired,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ObjectStoragePlan {
    GenericBox {
        reason: GenericBoxReason,
    },
    HostHandleEscaped {
        reason: EscapeReason,
    },
    ArcDynBox {
        reason: DynamicReason,
    },
    ExactNativeStruct {
        layout_id: LayoutId,
    },
    Scalarized {
        fields: Vec<FieldScalarPlan>,
    },
    FlattenedNestedFields {
        owner_layout_id: LayoutId,
        fields: Vec<FlattenedNestedFieldPlan>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectPlan {
    pub value_id: ObjectValueId,
    pub storage: ObjectStoragePlan,
    pub publication_sites: Vec<ObjectPublicationSite>,
}

impl ObjectStoragePlan {
    #[inline]
    pub fn is_exact_candidate(&self) -> bool {
        matches!(
            self,
            Self::ExactNativeStruct { .. }
                | Self::Scalarized { .. }
                | Self::FlattenedNestedFields { .. }
        )
    }

    #[inline]
    pub fn is_generic_or_escaped(&self) -> bool {
        matches!(
            self,
            Self::GenericBox { .. } | Self::HostHandleEscaped { .. } | Self::ArcDynBox { .. }
        )
    }
}

impl ObjectPlan {
    pub fn new(
        value_id: ObjectValueId,
        storage: ObjectStoragePlan,
        publication_sites: Vec<ObjectPublicationSite>,
    ) -> Self {
        Self {
            value_id,
            storage,
            publication_sites,
        }
    }

    #[inline]
    pub fn is_unpublished_local(&self) -> bool {
        self.publication_sites.is_empty() && self.storage.is_exact_candidate()
    }

    #[inline]
    pub fn requires_publication(&self) -> bool {
        !self.publication_sites.is_empty() || self.storage.is_generic_or_escaped()
    }
}
