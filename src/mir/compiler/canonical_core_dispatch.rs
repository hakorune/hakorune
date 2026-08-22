//! Compiler-owned one-shot dispatch for the canonical-core normal profile.
//!
//! The front door transfers an already classified source plan into this module
//! without matching its family. This module performs the sole family match and
//! returns an unpublished candidate; publication and VM execution remain later
//! boundaries.

mod callable;
#[path = "canonical_core_source_plan_request.rs"]
mod source_plan_request;

pub(crate) use source_plan_request::{
    CanonicalCoreSourcePlanCompileRequestV1, CanonicalCoreSourcePlanInputV1,
    CanonicalScriptSourcePlanInputV1, NormalSourcePlanReceiptV1,
    VerifiedCanonicalCoreSourcePlanAdmissionV1,
};
pub(in crate::mir) mod publication;

use crate::mir::builder::{
    CompletedNormalCallableCandidateV1, CompletedNormalMainModuleCandidateV1,
    NormalCanonicalModuleBatchErrorV1, NormalCanonicalModuleBatchV1,
    NormalMainModuleTransactionErrorV1, PreparedNormalScriptModuleTransactionV1,
};
use crate::mir::compiler::normal_source_plan::{
    NormalMainFunctionPlanErrorV1, NormalMainFunctionPreflightV1, NormalMainFunctionSourceErrorV1,
    NormalMainResolvedSourceErrorV1, NormalMainThunkPlanErrorV1, OpenScriptPhysicalEntryV1,
    RejectedNormalScriptPhysicalEntryV1, SealedNormalMainSourceV1, SealedNormalScalarRootV1,
    SealedNormalSourcePlanV1, VerifiedNormalMainThunkPlanV1,
};
use crate::mir::compiler::raw_root_source_facts::RawScriptRecipeProjectionErrorV1;
#[cfg(feature = "vm-reference")]
use crate::mir::compiler::source_entry_vm_invocation::PreparedVmReferenceSourceEntryInvocationV1;
#[cfg(feature = "vm-reference")]
use crate::mir::compiler::source_entry_vm_reference::{
    RawVmReferenceRunReportV1, VmReferenceProcessOutcomeV1,
};

use super::MirCompiler;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CanonicalCoreDispatchStageV1 {
    MainSource,
    MainResolution,
    MainFunction,
    MainThunk,
    MainBatch,
    MainCandidate,
    ScriptSourceEnvelope,
    ScriptRecipe,
    ScriptPhysical,
    ScriptCandidate,
    Callable,
}

/// Exact substage for the sole CallableModule dispatch sequence.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CanonicalCallableDispatchStageV1 {
    Source,
    Catalog,
    MainCatalog,
    MainPlan,
    HelperResolution,
    HelperDraft,
    MainPhysical,
    Batch,
    Commit,
}

#[derive(Debug)]
pub(crate) enum CanonicalCoreDispatchErrorV1 {
    MainSource(NormalMainFunctionSourceErrorV1),
    MainResolution(NormalMainResolvedSourceErrorV1),
    MainFunction(NormalMainFunctionPlanErrorV1),
    MainThunk(NormalMainThunkPlanErrorV1),
    MainBatch(NormalCanonicalModuleBatchErrorV1),
    MainCandidate(NormalMainModuleTransactionErrorV1),
    ScriptSourceEnvelope,
    ScriptRecipe(RawScriptRecipeProjectionErrorV1),
    ScriptPhysical,
    ScriptCandidate,
    Callable(CanonicalCallableDispatchStageV1),
}

/// The complete source/profile/receipt owner retained by a dispatch rejection.
#[derive(Debug)]
pub(crate) enum RetainedCanonicalCoreSourcePlanOwnerV1 {
    Plan {
        plan: SealedNormalSourcePlanV1,
        admission: VerifiedCanonicalCoreSourcePlanAdmissionV1,
        receipt: NormalSourcePlanReceiptV1,
    },
    ScriptPhysical {
        rejected: RejectedNormalScriptPhysicalEntryV1,
        admission: VerifiedCanonicalCoreSourcePlanAdmissionV1,
        receipt: NormalSourcePlanReceiptV1,
    },
    ScriptCandidate {
        rejected: crate::mir::builder::RejectedNormalScriptModuleTransactionV1,
        admission: VerifiedCanonicalCoreSourcePlanAdmissionV1,
        receipt: NormalSourcePlanReceiptV1,
    },
    Callable {
        rejected: callable::RejectedCanonicalCallableDispatchV1,
        admission: VerifiedCanonicalCoreSourcePlanAdmissionV1,
        receipt: NormalSourcePlanReceiptV1,
    },
}

#[derive(Debug)]
pub(crate) struct RejectedCanonicalCoreNormalDispatchV1 {
    owner: RetainedCanonicalCoreSourcePlanOwnerV1,
    stage: CanonicalCoreDispatchStageV1,
    cause: CanonicalCoreDispatchErrorV1,
}

impl RejectedCanonicalCoreNormalDispatchV1 {
    pub(crate) const fn stage(&self) -> CanonicalCoreDispatchStageV1 {
        self.stage
    }

    pub(crate) fn cause(&self) -> &CanonicalCoreDispatchErrorV1 {
        &self.cause
    }

    pub(crate) fn discard(self) {
        drop(self);
    }

    #[cfg(test)]
    pub(crate) fn receipt_counts(&self) -> (u8, u8) {
        match &self.owner {
            RetainedCanonicalCoreSourcePlanOwnerV1::Plan { receipt, .. }
            | RetainedCanonicalCoreSourcePlanOwnerV1::ScriptPhysical { receipt, .. }
            | RetainedCanonicalCoreSourcePlanOwnerV1::ScriptCandidate { receipt, .. } => {
                receipt.counts()
            }
            RetainedCanonicalCoreSourcePlanOwnerV1::Callable { receipt, .. } => receipt.counts(),
        }
    }
}

#[derive(Debug)]
pub(crate) enum CompletedCanonicalCoreSourceEntryFamilyV1 {
    Main(CompletedNormalMainModuleCandidateV1),
    Script(crate::mir::builder::CompletedNormalScriptModuleCandidateV1),
    Callable(CompletedNormalCallableCandidateV1),
}

/// A complete but unpublished source-entry candidate.
#[derive(Debug)]
pub(crate) struct CompletedCanonicalCoreSourceEntryCandidateV1 {
    family: CompletedCanonicalCoreSourceEntryFamilyV1,
    admission: VerifiedCanonicalCoreSourcePlanAdmissionV1,
    receipt: NormalSourcePlanReceiptV1,
    _seal: CompletedCanonicalCoreSourceEntryCandidateSealV1,
}

/// The sole canonical-core terminal that crosses from family dispatch into
/// shared publication. It never selects another family after rejection.
#[derive(Debug)]
pub(crate) enum RejectedCanonicalCorePublishedSourceEntryV1 {
    Dispatch(RejectedCanonicalCoreNormalDispatchV1),
    Publication(publication::RejectedCanonicalSourceEntryPublicationV1),
}

/// Bounded MIR-to-runner rejection report for the canonical-core lane.
///
/// This adapter consumes retained compiler owners without exposing them to the
/// runner. Executed program faults never use this report.
#[derive(Debug)]
pub(crate) struct CanonicalCoreInvocationFailureReportV1 {
    stage: CanonicalCoreInvocationFailureStageV1,
    code: &'static str,
    detail: Box<str>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CanonicalCoreInvocationFailureStageV1 {
    Dispatch,
    Publication,
}

impl CanonicalCoreInvocationFailureReportV1 {
    pub(crate) const fn stage(&self) -> CanonicalCoreInvocationFailureStageV1 {
        self.stage
    }

    pub(crate) const fn code(&self) -> &'static str {
        self.code
    }

    pub(crate) fn detail(&self) -> &str {
        &self.detail
    }
}

impl From<RejectedCanonicalCorePublishedSourceEntryV1> for CanonicalCoreInvocationFailureReportV1 {
    fn from(rejected: RejectedCanonicalCorePublishedSourceEntryV1) -> Self {
        match rejected {
            RejectedCanonicalCorePublishedSourceEntryV1::Dispatch(rejected) => {
                let detail = format!("stage={:?}", rejected.stage()).into_boxed_str();
                rejected.discard();
                Self {
                    stage: CanonicalCoreInvocationFailureStageV1::Dispatch,
                    code: "canonical-core-dispatch-rejected",
                    detail,
                }
            }
            RejectedCanonicalCorePublishedSourceEntryV1::Publication(rejected) => {
                let detail = format!("stage={:?}", rejected.stage()).into_boxed_str();
                rejected.discard();
                Self {
                    stage: CanonicalCoreInvocationFailureStageV1::Publication,
                    code: "canonical-core-publication-rejected",
                    detail,
                }
            }
        }
    }
}

impl RejectedCanonicalCorePublishedSourceEntryV1 {
    pub(crate) fn discard(self) {
        drop(self);
    }
}

/// The only owner allowed to select a canonical-core source family.
#[derive(Debug)]
pub(crate) struct NormalCanonicalCoreSourcePlanCompilerV1;

#[derive(Debug)]
struct CompletedCanonicalCoreSourceEntryCandidateSealV1;

impl CompletedCanonicalCoreSourceEntryCandidateV1 {
    #[cfg(test)]
    pub(crate) fn is_main(&self) -> bool {
        matches!(
            self.family,
            CompletedCanonicalCoreSourceEntryFamilyV1::Main(_)
        )
    }

    #[cfg(test)]
    pub(crate) fn is_script(&self) -> bool {
        matches!(
            self.family,
            CompletedCanonicalCoreSourceEntryFamilyV1::Script(_)
        )
    }

    #[cfg(test)]
    pub(crate) fn is_callable(&self) -> bool {
        matches!(
            self.family,
            CompletedCanonicalCoreSourceEntryFamilyV1::Callable(_)
        )
    }

    #[cfg(test)]
    pub(crate) fn receipt_counts(&self) -> (u8, u8) {
        self.receipt.counts()
    }

    #[cfg(test)]
    pub(crate) fn script_candidate_evidence_for_test(
        &self,
    ) -> Option<ScriptCandidateEvidenceTestV1> {
        match &self.family {
            CompletedCanonicalCoreSourceEntryFamilyV1::Script(candidate) => {
                let evidence = candidate.evidence();
                Some(ScriptCandidateEvidenceTestV1 {
                    target_is_main: evidence.target().is_main(),
                    target_symbol: evidence.target().symbol().to_owned(),
                    target_arity: evidence.target().arity(),
                    source_identity: evidence.source_identity().to_owned(),
                    schema_row_count: evidence.schema_row_count(),
                    result_kind: match evidence.result() {
                        crate::mir::builder::VerifiedScriptEntryResultContractV1::Unit {
                            ..
                        } => "unit",
                        crate::mir::builder::VerifiedScriptEntryResultContractV1::Integer => {
                            "integer"
                        }
                        crate::mir::builder::VerifiedScriptEntryResultContractV1::Bool => "bool",
                        crate::mir::builder::VerifiedScriptEntryResultContractV1::Float => "float",
                        crate::mir::builder::VerifiedScriptEntryResultContractV1::String => {
                            "string"
                        }
                    },
                    verification_function_count: candidate.verification().function_count(),
                    module_function_count: candidate.module().functions.len(),
                })
            }
            CompletedCanonicalCoreSourceEntryFamilyV1::Main(_)
            | CompletedCanonicalCoreSourceEntryFamilyV1::Callable(_) => None,
        }
    }
}

#[cfg(test)]
#[derive(Debug)]
pub(crate) struct ScriptCandidateEvidenceTestV1 {
    pub(crate) target_is_main: bool,
    pub(crate) target_symbol: String,
    pub(crate) target_arity: usize,
    pub(crate) source_identity: String,
    pub(crate) schema_row_count: usize,
    pub(crate) result_kind: &'static str,
    pub(crate) verification_function_count: usize,
    pub(crate) module_function_count: usize,
}

impl NormalCanonicalCoreSourcePlanCompilerV1 {
    /// The sole compiler-layer match over the sealed canonical-core family.
    pub(crate) fn consume(
        compiler: &mut MirCompiler,
        request: CanonicalCoreSourcePlanCompileRequestV1,
    ) -> Result<CompletedCanonicalCoreSourceEntryCandidateV1, RejectedCanonicalCoreNormalDispatchV1>
    {
        let (plan, admission, receipt, source_input) = request.into_parts();
        match plan {
            SealedNormalSourcePlanV1::ScalarRoot(SealedNormalScalarRootV1::Main0(main)) => {
                source_input.discard_before_a_consumer();
                Self::compile_main0(compiler, main, admission, receipt)
            }
            SealedNormalSourcePlanV1::ScalarRoot(SealedNormalScalarRootV1::Script(script)) => {
                Self::compile_script(compiler, script, admission, receipt, source_input)
            }
            SealedNormalSourcePlanV1::CallableModule(source) => {
                source_input.discard_before_a_consumer();
                callable::compile(compiler, source, admission, receipt).map_err(reject_callable)
            }
        }
    }

    fn compile_main0(
        compiler: &mut MirCompiler,
        main: SealedNormalMainSourceV1,
        admission: VerifiedCanonicalCoreSourcePlanAdmissionV1,
        receipt: NormalSourcePlanReceiptV1,
    ) -> Result<CompletedCanonicalCoreSourceEntryCandidateV1, RejectedCanonicalCoreNormalDispatchV1>
    {
        let source = match main.prepare_function_source() {
            Ok(source) => source,
            Err(rejected) => {
                let (main, error) = rejected.into_parts();
                return Err(reject_main(
                    main,
                    admission,
                    receipt,
                    CanonicalCoreDispatchStageV1::MainSource,
                    CanonicalCoreDispatchErrorV1::MainSource(error),
                ));
            }
        };
        let resolved = match source.prepare_embedded_resolved_main() {
            Ok(resolved) => resolved,
            Err(rejected) => {
                let (source, error) = rejected.into_parts();
                return Err(reject_main(
                    source.into_source(),
                    admission,
                    receipt,
                    CanonicalCoreDispatchStageV1::MainResolution,
                    CanonicalCoreDispatchErrorV1::MainResolution(error),
                ));
            }
        };
        let function = match NormalMainFunctionPreflightV1::seal(&resolved) {
            Ok(function) => function,
            Err(rejected) => {
                let error = rejected.into_error();
                return Err(reject_main(
                    restore_main_source(resolved),
                    admission,
                    receipt,
                    CanonicalCoreDispatchStageV1::MainFunction,
                    CanonicalCoreDispatchErrorV1::MainFunction(error),
                ));
            }
        };
        let thunk = match VerifiedNormalMainThunkPlanV1::seal(function) {
            Ok(thunk) => thunk,
            Err(rejected) => {
                let (function, error) = rejected.into_parts();
                drop(function);
                return Err(reject_main(
                    restore_main_source(resolved),
                    admission,
                    receipt,
                    CanonicalCoreDispatchStageV1::MainThunk,
                    CanonicalCoreDispatchErrorV1::MainThunk(error),
                ));
            }
        };
        let batch = match NormalCanonicalModuleBatchV1::prepare(thunk) {
            Ok(batch) => batch,
            Err(rejected) => {
                let (thunk, error) = rejected.into_parts();
                drop(thunk);
                return Err(reject_main(
                    restore_main_source(resolved),
                    admission,
                    receipt,
                    CanonicalCoreDispatchStageV1::MainBatch,
                    CanonicalCoreDispatchErrorV1::MainBatch(error),
                ));
            }
        };
        let transaction = match compiler
            .builder
            .prepare_normal_main_module_transaction(batch)
        {
            Ok(transaction) => transaction,
            Err(rejected) => {
                let error = rejected.into_error();
                return Err(reject_main(
                    restore_main_source(resolved),
                    admission,
                    receipt,
                    CanonicalCoreDispatchStageV1::MainCandidate,
                    CanonicalCoreDispatchErrorV1::MainCandidate(error),
                ));
            }
        };
        let main = transaction.commit();
        Ok(CompletedCanonicalCoreSourceEntryCandidateV1 {
            family: CompletedCanonicalCoreSourceEntryFamilyV1::Main(main),
            admission,
            receipt,
            _seal: CompletedCanonicalCoreSourceEntryCandidateSealV1,
        })
    }

    fn compile_script(
        compiler: &MirCompiler,
        script: crate::mir::compiler::normal_source_plan::SealedNormalScriptSourceV1,
        admission: VerifiedCanonicalCoreSourcePlanAdmissionV1,
        receipt: NormalSourcePlanReceiptV1,
        source_input: CanonicalCoreSourcePlanInputV1,
    ) -> Result<CompletedCanonicalCoreSourceEntryCandidateV1, RejectedCanonicalCoreNormalDispatchV1>
    {
        match source_input {
            CanonicalCoreSourcePlanInputV1::Script(
                CanonicalScriptSourcePlanInputV1::SourceEnvelopeReady(envelope),
            ) => {
                envelope.discard_before_a_consumer();
            }
            other => {
                other.discard_before_a_consumer();
                return Err(reject(
                    SealedNormalSourcePlanV1::ScalarRoot(SealedNormalScalarRootV1::Script(script)),
                    admission,
                    receipt,
                    CanonicalCoreDispatchStageV1::ScriptSourceEnvelope,
                    CanonicalCoreDispatchErrorV1::ScriptSourceEnvelope,
                ));
            }
        };
        let recipe = match script.prepare_script_recipe() {
            Ok(recipe) => recipe,
            Err(rejected) => {
                let (script, error) = rejected.into_parts();
                return Err(reject(
                    SealedNormalSourcePlanV1::ScalarRoot(SealedNormalScalarRootV1::Script(script)),
                    admission,
                    receipt,
                    CanonicalCoreDispatchStageV1::ScriptRecipe,
                    CanonicalCoreDispatchErrorV1::ScriptRecipe(error),
                ));
            }
        };
        let entry = match OpenScriptPhysicalEntryV1::open(compiler, recipe) {
            Ok(entry) => entry,
            Err(rejected) => return Err(reject_script_physical(rejected, admission, receipt)),
        };
        let exit = match entry.prepare() {
            Ok(exit) => exit,
            Err(rejected) => return Err(reject_script_physical(rejected, admission, receipt)),
        };
        let transaction = match PreparedNormalScriptModuleTransactionV1::prepare(exit) {
            Ok(transaction) => transaction,
            Err(rejected) => return Err(reject_script_candidate(rejected, admission, receipt)),
        };
        Ok(CompletedCanonicalCoreSourceEntryCandidateV1 {
            family: CompletedCanonicalCoreSourceEntryFamilyV1::Script(transaction.commit()),
            admission,
            receipt,
            _seal: CompletedCanonicalCoreSourceEntryCandidateSealV1,
        })
    }
}

impl MirCompiler {
    pub(crate) fn compile_canonical_core_source_plan(
        &mut self,
        request: CanonicalCoreSourcePlanCompileRequestV1,
    ) -> Result<CompletedCanonicalCoreSourceEntryCandidateV1, RejectedCanonicalCoreNormalDispatchV1>
    {
        NormalCanonicalCoreSourcePlanCompilerV1::consume(self, request)
    }

    /// Consume one sealed source plan through its sole family dispatch and
    /// the shared canonical publication core. VM execution remains later.
    pub(crate) fn compile_canonical_core_source_plan_to_published(
        &mut self,
        request: CanonicalCoreSourcePlanCompileRequestV1,
    ) -> Result<
        publication::PublishedCanonicalSourceEntryInvocationV1,
        RejectedCanonicalCorePublishedSourceEntryV1,
    > {
        let candidate = self
            .compile_canonical_core_source_plan(request)
            .map_err(RejectedCanonicalCorePublishedSourceEntryV1::Dispatch)?;
        candidate
            .prepare_canonical_publication()
            .map(|prepared| prepared.commit())
            .map_err(RejectedCanonicalCorePublishedSourceEntryV1::Publication)
    }

    /// The canonical-core VM-reference seam consumes the shared published
    /// invocation. It owns no entry selection or process-status policy.
    #[cfg(feature = "vm-reference")]
    pub(in crate::mir) fn run_canonical_core_source_plan_vm_reference(
        &mut self,
        request: CanonicalCoreSourcePlanCompileRequestV1,
    ) -> Result<VmReferenceProcessOutcomeV1, RejectedCanonicalCorePublishedSourceEntryV1> {
        let prepared: PreparedVmReferenceSourceEntryInvocationV1<_> = self
            .compile_canonical_core_source_plan_to_published(request)?
            .prepare_vm_reference();
        Ok(prepared.execute().complete_canonical_source_entry())
    }

    /// Runner-facing terminal for one already classified canonical-core plan.
    /// It preserves the canonical process outcome and bounds only pre-execution
    /// compiler rejection into a neutral report.
    #[cfg(feature = "vm-reference")]
    pub(crate) fn run_canonical_core_source_plan_for_runner_v1(
        &mut self,
        request: CanonicalCoreSourcePlanCompileRequestV1,
    ) -> Result<RawVmReferenceRunReportV1, CanonicalCoreInvocationFailureReportV1> {
        self.run_canonical_core_source_plan_vm_reference(request)
            .map(|outcome| outcome.into_run_report())
            .map_err(CanonicalCoreInvocationFailureReportV1::from)
    }

    #[cfg(test)]
    pub(crate) fn compile_canonical_core_source_plan_publication_summary_for_test(
        &mut self,
        request: CanonicalCoreSourcePlanCompileRequestV1,
    ) -> Result<
        publication::CanonicalPublicationSummaryForTestV1,
        RejectedCanonicalCorePublishedSourceEntryV1,
    > {
        self.compile_canonical_core_source_plan_to_published(request)
            .map(|published| published.publication_summary_for_test())
    }

    #[cfg(all(test, feature = "vm-reference"))]
    pub(crate) fn run_canonical_core_source_plan_vm_reference_summary_for_test(
        &mut self,
        request: CanonicalCoreSourcePlanCompileRequestV1,
    ) -> Result<CanonicalCoreVmReferenceSummaryForTestV1, RejectedCanonicalCorePublishedSourceEntryV1>
    {
        let outcome = self.run_canonical_core_source_plan_vm_reference(request)?;
        Ok(CanonicalCoreVmReferenceSummaryForTestV1 {
            status: outcome.status().value(),
            fault_tag: outcome.fault().map(|fault| match fault {
                crate::mir::compiler::source_entry_result::ProcessFaultV1::ExitCodeOutOfRange { .. } => "exit-code-out-of-range",
                crate::mir::compiler::source_entry_result::ProcessFaultV1::UnsupportedProcessResult { .. } => "unsupported-result",
                crate::mir::compiler::source_entry_result::ProcessFaultV1::SourceFault { code, .. } => code,
            }),
            route: match outcome.route_for_test() {
                crate::mir::compiler::source_entry_selection::SelectedSourceEntryRouteV1::Script => "script",
                crate::mir::compiler::source_entry_selection::SelectedSourceEntryRouteV1::AppMain0 => "main",
            },
        })
    }
}

#[cfg(all(test, feature = "vm-reference"))]
#[derive(Debug)]
pub(crate) struct CanonicalCoreVmReferenceSummaryForTestV1 {
    pub(crate) status: u8,
    pub(crate) fault_tag: Option<&'static str>,
    pub(crate) route: &'static str,
}

fn restore_main_source(
    resolved: crate::mir::compiler::normal_source_plan::VerifiedNormalMainResolvedSourceUnitV1,
) -> SealedNormalMainSourceV1 {
    resolved.into_source().into_source()
}

fn reject_main(
    main: SealedNormalMainSourceV1,
    admission: VerifiedCanonicalCoreSourcePlanAdmissionV1,
    receipt: NormalSourcePlanReceiptV1,
    stage: CanonicalCoreDispatchStageV1,
    cause: CanonicalCoreDispatchErrorV1,
) -> RejectedCanonicalCoreNormalDispatchV1 {
    reject(
        SealedNormalSourcePlanV1::ScalarRoot(SealedNormalScalarRootV1::Main0(main)),
        admission,
        receipt,
        stage,
        cause,
    )
}

fn reject(
    plan: SealedNormalSourcePlanV1,
    admission: VerifiedCanonicalCoreSourcePlanAdmissionV1,
    receipt: NormalSourcePlanReceiptV1,
    stage: CanonicalCoreDispatchStageV1,
    cause: CanonicalCoreDispatchErrorV1,
) -> RejectedCanonicalCoreNormalDispatchV1 {
    RejectedCanonicalCoreNormalDispatchV1 {
        owner: RetainedCanonicalCoreSourcePlanOwnerV1::Plan {
            plan,
            admission,
            receipt,
        },
        stage,
        cause,
    }
}

fn reject_script_physical(
    rejected: RejectedNormalScriptPhysicalEntryV1,
    admission: VerifiedCanonicalCoreSourcePlanAdmissionV1,
    receipt: NormalSourcePlanReceiptV1,
) -> RejectedCanonicalCoreNormalDispatchV1 {
    RejectedCanonicalCoreNormalDispatchV1 {
        owner: RetainedCanonicalCoreSourcePlanOwnerV1::ScriptPhysical {
            rejected,
            admission,
            receipt,
        },
        stage: CanonicalCoreDispatchStageV1::ScriptPhysical,
        cause: CanonicalCoreDispatchErrorV1::ScriptPhysical,
    }
}

fn reject_script_candidate(
    rejected: crate::mir::builder::RejectedNormalScriptModuleTransactionV1,
    admission: VerifiedCanonicalCoreSourcePlanAdmissionV1,
    receipt: NormalSourcePlanReceiptV1,
) -> RejectedCanonicalCoreNormalDispatchV1 {
    RejectedCanonicalCoreNormalDispatchV1 {
        owner: RetainedCanonicalCoreSourcePlanOwnerV1::ScriptCandidate {
            rejected,
            admission,
            receipt,
        },
        stage: CanonicalCoreDispatchStageV1::ScriptCandidate,
        cause: CanonicalCoreDispatchErrorV1::ScriptCandidate,
    }
}

fn reject_callable(
    rejected: callable::RejectedCanonicalCallableDispatchWithContextV1,
) -> RejectedCanonicalCoreNormalDispatchV1 {
    let callable_stage = rejected.stage();
    let (rejected, admission, receipt) = rejected.into_parts();
    RejectedCanonicalCoreNormalDispatchV1 {
        owner: RetainedCanonicalCoreSourcePlanOwnerV1::Callable {
            rejected,
            admission,
            receipt,
        },
        stage: CanonicalCoreDispatchStageV1::Callable,
        cause: CanonicalCoreDispatchErrorV1::Callable(callable_stage),
    }
}
