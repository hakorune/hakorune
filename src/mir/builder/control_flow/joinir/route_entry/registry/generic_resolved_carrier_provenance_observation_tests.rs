//! D3-S2-S0: resolver-owned Generic provenance observation only.
//!
//! This witness deliberately stops before Generic facts, logical recipe keys,
//! seed pairing, or route selection. It consumes the existing parsed source
//! and sealed resolver products so labels and plan ValueIds cannot become
//! source authority.

use super::generic_nested_carrier_bindingref_tests::{
    inner_loop_site, outer_loop_site, parse_function, post_loop_read_site, read_binding,
    resolved_binding, write_site, SHADOWING_SOURCE, SOURCE,
};
use crate::mir::resolved_semantics::{
    BindingRefV1, FunctionOriginV1, FunctionOwnerIdV1, FunctionSemanticResolverSessionV1,
    FunctionSyntaxViewV1, LoopExecutionFrameKeyV1, SemanticOwnerSourceKindV1, SourceExprSiteV1,
    SourceStmtSiteV1, VerifiedResolvedFunctionV1, VerifiedResolvedLoopSourceForestV1,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ProvenanceRoleV1 {
    NestedWrite,
    PostLoopRead,
}

#[derive(Debug, PartialEq, Eq)]
struct ProvenanceRoleClaimV1 {
    role: ProvenanceRoleV1,
    site: SourceExprSiteV1,
    binding: BindingRefV1,
    strict_ancestor: bool,
}

#[derive(Debug, PartialEq, Eq)]
struct ResolvedCarrierObservationV1 {
    owner: FunctionOwnerIdV1,
    function_origin: FunctionOriginV1,
    source_kind: SemanticOwnerSourceKindV1,
    outer_site: SourceStmtSiteV1,
    inner_site: SourceStmtSiteV1,
    forest: VerifiedResolvedLoopSourceForestV1,
    frame_key: LoopExecutionFrameKeyV1,
    roles: [ProvenanceRoleClaimV1; 2],
    _seal: ObservationSealV1,
}

#[derive(Debug, PartialEq, Eq)]
struct ObservationSealV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ObservationRejectV1 {
    ForestShape,
    ForestIdentity,
    FrameMismatch,
    ForeignBindingOwner,
    DuplicateRole,
    BindingRelation,
}

struct CandidateInputV1 {
    owner: FunctionOwnerIdV1,
    function_origin: FunctionOriginV1,
    source_kind: SemanticOwnerSourceKindV1,
    outer_site: SourceStmtSiteV1,
    inner_site: SourceStmtSiteV1,
    forest: VerifiedResolvedLoopSourceForestV1,
    frame_key: LoopExecutionFrameKeyV1,
    roles: [ProvenanceRoleClaimV1; 2],
}

fn strict_ancestor_binding(
    product: &VerifiedResolvedFunctionV1,
    binding: BindingRefV1,
    site: &SourceExprSiteV1,
) -> bool {
    let Some(ancestor_scope) = product.binding(binding).map(|record| record.owner_scope()) else {
        return false;
    };
    let Some(mut current) = product.exact_scope_containing(site.node()) else {
        return false;
    };
    while let Some(parent) = product.scope(current).and_then(|scope| scope.parent()) {
        if parent == ancestor_scope {
            return true;
        }
        current = parent;
    }
    false
}

fn candidate_input(source: &str, root: SourceStmtSiteV1) -> CandidateInputV1 {
    let function = parse_function(source);
    let mut resolver = FunctionSemanticResolverSessionV1::new(0).expect("resolver session");
    let product = resolver
        .resolve(FunctionSyntaxViewV1::from_ast(&function).expect("function view"))
        .expect("source resolves");
    let outer = outer_loop_site();
    let inner = inner_loop_site();
    let write = write_site(source == SHADOWING_SOURCE);
    let read = post_loop_read_site();
    let write_binding = resolved_binding(&product, &write);
    let read_binding = read_binding(&product, &read);
    let forest = product
        .resolved_loop_source_forest(&root)
        .expect("sealed loop forest");
    let frame_key = product
        .resolved_loop_source(&root)
        .expect("sealed loop source")
        .frame_key();
    CandidateInputV1 {
        owner: product.owner(),
        function_origin: product.function_origin(),
        source_kind: product.source_kind(),
        outer_site: outer,
        inner_site: inner,
        forest,
        frame_key,
        roles: [
            ProvenanceRoleClaimV1 {
                role: ProvenanceRoleV1::NestedWrite,
                site: write.clone(),
                binding: write_binding,
                strict_ancestor: strict_ancestor_binding(&product, write_binding, &write),
            },
            ProvenanceRoleClaimV1 {
                role: ProvenanceRoleV1::PostLoopRead,
                site: read,
                binding: read_binding,
                strict_ancestor: false,
            },
        ],
    }
}

fn issue_observation(
    input: CandidateInputV1,
) -> Result<ResolvedCarrierObservationV1, ObservationRejectV1> {
    if input.roles[0].role == input.roles[1].role {
        return Err(ObservationRejectV1::DuplicateRole);
    }
    if input.roles[0].binding.owner() != input.owner
        || input.roles[1].binding.owner() != input.owner
    {
        return Err(ObservationRejectV1::ForeignBindingOwner);
    }
    if input.roles[0].binding != input.roles[1].binding || !input.roles[0].strict_ancestor {
        return Err(ObservationRejectV1::BindingRelation);
    }
    let members = input.forest.members();
    if members.len() != 2
        || members[0].parent_index().is_some()
        || members[1].parent_index() != Some(0)
    {
        return Err(ObservationRejectV1::ForestShape);
    }
    if !members[0].source().matches_identity(
        input.function_origin,
        input.source_kind,
        &input.outer_site,
    ) || !members[1].source().matches_identity(
        input.function_origin,
        input.source_kind,
        &input.inner_site,
    ) {
        return Err(ObservationRejectV1::ForestIdentity);
    }
    if !members[0].source().frame_key().matches(&input.frame_key) {
        return Err(ObservationRejectV1::FrameMismatch);
    }
    Ok(ResolvedCarrierObservationV1 {
        owner: input.owner,
        function_origin: input.function_origin,
        source_kind: input.source_kind,
        outer_site: input.outer_site,
        inner_site: input.inner_site,
        forest: input.forest,
        frame_key: input.frame_key,
        roles: input.roles,
        _seal: ObservationSealV1,
    })
}

#[test]
fn generic_d3_s2_s0_natural_source_seals_typed_observation() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    let observation = issue_observation(candidate_input(SOURCE, outer_loop_site()))
        .expect("natural Both source must produce typed observation");
    assert_eq!(observation.forest.members().len(), 2);
    assert_eq!(observation.forest.members()[1].parent_index(), Some(0));
    assert_eq!(observation.roles[0].role, ProvenanceRoleV1::NestedWrite);
    assert_eq!(observation.roles[1].role, ProvenanceRoleV1::PostLoopRead);
    assert_eq!(observation.roles[0].binding, observation.roles[1].binding);
    assert!(observation.roles[0].strict_ancestor);
    assert!(observation.forest.members()[0]
        .source()
        .frame_key()
        .matches(&observation.frame_key));
}

#[test]
fn generic_d3_s2_s0_shadowing_rejects_binding_relation() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    assert_eq!(
        issue_observation(candidate_input(SHADOWING_SOURCE, outer_loop_site())),
        Err(ObservationRejectV1::BindingRelation)
    );
}

#[test]
fn generic_d3_s2_s0_foreign_owner_rejects_before_observation() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    let mut input = candidate_input(SOURCE, outer_loop_site());
    let foreign = candidate_input(SOURCE, outer_loop_site());
    input.roles[0].binding = foreign.roles[0].binding;
    assert_eq!(
        issue_observation(input),
        Err(ObservationRejectV1::ForeignBindingOwner)
    );
}

#[test]
fn generic_d3_s2_s0_forest_and_frame_mismatches_reject() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    let forest_mismatch = candidate_input(SOURCE, inner_loop_site());
    assert_eq!(
        issue_observation(forest_mismatch),
        Err(ObservationRejectV1::ForestShape)
    );

    let mut frame_mismatch = candidate_input(SOURCE, outer_loop_site());
    frame_mismatch.frame_key = candidate_input(SOURCE, inner_loop_site()).frame_key;
    assert_eq!(
        issue_observation(frame_mismatch),
        Err(ObservationRejectV1::FrameMismatch)
    );
}
