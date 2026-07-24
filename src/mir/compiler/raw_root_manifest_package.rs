//! DECLACCESS-MANIFEST0: source-bound package after exact manifest sealing.
//!
//! This product is the only package accepted by PHYSICAL0. Runtime inputs and
//! Builder configuration have already moved into the manifest; no duplicate
//! config/runtime fields remain in the physical owner.

use super::raw_root_eligibility::RawRootEligibilityV1;
use super::raw_root_environment_manifest::{
    RawRootEnvironmentManifestV1, RawRootPhysicalManifestV1,
};
use super::raw_root_plan0::RawRootPlanV1;
use super::raw_source_binding::RawRootContinuationV1;
use crate::mir::builder::OwnedRawSourceV1;
use crate::mir::module_invocation_identity::ModuleInvocationTokenV1;

#[derive(Debug)]
pub(in crate::mir) struct ManifestBoundRawRootPackageV1 {
    token: ModuleInvocationTokenV1,
    source: OwnedRawSourceV1,
    continuation: RawRootContinuationV1,
    module_name: Box<str>,
    plan: RawRootPlanV1,
    proof: RawRootEligibilityV1,
    manifest: RawRootEnvironmentManifestV1,
}

pub(in crate::mir) struct ManifestBoundRawRootPartsV1 {
    pub(in crate::mir::compiler) token: ModuleInvocationTokenV1,
    pub(in crate::mir::compiler) source: OwnedRawSourceV1,
    pub(in crate::mir::compiler) continuation: RawRootContinuationV1,
    pub(in crate::mir::compiler) module_name: Box<str>,
    pub(in crate::mir::compiler) plan: RawRootPlanV1,
    pub(in crate::mir::compiler) proof: RawRootEligibilityV1,
    pub(in crate::mir::compiler) manifest: RawRootEnvironmentManifestV1,
}

impl ManifestBoundRawRootPackageV1 {
    pub(in crate::mir) fn new(
        token: ModuleInvocationTokenV1,
        source: OwnedRawSourceV1,
        continuation: RawRootContinuationV1,
        module_name: Box<str>,
        plan: RawRootPlanV1,
        proof: RawRootEligibilityV1,
        manifest: RawRootEnvironmentManifestV1,
    ) -> Self {
        Self {
            token,
            source,
            continuation,
            module_name,
            plan,
            proof,
            manifest,
        }
    }

    pub(in crate::mir) const fn token(&self) -> &ModuleInvocationTokenV1 {
        &self.token
    }

    pub(in crate::mir) const fn continuation(&self) -> &RawRootContinuationV1 {
        &self.continuation
    }

    #[cfg(test)]
    pub(in crate::mir) const fn proof(&self) -> &RawRootEligibilityV1 {
        &self.proof
    }

    #[cfg(test)]
    pub(in crate::mir) const fn manifest(&self) -> &RawRootEnvironmentManifestV1 {
        &self.manifest
    }

    pub(in crate::mir) fn module_name(&self) -> &str {
        &self.module_name
    }

    pub(in crate::mir) fn into_physical_open_parts(self) -> ManifestBoundRawRootPartsV1 {
        ManifestBoundRawRootPartsV1 {
            token: self.token,
            source: self.source,
            continuation: self.continuation,
            module_name: self.module_name,
            plan: self.plan,
            proof: self.proof,
            manifest: self.manifest,
        }
    }
}
