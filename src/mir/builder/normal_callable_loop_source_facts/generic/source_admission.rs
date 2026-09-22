//! Source-only evidence and admission for the GenericLoop route.
//!
//! The registry's raw V0/V1 schedule remains neutral.  This module owns the
//! small source boundary that can admit the already observed `[V0, V1]`
//! overlap without manufacturing a global route precedence rule.  Everything
//! here is structural transport; no language-semantic `Verified*`/`Prepared*`
//! receipt is issued.

use super::*;
use crate::mir::builder::control_flow::joinir::route_entry::registry::VerifiedLocatedGenericLoopV1SelectionV1;
use crate::mir::builder::normal_callable_loop_source_route::{
    CallableLoopSourceItemBindingV1, CallableLoopSourceItemDispositionV1,
    CallableLoopSourceRouteRejectV1, CallableLoopSourceTargetProbeV1,
};

/// One source session identity.  The route admission and its evidence must
/// come from these exact owner/source sites; later consumers cannot pair rows
/// by method name, count, or MIR shape.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) struct CallableGenericLoopSourceSessionKeyV1 {
    owner: FunctionOwnerIdV1,
    parent: SourceNodeSiteV1,
    condition: SourceNodeSiteV1,
    body: SourceNodeSiteV1,
}

impl CallableGenericLoopSourceSessionKeyV1 {
    fn new(
        owner: FunctionOwnerIdV1,
        parent: SourceNodeSiteV1,
        condition: SourceNodeSiteV1,
        body: SourceNodeSiteV1,
    ) -> Self {
        Self {
            owner,
            parent,
            condition,
            body,
        }
    }

    pub(in crate::mir::builder) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(in crate::mir::builder) fn parent(&self) -> &SourceNodeSiteV1 {
        &self.parent
    }

    pub(in crate::mir::builder) fn condition(&self) -> &SourceNodeSiteV1 {
        &self.condition
    }

    pub(in crate::mir::builder) fn body(&self) -> &SourceNodeSiteV1 {
        &self.body
    }
}

/// Structural source evidence issued before the source route token.
///
/// The pre-effect and carrier relation are move-only and remain paired with
/// the source item dispositions until the GenericLoop Recipe consumer reads
/// them.  An empty source-item batch is allowed for the existing exact V1
/// unit tests; the source bridge itself still owns the non-empty requirement
/// for the later ArrayPush acceptance row.
#[derive(Debug)]
pub(in crate::mir::builder) struct PreparedCallableGenericLoopSourceEvidenceV1<'source> {
    session: CallableGenericLoopSourceSessionKeyV1,
    parent_source: &'source RawInvocationSourceContextV1,
    condition_source: RawInvocationSourceContextV1,
    body_source: RawInvocationSourceContextV1,
    pre_effect: CallableSemanticLoopHandoffPreEffectReceiptV1,
    carrier_relation: CallableLoopCarrierRelationV1,
    source_items: Box<[CallableLoopSourceItemBindingV1]>,
    source_dispositions: Box<[CallableLoopSourceItemDispositionV1]>,
}

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::builder) enum CallableGenericLoopSourceEvidenceRejectV1 {
    ParentSiteMissing,
    ConditionSiteMissing,
    BodySiteMissing,
    PreEffectRejected(String),
    CarrierRelation(CallableLoopCarrierRelationRejectV1),
    SourceTarget(CallableLoopSourceRouteRejectV1),
}

impl<'source> PreparedCallableGenericLoopSourceEvidenceV1<'source> {
    pub(super) fn issue(
        owner: FunctionOwnerIdV1,
        parent_source: &'source RawInvocationSourceContextV1,
        condition_source: RawInvocationSourceContextV1,
        body_source: RawInvocationSourceContextV1,
        binding_product: CallableLoopReadyBodyOnlyProductV1,
        generic: &GenericLoopV1Facts,
        source_items: Box<[CallableLoopSourceItemBindingV1]>,
        source_target_probe: CallableLoopSourceTargetProbeV1,
    ) -> Result<Self, CallableGenericLoopSourceEvidenceRejectV1> {
        let parent_site = parent_source
            .site()
            .cloned()
            .ok_or(CallableGenericLoopSourceEvidenceRejectV1::ParentSiteMissing)?;
        let condition_site = condition_source
            .site()
            .cloned()
            .ok_or(CallableGenericLoopSourceEvidenceRejectV1::ConditionSiteMissing)?;
        let body_site = body_source
            .site()
            .cloned()
            .ok_or(CallableGenericLoopSourceEvidenceRejectV1::BodySiteMissing)?;
        let pre_effect = binding_product
            .consume_pre_effect(&parent_site, &condition_site, &body_site)
            .map_err(CallableGenericLoopSourceEvidenceRejectV1::PreEffectRejected)?;
        let carrier_relation =
            carrier_relation::issue_pre_route(owner, &pre_effect, &body_source, generic)
                .map_err(CallableGenericLoopSourceEvidenceRejectV1::CarrierRelation)?;
        let source_dispositions = source_target_probe
            .into_item_dispositions(&source_items)
            .map_err(CallableGenericLoopSourceEvidenceRejectV1::SourceTarget)?;
        Ok(Self {
            session: CallableGenericLoopSourceSessionKeyV1::new(
                owner,
                parent_site,
                condition_site,
                body_site,
            ),
            parent_source,
            condition_source,
            body_source,
            pre_effect,
            carrier_relation,
            source_items,
            source_dispositions,
        })
    }

    pub(super) fn session(&self) -> &CallableGenericLoopSourceSessionKeyV1 {
        &self.session
    }

    pub(super) fn carrier_relation(&self) -> &CallableLoopCarrierRelationV1 {
        &self.carrier_relation
    }

    pub(super) fn pre_effect(&self) -> &CallableSemanticLoopHandoffPreEffectReceiptV1 {
        &self.pre_effect
    }

    pub(super) fn parent_source(&self) -> &'source RawInvocationSourceContextV1 {
        self.parent_source
    }

    pub(super) fn condition_source(&self) -> &RawInvocationSourceContextV1 {
        &self.condition_source
    }

    pub(super) fn body_source(&self) -> &RawInvocationSourceContextV1 {
        &self.body_source
    }

    pub(super) fn source_items(&self) -> &[CallableLoopSourceItemBindingV1] {
        &self.source_items
    }

    pub(super) fn source_dispositions(&self) -> &[CallableLoopSourceItemDispositionV1] {
        &self.source_dispositions
    }
}

/// Source route kind.  `SourceOverlap` is an opaque local seal; it does not
/// alter the registry's raw schedule or create a global V0/V1 winner.
#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::builder) enum CallableGenericLoopSourceRouteKindV1 {
    Exact(VerifiedLocatedGenericLoopV1SelectionV1),
    SourceOverlap(CallableGenericLoopSourceOverlapSealV1),
}

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::builder) struct CallableGenericLoopSourceOverlapSealV1;

/// Move-only aggregate binding the raw selection and source evidence from one
/// source session.  It is intentionally structural and private to the source
/// GenericLoop owner.
#[derive(Debug)]
pub(in crate::mir::builder) struct CallableGenericLoopSourceRouteAdmissionV1<'source> {
    selection: RecipeFirstRouteSelectionV1,
    evidence: PreparedCallableGenericLoopSourceEvidenceV1<'source>,
    selected: CallableGenericLoopSourceRouteKindV1,
}

impl<'source> CallableGenericLoopSourceRouteAdmissionV1<'source> {
    pub(super) fn issue(
        selection: RecipeFirstRouteSelectionV1,
        evidence: PreparedCallableGenericLoopSourceEvidenceV1<'source>,
    ) -> Result<Self, CallableGenericLoopSourceFactsRouteErrorV1> {
        let selected = match selection.raw_execution_routes() {
            [LoopRouteId::GenericLoopV1] => selection
                .verify_located_generic_loop_v1()
                .map(CallableGenericLoopSourceRouteKindV1::Exact)
                .map_err(route_error),
            [LoopRouteId::GenericLoopV0, LoopRouteId::GenericLoopV1] => {
                Ok(CallableGenericLoopSourceRouteKindV1::SourceOverlap(
                    CallableGenericLoopSourceOverlapSealV1,
                ))
            }
            routes => Err(
                CallableGenericLoopSourceFactsRouteErrorV1::NonGenericOrOverlapping {
                    routes: routes.into(),
                },
            ),
        }?;
        Ok(Self {
            selection,
            evidence,
            selected,
        })
    }

    pub(super) fn selection(&self) -> &RecipeFirstRouteSelectionV1 {
        &self.selection
    }

    pub(super) fn selected(&self) -> &CallableGenericLoopSourceRouteKindV1 {
        &self.selected
    }

    pub(super) fn evidence(&self) -> &PreparedCallableGenericLoopSourceEvidenceV1<'source> {
        &self.evidence
    }
}
