//! CallableModule dispatch through its already-sealed normal transaction.
//!
//! This module owns the only callable-family sequence. It never retries as
//! Script/Main, reconstructs a source plan, or publishes/executes a module.

use crate::mir::builder::{
    RejectedNormalCallableBatchV1, RejectedNormalCallableCommitV1,
    RejectedNormalCallableMainPhysicalV1, RejectedNormalHelperDraftPrefixV1,
};
use crate::mir::compiler::normal_source_plan::{
    NormalMainDirectCallPreflightV1, RejectedNormalCallableCatalogSourceV1,
    RejectedNormalCallableSourceV1, RejectedNormalMainDirectCallPlanV1,
    RejectedNormalMainDirectCallSourceV1, RejectedNormalMainHelperResolutionV1,
    SealedNormalCallableModuleSourceV1,
};

use super::{
    CanonicalCallableDispatchStageV1,
    CompletedCanonicalCoreSourceEntryCandidateSealV1, CompletedCanonicalCoreSourceEntryCandidateV1,
    CompletedCanonicalCoreSourceEntryFamilyV1, NormalSourcePlanReceiptV1,
    VerifiedCanonicalCoreSourcePlanAdmissionV1,
};
use super::super::MirCompiler;

/// The fixed no-import profile owns one source file, hence one catalog unit.
const CANONICAL_CORE_SINGLE_FILE_UNIT_ORDINAL: u32 = 0;

#[derive(Debug)]
pub(super) enum RejectedCanonicalCallableDispatchV1 {
    Source(RejectedNormalCallableSourceV1),
    Catalog(RejectedNormalCallableCatalogSourceV1),
    MainCatalog(RejectedNormalMainDirectCallSourceV1),
    MainPlan(RejectedNormalMainDirectCallPlanV1),
    HelperResolution(RejectedNormalMainHelperResolutionV1),
    HelperDraft(RejectedNormalHelperDraftPrefixV1),
    MainPhysical(RejectedNormalCallableMainPhysicalV1),
    Batch(RejectedNormalCallableBatchV1),
    Commit(RejectedNormalCallableCommitV1),
}

impl RejectedCanonicalCallableDispatchV1 {
    pub(super) const fn stage(&self) -> CanonicalCallableDispatchStageV1 {
        match self {
            Self::Source(_) => CanonicalCallableDispatchStageV1::Source,
            Self::Catalog(_) => CanonicalCallableDispatchStageV1::Catalog,
            Self::MainCatalog(_) => CanonicalCallableDispatchStageV1::MainCatalog,
            Self::MainPlan(_) => CanonicalCallableDispatchStageV1::MainPlan,
            Self::HelperResolution(_) => CanonicalCallableDispatchStageV1::HelperResolution,
            Self::HelperDraft(_) => CanonicalCallableDispatchStageV1::HelperDraft,
            Self::MainPhysical(_) => CanonicalCallableDispatchStageV1::MainPhysical,
            Self::Batch(_) => CanonicalCallableDispatchStageV1::Batch,
            Self::Commit(_) => CanonicalCallableDispatchStageV1::Commit,
        }
    }
}

#[derive(Debug)]
pub(super) struct RejectedCanonicalCallableDispatchWithContextV1 {
    rejected: RejectedCanonicalCallableDispatchV1,
    admission: VerifiedCanonicalCoreSourcePlanAdmissionV1,
    receipt: NormalSourcePlanReceiptV1,
}

#[derive(Debug)]
struct OpenCanonicalCallableDispatchContextV1 {
    admission: VerifiedCanonicalCoreSourcePlanAdmissionV1,
    receipt: NormalSourcePlanReceiptV1,
}

impl OpenCanonicalCallableDispatchContextV1 {
    fn reject(
        self,
        rejected: RejectedCanonicalCallableDispatchV1,
    ) -> RejectedCanonicalCallableDispatchWithContextV1 {
        RejectedCanonicalCallableDispatchWithContextV1 {
            rejected,
            admission: self.admission,
            receipt: self.receipt,
        }
    }

    fn complete(
        self,
        candidate: crate::mir::builder::CompletedNormalCallableCandidateV1,
    ) -> CompletedCanonicalCoreSourceEntryCandidateV1 {
        CompletedCanonicalCoreSourceEntryCandidateV1 {
            family: CompletedCanonicalCoreSourceEntryFamilyV1::Callable(candidate),
            admission: self.admission,
            receipt: self.receipt,
            _seal: CompletedCanonicalCoreSourceEntryCandidateSealV1,
        }
    }
}

impl RejectedCanonicalCallableDispatchWithContextV1 {
    pub(super) const fn stage(&self) -> CanonicalCallableDispatchStageV1 {
        self.rejected.stage()
    }

    pub(super) fn into_parts(
        self,
    ) -> (
        RejectedCanonicalCallableDispatchV1,
        VerifiedCanonicalCoreSourcePlanAdmissionV1,
        NormalSourcePlanReceiptV1,
    ) {
        (self.rejected, self.admission, self.receipt)
    }
}

pub(super) fn compile(
    compiler: &mut MirCompiler,
    source: SealedNormalCallableModuleSourceV1,
    admission: VerifiedCanonicalCoreSourcePlanAdmissionV1,
    receipt: NormalSourcePlanReceiptV1,
) -> Result<CompletedCanonicalCoreSourceEntryCandidateV1, RejectedCanonicalCallableDispatchWithContextV1>
{
    let context = OpenCanonicalCallableDispatchContextV1 { admission, receipt };
    let callable = match source.prepare_callable_source() {
        Ok(callable) => callable,
        Err(rejected) => return Err(context.reject(RejectedCanonicalCallableDispatchV1::Source(rejected))),
    };
    let catalog = match callable.prepare_helper_catalog(CANONICAL_CORE_SINGLE_FILE_UNIT_ORDINAL) {
        Ok(catalog) => catalog,
        Err(rejected) => return Err(context.reject(RejectedCanonicalCallableDispatchV1::Catalog(rejected))),
    };
    let main = match catalog.prepare_main_with_helper_catalog() {
        Ok(main) => main,
        Err(rejected) => return Err(context.reject(RejectedCanonicalCallableDispatchV1::MainCatalog(rejected))),
    };
    let plan = match NormalMainDirectCallPreflightV1::seal(main) {
        Ok(plan) => plan,
        Err(rejected) => return Err(context.reject(RejectedCanonicalCallableDispatchV1::MainPlan(rejected))),
    };
    let resolved = match plan.prepare_helper_resolution().resolve() {
        Ok(resolved) => resolved,
        Err(rejected) => return Err(context.reject(RejectedCanonicalCallableDispatchV1::HelperResolution(rejected))),
    };
    let prefix = match compiler
        .builder
        .prepare_normal_helper_draft_prefix_v1(resolved.into_tx0_handoff())
    {
        Ok(prefix) => prefix,
        Err(rejected) => return Err(context.reject(RejectedCanonicalCallableDispatchV1::HelperDraft(rejected))),
    };
    let physical = match compiler
        .builder
        .prepare_normal_callable_main_physical_v1(prefix)
    {
        Ok(physical) => physical,
        Err(rejected) => return Err(context.reject(RejectedCanonicalCallableDispatchV1::MainPhysical(rejected))),
    };
    let batch = match physical.seal_normal_callable_batch_v1() {
        Ok(batch) => batch,
        Err(rejected) => return Err(context.reject(RejectedCanonicalCallableDispatchV1::Batch(rejected))),
    };
    let prepared = match batch.prepare_normal_callable_commit_v1() {
        Ok(prepared) => prepared,
        Err(rejected) => return Err(context.reject(RejectedCanonicalCallableDispatchV1::Commit(rejected))),
    };
    Ok(context.complete(prepared.commit()))
}
