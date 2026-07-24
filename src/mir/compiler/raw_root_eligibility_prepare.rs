//! COVERAGE0-REPAIR: profile-specific eligibility handoff.
//!
//! The general Raw eligibility authority remains reusable for internal
//! fixtures. Public NarrowV1 opts into the branded StaticHelper0 witness
//! without hiding that policy inside the general verifier.

use super::raw_root_eligibility::{
    EligibleSourceBoundRawRootPackageV1, RawRootEligibilityErrorV1,
    RawRootEligibilityStageV1, RawRootEligibilityV1, RejectedRawRootEligibilityV1,
};
use super::raw_root_environment_manifest::RawRootEnvironmentManifestV1;
use super::raw_root_helper_coverage::RawPublicEligibilityProfileV1;
use super::raw_root_manifest_package::ManifestBoundRawRootPackageV1;
use super::raw_root_package::SourceBoundRawRootPackageV1;

impl SourceBoundRawRootPackageV1 {
    pub(in crate::mir) fn prepare_eligibility(
        self,
    ) -> Result<EligibleSourceBoundRawRootPackageV1, RejectedRawRootEligibilityV1> {
        self.prepare_with(RawRootEligibilityV1::verify)
    }

    pub(in crate::mir) fn prepare_public_eligibility(
        self,
        profile: RawPublicEligibilityProfileV1,
    ) -> Result<EligibleSourceBoundRawRootPackageV1, RejectedRawRootEligibilityV1> {
        self.prepare_with(|package| RawRootEligibilityV1::verify_public(package, profile))
    }

    fn prepare_with(
        self,
        verify: impl FnOnce(
            &SourceBoundRawRootPackageV1,
        ) -> Result<RawRootEligibilityV1, (RawRootEligibilityStageV1, RawRootEligibilityErrorV1)>,
    ) -> Result<EligibleSourceBoundRawRootPackageV1, RejectedRawRootEligibilityV1> {
        match verify(&self) {
            Ok(proof) => {
                let facts = match RawRootEnvironmentManifestV1::source_facts(
                    self.source(),
                    self.plan(),
                ) {
                    Ok(facts) => facts,
                    Err(error) => {
                        return Err(RejectedRawRootEligibilityV1 {
                            owner: self,
                            stage: RawRootEligibilityStageV1::Manifest,
                            error: RawRootEligibilityErrorV1::Manifest(error),
                        });
                    }
                };
                let manifest = match RawRootEnvironmentManifestV1::from_facts(
                    facts,
                    self.runtime_inputs().clone(),
                    self.config().clone(),
                ) {
                    Ok(manifest) => manifest,
                    Err(error) => {
                        return Err(RejectedRawRootEligibilityV1 {
                            owner: self,
                            stage: RawRootEligibilityStageV1::Manifest,
                            error: RawRootEligibilityErrorV1::BodyRecipe(error),
                        });
                    }
                };
                let (token, source, continuation, _runtime_inputs, _config, module_name, plan) =
                    self.into_manifest_parts();
                Ok(ManifestBoundRawRootPackageV1::new(
                    token,
                    source,
                    continuation,
                    module_name,
                    plan,
                    proof,
                    manifest,
                ))
            }
            Err((stage, error)) => Err(RejectedRawRootEligibilityV1 {
                owner: self,
                stage,
                error,
            }),
        }
    }
}
