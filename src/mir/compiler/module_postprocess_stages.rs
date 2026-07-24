//! Shared postprocess stage order for canonical and Raw module targets.
//!
//! Targets expose named operations rather than a mutable `MirModule` loan.
//! This keeps the stage authority in one place while allowing the new Raw
//! finalized carrier to remain opaque.

use super::module_postprocess::{
    CanonicalFinalVerificationSealInnerV1, CanonicalFinalVerificationSealV1,
    ModulePostprocessErrorV1, ModulePostprocessScheduleV1, ModuleVerificationEvidenceV1,
    PostprocessFailureStageV1, RcInsertionScheduleV1, VerificationBarrierV1,
};
use crate::mir::optimizer_stats::OptimizationStats;
use crate::mir::verification::MirVerifier;
use crate::mir::verification_types::VerificationError;

pub(in crate::mir) trait PostprocessStageTarget {
    fn refresh_rune_plans(&mut self);
    fn optimize(&mut self) -> OptimizationStats;
    fn refresh_contracts(&mut self) -> Result<(), String>;
    fn verify(&mut self, verifier: &mut MirVerifier) -> Result<(), Box<[VerificationError]>>;
    fn insert_rc(&mut self);
    fn refresh_semantic_metadata(&mut self);
    fn canonicalize_callsites(&mut self) -> usize;
}

pub(in crate::mir) struct PostprocessStageFailureV1 {
    pub(in crate::mir) stage: PostprocessFailureStageV1,
    pub(in crate::mir) error: ModulePostprocessErrorV1,
}

pub(in crate::mir) fn run_postprocess_stages<T: PostprocessStageTarget>(
    target: &mut T,
    schedule: ModulePostprocessScheduleV1,
    verifier: &mut MirVerifier,
    optimize: bool,
) -> Result<ModuleVerificationEvidenceV1, PostprocessStageFailureV1> {
    target.refresh_rune_plans();
    if optimize {
        let stats = target.optimize();
        if (crate::config::env::opt_diag_fail() || crate::config::env::opt_diag_forbid_legacy())
            && stats.diagnostics_reported > 0
        {
            return Err(PostprocessStageFailureV1 {
                stage: PostprocessFailureStageV1::Optimizer,
                error: ModulePostprocessErrorV1::OptimizerDiagnostics {
                    count: stats.diagnostics_reported,
                },
            });
        }
    }
    if let Err(error) = target.refresh_contracts() {
        return Err(PostprocessStageFailureV1 {
            stage: PostprocessFailureStageV1::ContractRefresh,
            error: ModulePostprocessErrorV1::ContractRefresh(error),
        });
    }
    let pre_transform = target
        .verify(verifier)
        .map(|()| ())
        .map_err(|errors| errors);
    if schedule.rc() == RcInsertionScheduleV1::Run {
        target.insert_rc();
    }
    target.refresh_semantic_metadata();
    let changed = target.canonicalize_callsites();
    if changed > 0 {
        target.refresh_semantic_metadata();
    }
    match schedule.verifier() {
        VerificationBarrierV1::ReportPreTransformOnly => {
            Ok(ModuleVerificationEvidenceV1::Raw { pre_transform })
        }
        VerificationBarrierV1::RequireFinal => {
            if let Err(errors) = target.verify(verifier) {
                return Err(PostprocessStageFailureV1 {
                    stage: PostprocessFailureStageV1::FinalVerification,
                    error: ModulePostprocessErrorV1::FinalVerification(errors),
                });
            }
            Ok(ModuleVerificationEvidenceV1::Canonical {
                pre_transform,
                final_verified: CanonicalFinalVerificationSealV1 {
                    _seal: CanonicalFinalVerificationSealInnerV1,
                },
            })
        }
    }
}
