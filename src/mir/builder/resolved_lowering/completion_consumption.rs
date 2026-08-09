//! Exact completion consumption for the canonical draft-seal handoff.

use crate::mir::compiler::located::SourceBodySiteV1;
use crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1;
use crate::mir::resolved_semantics::{FunctionOwnerIdV1, RegionId, SourceStmtSiteV1};
use crate::mir::{BasicBlockId, ValueId};

#[derive(Debug)]
pub(super) struct ResolvedFunctionCompletionConsumptionV1 {
    completion: VerifiedFunctionCompletionV1,
    explicit_consumed: bool,
    explicit_exit: Option<ExplicitReturnWitnessV1>,
}

/// Temporal witness minted only after every current canonical Lower finish.
///
/// The future SSA-I1 finish slots before this witness without changing the
/// finalizer API. Raw pre-Builder completion products cannot finalize a draft.
#[derive(Debug)]
pub(super) struct ReadyFunctionCompletionV1 {
    completion: VerifiedFunctionCompletionV1,
    explicit_exit: Option<ExplicitReturnWitnessV1>,
}

impl ReadyFunctionCompletionV1 {
    pub(super) fn explicit_operand(&self) -> Option<ReturnOperandWitnessV1> {
        match self.explicit_exit {
            Some(ExplicitReturnWitnessV1::Value(witness)) => Some(witness),
            Some(ExplicitReturnWitnessV1::Unit) | None => None,
        }
    }

    pub(super) fn explicit_is_unit(&self) -> bool {
        matches!(self.explicit_exit, Some(ExplicitReturnWitnessV1::Unit))
    }

    pub(super) fn returns_value(&self) -> bool {
        self.completion.returns_value()
    }

    pub(super) fn is_implicit_void(&self) -> bool {
        self.completion.is_implicit_void()
    }
}

/// Builder-side evidence for the one explicit source exit accepted by F1.
///
/// The source completion contract decides whether the operand is a return;
/// this witness records only the exact already-lowered physical operand and
/// block so draft sealing never rediscovers it by scanning MIR.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct ReturnOperandWitnessV1 {
    block: BasicBlockId,
    value: ValueId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum ExplicitReturnWitnessV1 {
    Value(ReturnOperandWitnessV1),
    Unit,
}

impl ReturnOperandWitnessV1 {
    pub(super) fn new(block: BasicBlockId, value: ValueId) -> Self {
        Self { block, value }
    }

    pub(super) fn block(self) -> BasicBlockId {
        self.block
    }

    pub(super) fn value(self) -> ValueId {
        self.value
    }
}

impl ResolvedFunctionCompletionConsumptionV1 {
    pub(super) fn returns_value(&self) -> bool {
        self.completion.returns_value()
    }

    pub(super) fn new(
        expected_owner: FunctionOwnerIdV1,
        completion: VerifiedFunctionCompletionV1,
    ) -> Result<Self, String> {
        if completion.owner() != expected_owner {
            return Err("[freeze:contract][canonical_completion/owner_mismatch]".to_string());
        }
        if completion.unreachable_suffix_count() != 0 {
            return Err("[freeze:contract][canonical_completion/unreachable_suffix]".to_string());
        }
        if !completion.cleanup().crossed_scopes().is_empty() {
            return Err("[freeze:contract][canonical_completion/e0_cleanup_not_empty]".to_string());
        }
        Ok(Self {
            completion,
            explicit_consumed: false,
            explicit_exit: None,
        })
    }

    pub(super) fn claim_explicit_return(
        &mut self,
        site: &SourceStmtSiteV1,
        target_function: RegionId,
        block: BasicBlockId,
        value: ValueId,
    ) -> Result<(), String> {
        if !self.completion.returns_value() {
            return Err("[freeze:contract][canonical_completion/value_kind_mismatch]".to_string());
        }
        if self.explicit_consumed {
            return Err("[freeze:contract][canonical_completion/explicit_reconsumed]".to_string());
        }
        if self.completion.explicit_site() != Some(site) {
            return Err(
                "[freeze:contract][canonical_completion/explicit_site_mismatch]".to_string(),
            );
        }
        if self.completion.target_function() != target_function {
            return Err("[freeze:contract][canonical_completion/target_mismatch]".to_string());
        }
        self.explicit_consumed = true;
        self.explicit_exit = Some(ExplicitReturnWitnessV1::Value(ReturnOperandWitnessV1::new(
            block, value,
        )));
        Ok(())
    }

    pub(super) fn claim_explicit_unit(
        &mut self,
        site: &SourceStmtSiteV1,
        target_function: RegionId,
    ) -> Result<(), String> {
        if self.completion.returns_value() {
            return Err("[freeze:contract][canonical_completion/unit_kind_mismatch]".to_string());
        }
        if self.explicit_consumed {
            return Err("[freeze:contract][canonical_completion/explicit_reconsumed]".to_string());
        }
        if self.completion.explicit_site() != Some(site) {
            return Err(
                "[freeze:contract][canonical_completion/explicit_site_mismatch]".to_string(),
            );
        }
        if self.completion.target_function() != target_function {
            return Err("[freeze:contract][canonical_completion/target_mismatch]".to_string());
        }
        self.explicit_consumed = true;
        self.explicit_exit = Some(ExplicitReturnWitnessV1::Unit);
        Ok(())
    }

    pub(super) fn finish(
        self,
        root_body: &SourceBodySiteV1,
        root_body_end: u32,
        target_function: RegionId,
    ) -> Result<ReadyFunctionCompletionV1, String> {
        if self.completion.explicit_sites().len() > 1 {
            return Err(
                "[freeze:contract][canonical_completion/multiple_exit_draft_seal_closed]"
                    .to_string(),
            );
        }
        if self.completion.target_function() != target_function {
            return Err(
                "[freeze:contract][canonical_completion/finish_target_mismatch]".to_string(),
            );
        }
        if self.completion.explicit_site().is_some() != self.explicit_consumed {
            return Err("[freeze:contract][canonical_completion/consumption_mismatch]".to_string());
        }
        if self.completion.explicit_site().is_some() != self.explicit_exit.is_some() {
            return Err(
                "[freeze:contract][canonical_completion/operand_witness_missing]".to_string(),
            );
        }
        if let Some((expected_body, expected_end)) = self.completion.implicit_body_end() {
            if expected_body != root_body || expected_end != root_body_end {
                return Err(
                    "[freeze:contract][canonical_completion/implicit_body_mismatch]".to_string(),
                );
            }
        }
        Ok(ReadyFunctionCompletionV1 {
            completion: self.completion,
            explicit_exit: self.explicit_exit,
        })
    }
}
