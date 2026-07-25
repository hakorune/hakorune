//! CUT0-I0-POST0: disconnected, family-owned module postprocessing.
//!
//! The existing public finish path remains unchanged until atomic CUT0.  This
//! owner records the same stage order for finalized invocation products and
//! keeps the RC/verifier policy derived from the invocation family.

use super::canonical_finalization::{CanonicalFinalizationInputV1, FinalizedModuleInvocationV1};
use super::module_postprocess_stages::{run_postprocess_stages, PostprocessStageTarget};
use super::source_bound_package::CanonicalSourceContinuationV1;
use crate::mir::builder::PreparedBuilderExternalCommitV1;
use crate::mir::builder::{
    CanonicalCallableCapabilityWitnessV1, CanonicalDrainedCallablePhysicalV1,
    CanonicalDrainedSinglePhysicalV1, CommitCallableCollectorBatchReceiptV1,
    CommitCollectedDraftAdmissionReceiptV1, InvocationBranded,
};
use crate::mir::canonical_physical_drain::CanonicalPhysicalDrainManifestV1;
use crate::mir::function::MirModule;
use crate::mir::module_invocation_identity::{
    ModuleInvocationBrandV1, ModuleInvocationFamilyV1, ModuleInvocationTokenV1,
};
use crate::mir::verification::MirVerifier;
use crate::mir::verification_types::VerificationError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum RcInsertionScheduleV1 {
    Run,
    Skip,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum VerificationBarrierV1 {
    ReportPreTransformOnly,
    RequireFinal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) struct ModulePostprocessScheduleV1 {
    rc: RcInsertionScheduleV1,
    verifier: VerificationBarrierV1,
}

impl ModulePostprocessScheduleV1 {
    pub(in crate::mir) const fn for_family(family: ModuleInvocationFamilyV1) -> Self {
        match family {
            ModuleInvocationFamilyV1::Raw => Self {
                rc: RcInsertionScheduleV1::Run,
                verifier: VerificationBarrierV1::ReportPreTransformOnly,
            },
            ModuleInvocationFamilyV1::CanonicalAPlus => Self {
                rc: RcInsertionScheduleV1::Run,
                verifier: VerificationBarrierV1::RequireFinal,
            },
            ModuleInvocationFamilyV1::BindingSsaTrivial
            | ModuleInvocationFamilyV1::BindingSsaAcyclic
            | ModuleInvocationFamilyV1::BindingSsaRecursive => Self {
                rc: RcInsertionScheduleV1::Skip,
                verifier: VerificationBarrierV1::RequireFinal,
            },
        }
    }

    pub(in crate::mir) const fn rc(self) -> RcInsertionScheduleV1 {
        self.rc
    }

    pub(in crate::mir) const fn verifier(self) -> VerificationBarrierV1 {
        self.verifier
    }
}

#[derive(Debug)]
pub(in crate::mir) enum ModuleVerificationEvidenceV1 {
    Canonical {
        pre_transform: Result<(), Box<[VerificationError]>>,
        final_verified: CanonicalFinalVerificationSealV1,
    },
    Raw {
        pre_transform: Result<(), Box<[VerificationError]>>,
    },
}

#[derive(Debug)]
pub(in crate::mir) struct CanonicalFinalVerificationSealV1 {
    pub(in crate::mir) _seal: CanonicalFinalVerificationSealInnerV1,
}

#[derive(Debug)]
pub(in crate::mir) struct CanonicalFinalVerificationSealInnerV1;

#[derive(Debug)]
pub(in crate::mir) enum ModulePostprocessErrorV1 {
    OptimizerDiagnostics { count: usize },
    ContractRefresh(String),
    FinalVerification(Box<[VerificationError]>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum PostprocessFailureStageV1 {
    Optimizer,
    ContractRefresh,
    FinalVerification,
}

#[derive(Debug)]
pub(in crate::mir) struct PostprocessedModuleInvocationV1<'a> {
    pub(in crate::mir) input: ModulePostprocessInputV1<'a>,
    pub(in crate::mir) schedule: ModulePostprocessScheduleV1,
    pub(in crate::mir) verification: ModuleVerificationEvidenceV1,
}

#[derive(Debug)]
pub(in crate::mir) enum ModulePostprocessInputV1<'a> {
    Canonical(CanonicalFinalizationInputV1<'a>),
}

impl PostprocessStageTarget for MirModule {
    fn refresh_rune_plans(&mut self) {
        crate::mir::rune_plan_refresh::refresh_module_rune_plans(self);
    }

    fn optimize(&mut self) -> crate::mir::optimizer_stats::OptimizationStats {
        crate::mir::optimizer::MirOptimizer::new().optimize_module(self)
    }

    fn refresh_contracts(&mut self) -> Result<(), String> {
        crate::mir::semantic_refresh::refresh_and_validate_for_boundary(
            self,
            crate::mir::semantic_refresh::ContractRefreshBoundary::Verifier,
        )
        .map(|_| ())
    }

    fn verify(&mut self, verifier: &mut MirVerifier) -> Result<(), Box<[VerificationError]>> {
        verifier
            .verify_module(self)
            .map_err(|errors| errors.into_boxed_slice())
    }

    fn insert_rc(&mut self) {
        crate::mir::passes::rc_insertion::insert_rc_instructions(self);
    }

    fn refresh_semantic_metadata(&mut self) {
        crate::mir::semantic_refresh::refresh_module_semantic_metadata(self);
    }

    fn canonicalize_callsites(&mut self) -> usize {
        crate::mir::passes::callsite_canonicalize::canonicalize_for_site(
            self,
            crate::mir::passes::callsite_canonicalize::CallsiteCanonicalizeScheduleSite::MirCompilerPostRc,
        )
    }
}

/// Route evidence extracted exactly once when the postprocessed owner is
/// handed to paired external commit preparation.  It is not a lookup source;
/// it only preserves the source/receipt/physical correspondence until the
/// one-shot commit consumes it.
#[derive(Debug)]
pub(in crate::mir) enum PostprocessEvidenceInputV1<'a> {
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

/// Rejected postprocess keeps the unpublished invocation at the exact stage
/// where it failed.  The owner is intentionally discard-only: no retry,
/// resume, replacement manifest, or fallback terminal is exposed.
#[derive(Debug)]
pub(in crate::mir) struct RejectedModulePostprocessV1<'a> {
    input: ModulePostprocessInputV1<'a>,
    schedule: ModulePostprocessScheduleV1,
    stage: PostprocessFailureStageV1,
    error: ModulePostprocessErrorV1,
}

impl<'a> RejectedModulePostprocessV1<'a> {
    pub(in crate::mir) fn stage(&self) -> PostprocessFailureStageV1 {
        self.stage
    }

    pub(in crate::mir) fn error(&self) -> &ModulePostprocessErrorV1 {
        &self.error
    }

    pub(in crate::mir) fn discard(self) {
        let Self {
            input,
            schedule: _,
            stage: _,
            error: _,
        } = self;
        drop(input);
    }
}

impl<'a> PostprocessedModuleInvocationV1<'a> {
    pub(in crate::mir) const fn brand(&self) -> ModuleInvocationBrandV1 {
        match &self.input {
            ModulePostprocessInputV1::Canonical(CanonicalFinalizationInputV1::Single(input)) => {
                input.token.brand()
            }
            ModulePostprocessInputV1::Canonical(CanonicalFinalizationInputV1::Callable(input)) => {
                input.token.brand()
            }
        }
    }

    pub(in crate::mir) const fn family(&self) -> ModuleInvocationFamilyV1 {
        match &self.input {
            ModulePostprocessInputV1::Canonical(CanonicalFinalizationInputV1::Single(input)) => {
                input.token.family()
            }
            ModulePostprocessInputV1::Canonical(CanonicalFinalizationInputV1::Callable(input)) => {
                input.token.family()
            }
        }
    }

    pub(in crate::mir) fn module(&self) -> &MirModule {
        match &self.input {
            ModulePostprocessInputV1::Canonical(CanonicalFinalizationInputV1::Single(input)) => {
                &input.physical.module
            }
            ModulePostprocessInputV1::Canonical(CanonicalFinalizationInputV1::Callable(input)) => {
                &input.physical.module
            }
        }
    }

    pub(in crate::mir) fn into_external_commit_parts(
        self,
    ) -> (
        ModuleInvocationTokenV1,
        PreparedBuilderExternalCommitV1,
        MirModule,
        ModuleVerificationEvidenceV1,
        PostprocessEvidenceInputV1<'a>,
    ) {
        let Self {
            input,
            verification,
            ..
        } = self;
        match input {
            ModulePostprocessInputV1::Canonical(CanonicalFinalizationInputV1::Single(input)) => {
                let builder = input.builder.into_external_commit();
                let CanonicalDrainedSinglePhysicalV1 {
                    module,
                    receipt,
                    inventory,
                    brand: _,
                    family: _,
                } = input.physical;
                (
                    input.token,
                    builder,
                    module,
                    verification,
                    PostprocessEvidenceInputV1::CanonicalSingle {
                        continuation: input.continuation,
                        receipt,
                        inventory,
                    },
                )
            }
            ModulePostprocessInputV1::Canonical(CanonicalFinalizationInputV1::Callable(input)) => {
                let builder = input.builder.into_external_commit();
                let CanonicalDrainedCallablePhysicalV1 {
                    module,
                    receipt,
                    inventory,
                    brand: _,
                    family: _,
                } = input.physical;
                (
                    input.token,
                    builder,
                    module,
                    verification,
                    PostprocessEvidenceInputV1::CanonicalCallable {
                        continuation: input.continuation,
                        receipt,
                        inventory,
                        capability: input.capability,
                    },
                )
            }
        }
    }
}

pub(in crate::mir) struct ModulePostprocessOwnerV1<'a> {
    verifier: &'a mut MirVerifier,
    optimize: bool,
}

impl<'a> ModulePostprocessOwnerV1<'a> {
    pub(in crate::mir) fn new(verifier: &'a mut MirVerifier, optimize: bool) -> Self {
        Self { verifier, optimize }
    }

    pub(in crate::mir) fn into_stage_parts(self) -> (&'a mut MirVerifier, bool) {
        (self.verifier, self.optimize)
    }

    pub(in crate::mir) fn run(
        self,
        finalized: FinalizedModuleInvocationV1<'a>,
    ) -> Result<PostprocessedModuleInvocationV1<'a>, RejectedModulePostprocessV1<'a>> {
        let FinalizedModuleInvocationV1 { input, .. } = finalized;
        let family = match &input {
            CanonicalFinalizationInputV1::Single(input) => input.token.family(),
            CanonicalFinalizationInputV1::Callable(input) => input.token.family(),
        };
        let schedule = ModulePostprocessScheduleV1::for_family(family);
        process_input(
            ModulePostprocessInputV1::Canonical(input),
            schedule,
            self.verifier,
            self.optimize,
        )
    }
}

fn process_input<'a>(
    input: ModulePostprocessInputV1<'a>,
    schedule: ModulePostprocessScheduleV1,
    verifier: &mut MirVerifier,
    optimize: bool,
) -> Result<PostprocessedModuleInvocationV1<'a>, RejectedModulePostprocessV1<'a>> {
    let mut input = input;
    let verification = match &mut input {
        ModulePostprocessInputV1::Canonical(CanonicalFinalizationInputV1::Single(input)) => {
            run_postprocess_stages(&mut input.physical.module, schedule, verifier, optimize)
        }
        ModulePostprocessInputV1::Canonical(CanonicalFinalizationInputV1::Callable(input)) => {
            run_postprocess_stages(&mut input.physical.module, schedule, verifier, optimize)
        }
    };
    match verification {
        Ok(verification) => Ok(PostprocessedModuleInvocationV1 {
            input,
            schedule,
            verification,
        }),
        Err(failure) => Err(rejected(input, schedule, failure.stage, failure.error)),
    }
}

fn rejected<'a>(
    input: ModulePostprocessInputV1<'a>,
    schedule: ModulePostprocessScheduleV1,
    stage: PostprocessFailureStageV1,
    error: ModulePostprocessErrorV1,
) -> RejectedModulePostprocessV1<'a> {
    RejectedModulePostprocessV1 {
        input,
        schedule,
        stage,
        error,
    }
}
