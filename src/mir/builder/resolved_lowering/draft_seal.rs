//! Strict F1 draft-seal vocabulary.
//!
//! This module is deliberately small in DRAFT-SEAL0-S0.  It does not touch
//! the legacy `MirBuilder::finalize_function_draft` terminal.  The only
//! responsibility here is to turn the already verified completion witness
//! into one move-only exit plan before a future builder/session prepare is
//! added.  Keeping this transition separate prevents the lowerers from
//! becoming a second Return/signature authority.

use crate::mir::BasicBlockId;

use super::completion_consumption::ReadyFunctionCompletionV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum PreparedFunctionExitV1 {
    ExplicitValue {
        block: BasicBlockId,
        value: crate::mir::ValueId,
    },
    ExplicitUnit {
        block: BasicBlockId,
    },
    ImplicitUnit {
        block: BasicBlockId,
    },
}

#[derive(Debug)]
pub(super) struct ReadyFunctionDraftSealV1 {
    completion: ReadyFunctionCompletionV1,
    current_block: BasicBlockId,
}

#[derive(Debug)]
pub(super) struct PreparedFunctionDraftSealV1 {
    completion: ReadyFunctionCompletionV1,
    exit: PreparedFunctionExitV1,
}

#[derive(Debug)]
pub(super) struct CompletedFunctionDraftV1 {
    completion: ReadyFunctionCompletionV1,
    exit: PreparedFunctionExitV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum FunctionDraftSealPreparationErrorV1 {
    ExplicitValueOperandMissing,
}

#[derive(Debug)]
pub(super) struct RejectedFunctionDraftSealV1 {
    owner: ReadyFunctionDraftSealV1,
    error: FunctionDraftSealPreparationErrorV1,
}

impl ReadyFunctionDraftSealV1 {
    pub(super) fn new(completion: ReadyFunctionCompletionV1, current_block: BasicBlockId) -> Self {
        Self {
            completion,
            current_block,
        }
    }

    pub(super) fn prepare(
        self,
    ) -> Result<PreparedFunctionDraftSealV1, RejectedFunctionDraftSealV1> {
        let exit = if self.completion.is_implicit_void() {
            PreparedFunctionExitV1::ImplicitUnit {
                block: self.current_block,
            }
        } else if self.completion.returns_value() {
            let Some(witness) = self.completion.explicit_operand() else {
                return Err(RejectedFunctionDraftSealV1 {
                    owner: self,
                    error: FunctionDraftSealPreparationErrorV1::ExplicitValueOperandMissing,
                });
            };
            let block = witness.block();
            let value = witness.value();
            PreparedFunctionExitV1::ExplicitValue { block, value }
        } else {
            PreparedFunctionExitV1::ExplicitUnit {
                block: self.current_block,
            }
        };

        Ok(PreparedFunctionDraftSealV1 {
            completion: self.completion,
            exit,
        })
    }
}

impl PreparedFunctionDraftSealV1 {
    /// The DRAFT-SEAL0 commit is intentionally an ownership-only transition.
    /// Physical Return/signature writes will be added here once the projected
    /// type/session plans land; no fallible edge is allowed after this point.
    pub(super) fn commit(self) -> CompletedFunctionDraftV1 {
        CompletedFunctionDraftV1 {
            completion: self.completion,
            exit: self.exit,
        }
    }
}

impl RejectedFunctionDraftSealV1 {
    pub(super) fn error(&self) -> FunctionDraftSealPreparationErrorV1 {
        self.error
    }

    pub(super) fn discard(self) {
        let _ = self.owner;
    }
}

impl CompletedFunctionDraftV1 {
    pub(super) fn exit(&self) -> PreparedFunctionExitV1 {
        self.exit
    }

    pub(super) fn completion(&self) -> &ReadyFunctionCompletionV1 {
        &self.completion
    }
}
