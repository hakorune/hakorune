//! One atomic canonical source-Main plus physical-entry candidate transaction.

use crate::mir::builder::module_lowering_shell::{
    ModuleLoweringShellDrainInventoryV1, ModuleLoweringShellErrorV1, ModuleLoweringShellV1,
    PreparedModuleLoweringShellDrainV1,
};
use crate::mir::builder::resolved_lowering::CanonicalResolvedBuildErrorV1;
use crate::mir::compiler::capability::VerifiedResolvedOwnerHeaderV1;
use crate::mir::compiler::normal_source_plan::{
    VerifiedNormalMainEntryRelationV1, VerifiedNormalMainResolvedSourceUnitV1,
    VerifiedNormalMainThunkResultV1,
};
use crate::mir::resolved_semantics::FunctionOwnerIdV1;
use crate::mir::verification::MirVerifier;
use crate::mir::verification_types::VerificationError;
use crate::mir::{MirFunction, MirModule};

use super::super::MirBuilder;
use super::canonical_batch::PreparedNormalCanonicalModuleBatchV1;
use super::physical_thunk::{
    NormalMainPhysicalThunkErrorV1, VerifiedNormalMainPhysicalThunkDraftV1,
};
use super::schema::{NormalModuleDraftRoleV1, NormalModuleTransactionSchemaV1};
use super::source_draft::{NormalMainSourceDraftErrorV1, VerifiedNormalMainSourceDraftV1};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum NormalMainModuleTransactionStageV1 {
    SourceDraft,
    PhysicalThunk,
    BatchCorrespondence,
    CandidateVerification,
}

#[derive(Debug, PartialEq)]
pub(in crate::mir) enum NormalMainModuleTransactionErrorV1 {
    SourceLowering(CanonicalResolvedBuildErrorV1),
    SourceDraft(NormalMainSourceDraftErrorV1),
    PhysicalThunk(NormalMainPhysicalThunkErrorV1),
    BatchCorrespondence(NormalMainBatchCorrespondenceErrorV1),
    CandidateVerification(Box<[VerificationError]>),
    Shell(ModuleLoweringShellErrorV1),
}

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir) enum NormalMainBatchCorrespondenceErrorV1 {
    RowCardinality { actual: usize },
    MissingSourceMain,
    MissingPhysicalEntry,
    SourceOwnerMismatch,
    SourceSymbolMismatch,
    SourceArityMismatch,
    PhysicalSymbolMismatch,
    PhysicalArityMismatch,
    ResultMismatch,
}

#[derive(Debug)]
pub(in crate::mir) struct RetainedNormalMainTransactionEvidenceV1<'unit> {
    source_unit: &'unit VerifiedNormalMainResolvedSourceUnitV1,
    schema: NormalModuleTransactionSchemaV1,
    source_header: VerifiedResolvedOwnerHeaderV1,
    result: VerifiedNormalMainThunkResultV1,
    entry: VerifiedNormalMainEntryRelationV1,
}

#[derive(Debug)]
pub(in crate::mir) enum RetainedNormalMainPreparedDraftsV1 {
    None,
    UnsealedSource(MirFunction),
    Source(VerifiedNormalMainSourceDraftV1),
    SourceAndPhysical {
        source: VerifiedNormalMainSourceDraftV1,
        physical: VerifiedNormalMainPhysicalThunkDraftV1,
    },
}

#[derive(Debug)]
pub(in crate::mir) struct NormalMainBuilderRestorationReceiptV1 {
    _seal: NormalMainBuilderRestorationReceiptSealV1,
}

#[derive(Debug)]
struct NormalMainBuilderRestorationReceiptSealV1;

#[derive(Debug)]
pub(in crate::mir) struct RejectedNormalMainModuleTransactionV1<'unit> {
    evidence: RetainedNormalMainTransactionEvidenceV1<'unit>,
    stage: NormalMainModuleTransactionStageV1,
    error: NormalMainModuleTransactionErrorV1,
    prepared: RetainedNormalMainPreparedDraftsV1,
    restoration: NormalMainBuilderRestorationReceiptV1,
}

#[derive(Debug)]
pub(in crate::mir) struct PreparedNormalMainModuleTransactionV1<'unit> {
    evidence: RetainedNormalMainTransactionEvidenceV1<'unit>,
    source: VerifiedNormalMainSourceDraftV1,
    physical: VerifiedNormalMainPhysicalThunkDraftV1,
    shell: PreparedModuleLoweringShellDrainV1,
    verification: NormalMainCandidateVerificationReceiptV1,
    _seal: PreparedNormalMainModuleTransactionSealV1,
}

#[derive(Debug)]
struct PreparedNormalMainModuleTransactionSealV1;

#[derive(Debug)]
pub(in crate::mir) struct NormalMainCandidateVerificationReceiptV1 {
    function_count: usize,
    _seal: NormalMainCandidateVerificationReceiptSealV1,
}

#[derive(Debug)]
struct NormalMainCandidateVerificationReceiptSealV1;

#[derive(Debug)]
pub(in crate::mir) struct CompletedNormalMainModuleEvidenceV1 {
    schema: NormalModuleTransactionSchemaV1,
    source_header: VerifiedResolvedOwnerHeaderV1,
    result: VerifiedNormalMainThunkResultV1,
    entry: VerifiedNormalMainEntryRelationV1,
}

#[derive(Debug)]
pub(in crate::mir) struct CompletedNormalMainModuleCandidateV1 {
    module: MirModule,
    evidence: CompletedNormalMainModuleEvidenceV1,
    verification: NormalMainCandidateVerificationReceiptV1,
    _seal: CompletedNormalMainModuleCandidateSealV1,
}

#[derive(Debug)]
struct CompletedNormalMainModuleCandidateSealV1;

#[derive(Debug)]
pub(in crate::mir) struct PublishedNormalMainInvocationV1 {
    module: MirModule,
    evidence: CompletedNormalMainModuleEvidenceV1,
    verification: NormalMainCandidateVerificationReceiptV1,
    _seal: PublishedNormalMainInvocationSealV1,
}

#[derive(Debug)]
struct PublishedNormalMainInvocationSealV1;

impl MirBuilder {
    pub(in crate::mir) fn prepare_normal_main_module_transaction<'unit>(
        &mut self,
        batch: PreparedNormalCanonicalModuleBatchV1<'unit>,
    ) -> Result<
        PreparedNormalMainModuleTransactionV1<'unit>,
        RejectedNormalMainModuleTransactionV1<'unit>,
    > {
        let (thunk, schema) = batch.into_parts();
        let (source_plan, source_header, result, entry) = thunk.into_parts();
        let source_unit = source_plan.source_unit();
        let evidence = RetainedNormalMainTransactionEvidenceV1 {
            source_unit,
            schema,
            source_header,
            result,
            entry,
        };

        let source_draft =
            match self.lower_resolved_trivial_function_draft(source_plan.into_lowering()) {
                Ok(draft) => draft,
                Err(error) => {
                    return Err(reject(
                        evidence,
                        NormalMainModuleTransactionStageV1::SourceDraft,
                        NormalMainModuleTransactionErrorV1::SourceLowering(error),
                        RetainedNormalMainPreparedDraftsV1::None,
                    ))
                }
            };
        let source = match VerifiedNormalMainSourceDraftV1::seal(
            source_draft,
            evidence.source_header.symbol().as_mir_name(),
            evidence.source_header.arity(),
            evidence.result,
        ) {
            Ok(source) => source,
            Err((draft, error)) => {
                return Err(reject(
                    evidence,
                    NormalMainModuleTransactionStageV1::SourceDraft,
                    NormalMainModuleTransactionErrorV1::SourceDraft(error),
                    RetainedNormalMainPreparedDraftsV1::UnsealedSource(draft),
                ))
            }
        };
        let physical = match VerifiedNormalMainPhysicalThunkDraftV1::prepare(
            &evidence.source_header,
            evidence.result,
            &evidence.entry,
        ) {
            Ok(physical) => physical,
            Err(error) => {
                return Err(reject(
                    evidence,
                    NormalMainModuleTransactionStageV1::PhysicalThunk,
                    NormalMainModuleTransactionErrorV1::PhysicalThunk(error),
                    RetainedNormalMainPreparedDraftsV1::Source(source),
                ))
            }
        };
        if let Err(error) = validate_correspondence(&evidence, &source, &physical) {
            return Err(reject(
                evidence,
                NormalMainModuleTransactionStageV1::BatchCorrespondence,
                NormalMainModuleTransactionErrorV1::BatchCorrespondence(error),
                RetainedNormalMainPreparedDraftsV1::SourceAndPhysical { source, physical },
            ));
        }
        let verification = match verify_candidate(&source, &physical) {
            Ok(receipt) => receipt,
            Err(errors) => {
                return Err(reject(
                    evidence,
                    NormalMainModuleTransactionStageV1::CandidateVerification,
                    NormalMainModuleTransactionErrorV1::CandidateVerification(errors),
                    RetainedNormalMainPreparedDraftsV1::SourceAndPhysical { source, physical },
                ))
            }
        };
        let symbols = evidence
            .schema
            .rows()
            .iter()
            .map(|row| row.symbol().to_owned())
            .collect::<Vec<_>>();
        let inventory = match ModuleLoweringShellDrainInventoryV1::from_symbols(symbols) {
            Ok(inventory) => inventory,
            Err(error) => {
                return Err(reject(
                    evidence,
                    NormalMainModuleTransactionStageV1::BatchCorrespondence,
                    NormalMainModuleTransactionErrorV1::Shell(error),
                    RetainedNormalMainPreparedDraftsV1::SourceAndPhysical { source, physical },
                ))
            }
        };
        let shell =
            match ModuleLoweringShellV1::from_empty_module(MirModule::new("main".to_owned())) {
                Ok(shell) => shell,
                Err(error) => {
                    return Err(reject(
                        evidence,
                        NormalMainModuleTransactionStageV1::CandidateVerification,
                        NormalMainModuleTransactionErrorV1::Shell(error),
                        RetainedNormalMainPreparedDraftsV1::SourceAndPhysical { source, physical },
                    ))
                }
            }
            .prepare_drain(inventory);
        Ok(PreparedNormalMainModuleTransactionV1 {
            evidence,
            source,
            physical,
            shell,
            verification,
            _seal: PreparedNormalMainModuleTransactionSealV1,
        })
    }

    #[cfg(test)]
    pub(in crate::mir) fn complete_normal_main_candidate_for_test<'unit>(
        &mut self,
        thunk: crate::mir::compiler::normal_source_plan::VerifiedNormalMainThunkPlanV1<'unit>,
    ) -> CompletedNormalMainModuleCandidateV1 {
        let batch = super::canonical_batch::NormalCanonicalModuleBatchV1::prepare(thunk)
            .expect("test Main batch");
        self.prepare_normal_main_module_transaction(batch)
            .expect("test Main transaction")
            .commit()
    }
}

impl<'unit> PreparedNormalMainModuleTransactionV1<'unit> {
    pub(in crate::mir) fn commit(self) -> CompletedNormalMainModuleCandidateV1 {
        let functions = vec![self.source.into_draft(), self.physical.into_draft()];
        let module = self.shell.commit_preflighted(functions);
        let RetainedNormalMainTransactionEvidenceV1 {
            source_unit: _,
            schema,
            source_header,
            result,
            entry,
        } = self.evidence;
        CompletedNormalMainModuleCandidateV1 {
            module,
            evidence: CompletedNormalMainModuleEvidenceV1 {
                schema,
                source_header,
                result,
                entry,
            },
            verification: self.verification,
            _seal: CompletedNormalMainModuleCandidateSealV1,
        }
    }
}

impl RejectedNormalMainModuleTransactionV1<'_> {
    pub(in crate::mir) const fn stage(&self) -> NormalMainModuleTransactionStageV1 {
        self.stage
    }

    pub(in crate::mir) fn error(&self) -> &NormalMainModuleTransactionErrorV1 {
        &self.error
    }

    pub(in crate::mir) fn discard(self) {
        drop(self);
    }

    #[cfg(test)]
    pub(super) fn retained_source_owner(&self) -> FunctionOwnerIdV1 {
        let _ = self.evidence.source_unit;
        self.evidence.source_header.owner()
    }

    #[cfg(test)]
    pub(super) fn prepared_kind(&self) -> RetainedNormalMainPreparedDraftKindV1 {
        match &self.prepared {
            RetainedNormalMainPreparedDraftsV1::None => RetainedNormalMainPreparedDraftKindV1::None,
            RetainedNormalMainPreparedDraftsV1::UnsealedSource(_) => {
                RetainedNormalMainPreparedDraftKindV1::UnsealedSource
            }
            RetainedNormalMainPreparedDraftsV1::Source(_) => {
                RetainedNormalMainPreparedDraftKindV1::Source
            }
            RetainedNormalMainPreparedDraftsV1::SourceAndPhysical { .. } => {
                RetainedNormalMainPreparedDraftKindV1::SourceAndPhysical
            }
        }
    }

    #[cfg(test)]
    pub(super) fn has_restoration_receipt(&self) -> bool {
        let _ = &self.restoration;
        true
    }
}

#[cfg(test)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum RetainedNormalMainPreparedDraftKindV1 {
    None,
    UnsealedSource,
    Source,
    SourceAndPhysical,
}

impl CompletedNormalMainModuleCandidateV1 {
    pub(in crate::mir) fn module(&self) -> &MirModule {
        &self.module
    }

    pub(in crate::mir) fn result(&self) -> VerifiedNormalMainThunkResultV1 {
        self.evidence.result
    }

    pub(in crate::mir) fn source_owner(&self) -> FunctionOwnerIdV1 {
        self.evidence.source_header.owner()
    }

    pub(in crate::mir) const fn verification_count(&self) -> usize {
        self.verification.function_count
    }

    pub(in crate::mir) fn publish(self) -> PublishedNormalMainInvocationV1 {
        PublishedNormalMainInvocationV1 {
            module: self.module,
            evidence: self.evidence,
            verification: self.verification,
            _seal: PublishedNormalMainInvocationSealV1,
        }
    }
}

impl PublishedNormalMainInvocationV1 {
    #[cfg(feature = "vm-reference")]
    pub(in crate::mir) fn execute_exact_vm_entry(
        &self,
        symbol: &str,
    ) -> Result<crate::backend::vm_types::VMValue, crate::backend::vm_types::VMError> {
        let mut interpreter = crate::backend::mir_interpreter::MirInterpreter::new();
        interpreter.execute_function_with_args(&self.module, symbol, &[])
    }

    pub(in crate::mir) fn result(&self) -> VerifiedNormalMainThunkResultV1 {
        self.evidence.result
    }

    pub(in crate::mir) fn source_owner(&self) -> FunctionOwnerIdV1 {
        self.evidence.source_header.owner()
    }

    pub(in crate::mir) fn entry_source_owner(&self) -> FunctionOwnerIdV1 {
        self.evidence.entry.source_owner()
    }

    pub(in crate::mir) fn physical_symbol(&self) -> &str {
        self.evidence.entry.physical_symbol()
    }

    pub(in crate::mir) const fn physical_arity(&self) -> usize {
        self.evidence.entry.physical_arity()
    }

    pub(in crate::mir) fn has_exact_membership(&self) -> bool {
        self.verification.function_count == 2
            && self.evidence.schema.rows().len() == 2
            && self.module.functions.len() == 2
    }

    pub(in crate::mir) const fn verification_count(&self) -> usize {
        self.verification.function_count
    }
}

fn validate_correspondence(
    evidence: &RetainedNormalMainTransactionEvidenceV1<'_>,
    source: &VerifiedNormalMainSourceDraftV1,
    physical: &VerifiedNormalMainPhysicalThunkDraftV1,
) -> Result<(), NormalMainBatchCorrespondenceErrorV1> {
    let rows = evidence.schema.rows();
    if rows.len() != 2 {
        return Err(NormalMainBatchCorrespondenceErrorV1::RowCardinality { actual: rows.len() });
    }
    let source_row = rows
        .iter()
        .find(|row| matches!(row.role(), NormalModuleDraftRoleV1::SourceMain { .. }))
        .ok_or(NormalMainBatchCorrespondenceErrorV1::MissingSourceMain)?;
    let physical_row = rows
        .iter()
        .find(|row| matches!(row.role(), NormalModuleDraftRoleV1::PhysicalEntry))
        .ok_or(NormalMainBatchCorrespondenceErrorV1::MissingPhysicalEntry)?;
    let relation = evidence.schema.source_entry();
    if relation.source_main_owner() != evidence.source_header.owner() {
        return Err(NormalMainBatchCorrespondenceErrorV1::SourceOwnerMismatch);
    }
    if source_row.symbol() != source.draft().signature.name
        || relation.source_main_symbol() != source.draft().signature.name
    {
        return Err(NormalMainBatchCorrespondenceErrorV1::SourceSymbolMismatch);
    }
    if source_row.arity() != source.draft().signature.params.len()
        || relation.source_main_arity() != source.draft().signature.params.len()
    {
        return Err(NormalMainBatchCorrespondenceErrorV1::SourceArityMismatch);
    }
    if physical_row.symbol() != physical.draft().signature.name
        || relation.physical_symbol() != physical.draft().signature.name
    {
        return Err(NormalMainBatchCorrespondenceErrorV1::PhysicalSymbolMismatch);
    }
    if physical_row.arity() != physical.draft().signature.params.len()
        || relation.physical_arity() != physical.draft().signature.params.len()
    {
        return Err(NormalMainBatchCorrespondenceErrorV1::PhysicalArityMismatch);
    }
    if source.draft().signature.return_type != physical.draft().signature.return_type
        || physical.result() != evidence.result
    {
        return Err(NormalMainBatchCorrespondenceErrorV1::ResultMismatch);
    }
    Ok(())
}

fn verify_candidate(
    source: &VerifiedNormalMainSourceDraftV1,
    physical: &VerifiedNormalMainPhysicalThunkDraftV1,
) -> Result<NormalMainCandidateVerificationReceiptV1, Box<[VerificationError]>> {
    let mut errors = Vec::new();
    for function in [source.draft(), physical.draft()] {
        if let Err(mut function_errors) = MirVerifier::new().verify_function(function) {
            errors.append(&mut function_errors);
        }
    }
    if errors.is_empty() {
        Ok(NormalMainCandidateVerificationReceiptV1 {
            function_count: 2,
            _seal: NormalMainCandidateVerificationReceiptSealV1,
        })
    } else {
        Err(errors.into_boxed_slice())
    }
}

fn reject<'unit>(
    evidence: RetainedNormalMainTransactionEvidenceV1<'unit>,
    stage: NormalMainModuleTransactionStageV1,
    error: NormalMainModuleTransactionErrorV1,
    prepared: RetainedNormalMainPreparedDraftsV1,
) -> RejectedNormalMainModuleTransactionV1<'unit> {
    RejectedNormalMainModuleTransactionV1 {
        evidence,
        stage,
        error,
        prepared,
        restoration: NormalMainBuilderRestorationReceiptV1 {
            _seal: NormalMainBuilderRestorationReceiptSealV1,
        },
    }
}

#[cfg(test)]
pub(super) fn reject_normal_main_batch_at_stage_for_test<'unit>(
    builder: &mut MirBuilder,
    batch: PreparedNormalCanonicalModuleBatchV1<'unit>,
    stage: NormalMainModuleTransactionStageV1,
) -> RejectedNormalMainModuleTransactionV1<'unit> {
    let (thunk, schema) = batch.into_parts();
    let (source_plan, source_header, result, entry) = thunk.into_parts();
    let source_unit = source_plan.source_unit();
    let evidence = RetainedNormalMainTransactionEvidenceV1 {
        source_unit,
        schema,
        source_header,
        result,
        entry,
    };
    if stage == NormalMainModuleTransactionStageV1::SourceDraft {
        return reject(
            evidence,
            stage,
            NormalMainModuleTransactionErrorV1::SourceLowering(
                CanonicalResolvedBuildErrorV1::BuilderContract(
                    "[test/normal-main/source-draft]".to_owned(),
                ),
            ),
            RetainedNormalMainPreparedDraftsV1::None,
        );
    }

    let source_draft = builder
        .lower_resolved_trivial_function_draft(source_plan.into_lowering())
        .expect("test failure injection requires a valid source draft");
    let source = VerifiedNormalMainSourceDraftV1::seal(
        source_draft,
        evidence.source_header.symbol().as_mir_name(),
        evidence.source_header.arity(),
        evidence.result,
    )
    .expect("test failure injection requires exact source correspondence");
    if stage == NormalMainModuleTransactionStageV1::PhysicalThunk {
        return reject(
            evidence,
            stage,
            NormalMainModuleTransactionErrorV1::PhysicalThunk(
                NormalMainPhysicalThunkErrorV1::PhysicalArityMismatch { actual: 1 },
            ),
            RetainedNormalMainPreparedDraftsV1::Source(source),
        );
    }

    let physical = VerifiedNormalMainPhysicalThunkDraftV1::prepare(
        &evidence.source_header,
        evidence.result,
        &evidence.entry,
    )
    .expect("test failure injection requires a valid physical draft");
    let prepared = RetainedNormalMainPreparedDraftsV1::SourceAndPhysical { source, physical };
    match stage {
        NormalMainModuleTransactionStageV1::BatchCorrespondence => reject(
            evidence,
            stage,
            NormalMainModuleTransactionErrorV1::BatchCorrespondence(
                NormalMainBatchCorrespondenceErrorV1::SourceSymbolMismatch,
            ),
            prepared,
        ),
        NormalMainModuleTransactionStageV1::CandidateVerification => reject(
            evidence,
            stage,
            NormalMainModuleTransactionErrorV1::CandidateVerification(
                vec![VerificationError::ControlFlowError {
                    block: crate::mir::BasicBlockId::new(0),
                    reason: "injected candidate verification failure".to_owned(),
                }]
                .into_boxed_slice(),
            ),
            prepared,
        ),
        NormalMainModuleTransactionStageV1::SourceDraft
        | NormalMainModuleTransactionStageV1::PhysicalThunk => {
            unreachable!("earlier injected stages already returned")
        }
    }
}
