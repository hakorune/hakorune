use crate::mir::core_method_op::{CoreMethodLoweringTier, CoreMethodOp};
use crate::mir::generic_method_route_facts::{
    GenericMethodPublicationPolicy, GenericMethodReturnShape, GenericMethodValueDemand,
};

use super::{GenericMethodRouteKind, GenericMethodRouteProof};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ScalarKnownContractId {
    MapLoadScalarI64,
    StringSearchScalarI64,
    CollectionLenScalarI64,
    WriteScalarI64Routes,
}

impl ScalarKnownContractId {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::MapLoadScalarI64 => "MapLoadScalarI64ScalarKnownTypedDirectCloseoutContract",
            Self::StringSearchScalarI64 => "StringSearchScalarI64TypedDirectCloseoutContract",
            Self::CollectionLenScalarI64 => "CollectionLenScalarI64TypedDirectCloseoutContract",
            Self::WriteScalarI64Routes => "WriteScalarI64RoutesScopedCloseout",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ScalarKnownSurfaceId {
    MapLoadScalarI64Routes,
    StringScalarI64Routes,
    CollectionScalarI64Routes,
    WriteScalarI64Routes,
}

impl ScalarKnownSurfaceId {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::MapLoadScalarI64Routes => "MapLoadScalarI64Routes",
            Self::StringScalarI64Routes => "StringScalarI64Routes",
            Self::CollectionScalarI64Routes => "CollectionScalarI64Routes",
            Self::WriteScalarI64Routes => "WriteScalarI64Routes",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ScalarKnownContractStatus {
    AcceptedScopedCloseout,
    CandidateNeedsPolicy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ScalarKnownEffectClass {
    Read,
    Observe,
    Mutate,
}

impl ScalarKnownEffectClass {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::Read => "read",
            Self::Observe => "observe",
            Self::Mutate => "mutate",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ScalarKnownTypedDirectCloseoutContract {
    pub(crate) contract_id: ScalarKnownContractId,
    pub(crate) surface_id: ScalarKnownSurfaceId,
    pub(crate) status: ScalarKnownContractStatus,
    pub(crate) route_kind_set: &'static [GenericMethodRouteKind],
    pub(crate) proof_or_policy_source: &'static [GenericMethodRouteProof],
    pub(crate) core_method_ops: &'static [CoreMethodOp],
    pub(crate) return_shape: Option<GenericMethodReturnShape>,
    pub(crate) value_demand: GenericMethodValueDemand,
    pub(crate) publication_policy: Option<GenericMethodPublicationPolicy>,
    pub(crate) lowering_tier: Option<CoreMethodLoweringTier>,
    pub(crate) effect_class: ScalarKnownEffectClass,
}

const MAP_LOAD_SCALAR_I64_ROUTES: &[GenericMethodRouteKind] =
    &[GenericMethodRouteKind::MapLoadScalarI64];
const MAP_LOAD_SCALAR_I64_PROOFS: &[GenericMethodRouteProof] = &[
    GenericMethodRouteProof::MapSetScalarI64SameKeyNoEscape,
    GenericMethodRouteProof::MapSetScalarI64DominatesNoEscape,
    GenericMethodRouteProof::MapSetScalarI64CoveredDynamicI64KeyNoEscape,
];
const MAP_LOAD_SCALAR_I64_OPS: &[CoreMethodOp] = &[CoreMethodOp::MapGet];

const STRING_SEARCH_SCALAR_I64_ROUTES: &[GenericMethodRouteKind] = &[
    GenericMethodRouteKind::StringIndexOf,
    GenericMethodRouteKind::StringLastIndexOf,
    GenericMethodRouteKind::StringContains,
];
const STRING_SEARCH_SCALAR_I64_PROOFS: &[GenericMethodRouteProof] = &[
    GenericMethodRouteProof::IndexOfSurfacePolicy,
    GenericMethodRouteProof::LastIndexOfSurfacePolicy,
    GenericMethodRouteProof::ContainsSurfacePolicy,
];
const STRING_SEARCH_SCALAR_I64_OPS: &[CoreMethodOp] = &[
    CoreMethodOp::StringIndexOf,
    CoreMethodOp::StringLastIndexOf,
    CoreMethodOp::StringContains,
];

const COLLECTION_LEN_SCALAR_I64_ROUTES: &[GenericMethodRouteKind] = &[
    GenericMethodRouteKind::MapEntryCount,
    GenericMethodRouteKind::ArraySlotLen,
    GenericMethodRouteKind::StringLen,
    GenericMethodRouteKind::AnyLength,
];
const COLLECTION_LEN_SCALAR_I64_PROOFS: &[GenericMethodRouteProof] =
    &[GenericMethodRouteProof::LenSurfacePolicy];
const COLLECTION_LEN_SCALAR_I64_OPS: &[CoreMethodOp] = &[
    CoreMethodOp::MapLen,
    CoreMethodOp::ArrayLen,
    CoreMethodOp::StringLen,
    CoreMethodOp::AnyLen,
];

const WRITE_RESULT_SCALAR_I64_ROUTES: &[GenericMethodRouteKind] = &[
    GenericMethodRouteKind::ArrayAppendAny,
    GenericMethodRouteKind::MapDeleteAny,
    GenericMethodRouteKind::MapStoreI64,
    GenericMethodRouteKind::MapStoreAny,
];
const WRITE_RESULT_SCALAR_I64_PROOFS: &[GenericMethodRouteProof] = &[
    GenericMethodRouteProof::PushSurfacePolicy,
    GenericMethodRouteProof::DeleteSurfacePolicy,
    GenericMethodRouteProof::SetSurfacePolicy,
];
const WRITE_RESULT_SCALAR_I64_OPS: &[CoreMethodOp] = &[
    CoreMethodOp::ArrayPush,
    CoreMethodOp::MapDelete,
    CoreMethodOp::MapSet,
];

pub(crate) const SCALAR_KNOWN_TYPED_DIRECT_CLOSEOUT_CONTRACTS:
    &[ScalarKnownTypedDirectCloseoutContract] = &[
    ScalarKnownTypedDirectCloseoutContract {
        contract_id: ScalarKnownContractId::MapLoadScalarI64,
        surface_id: ScalarKnownSurfaceId::MapLoadScalarI64Routes,
        status: ScalarKnownContractStatus::AcceptedScopedCloseout,
        route_kind_set: MAP_LOAD_SCALAR_I64_ROUTES,
        proof_or_policy_source: MAP_LOAD_SCALAR_I64_PROOFS,
        core_method_ops: MAP_LOAD_SCALAR_I64_OPS,
        return_shape: Some(GenericMethodReturnShape::ScalarI64OrMissingZero),
        value_demand: GenericMethodValueDemand::ScalarI64,
        publication_policy: Some(GenericMethodPublicationPolicy::NoPublication),
        lowering_tier: Some(CoreMethodLoweringTier::WarmDirectAbi),
        effect_class: ScalarKnownEffectClass::Read,
    },
    ScalarKnownTypedDirectCloseoutContract {
        contract_id: ScalarKnownContractId::StringSearchScalarI64,
        surface_id: ScalarKnownSurfaceId::StringScalarI64Routes,
        status: ScalarKnownContractStatus::AcceptedScopedCloseout,
        route_kind_set: STRING_SEARCH_SCALAR_I64_ROUTES,
        proof_or_policy_source: STRING_SEARCH_SCALAR_I64_PROOFS,
        core_method_ops: STRING_SEARCH_SCALAR_I64_OPS,
        return_shape: Some(GenericMethodReturnShape::ScalarI64),
        value_demand: GenericMethodValueDemand::ScalarI64,
        publication_policy: Some(GenericMethodPublicationPolicy::NoPublication),
        lowering_tier: Some(CoreMethodLoweringTier::WarmDirectAbi),
        effect_class: ScalarKnownEffectClass::Read,
    },
    ScalarKnownTypedDirectCloseoutContract {
        contract_id: ScalarKnownContractId::CollectionLenScalarI64,
        surface_id: ScalarKnownSurfaceId::CollectionScalarI64Routes,
        status: ScalarKnownContractStatus::AcceptedScopedCloseout,
        route_kind_set: COLLECTION_LEN_SCALAR_I64_ROUTES,
        proof_or_policy_source: COLLECTION_LEN_SCALAR_I64_PROOFS,
        core_method_ops: COLLECTION_LEN_SCALAR_I64_OPS,
        return_shape: Some(GenericMethodReturnShape::ScalarI64),
        value_demand: GenericMethodValueDemand::ScalarI64,
        publication_policy: Some(GenericMethodPublicationPolicy::NoPublication),
        lowering_tier: Some(CoreMethodLoweringTier::WarmDirectAbi),
        effect_class: ScalarKnownEffectClass::Observe,
    },
    ScalarKnownTypedDirectCloseoutContract {
        contract_id: ScalarKnownContractId::WriteScalarI64Routes,
        surface_id: ScalarKnownSurfaceId::WriteScalarI64Routes,
        status: ScalarKnownContractStatus::AcceptedScopedCloseout,
        route_kind_set: WRITE_RESULT_SCALAR_I64_ROUTES,
        proof_or_policy_source: WRITE_RESULT_SCALAR_I64_PROOFS,
        core_method_ops: WRITE_RESULT_SCALAR_I64_OPS,
        return_shape: None,
        value_demand: GenericMethodValueDemand::WriteAny,
        publication_policy: None,
        lowering_tier: None,
        effect_class: ScalarKnownEffectClass::Mutate,
    },
];

pub(crate) fn accepted_scalar_known_contracts(
) -> impl Iterator<Item = &'static ScalarKnownTypedDirectCloseoutContract> {
    SCALAR_KNOWN_TYPED_DIRECT_CLOSEOUT_CONTRACTS
        .iter()
        .filter(|contract| contract.status == ScalarKnownContractStatus::AcceptedScopedCloseout)
}

pub(crate) fn candidate_scalar_known_surfaces(
) -> impl Iterator<Item = &'static ScalarKnownTypedDirectCloseoutContract> {
    SCALAR_KNOWN_TYPED_DIRECT_CLOSEOUT_CONTRACTS
        .iter()
        .filter(|contract| contract.status == ScalarKnownContractStatus::CandidateNeedsPolicy)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scalar_known_contract_boundary_keeps_all_surfaces_accepted_after_write_closeout() {
        let accepted: Vec<_> = accepted_scalar_known_contracts().collect();
        let candidates: Vec<_> = candidate_scalar_known_surfaces().collect();

        assert_eq!(accepted.len(), 4);
        assert!(candidates.is_empty());
        assert_eq!(
            accepted[0].contract_id.as_str(),
            "MapLoadScalarI64ScalarKnownTypedDirectCloseoutContract"
        );
        assert_eq!(
            accepted[1].contract_id.as_str(),
            "StringSearchScalarI64TypedDirectCloseoutContract"
        );
        assert_eq!(
            accepted[2].surface_id.as_str(),
            "CollectionScalarI64Routes"
        );
        assert_eq!(accepted[3].surface_id.as_str(), "WriteScalarI64Routes");
    }

    #[test]
    fn scalar_known_contract_boundary_preserves_semantic_fields() {
        let string_search = accepted_scalar_known_contracts()
            .find(|contract| contract.contract_id == ScalarKnownContractId::StringSearchScalarI64)
            .expect("string search contract");
        assert_eq!(string_search.route_kind_set.len(), 3);
        assert_eq!(
            string_search.return_shape,
            Some(GenericMethodReturnShape::ScalarI64)
        );
        assert_eq!(
            string_search.publication_policy,
            Some(GenericMethodPublicationPolicy::NoPublication)
        );
        assert_eq!(string_search.effect_class.as_str(), "read");

        let write = accepted_scalar_known_contracts()
            .find(|contract| contract.surface_id == ScalarKnownSurfaceId::WriteScalarI64Routes)
            .expect("write candidate");
        assert_eq!(write.value_demand, GenericMethodValueDemand::WriteAny);
        assert_eq!(write.return_shape, None);
        assert_eq!(write.publication_policy, None);
        assert_eq!(write.effect_class.as_str(), "mutate");
    }
}
