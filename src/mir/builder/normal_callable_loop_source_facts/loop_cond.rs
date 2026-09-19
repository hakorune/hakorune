//! LoopCond source-facts extension for the selected parser loop.
//!
//! This module keeps the existing planner outcome as the Facts/Recipe source
//! and only co-seals the resolver forest, source item rows, and canonical
//! target relation. Physical LoopCond lowering is a later task.

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::joinir::route_entry::registry::RecipeFirstRouteSelectionV1;
use crate::mir::builder::control_flow::plan::PlanBuildOutcome;
use crate::mir::builder::normal_callable_loop_handoff::CallableLoopReadyBodyOnlyProductV1;
use crate::mir::builder::normal_callable_loop_source_facts::CallableGenericLoopSourceFactsRouteErrorV1;
use crate::mir::builder::normal_callable_loop_source_route::{
    CallableLoopSourceItemBindingV1, CallableLoopSourceRouteRejectV1,
    CallableLoopSourceRouteTokenV1, CallableLoopSourceTargetRelationV1,
};
use crate::mir::builder::raw_invocation_source_transport::RawInvocationSourceContextV1;
use crate::mir::loop_structural_facts::VerifiedLoopCondBreakContinueSourceForestProjectionV1;
use crate::mir::resolved_semantics::{
    FunctionOriginV1, FunctionOwnerIdV1, SemanticOwnerSourceKindV1,
};

#[derive(Debug)]
pub(in crate::mir::builder) struct CallableLoopCondSourceFactsV1<'source> {
    _owner: FunctionOwnerIdV1,
    _parent_source: &'source RawInvocationSourceContextV1,
    _condition_source: RawInvocationSourceContextV1,
    _body_source: RawInvocationSourceContextV1,
    _condition: ASTNode,
    _body: Vec<ASTNode>,
    _binding_product: CallableLoopReadyBodyOnlyProductV1,
    route_token: CallableLoopSourceRouteTokenV1,
}

impl CallableLoopCondSourceFactsV1<'_> {
    pub(in crate::mir::builder) fn into_route_token(self) -> CallableLoopSourceRouteTokenV1 {
        self.route_token
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
    selection: RecipeFirstRouteSelectionV1,
    projection: Option<VerifiedLoopCondBreakContinueSourceForestProjectionV1>,
    source_items: Box<[CallableLoopSourceItemBindingV1]>,
    source_target: Option<CallableLoopSourceTargetRelationV1>,
) -> Result<CallableLoopCondSourceFactsV1<'source>, CallableGenericLoopSourceFactsRouteErrorV1> {
    let function_origin = function_origin.ok_or_else(|| {
        CallableGenericLoopSourceFactsRouteErrorV1::LoopCondRouteRejected(
            CallableLoopSourceRouteRejectV1::SourceIdentityMissing,
        )
    })?;
    let source_kind = source_kind.ok_or_else(|| {
        CallableGenericLoopSourceFactsRouteErrorV1::LoopCondRouteRejected(
            CallableLoopSourceRouteRejectV1::SourceIdentityMissing,
        )
    })?;
    let parent_site = parent_source.site().cloned().ok_or_else(|| {
        CallableGenericLoopSourceFactsRouteErrorV1::LoopCondRouteRejected(
            CallableLoopSourceRouteRejectV1::SourceParentMissing,
        )
    })?;
    let route_token = CallableLoopSourceRouteTokenV1::issue_with_source_relations(
        owner,
        parent_site,
        function_origin,
        source_kind,
        outcome,
        selection,
        projection,
        source_items,
        source_target,
    )
    .map_err(CallableGenericLoopSourceFactsRouteErrorV1::LoopCondRouteRejected)?;
    Ok(CallableLoopCondSourceFactsV1 {
        _owner: owner,
        _parent_source: parent_source,
        _condition_source: condition_source,
        _body_source: body_source,
        _condition: condition,
        _body: body,
        _binding_product: binding_product,
        route_token,
    })
}
