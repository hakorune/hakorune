//! Exact resolver-owned Exit/Tail co-seal for forward ScanWithInit.
//!
//! This owner consumes the existing source-bound call relation and callable
//! Completion. It never rereads AST, infers from source order, or issues
//! Recipe keys. The later S6C Facts owner must consume this product whole.

use crate::mir::callable_semantic_batch::{S6CBinaryRoleV1, S6CTypedInputRoleV1};
use crate::mir::resolved_control_flow::{
    FunctionExitCoverageV1, SealedFunctionExitDispositionV1, VerifiedFunctionCompletionV1,
};
use crate::mir::resolved_semantics::{
    CallableSemanticSourceLedgerView, ResolvedControlTransferV1, ResolvedExitOriginV1,
    ResolvedExitRecordV1, ResolvedExitSiteV1, ResolvedIfRegionLookupErrorV1, ResolvedLexicalRefV1,
    ResolvedLiteralSourceV1, ResolvedUnaryOperatorV1, SourceExprSiteV1, SourcePathSegmentV1,
    SourcePathV1, SourceStmtSiteV1,
};
use crate::mir::source_call_target::{
    S6CSourceBoundCallRelationRefV1, VerifiedSourceBoundS6CCallRelationV1,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum S6CExitRoleV1 {
    LoopReturn,
    CallableTail,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum S6CExitTailSourceCoSealRejectV1 {
    ForeignOwner,
    ForeignLoopMembership,
    CompletionShape,
    CompletionCoverage,
    CompletionCleanup,
    ExitCoverage { actual: usize },
    ExitSiteCoverage,
    WrongExitShape(S6CExitRoleV1),
    WrongExitRegion(S6CExitRoleV1),
    WrongExitValue(S6CExitRoleV1),
    MissingTypedRole,
    MissingTextEqual,
    IfRegion(ResolvedIfRegionLookupErrorV1),
    IfOutsideLoop,
}

/// Borrow-only view over the non-splittable S6C source co-seal.
#[derive(Debug, Clone, Copy)]
pub(crate) struct S6CExitTailSourceCoSealRefV1<'a> {
    calls: S6CSourceBoundCallRelationRefV1<'a>,
    completion: &'a VerifiedFunctionCompletionV1,
    if_site: &'a SourceStmtSiteV1,
    loop_return_site: &'a SourceStmtSiteV1,
    loop_return_value: &'a SourceExprSiteV1,
    tail_site: &'a SourceStmtSiteV1,
    tail_value: &'a SourceExprSiteV1,
    tail_operand: &'a SourceExprSiteV1,
}

impl<'a> S6CExitTailSourceCoSealRefV1<'a> {
    pub(crate) const fn calls(self) -> S6CSourceBoundCallRelationRefV1<'a> {
        self.calls
    }

    pub(crate) const fn completion(self) -> &'a VerifiedFunctionCompletionV1 {
        self.completion
    }

    pub(crate) const fn if_site(self) -> &'a SourceStmtSiteV1 {
        self.if_site
    }

    pub(crate) const fn loop_return_site(self) -> &'a SourceStmtSiteV1 {
        self.loop_return_site
    }

    pub(crate) const fn loop_return_value(self) -> &'a SourceExprSiteV1 {
        self.loop_return_value
    }

    pub(crate) const fn tail_site(self) -> &'a SourceStmtSiteV1 {
        self.tail_site
    }

    pub(crate) const fn tail_value(self) -> &'a SourceExprSiteV1 {
        self.tail_value
    }

    pub(crate) const fn tail_operand(self) -> &'a SourceExprSiteV1 {
        self.tail_operand
    }
}

/// Non-Clone source product consumed by the future complete S6C Facts owner.
#[derive(Debug)]
pub(crate) struct VerifiedS6CExitTailSourceCoSealV1 {
    calls: VerifiedSourceBoundS6CCallRelationV1,
    completion: VerifiedFunctionCompletionV1,
    if_site: SourceStmtSiteV1,
    loop_return_site: SourceStmtSiteV1,
    loop_return_value: SourceExprSiteV1,
    tail_site: SourceStmtSiteV1,
    tail_value: SourceExprSiteV1,
    tail_operand: SourceExprSiteV1,
}

impl VerifiedS6CExitTailSourceCoSealV1 {
    pub(crate) fn with_coseal<R>(
        &self,
        callback: impl for<'source> FnOnce(S6CExitTailSourceCoSealRefV1<'source>) -> R,
    ) -> R {
        self.calls.with_relation(|calls| {
            callback(S6CExitTailSourceCoSealRefV1 {
                calls,
                completion: &self.completion,
                if_site: &self.if_site,
                loop_return_site: &self.loop_return_site,
                loop_return_value: &self.loop_return_value,
                tail_site: &self.tail_site,
                tail_value: &self.tail_value,
                tail_operand: &self.tail_operand,
            })
        })
    }
}

pub(crate) fn issue_s6c_exit_tail_source_coseal_v1(
    ledger: &CallableSemanticSourceLedgerView<'_>,
    calls: VerifiedSourceBoundS6CCallRelationV1,
    completion: VerifiedFunctionCompletionV1,
) -> Result<VerifiedS6CExitTailSourceCoSealV1, S6CExitTailSourceCoSealRejectV1> {
    let (index, text_equal_site, loop_region, source_identity_matches) =
        calls.with_relation(|view| {
            let index = view
                .typed()
                .inputs()
                .iter()
                .find(|input| input.role() == S6CTypedInputRoleV1::Index)
                .map(|input| input.binding());
            let text_equal_site = view
                .typed()
                .binaries()
                .iter()
                .find(|binary| binary.role() == S6CBinaryRoleV1::TextEqual)
                .map(|binary| binary.source().site().clone());
            let membership = view.typed().membership();
            (
                index,
                text_equal_site,
                membership.scope_region().region(),
                membership.source().matches_identity(
                    ledger.function_origin(),
                    ledger.source_kind(),
                    membership.source().site(),
                ),
            )
        });
    let index = index.ok_or(S6CExitTailSourceCoSealRejectV1::MissingTypedRole)?;
    let text_equal_site =
        text_equal_site.ok_or(S6CExitTailSourceCoSealRejectV1::MissingTextEqual)?;
    if index.owner() != ledger.owner() {
        return Err(S6CExitTailSourceCoSealRejectV1::ForeignOwner);
    }
    if !source_identity_matches {
        return Err(S6CExitTailSourceCoSealRejectV1::ForeignLoopMembership);
    }

    require_completion(ledger, &completion)?;
    let (if_site, if_control, then_region) = ledger
        .with_if_region_for_condition(&text_equal_site, |row| {
            (
                row.site().clone(),
                row.bundle().control(),
                row.bundle().then_pair().region(),
            )
        })
        .map_err(S6CExitTailSourceCoSealRejectV1::IfRegion)?;
    if ledger.region_parent(if_control) != Some(loop_region) {
        return Err(S6CExitTailSourceCoSealRejectV1::IfOutsideLoop);
    }

    let exits = ledger.resolved_exits().collect::<Vec<_>>();
    if exits.len() != 2 {
        return Err(S6CExitTailSourceCoSealRejectV1::ExitCoverage {
            actual: exits.len(),
        });
    }
    if completion.explicit_sites().len() != exits.len()
        || exits.iter().any(|(site, _)| match site {
            ResolvedExitSiteV1::Statement(site) => !completion.explicit_sites().contains(site),
            ResolvedExitSiteV1::Expression(_) => true,
        })
    {
        return Err(S6CExitTailSourceCoSealRejectV1::ExitSiteCoverage);
    }

    let mut loop_return = None;
    let mut tail = None;
    for (site, exit) in exits {
        let ResolvedExitSiteV1::Statement(site) = site else {
            return Err(S6CExitTailSourceCoSealRejectV1::CompletionShape);
        };
        require_return_transfer(exit, completion.target_function())?;
        let value = SourcePathV1::from_node(site.node())
            .child(SourcePathSegmentV1::Value)
            .expr();
        let is_index = ledger.variable_ref(&value) == Some(ResolvedLexicalRefV1::Local(index));
        let tail_operand = ledger.unary_source(&value).and_then(|unary| {
            (unary.operator() == ResolvedUnaryOperatorV1::Minus
                && ledger.literal_source(unary.operand())
                    == Some(&ResolvedLiteralSourceV1::Integer(1)))
            .then(|| unary.operand().clone())
        });
        match (is_index, tail_operand) {
            (true, None) if exit.source_region() == then_region && loop_return.is_none() => {
                loop_return = Some((site.clone(), value));
            }
            (false, Some(tail_operand))
                if exit.source_region() == ledger.root_body_region() && tail.is_none() =>
            {
                tail = Some((site.clone(), value, tail_operand));
            }
            (true, None) => {
                return Err(S6CExitTailSourceCoSealRejectV1::WrongExitRegion(
                    S6CExitRoleV1::LoopReturn,
                ))
            }
            (false, Some(_)) => {
                return Err(S6CExitTailSourceCoSealRejectV1::WrongExitRegion(
                    S6CExitRoleV1::CallableTail,
                ))
            }
            _ => {
                return Err(S6CExitTailSourceCoSealRejectV1::WrongExitValue(
                    if exit.source_region() == ledger.root_body_region() {
                        S6CExitRoleV1::CallableTail
                    } else {
                        S6CExitRoleV1::LoopReturn
                    },
                ))
            }
        }
    }
    let (loop_return_site, loop_return_value) = loop_return.ok_or(
        S6CExitTailSourceCoSealRejectV1::WrongExitShape(S6CExitRoleV1::LoopReturn),
    )?;
    let (tail_site, tail_value, tail_operand) = tail.ok_or(
        S6CExitTailSourceCoSealRejectV1::WrongExitShape(S6CExitRoleV1::CallableTail),
    )?;

    Ok(VerifiedS6CExitTailSourceCoSealV1 {
        calls,
        completion,
        if_site,
        loop_return_site,
        loop_return_value,
        tail_site,
        tail_value,
        tail_operand,
    })
}

fn require_completion(
    ledger: &CallableSemanticSourceLedgerView<'_>,
    completion: &VerifiedFunctionCompletionV1,
) -> Result<(), S6CExitTailSourceCoSealRejectV1> {
    if completion.owner() != ledger.owner()
        || completion.target_function() != ledger.function_region()
        || !completion.returns_value()
    {
        return Err(S6CExitTailSourceCoSealRejectV1::CompletionShape);
    }
    if !completion.cleanup().crossed_scopes().is_empty() {
        return Err(S6CExitTailSourceCoSealRejectV1::CompletionCleanup);
    }
    let contract = completion.function_exit_contract();
    if contract.coverage() != (FunctionExitCoverageV1::ExactExplicitReturnSet { count: 2 })
        || !matches!(
            contract.disposition(),
            SealedFunctionExitDispositionV1::ExplicitValueSet { sites } if sites.len() == 2
        )
    {
        return Err(S6CExitTailSourceCoSealRejectV1::CompletionCoverage);
    }
    Ok(())
}

fn require_return_transfer(
    exit: &ResolvedExitRecordV1,
    expected_target: crate::mir::resolved_semantics::RegionId,
) -> Result<(), S6CExitTailSourceCoSealRejectV1> {
    if exit.origin() != ResolvedExitOriginV1::ExplicitReturn
        || exit.transfer()
            != (ResolvedControlTransferV1::Return {
                target_function: expected_target,
            })
    {
        return Err(S6CExitTailSourceCoSealRejectV1::CompletionShape);
    }
    Ok(())
}
