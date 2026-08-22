//! Exact completion consumption for the canonical draft-seal handoff.

use crate::mir::compiler::located::SourceBodySiteV1;
use crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1;
use crate::mir::resolved_semantics::{FunctionOwnerIdV1, RegionId, SourceStmtSiteV1};
use crate::mir::{BasicBlockId, ValueId};

#[derive(Debug)]
pub(super) struct ResolvedFunctionCompletionConsumptionV1 {
    expected: CompletionExpectationV1,
    /// Slots are indexed by the resolver-owned `explicit_sites()` order.
    /// The site is still carried in every claim so a future consumer cannot
    /// silently treat storage order as source identity.
    explicit_claims: Box<[Option<ExplicitReturnClaimV1>]>,
}

/// Borrow-free physical expectations copied from the sole semantic
/// Completion owner at the admission boundary.  Keeping only the facts the
/// physical consumer needs prevents a borrowed semantic product from leaking
/// into DraftSeal while still allowing selected Dynamic lowering to borrow
/// the installed package exactly once.
#[derive(Debug, Clone, PartialEq, Eq)]
struct CompletionExpectationV1 {
    owner: FunctionOwnerIdV1,
    target_function: RegionId,
    kind: CompletionPhysicalKindV1,
    explicit_sites: Box<[SourceStmtSiteV1]>,
    implicit_body_end: Option<(SourceBodySiteV1, u32)>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CompletionPhysicalKindV1 {
    ExplicitValue,
    ExplicitUnit,
    ImplicitVoid,
}

/// Temporal witness minted only after every current canonical Lower finish.
///
/// The future SSA-I1 finish slots before this witness without changing the
/// finalizer API. Raw pre-Builder completion products cannot finalize a draft.
#[derive(Debug)]
pub(super) struct ReadyFunctionCompletionV1 {
    owner: FunctionOwnerIdV1,
    kind: CompletionPhysicalKindV1,
    explicit_claims: Box<[ExplicitReturnClaimV1]>,
}

impl ReadyFunctionCompletionV1 {
    pub(super) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

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
        matches!(self.kind, CompletionPhysicalKindV1::ExplicitValue)
    }

    pub(super) fn is_implicit_void(&self) -> bool {
        matches!(self.kind, CompletionPhysicalKindV1::ImplicitVoid)
    }

    /// Exact site-keyed physical claims in the resolver's canonical source
    /// order.  A multi-site caller must consume this complete set; the
    /// single-operand helper above intentionally returns `None` for it.
    pub(super) fn explicit_claims(&self) -> &[ExplicitReturnClaimV1] {
        &self.explicit_claims
    }
}

/// Builder-side evidence for one explicit source exit.  The completion
/// consumer retains a complete site-keyed set and the DraftSeal writer
/// consumes the same set for one- or two-site exits.
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

    pub(super) fn site(&self) -> &SourceStmtSiteV1 {
        &self.site
    }

    pub(super) fn witness(&self) -> ExplicitReturnWitnessV1 {
        self.witness
    }

    #[cfg(test)]
    pub(super) fn from_test_value(
        site: SourceStmtSiteV1,
        block: BasicBlockId,
        value: ValueId,
    ) -> Self {
        Self::value(site, block, value)
    }

    #[cfg(test)]
    pub(super) fn from_test_unit(site: SourceStmtSiteV1) -> Self {
        Self::unit(site)
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
    pub(super) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.expected.owner
    }

    pub(super) fn returns_value(&self) -> bool {
        matches!(self.expected.kind, CompletionPhysicalKindV1::ExplicitValue)
    }

    pub(super) fn is_implicit_void(&self) -> bool {
        matches!(self.expected.kind, CompletionPhysicalKindV1::ImplicitVoid)
    }

    pub(super) fn new(
        expected_owner: FunctionOwnerIdV1,
        completion: VerifiedFunctionCompletionV1,
    ) -> Result<Self, String> {
        Self::project(expected_owner, &completion)
    }

    /// Borrowed selected-Dynamic admission.  Only the exact physical
    /// expectations are copied; the semantic Completion remains owned by the
    /// installed package and cannot escape this call.
    pub(super) fn new_borrowed(
        expected_owner: FunctionOwnerIdV1,
        completion: &VerifiedFunctionCompletionV1,
    ) -> Result<Self, String> {
        Self::project(expected_owner, completion)
    }

    fn project(
        expected_owner: FunctionOwnerIdV1,
        completion: &VerifiedFunctionCompletionV1,
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
        let kind = if completion.returns_value() {
            CompletionPhysicalKindV1::ExplicitValue
        } else if completion.is_implicit_void() {
            CompletionPhysicalKindV1::ImplicitVoid
        } else {
            CompletionPhysicalKindV1::ExplicitUnit
        };
        let expected = CompletionExpectationV1 {
            owner: completion.owner(),
            target_function: completion.target_function(),
            kind,
            explicit_sites: completion.explicit_sites().to_vec().into_boxed_slice(),
            implicit_body_end: completion
                .implicit_body_end()
                .map(|(body, end)| (body.clone(), end)),
        };
        Ok(Self {
            explicit_claims: vec![None; completion.explicit_sites().len()].into_boxed_slice(),
            expected,
        })
    }

    fn claim_slot(
        &self,
        site: &SourceStmtSiteV1,
        target_function: RegionId,
    ) -> Result<usize, String> {
        if self.expected.target_function != target_function {
            return Err("[freeze:contract][canonical_completion/target_mismatch]".to_string());
        }
        self.expected
            .explicit_sites
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
        if !self.returns_value() {
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
        if self.returns_value() {
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
        if self.expected.target_function != target_function {
            return Err(
                "[freeze:contract][canonical_completion/finish_target_mismatch]".to_string(),
            );
        }
        let expected_count = self.expected.explicit_sites.len();
        if self
            .explicit_claims
            .iter()
            .filter(|claim| claim.is_some())
            .count()
            != expected_count
        {
            return Err("[freeze:contract][canonical_completion/consumption_mismatch]".to_string());
        }
        if let Some((expected_body, expected_end)) = self.expected.implicit_body_end.as_ref() {
            if expected_body != root_body || *expected_end != root_body_end {
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
        let kind = self.expected.kind;
        Ok(ReadyFunctionCompletionV1 {
            owner: self.expected.owner,
            kind,
            explicit_claims,
        })
    }
}

#[cfg(test)]
impl ReadyFunctionCompletionV1 {
    pub(super) fn from_test_explicit_value(
        owner: FunctionOwnerIdV1,
        claims: Box<[ExplicitReturnClaimV1]>,
    ) -> Self {
        Self {
            owner,
            kind: CompletionPhysicalKindV1::ExplicitValue,
            explicit_claims: claims,
        }
    }

    pub(super) fn from_test_explicit_unit(
        owner: FunctionOwnerIdV1,
        claims: Box<[ExplicitReturnClaimV1]>,
    ) -> Self {
        Self {
            owner,
            kind: CompletionPhysicalKindV1::ExplicitUnit,
            explicit_claims: claims,
        }
    }

    pub(super) fn from_test_implicit_void(owner: FunctionOwnerIdV1) -> Self {
        Self {
            owner,
            kind: CompletionPhysicalKindV1::ImplicitVoid,
            explicit_claims: Box::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ResolvedFunctionCompletionConsumptionV1;
    use crate::ast::{ASTNode, DeclarationAttrs, LiteralValue, Span};
    use crate::mir::compiler::VerifiedResolvedSourceUnitV1;
    use crate::mir::resolved_control_flow::verify_function_completion_v1;

    fn function(name: &str) -> ASTNode {
        ASTNode::FunctionDeclaration {
            name: name.into(),
            params: Vec::new(),
            param_decls: Vec::new(),
            return_type_name: None,
            body: vec![ASTNode::Return {
                value: Some(Box::new(ASTNode::Literal {
                    value: LiteralValue::Integer(1),
                    span: Span::unknown(),
                })),
                span: Span::unknown(),
            }],
            uses: Vec::new(),
            contracts: Vec::new(),
            is_static: true,
            is_override: false,
            attrs: DeclarationAttrs::default(),
            span: Span::unknown(),
        }
    }

    #[test]
    fn borrowed_completion_projects_to_a_borrow_free_consumer() {
        let unit = VerifiedResolvedSourceUnitV1::resolve_function(function("borrowed_completion"))
            .expect("resolved unit");
        let input = unit.root_function_input().expect("root input");
        let completion = verify_function_completion_v1(input).expect("completion");
        let owner = input.owner();
        let target = input.function().lowering_roots().function_pair().region();
        let body = input.source().root_body().expect("root body");
        let site = completion
            .explicit_sites()
            .first()
            .expect("explicit return site")
            .clone();

        let mut consumer =
            ResolvedFunctionCompletionConsumptionV1::new_borrowed(owner, &completion)
                .expect("borrowed completion");
        assert_eq!(consumer.owner(), owner);
        assert!(consumer.returns_value());
        consumer
            .claim_explicit_return(
                &site,
                target,
                crate::mir::BasicBlockId::new(1),
                crate::mir::ValueId::new(2),
            )
            .expect("claim");
        let ready = consumer
            .finish(body.site(), body.statements().len() as u32, target)
            .expect("borrow-free ready completion");
        assert!(ready.returns_value());
        assert_eq!(ready.explicit_claims().len(), 1);
    }

    #[test]
    fn borrowed_completion_rejects_a_foreign_owner() {
        let first = VerifiedResolvedSourceUnitV1::resolve_function(function("first_completion"))
            .expect("first unit");
        let second = VerifiedResolvedSourceUnitV1::resolve_function(function("second_completion"))
            .expect("second unit");
        let first_input = first.root_function_input().expect("first input");
        let second_input = second.root_function_input().expect("second input");
        let completion = verify_function_completion_v1(first_input).expect("completion");
        let error = ResolvedFunctionCompletionConsumptionV1::new_borrowed(
            second_input.owner(),
            &completion,
        )
        .expect_err("foreign owner must reject");
        assert!(error.contains("owner_mismatch"));
    }
}
