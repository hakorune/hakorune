//! Exact completion consumption for the canonical draft-seal handoff.

use crate::mir::compiler::located::SourceBodySiteV1;
use crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1;
use crate::mir::resolved_semantics::{FunctionOwnerIdV1, RegionId, SourceStmtSiteV1};
use crate::mir::{BasicBlockId, ValueId};

#[derive(Debug)]
pub(super) struct ResolvedFunctionCompletionConsumptionV1 {
    completion: VerifiedFunctionCompletionV1,
    /// Slots are indexed by the resolver-owned `explicit_sites()` order.
    /// The site is still carried in every claim so a future consumer cannot
    /// silently treat storage order as source identity.
    explicit_claims: Box<[Option<ExplicitReturnClaimV1>]>,
}

/// Temporal witness minted only after every current canonical Lower finish.
///
/// The future SSA-I1 finish slots before this witness without changing the
/// finalizer API. Raw pre-Builder completion products cannot finalize a draft.
#[derive(Debug)]
pub(super) struct ReadyFunctionCompletionV1 {
    completion: VerifiedFunctionCompletionV1,
    explicit_claims: Box<[ExplicitReturnClaimV1]>,
}

impl ReadyFunctionCompletionV1 {
    pub(super) fn explicit_operand(&self) -> Option<ReturnOperandWitnessV1> {
        if self.explicit_claims.len() != 1 {
            return None;
        }
        match self.explicit_claims[0].witness {
            ExplicitReturnWitnessV1::Value(witness) => Some(witness),
            ExplicitReturnWitnessV1::Unit => None,
        }
    }

    pub(super) fn explicit_is_unit(&self) -> bool {
        self.explicit_claims.len() == 1
            && matches!(
                self.explicit_claims[0].witness,
                ExplicitReturnWitnessV1::Unit
            )
    }

    pub(super) fn returns_value(&self) -> bool {
        self.completion.returns_value()
    }

    pub(super) fn is_implicit_void(&self) -> bool {
        self.completion.is_implicit_void()
    }

    /// Exact site-keyed physical claims in the resolver's canonical source
    /// order.  A multi-site caller must consume this complete set; the
    /// single-operand helper above intentionally returns `None` for it.
    pub(super) fn explicit_claims(&self) -> &[ExplicitReturnClaimV1] {
        &self.explicit_claims
    }
}

/// Builder-side evidence for one explicit source exit.  The completion
/// consumer can retain a complete site-keyed set; the current DraftSeal
/// writer still admits only a single claim.
///
/// The source completion contract decides whether the operand is a return;
/// this witness records only the exact already-lowered physical operand and
/// block so draft sealing never rediscovers it by scanning MIR.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct ReturnOperandWitnessV1 {
    block: BasicBlockId,
    value: ValueId,
}

/// One physical claim tied to one source Completion site.  This is a mutable
/// lowering-side witness, not a second semantic return-site authority.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct ExplicitReturnClaimV1 {
    site: SourceStmtSiteV1,
    witness: ExplicitReturnWitnessV1,
}

impl ExplicitReturnClaimV1 {
    fn value(site: SourceStmtSiteV1, block: BasicBlockId, value: ValueId) -> Self {
        Self {
            site,
            witness: ExplicitReturnWitnessV1::Value(ReturnOperandWitnessV1::new(block, value)),
        }
    }

    fn unit(site: SourceStmtSiteV1) -> Self {
        Self {
            site,
            witness: ExplicitReturnWitnessV1::Unit,
        }
    }

    #[cfg(test)]
    pub(super) fn site(&self) -> &SourceStmtSiteV1 {
        &self.site
    }

    #[cfg(test)]
    pub(super) fn witness(&self) -> ExplicitReturnWitnessV1 {
        self.witness
    }
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
        let mut expected_sites = std::collections::BTreeSet::new();
        for site in completion.explicit_sites() {
            if !expected_sites.insert(site.clone()) {
                return Err(
                    "[freeze:contract][canonical_completion/duplicate_expected_site]".to_string(),
                );
            }
        }
        Ok(Self {
            explicit_claims: vec![None; completion.explicit_sites().len()].into_boxed_slice(),
            completion,
        })
    }

    fn claim_slot(
        &self,
        site: &SourceStmtSiteV1,
        target_function: RegionId,
    ) -> Result<usize, String> {
        if self.completion.target_function() != target_function {
            return Err("[freeze:contract][canonical_completion/target_mismatch]".to_string());
        }
        self.completion
            .explicit_sites()
            .iter()
            .position(|expected| expected == site)
            .ok_or_else(|| {
                "[freeze:contract][canonical_completion/explicit_site_mismatch]".to_string()
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
        let index = self.claim_slot(site, target_function)?;
        if self.explicit_claims[index].is_some() {
            return Err("[freeze:contract][canonical_completion/explicit_reconsumed]".to_string());
        }
        self.explicit_claims[index] =
            Some(ExplicitReturnClaimV1::value(site.clone(), block, value));
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
        let index = self.claim_slot(site, target_function)?;
        if self.explicit_claims[index].is_some() {
            return Err("[freeze:contract][canonical_completion/explicit_reconsumed]".to_string());
        }
        self.explicit_claims[index] = Some(ExplicitReturnClaimV1::unit(site.clone()));
        Ok(())
    }

    pub(super) fn finish(
        self,
        root_body: &SourceBodySiteV1,
        root_body_end: u32,
        target_function: RegionId,
    ) -> Result<ReadyFunctionCompletionV1, String> {
        if self.completion.target_function() != target_function {
            return Err(
                "[freeze:contract][canonical_completion/finish_target_mismatch]".to_string(),
            );
        }
        let expected_count = self.completion.explicit_sites().len();
        if self
            .explicit_claims
            .iter()
            .filter(|claim| claim.is_some())
            .count()
            != expected_count
        {
            return Err("[freeze:contract][canonical_completion/consumption_mismatch]".to_string());
        }
        if let Some((expected_body, expected_end)) = self.completion.implicit_body_end() {
            if expected_body != root_body || expected_end != root_body_end {
                return Err(
                    "[freeze:contract][canonical_completion/implicit_body_mismatch]".to_string(),
                );
            }
        }
        let explicit_claims = self
            .explicit_claims
            .into_vec()
            .into_iter()
            .collect::<Option<Vec<_>>>()
            .ok_or_else(|| {
                "[freeze:contract][canonical_completion/operand_witness_missing]".to_string()
            })?
            .into_boxed_slice();
        Ok(ReadyFunctionCompletionV1 {
            completion: self.completion,
            explicit_claims,
        })
    }
}
