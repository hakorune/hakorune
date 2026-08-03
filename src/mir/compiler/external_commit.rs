//! CUT0-I0-COMMIT0: paired Builder/module external commit product.
//!
//! This remains disconnected from public ingress.  It is the only product
//! allowed to pair a postprocessed module with the Builder readiness owner.

use super::module_postprocess::{
    CanonicalFinalVerificationSealInnerV1, CanonicalFinalVerificationSealV1,
    ModuleVerificationEvidenceV1, PostprocessEvidenceInputV1, PostprocessedModuleInvocationV1,
};
use super::publication_kernel::PublishedModuleTransferV1;
use super::MirCompileResult;
use crate::mir::builder::{
    CanonicalCallableCapabilityWitnessV1, CommitCallableCollectorBatchReceiptV1,
    CommitCollectedDraftAdmissionReceiptV1, InvocationBranded, MirBuilder,
    PreparedBuilderExternalCommitV1,
};
use crate::mir::canonical_physical_drain::CanonicalPhysicalDrainManifestV1;
use crate::mir::compiler::source_bound_package::CanonicalSourceContinuationV1;
use crate::mir::module_invocation_identity::{ModuleInvocationFamilyV1, ModuleInvocationTokenV1};
use crate::mir::verification_types::VerificationError;

struct LegacyPublicationPayload {
    module: crate::mir::MirModule,
    verification: ModuleVerificationEvidenceV1,
}

impl super::publication_kernel::SealedPublicationPayloadV1 for LegacyPublicationPayload {
    type Published = MirCompileResult;

    fn finish(
        self,
        _receipt: crate::mir::builder::BuilderPublicationReceiptV1,
        _module: PublishedModuleTransferV1,
    ) -> Self::Published {
        let verification_result = project_canonical_verification_result(self.verification);
        MirCompileResult {
            module: self.module,
            verification_result,
        }
    }
}

/// Canonical publication has already crossed `RequireFinal`. Its public
/// result therefore reports final-barrier success; the retained pre-transform
/// evidence remains diagnostic state and is not the result contract. Raw uses
/// a separate publication payload and keeps its reportable pre-transform
/// semantics.
fn project_canonical_verification_result(
    evidence: ModuleVerificationEvidenceV1,
) -> Result<(), Vec<VerificationError>> {
    match evidence {
        ModuleVerificationEvidenceV1::Canonical { .. } => Ok(()),
        ModuleVerificationEvidenceV1::Raw { .. } => {
            unreachable!("Raw evidence is owned by the RawDirect publication path")
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir) enum ExternalCommitPreparationErrorV1 {
    ForeignBrand,
    EvidenceMismatch,
}

#[derive(Debug)]
pub(in crate::mir) enum PostprocessEvidenceSealV1<'a> {
    CanonicalSingle {
        continuation: CanonicalSourceContinuationV1<'a>,
        receipt: InvocationBranded<CommitCollectedDraftAdmissionReceiptV1>,
        inventory: CanonicalPhysicalDrainManifestV1,
    },
    CanonicalCallable {
        continuation: CanonicalSourceContinuationV1<'a>,
        receipt: InvocationBranded<CommitCallableCollectorBatchReceiptV1>,
        inventory: CanonicalPhysicalDrainManifestV1,
        capability: CanonicalCallableCapabilityWitnessV1,
    },
}

#[derive(Debug)]
pub(in crate::mir) struct PreparedModuleExternalCommitV1<'a> {
    token: ModuleInvocationTokenV1,
    builder: PreparedBuilderExternalCommitV1,
    module: crate::mir::MirModule,
    verification: ModuleVerificationEvidenceV1,
    evidence: PostprocessEvidenceSealV1<'a>,
    _seal: PreparedModuleExternalCommitSealV1,
}

#[derive(Debug)]
struct PreparedModuleExternalCommitSealV1;

impl<'a> PreparedModuleExternalCommitV1<'a> {
    pub(in crate::mir) fn evidence(&self) -> &PostprocessEvidenceSealV1<'a> {
        &self.evidence
    }

    pub(in crate::mir) fn prepare(
        postprocessed: PostprocessedModuleInvocationV1<'a>,
    ) -> Result<Self, ExternalCommitPreparationErrorV1> {
        let (token, builder, module, verification, evidence_input) =
            postprocessed.into_external_commit_parts();
        if token.brand() != builder.brand() || token.family() != builder.family() {
            return Err(ExternalCommitPreparationErrorV1::ForeignBrand);
        }
        let evidence_matches = match (&token.family(), &verification) {
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
        let evidence = PostprocessEvidenceSealV1::seal(evidence_input, &token)?;
        Ok(Self {
            token,
            builder,
            module,
            verification,
            evidence,
            _seal: PreparedModuleExternalCommitSealV1,
        })
    }

    pub(in crate::mir) fn commit(self, current: &mut MirBuilder) -> MirCompileResult {
        let Self {
            token: _,
            builder,
            module,
            verification,
            evidence: _,
            _seal: _,
        } = self;
        super::publication_kernel::publish_once(
            current,
            builder,
            LegacyPublicationPayload {
                module,
                verification,
            },
            None,
        )
    }
}

impl<'a> PostprocessEvidenceSealV1<'a> {
    fn seal(
        input: PostprocessEvidenceInputV1<'a>,
        token: &ModuleInvocationTokenV1,
    ) -> Result<Self, ExternalCommitPreparationErrorV1> {
        let brand = token.brand();
        let family = token.family();
        match input {
            PostprocessEvidenceInputV1::CanonicalSingle {
                continuation,
                receipt,
                inventory,
            } => {
                if !matches!(
                    family,
                    ModuleInvocationFamilyV1::CanonicalAPlus
                        | ModuleInvocationFamilyV1::BindingSsaTrivial
                ) || receipt.brand() != brand
                    || inventory.brand() != brand
                    || inventory.family() != family
                {
                    return Err(ExternalCommitPreparationErrorV1::EvidenceMismatch);
                }
                Ok(Self::CanonicalSingle {
                    continuation,
                    receipt,
                    inventory,
                })
            }
            PostprocessEvidenceInputV1::CanonicalCallable {
                continuation,
                receipt,
                inventory,
                capability,
            } => {
                if !matches!(
                    family,
                    ModuleInvocationFamilyV1::BindingSsaAcyclic
                        | ModuleInvocationFamilyV1::BindingSsaRecursive
                ) || receipt.brand() != brand
                    || inventory.brand() != brand
                    || inventory.family() != family
                    || capability.brand() != brand
                    || capability.family() != family
                {
                    return Err(ExternalCommitPreparationErrorV1::EvidenceMismatch);
                }
                Ok(Self::CanonicalCallable {
                    continuation,
                    receipt,
                    inventory,
                    capability,
                })
            }
        }
    }
}

impl super::MirCompiler {
    pub(in crate::mir) fn prepare_module_external_commit<'a>(
        &mut self,
        postprocessed: PostprocessedModuleInvocationV1<'a>,
    ) -> Result<PreparedModuleExternalCommitV1<'a>, ExternalCommitPreparationErrorV1> {
        PreparedModuleExternalCommitV1::prepare(postprocessed)
    }

    pub(in crate::mir) fn commit_prepared_module<'a>(
        &mut self,
        prepared: PreparedModuleExternalCommitV1<'a>,
    ) -> MirCompileResult {
        prepared.commit(&mut self.builder)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_projection_reports_final_barrier_success() {
        let evidence = ModuleVerificationEvidenceV1::Canonical {
            pre_transform: Err(Vec::<VerificationError>::new().into_boxed_slice()),
            final_verified: CanonicalFinalVerificationSealV1 {
                _seal: CanonicalFinalVerificationSealInnerV1,
            },
        };

        assert_eq!(project_canonical_verification_result(evidence), Ok(()));
    }
}
