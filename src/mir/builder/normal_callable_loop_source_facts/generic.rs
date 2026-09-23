//! Source-aware GenericLoop Facts/Recipe issuer.
//!
//! This module only transports one already-located callable Loop into the
//! existing planner/Facts authority. It does not lower, consume a ledger,
//! enter the route registry, or provide a fallback path.  Its only production
//! caller is the Ready branch in `raw_loop_child_entry`.

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::joinir::route_entry::registry::{
    select_recipe_first_routes, LocatedGenericLoopV1SelectionErrorV1, RecipeFirstRouteSelectionV1,
};
use crate::mir::builder::control_flow::lower::normalize::CanonicalLoopFacts;
use crate::mir::builder::control_flow::plan::features::generic_loop_body;
use crate::mir::builder::control_flow::plan::single_planner::{
    self, CallableLoopFactsPlannerInputV1,
};
use crate::mir::builder::control_flow::plan::GenericLoopFactsPolicyFrameV1;
use crate::mir::builder::control_flow::plan::GenericLoopV1Facts;
use crate::mir::builder::control_flow::plan::PlanBuildOutcome;
use crate::mir::builder::normal_callable_loop_source_route::CallableLoopSourceRouteRejectV1;
use crate::mir::builder::normal_callable_loop_source_route::{
    CallableLoopSourceItemBindingV1, CallableLoopSourceItemDispositionV1,
};
use crate::mir::loop_recipe_contract::route_id::LoopRouteId;
use crate::mir::resolved_semantics::{FunctionOwnerIdV1, SourceNodeSiteV1, SourcePathSegmentV1};

#[cfg(test)]
use crate::mir::builder::control_flow::joinir::structural_port::{
    issue_route_neutral_structural_seed, CallableLoopRouteNeutralStructuralSeedV1,
    CallableLoopSourceBoundStructuralPortV1, CallableLoopStructuralLeaseRejectV1,
};
use crate::mir::builder::normal_callable_loop_handoff::{
    CallableLoopReadyBodyOnlyProductV1, CallableSemanticLoopHandoffPreEffectReceiptV1,
};
use crate::mir::builder::raw_invocation_source_transport::RawInvocationSourceContextV1;
use crate::mir::builder::raw_loop_child_entry::PreparedCallableGenericLoopSourceFactsPayloadV1;

#[path = "generic/carrier_relation.rs"]
mod carrier_relation;
pub(in crate::mir::builder) use carrier_relation::{
    CallableLoopCarrierRelationRejectV1, CallableLoopCarrierRelationV1, CallableLoopCarrierSlotV1,
};
#[path = "generic/source_admission.rs"]
mod source_admission;
pub(in crate::mir::builder) use source_admission::{
    CallableGenericLoopSourceRouteAdmissionV1, CallableGenericLoopSourceRouteKindV1,
    CallableGenericLoopSourceSessionKeyV1, PreparedCallableGenericLoopSourceEvidenceV1,
};

use super::loop_cond;
use super::loop_cond::CallableLoopCondSourceFactsV1;
#[allow(unused_imports)]
pub(in crate::mir::builder) use super::loop_cond::SourceLoopCondPhysicalInputV1;
use super::loop_true;
use super::loop_true::CallableLoopTrueSourceFactsV1;
#[allow(unused_imports)]
pub(in crate::mir::builder) use super::loop_true::SourceLoopTruePhysicalInputV1;

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::builder) enum CallableGenericLoopSourceFactsSourceErrorV1 {
    ParentNotLocated,
    ConditionNotLocated,
    BodyNotLocated,
    ForeignRootLineage,
    ParentSiteMismatch,
    ConditionSiteMismatch,
    BodySiteMismatch,
    OwnerMismatch,
}

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::builder) enum CallableGenericLoopSourceFactsRouteErrorV1 {
    GenericLoopV1NotSelected,
    NonGenericOrOverlapping { routes: Box<[LoopRouteId]> },
    SourceEvidenceRejected(Box<str>),
    LoopCondRouteRejected(CallableLoopSourceRouteRejectV1),
    LoopTrueRouteRejected(CallableLoopSourceRouteRejectV1),
}

#[derive(Debug)]
pub(in crate::mir::builder) enum CallableGenericLoopSourceFactsDispositionV1<'source> {
    SourceUnavailable(CallableGenericLoopSourceFactsSourceErrorV1),
    FactsAbsent,
    FactsRejected(Box<str>),
    RouteNotFrontSelected(CallableGenericLoopSourceFactsRouteErrorV1),
    Ready(CallableGenericLoopSourceFactsV1<'source>),
    LoopCondReady(CallableLoopCondSourceFactsV1<'source>),
    LoopTrueReady(CallableLoopTrueSourceFactsV1<'source>),
}

/// One move-only source-located Facts/Recipe outcome.
///
/// `PlanBuildOutcome` remains the existing Facts/Recipe authority. This
/// aggregate only co-seals it with the already-issued source schedule and the
/// exact route selection; it does not issue a new semantic binding or policy.
#[derive(Debug)]
pub(in crate::mir::builder) struct CallableGenericLoopSourceFactsV1<'source> {
    owner: FunctionOwnerIdV1,
    parent_source: &'source RawInvocationSourceContextV1,
    condition_source: RawInvocationSourceContextV1,
    body_source: RawInvocationSourceContextV1,
    condition: ASTNode,
    body: Vec<ASTNode>,
    policy: GenericLoopFactsPolicyFrameV1,
    debug: bool,
    in_static_box: bool,
    outcome: PlanBuildOutcome,
    route_admission: CallableGenericLoopSourceRouteAdmissionV1<'source>,
}

impl<'source> CallableGenericLoopSourceFactsV1<'source> {
    #[cfg(test)]
    pub(in crate::mir::builder) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    #[cfg(test)]
    pub(in crate::mir::builder) const fn policy(&self) -> GenericLoopFactsPolicyFrameV1 {
        self.policy
    }

    #[cfg(test)]
    pub(in crate::mir::builder) fn selection(&self) -> &RecipeFirstRouteSelectionV1 {
        self.route_admission.selection()
    }

    #[cfg(test)]
    pub(in crate::mir::builder) fn outcome(&self) -> &PlanBuildOutcome {
        &self.outcome
    }

    pub(in crate::mir::builder) fn claim_all(
        self,
    ) -> Result<
        CallableGenericLoopSourceFactsReceiptV1<'source>,
        CallableGenericLoopSourceFactsClaimErrorV1,
    > {
        let CallableGenericLoopSourceFactsV1 {
            owner,
            parent_source,
            condition_source,
            body_source,
            condition,
            body,
            policy,
            debug,
            in_static_box,
            outcome,
            route_admission,
        } = self;
        parent_source
            .site()
            .ok_or(CallableGenericLoopSourceFactsClaimErrorV1::ParentNotLocated)?;
        condition_source
            .site()
            .ok_or(CallableGenericLoopSourceFactsClaimErrorV1::ConditionNotLocated)?;
        body_source
            .site()
            .ok_or(CallableGenericLoopSourceFactsClaimErrorV1::BodyNotLocated)?;
        Ok(CallableGenericLoopSourceFactsReceiptV1 {
            owner,
            parent_source,
            condition_source,
            body_source,
            _condition: condition,
            _body: body,
            policy,
            debug,
            in_static_box,
            outcome,
            route_admission,
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::builder) enum CallableGenericLoopSourceFactsClaimErrorV1 {
    ParentNotLocated,
    ConditionNotLocated,
    BodyNotLocated,
}

/// One-shot source-facts claim receipt.  The pre-effect receipt is retained
/// here; it is not an observation that may be discarded at the old route.
#[derive(Debug)]
pub(in crate::mir::builder) struct CallableGenericLoopSourceFactsReceiptV1<'source> {
    owner: FunctionOwnerIdV1,
    parent_source: &'source RawInvocationSourceContextV1,
    condition_source: RawInvocationSourceContextV1,
    body_source: RawInvocationSourceContextV1,
    _condition: ASTNode,
    _body: Vec<ASTNode>,
    policy: GenericLoopFactsPolicyFrameV1,
    debug: bool,
    in_static_box: bool,
    outcome: PlanBuildOutcome,
    route_admission: CallableGenericLoopSourceRouteAdmissionV1<'source>,
}

impl<'source> CallableGenericLoopSourceFactsReceiptV1<'source> {
    #[cfg(test)]
    pub(in crate::mir::builder) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    #[cfg(test)]
    pub(in crate::mir::builder) fn pre_effect(
        &self,
    ) -> &CallableSemanticLoopHandoffPreEffectReceiptV1 {
        self.route_admission.evidence().pre_effect()
    }

    fn route_admission(&self) -> &CallableGenericLoopSourceRouteAdmissionV1<'source> {
        &self.route_admission
    }

    #[cfg(test)]
    pub(in crate::mir::builder) fn policy(&self) -> GenericLoopFactsPolicyFrameV1 {
        self.policy
    }

    pub(in crate::mir::builder) fn into_semantic_recipe(
        self,
    ) -> Result<
        CallableGenericLoopV1SemanticRecipeV1<'source>,
        CallableGenericLoopV1SemanticRecipeRejectV1,
    > {
        CallableGenericLoopV1SemanticRecipeIssuerV1::issue(self)
    }
}

/// The single source-backed semantic Recipe owner for GenericLoopV1.
///
/// This wrapper owns the already-claimed source Facts receipt.  It does not
/// copy Facts, re-run the planner, select a route, or issue physical IDs.
#[derive(Debug)]
pub(in crate::mir::builder) struct CallableGenericLoopV1SemanticRecipeV1<'source> {
    receipt: CallableGenericLoopSourceFactsReceiptV1<'source>,
}

/// Borrowed source-relation view for the caller-zero bridge row.
///
/// Every field is borrowed from the one claimed receipt.  This view exposes
/// source lineage and grouped pre-effect rows to the future normalizer port,
/// but it cannot issue a binding, select a route, or publish a physical value.
/// The higher-ranked callback keeps the view scoped to this observation.
#[derive(Debug)]
pub(in crate::mir::builder) struct CallableGenericLoopSourceRelationViewV1<'view> {
    owner: FunctionOwnerIdV1,
    parent_source: &'view RawInvocationSourceContextV1,
    condition_source: &'view RawInvocationSourceContextV1,
    body_source: &'view RawInvocationSourceContextV1,
    session: &'view CallableGenericLoopSourceSessionKeyV1,
    pre_effect: &'view CallableSemanticLoopHandoffPreEffectReceiptV1,
    facts: &'view CanonicalLoopFacts,
    generic: &'view GenericLoopV1Facts,
    carrier_relation: &'view CallableLoopCarrierRelationV1,
    source_items: &'view [CallableLoopSourceItemBindingV1],
    source_dispositions: &'view [CallableLoopSourceItemDispositionV1],
    selection: &'view RecipeFirstRouteSelectionV1,
    selected: &'view CallableGenericLoopSourceRouteKindV1,
    policy: GenericLoopFactsPolicyFrameV1,
    debug: bool,
    in_static_box: bool,
}

impl CallableGenericLoopSourceRelationViewV1<'_> {
    pub(in crate::mir::builder) fn carrier_relation(&self) -> &CallableLoopCarrierRelationV1 {
        self.carrier_relation
    }

    pub(in crate::mir::builder) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(in crate::mir::builder) fn parent_source(&self) -> &RawInvocationSourceContextV1 {
        self.parent_source
    }

    pub(in crate::mir::builder) fn condition_source(&self) -> &RawInvocationSourceContextV1 {
        self.condition_source
    }

    pub(in crate::mir::builder) fn body_source(&self) -> &RawInvocationSourceContextV1 {
        self.body_source
    }

    pub(in crate::mir::builder) fn pre_effect(
        &self,
    ) -> &CallableSemanticLoopHandoffPreEffectReceiptV1 {
        self.pre_effect
    }

    pub(in crate::mir::builder) fn session(&self) -> &CallableGenericLoopSourceSessionKeyV1 {
        self.session
    }

    pub(in crate::mir::builder) fn facts(&self) -> &CanonicalLoopFacts {
        self.facts
    }

    pub(in crate::mir::builder) fn generic(&self) -> &GenericLoopV1Facts {
        self.generic
    }

    pub(in crate::mir::builder) fn source_items(&self) -> &[CallableLoopSourceItemBindingV1] {
        self.source_items
    }

    pub(in crate::mir::builder) fn source_dispositions(
        &self,
    ) -> &[CallableLoopSourceItemDispositionV1] {
        self.source_dispositions
    }

    pub(in crate::mir::builder) fn selection(&self) -> &RecipeFirstRouteSelectionV1 {
        self.selection
    }

    pub(in crate::mir::builder) fn selected(&self) -> &CallableGenericLoopSourceRouteKindV1 {
        self.selected
    }

    pub(in crate::mir::builder) const fn policy(&self) -> GenericLoopFactsPolicyFrameV1 {
        self.policy
    }

    pub(in crate::mir::builder) const fn debug(&self) -> bool {
        self.debug
    }

    pub(in crate::mir::builder) const fn in_static_box(&self) -> bool {
        self.in_static_box
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::builder) enum CallableGenericLoopV1SemanticRecipeRejectV1 {
    FactsMissing,
    GenericFactsMissing,
    NestedLoopOutsideFirstCohort,
    BlockExprPreludeOutsideFirstCohort,
    CarrierRelation(CallableLoopCarrierRelationRejectV1),
}

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::builder) enum CallableGenericLoopV1SemanticRecipeViewRejectV1 {
    FactsMissing,
    GenericFactsMissing,
}

/// HRTB-bounded semantic view.  The borrowed fields all come from the one
/// claimed receipt and cannot escape the callback with a source lifetime.
#[derive(Debug)]
pub(in crate::mir::builder) struct CallableGenericLoopV1SemanticViewV1<'view> {
    owner: FunctionOwnerIdV1,
    _loop_site: &'view SourceNodeSiteV1,
    pre_effect: &'view CallableSemanticLoopHandoffPreEffectReceiptV1,
    facts: &'view CanonicalLoopFacts,
    _generic: &'view GenericLoopV1Facts,
    _selection: &'view RecipeFirstRouteSelectionV1,
    _selected: &'view CallableGenericLoopSourceRouteKindV1,
    debug: bool,
    in_static_box: bool,
}

impl CallableGenericLoopV1SemanticViewV1<'_> {
    pub(in crate::mir::builder) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(in crate::mir::builder) fn pre_effect(
        &self,
    ) -> &CallableSemanticLoopHandoffPreEffectReceiptV1 {
        self.pre_effect
    }

    pub(in crate::mir::builder) fn facts(&self) -> &CanonicalLoopFacts {
        self.facts
    }

    pub(in crate::mir::builder) const fn debug(&self) -> bool {
        self.debug
    }

    pub(in crate::mir::builder) const fn in_static_box(&self) -> bool {
        self.in_static_box
    }
}

impl<'source> CallableGenericLoopV1SemanticRecipeV1<'source> {
    #[cfg(test)]
    pub(in crate::mir::builder) fn replace_session_parent_for_test(
        &mut self,
        parent: SourceNodeSiteV1,
    ) {
        self.receipt
            .route_admission
            .replace_session_parent_for_test(parent);
    }

    /// Observe the co-sealed source relation without consuming the Recipe.
    ///
    /// This is caller-zero infrastructure for the future normalizer port.  A
    /// consumer must use the existing `with_view`/physical path after this
    /// callback; this method does not mutate Builder state or reselect a route.
    pub(in crate::mir::builder) fn with_source_relation_view<R>(
        &self,
        use_view: impl for<'view> FnOnce(CallableGenericLoopSourceRelationViewV1<'view>) -> R,
    ) -> Result<R, CallableGenericLoopV1SemanticRecipeViewRejectV1> {
        let Some(facts) = self.receipt.outcome.facts.as_ref() else {
            return Err(CallableGenericLoopV1SemanticRecipeViewRejectV1::FactsMissing);
        };
        let Some(generic) = facts.facts.generic_loop_v1() else {
            return Err(CallableGenericLoopV1SemanticRecipeViewRejectV1::GenericFactsMissing);
        };
        let evidence = self.receipt.route_admission().evidence();
        let view = CallableGenericLoopSourceRelationViewV1 {
            owner: self.receipt.owner,
            parent_source: evidence.parent_source(),
            condition_source: evidence.condition_source(),
            body_source: evidence.body_source(),
            session: evidence.session(),
            pre_effect: evidence.pre_effect(),
            carrier_relation: evidence.carrier_relation(),
            source_items: evidence.source_items(),
            source_dispositions: evidence.source_dispositions(),
            facts,
            generic,
            selection: self.receipt.route_admission().selection(),
            selected: self.receipt.route_admission().selected(),
            policy: self.receipt.policy,
            debug: self.receipt.debug,
            in_static_box: self.receipt.in_static_box,
        };
        Ok(use_view(view))
    }

    /// Consume the Recipe once for its source-aware physical consumer.
    ///
    /// Unlike `with_source_relation_view`, this method moves the Recipe into
    /// the callback boundary.  The callback receives only a borrow of the
    /// already co-sealed receipt, so it cannot retain or reissue source
    /// authority after the physical adapter returns.
    pub(in crate::mir::builder) fn with_source_relation_view_once<R>(
        self,
        use_view: impl for<'view> FnOnce(CallableGenericLoopSourceRelationViewV1<'view>) -> R,
    ) -> Result<R, CallableGenericLoopV1SemanticRecipeViewRejectV1> {
        let Self { receipt } = self;
        let Some(facts) = receipt.outcome.facts.as_ref() else {
            return Err(CallableGenericLoopV1SemanticRecipeViewRejectV1::FactsMissing);
        };
        let Some(generic) = facts.facts.generic_loop_v1() else {
            return Err(CallableGenericLoopV1SemanticRecipeViewRejectV1::GenericFactsMissing);
        };
        let evidence = receipt.route_admission().evidence();
        let view = CallableGenericLoopSourceRelationViewV1 {
            owner: receipt.owner,
            parent_source: evidence.parent_source(),
            condition_source: evidence.condition_source(),
            body_source: evidence.body_source(),
            session: evidence.session(),
            pre_effect: evidence.pre_effect(),
            carrier_relation: evidence.carrier_relation(),
            source_items: evidence.source_items(),
            source_dispositions: evidence.source_dispositions(),
            facts,
            generic,
            selection: receipt.route_admission().selection(),
            selected: receipt.route_admission().selected(),
            policy: receipt.policy,
            debug: receipt.debug,
            in_static_box: receipt.in_static_box,
        };
        Ok(use_view(view))
    }

    pub(in crate::mir::builder) fn with_view<R>(
        self,
        use_view: impl for<'view> FnOnce(CallableGenericLoopV1SemanticViewV1<'view>) -> R,
    ) -> Result<R, CallableGenericLoopV1SemanticRecipeViewRejectV1> {
        let Self { receipt, .. } = self;
        let Some(facts) = receipt.outcome.facts.as_ref() else {
            return Err(CallableGenericLoopV1SemanticRecipeViewRejectV1::FactsMissing);
        };
        let Some(generic) = facts.facts.generic_loop_v1() else {
            return Err(CallableGenericLoopV1SemanticRecipeViewRejectV1::GenericFactsMissing);
        };
        let evidence = receipt.route_admission().evidence();
        let view = CallableGenericLoopV1SemanticViewV1 {
            owner: receipt.owner,
            _loop_site: evidence.pre_effect().loop_site(),
            pre_effect: evidence.pre_effect(),
            facts,
            _generic: generic,
            _selection: receipt.route_admission().selection(),
            _selected: receipt.route_admission().selected(),
            debug: receipt.debug,
            in_static_box: receipt.in_static_box,
        };
        Ok(use_view(view))
    }
}

pub(in crate::mir::builder) struct CallableGenericLoopV1SemanticRecipeIssuerV1;

impl CallableGenericLoopV1SemanticRecipeIssuerV1 {
    pub(in crate::mir::builder) fn issue<'source>(
        receipt: CallableGenericLoopSourceFactsReceiptV1<'source>,
    ) -> Result<
        CallableGenericLoopV1SemanticRecipeV1<'source>,
        CallableGenericLoopV1SemanticRecipeRejectV1,
    > {
        let facts = receipt
            .outcome
            .facts
            .as_ref()
            .ok_or(CallableGenericLoopV1SemanticRecipeRejectV1::FactsMissing)?;
        let generic = facts
            .facts
            .generic_loop_v1()
            .ok_or(CallableGenericLoopV1SemanticRecipeRejectV1::GenericFactsMissing)?;
        if facts.nested_loop {
            return Err(CallableGenericLoopV1SemanticRecipeRejectV1::NestedLoopOutsideFirstCohort);
        }
        if generic_loop_body::body_has_blockexpr_prelude_loop(&generic.body.body) {
            return Err(
                CallableGenericLoopV1SemanticRecipeRejectV1::BlockExprPreludeOutsideFirstCohort,
            );
        }
        // Recheck the pre-route relation against the still-owned planner
        // outcome before exposing the Recipe. This preserves the existing
        // mutation/negative tests while keeping the selected token out of the
        // source evidence issuer.
        carrier_relation::issue_pre_route(
            receipt.owner,
            receipt.route_admission().evidence().pre_effect(),
            receipt.route_admission().evidence().body_source(),
            generic,
        )
        .map_err(CallableGenericLoopV1SemanticRecipeRejectV1::CarrierRelation)?;
        Ok(CallableGenericLoopV1SemanticRecipeV1 { receipt })
    }
}

/// Move-only source/structural handoff.  The seed is transport-only and the
/// source receipt remains the sole Facts/Recipe authority.
#[cfg(test)]
#[derive(Debug)]
pub(in crate::mir::builder) struct PreparedCallableLoopStructuralHandoffV1<'source> {
    receipt: CallableGenericLoopSourceFactsReceiptV1<'source>,
    seed: CallableLoopRouteNeutralStructuralSeedV1,
}

/// Opaque view borrowed only for one callback invocation.
#[cfg(test)]
#[derive(Debug)]
pub(in crate::mir::builder) struct CallableLoopReadyStructuralViewV1<'view> {
    owner: FunctionOwnerIdV1,
    loop_site: &'view SourceNodeSiteV1,
    pre_effect: &'view CallableSemanticLoopHandoffPreEffectReceiptV1,
    outcome: &'view PlanBuildOutcome,
    selection: &'view RecipeFirstRouteSelectionV1,
    port: CallableLoopSourceBoundStructuralPortV1<'view>,
}

#[cfg(test)]
impl CallableLoopReadyStructuralViewV1<'_> {
    pub(in crate::mir::builder) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(in crate::mir::builder) fn loop_site(&self) -> &SourceNodeSiteV1 {
        self.loop_site
    }

    pub(in crate::mir::builder) fn pre_effect(
        &self,
    ) -> &CallableSemanticLoopHandoffPreEffectReceiptV1 {
        self.pre_effect
    }

    pub(in crate::mir::builder) fn outcome(&self) -> &PlanBuildOutcome {
        self.outcome
    }

    pub(in crate::mir::builder) fn selection(&self) -> &RecipeFirstRouteSelectionV1 {
        self.selection
    }

    pub(in crate::mir::builder) fn structural_port(
        &self,
    ) -> &CallableLoopSourceBoundStructuralPortV1<'_> {
        &self.port
    }
}

/// Sole issuer for the caller-zero route-neutral structural lease.
#[cfg(test)]
pub(in crate::mir::builder) struct CallableLoopStructuralLeaseIssuerV1;

#[cfg(test)]
impl CallableLoopStructuralLeaseIssuerV1 {
    pub(in crate::mir::builder) fn prepare<'source>(
        receipt: CallableGenericLoopSourceFactsReceiptV1<'source>,
    ) -> Result<PreparedCallableLoopStructuralHandoffV1<'source>, CallableLoopStructuralLeaseRejectV1>
    {
        let seed = issue_route_neutral_structural_seed(
            receipt.owner,
            &receipt.parent_source,
            &receipt.condition_source,
            &receipt.body_source,
            receipt.pre_effect(),
        )?;
        Ok(PreparedCallableLoopStructuralHandoffV1 { receipt, seed })
    }
}

#[cfg(test)]
impl<'source> PreparedCallableLoopStructuralHandoffV1<'source> {
    /// Consume the handoff exactly once; the borrowed view cannot escape this
    /// higher-ranked callback and no physical effect occurs here.
    pub(in crate::mir::builder) fn with_view<R>(
        self,
        use_view: impl for<'view> FnOnce(CallableLoopReadyStructuralViewV1<'view>) -> R,
    ) -> R {
        let Self { receipt, seed } = self;
        let view = CallableLoopReadyStructuralViewV1 {
            owner: receipt.owner,
            loop_site: receipt.pre_effect().loop_site(),
            pre_effect: receipt.pre_effect(),
            outcome: &receipt.outcome,
            selection: receipt.route_admission().selection(),
            port: CallableLoopSourceBoundStructuralPortV1::from_seed(&seed),
        };
        use_view(view)
    }
}

#[path = "generic/issuer.rs"]
mod issuer;
#[allow(unused_imports)]
pub(in crate::mir::builder) use issuer::CallableGenericLoopSourceFactsIssuerV1;

fn validate_source_input(
    parent_source: &RawInvocationSourceContextV1,
    condition_source: &RawInvocationSourceContextV1,
    body_source: &RawInvocationSourceContextV1,
    owner: FunctionOwnerIdV1,
    binding_product: &CallableLoopReadyBodyOnlyProductV1,
) -> Result<(), CallableGenericLoopSourceFactsSourceErrorV1> {
    let parent_site = parent_source
        .site()
        .ok_or(CallableGenericLoopSourceFactsSourceErrorV1::ParentNotLocated)?;
    let condition_site = condition_source
        .site()
        .ok_or(CallableGenericLoopSourceFactsSourceErrorV1::ConditionNotLocated)?;
    let body_site = body_source
        .site()
        .ok_or(CallableGenericLoopSourceFactsSourceErrorV1::BodyNotLocated)?;
    if !parent_source.shares_root_lineage(condition_source)
        || !parent_source.shares_root_lineage(body_source)
    {
        return Err(CallableGenericLoopSourceFactsSourceErrorV1::ForeignRootLineage);
    }
    if binding_product.loop_site() != parent_site {
        return Err(CallableGenericLoopSourceFactsSourceErrorV1::ParentSiteMismatch);
    }
    if binding_product.owner() != owner {
        return Err(CallableGenericLoopSourceFactsSourceErrorV1::OwnerMismatch);
    }
    if !condition_source.is_exact_loop_condition()
        || !is_direct_child(
            parent_site,
            condition_site,
            SourcePathSegmentV1::LoopCondition,
        )
    {
        return Err(CallableGenericLoopSourceFactsSourceErrorV1::ConditionSiteMismatch);
    }
    if !body_source.is_exact_loop_body_root()
        || !is_direct_child(parent_site, body_site, SourcePathSegmentV1::LoopBodyRoot)
    {
        return Err(CallableGenericLoopSourceFactsSourceErrorV1::BodySiteMismatch);
    }
    Ok(())
}

fn is_direct_child(
    parent: &SourceNodeSiteV1,
    child: &SourceNodeSiteV1,
    expected: SourcePathSegmentV1,
) -> bool {
    let parent_segments = parent.segments();
    let child_segments = child.segments();
    child_segments.len() == parent_segments.len() + 1
        && child_segments.starts_with(parent_segments)
        && child_segments.last() == Some(&expected)
}

fn route_error(
    error: LocatedGenericLoopV1SelectionErrorV1,
) -> CallableGenericLoopSourceFactsRouteErrorV1 {
    match error {
        LocatedGenericLoopV1SelectionErrorV1::GenericLoopV1NotSelected => {
            CallableGenericLoopSourceFactsRouteErrorV1::GenericLoopV1NotSelected
        }
        LocatedGenericLoopV1SelectionErrorV1::NonGenericOrOverlappingSelection {
            raw_execution_routes,
        } => CallableGenericLoopSourceFactsRouteErrorV1::NonGenericOrOverlapping {
            routes: raw_execution_routes,
        },
    }
}

#[cfg(test)]
#[path = "../normal_callable_loop_source_facts_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "../normal_callable_loop_structural_lease_tests.rs"]
mod structural_lease_tests;

#[cfg(test)]
#[path = "generic/carrier_relation_tests.rs"]
mod carrier_relation_tests;

#[cfg(test)]
#[path = "generic/source_admission_tests.rs"]
mod source_admission_tests;

#[cfg(test)]
pub(in crate::mir::builder) use carrier_relation_tests::source_final_values_for_test;
