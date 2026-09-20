//! Source/Facts co-seal for the bounded direct LoopBreak profile.
//!
//! This owner joins one resolver-issued loop membership with the existing
//! LoopBreak planner outcome and scheduler terminality. It does not lower a
//! route, allocate a Builder, or infer a candidate from names.

use crate::ast::ASTNode;
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::compiler::located::LocatedStmtV1;
use crate::mir::compiler::loop_break_source_projection::{
    issue_loop_break_source_projection_v1, LoopBreakSourceProjectionRejectV1,
    VerifiedLoopBreakSourceProjectionV1,
};
use crate::mir::resolved_semantics::{
    FunctionOriginV1, FunctionOwnerIdV1, SemanticOwnerSourceKindV1,
};

use super::super::control_flow::joinir::route_entry::registry::{
    certify_direct_loop_break_terminality, DirectLoopBreakTerminalityV1,
};
use super::super::control_flow::plan::planner::PlanBuildOutcome;
use super::super::control_flow::plan::single_planner::{self, CallableLoopFactsPlannerInputV1};
use super::super::control_flow::plan::GenericLoopFactsPolicyFrameV1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir) enum CallableLoopBreakSourceFactsIssueV1 {
    SourceLedger(Box<str>),
    SourceNavigation(Box<str>),
    Planner(Box<str>),
    Projection(LoopBreakSourceProjectionRejectV1),
}

/// One source-aligned direct LoopBreak candidate retained for the later
/// physical consumer. The planner outcome and terminality proof are existing
/// products; this type only keeps them paired with the resolver projection.
#[derive(Debug, Clone)]
pub(in crate::mir) struct VerifiedCallableLoopBreakSourceCandidateV1 {
    owner: FunctionOwnerIdV1,
    function_origin: FunctionOriginV1,
    source_kind: SemanticOwnerSourceKindV1,
    projection: VerifiedLoopBreakSourceProjectionV1,
    outcome: PlanBuildOutcome,
    terminality: DirectLoopBreakTerminalityV1,
}

impl VerifiedCallableLoopBreakSourceCandidateV1 {
    pub(in crate::mir) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(in crate::mir) const fn function_origin(&self) -> FunctionOriginV1 {
        self.function_origin
    }

    pub(in crate::mir) const fn source_kind(&self) -> SemanticOwnerSourceKindV1 {
        self.source_kind
    }

    pub(in crate::mir) fn projection(&self) -> &VerifiedLoopBreakSourceProjectionV1 {
        &self.projection
    }

    pub(in crate::mir::builder) fn outcome(&self) -> &PlanBuildOutcome {
        &self.outcome
    }

    pub(in crate::mir) fn terminality(&self) -> &DirectLoopBreakTerminalityV1 {
        &self.terminality
    }
}

#[derive(Debug, Clone)]
pub(in crate::mir) struct VerifiedCallableLoopBreakSourceFactsV1 {
    owner: FunctionOwnerIdV1,
    function_origin: FunctionOriginV1,
    source_kind: SemanticOwnerSourceKindV1,
    candidates: Box<[VerifiedCallableLoopBreakSourceCandidateV1]>,
}

impl VerifiedCallableLoopBreakSourceFactsV1 {
    pub(in crate::mir) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(in crate::mir) const fn function_origin(&self) -> FunctionOriginV1 {
        self.function_origin
    }

    pub(in crate::mir) const fn source_kind(&self) -> SemanticOwnerSourceKindV1 {
        self.source_kind
    }

    pub(in crate::mir) fn candidates(&self) -> &[VerifiedCallableLoopBreakSourceCandidateV1] {
        &self.candidates
    }
}

#[derive(Debug, Clone)]
pub(in crate::mir) enum CallableLoopBreakSourceFactsDispositionV1 {
    Candidate(VerifiedCallableLoopBreakSourceFactsV1),
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
        LoopBreakSourceProjectionRejectV1::ScopeBox
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

/// Observe every resolver loop in one callable and retain only the direct
/// source-backed LoopBreak candidates. Empty/unsupported shapes are explicit
/// non-candidates; missing resolver/planner evidence is never collapsed into
/// `None`.
pub(in crate::mir) fn issue_callable_loop_break_source_facts_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
    policy: GenericLoopFactsPolicyFrameV1,
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
        let projection =
            match issue_loop_break_source_projection_v1(input, &loop_stmt, resolved_source) {
                Ok(projection) => projection,
                Err(error) if unsupported_shape(&error) => continue,
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
        let Some(loop_break) = facts.facts.loop_break() else {
            continue;
        };
        let Some(terminality) = certify_direct_loop_break_terminality(&facts.facts) else {
            continue;
        };
        if loop_break.source_topology.is_none() {
            continue;
        }
        candidates.push(VerifiedCallableLoopBreakSourceCandidateV1 {
            owner,
            function_origin: projection.function_origin(),
            source_kind: projection.source_kind(),
            projection,
            outcome,
            terminality,
        });
    }

    if candidates.is_empty() {
        CallableLoopBreakSourceFactsDispositionV1::SupportedNonCandidate {
            owner,
            loop_count: loop_sites.len(),
        }
    } else {
        let function = input.function();
        CallableLoopBreakSourceFactsDispositionV1::Candidate(
            VerifiedCallableLoopBreakSourceFactsV1 {
                owner,
                function_origin: function.function_origin(),
                source_kind: function.source_kind(),
                candidates: candidates.into_boxed_slice(),
            },
        )
    }
}
