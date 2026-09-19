//! Pure raw-Loop child-entry quarantine.
//!
//! This module answers one question before the future invocation-aware raw
//! Loop boundary calls `cf_loop`: can this exact raw syntax enter a Box child
//! function? It owns no Builder, JoinIR route, module collector, header port,
//! source-site identity, or AST rewrite authority.

use crate::ast::ASTNode;
use crate::mir::resolved_semantics::{
    BodyChildRoleV1, ExprChildRoleV1, SourceBodyKindV1, SourcePathSegmentV1,
};
use crate::mir::{MirBuilder, ValueId};

use super::generic_loop_admission_observation::GenericLoopAdmissionObservationV1;
use super::module_invocation_session::UnpublishedCallableLoopRootScopeV1;
use super::normal_callable_loop_handoff::{
    CallableLoopBindingProjectionDispositionV1, CallableLoopOutsideReasonV1,
    CallableLoopReadyBodyOnlyProductV1, VerifiedCallableSemanticLoopBindingScheduleV1,
};
use super::normal_callable_loop_physical_adapter::CallableGenericLoopV1PhysicalAdapterV1;
use super::normal_callable_loop_source_facts::{
    CallableGenericLoopSourceFactsDispositionV1, CallableGenericLoopSourceFactsIssuerV1,
};
use super::normal_callable_loop_source_route::{
    CallableLoopSourceItemBindingV1, CallableLoopSourceTargetRelationV1,
};
use super::normal_callable_semantic_lowering_state::CallableLoopSourceBridgeTakeV1;
use super::raw_invocation_source_transport::RawInvocationSourceContextV1;
use crate::mir::builder::control_flow::plan::GenericLoopFactsPolicyFrameV1;
use crate::mir::resolved_semantics::FunctionOwnerIdV1;
use crate::parser::CallableMethodSourceObservationV1;

#[path = "raw_loop_child_entry/ledger_bridge.rs"]
mod ledger_bridge;

/// Exact child-entry result for one raw Loop syntax surface.
///
/// `NoChildFunctionEntry` is deliberately narrow: it says only that the
/// executable syntax has no reachable `BoxDeclaration`. It does not prove a
/// JoinIR route, recipe, CFG, type fact, or general Loop acceptance.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) enum RawLoopChildEntryDispositionV1 {
    NoChildFunctionEntry,
    ReachableBoxDeclaration,
}

/// One located invocation Loop admitted at the existing child-entry seam.
///
/// The exact parent, condition, and body-root receipts stay owned until the
/// single route terminal returns. This product deliberately does not claim
/// located JoinIR planning; it only prevents the invocation path from erasing
/// source location before the current route owner.
pub(in crate::mir::builder) struct PreparedLocatedRawLoopChildEntryV1<'source> {
    parent_source: &'source RawInvocationSourceContextV1,
    condition_source: RawInvocationSourceContextV1,
    body_source: RawInvocationSourceContextV1,
    condition: ASTNode,
    body: Vec<ASTNode>,
    disposition: RawLoopChildEntryDispositionV1,
    callable_handoff: Option<CallableLoopBindingProjectionDispositionV1>,
    method_source_observation: Option<CallableMethodSourceObservationV1>,
    admission_observation: Option<GenericLoopAdmissionObservationV1>,
}

/// Opaque move-only payload assembled from one prepared Loop entry.
///
/// The source-aware issuer receives this aggregate, not independently
/// supplied AST/source fragments.  The fields are visible only inside the
/// Builder tree so the sole issuer can destructure this one aggregate; they
/// are never exposed as an independently pairable tuple.
#[derive(Debug)]
pub(in crate::mir::builder) struct PreparedCallableGenericLoopSourceFactsPayloadV1<'source> {
    pub(in crate::mir::builder) parent_source: &'source RawInvocationSourceContextV1,
    pub(in crate::mir::builder) condition_source: RawInvocationSourceContextV1,
    pub(in crate::mir::builder) body_source: RawInvocationSourceContextV1,
    pub(in crate::mir::builder) condition: ASTNode,
    pub(in crate::mir::builder) body: Vec<ASTNode>,
    pub(in crate::mir::builder) owner: FunctionOwnerIdV1,
    pub(in crate::mir::builder) binding_product: CallableLoopReadyBodyOnlyProductV1,
    pub(in crate::mir::builder) function_name: Box<str>,
    pub(in crate::mir::builder) debug: bool,
    pub(in crate::mir::builder) in_static_box: bool,
    pub(in crate::mir::builder) policy: GenericLoopFactsPolicyFrameV1,
    pub(in crate::mir::builder) function_origin:
        Option<crate::mir::resolved_semantics::FunctionOriginV1>,
    pub(in crate::mir::builder) source_kind:
        Option<crate::mir::resolved_semantics::SemanticOwnerSourceKindV1>,
    pub(in crate::mir::builder) source_projection: Option<
        crate::mir::loop_structural_facts::VerifiedLoopCondBreakContinueSourceForestProjectionV1,
    >,
    pub(in crate::mir::builder) source_items: Box<[CallableLoopSourceItemBindingV1]>,
    pub(in crate::mir::builder) source_target: Option<CallableLoopSourceTargetRelationV1>,
}

impl<'source> PreparedLocatedRawLoopChildEntryV1<'source> {
    pub(in crate::mir::builder) fn prepare(
        parent_source: &'source RawInvocationSourceContextV1,
        loop_node: ASTNode,
        callable_handoff: Option<CallableLoopBindingProjectionDispositionV1>,
    ) -> Result<Self, String> {
        Self::prepare_with_method_source_observation(
            parent_source,
            loop_node,
            callable_handoff,
            None,
            None,
        )
    }

    pub(in crate::mir::builder) fn prepare_with_method_source_observation(
        parent_source: &'source RawInvocationSourceContextV1,
        loop_node: ASTNode,
        callable_handoff: Option<CallableLoopBindingProjectionDispositionV1>,
        method_source_observation: Option<CallableMethodSourceObservationV1>,
        admission_observation: Option<GenericLoopAdmissionObservationV1>,
    ) -> Result<Self, String> {
        if !matches!(&parent_source, RawInvocationSourceContextV1::Located { .. }) {
            return Err(
                "[freeze:contract][raw-loop-child-entry/requires-located-loop-source]".to_owned(),
            );
        }
        if let Some(admission) = admission_observation.as_ref() {
            let Some(method) = method_source_observation.as_ref() else {
                return Err(
                    "[freeze:contract][raw-loop-child-entry/missing-method-source-observation]"
                        .to_owned(),
                );
            };
            if !admission
                .method_source()
                .identity()
                .same_as(method.identity())
                || !admission
                    .method_source()
                    .parser_provenance()
                    .same_as(method.parser_provenance())
                || admission.method_source().source_site() != method.source_site()
            {
                return Err(
                    "[freeze:contract][raw-loop-child-entry/foreign-method-source-observation]"
                        .to_owned(),
                );
            }
        }

        let condition_source =
            parent_source.child_expression(&loop_node, ExprChildRoleV1::LoopCondition)?;
        let body_source = parent_source.child_body(&loop_node, BodyChildRoleV1::LoopBody)?;
        verify_exact_loop_child_receipts(&condition_source, &body_source)?;

        let ASTNode::Loop {
            condition, body, ..
        } = loop_node
        else {
            return Err("[freeze:contract][raw-loop-child-entry/expected-loop]".to_owned());
        };
        let disposition = classify_raw_loop_child_entry_v1(&condition, &body);

        Ok(Self {
            parent_source,
            condition_source,
            body_source,
            condition: *condition,
            body,
            disposition,
            callable_handoff,
            method_source_observation,
            admission_observation,
        })
    }

    pub(in crate::mir::builder) fn lower_v1(
        self,
        builder: &mut MirBuilder,
        function_name: &str,
        debug: bool,
        in_static_box: bool,
        policy: GenericLoopFactsPolicyFrameV1,
    ) -> Result<ValueId, String> {
        self.lower_v1_with_optional_root_scope(
            builder,
            function_name,
            debug,
            in_static_box,
            policy,
            None,
            None,
            None,
        )
    }

    pub(in crate::mir::builder) fn lower_v1_with_root_scope(
        self,
        builder: &mut MirBuilder,
        function_name: &str,
        debug: bool,
        in_static_box: bool,
        policy: GenericLoopFactsPolicyFrameV1,
        callable_loop_root_scope: &mut UnpublishedCallableLoopRootScopeV1,
    ) -> Result<ValueId, String> {
        self.lower_v1_with_optional_root_scope(
            builder,
            function_name,
            debug,
            in_static_box,
            policy,
            Some(callable_loop_root_scope),
            None,
            None,
        )
    }

    fn lower_v1_with_optional_root_scope(
        self,
        builder: &mut MirBuilder,
        function_name: &str,
        debug: bool,
        in_static_box: bool,
        policy: GenericLoopFactsPolicyFrameV1,
        mut callable_loop_root_scope: Option<&mut UnpublishedCallableLoopRootScopeV1>,
        callable_ledger: Option<
            &std::rc::Rc<
                std::cell::RefCell<
                    super::normal_callable_semantic_lowering_state::CallableSemanticLoweringState,
                >,
            >,
        >,
        source_target: Option<CallableLoopSourceTargetRelationV1>,
    ) -> Result<ValueId, String> {
        let Self {
            parent_source,
            condition_source,
            body_source,
            condition,
            body,
            disposition,
            callable_handoff,
            method_source_observation: _,
            admission_observation: _,
        } = self;
        if disposition == RawLoopChildEntryDispositionV1::ReachableBoxDeclaration {
            return Err(
                super::control_flow::lower::Freeze::contract(
                    "raw_loop_child_entry: reachable BoxDeclaration requires a pure-plan/function-session bridge",
                )
                .to_string(),
            );
        }

        let binding_product = match callable_handoff {
            Some(CallableLoopBindingProjectionDispositionV1::Ready(schedule)) => {
                CallableLoopReadyBodyOnlyProductV1::without_body_only(schedule)
            }
            Some(CallableLoopBindingProjectionDispositionV1::ReadyWithBodyOnly(product)) => product,
            Some(CallableLoopBindingProjectionDispositionV1::Outside(reason)) => {
                return lower_outside_callable_loop_v1(reason)
            }
            None => return lower_non_callable_loop_legacy_v1(builder, condition, body),
        };
        let owner = binding_product.owner();
        let (function_origin, source_kind, source_projection, source_items) =
            if let Some(callable_ledger) = callable_ledger {
                let Some(parent_site) = parent_source.site() else {
                    return Err(
                        "[freeze:contract][callable-loop/source-bridge/parent-site-missing]"
                            .to_owned(),
                    );
                };
                let state = callable_ledger.borrow();
                let items = state.source_loop_items(parent_site).unwrap_or_default();
                let function_origin = Some(state.function_origin());
                let source_kind = Some(state.source_kind());
                drop(state);
                let projection = match callable_ledger
                    .borrow_mut()
                    .take_source_loop_bridge(parent_site)?
                {
                    CallableLoopSourceBridgeTakeV1::Armed(projection) => Some(projection),
                    CallableLoopSourceBridgeTakeV1::Unarmed
                    | CallableLoopSourceBridgeTakeV1::BridgeAbsent => None,
                };
                (function_origin, source_kind, projection, items)
            } else {
                (None, None, None, Box::default())
            };
        let prepared = Self {
            parent_source,
            condition_source,
            body_source,
            condition,
            body,
            disposition,
            callable_handoff: Some(
                CallableLoopBindingProjectionDispositionV1::ReadyWithBodyOnly(binding_product),
            ),
            method_source_observation: None,
            admission_observation: None,
        };
        let payload = prepared
            .into_callable_generic_loop_source_facts_payload_with_source_relations(
                owner,
                function_name,
                debug,
                in_static_box,
                policy,
                function_origin,
                source_kind,
                source_projection,
                source_items,
                source_target,
            )?;
        let source_facts = match CallableGenericLoopSourceFactsIssuerV1::issue_once(payload) {
            CallableGenericLoopSourceFactsDispositionV1::Ready(source_facts) => source_facts,
            CallableGenericLoopSourceFactsDispositionV1::LoopCondReady(source_facts) => {
                let source_ledger = callable_ledger.ok_or_else(|| {
                    "[freeze:contract][callable-loop/loop-cond/source-port-ledger-missing]"
                        .to_owned()
                })?;
                let physical_input = source_facts.into_physical_input(source_ledger)?;
                physical_input.validate_for_source_port()?;
                physical_input.preflight_source_port()?;
                return Err(
                    "[freeze:contract][callable-loop/loop-cond/source-port-lowering-missing]"
                        .to_owned(),
                );
            }
            CallableGenericLoopSourceFactsDispositionV1::SourceUnavailable(error) => {
                return Err(format!(
                    "[freeze:contract][callable-loop/source-unavailable] {error:?}"
                ));
            }
            CallableGenericLoopSourceFactsDispositionV1::FactsAbsent => {
                return Err("[freeze:contract][callable-loop/facts-absent]".to_owned());
            }
            CallableGenericLoopSourceFactsDispositionV1::FactsRejected(error) => {
                return Err(format!(
                    "[freeze:contract][callable-loop/facts-rejected] {error}"
                ));
            }
            CallableGenericLoopSourceFactsDispositionV1::RouteNotFrontSelected(error) => {
                return Err(format!(
                    "[freeze:contract][callable-loop/route-not-front-selected] {error:?}"
                ));
            }
        };
        let receipt = source_facts
            .claim_all()
            .map_err(|error| format!("[freeze:contract][callable-loop/source-claim] {error:?}"))?;
        let recipe = receipt.into_semantic_recipe().map_err(|error| {
            format!("[freeze:contract][callable-loop/semantic-recipe] {error:?}")
        })?;
        let root_scope = callable_loop_root_scope
            .as_deref_mut()
            .ok_or_else(|| "[freeze:contract][callable-loop/root-scope/missing]".to_owned())?;
        let callable_ledger = callable_ledger
            .ok_or_else(|| "[freeze:contract][callable-loop/callable-ledger/missing]".to_owned())?;
        CallableGenericLoopV1PhysicalAdapterV1::lower(builder, root_scope, recipe, callable_ledger)
    }

    /// Move the exact prepared Loop into the sole source-aware Facts issuer.
    pub(in crate::mir::builder) fn into_callable_generic_loop_source_facts_payload(
        self,
        owner: FunctionOwnerIdV1,
        function_name: &str,
        debug: bool,
        in_static_box: bool,
        policy: GenericLoopFactsPolicyFrameV1,
    ) -> Result<PreparedCallableGenericLoopSourceFactsPayloadV1<'source>, String> {
        self.into_callable_generic_loop_source_facts_payload_with_source_relations(
            owner,
            function_name,
            debug,
            in_static_box,
            policy,
            None,
            None,
            None,
            Box::default(),
            None,
        )
    }

    /// Extended source payload used by the selected LoopCond bridge. The
    /// compatibility wrapper above keeps existing GenericLoop tests and
    /// callers on the same preparation contract.
    pub(in crate::mir::builder) fn into_callable_generic_loop_source_facts_payload_with_source_relations(
        self,
        owner: FunctionOwnerIdV1,
        function_name: &str,
        debug: bool,
        in_static_box: bool,
        policy: GenericLoopFactsPolicyFrameV1,
        function_origin: Option<crate::mir::resolved_semantics::FunctionOriginV1>,
        source_kind: Option<crate::mir::resolved_semantics::SemanticOwnerSourceKindV1>,
        source_projection: Option<
            crate::mir::loop_structural_facts::VerifiedLoopCondBreakContinueSourceForestProjectionV1,
        >,
        source_items: Box<[CallableLoopSourceItemBindingV1]>,
        source_target: Option<CallableLoopSourceTargetRelationV1>,
    ) -> Result<PreparedCallableGenericLoopSourceFactsPayloadV1<'source>, String> {
        let Self {
            parent_source,
            condition_source,
            body_source,
            condition,
            body,
            disposition,
            callable_handoff,
            method_source_observation: _,
            admission_observation: _,
        } = self;
        if disposition != RawLoopChildEntryDispositionV1::NoChildFunctionEntry {
            return Err(
                "[freeze:contract][callable-loop-source-facts/reachable-child-entry]".to_owned(),
            );
        }
        let binding_product = match callable_handoff {
            Some(CallableLoopBindingProjectionDispositionV1::Ready(schedule)) => {
                CallableLoopReadyBodyOnlyProductV1::without_body_only(schedule)
            }
            Some(CallableLoopBindingProjectionDispositionV1::ReadyWithBodyOnly(product)) => product,
            Some(CallableLoopBindingProjectionDispositionV1::Outside(_)) => {
                return Err(
                    "[freeze:contract][callable-loop-source-facts/outside-schedule]".to_owned(),
                )
            }
            None => {
                return Err(
                    "[freeze:contract][callable-loop-source-facts/missing-schedule]".to_owned(),
                )
            }
        };
        Ok(PreparedCallableGenericLoopSourceFactsPayloadV1 {
            parent_source,
            condition_source,
            body_source,
            condition,
            body,
            owner,
            binding_product,
            function_name: function_name.into(),
            debug,
            in_static_box,
            policy,
            function_origin,
            source_kind,
            source_projection,
            source_items,
            source_target,
        })
    }
}

fn lower_outside_callable_loop_v1(reason: CallableLoopOutsideReasonV1) -> Result<ValueId, String> {
    // Ordinary JoinIR currently accepts only MirBuilder and cannot consume
    // the callable source ledger.  Keep Outside terminal until a named
    // ledger-aware bridge is designed; never create partial MIR and defer the
    // source-consumption failure to CallableSemanticLoweringState::finish.
    Err(reason.into_terminal_error())
}

fn lower_non_callable_loop_legacy_v1(
    builder: &mut MirBuilder,
    condition: ASTNode,
    body: Vec<ASTNode>,
) -> Result<ValueId, String> {
    super::control_flow::joinir::routing::lower_loop_or_freeze_v1(builder, condition, body)
}

fn verify_exact_loop_child_receipts(
    condition: &RawInvocationSourceContextV1,
    body: &RawInvocationSourceContextV1,
) -> Result<(), String> {
    let condition_is_exact = condition
        .site()
        .is_some_and(|site| site.segments().last() == Some(&SourcePathSegmentV1::LoopCondition));
    let body_is_exact = matches!(
        body,
        RawInvocationSourceContextV1::Located {
            site,
            body_kind: Some(SourceBodyKindV1::Loop),
            ..
        } if site.segments().last() == Some(&SourcePathSegmentV1::LoopBodyRoot)
    );
    if condition_is_exact && body_is_exact {
        Ok(())
    } else {
        Err("[freeze:contract][raw-loop-child-entry/exact-child-receipts]".to_owned())
    }
}

/// Classify the original condition and body of one raw `ASTNode::Loop`.
///
/// The AST traversal API is the generic child-topology SSOT. Lambda and nested
/// function declaration bodies are deferred ownership surfaces: neither is
/// executed by the surrounding raw Loop lowering, so this classifier does not
/// descend into them. A `BoxDeclaration` itself is executable on the raw
/// dispatcher path and is therefore a direct child-entry boundary.
pub(in crate::mir::builder) fn classify_raw_loop_child_entry_v1(
    condition: &ASTNode,
    body: &[ASTNode],
) -> RawLoopChildEntryDispositionV1 {
    let has_child_entry = contains_reachable_box_declaration(condition)
        || body.iter().any(contains_reachable_box_declaration);

    if has_child_entry {
        RawLoopChildEntryDispositionV1::ReachableBoxDeclaration
    } else {
        RawLoopChildEntryDispositionV1::NoChildFunctionEntry
    }
}

fn contains_reachable_box_declaration(node: &ASTNode) -> bool {
    match node {
        ASTNode::BoxDeclaration { .. } => true,
        ASTNode::Lambda { .. } | ASTNode::FunctionDeclaration { .. } => false,
        _ => node.any_child(contains_reachable_box_declaration),
    }
}

#[cfg(test)]
#[path = "raw_loop_child_entry/tests.rs"]
mod tests;
