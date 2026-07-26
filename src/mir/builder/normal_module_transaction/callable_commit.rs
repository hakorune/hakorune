//! Isolated candidate preparation and infallible commit for a normal batch.

use crate::mir::builder::module_lowering_shell::{
    ModuleLoweringShellDrainInventoryV1, ModuleLoweringShellErrorV1, ModuleLoweringShellV1,
    PreparedModuleLoweringShellDrainV1,
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
    pub(in crate::mir) fn discard(self) {
        drop(self);
    }
}

#[derive(Debug)]
pub(in crate::mir) struct PreparedNormalCallableCommitV1 {
    batch: PreparedNormalCallableBatchV1,
    shell: PreparedModuleLoweringShellDrainV1,
}

#[derive(Debug)]
pub(in crate::mir) struct CompletedNormalCallableCandidateV1 {
    module: MirModule,
}

impl CompletedNormalCallableCandidateV1 {
    pub(in crate::mir) fn module(&self) -> &MirModule {
        &self.module
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
        Ok(PreparedNormalCallableCommitV1 { batch: self, shell })
    }
}

impl PreparedNormalCallableCommitV1 {
    pub(in crate::mir) fn commit(self) -> CompletedNormalCallableCandidateV1 {
        let (helpers, source, physical) = self.batch.into_drafts().into_drafts();
        let mut functions = helpers
            .into_drafts()
            .into_iter()
            .map(|helper| helper.into_draft())
            .collect::<Vec<_>>();
        functions.push(source.into_draft());
        functions.push(physical.into_draft());
        CompletedNormalCallableCandidateV1 {
            module: self.shell.commit_preflighted(functions),
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
