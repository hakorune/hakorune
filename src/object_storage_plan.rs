//! Object representation planning vocabulary.
//!
//! This module is intentionally passive. It names exact-AOT object storage
//! outcomes, but it does not choose them, does not mutate MIR, and is not wired
//! to lowering. MIRBuilder records object meaning; later analysis can produce
//! these plans for backend consumers.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LayoutId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FieldId(pub u32);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldScalarPlan {
    pub field_id: FieldId,
    pub layout_id: LayoutId,
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
    GenericBox { reason: GenericBoxReason },
    HostHandleEscaped { reason: EscapeReason },
    ArcDynBox { reason: DynamicReason },
    ExactStackObject { layout_id: LayoutId },
    ExactNativeStruct { layout_id: LayoutId },
    Scalarized { fields: Vec<FieldScalarPlan> },
}

impl ObjectStoragePlan {
    #[inline]
    pub fn is_exact_candidate(&self) -> bool {
        matches!(
            self,
            Self::ExactStackObject { .. }
                | Self::ExactNativeStruct { .. }
                | Self::Scalarized { .. }
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

pub fn object_storage_plan_report_fields() -> &'static [(&'static str, &'static str)] {
    &[
        ("output_contract", "hako-object-storage-plan-ssot-v0"),
        ("mirbuilder_object_management_enabled", "0"),
        ("mirbuilder_records_object_meaning", "1"),
        ("box_callable_registry_is_callable_truth", "1"),
        ("routeplan_is_call_execution_truth", "1"),
        ("object_storage_plan_is_representation_truth", "1"),
        ("object_storage_plan_vocabulary_defined", "1"),
        ("object_storage_plan_execution_enabled", "0"),
        ("exact_object_shadow_ready", "1"),
        ("product_default_changed", "0"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_candidates_are_separate_from_generic_or_escaped_routes() {
        assert!(ObjectStoragePlan::ExactStackObject {
            layout_id: LayoutId(7),
        }
        .is_exact_candidate());
        assert!(ObjectStoragePlan::ExactNativeStruct {
            layout_id: LayoutId(7),
        }
        .is_exact_candidate());
        assert!(ObjectStoragePlan::Scalarized {
            fields: vec![FieldScalarPlan {
                field_id: FieldId(1),
                layout_id: LayoutId(7),
                scalar_type: ScalarStorageType::I64,
            }],
        }
        .is_exact_candidate());

        assert!(ObjectStoragePlan::GenericBox {
            reason: GenericBoxReason::MissingTypeProof,
        }
        .is_generic_or_escaped());
        assert!(ObjectStoragePlan::HostHandleEscaped {
            reason: EscapeReason::HostHandlePublicationRequired,
        }
        .is_generic_or_escaped());
        assert!(ObjectStoragePlan::ArcDynBox {
            reason: DynamicReason::ArcDynBoxCarrierRequired,
        }
        .is_generic_or_escaped());
    }

    #[test]
    fn report_fields_keep_execution_disabled() {
        let fields = object_storage_plan_report_fields();
        assert!(fields.contains(&("mirbuilder_object_management_enabled", "0")));
        assert!(fields.contains(&("object_storage_plan_is_representation_truth", "1")));
        assert!(fields.contains(&("object_storage_plan_vocabulary_defined", "1")));
        assert!(fields.contains(&("object_storage_plan_execution_enabled", "0")));
        assert!(fields.contains(&("exact_object_shadow_ready", "1")));
    }
}
