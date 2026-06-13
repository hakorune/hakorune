//! Box object model replacement map for Arc retirement.
//!
//! This module records the contract needed before `Arc<dyn NyashBox>` can be
//! retired from any Box family.  It is a reportable design surface, not a new
//! dispatch implementation and not a TypeAbiCatalog truth owner.

/// Clone/share behavior a Box family must preserve during Arc retirement.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CloneShareSemantics {
    /// `clone_box` returns an independent value copy.
    FreshValueClone,
    /// `share_box` preserves stateful identity.
    StatePreservingShare,
    /// `share_box` is the identity-preserving operation for the family.
    IdentityShare,
    /// Plugin clone asks the plugin to create a new instance.
    PluginCloneCreatesInstance,
    /// Plugin share preserves the plugin instance handle.
    PluginSharePreservesInstance,
    /// Semantics still require family-level classification.
    Unknown,
}

/// Dispatch/type surface currently provided by `dyn NyashBox`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BoxDispatchSurface {
    DynNyashBox,
    BoxCore,
    AnyDowncast,
    TypeName,
    ParentTypeId,
    BoxCallableRegistry,
    TypeAbiProjection,
}

/// Current owner of finalization / fini behavior for a Box family.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PluginLifecycleOwner {
    NotPlugin,
    PluginHandleInnerDrop,
    PluginHandleInnerFinalizeNow,
    LeakTrackerDiagnostic,
}

/// Current and future carrier for VM object references.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum VMBoxRefCarrier {
    DirectVmScalar,
    StableTextPayload,
    ArcDynNyashBox,
    WeakDynNyashBox,
    ObjectHandle,
    WeakObjectHandle,
}

/// Migration plan for `VMValue::BoxRef`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct VMBoxRefCarrierMigrationPlan {
    pub current_strong: VMBoxRefCarrier,
    pub future_strong: VMBoxRefCarrier,
    pub current_weak: VMBoxRefCarrier,
    pub future_weak: VMBoxRefCarrier,
}

impl VMBoxRefCarrierMigrationPlan {
    pub const CURRENT_TO_OBJECT_HANDLE: Self = Self {
        current_strong: VMBoxRefCarrier::ArcDynNyashBox,
        future_strong: VMBoxRefCarrier::ObjectHandle,
        current_weak: VMBoxRefCarrier::WeakDynNyashBox,
        future_weak: VMBoxRefCarrier::WeakObjectHandle,
    };
}

/// Aggregate map used by ARC-RETIRE-005A..005D.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BoxObjectModelReplacementMap {
    pub clone_share: &'static [CloneShareSemantics],
    pub dispatch_surfaces: &'static [BoxDispatchSurface],
    pub plugin_lifecycle: &'static [PluginLifecycleOwner],
    pub vm_boxref_carrier: VMBoxRefCarrierMigrationPlan,
}

const CLONE_SHARE_SURFACES: &[CloneShareSemantics] = &[
    CloneShareSemantics::FreshValueClone,
    CloneShareSemantics::StatePreservingShare,
    CloneShareSemantics::IdentityShare,
    CloneShareSemantics::PluginCloneCreatesInstance,
    CloneShareSemantics::PluginSharePreservesInstance,
    CloneShareSemantics::Unknown,
];

const DISPATCH_SURFACES: &[BoxDispatchSurface] = &[
    BoxDispatchSurface::DynNyashBox,
    BoxDispatchSurface::BoxCore,
    BoxDispatchSurface::AnyDowncast,
    BoxDispatchSurface::TypeName,
    BoxDispatchSurface::ParentTypeId,
    BoxDispatchSurface::BoxCallableRegistry,
    BoxDispatchSurface::TypeAbiProjection,
];

const PLUGIN_LIFECYCLE_OWNERS: &[PluginLifecycleOwner] = &[
    PluginLifecycleOwner::NotPlugin,
    PluginLifecycleOwner::PluginHandleInnerDrop,
    PluginLifecycleOwner::PluginHandleInnerFinalizeNow,
    PluginLifecycleOwner::LeakTrackerDiagnostic,
];

/// Return the current replacement map without inspecting any live object.
pub fn box_object_model_replacement_map() -> BoxObjectModelReplacementMap {
    BoxObjectModelReplacementMap {
        clone_share: CLONE_SHARE_SURFACES,
        dispatch_surfaces: DISPATCH_SURFACES,
        plugin_lifecycle: PLUGIN_LIFECYCLE_OWNERS,
        vm_boxref_carrier: VMBoxRefCarrierMigrationPlan::CURRENT_TO_OBJECT_HANDLE,
    }
}

/// Stable report fields for ARC-RETIRE-005A..005D.
pub fn box_object_model_report_fields() -> &'static [(&'static str, &'static str)] {
    &[
        ("box_object_model_replacement_map", "1"),
        ("clone_share_semantics_classified", "1"),
        ("identity_share_box_count_reported", "1"),
        ("clone_returns_fresh_value_count_reported", "1"),
        ("share_preserves_state_count_reported", "1"),
        ("plugin_clone_share_semantics_reported", "1"),
        ("dyn_dispatch_surface_reported", "1"),
        ("downcast_typeid_surface_reported", "1"),
        ("plugin_lifecycle_owner_defined", "1"),
        ("vmvalue_boxref_carrier_migration_plan", "1"),
        ("vmvalue_boxref_current_carrier", "arc_dyn_nyashbox"),
        ("vmvalue_boxref_future_carrier", "object_handle"),
        ("vmvalue_weakbox_current_carrier", "weak_dyn_nyashbox"),
        ("vmvalue_weakbox_future_carrier", "weak_object_handle"),
        ("typeabi_identity_truth_count", "0"),
        ("arc_hot_path_retirement_started", "0"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replacement_map_covers_005a_to_005d() {
        let map = box_object_model_replacement_map();

        assert!(map
            .clone_share
            .contains(&CloneShareSemantics::PluginSharePreservesInstance));
        assert!(map
            .dispatch_surfaces
            .contains(&BoxDispatchSurface::AnyDowncast));
        assert!(map
            .plugin_lifecycle
            .contains(&PluginLifecycleOwner::PluginHandleInnerDrop));
        assert_eq!(
            map.vm_boxref_carrier,
            VMBoxRefCarrierMigrationPlan::CURRENT_TO_OBJECT_HANDLE
        );
    }

    #[test]
    fn vm_boxref_carrier_plan_moves_to_object_handles() {
        let plan = VMBoxRefCarrierMigrationPlan::CURRENT_TO_OBJECT_HANDLE;

        assert_eq!(plan.current_strong, VMBoxRefCarrier::ArcDynNyashBox);
        assert_eq!(plan.future_strong, VMBoxRefCarrier::ObjectHandle);
        assert_eq!(plan.current_weak, VMBoxRefCarrier::WeakDynNyashBox);
        assert_eq!(plan.future_weak, VMBoxRefCarrier::WeakObjectHandle);
    }

    #[test]
    fn report_fields_keep_typeabi_out_of_identity_truth() {
        let fields = box_object_model_report_fields();

        assert!(fields.contains(&("box_object_model_replacement_map", "1")));
        assert!(fields.contains(&("clone_share_semantics_classified", "1")));
        assert!(fields.contains(&("plugin_lifecycle_owner_defined", "1")));
        assert!(fields.contains(&("vmvalue_boxref_carrier_migration_plan", "1")));
        assert!(fields.contains(&("typeabi_identity_truth_count", "0")));
        assert!(fields.contains(&("arc_hot_path_retirement_started", "0")));
    }
}
