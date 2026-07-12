use super::super::scalar_known_typed_direct_closeout_contract::ScalarKnownContractId;
use super::super::generated::write_set_mapstore_route_policy::MAPSTORE_ROUTE_POLICY_ROWS;
use super::*;

#[test]
fn mapstore_i64_shadow_artifact_matches_rust_fastpath_policy() {
    let _ = mapstore_i64_shadow_consumed_decision();
    assert_eq!(
        ScalarKnownContractId::WriteScalarI64Routes.as_str(),
        "WriteScalarI64RoutesScopedCloseout"
    );
}

#[test]
fn mapload_scalar_i64_shadow_artifact_matches_rust_fastpath_policy() {
    let decision = mapload_scalar_i64_hako_route_authority_pilot_decision(
        GenericMethodRouteProof::MapSetScalarI64SameKeyNoEscape,
    );
    assert_eq!(
        decision,
        GenericMethodRouteDecision::new(
            GenericMethodRouteKind::MapLoadScalarI64,
            GenericMethodRouteProof::MapSetScalarI64SameKeyNoEscape,
            Some(CoreMethodOpCarrier::manifest(
                CoreMethodOp::MapGet,
                CoreMethodLoweringTier::WarmDirectAbi,
            )),
            Some(GenericMethodReturnShape::ScalarI64OrMissingZero),
            GenericMethodValueDemand::ScalarI64,
            Some(GenericMethodPublicationPolicy::NoPublication),
        )
    );
    assert_eq!(
        ScalarKnownContractId::MapLoadScalarI64.as_str(),
        "MapLoadScalarI64ScalarKnownTypedDirectCloseoutContract"
    );
}

#[test]
fn string_scalar_i64_shadow_artifact_matches_rust_fastpath_policy() {
    for (route_kind, route_proof, core_op) in [
        (
            GenericMethodRouteKind::StringIndexOf,
            GenericMethodRouteProof::IndexOfSurfacePolicy,
            CoreMethodOp::StringIndexOf,
        ),
        (
            GenericMethodRouteKind::StringLastIndexOf,
            GenericMethodRouteProof::LastIndexOfSurfacePolicy,
            CoreMethodOp::StringLastIndexOf,
        ),
        (
            GenericMethodRouteKind::StringContains,
            GenericMethodRouteProof::ContainsSurfacePolicy,
            CoreMethodOp::StringContains,
        ),
    ] {
        let decision =
            string_scalar_i64_shadow_consumed_decision(route_kind, route_proof, core_op);
        assert_eq!(
            decision,
            GenericMethodRouteDecision::new(
                route_kind,
                route_proof,
                Some(CoreMethodOpCarrier::manifest(
                    core_op,
                    CoreMethodLoweringTier::WarmDirectAbi,
                )),
                Some(GenericMethodReturnShape::ScalarI64),
                GenericMethodValueDemand::ScalarI64,
                Some(GenericMethodPublicationPolicy::NoPublication),
            )
        );
    }
    assert_eq!(
        ScalarKnownContractId::StringSearchScalarI64.as_str(),
        "StringSearchScalarI64TypedDirectCloseoutContract"
    );
}

#[test]
fn collection_scalar_i64_shadow_artifact_matches_rust_fastpath_policy() {
    for (route_kind, core_op) in [
        (GenericMethodRouteKind::MapEntryCount, CoreMethodOp::MapLen),
        (GenericMethodRouteKind::ArraySlotLen, CoreMethodOp::ArrayLen),
        (GenericMethodRouteKind::StringLen, CoreMethodOp::StringLen),
        (GenericMethodRouteKind::AnyLength, CoreMethodOp::AnyLen),
    ] {
        let decision = collection_scalar_i64_shadow_consumed_decision(route_kind, core_op);
        assert_eq!(
            decision,
            GenericMethodRouteDecision::new(
                route_kind,
                GenericMethodRouteProof::LenSurfacePolicy,
                Some(CoreMethodOpCarrier::manifest(
                    core_op,
                    CoreMethodLoweringTier::WarmDirectAbi,
                )),
                Some(GenericMethodReturnShape::ScalarI64),
                GenericMethodValueDemand::ScalarI64,
                Some(GenericMethodPublicationPolicy::NoPublication),
            )
        );
    }
    assert_eq!(
        ScalarKnownContractId::CollectionLenScalarI64.as_str(),
        "CollectionLenScalarI64TypedDirectCloseoutContract"
    );
}

#[test]
fn write_push_shadow_artifact_matches_rust_fastpath_policy() {
    let _ = write_push_shadow_consumed_decision();
    assert_eq!(
        ScalarKnownContractId::WriteScalarI64Routes.as_str(),
        "WriteScalarI64RoutesScopedCloseout"
    );
}

#[test]
fn mapstore_any_shadow_artifact_matches_rust_fastpath_policy() {
    let _ = mapstore_any_shadow_consumed_decision();
    assert_eq!(
        ScalarKnownContractId::WriteScalarI64Routes.as_str(),
        "WriteScalarI64RoutesScopedCloseout"
    );
}

#[test]
fn mapstore_policy_rows_keep_key_and_stored_value_domains_independent() {
    let i64_row = MAPSTORE_ROUTE_POLICY_ROWS
        .iter()
        .find(|row| row.route_kind == GenericMethodRouteKind::MapStoreI64)
        .expect("MapStoreI64 RoutePolicyRow missing");
    let any_row = MAPSTORE_ROUTE_POLICY_ROWS
        .iter()
        .find(|row| row.route_kind == GenericMethodRouteKind::MapStoreAny)
        .expect("MapStoreAny RoutePolicyRow missing");

    assert_eq!((i64_row.key_domain, i64_row.stored_value_domain), ("I64", "Any"));
    assert_eq!((any_row.key_domain, any_row.stored_value_domain), ("Any", "Any"));
}

#[test]
fn mapstore_policy_rows_do_not_infer_route_from_stored_value_domain() {
    let rows_with_any_value = MAPSTORE_ROUTE_POLICY_ROWS
        .iter()
        .filter(|row| row.stored_value_domain == "Any")
        .map(|row| row.route_kind)
        .collect::<Vec<_>>();

    assert_eq!(rows_with_any_value.len(), 2);
    assert!(rows_with_any_value.contains(&GenericMethodRouteKind::MapStoreI64));
    assert!(rows_with_any_value.contains(&GenericMethodRouteKind::MapStoreAny));
}

#[test]
#[should_panic(expected = "assertion `left == right` failed")]
fn mapstore_i64_shadow_rejects_route_kind_mismatch() {
    let mut policy = WRITE_SET_MAPSTORE_I64_HAKO_POLICY;
    policy.route_kind = GenericMethodRouteKind::MapStoreAny;
    assert_hako_policy_matches_rust(&policy);
}

#[test]
#[should_panic(expected = "assertion `left == right` failed")]
fn mapstore_i64_shadow_rejects_core_op_mismatch() {
    let mut policy = WRITE_SET_MAPSTORE_I64_HAKO_POLICY;
    policy.core_op = CoreMethodOp::MapDelete;
    assert_hako_policy_matches_rust(&policy);
}

#[test]
#[should_panic(expected = "MapLoad .hako policy does not allow Rust scalar-map proof")]
fn mapload_scalar_i64_shadow_rejects_unlisted_proof() {
    assert_hako_mapload_scalar_i64_policy_matches_rust(
        &MAPLOAD_SCALAR_I64_HAKO_POLICY,
        GenericMethodRouteProof::GetSurfacePolicy,
    );
}

#[test]
#[should_panic(expected = "assertion `left == right` failed")]
fn mapload_scalar_i64_shadow_rejects_role_mismatch() {
    let mut policy = MAPLOAD_SCALAR_I64_HAKO_POLICY;
    policy.role = "hako_runtime_route_authority";
    assert_hako_mapload_scalar_i64_policy_matches_rust(
        &policy,
        GenericMethodRouteProof::MapSetScalarI64SameKeyNoEscape,
    );
}

#[test]
#[should_panic(expected = "assertion `left == right` failed")]
fn string_scalar_i64_shadow_rejects_route_kind_mismatch() {
    let mut policy = STRING_SEARCH_SCALAR_I64_HAKO_POLICIES[0];
    policy.route_kind = GenericMethodRouteKind::StringContains;
    assert_hako_string_scalar_i64_policy_matches_rust(
        &policy,
        GenericMethodRouteKind::StringIndexOf,
        GenericMethodRouteProof::IndexOfSurfacePolicy,
        CoreMethodOp::StringIndexOf,
    );
}

#[test]
#[should_panic(expected = "assertion `left == right` failed")]
fn string_scalar_i64_shadow_rejects_role_mismatch() {
    let mut policy = STRING_SEARCH_SCALAR_I64_HAKO_POLICIES[0];
    policy.role = "hako_runtime_route_authority";
    assert_hako_string_scalar_i64_policy_matches_rust(
        &policy,
        GenericMethodRouteKind::StringIndexOf,
        GenericMethodRouteProof::IndexOfSurfacePolicy,
        CoreMethodOp::StringIndexOf,
    );
}

#[test]
#[should_panic(expected = "assertion `left == right` failed")]
fn collection_scalar_i64_shadow_rejects_core_op_mismatch() {
    let mut policy = COLLECTION_LEN_SCALAR_I64_HAKO_POLICIES[0];
    policy.core_op = CoreMethodOp::AnyLen;
    assert_hako_collection_scalar_i64_policy_matches_rust(
        &policy,
        GenericMethodRouteKind::MapEntryCount,
        CoreMethodOp::MapLen,
    );
}

#[test]
#[should_panic(expected = "assertion `left == right` failed")]
fn collection_scalar_i64_shadow_rejects_role_mismatch() {
    let mut policy = COLLECTION_LEN_SCALAR_I64_HAKO_POLICIES[0];
    policy.role = "hako_runtime_route_authority";
    assert_hako_collection_scalar_i64_policy_matches_rust(
        &policy,
        GenericMethodRouteKind::MapEntryCount,
        CoreMethodOp::MapLen,
    );
}

#[test]
#[should_panic(expected = "assertion `left == right` failed")]
fn mapstore_i64_shadow_rejects_role_mismatch() {
    let mut policy = WRITE_SET_MAPSTORE_I64_HAKO_POLICY;
    policy.role = "hako_runtime_route_authority";
    assert_hako_policy_matches_rust(&policy);
}

#[test]
#[should_panic(expected = "assertion `left == right` failed")]
fn write_push_shadow_rejects_publication_mismatch() {
    let mut policy = WRITE_PUSH_HAKO_POLICY;
    policy.publication_policy = "RuntimePublication";
    assert_hako_write_push_policy_matches_rust(&policy);
}

#[test]
#[should_panic(expected = "assertion `left == right` failed")]
fn write_push_shadow_rejects_role_mismatch() {
    let mut policy = WRITE_PUSH_HAKO_POLICY;
    policy.role = "hako_runtime_route_authority";
    assert_hako_write_push_policy_matches_rust(&policy);
}

#[test]
#[should_panic(expected = "assertion `left == right` failed")]
fn mapstore_any_shadow_rejects_value_boundary_mismatch() {
    let mut policy = WRITE_SET_MAPSTORE_ANY_HAKO_POLICY;
    policy.value_boundary = "ScalarI64";
    assert_hako_mapstore_any_policy_matches_rust(&policy);
}

#[test]
#[should_panic(expected = "assertion `left == right` failed")]
fn mapstore_any_shadow_rejects_any_boundary_policy_mismatch() {
    let mut policy = WRITE_SET_MAPSTORE_ANY_HAKO_POLICY;
    policy.any_boundary_policy = "RuntimeAuthority";
    assert_hako_mapstore_any_policy_matches_rust(&policy);
}
