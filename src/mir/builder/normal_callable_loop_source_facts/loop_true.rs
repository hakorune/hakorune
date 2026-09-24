//! Source-aware LoopTrue Facts/Recipe co-seal for the selected parser loop.
//!
//! The planner's `LoopTrueBreakContinueFacts` remains the only semantic
//! issuer. This module only binds that outcome to resolver-owned source
//! identity and the source expression port; it never classifies syntax or
//! creates a second Recipe/JoinSig.

use crate::ast::{ASTNode, LiteralValue};
use crate::mir::builder::control_flow::plan::loop_cond::true_break_continue::LoopTrueBreakContinueFacts;
use crate::mir::builder::control_flow::plan::PlanBuildOutcome;
use crate::mir::builder::normal_callable_loop_handoff::{
    CallableLoopReadyBodyOnlyProductV1, CallableSemanticLoopHandoffPreEffectReceiptV1,
};
use crate::mir::builder::normal_callable_loop_source_facts::CallableGenericLoopSourceFactsRouteErrorV1;
use crate::mir::builder::normal_callable_loop_source_port::CallableLoopSourceExpressionPortV1;
use crate::mir::builder::normal_callable_loop_source_route::{
    CallableLoopRouteMatchV1, CallableLoopSourceItemBindingV1,
    CallableLoopSourceRouteRejectV1, CallableLoopSourceTargetProbeV1,
    CallableLoopSourceTargetRelationV1,
};
use crate::mir::builder::raw_invocation_source_transport::RawInvocationSourceContextV1;
use crate::mir::loop_recipe_contract::route_id::LoopRouteId;
use crate::mir::loop_structural_facts::VerifiedLoopCondBreakContinueSourceForestProjectionV1;
use crate::mir::resolved_semantics::{
    FunctionOriginV1, FunctionOwnerIdV1, SemanticOwnerSourceKindV1, SourceNodeSiteV1,
    SourceStmtSiteV1,
};
use std::cell::RefCell;
use std::rc::Rc;

/// Move-only source/physical transport for one selected LoopTrue route.
///
/// All source relations are issued by the same callable invocation. The
/// physical adapter consumes this product before the LoopTrue skeleton is
/// allocated; no field is recovered from a name, line, or route-local AST.
#[derive(Debug)]
pub(in crate::mir::builder) struct SourceLoopTruePhysicalInputV1<'source, 'ledger> {
    owner: FunctionOwnerIdV1,
    parent_site: SourceNodeSiteV1,
    parent_source: &'source RawInvocationSourceContextV1,
    condition_source: RawInvocationSourceContextV1,
    body_source: RawInvocationSourceContextV1,
    condition: ASTNode,
    body: Vec<ASTNode>,
    pre_effect: CallableSemanticLoopHandoffPreEffectReceiptV1,
    outcome: PlanBuildOutcome,
    selection: CallableLoopRouteMatchV1,
    projection: VerifiedLoopCondBreakContinueSourceForestProjectionV1,
    source_items: Box<[CallableLoopSourceItemBindingV1]>,
    source_target: CallableLoopSourceTargetRelationV1,
    source_port: CallableLoopSourceExpressionPortV1<'ledger>,
}

impl SourceLoopTruePhysicalInputV1<'_, '_> {
    pub(in crate::mir::builder) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(in crate::mir::builder) fn parent_site(&self) -> &SourceNodeSiteV1 {
        &self.parent_site
    }

    pub(in crate::mir::builder) fn parent_source(&self) -> &RawInvocationSourceContextV1 {
        self.parent_source
    }

    pub(in crate::mir::builder) fn condition_source(&self) -> &RawInvocationSourceContextV1 {
        &self.condition_source
    }

    pub(in crate::mir::builder) fn body_source(&self) -> &RawInvocationSourceContextV1 {
        &self.body_source
    }

    pub(in crate::mir::builder) fn condition(&self) -> &ASTNode {
        &self.condition
    }

    pub(in crate::mir::builder) fn body(&self) -> &[ASTNode] {
        &self.body
    }

    pub(in crate::mir::builder) fn pre_effect(
        &self,
    ) -> &CallableSemanticLoopHandoffPreEffectReceiptV1 {
        &self.pre_effect
    }

    pub(in crate::mir::builder) fn outcome(&self) -> &PlanBuildOutcome {
        &self.outcome
    }

    pub(in crate::mir::builder) fn selection(&self) -> &CallableLoopRouteMatchV1 {
        &self.selection
    }

    pub(in crate::mir::builder) fn projection(
        &self,
    ) -> &VerifiedLoopCondBreakContinueSourceForestProjectionV1 {
        &self.projection
    }

    pub(in crate::mir::builder) fn source_items(&self) -> &[CallableLoopSourceItemBindingV1] {
        &self.source_items
    }

    pub(in crate::mir::builder) fn source_target(&self) -> &CallableLoopSourceTargetRelationV1 {
        &self.source_target
    }

    pub(in crate::mir::builder) const fn source_port(
        &self,
    ) -> &CallableLoopSourceExpressionPortV1<'_> {
        &self.source_port
    }

    pub(in crate::mir::builder) fn loop_true_facts(
        &self,
    ) -> Result<&LoopTrueBreakContinueFacts, String> {
        self.outcome
            .facts
            .as_ref()
            .and_then(|facts| facts.facts.loop_true_break_continue())
            .ok_or_else(|| "[freeze:contract][callable-loop/loop-true/facts-missing]".to_owned())
    }

    /// Recheck the co-seal at the physical boundary before any Builder effect.
    pub(in crate::mir::builder) fn validate_for_source_port(&self) -> Result<(), String> {
        if self.pre_effect.owner() != self.owner || self.pre_effect.loop_site() != &self.parent_site
        {
            return Err(
                "[freeze:contract][callable-loop/loop-true/pre-effect-owner-site-mismatch]"
                    .to_owned(),
            );
        }
        if self.projection.owner() != self.owner {
            return Err(
                "[freeze:contract][callable-loop/loop-true/projection-owner-mismatch]".to_owned(),
            );
        }
        let root_site = SourceStmtSiteV1::from_node(self.parent_site.clone());
        if self.projection.member_sites().first() != Some(&root_site)
            || self.projection.exits().is_empty()
        {
            return Err(
                "[freeze:contract][callable-loop/loop-true/forest-root-or-exit-mismatch]"
                    .to_owned(),
            );
        }
        let condition_site = self.condition_source.site().ok_or_else(|| {
            "[freeze:contract][callable-loop/loop-true/condition-site-missing]".to_owned()
        })?;
        let body_site = self.body_source.site().ok_or_else(|| {
            "[freeze:contract][callable-loop/loop-true/body-site-missing]".to_owned()
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
                "[freeze:contract][callable-loop/loop-true/source-site-parent-mismatch]".to_owned(),
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
                "[freeze:contract][callable-loop/loop-true/source-item-parent-mismatch]".to_owned(),
            );
        }
        let target_matches = self
            .source_items
            .iter()
            .filter(|item| item.call_site() == self.source_target.call_site())
            .count();
        if target_matches != 1 {
            return Err(
                "[freeze:contract][callable-loop/loop-true/source-target-site-coverage]".to_owned(),
            );
        }
        if !self.source_target.has_exact_i64_result() {
            return Err(
                "[freeze:contract][callable-loop/loop-true/result-requirement-mismatch]".to_owned(),
            );
        }
        let facts = self.loop_true_facts()?;
        if !matches!(
            self.condition,
            ASTNode::Literal {
                value: LiteralValue::Bool(true),
                ..
            }
        ) || facts.recipe.body.body != self.body
        {
            return Err(
                "[freeze:contract][callable-loop/loop-true/recipe-source-mismatch]".to_owned(),
            );
        }
        if self.selection.matched_routes() != [LoopRouteId::LoopTrueBreakContinue] {
            return Err(
                "[freeze:contract][callable-loop/loop-true/route-selection-mismatch]".to_owned(),
            );
        }
        self.source_port
            .expr(&self.condition, &self.condition_source)
            .map_err(|error| {
                format!("[freeze:contract][callable-loop/loop-true/source-port] {error}")
            })?;
        self.source_port
            .body(&self.body, &self.body_source)
            .map_err(|error| {
                format!("[freeze:contract][callable-loop/loop-true/source-port] {error}")
            })?;
        Ok(())
    }
}

/// One source Facts result that retains the existing planner outcome and
/// defers all Builder work to `into_physical_input`.
#[derive(Debug)]
pub(in crate::mir::builder) struct CallableLoopTrueSourceFactsV1<'source> {
    owner: FunctionOwnerIdV1,
    parent_source: &'source RawInvocationSourceContextV1,
    condition_source: RawInvocationSourceContextV1,
    body_source: RawInvocationSourceContextV1,
    condition: ASTNode,
    body: Vec<ASTNode>,
    binding_product: CallableLoopReadyBodyOnlyProductV1,
    outcome: PlanBuildOutcome,
    selection: CallableLoopRouteMatchV1,
    projection: VerifiedLoopCondBreakContinueSourceForestProjectionV1,
    source_items: Box<[CallableLoopSourceItemBindingV1]>,
    source_target: CallableLoopSourceTargetRelationV1,
}

impl<'source> CallableLoopTrueSourceFactsV1<'source> {
    pub(in crate::mir::builder) fn into_physical_input<'ledger>(
        self,
        source_ledger: &'ledger Rc<RefCell<
            crate::mir::builder::normal_callable_semantic_lowering_state::
                CallableSemanticLoweringState,
        >>,
    ) -> Result<SourceLoopTruePhysicalInputV1<'source, 'ledger>, String> {
        let Self {
            owner,
            parent_source,
            condition_source,
            body_source,
            condition,
            body,
            binding_product,
            outcome,
            selection,
            projection,
            source_items,
            source_target,
        } = self;
        if source_ledger.borrow().owner() != owner {
            return Err(
                "[freeze:contract][callable-loop/loop-true/source-port-owner-mismatch]".to_owned(),
            );
        }
        let parent_site = parent_source.site().cloned().ok_or_else(|| {
            "[freeze:contract][callable-loop/loop-true/parent-site-missing]".to_owned()
        })?;
        let condition_site = condition_source.site().ok_or_else(|| {
            "[freeze:contract][callable-loop/loop-true/condition-site-missing]".to_owned()
        })?;
        let body_site = body_source.site().ok_or_else(|| {
            "[freeze:contract][callable-loop/loop-true/body-site-missing]".to_owned()
        })?;
        let facts = outcome
            .facts
            .as_ref()
            .and_then(|facts| facts.facts.loop_true_break_continue())
            .ok_or_else(|| "[freeze:contract][callable-loop/loop-true/facts-missing]".to_owned())?;
        if !matches!(
            condition,
            ASTNode::Literal {
                value: LiteralValue::Bool(true),
                ..
            }
        ) || facts.recipe.body.body != body
        {
            return Err(
                "[freeze:contract][callable-loop/loop-true/source-facts-shape-mismatch]".to_owned(),
            );
        }
        let pre_effect = binding_product
            .consume_pre_effect(&parent_site, condition_site, body_site)
            .map_err(|error| {
                format!("[freeze:contract][callable-loop/loop-true/pre-effect] {error}")
            })?;
        let source_port = CallableLoopSourceExpressionPortV1::new(source_ledger);
        Ok(SourceLoopTruePhysicalInputV1 {
            owner,
            parent_site,
            parent_source,
            condition_source,
            body_source,
            condition,
            body,
            pre_effect,
            outcome,
            selection,
            projection,
            source_items,
            source_target,
            source_port,
        })
    }
}

#[allow(clippy::too_many_arguments)]
pub(super) fn issue<'source>(
    owner: FunctionOwnerIdV1,
    parent_source: &'source RawInvocationSourceContextV1,
    condition_source: RawInvocationSourceContextV1,
    body_source: RawInvocationSourceContextV1,
    condition: ASTNode,
    body: Vec<ASTNode>,
    binding_product: CallableLoopReadyBodyOnlyProductV1,
    function_origin: Option<FunctionOriginV1>,
    source_kind: Option<SemanticOwnerSourceKindV1>,
    outcome: PlanBuildOutcome,
    selection: CallableLoopRouteMatchV1,
    projection: Option<VerifiedLoopCondBreakContinueSourceForestProjectionV1>,
    source_items: Box<[CallableLoopSourceItemBindingV1]>,
    source_target_probe: CallableLoopSourceTargetProbeV1,
) -> Result<CallableLoopTrueSourceFactsV1<'source>, CallableGenericLoopSourceFactsRouteErrorV1> {
    let function_origin = function_origin.ok_or_else(|| {
        CallableGenericLoopSourceFactsRouteErrorV1::LoopTrueRouteRejected(
            CallableLoopSourceRouteRejectV1::SourceIdentityMissing,
        )
    })?;
    let source_kind = source_kind.ok_or_else(|| {
        CallableGenericLoopSourceFactsRouteErrorV1::LoopTrueRouteRejected(
            CallableLoopSourceRouteRejectV1::SourceIdentityMissing,
        )
    })?;
    let parent_site = parent_source.site().cloned().ok_or_else(|| {
        CallableGenericLoopSourceFactsRouteErrorV1::LoopTrueRouteRejected(
            CallableLoopSourceRouteRejectV1::SourceParentMissing,
        )
    })?;
    if selection.matched_routes() != [LoopRouteId::LoopTrueBreakContinue] {
        return Err(
            CallableGenericLoopSourceFactsRouteErrorV1::NonGenericOrOverlapping {
                routes: selection.matched_routes().into(),
            },
        );
    }
    let facts = outcome
        .facts
        .as_ref()
        .and_then(|facts| facts.facts.loop_true_break_continue())
        .ok_or_else(|| {
            CallableGenericLoopSourceFactsRouteErrorV1::LoopTrueRouteRejected(
                CallableLoopSourceRouteRejectV1::FactsMissing,
            )
        })?;
    if !matches!(
        condition,
        ASTNode::Literal {
            value: LiteralValue::Bool(true),
            ..
        }
    ) || facts.recipe.body.body != body
    {
        return Err(
            CallableGenericLoopSourceFactsRouteErrorV1::LoopTrueRouteRejected(
                CallableLoopSourceRouteRejectV1::SourceTargetRequirementMismatch,
            ),
        );
    }
    let projection = projection.ok_or_else(|| {
        CallableGenericLoopSourceFactsRouteErrorV1::LoopTrueRouteRejected(
            CallableLoopSourceRouteRejectV1::ProjectionMissing,
        )
    })?;
    let parent_stmt_site = SourceStmtSiteV1::from_node(parent_site.clone());
    if projection.owner() != owner
        || !projection.matches_source_identity(function_origin, source_kind, &parent_stmt_site)
    {
        return Err(
            CallableGenericLoopSourceFactsRouteErrorV1::LoopTrueRouteRejected(
                CallableLoopSourceRouteRejectV1::ProjectionIdentityMismatch,
            ),
        );
    }
    if source_items.is_empty()
        || source_items.iter().any(|item| {
            !item
                .call_site()
                .node()
                .segments()
                .starts_with(parent_site.segments())
        })
    {
        return Err(
            CallableGenericLoopSourceFactsRouteErrorV1::LoopTrueRouteRejected(
                CallableLoopSourceRouteRejectV1::SourceItemOutsideLoop,
            ),
        );
    }
    let source_target = source_target_probe
        .into_selected_relation(&source_items)
        .map_err(CallableGenericLoopSourceFactsRouteErrorV1::LoopTrueRouteRejected)?;
    if source_items
        .iter()
        .filter(|item| item.call_site() == source_target.call_site())
        .count()
        != 1
    {
        return Err(
            CallableGenericLoopSourceFactsRouteErrorV1::LoopTrueRouteRejected(
                CallableLoopSourceRouteRejectV1::SourceTargetSiteMismatch,
            ),
        );
    }
    if !source_target.has_exact_i64_result() {
        return Err(
            CallableGenericLoopSourceFactsRouteErrorV1::LoopTrueRouteRejected(
                CallableLoopSourceRouteRejectV1::SourceTargetRequirementMismatch,
            ),
        );
    }
    Ok(CallableLoopTrueSourceFactsV1 {
        owner,
        parent_source,
        condition_source,
        body_source,
        condition,
        body,
        binding_product,
        outcome,
        selection,
        projection,
        source_items,
        source_target,
    })
}
