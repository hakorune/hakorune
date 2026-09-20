//! Source/Facts co-seal for the bounded direct LoopBreak profile.
//!
//! This owner joins one resolver-issued loop membership with the existing
//! LoopBreak planner outcome and scheduler terminality. It does not lower a
//! route, allocate a Builder, or infer a candidate from names.

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::facts::canon::cond_block_view::CondBlockView;
use crate::mir::builder::control_flow::plan::recipe_tree::build_loop_break_source_recipe;
use crate::mir::builder::control_flow::plan::LoopPlanExpressionPortV1;
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
use super::super::normal_callable_loop_handoff::CallableLoopReadyBodyOnlyProductV1;
use super::super::normal_callable_loop_source_port::CallableLoopSourceExpressionPortV1;
use super::super::normal_callable_loop_source_route::{
    CallableLoopSourceItemBindingV1, CallableLoopSourceTargetProbeV1,
    CallableLoopSourceTargetRelationV1,
};
use super::super::normal_callable_semantic_lowering_state::CallableSemanticLoweringState;
use super::super::raw_invocation_source_transport::RawInvocationSourceContextV1;
use crate::mir::loop_structural_facts::{
    LoopRootSourceBindingRejectV1, LoopSourceForestBindingRejectV1,
};
use crate::mir::resolved_semantics::SourceNodeSiteV1;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir) enum CallableLoopBreakSourceFactsIssueV1 {
    SourceLedger(Box<str>),
    SourceNavigation(Box<str>),
    Planner(Box<str>),
    Projection(LoopBreakSourceProjectionRejectV1),
}

/// One source-aligned direct LoopBreak candidate retained for the later
/// physical consumer. The planner outcome and terminality proof are existing
/// products; this type only keeps them paired with the resolver projection.
#[derive(Debug)]
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

    pub(in crate::mir::builder) fn into_physical_input<'source, 'ledger>(
        self,
        parent_source: &'source RawInvocationSourceContextV1,
        parent_node: ASTNode,
        condition_source: RawInvocationSourceContextV1,
        body_source: RawInvocationSourceContextV1,
        condition: ASTNode,
        body: Vec<ASTNode>,
        binding_product: CallableLoopReadyBodyOnlyProductV1,
        forest_projection: crate::mir::loop_structural_facts::
            VerifiedLoopCondBreakContinueSourceForestProjectionV1,
        source_items: Box<[CallableLoopSourceItemBindingV1]>,
        source_target_probe: CallableLoopSourceTargetProbeV1,
        source_ledger: &'ledger Rc<RefCell<CallableSemanticLoweringState>>,
    ) -> Result<SourceLoopBreakPhysicalInputV1<'source, 'ledger>, String> {
        SourceLoopBreakPhysicalInputV1::from_candidate(
            self,
            parent_source,
            parent_node,
            condition_source,
            body_source,
            condition,
            body,
            binding_product,
            forest_projection,
            source_items,
            source_target_probe,
            source_ledger,
        )
    }
}

/// Source/Facts/Recipe handoff consumed by the existing associated-source
/// LoopV0 physical spine.  The source relations are owned by the same
/// candidate and are validated again before any frame allocation.
#[derive(Debug)]
pub(in crate::mir::builder) struct SourceLoopBreakPhysicalInputV1<'source, 'ledger> {
    owner: FunctionOwnerIdV1,
    function_origin: FunctionOriginV1,
    source_kind: SemanticOwnerSourceKindV1,
    parent_site: SourceNodeSiteV1,
    parent_source: &'source RawInvocationSourceContextV1,
    parent_node: ASTNode,
    condition_source: RawInvocationSourceContextV1,
    body_source: RawInvocationSourceContextV1,
    condition: ASTNode,
    body: Vec<ASTNode>,
    outcome: PlanBuildOutcome,
    projection: VerifiedLoopBreakSourceProjectionV1,
    forest_projection:
        crate::mir::loop_structural_facts::VerifiedLoopCondBreakContinueSourceForestProjectionV1,
    source_items: Box<[CallableLoopSourceItemBindingV1]>,
    source_target: CallableLoopSourceTargetRelationV1,
    source_port: CallableLoopSourceExpressionPortV1<'ledger>,
}

impl<'source, 'ledger> SourceLoopBreakPhysicalInputV1<'source, 'ledger> {
    #[allow(clippy::too_many_arguments)]
    fn from_candidate(
        candidate: VerifiedCallableLoopBreakSourceCandidateV1,
        parent_source: &'source RawInvocationSourceContextV1,
        parent_node: ASTNode,
        condition_source: RawInvocationSourceContextV1,
        body_source: RawInvocationSourceContextV1,
        condition: ASTNode,
        body: Vec<ASTNode>,
        binding_product: CallableLoopReadyBodyOnlyProductV1,
        forest_projection: crate::mir::loop_structural_facts::
            VerifiedLoopCondBreakContinueSourceForestProjectionV1,
        source_items: Box<[CallableLoopSourceItemBindingV1]>,
        source_target_probe: CallableLoopSourceTargetProbeV1,
        source_ledger: &'ledger Rc<RefCell<CallableSemanticLoweringState>>,
    ) -> Result<Self, String> {
        let VerifiedCallableLoopBreakSourceCandidateV1 {
            owner,
            function_origin,
            source_kind,
            projection,
            outcome,
            terminality,
        } = candidate;
        let parent_site = parent_source.site().cloned().ok_or_else(|| {
            "[freeze:contract][callable-loop/loop-break/parent-site-missing]".to_owned()
        })?;
        if projection.owner() != owner
            || projection.function_origin() != function_origin
            || projection.source_kind() != source_kind
            || projection.loop_site().node() != &parent_site
        {
            return Err(
                "[freeze:contract][callable-loop/loop-break/candidate-source-mismatch]".to_owned(),
            );
        }
        if source_ledger.borrow().owner() != owner {
            return Err(
                "[freeze:contract][callable-loop/loop-break/source-port-owner-mismatch]".to_owned(),
            );
        }
        if terminality.route()
            != crate::mir::loop_recipe_contract::route_id::LoopRouteId::LoopBreakRecipe
        {
            return Err(
                "[freeze:contract][callable-loop/loop-break/terminality-route-mismatch]".to_owned(),
            );
        }
        let condition_site = condition_source.site().ok_or_else(|| {
            "[freeze:contract][callable-loop/loop-break/condition-site-missing]".to_owned()
        })?;
        let body_site = body_source.site().ok_or_else(|| {
            "[freeze:contract][callable-loop/loop-break/body-site-missing]".to_owned()
        })?;
        let source_target = source_target_probe
            .into_selected_relation(&source_items)
            .map_err(|error| {
                format!("[freeze:contract][callable-loop/loop-break/source-target] {error:?}")
            })?;
        binding_product
            .consume_pre_effect(&parent_site, condition_site, body_site)
            .map_err(|error| {
                format!("[freeze:contract][callable-loop/loop-break/pre-effect] {error}")
            })?;
        Ok(Self {
            owner,
            function_origin,
            source_kind,
            parent_site,
            parent_source,
            parent_node,
            condition_source,
            body_source,
            condition,
            body,
            outcome,
            projection,
            forest_projection,
            source_items,
            source_target,
            source_port: CallableLoopSourceExpressionPortV1::new(source_ledger),
        })
    }

    pub(in crate::mir::builder) fn parent_source(&self) -> &RawInvocationSourceContextV1 {
        self.parent_source
    }

    pub(in crate::mir::builder) fn parent_node(&self) -> &ASTNode {
        &self.parent_node
    }

    pub(in crate::mir::builder) const fn source_port(
        &self,
    ) -> &CallableLoopSourceExpressionPortV1<'_> {
        &self.source_port
    }

    pub(in crate::mir::builder) fn loop_break_facts(
        &self,
    ) -> Result<&crate::mir::builder::control_flow::plan::facts::LoopBreakFacts, String> {
        self.outcome
            .facts
            .as_ref()
            .and_then(|facts| facts.facts.loop_break())
            .ok_or_else(|| "[freeze:contract][callable-loop/loop-break/facts-missing]".to_owned())
    }

    /// Recheck all source relations before the shared physical owner allocates.
    pub(in crate::mir::builder) fn validate_for_source_port(&self) -> Result<(), String> {
        if self.projection.owner() != self.owner
            || self.forest_projection.owner() != self.owner
            || self.projection.forest() != &self.forest_projection
        {
            return Err(
                "[freeze:contract][callable-loop/loop-break/source-forest-owner-mismatch]"
                    .to_owned(),
            );
        }
        let parent_stmt =
            crate::mir::resolved_semantics::SourceStmtSiteV1::from_node(self.parent_site.clone());
        if !self.projection.matches_source_identity(
            self.function_origin,
            self.source_kind,
            &parent_stmt,
        ) || self.projection.loop_site() != &parent_stmt
        {
            return Err(
                "[freeze:contract][callable-loop/loop-break/source-parent-mismatch]".to_owned(),
            );
        }
        let condition_site = self.condition_source.site().ok_or_else(|| {
            "[freeze:contract][callable-loop/loop-break/condition-site-missing]".to_owned()
        })?;
        let body_site = self.body_source.site().ok_or_else(|| {
            "[freeze:contract][callable-loop/loop-break/body-site-missing]".to_owned()
        })?;
        if !self
            .parent_source
            .shares_root_lineage(&self.condition_source)
            || !self.parent_source.shares_root_lineage(&self.body_source)
            || !condition_site
                .segments()
                .starts_with(self.parent_site.segments())
            || !body_site
                .segments()
                .starts_with(self.parent_site.segments())
        {
            return Err(
                "[freeze:contract][callable-loop/loop-break/source-lineage-mismatch]".to_owned(),
            );
        }
        if self.source_items.is_empty()
            || self.source_items.iter().any(|item| {
                !item
                    .call_site()
                    .node()
                    .segments()
                    .starts_with(self.parent_site.segments())
            })
        {
            return Err(
                "[freeze:contract][callable-loop/loop-break/source-item-parent-mismatch]"
                    .to_owned(),
            );
        }
        let target_matches = self
            .source_items
            .iter()
            .filter(|item| item.call_site() == self.source_target.call_site())
            .count();
        if target_matches != 1 || !self.source_target.has_exact_i64_requirement(&[1]) {
            return Err(
                "[freeze:contract][callable-loop/loop-break/source-target-relation-mismatch]"
                    .to_owned(),
            );
        }
        let facts = self.loop_break_facts()?;
        if facts.loop_condition != self.condition || self.body.len() != 3 {
            return Err(
                "[freeze:contract][callable-loop/loop-break/facts-source-mismatch]".to_owned(),
            );
        }
        self.source_port
            .expr(&self.condition, &self.condition_source)
            .map_err(|error| {
                format!("[freeze:contract][callable-loop/loop-break/source-port] {error}")
            })?;
        let body = self
            .source_port
            .body(&self.body, &self.body_source)
            .map_err(|error| {
                format!("[freeze:contract][callable-loop/loop-break/source-port] {error}")
            })?;
        if self.source_port.body_statements(&body).len() != 3 {
            return Err("[freeze:contract][callable-loop/loop-break/body-cardinality]".to_owned());
        }
        let break_if = self
            .source_port
            .body_stmt(&body, 0)
            .map_err(|error| error.render())?;
        let break_condition = self
            .source_port
            .child_expr_from_stmt(
                &break_if,
                crate::mir::resolved_semantics::ExprChildRoleV1::IfCondition,
            )
            .map_err(|error| error.render())?;
        let then_body = self
            .source_port
            .child_body_from_stmt(
                &break_if,
                crate::mir::resolved_semantics::BodyChildRoleV1::IfThen,
            )
            .map_err(|error| error.render())?;
        if self.source_port.body_statements(&then_body).len() != 1
            || self.source_port.body_stmt(&then_body, 0).is_err()
            || self.source_port.stmt_syntax(&break_if)
                != self.body.first().ok_or_else(|| {
                    "[freeze:contract][callable-loop/loop-break/body-missing]".to_owned()
                })?
        {
            return Err(
                "[freeze:contract][callable-loop/loop-break/break-if-source-mismatch]".to_owned(),
            );
        }
        if self.source_port.expr_syntax(&break_condition) != &facts.break_condition {
            return Err(
                "[freeze:contract][callable-loop/loop-break/break-condition-mismatch]".to_owned(),
            );
        }
        Ok(())
    }

    pub(in crate::mir::builder) fn source_recipe(
        &self,
    ) -> Result<crate::mir::builder::control_flow::plan::recipe_tree::BuiltRecipeTree, String> {
        let facts = self.loop_break_facts()?;
        let loop_cond_view = CondBlockView::from_expr(&facts.loop_condition);
        let break_cond_view = CondBlockView::from_expr(&facts.break_condition);
        let body = self
            .source_port
            .body(&self.body, &self.body_source)
            .map_err(|error| error.to_owned())?;
        let break_if = self
            .source_port
            .body_stmt(&body, 0)
            .map_err(|error| error.render())?;
        let then_body = self
            .source_port
            .child_body_from_stmt(
                &break_if,
                crate::mir::resolved_semantics::BodyChildRoleV1::IfThen,
            )
            .map_err(|error| error.render())?;
        let break_then = (0..self.source_port.body_statements(&then_body).len())
            .map(|index| self.source_port.body_stmt(&then_body, index))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|error| error.render())?
            .into_iter()
            .map(|stmt| self.source_port.stmt_syntax(&stmt).clone())
            .collect::<Vec<_>>();
        build_loop_break_source_recipe(
            &self.parent_node,
            &self.body,
            &break_then,
            loop_cond_view,
            break_cond_view,
            facts,
        )
        .map_err(|error| format!("[freeze:contract][callable-loop/loop-break/recipe] {error}"))
    }
}

#[derive(Debug)]
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
