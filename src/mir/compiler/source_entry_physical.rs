//! Backend-neutral physical source-entry carrier.
//!
//! This row transports a completed source result without opening a Builder,
//! exposing a MIR module, or projecting a process status. Those authorities
//! belong to later, explicitly selected rows.

use super::source_entry_result::SourceEntryResultV1;
use super::source_entry_selection::SelectedSourceEntryRouteV1;
use super::source_entry_thunk::CompletedSourceEntryV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum PhysicalEntryRoleV1 {
    SourceResultThunk,
}

#[derive(Debug)]
pub(in crate::mir) struct PhysicalSourceEntryCarrierV1 {
    completed: CompletedSourceEntryV1,
    role: PhysicalEntryRoleV1,
    _seal: PhysicalSourceEntryCarrierSealV1,
}

#[derive(Debug)]
struct PhysicalSourceEntryCarrierSealV1;

impl CompletedSourceEntryV1 {
    /// Consume the source-result carrier without reopening route selection.
    pub(in crate::mir) fn into_physical(self) -> PhysicalSourceEntryCarrierV1 {
        PhysicalSourceEntryCarrierV1 {
            completed: self,
            role: PhysicalEntryRoleV1::SourceResultThunk,
            _seal: PhysicalSourceEntryCarrierSealV1,
        }
    }
}

impl PhysicalSourceEntryCarrierV1 {
    pub(in crate::mir) const fn role(&self) -> PhysicalEntryRoleV1 {
        self.role
    }

    pub(in crate::mir) const fn route(&self) -> SelectedSourceEntryRouteV1 {
        self.completed.route()
    }

    pub(in crate::mir) fn result(&self) -> &SourceEntryResultV1 {
        self.completed.result()
    }

    pub(in crate::mir) fn into_completed(self) -> CompletedSourceEntryV1 {
        self.completed
    }
}

#[cfg(test)]
mod tests {
    use super::super::raw_root_environment_manifest::RawRootEnvironmentManifestV1;
    use super::super::raw_root_source_facts::RawRootSourceRouteV1;
    use super::super::source_entry_result::{SourceEntryResultV1, UnitOriginV1};
    use super::super::source_entry_selection::select_source_entry;
    use super::*;

    #[test]
    fn script_result_enters_physical_carrier_without_status_projection() {
        let completed = select_source_entry(RawRootEnvironmentManifestV1::from_test(
            RawRootSourceRouteV1::Script,
        ))
        .begin_thunk()
        .complete(SourceEntryResultV1::Unit(UnitOriginV1::ImplicitFallthrough));
        let carrier = completed.into_physical();
        assert_eq!(carrier.role(), PhysicalEntryRoleV1::SourceResultThunk);
        assert_eq!(carrier.route(), SelectedSourceEntryRouteV1::Script);
        assert!(matches!(carrier.result(), SourceEntryResultV1::Unit(_)));
    }

    #[test]
    fn app_result_keeps_typed_route_and_result_in_opaque_carrier() {
        let carrier = select_source_entry(RawRootEnvironmentManifestV1::from_test(
            RawRootSourceRouteV1::App,
        ))
        .begin_thunk()
        .complete(SourceEntryResultV1::Integer(7))
        .into_physical();
        assert_eq!(carrier.role(), PhysicalEntryRoleV1::SourceResultThunk);
        assert_eq!(carrier.route(), SelectedSourceEntryRouteV1::AppMain0);
        assert!(matches!(carrier.result(), SourceEntryResultV1::Integer(7)));
        let completed = carrier.into_completed();
        assert_eq!(completed.route(), SelectedSourceEntryRouteV1::AppMain0);
    }
}
