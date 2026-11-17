//! DECLACCESS-MANIFEST0: one source-derived environment authority.
//!
//! The manifest is sealed before PHYSICAL0 opens. It owns the exact source
//! facts used by the later root/body lane and is moved through CHILDREN0 and
//! CALLMAIN0 without reconstruction.

use super::raw_root_source_facts::{
    RawRootPostInstallFactsV1, RawRootSourceFactsV1,
    RawRootSourceRouteV1,
};
use super::raw_runtime_inputs::RawRuntimeInputSnapshotV1;
use crate::mir::builder::{
    VerifiedSameModuleCallableDeclarationCatalogV1,
};
use crate::mir::builder::{RawRootEnvironmentInstallRouteV1, RawRootEnvironmentProjectionV1};


#[derive(Debug)]
pub(in crate::mir) struct RawRootPhysicalManifestV1 {
    facts: RawRootPostInstallFactsV1,
    catalog: VerifiedSameModuleCallableDeclarationCatalogV1,
    runtime_inputs: RawRuntimeInputSnapshotV1,
}

/// The source/body remainder after the Builder projection is consumed.
/// Callable catalog and declaration facts live in the installed Builder/shell
/// owner and are never duplicated here.
#[derive(Debug, PartialEq)]
pub(in crate::mir) struct RawRootPostInstallManifestV1 {
    facts: RawRootPostInstallFactsV1,
    runtime_inputs: RawRuntimeInputSnapshotV1,
}


impl RawRootPhysicalManifestV1 {
    #[cfg(test)]
    pub(in crate::mir) fn from_test(route: RawRootSourceRouteV1) -> Self {
        let (facts, catalog) = RawRootSourceFactsV1::empty_for_test(route)
            .into_post_install_parts()
            .expect("test body recipe");
        Self {
            facts,
            catalog,
            runtime_inputs: RawRuntimeInputSnapshotV1::capture().expect("test env snapshot"),
        }
    }

    /// Consume the manifest once into the Builder projection and the exact
    /// BODY/runtime remainder. No catalog clone or second source scan occurs.
    pub(in crate::mir) fn into_install_parts(
        self,
        source_file: Option<&str>,
    ) -> (RawRootEnvironmentProjectionV1, RawRootPostInstallManifestV1) {
        let Self {
            facts,
            catalog,
            runtime_inputs,
        } = self;
        let route = match facts.route() {
            RawRootSourceRouteV1::Script => RawRootEnvironmentInstallRouteV1::Script,
            RawRootSourceRouteV1::App => RawRootEnvironmentInstallRouteV1::App,
        };
        let projection = RawRootEnvironmentProjectionV1::from_parts(route, source_file, catalog);
        (
            projection,
            RawRootPostInstallManifestV1 {
                facts,
                runtime_inputs,
            },
        )
    }
}
