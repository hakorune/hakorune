//! DECLACCESS-MANIFEST0: one source-derived environment authority.
//!
//! The manifest is sealed before PHYSICAL0 opens. It owns the exact source
//! facts used by the later root/body lane and is moved through CHILDREN0 and
//! CALLMAIN0 without reconstruction.

use super::raw_root_plan0::RawRootPlanV1;
use super::raw_root_source_facts::{RawRootSourceFactsErrorV1, RawRootSourceFactsV1};
use super::raw_runtime_inputs::RawRuntimeInputSnapshotV1;
use crate::mir::builder::{BuilderInvocationConfigV1, OwnedRawSourceV1};

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
    pub(in crate::mir) fn facts(&self) -> &RawRootSourceFactsV1 {
        &self.facts
    }

    pub(in crate::mir) fn runtime_inputs(&self) -> &RawRuntimeInputSnapshotV1 {
        &self.runtime_inputs
    }
}
