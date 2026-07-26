//! Compiler-owned one-shot dispatch for the canonical-core normal profile.
//!
//! The front door transfers an already classified source plan into this module
//! without matching its family. This module performs the sole family match and
//! returns an unpublished candidate; publication and VM execution remain later
//! boundaries.

use crate::mir::builder::{
    CompletedNormalMainModuleCandidateV1, NormalCanonicalModuleBatchErrorV1,
    NormalCanonicalModuleBatchV1, NormalMainModuleTransactionErrorV1,
    PreparedNormalScriptModuleTransactionV1,
};
use crate::mir::compiler::normal_source_plan::{
    NormalMainFunctionPlanErrorV1, NormalMainFunctionPreflightV1, NormalMainFunctionSourceErrorV1,
    NormalMainResolvedSourceErrorV1, NormalMainThunkPlanErrorV1, SealedNormalMainSourceV1,
    OpenScriptPhysicalEntryV1, RejectedNormalScriptPhysicalEntryV1, SealedNormalScalarRootV1,
    SealedNormalSourcePlanV1, VerifiedNormalMainThunkPlanV1,
};
use crate::mir::compiler::raw_root_source_facts::RawScriptRecipeProjectionErrorV1;

use super::MirCompiler;

/// A compiler-neutral receipt for the front door's one read and one parse.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct NormalSourcePlanReceiptV1 {
    source_identity: Box<str>,
    utf8_len: usize,
    read_count: u8,
    parse_count: u8,
    _seal: NormalSourcePlanReceiptSealV1,
}

#[derive(Debug, PartialEq, Eq)]
struct NormalSourcePlanReceiptSealV1;

impl NormalSourcePlanReceiptV1 {
    pub(crate) fn one_read_one_parse(
        source_identity: Box<str>,
        utf8_len: usize,
        read_count: u8,
        parse_count: u8,
    ) -> Self {
        Self {
            source_identity,
            utf8_len,
            read_count,
            parse_count,
            _seal: NormalSourcePlanReceiptSealV1,
        }
    }

    #[cfg(test)]
    pub(crate) fn counts(&self) -> (u8, u8) {
        (self.read_count, self.parse_count)
    }
}

/// Evidence that the front door selected the fixed canonical-core profile.
#[derive(Debug)]
pub(crate) struct VerifiedCanonicalCoreSourcePlanAdmissionV1 {
    _seal: VerifiedCanonicalCoreSourcePlanAdmissionSealV1,
}

#[derive(Debug)]
struct VerifiedCanonicalCoreSourcePlanAdmissionSealV1;

impl VerifiedCanonicalCoreSourcePlanAdmissionV1 {
    pub(crate) fn seal_from_frontdoor_profile() -> Self {
        Self {
            _seal: VerifiedCanonicalCoreSourcePlanAdmissionSealV1,
        }
    }
}

/// The compiler-neutral consuming input emitted by the normal-file front door.
#[derive(Debug)]
pub(crate) struct CanonicalCoreSourcePlanCompileRequestV1 {
    plan: SealedNormalSourcePlanV1,
    admission: VerifiedCanonicalCoreSourcePlanAdmissionV1,
    receipt: NormalSourcePlanReceiptV1,
    _seal: CanonicalCoreSourcePlanCompileRequestSealV1,
}

#[derive(Debug)]
struct CanonicalCoreSourcePlanCompileRequestSealV1;

impl CanonicalCoreSourcePlanCompileRequestV1 {
    pub(crate) fn new(
        plan: SealedNormalSourcePlanV1,
        admission: VerifiedCanonicalCoreSourcePlanAdmissionV1,
        receipt: NormalSourcePlanReceiptV1,
    ) -> Self {
        Self {
            plan,
            admission,
            receipt,
            _seal: CanonicalCoreSourcePlanCompileRequestSealV1,
        }
    }

    fn into_parts(
        self,
    ) -> (
        SealedNormalSourcePlanV1,
        VerifiedCanonicalCoreSourcePlanAdmissionV1,
        NormalSourcePlanReceiptV1,
    ) {
        (self.plan, self.admission, self.receipt)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CanonicalCoreDispatchStageV1 {
    FamilyCapability,
    MainSource,
    MainResolution,
    MainFunction,
    MainThunk,
    MainBatch,
    MainCandidate,
    ScriptRecipe,
    ScriptPhysical,
    ScriptCandidate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CanonicalCorePendingFamilyV1 {
    Script,
    CallableModule,
}

#[derive(Debug)]
pub(crate) enum CanonicalCoreDispatchErrorV1 {
    FamilyCapabilityPending(CanonicalCorePendingFamilyV1),
    MainSource(NormalMainFunctionSourceErrorV1),
    MainResolution(NormalMainResolvedSourceErrorV1),
    MainFunction(NormalMainFunctionPlanErrorV1),
    MainThunk(NormalMainThunkPlanErrorV1),
    MainBatch(NormalCanonicalModuleBatchErrorV1),
    MainCandidate(NormalMainModuleTransactionErrorV1),
    ScriptRecipe(RawScriptRecipeProjectionErrorV1),
    ScriptPhysical,
    ScriptCandidate,
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
        }
    }
}

#[derive(Debug)]
pub(crate) enum CompletedCanonicalCoreSourceEntryFamilyV1 {
    Main(CompletedNormalMainModuleCandidateV1),
    Script(crate::mir::builder::CompletedNormalScriptModuleCandidateV1),
}

/// A complete but unpublished source-entry candidate.
#[derive(Debug)]
pub(crate) struct CompletedCanonicalCoreSourceEntryCandidateV1 {
    family: CompletedCanonicalCoreSourceEntryFamilyV1,
    admission: VerifiedCanonicalCoreSourcePlanAdmissionV1,
    receipt: NormalSourcePlanReceiptV1,
    _seal: CompletedCanonicalCoreSourceEntryCandidateSealV1,
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
            CompletedCanonicalCoreSourceEntryFamilyV1::Main(_) => None,
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
        let (plan, admission, receipt) = request.into_parts();
        match plan {
            SealedNormalSourcePlanV1::ScalarRoot(SealedNormalScalarRootV1::Main0(main)) => {
                Self::compile_main0(compiler, main, admission, receipt)
            }
            SealedNormalSourcePlanV1::ScalarRoot(SealedNormalScalarRootV1::Script(script)) => {
                Self::compile_script(compiler, script, admission, receipt)
            }
            SealedNormalSourcePlanV1::CallableModule(callable) => Err(reject(
                SealedNormalSourcePlanV1::CallableModule(callable),
                admission,
                receipt,
                CanonicalCoreDispatchStageV1::FamilyCapability,
                CanonicalCoreDispatchErrorV1::FamilyCapabilityPending(
                    CanonicalCorePendingFamilyV1::CallableModule,
                ),
            )),
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
        let transaction = match compiler.builder.prepare_normal_main_module_transaction(batch) {
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
    ) -> Result<CompletedCanonicalCoreSourceEntryCandidateV1, RejectedCanonicalCoreNormalDispatchV1>
    {
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
    ) -> Result<
        CompletedCanonicalCoreSourceEntryCandidateV1,
        RejectedCanonicalCoreNormalDispatchV1,
    > {
        NormalCanonicalCoreSourcePlanCompilerV1::consume(self, request)
    }
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
