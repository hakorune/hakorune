//! One-shot source-entry selection from sealed compiler route evidence.
//!
//! Backend entry helpers are intentionally not consulted here. The manifest
//! already owns the source route; this terminal only gives that route a typed
//! source-entry identity and keeps the manifest for the next handoff.

use super::raw_root_environment_manifest::RawRootEnvironmentManifestV1;
use super::raw_root_source_facts::RawRootSourceRouteV1;
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

    pub(in crate::mir) fn is_main_target(&self) -> bool {
        self.target.is_main()
    }

    pub(in crate::mir) fn target_matches(
        &self,
        target: &crate::mir::builder::RawMainEntryTargetV1,
    ) -> bool {
        &self.target == target
    }
}

#[derive(Debug)]
pub(in crate::mir) struct SelectedSourceEntryV1 {
    manifest: RawRootEnvironmentManifestV1,
    route: SelectedSourceEntryRouteV1,
    _seal: SelectedSourceEntrySealV1,
}

#[derive(Debug)]
struct SelectedSourceEntrySealV1;

impl SelectedSourceEntryV1 {
    pub(in crate::mir) fn select(manifest: RawRootEnvironmentManifestV1) -> Self {
        let route = match manifest.route() {
            RawRootSourceRouteV1::Script => SelectedSourceEntryRouteV1::Script,
            RawRootSourceRouteV1::App => SelectedSourceEntryRouteV1::AppMain0,
        };
        Self {
            manifest,
            route,
            _seal: SelectedSourceEntrySealV1,
        }
    }

    pub(in crate::mir) const fn route(&self) -> SelectedSourceEntryRouteV1 {
        self.route
    }

    pub(in crate::mir) fn manifest(&self) -> &RawRootEnvironmentManifestV1 {
        &self.manifest
    }

    /// The only consuming handoff. The route is never reconstructed from a
    /// symbol or module map after this point.
    pub(in crate::mir) fn into_parts(
        self,
    ) -> (RawRootEnvironmentManifestV1, SelectedSourceEntryRouteV1) {
        (self.manifest, self.route)
    }
}

pub(in crate::mir) fn select_source_entry(
    manifest: RawRootEnvironmentManifestV1,
) -> SelectedSourceEntryV1 {
    SelectedSourceEntryV1::select(manifest)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sealed_script_manifest_selects_script_route_once() {
        let selected = select_source_entry(RawRootEnvironmentManifestV1::from_test(
            RawRootSourceRouteV1::Script,
        ));
        assert_eq!(selected.route(), SelectedSourceEntryRouteV1::Script);
        assert_eq!(selected.manifest().route(), RawRootSourceRouteV1::Script);
        let (_manifest, route) = selected.into_parts();
        assert_eq!(route, SelectedSourceEntryRouteV1::Script);
    }

    #[test]
    fn sealed_app_manifest_selects_app_main_zero_route_once() {
        let selected = select_source_entry(RawRootEnvironmentManifestV1::from_test(
            RawRootSourceRouteV1::App,
        ));
        assert_eq!(selected.route(), SelectedSourceEntryRouteV1::AppMain0);
        assert_eq!(selected.manifest().route(), RawRootSourceRouteV1::App);
        let (_manifest, route) = selected.into_parts();
        assert_eq!(route, SelectedSourceEntryRouteV1::AppMain0);
    }

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
