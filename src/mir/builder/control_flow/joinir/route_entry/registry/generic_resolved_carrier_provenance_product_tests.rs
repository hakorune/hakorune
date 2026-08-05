//! D3-S2-S2: passive resolver provenance product only.
//!
//! This test fixture proves that one co-sealed owner/forest/frame/role handoff
//! can become an opaque AST-free witness without opening Generic selection or
//! any Builder/MIR authority.

use super::generic_nested_carrier_bindingref_tests::{
    inner_loop_site, outer_loop_site, parse_function, post_loop_read_site, read_binding,
    resolved_binding, write_site, SHADOWING_SOURCE, SOURCE,
};
use crate::mir::resolved_semantics::generic_resolved_carrier_provenance::{
    issue_provenance_repeat_audit_v1, issue_resolved_carrier_provenance_v1,
    BrandedResolvedForestV1, BrandedResolvedFrameV1, ProvenanceRejectV1, ProvenanceRepeatInputV1,
    ResolvedCarrierHandoffV1, ResolvedCarrierRoleKindV1, ResolvedCarrierRoleV1,
};
use crate::mir::resolved_semantics::{FunctionSemanticResolverSessionV1, FunctionSyntaxViewV1};

fn handoff(source: &str) -> ResolvedCarrierHandoffV1 {
    handoff_in(source, 0)
}

fn handoff_in(source: &str, compilation_unit_ordinal: u32) -> ResolvedCarrierHandoffV1 {
    let function = parse_function(source);
    let mut resolver =
        FunctionSemanticResolverSessionV1::new(compilation_unit_ordinal).expect("resolver session");
    let product = resolver
        .resolve(FunctionSyntaxViewV1::from_ast(&function).expect("function view"))
        .expect("source resolves");
    let owner = product.owner();
    let outer = outer_loop_site();
    let inner = inner_loop_site();
    let write = write_site(source == SHADOWING_SOURCE);
    let read = post_loop_read_site();
    let write_binding = resolved_binding(&product, &write);
    let read_binding = read_binding(&product, &read);
    let forest = product
        .resolved_loop_source_forest(&outer)
        .expect("sealed loop forest");
    let frame = product
        .resolved_loop_source(&outer)
        .expect("sealed loop source")
        .frame_key();
    ResolvedCarrierHandoffV1::for_test(
        owner,
        product.function_origin(),
        product.source_kind(),
        outer,
        inner,
        BrandedResolvedForestV1::for_test(owner, forest),
        BrandedResolvedFrameV1::for_test(owner, frame),
        [
            ResolvedCarrierRoleV1::for_test(
                ResolvedCarrierRoleKindV1::NestedWrite,
                write,
                write_binding,
                source != SHADOWING_SOURCE,
            ),
            ResolvedCarrierRoleV1::for_test(
                ResolvedCarrierRoleKindV1::PostLoopRead,
                read,
                read_binding,
                false,
            ),
        ],
    )
}

#[test]
fn generic_d3_s2_s2_natural_source_seals_opaque_provenance() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    let product = issue_resolved_carrier_provenance_v1(handoff(SOURCE))
        .expect("natural Both source must seal provenance");
    assert_eq!(
        product.role_kinds(),
        [
            ResolvedCarrierRoleKindV1::NestedWrite,
            ResolvedCarrierRoleKindV1::PostLoopRead,
        ]
    );
    assert_eq!(product.outer_site(), &outer_loop_site());
    assert_eq!(product.inner_site(), &inner_loop_site());
}

#[test]
fn generic_d3_s2_s2_shadowing_rejects_strict_ancestor_relation() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    assert_eq!(
        issue_resolved_carrier_provenance_v1(handoff(SHADOWING_SOURCE)),
        Err(ProvenanceRejectV1::BindingRelation)
    );
}

#[test]
fn generic_d3_s2_s2_mixed_session_brand_rejects_before_effects() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    // Build the adversarial case through the same test-only resolver ingress,
    // retaining A's owner/source coordinates but B's branded forest/frame and
    // BindingRef roles.
    let function_a = parse_function(SOURCE);
    let mut resolver_a = FunctionSemanticResolverSessionV1::new(0).expect("resolver session");
    let product_a = resolver_a
        .resolve(FunctionSyntaxViewV1::from_ast(&function_a).expect("function view"))
        .expect("source resolves");
    let function_b = parse_function(SOURCE);
    let mut resolver_b = FunctionSemanticResolverSessionV1::new(0).expect("resolver session");
    let product_b = resolver_b
        .resolve(FunctionSyntaxViewV1::from_ast(&function_b).expect("function view"))
        .expect("source resolves");
    let owner_a = product_a.owner();
    let owner_b = product_b.owner();
    assert_ne!(owner_a, owner_b);
    let mixed = ResolvedCarrierHandoffV1::for_test(
        owner_a,
        product_a.function_origin(),
        product_a.source_kind(),
        outer_loop_site(),
        inner_loop_site(),
        BrandedResolvedForestV1::for_test(
            owner_b,
            product_b
                .resolved_loop_source_forest(&outer_loop_site())
                .expect("forest"),
        ),
        BrandedResolvedFrameV1::for_test(
            owner_b,
            product_b
                .resolved_loop_source(&outer_loop_site())
                .expect("source")
                .frame_key(),
        ),
        [
            ResolvedCarrierRoleV1::for_test(
                ResolvedCarrierRoleKindV1::NestedWrite,
                write_site(false),
                resolved_binding(&product_b, &write_site(false)),
                true,
            ),
            ResolvedCarrierRoleV1::for_test(
                ResolvedCarrierRoleKindV1::PostLoopRead,
                post_loop_read_site(),
                read_binding(&product_b, &post_loop_read_site()),
                false,
            ),
        ],
    );
    assert_eq!(
        issue_resolved_carrier_provenance_v1(mixed),
        Err(ProvenanceRejectV1::MixedOwnerBrand)
    );
}

#[test]
fn generic_d3_s2_s2_duplicate_role_rejects_before_effects() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    let function = parse_function(SOURCE);
    let mut resolver = FunctionSemanticResolverSessionV1::new(0).expect("resolver session");
    let product = resolver
        .resolve(FunctionSyntaxViewV1::from_ast(&function).expect("function view"))
        .expect("source resolves");
    let owner = product.owner();
    let outer = outer_loop_site();
    let forest = product.resolved_loop_source_forest(&outer).expect("forest");
    let frame = product
        .resolved_loop_source(&outer)
        .expect("source")
        .frame_key();
    let write = write_site(false);
    let binding = resolved_binding(&product, &write);
    let duplicate = ResolvedCarrierHandoffV1::for_test(
        owner,
        product.function_origin(),
        product.source_kind(),
        outer,
        inner_loop_site(),
        BrandedResolvedForestV1::for_test(owner, forest),
        BrandedResolvedFrameV1::for_test(owner, frame),
        [
            ResolvedCarrierRoleV1::for_test(
                ResolvedCarrierRoleKindV1::NestedWrite,
                write.clone(),
                binding,
                true,
            ),
            ResolvedCarrierRoleV1::for_test(
                ResolvedCarrierRoleKindV1::NestedWrite,
                write,
                binding,
                true,
            ),
        ],
    );
    assert_eq!(
        issue_resolved_carrier_provenance_v1(duplicate),
        Err(ProvenanceRejectV1::DuplicateRole)
    );
}

#[test]
fn generic_d3_s2_s2_unknown_role_rejects_before_effects() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    // Rebuild through the test-only ingress to avoid exposing product parts.
    let function = parse_function(SOURCE);
    let mut resolver = FunctionSemanticResolverSessionV1::new(0).expect("resolver session");
    let product = resolver
        .resolve(FunctionSyntaxViewV1::from_ast(&function).expect("function view"))
        .expect("source resolves");
    let owner = product.owner();
    let outer = outer_loop_site();
    let write = write_site(false);
    let binding = resolved_binding(&product, &write);
    let invalid = ResolvedCarrierHandoffV1::for_test(
        owner,
        product.function_origin(),
        product.source_kind(),
        outer,
        inner_loop_site(),
        BrandedResolvedForestV1::for_test(
            owner,
            product
                .resolved_loop_source_forest(&outer_loop_site())
                .expect("forest"),
        ),
        BrandedResolvedFrameV1::for_test(
            owner,
            product
                .resolved_loop_source(&outer_loop_site())
                .expect("source")
                .frame_key(),
        ),
        [
            ResolvedCarrierRoleV1::for_test(
                ResolvedCarrierRoleKindV1::Unknown,
                write.clone(),
                binding,
                true,
            ),
            ResolvedCarrierRoleV1::for_test(
                ResolvedCarrierRoleKindV1::PostLoopRead,
                post_loop_read_site(),
                read_binding(&product, &post_loop_read_site()),
                false,
            ),
        ],
    );
    assert_eq!(
        issue_resolved_carrier_provenance_v1(invalid),
        Err(ProvenanceRejectV1::UnsupportedRole)
    );
}

#[test]
fn generic_d3_s2_s3_fresh_products_repeat_without_identity_alias() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    let left = issue_resolved_carrier_provenance_v1(handoff(SOURCE)).expect("session A product");
    let right = issue_resolved_carrier_provenance_v1(handoff(SOURCE)).expect("session B product");
    let observation =
        issue_provenance_repeat_audit_v1(ProvenanceRepeatInputV1::for_test(left, right))
            .expect("fresh products have repeatable source structure");
    assert!(observation.source_topology_equal());
    assert!(observation.outer_inner_sites_equal());
    assert!(observation.roles_equal());
    assert!(observation.strict_ancestor_equal());
    assert!(observation.owner_brands_distinct());
    assert!(observation.raw_frame_coordinates_equal());
}

#[test]
fn generic_d3_s2_s3_function_origin_mismatch_rejects_before_effects() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    let left =
        issue_resolved_carrier_provenance_v1(handoff_in(SOURCE, 0)).expect("session A product");
    let right =
        issue_resolved_carrier_provenance_v1(handoff_in(SOURCE, 1)).expect("session B product");
    assert_eq!(
        issue_provenance_repeat_audit_v1(ProvenanceRepeatInputV1::for_test(left, right)),
        Err(crate::mir::resolved_semantics::generic_resolved_carrier_provenance::
            ProvenanceRepeatRejectV1::FunctionOriginMismatch)
    );
}
