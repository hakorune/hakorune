//! DECLACCESS-MANIFEST0: one source-derived environment authority.
//!
//! The manifest is sealed before PHYSICAL0 opens. It owns the exact source
//! facts used by the later root/body lane and is moved through CHILDREN0 and
//! CALLMAIN0 without reconstruction.

use super::raw_root_plan0::RawRootPlanV1;
use super::raw_root_source_facts::{RawRootSourceFactsErrorV1, RawRootSourceFactsV1};
use crate::mir::builder::OwnedRawSourceV1;

#[derive(Debug)]
pub(in crate::mir) struct RawRootEnvironmentManifestV1 {
    facts: RawRootSourceFactsV1,
}

impl RawRootEnvironmentManifestV1 {
    pub(in crate::mir) fn from_source(
        source: &OwnedRawSourceV1,
        plan: &RawRootPlanV1,
    ) -> Result<Self, RawRootSourceFactsErrorV1> {
        Ok(Self {
            facts: RawRootSourceFactsV1::from_source(source, plan)?,
        })
    }

    pub(in crate::mir) fn facts(&self) -> &RawRootSourceFactsV1 {
        &self.facts
    }

    pub(in crate::mir) const fn route(&self) -> super::raw_root_source_facts::RawRootSourceRouteV1 {
        self.facts.route()
    }
}
