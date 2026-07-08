use crate::mir::core_method_op::{CoreMethodLoweringTier, CoreMethodOp, CoreMethodOpCarrier};
use crate::mir::generic_method_route_facts::{
    GenericMethodPublicationPolicy, GenericMethodReturnShape, GenericMethodValueDemand,
};

use super::scalar_known_typed_direct_closeout_contract::{
    accepted_scalar_known_contracts, candidate_scalar_known_surfaces, ScalarKnownEffectClass,
    ScalarKnownSurfaceId,
};
use super::{GenericMethodRouteDecision, GenericMethodRouteKind, GenericMethodRouteProof};

const MAPSTORE_I64_HAKO: &str =
    include_str!("../../../lang/src/compiler/lib/write_set_mapstore_i64_policy_classifier.hako");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct HakoMapStoreI64Policy<'a> {
    surface: &'a str,
    route_kind: &'a str,
    core_op: &'a str,
    lowering_tier: &'a str,
    result_class: &'a str,
    return_shape: &'a str,
    value_demand: &'a str,
    value_boundary: &'a str,
    publication_policy: &'a str,
    effect_class: &'a str,
    mutation_class: &'a str,
    role: &'a str,
}

pub(super) fn mapstore_i64_shadow_consumed_decision() -> GenericMethodRouteDecision {
    let policy = hako_mapstore_i64_policy();
    let accepted_contract_count = accepted_scalar_known_contracts().count();
    assert!(
        accepted_contract_count >= 2,
        "ScalarKnown accepted contract boundary lost prior closeouts"
    );
    let contract_contains_route = candidate_scalar_known_surfaces().any(|contract| {
        contract.contract_id.as_str() == "WriteResultScalarI64ClassificationOnly"
            && contract.surface_id.as_str() == "WriteScalarI64Routes"
            && contract.surface_id == ScalarKnownSurfaceId::WriteScalarI64Routes
            && contract.effect_class.as_str() == ScalarKnownEffectClass::Mutate.as_str()
            && contract
                .route_kind_set
                .contains(&GenericMethodRouteKind::MapStoreI64)
    });
    assert!(
        contract_contains_route,
        "ScalarKnown Write contract no longer contains MapStoreI64"
    );
    assert_hako_policy_matches_rust(&policy);

    GenericMethodRouteDecision::new(
        GenericMethodRouteKind::MapStoreI64,
        GenericMethodRouteProof::SetSurfacePolicy,
        Some(CoreMethodOpCarrier::manifest(
            CoreMethodOp::MapSet,
            CoreMethodLoweringTier::ColdFallback,
        )),
        None,
        GenericMethodValueDemand::WriteAny,
        None,
    )
}

fn hako_mapstore_i64_policy() -> HakoMapStoreI64Policy<'static> {
    let row = MAPSTORE_I64_HAKO
        .lines()
        .find_map(|line| {
            let start = line.find("\"map_store_i64_set_surface|")?;
            let rest = &line[start + 1..];
            let end = rest.find('"')?;
            Some(&rest[..end])
        })
        .expect("MapStoreI64 .hako policy row missing");
    parse_hako_mapstore_i64_policy_row(row)
}

fn parse_hako_mapstore_i64_policy_row(row: &str) -> HakoMapStoreI64Policy<'_> {
    let fields: Vec<_> = row.split('|').collect();
    assert_eq!(
        fields.len(),
        13,
        "MapStoreI64 .hako policy row field count changed"
    );
    HakoMapStoreI64Policy {
        surface: fields[1],
        route_kind: fields[2],
        core_op: fields[3],
        lowering_tier: fields[4],
        result_class: fields[5],
        return_shape: fields[6],
        value_demand: fields[7],
        value_boundary: fields[8],
        publication_policy: fields[9],
        effect_class: fields[10],
        mutation_class: fields[11],
        role: fields[12],
    }
}

fn assert_hako_policy_matches_rust(policy: &HakoMapStoreI64Policy<'_>) {
    assert_eq!(policy.surface, "SetSurfacePolicy");
    assert_eq!(policy.route_kind, "MapStoreI64");
    assert_eq!(policy.core_op, CoreMethodOp::MapSet.as_manifest_name());
    assert_eq!(policy.lowering_tier, "ColdFallback");
    assert_eq!(policy.result_class, "NoneResult");
    assert_eq!(policy.return_shape, "None");
    assert_eq!(
        GenericMethodReturnShape::ScalarI64.as_metadata_name(),
        "scalar_i64"
    );
    assert_eq!(
        policy.value_demand,
        "WriteAny",
        "MapStoreI64 .hako policy value demand drifted"
    );
    assert_eq!(GenericMethodValueDemand::WriteAny.as_metadata_name(), "write_any");
    assert_eq!(policy.value_boundary, "ScalarI64");
    assert_eq!(policy.publication_policy, "NonePublication");
    assert_eq!(
        GenericMethodPublicationPolicy::NoPublication.as_metadata_name(),
        "no_publication"
    );
    assert_eq!(policy.effect_class, "mutate");
    assert_eq!(policy.mutation_class, "MutatesReceiverOrContainer");
    assert_eq!(policy.role, "classifier_policy_mirror_only");
}

#[cfg(test)]
mod tests {
    use super::super::scalar_known_typed_direct_closeout_contract::ScalarKnownContractId;
    use super::*;

    const VALID_ROW: &str = "map_store_i64_set_surface|SetSurfacePolicy|MapStoreI64|MapSet|ColdFallback|NoneResult|None|WriteAny|ScalarI64|NonePublication|mutate|MutatesReceiverOrContainer|classifier_policy_mirror_only";

    #[test]
    fn mapstore_i64_shadow_artifact_matches_rust_fastpath_policy() {
        let _ = mapstore_i64_shadow_consumed_decision();
        assert_eq!(
            ScalarKnownContractId::WriteResultScalarI64.as_str(),
            "WriteResultScalarI64ClassificationOnly"
        );
    }

    #[test]
    #[should_panic(expected = "assertion `left == right` failed")]
    fn mapstore_i64_shadow_rejects_route_kind_mismatch() {
        let row = VALID_ROW.replace("|MapStoreI64|", "|MapStoreAny|");
        assert_hako_policy_matches_rust(&parse_hako_mapstore_i64_policy_row(&row));
    }

    #[test]
    #[should_panic(expected = "assertion `left == right` failed")]
    fn mapstore_i64_shadow_rejects_core_op_mismatch() {
        let row = VALID_ROW.replace("|MapSet|", "|MapDelete|");
        assert_hako_policy_matches_rust(&parse_hako_mapstore_i64_policy_row(&row));
    }

    #[test]
    #[should_panic(expected = "assertion `left == right` failed")]
    fn mapstore_i64_shadow_rejects_role_mismatch() {
        let row = VALID_ROW.replace(
            "|classifier_policy_mirror_only",
            "|hako_runtime_route_authority",
        );
        assert_hako_policy_matches_rust(&parse_hako_mapstore_i64_policy_row(&row));
    }
}
