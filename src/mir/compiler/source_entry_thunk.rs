//! Source-entry result transport after route selection.
//!
//! This is intentionally a compiler-internal, backend-neutral handoff. The
//! selected route is consumed once and never reconstructed from a symbol,
//! module name, or backend entry helper.

use super::source_entry_result::SourceEntryResultV1;
use super::source_entry_selection::{SelectedSourceEntryRouteV1, SelectedSourceEntryV1};

#[derive(Debug)]
pub(in crate::mir) struct SourceEntryThunkV1 {
    selected: SelectedSourceEntryV1,
    _seal: SourceEntryThunkSealV1,
}

#[derive(Debug)]
pub(in crate::mir) struct CompletedSourceEntryV1 {
    selected: SelectedSourceEntryV1,
    result: SourceEntryResultV1,
    _seal: CompletedSourceEntrySealV1,
}

#[derive(Debug)]
struct SourceEntryThunkSealV1;

#[derive(Debug)]
struct CompletedSourceEntrySealV1;

impl SelectedSourceEntryV1 {
    /// Consume the selected source identity before any result is transported.
    pub(in crate::mir) fn begin_thunk(self) -> SourceEntryThunkV1 {
        SourceEntryThunkV1 {
            selected: self,
            _seal: SourceEntryThunkSealV1,
        }
    }
}

impl SourceEntryThunkV1 {
    pub(in crate::mir) const fn route(&self) -> SelectedSourceEntryRouteV1 {
        self.selected.route()
    }

    /// Seal exactly one source result; this does not project a process status.
    pub(in crate::mir) fn complete(
        self,
        result: SourceEntryResultV1,
    ) -> CompletedSourceEntryV1 {
        CompletedSourceEntryV1 {
            selected: self.selected,
            result,
            _seal: CompletedSourceEntrySealV1,
        }
    }
}

impl CompletedSourceEntryV1 {
    pub(in crate::mir) const fn route(&self) -> SelectedSourceEntryRouteV1 {
        self.selected.route()
    }

    pub(in crate::mir) fn result(&self) -> &SourceEntryResultV1 {
        &self.result
    }

    pub(in crate::mir) fn into_parts(
        self,
    ) -> (SelectedSourceEntryV1, SourceEntryResultV1) {
        (self.selected, self.result)
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
    fn selected_script_transports_one_unit_result_without_route_reselection() {
        let selected = select_source_entry(RawRootEnvironmentManifestV1::from_test(
            RawRootSourceRouteV1::Script,
        ));
        let thunk = selected.begin_thunk();
        assert_eq!(thunk.route(), SelectedSourceEntryRouteV1::Script);
        let completed = thunk.complete(SourceEntryResultV1::Unit(UnitOriginV1::EmptyBody));
        assert_eq!(completed.route(), SelectedSourceEntryRouteV1::Script);
        assert!(matches!(completed.result(), SourceEntryResultV1::Unit(_)));
    }

    #[test]
    fn selected_app_transports_integer_result_and_keeps_typed_route() {
        let selected = select_source_entry(RawRootEnvironmentManifestV1::from_test(
            RawRootSourceRouteV1::App,
        ));
        let completed = selected
            .begin_thunk()
            .complete(SourceEntryResultV1::Integer(42));
        assert_eq!(completed.route(), SelectedSourceEntryRouteV1::AppMain0);
        assert!(matches!(completed.result(), SourceEntryResultV1::Integer(42)));
        let (selected, _result) = completed.into_parts();
        assert_eq!(selected.route(), SelectedSourceEntryRouteV1::AppMain0);
    }
}
