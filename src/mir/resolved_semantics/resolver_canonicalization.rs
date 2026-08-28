//! Draft-to-resolved conversion owned by the resolver session.
//!
//! This child keeps canonical draft conversion separate from session lifecycle
//! so source-bound direct-call co-issuers can remain local.

use super::super::{FunctionOwnerIdV1, ResolvedExplicitExternCallV1, UpvarRefV1};
use super::*;

pub(super) fn canonicalize_draft(
    owner: FunctionOwnerIdV1,
    function_origin: FunctionOriginV1,
    mut draft: ShadowResolvedFunctionV0,
    ancestors: &BTreeMap<Box<str>, AncestorBindingV1>,
    callable_index: Option<&VerifiedCallableIndexV1>,
    direct_call_policy: DirectCallCanonicalizationPolicyV1,
) -> Result<CanonicalizedDraftV1, ResolveFunctionErrorV1> {
    let (direct_call_targets, direct_call_observations) = match (callable_index, direct_call_policy)
    {
        (Some(index), DirectCallCanonicalizationPolicyV1::RequireCallableIndex) => {
            let mut targets = BTreeMap::new();
            let mut observations = BTreeMap::new();
            for (site, call) in std::mem::take(&mut draft.direct_calls) {
                let header = index
                    .resolve_free_static_source_call(&call.name, call.arity)
                    .map_err(ResolveFunctionErrorV1::CallableLookup)?;
                let target = ResolvedDirectCallTargetV1::from_resolved(header.callable());
                let observation = ResolvedDirectCallObservationV1::from_parts(
                    call.name,
                    call.arity,
                    call.argument_sites,
                );
                if targets.insert(site.clone(), target).is_some()
                    || observations.insert(site, observation).is_some()
                {
                    return Err(ResolveFunctionErrorV1::DraftInvariant(
                        "direct-call site was issued twice",
                    ));
                }
            }
            if targets.keys().ne(observations.keys()) {
                return Err(ResolveFunctionErrorV1::DraftInvariant(
                    "direct-call target and observation sites differ",
                ));
            }
            (targets, observations)
        }
        (Some(index), DirectCallCanonicalizationPolicyV1::RequireCallableIndexAtRoot) => {
            // The root-only policy is lowered to `RequireCallableIndex` by
            // the recursive owner resolver.  Reaching this canonicalizer
            // directly would mean a child/root boundary was lost.
            let _ = index;
            return Err(ResolveFunctionErrorV1::DraftInvariant(
                "root-only direct-call policy must be handled by the owner resolver",
            ));
        }
        (None, DirectCallCanonicalizationPolicyV1::ObserveOnly) => (
            BTreeMap::new(),
            std::mem::take(&mut draft.direct_calls)
                .into_iter()
                .map(|(site, call)| {
                    (
                        site,
                        ResolvedDirectCallObservationV1::from_parts(
                            call.name,
                            call.arity,
                            call.argument_sites,
                        ),
                    )
                })
                .collect(),
        ),
        (None, DirectCallCanonicalizationPolicyV1::RejectUnindexed) => {
            if !draft.direct_calls.is_empty() {
                return Err(ResolveFunctionErrorV1::DraftInvariant(
                    "direct calls require a callable index",
                ));
            }
            (BTreeMap::new(), BTreeMap::new())
        }
        (None, DirectCallCanonicalizationPolicyV1::RequireCallableIndex) => {
            return Err(ResolveFunctionErrorV1::DraftInvariant(
                "callable index is required by direct-call policy",
            ));
        }
        (None, DirectCallCanonicalizationPolicyV1::RequireCallableIndexAtRoot) => {
            return Err(ResolveFunctionErrorV1::DraftInvariant(
                "root-only direct-call policy requires a callable index",
            ));
        }
        (Some(_), DirectCallCanonicalizationPolicyV1::ObserveOnly) => {
            return Err(ResolveFunctionErrorV1::DraftInvariant(
                "observer-only direct-call policy cannot use a callable index",
            ));
        }
        (Some(_), DirectCallCanonicalizationPolicyV1::RejectUnindexed) => {
            return Err(ResolveFunctionErrorV1::DraftInvariant(
                "reject-unindexed direct-call policy cannot use a callable index",
            ));
        }
    };
    let binding_ids = draft
        .bindings
        .keys()
        .map(|id| (*id, BindingId::new(id.raw())))
        .collect::<BTreeMap<_, _>>();
    let scope_ids = draft
        .scopes
        .keys()
        .map(|id| (*id, ScopeId::new(owner, id.raw())))
        .collect::<BTreeMap<_, _>>();
    let region_ids = draft
        .regions
        .keys()
        .map(|id| (*id, RegionId::new(owner, id.raw())))
        .collect::<BTreeMap<_, _>>();
    let scope_owner_regions = scope_owner_regions(&draft)?;

    let bindings = draft
        .bindings
        .iter()
        .map(|(id, record)| {
            let binding = binding_ids[id];
            let owner_scope = scope_ids[&record.owner_scope];
            (
                binding,
                ResolvedBindingRecordV1::new(
                    record.diagnostic_name.clone(),
                    binding_kind(record.kind),
                    owner_scope,
                    BindingOriginV1::Source(record.origin.clone()),
                ),
            )
        })
        .collect();

    let scopes = draft
        .scopes
        .iter()
        .map(|(id, record)| {
            let scope = scope_ids[id];
            let origin = if *id == draft.function_scope {
                ScopeOriginV1::Function(function_origin)
            } else {
                ScopeOriginV1::Source(record.origin.clone().ok_or(
                    ResolveFunctionErrorV1::DraftInvariant(
                        "non-function scope lacks source origin",
                    ),
                )?)
            };
            let declarations = record
                .declarations
                .iter()
                .map(|id| BindingRefV1::new(owner, binding_ids[id]))
                .collect();
            Ok((
                scope,
                ResolvedScopeRecordV1::new(
                    scope_kind(record.kind),
                    record.parent.map(|id| scope_ids[&id]),
                    region_ids[&scope_owner_regions[id]],
                    declarations,
                    origin,
                ),
            ))
        })
        .collect::<Result<BTreeMap<_, _>, ResolveFunctionErrorV1>>()?;

    let regions = draft
        .regions
        .iter()
        .map(|(id, record)| {
            let origin = if *id == draft.function_region {
                RegionOriginV1::Function(function_origin)
            } else {
                RegionOriginV1::Source(record.origin.clone().ok_or(
                    ResolveFunctionErrorV1::DraftInvariant(
                        "non-function region lacks source origin",
                    ),
                )?)
            };
            Ok((
                region_ids[id],
                ResolvedRegionRecordV1::new(
                    region_kind(record.kind),
                    record.parent.map(|id| region_ids[&id]),
                    record.lexical_scope.map(|id| scope_ids[&id]),
                    origin,
                ),
            ))
        })
        .collect::<Result<BTreeMap<_, _>, ResolveFunctionErrorV1>>()?;

    let binding_ref = |id: ShadowBindingOrdinalV0| BindingRefV1::new(owner, binding_ids[&id]);
    let declarations = draft
        .declarations
        .iter()
        .map(|(site, id)| (site.clone(), binding_ref(*id)))
        .collect();
    let variable_uses = draft
        .variable_uses
        .iter()
        .map(|(site, lexical_ref)| {
            let lexical_ref = match lexical_ref {
                ShadowLexicalRefV0::Local(id) => ResolvedLexicalRefV1::Local(binding_ref(*id)),
                ShadowLexicalRefV0::Ancestor(name) => {
                    let ancestor =
                        ancestors
                            .get(name)
                            .ok_or(ResolveFunctionErrorV1::DraftInvariant(
                                "shadow ancestor reference lacks canonical source",
                            ))?;
                    let upvar = UpvarRefV1::new(owner, ancestor.reference);
                    ResolvedLexicalRefV1::Upvar(upvar)
                }
            };
            Ok((site.clone(), lexical_ref))
        })
        .collect::<Result<BTreeMap<_, _>, ResolveFunctionErrorV1>>()?;
    let body_shape = seal_shadow_body_shape(
        owner,
        draft.root_profile,
        draft.body_shape.clone(),
        &variable_uses,
        &draft.statement_sites,
        &draft.expression_sites,
    )
    .map_err(ResolveFunctionErrorV1::DraftInvariant)?;
    let method_calls = issue_resolved_method_call_sources_v1(&body_shape)
        .map_err(ResolveFunctionErrorV1::MethodCallSource)?;
    let assignment_targets = draft
        .assignment_targets
        .iter()
        .map(|(site, target)| {
            let target = match target {
                ShadowAssignmentTargetV0::BindingRebind(id) => {
                    ResolvedAssignmentTargetV1::BindingRebind(binding_ref(*id))
                }
                ShadowAssignmentTargetV0::AncestorRebind(name) => {
                    let ancestor =
                        ancestors
                            .get(name)
                            .ok_or(ResolveFunctionErrorV1::DraftInvariant(
                                "shadow ancestor rebind lacks canonical source",
                            ))?;
                    ResolvedAssignmentTargetV1::UpvarRebind(UpvarRefV1::new(
                        owner,
                        ancestor.reference,
                    ))
                }
                ShadowAssignmentTargetV0::FieldWrite { receiver } => {
                    ResolvedAssignmentTargetV1::FieldWrite {
                        receiver: receiver.clone(),
                    }
                }
                ShadowAssignmentTargetV0::IndexWrite { receiver } => {
                    ResolvedAssignmentTargetV1::IndexWrite {
                        receiver: receiver.clone(),
                    }
                }
            };
            Ok((site.clone(), target))
        })
        .collect::<Result<BTreeMap<_, _>, ResolveFunctionErrorV1>>()?;
    let expression_source =
        seal_shadow_expression_source_v1(std::mem::take(&mut draft.expression_source), binding_ref)
            .map_err(ResolveFunctionErrorV1::DraftInvariant)?;
    let resolved_exits = draft
        .resolved_exits
        .iter()
        .map(|(site, record)| {
            let origin = match record.origin {
                ShadowExitOriginV0::ExplicitContinue => ResolvedExitOriginV1::ExplicitContinue,
                ShadowExitOriginV0::ExplicitBreak => ResolvedExitOriginV1::ExplicitBreak,
                ShadowExitOriginV0::ExplicitReturn => ResolvedExitOriginV1::ExplicitReturn,
            };
            let transfer = match record.transfer {
                ShadowControlExitV0::Continue { target_loop } => {
                    ResolvedControlTransferV1::Continue {
                        target_loop: region_ids[&target_loop],
                    }
                }
                ShadowControlExitV0::Break { target_loop } => ResolvedControlTransferV1::Break {
                    target_loop: region_ids[&target_loop],
                },
                ShadowControlExitV0::Return { target_function } => {
                    ResolvedControlTransferV1::Return {
                        target_function: region_ids[&target_function],
                    }
                }
            };
            (
                ResolvedExitSiteV1::Statement(site.clone()),
                ResolvedExitRecordV1::new(region_ids[&record.source_region], origin, transfer),
            )
        })
        .collect();

    let brand_call_relations = seal_brand_call_source_relations_v1(
        owner,
        std::mem::take(&mut draft.brand_calls),
        &draft.expression_sites,
    )
    .map_err(ResolveFunctionErrorV1::BrandSourceRelation)?;
    let explicit_extern_calls = draft
        .explicit_extern_calls
        .into_iter()
        .map(|(site, call)| (site, ResolvedExplicitExternCallV1::from_source(call.symbol)))
        .collect();

    let mut seen_capture_bindings = std::collections::BTreeSet::new();
    let mut ordered_capture_demands = Vec::new();
    for event in &draft.ancestor_capture_events {
        let source_binding = ancestors
            .get(&event.name)
            .ok_or(ResolveFunctionErrorV1::DraftInvariant(
                "shadow capture event has no canonical ancestor binding",
            ))?
            .reference;
        if seen_capture_bindings.insert(source_binding) {
            ordered_capture_demands.push(OrderedCaptureDemandV1::new(
                source_binding,
                event.site.clone(),
                match event.access {
                    ShadowAncestorCaptureAccessV0::Read => UpvarAccessKindV1::Read,
                    ShadowAncestorCaptureAccessV0::Rebind => UpvarAccessKindV1::Rebind,
                },
            ));
        }
    }
    let ordered_capture_demands = ordered_capture_demands.into_boxed_slice();

    let data = ResolvedFunctionDataV1 {
        owner,
        function_origin,
        root_profile: draft.root_profile,
        function_scope: scope_ids[&draft.function_scope],
        function_region: region_ids[&draft.function_region],
        bindings,
        scopes,
        regions,
        declarations,
        variable_uses,
        assignment_targets,
        direct_call_targets,
        direct_call_observations,
        brand_call_relations,
        explicit_extern_calls,
        method_calls,
        expression_source,
        resolved_exits,
    };
    let mut source_sites = ResolvedSourceSiteInventoryDraftV1::default();
    for site in draft.statement_sites {
        source_sites.record_statement(site);
    }
    for site in draft.expression_sites {
        source_sites.record_expression(site);
    }
    let binding_refs = binding_ids
        .into_iter()
        .map(|(shadow, binding)| (shadow, BindingRefV1::new(owner, binding)))
        .collect();
    Ok(CanonicalizedDraftV1 {
        data,
        source_sites,
        binding_refs,
        scope_ids,
        ordered_capture_demands,
        body_shape,
    })
}

fn scope_owner_regions(
    draft: &ShadowResolvedFunctionV0,
) -> Result<BTreeMap<ShadowScopeIdV0, ShadowRegionIdV0>, ResolveFunctionErrorV1> {
    let mut owners = BTreeMap::new();
    for (region, record) in &draft.regions {
        if let Some(scope) = record.lexical_scope {
            if owners.insert(scope, *region).is_some() {
                return Err(ResolveFunctionErrorV1::DraftInvariant(
                    "scope has multiple owner regions",
                ));
            }
        }
    }
    if draft.scopes.keys().any(|scope| !owners.contains_key(scope)) {
        return Err(ResolveFunctionErrorV1::DraftInvariant(
            "scope lacks owner region",
        ));
    }
    Ok(owners)
}

const fn binding_kind(kind: ShadowBindingKindV0) -> BindingKindV1 {
    match kind {
        ShadowBindingKindV0::Receiver => BindingKindV1::Receiver,
        ShadowBindingKindV0::Parameter { index } => BindingKindV1::Parameter { index },
        ShadowBindingKindV0::Local { ordinal } => BindingKindV1::Local { ordinal },
        ShadowBindingKindV0::Outbox { ordinal } => BindingKindV1::Outbox { ordinal },
        ShadowBindingKindV0::Nowait => BindingKindV1::Nowait,
    }
}

const fn scope_kind(kind: ShadowScopeKindV0) -> ScopeKindV1 {
    match kind {
        ShadowScopeKindV0::Function => ScopeKindV1::Function,
        ShadowScopeKindV0::LexicalBlock => ScopeKindV1::LexicalBlock,
        ShadowScopeKindV0::BlockExpr => ScopeKindV1::BlockExpr,
        ShadowScopeKindV0::IfThen => ScopeKindV1::IfThen,
        ShadowScopeKindV0::IfElse => ScopeKindV1::IfElse,
        ShadowScopeKindV0::LoopBody => ScopeKindV1::LoopBody,
    }
}

const fn region_kind(kind: ShadowRegionKindV0) -> RegionKindV1 {
    match kind {
        ShadowRegionKindV0::Function => RegionKindV1::Function,
        ShadowRegionKindV0::Sequence => RegionKindV1::Sequence,
        ShadowRegionKindV0::LexicalScope => RegionKindV1::LexicalScope,
        ShadowRegionKindV0::BlockExpr => RegionKindV1::BlockExpr,
        ShadowRegionKindV0::If => RegionKindV1::If,
        ShadowRegionKindV0::IfThen => RegionKindV1::IfThen,
        ShadowRegionKindV0::IfElse => RegionKindV1::IfElse,
        ShadowRegionKindV0::Loop => RegionKindV1::Loop,
    }
}
