//! DECLACCESS-MANIFEST0: one source-derived environment authority.
//!
//! The manifest is sealed before PHYSICAL0 opens. It owns the exact source
//! facts used by the later root/body lane and is moved through CHILDREN0 and
//! CALLMAIN0 without reconstruction.

use super::raw_root_plan0::RawRootPlanV1;
use super::raw_root_source_facts::{
    RawRootPostInstallFactsV1, RawRootSourceFactsErrorV1, RawRootSourceFactsV1,
    RawRootSourceRouteV1,
};
use super::raw_runtime_inputs::RawRuntimeInputSnapshotV1;
use crate::mir::builder::{BuilderInvocationConfigV1, OwnedRawSourceV1};
use crate::mir::builder::{RawRootEnvironmentInstallRouteV1, RawRootEnvironmentProjectionV1};

#[derive(Debug)]
pub(in crate::mir) struct RawRootEnvironmentManifestV1 {
    facts: RawRootSourceFactsV1,
    runtime_inputs: RawRuntimeInputSnapshotV1,
    config: BuilderInvocationConfigV1,
}

#[derive(Debug)]
pub(in crate::mir) struct RawRootPhysicalManifestV1 {
    facts: RawRootSourceFactsV1,
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

impl RawRootEnvironmentManifestV1 {
    pub(in crate::mir) fn from_facts(
        facts: RawRootSourceFactsV1,
        runtime_inputs: RawRuntimeInputSnapshotV1,
        config: BuilderInvocationConfigV1,
    ) -> Self {
        Self {
            facts,
            runtime_inputs,
            config,
        }
    }

    pub(in crate::mir) fn source_facts(
        source: &OwnedRawSourceV1,
        plan: &RawRootPlanV1,
    ) -> Result<RawRootSourceFactsV1, RawRootSourceFactsErrorV1> {
        RawRootSourceFactsV1::from_source(source, plan)
    }

    pub(in crate::mir::compiler) fn into_physical_parts(
        self,
    ) -> (RawRootPhysicalManifestV1, BuilderInvocationConfigV1) {
        let Self {
            facts,
            runtime_inputs,
            config,
        } = self;
        (
            RawRootPhysicalManifestV1 {
                facts,
                runtime_inputs,
            },
            config,
        )
    }

    pub(in crate::mir) fn facts(&self) -> &RawRootSourceFactsV1 {
        &self.facts
    }

    pub(in crate::mir) const fn route(&self) -> super::raw_root_source_facts::RawRootSourceRouteV1 {
        self.facts.route()
    }
}

impl RawRootPhysicalManifestV1 {
    #[cfg(test)]
    pub(in crate::mir) fn from_test(route: RawRootSourceRouteV1) -> Self {
        Self {
            facts: RawRootSourceFactsV1::empty_for_test(route),
            runtime_inputs: RawRuntimeInputSnapshotV1::capture().expect("test env snapshot"),
        }
    }

    pub(in crate::mir) fn facts(&self) -> &RawRootSourceFactsV1 {
        &self.facts
    }

    pub(in crate::mir) fn runtime_inputs(&self) -> &RawRuntimeInputSnapshotV1 {
        &self.runtime_inputs
    }

    pub(in crate::mir) fn route(&self) -> RawRootSourceRouteV1 {
        self.facts.route()
    }

    /// Consume the manifest once into the Builder projection and the exact
    /// BODY/runtime remainder. No catalog clone or second source scan occurs.
    pub(in crate::mir) fn into_install_parts(
        self,
        source_file: Option<&str>,
    ) -> (RawRootEnvironmentProjectionV1, RawRootPostInstallManifestV1) {
        let Self {
            facts,
            runtime_inputs,
        } = self;
        let (facts, catalog) = facts.into_post_install_parts();
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

impl RawRootPostInstallManifestV1 {
    pub(in crate::mir) fn facts(&self) -> &RawRootPostInstallFactsV1 {
        &self.facts
    }

    pub(in crate::mir) fn runtime_inputs(&self) -> &RawRuntimeInputSnapshotV1 {
        &self.runtime_inputs
    }
}
