//! CUT0-I0-COMMIT0: paired Builder/module external commit product.
//!
//! This remains disconnected from public ingress.  It is the only product
//! allowed to pair a postprocessed module with the Builder readiness owner.

use super::module_postprocess::{ModuleVerificationEvidenceV1, PostprocessedModuleInvocationV1};
use super::MirCompileResult;
use crate::mir::builder::{MirBuilder, PreparedBuilderExternalCommitV1};
use crate::mir::module_invocation_identity::{ModuleInvocationFamilyV1, ModuleInvocationTokenV1};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir) enum ExternalCommitPreparationErrorV1 {
    ForeignBrand,
    EvidenceMismatch,
}

#[derive(Debug)]
pub(in crate::mir) struct PreparedModuleExternalCommitV1 {
    token: ModuleInvocationTokenV1,
    builder: PreparedBuilderExternalCommitV1,
    module: crate::mir::MirModule,
    verification: ModuleVerificationEvidenceV1,
    _seal: PreparedModuleExternalCommitSealV1,
}

#[derive(Debug)]
struct PreparedModuleExternalCommitSealV1;

impl PreparedModuleExternalCommitV1 {
    pub(in crate::mir) fn prepare(
        postprocessed: PostprocessedModuleInvocationV1<'_>,
    ) -> Result<Self, ExternalCommitPreparationErrorV1> {
        let (token, builder, module, verification) = postprocessed.into_external_commit_parts();
        if token.brand() != builder.brand() || token.family() != builder.family() {
            return Err(ExternalCommitPreparationErrorV1::ForeignBrand);
        }
        let evidence_matches = match (&token.family(), &verification) {
            (ModuleInvocationFamilyV1::Raw, ModuleVerificationEvidenceV1::Raw { .. }) => true,
            (
                ModuleInvocationFamilyV1::CanonicalAPlus,
                ModuleVerificationEvidenceV1::Canonical { .. },
            )
            | (
                ModuleInvocationFamilyV1::BindingSsaTrivial,
                ModuleVerificationEvidenceV1::Canonical { .. },
            )
            | (
                ModuleInvocationFamilyV1::BindingSsaAcyclic,
                ModuleVerificationEvidenceV1::Canonical { .. },
            )
            | (
                ModuleInvocationFamilyV1::BindingSsaRecursive,
                ModuleVerificationEvidenceV1::Canonical { .. },
            ) => true,
            _ => false,
        };
        if !evidence_matches {
            return Err(ExternalCommitPreparationErrorV1::EvidenceMismatch);
        }
        Ok(Self {
            token,
            builder,
            module,
            verification,
            _seal: PreparedModuleExternalCommitSealV1,
        })
    }

    pub(in crate::mir) fn commit(self, current: &mut MirBuilder) -> MirCompileResult {
        let Self {
            token: _,
            builder,
            module,
            verification,
            _seal: _,
        } = self;
        builder.commit(current);
        let verification_result = match verification {
            ModuleVerificationEvidenceV1::Canonical { pre_transform, .. }
            | ModuleVerificationEvidenceV1::Raw { pre_transform } => {
                pre_transform.map_err(|errors| errors.into_vec())
            }
        };
        MirCompileResult {
            module,
            verification_result,
        }
    }
}

impl super::MirCompiler {
    pub(in crate::mir) fn prepare_module_external_commit(
        &mut self,
        postprocessed: PostprocessedModuleInvocationV1<'_>,
    ) -> Result<PreparedModuleExternalCommitV1, ExternalCommitPreparationErrorV1> {
        PreparedModuleExternalCommitV1::prepare(postprocessed)
    }

    pub(in crate::mir) fn commit_prepared_module(
        &mut self,
        prepared: PreparedModuleExternalCommitV1,
    ) -> MirCompileResult {
        prepared.commit(&mut self.builder)
    }
}
