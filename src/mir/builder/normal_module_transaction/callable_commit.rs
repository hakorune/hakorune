//! Isolated candidate preparation and infallible commit for a normal batch.

use crate::mir::builder::module_lowering_shell::{
    ModuleLoweringShellDrainInventoryV1, ModuleLoweringShellErrorV1, ModuleLoweringShellV1,
    PreparedModuleLoweringShellDrainV1,
};
use crate::mir::compiler::normal_source_plan::{
    PreparedNormalHelperTopologyReceiptV1, VerifiedNormalMainPhysicalRelationV1,
    VerifiedNormalMainThunkResultV1,
};
use crate::mir::verification::MirVerifier;
use crate::mir::verification_types::VerificationError;
use crate::mir::{MirFunction, MirModule};

use super::callable_batch::PreparedNormalCallableBatchV1;

#[derive(Debug)]
pub(in crate::mir) enum NormalCallableCommitErrorV1 {
    Correspondence,
    Verification(Box<[VerificationError]>),
    Shell(ModuleLoweringShellErrorV1),
}

#[derive(Debug)]
pub(in crate::mir) struct RejectedNormalCallableCommitV1 {
    batch: PreparedNormalCallableBatchV1,
    error: NormalCallableCommitErrorV1,
}

impl RejectedNormalCallableCommitV1 {
    pub(in crate::mir) fn error(&self) -> &NormalCallableCommitErrorV1 {
        &self.error
    }

    pub(in crate::mir) fn discard(self) {
        drop(self);
    }
}

#[derive(Debug)]
pub(in crate::mir) struct PreparedNormalCallableCommitV1 {
    batch: PreparedNormalCallableBatchV1,
    shell: PreparedModuleLoweringShellDrainV1,
    verification: NormalCallableCandidateVerificationReceiptV1,
}

#[derive(Debug)]
pub(in crate::mir) struct CompletedNormalCallableCandidateV1 {
    module: MirModule,
    evidence: CompletedNormalCallableModuleEvidenceV1,
    verification: NormalCallableCandidateVerificationReceiptV1,
}

#[derive(Debug)]
pub(in crate::mir) struct CompletedNormalCallableModuleEvidenceV1 {
    schema: super::NormalModuleTransactionSchemaV1,
    relation: VerifiedNormalMainPhysicalRelationV1,
    topology: PreparedNormalHelperTopologyReceiptV1,
    source_identity: Box<str>,
}

#[derive(Debug)]
pub(in crate::mir) struct NormalCallableCandidateVerificationReceiptV1 {
    function_count: usize,
    schema_row_count: usize,
}

impl CompletedNormalCallableCandidateV1 {
    pub(in crate::mir) fn module(&self) -> &MirModule {
        &self.module
    }

    pub(in crate::mir) const fn evidence(&self) -> &CompletedNormalCallableModuleEvidenceV1 {
        &self.evidence
    }

    pub(in crate::mir) const fn verification(
        &self,
    ) -> &NormalCallableCandidateVerificationReceiptV1 {
        &self.verification
    }

    /// Consuming candidate split for the shared canonical publication owner.
    /// Target and result remain sealed in `evidence`; publication must project
    /// them rather than inspect this module.
    pub(in crate::mir) fn into_publication_parts(
        self,
    ) -> (
        MirModule,
        CompletedNormalCallableModuleEvidenceV1,
        NormalCallableCandidateVerificationReceiptV1,
    ) {
        (self.module, self.evidence, self.verification)
    }
}

impl CompletedNormalCallableModuleEvidenceV1 {
    pub(in crate::mir) fn source_identity(&self) -> &str {
        &self.source_identity
    }

    pub(in crate::mir) fn source_owner(&self) -> crate::mir::resolved_semantics::FunctionOwnerIdV1 {
        self.relation.entry().source_owner()
    }

    /// This is sealed before candidate commit. Publication projects it and
    /// never infers a source result from the module or its physical Return.
    pub(in crate::mir) const fn source_result(&self) -> VerifiedNormalMainThunkResultV1 {
        self.relation.source_result()
    }

    pub(in crate::mir) fn physical_symbol(&self) -> &str {
        self.relation.entry().physical_symbol()
    }

    pub(in crate::mir) fn physical_arity(&self) -> usize {
        self.relation.entry().physical_arity()
    }

    pub(in crate::mir) fn schema_row_count(&self) -> usize {
        self.schema.rows().len()
    }

    pub(in crate::mir) fn helper_count(&self) -> usize {
        self.topology.helper_count()
    }
}

impl NormalCallableCandidateVerificationReceiptV1 {
    pub(in crate::mir) const fn function_count(&self) -> usize {
        self.function_count
    }

    pub(in crate::mir) const fn schema_row_count(&self) -> usize {
        self.schema_row_count
    }
}

impl PreparedNormalCallableBatchV1 {
    pub(in crate::mir) fn prepare_normal_callable_commit_v1(
        self,
    ) -> Result<PreparedNormalCallableCommitV1, RejectedNormalCallableCommitV1> {
        let functions = draft_views(&self);
        if self.schema().rows().len() != functions.len()
            || self.schema().rows().iter().any(|row| {
                !functions.iter().any(|function| {
                    row.symbol() == function.signature.name
                        && row.arity() == function.signature.params.len()
                })
            })
        {
            return Err(RejectedNormalCallableCommitV1 {
                batch: self,
                error: NormalCallableCommitErrorV1::Correspondence,
            });
        }
        let mut errors = Vec::new();
        for function in &functions {
            if let Err(mut found) = MirVerifier::new().verify_function(function) {
                errors.append(&mut found);
            }
        }
        if !errors.is_empty() {
            return Err(RejectedNormalCallableCommitV1 {
                batch: self,
                error: NormalCallableCommitErrorV1::Verification(errors.into_boxed_slice()),
            });
        }
        let inventory = match ModuleLoweringShellDrainInventoryV1::from_symbols(
            functions
                .iter()
                .map(|function| function.signature.name.clone()),
        ) {
            Ok(inventory) => inventory,
            Err(error) => {
                return Err(RejectedNormalCallableCommitV1 {
                    batch: self,
                    error: NormalCallableCommitErrorV1::Shell(error),
                })
            }
        };
        let shell =
            match ModuleLoweringShellV1::from_empty_module(MirModule::new("main".to_owned())) {
                Ok(shell) => shell.prepare_drain(inventory),
                Err(error) => {
                    return Err(RejectedNormalCallableCommitV1 {
                        batch: self,
                        error: NormalCallableCommitErrorV1::Shell(error),
                    })
                }
            };
        Ok(PreparedNormalCallableCommitV1 {
            verification: NormalCallableCandidateVerificationReceiptV1 {
                function_count: functions.len(),
                schema_row_count: self.schema().rows().len(),
            },
            batch: self,
            shell,
        })
    }
}

impl PreparedNormalCallableCommitV1 {
    pub(in crate::mir) fn commit(self) -> CompletedNormalCallableCandidateV1 {
        let (drafts, schema) = self.batch.into_parts();
        let source_identity = drafts.source_identity().into();
        let (topology, helpers, source, physical, relation) = drafts.into_evidence_parts();
        let mut functions = helpers
            .into_iter()
            .map(|helper| helper.into_draft())
            .collect::<Vec<_>>();
        functions.push(source.into_draft());
        functions.push(physical.into_draft());
        CompletedNormalCallableCandidateV1 {
            module: self.shell.commit_preflighted(functions),
            evidence: CompletedNormalCallableModuleEvidenceV1 {
                schema,
                relation,
                topology,
                source_identity,
            },
            verification: self.verification,
        }
    }
}

fn draft_views(batch: &PreparedNormalCallableBatchV1) -> Vec<&MirFunction> {
    let mut functions = batch
        .drafts()
        .helpers()
        .drafts()
        .iter()
        .map(|helper| helper.draft())
        .collect::<Vec<_>>();
    functions.push(batch.drafts().source().draft());
    functions.push(batch.drafts().physical().draft());
    functions
}

#[cfg(test)]
pub(crate) fn reject_normal_callable_commit_for_test(
    batch: PreparedNormalCallableBatchV1,
) -> RejectedNormalCallableCommitV1 {
    RejectedNormalCallableCommitV1 {
        batch,
        error: NormalCallableCommitErrorV1::Correspondence,
    }
}
