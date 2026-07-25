//! One-shot source-entry selection from sealed compiler route evidence.
//!
//! Backend entry helpers are intentionally not consulted here. The manifest
//! already owns the source route; this terminal only gives that route a typed
//! source-entry identity and keeps the manifest for the next handoff.

use super::raw_root_environment_manifest::RawRootEnvironmentManifestV1;
use super::raw_root_source_facts::RawRootSourceRouteV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum SelectedSourceEntryRouteV1 {
    Script,
    AppMain0,
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
    pub(in crate::mir) fn select(
        manifest: RawRootEnvironmentManifestV1,
    ) -> Self {
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
}
