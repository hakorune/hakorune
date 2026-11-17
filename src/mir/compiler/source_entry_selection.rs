//! Source-entry continuation for compiler-selected raw routes.
//!
//! Preserve the invocation brand, source route and sealed main-target
//! identity across raw binding, postprocessing and publication.

use crate::mir::module_invocation_identity::ModuleInvocationBrandV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum SelectedSourceEntryRouteV1 {
    Script,
    AppMain0,
}

#[derive(Debug)]
pub(in crate::mir) struct SelectedSourceEntryContinuationV1 {
    brand: ModuleInvocationBrandV1,
    route: SelectedSourceEntryRouteV1,
    target: crate::mir::builder::RawMainEntryTargetV1,
    _seal: SelectedSourceEntryContinuationSealV1,
}

#[derive(Debug)]
struct SelectedSourceEntryContinuationSealV1;

impl SelectedSourceEntryContinuationV1 {
    pub(in crate::mir) fn from_projection(brand: ModuleInvocationBrandV1, is_script: bool) -> Self {
        let route = if is_script {
            SelectedSourceEntryRouteV1::Script
        } else {
            SelectedSourceEntryRouteV1::AppMain0
        };
        let target = crate::mir::builder::raw_main_entry_target();
        Self {
            brand,
            route,
            target,
            _seal: SelectedSourceEntryContinuationSealV1,
        }
    }

    pub(in crate::mir) const fn brand(&self) -> ModuleInvocationBrandV1 {
        self.brand
    }

    pub(in crate::mir) const fn route(&self) -> SelectedSourceEntryRouteV1 {
        self.route
    }

    pub(in crate::mir) fn symbol(&self) -> &str {
        self.target.symbol()
    }

    pub(in crate::mir) const fn arity(&self) -> usize {
        self.target.arity()
    }


    pub(in crate::mir) fn target_matches(
        &self,
        target: &crate::mir::builder::RawMainEntryTargetV1,
    ) -> bool {
        &self.target == target
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raw_continuation_seals_main_target_from_root_slot_contract() {
        let brand = crate::mir::module_invocation_identity::ModuleInvocationBrandV1::legacy_test();
        let script = SelectedSourceEntryContinuationV1::from_projection(brand, true);
        assert_eq!(script.brand(), brand);
        assert_eq!(script.route(), SelectedSourceEntryRouteV1::Script);
        assert_eq!(script.symbol(), "main");
        assert_eq!(script.arity(), 0);

        let app = SelectedSourceEntryContinuationV1::from_projection(brand, false);
        assert_eq!(app.route(), SelectedSourceEntryRouteV1::AppMain0);
        assert_eq!(app.symbol(), "main");
        assert_eq!(app.arity(), 0);
    }
}
