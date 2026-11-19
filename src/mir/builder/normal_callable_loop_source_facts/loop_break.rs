//! Source/Facts co-seal for the bounded direct LoopBreak profile.
//!
//! This owner joins one resolver-issued loop membership with the existing
//! LoopBreak planner outcome and scheduler terminality. It does not lower a
//! route, allocate a Builder, or infer a candidate from names.

use super::composite::{
    issue_composite_source_candidate_v1, CompositeLoopBreakSourceFactsIssueV1,
    VerifiedCallableLoopBreakCompositeSourceCandidateV1,
};
use crate::ast::ASTNode;
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::compiler::located::LocatedStmtV1;
use crate::mir::compiler::loop_break_source_projection::{
    issue_loop_break_source_projection_v1, LoopBreakSourceProjectionRejectV1,
    VerifiedLoopBreakSourceProjectionV1,
};
use crate::mir::resolved_semantics::{FunctionOriginV1, FunctionOwnerIdV1};

use super::super::control_flow::plan::single_planner::{self, CallableLoopFactsPlannerInputV1};
use super::super::control_flow::plan::LoopFactsPolicyFrameV1;
use crate::mir::loop_structural_facts::{
    LoopRootSourceBindingRejectV1, LoopSourceForestBindingRejectV1,
};

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir) enum CallableLoopBreakSourceFactsIssueV1 {
    SourceLedger(Box<str>),
    SourceNavigation(Box<str>),
    Planner(Box<str>),
    Projection(LoopBreakSourceProjectionRejectV1),
    Composite(CompositeLoopBreakSourceFactsIssueV1),
}

/// Resolver membership of a direct candidate in the package. Kept for
/// source ownership and overlap checks until the composite selector is retired.
#[derive(Debug)]
pub(in crate::mir) struct VerifiedCallableLoopBreakSourceCandidateV1 {
    projection: VerifiedLoopBreakSourceProjectionV1,
}

impl VerifiedCallableLoopBreakSourceCandidateV1 {
    pub(in crate::mir) fn projection(&self) -> &VerifiedLoopBreakSourceProjectionV1 {
        &self.projection
    }
}

#[derive(Debug)]
pub(in crate::mir) struct VerifiedCallableLoopBreakSourceFactsV1 {
    owner: FunctionOwnerIdV1,
    function_origin: FunctionOriginV1,
    candidates: Box<[VerifiedCallableLoopBreakSourceCandidateV1]>,
}

impl VerifiedCallableLoopBreakSourceFactsV1 {
    pub(in crate::mir) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(in crate::mir) const fn function_origin(&self) -> FunctionOriginV1 {
        self.function_origin
    }

    pub(in crate::mir) fn candidates(&self) -> &[VerifiedCallableLoopBreakSourceCandidateV1] {
        &self.candidates
    }

    pub(in crate::mir) fn take_candidate_for_site(
        &mut self,
        site: &crate::mir::resolved_semantics::SourceStmtSiteV1,
    ) -> Option<VerifiedCallableLoopBreakSourceCandidateV1> {
        let mut candidates = std::mem::take(&mut self.candidates).into_vec();
        let Some(index) = candidates
            .iter()
            .position(|candidate| candidate.projection.loop_site() == site)
        else {
            self.candidates = candidates.into_boxed_slice();
            return None;
        };
        let candidate = candidates.remove(index);
        self.candidates = candidates.into_boxed_slice();
        Some(candidate)
    }
}

#[derive(Debug)]
pub(in crate::mir) struct VerifiedCallableLoopBreakCompositeSourceFactsV1 {
    owner: FunctionOwnerIdV1,
    function_origin: FunctionOriginV1,
    candidates: Box<[VerifiedCallableLoopBreakCompositeSourceCandidateV1]>,
}

impl VerifiedCallableLoopBreakCompositeSourceFactsV1 {
    pub(in crate::mir) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(in crate::mir) const fn function_origin(&self) -> FunctionOriginV1 {
        self.function_origin
    }

    pub(in crate::mir) fn candidates(
        &self,
    ) -> &[VerifiedCallableLoopBreakCompositeSourceCandidateV1] {
        &self.candidates
    }

    pub(in crate::mir) fn validate_package_candidate(&self) -> Result<(), String> {
        for candidate in &self.candidates {
            let projection = candidate.projection();
            if projection.owner() != self.owner
                || projection.function_origin() != self.function_origin
                || !candidate.recipe_is_nonempty()
            {
                return Err(
                    "[freeze:contract][callable-loop-break/composite-package-relation]".to_owned(),
                );
            }
        }
        Ok(())
    }

    pub(in crate::mir) fn take_candidate_for_site(
        &mut self,
        site: &crate::mir::resolved_semantics::SourceStmtSiteV1,
    ) -> Option<VerifiedCallableLoopBreakCompositeSourceCandidateV1> {
        let mut candidates = std::mem::take(&mut self.candidates).into_vec();
        let Some(index) = candidates
            .iter()
            .position(|candidate| candidate.projection().loop_site() == site)
        else {
            self.candidates = candidates.into_boxed_slice();
            return None;
        };
        let candidate = candidates.remove(index);
        if !candidate.recipe_is_nonempty() {
            candidates.insert(index, candidate);
            self.candidates = candidates.into_boxed_slice();
            return None;
        }
        self.candidates = candidates.into_boxed_slice();
        Some(candidate)
    }
}

#[derive(Debug)]
pub(in crate::mir) enum CallableLoopBreakSourceFactsDispositionV1 {
    Candidate(VerifiedCallableLoopBreakSourceFactsV1),
    CompositeCandidate(VerifiedCallableLoopBreakCompositeSourceFactsV1),
    SupportedNonCandidate {
        owner: FunctionOwnerIdV1,
        loop_count: usize,
    },
    Unresolved {
        owner: FunctionOwnerIdV1,
        issue: CallableLoopBreakSourceFactsIssueV1,
    },
    Rejected {
        owner: FunctionOwnerIdV1,
        issue: CallableLoopBreakSourceFactsIssueV1,
    },
}

fn unsupported_shape(error: &LoopBreakSourceProjectionRejectV1) -> bool {
    matches!(
        error,
        LoopBreakSourceProjectionRejectV1::Forest(
            crate::mir::compiler::loop_cond_break_continue_projection::
                LoopCondBreakContinueForestProjectionRejectV1::ForestLookup,
        )
            | LoopBreakSourceProjectionRejectV1::Forest(
                crate::mir::compiler::loop_cond_break_continue_projection::
                    LoopCondBreakContinueForestProjectionRejectV1::ForestBinding(
                        LoopSourceForestBindingRejectV1::Source {
                            reason: LoopRootSourceBindingRejectV1::UnsupportedAncestor { .. },
                            ..
                        },
                    ),
            )
            | LoopBreakSourceProjectionRejectV1::ScopeBox
            | LoopBreakSourceProjectionRejectV1::BodyArity
            | LoopBreakSourceProjectionRejectV1::BreakIfShape
            | LoopBreakSourceProjectionRejectV1::BreakIfBodyArity
            | LoopBreakSourceProjectionRejectV1::BreakIfElseBody
            | LoopBreakSourceProjectionRejectV1::CarrierUpdateShape
            | LoopBreakSourceProjectionRejectV1::StepShape
    )
}

fn source_loop_parts<'source>(
    input: ResolvedFunctionLoweringInputV1<'source>,
    loop_stmt: &LocatedStmtV1<'source>,
) -> Result<(ASTNode, Vec<ASTNode>), CallableLoopBreakSourceFactsIssueV1> {
    let condition = input
        .source()
        .child_expr_from_stmt(
            loop_stmt,
            crate::mir::resolved_semantics::ExprChildRoleV1::LoopCondition,
        )
        .map_err(|error| {
            CallableLoopBreakSourceFactsIssueV1::SourceNavigation(error.to_string().into())
        })?;
    let body = input
        .source()
        .child_body_from_stmt(
            loop_stmt,
            crate::mir::resolved_semantics::BodyChildRoleV1::LoopBody,
        )
        .map_err(|error| {
            CallableLoopBreakSourceFactsIssueV1::SourceNavigation(error.to_string().into())
        })?;
    Ok((condition.node().clone(), body.statements().to_vec()))
}

/// Direct three-site LoopBreak topology gate: the loop body is exactly the
/// break-if / carrier-update / step sites in order with no scope-box children.
fn has_direct_loop_break_topology(
    facts: &super::super::control_flow::plan::facts::LoopFacts,
) -> bool {
    let Some(loop_break) = facts.loop_break() else {
        return false;
    };
    let Some(topology) = loop_break.source_topology.as_ref() else {
        return false;
    };
    let break_if = topology.break_if();
    let carrier_update = topology.carrier_update();
    let step = topology.step();
    facts.source_receipt().raw_body_statement_count() == Some(3)
        && break_if.raw_body_index() == 0
        && carrier_update.raw_body_index() == 1
        && step.raw_body_index() == 2
        && break_if.raw_body_index() != carrier_update.raw_body_index()
        && carrier_update.raw_body_index() != step.raw_body_index()
        && break_if.scope_box_children().is_empty()
        && carrier_update.scope_box_children().is_empty()
        && step.scope_box_children().is_empty()
}

/// Observe every resolver loop in one callable and retain only the direct
/// source-backed LoopBreak candidates. Empty/unsupported shapes are explicit
/// non-candidates; missing resolver/planner evidence is never collapsed into
/// `None`.
pub(in crate::mir) fn issue_callable_loop_break_source_facts_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
    policy: LoopFactsPolicyFrameV1,
) -> CallableLoopBreakSourceFactsDispositionV1 {
    let owner = input.owner();
    let ledger = match input.forest().callable_source_ledger(owner) {
        Ok(ledger) => ledger,
        Err(error) => {
            return CallableLoopBreakSourceFactsDispositionV1::Unresolved {
                owner,
                issue: CallableLoopBreakSourceFactsIssueV1::SourceLedger(
                    format!("{error:?}").into(),
                ),
            }
        }
    };
    let loop_sites = ledger.loop_sites().cloned().collect::<Vec<_>>();
    if loop_sites.is_empty() {
        return CallableLoopBreakSourceFactsDispositionV1::SupportedNonCandidate {
            owner,
            loop_count: 0,
        };
    }

    let mut candidates = Vec::new();
    let mut composite_candidates = Vec::new();
    for site in loop_sites.iter() {
        let membership = match ledger.resolved_loop_source(site) {
            Ok(membership) => membership,
            Err(error) => {
                return CallableLoopBreakSourceFactsDispositionV1::Unresolved {
                    owner,
                    issue: CallableLoopBreakSourceFactsIssueV1::SourceLedger(
                        format!("{error:?}").into(),
                    ),
                }
            }
        };
        let loop_stmt = match input.source().stmt_at(&membership) {
            Ok(loop_stmt) => loop_stmt,
            Err(error) => {
                return CallableLoopBreakSourceFactsDispositionV1::Unresolved {
                    owner,
                    issue: CallableLoopBreakSourceFactsIssueV1::SourceNavigation(
                        error.to_string().into(),
                    ),
                }
            }
        };
        let (condition, body) = match source_loop_parts(input, &loop_stmt) {
            Ok(parts) => parts,
            Err(issue) => {
                return CallableLoopBreakSourceFactsDispositionV1::Unresolved { owner, issue }
            }
        };
        let (resolved_source, _, _) = membership.into_parts();
        let projection = match issue_loop_break_source_projection_v1(
            input,
            &loop_stmt,
            resolved_source,
        ) {
            Ok(projection) => projection,
            Err(error) if unsupported_shape(&error) => {
                let composite_membership = match ledger.resolved_loop_source(site) {
                    Ok(membership) => membership,
                    Err(error) => {
                        return CallableLoopBreakSourceFactsDispositionV1::Unresolved {
                            owner,
                            issue: CallableLoopBreakSourceFactsIssueV1::SourceLedger(
                                format!("{error:?}").into(),
                            ),
                        }
                    }
                };
                let (composite_source, _, _) = composite_membership.into_parts();
                match issue_composite_source_candidate_v1(input, &loop_stmt, composite_source) {
                        Ok(candidate) => composite_candidates.push(candidate),
                        Err(CompositeLoopBreakSourceFactsIssueV1::Projection(
                            crate::mir::compiler::loop_break_composite_source_projection::LoopBreakCompositeSourceProjectionRejectV1::RootExitMissing,
                        )) => {}
                        Err(issue) => {
                            return CallableLoopBreakSourceFactsDispositionV1::Rejected {
                                owner,
                                issue: CallableLoopBreakSourceFactsIssueV1::Composite(issue),
                            }
                        }
                    }
                continue;
            }
            Err(error) => {
                return CallableLoopBreakSourceFactsDispositionV1::Rejected {
                    owner,
                    issue: CallableLoopBreakSourceFactsIssueV1::Projection(error),
                }
            }
        };
        let planner_input = CallableLoopFactsPlannerInputV1::new(
            &condition,
            &body,
            policy,
            "callable-loopbreak-source-package".into(),
            policy.debug_enabled(),
        );
        let outcome = match single_planner::try_build_source_outcome(planner_input) {
            Ok(outcome) => outcome,
            Err(error) => {
                return CallableLoopBreakSourceFactsDispositionV1::Rejected {
                    owner,
                    issue: CallableLoopBreakSourceFactsIssueV1::Planner(error.into_boxed_str()),
                }
            }
        };
        let Some(facts) = outcome.facts.as_ref() else {
            continue;
        };
        if !has_direct_loop_break_topology(&facts.facts) {
            continue;
        }
        candidates.push(VerifiedCallableLoopBreakSourceCandidateV1 { projection });
    }

    if !candidates.is_empty() && !composite_candidates.is_empty() {
        return CallableLoopBreakSourceFactsDispositionV1::Rejected {
            owner,
            issue: CallableLoopBreakSourceFactsIssueV1::Composite(
                CompositeLoopBreakSourceFactsIssueV1::Recipe(
                    "[freeze:contract][callable-loopbreak/mixed-direct-composite]".into(),
                ),
            ),
        };
    }
    if !candidates.is_empty() {
        let function = input.function();
        CallableLoopBreakSourceFactsDispositionV1::Candidate(
            VerifiedCallableLoopBreakSourceFactsV1 {
                owner,
                function_origin: function.function_origin(),
                candidates: candidates.into_boxed_slice(),
            },
        )
    } else if !composite_candidates.is_empty() {
        let function = input.function();
        CallableLoopBreakSourceFactsDispositionV1::CompositeCandidate(
            VerifiedCallableLoopBreakCompositeSourceFactsV1 {
                owner,
                function_origin: function.function_origin(),
                candidates: composite_candidates.into_boxed_slice(),
            },
        )
    } else {
        CallableLoopBreakSourceFactsDispositionV1::SupportedNonCandidate {
            owner,
            loop_count: loop_sites.len(),
        }
    }
}
