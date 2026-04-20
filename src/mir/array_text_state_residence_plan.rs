/*!
 * MIR-owned route plan for array/text state residence.
 *
 * This route is the generic residence-side owner for current indexOf keeper
 * fronts. It may be derived from the exact bridge proof for now, but MIR JSON
 * and backend consumers read this field as a separate contract.
 */

use super::{
    indexof_search_micro_seed_plan::{
        IndexOfSearchBackendAction, IndexOfSearchCandidateOutcome, IndexOfSearchMicroSeedProof,
        IndexOfSearchMicroSeedRoute, IndexOfSearchMicroSeedVariant, IndexOfSearchResultUse,
    },
    MirFunction, MirModule,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArrayTextStateResidenceKind {
    IndexOf,
}

impl std::fmt::Display for ArrayTextStateResidenceKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IndexOf => f.write_str("indexof"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArrayTextStateResidence {
    LoopLocalPointerArray,
}

impl std::fmt::Display for ArrayTextStateResidence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LoopLocalPointerArray => f.write_str("loop_local_pointer_array"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArrayTextStateResidenceResultRepr {
    ScalarI64,
}

impl std::fmt::Display for ArrayTextStateResidenceResultRepr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ScalarI64 => f.write_str("scalar_i64"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArrayTextStateResidenceConsumerCapability {
    DirectArrayTextStateResidence,
}

impl std::fmt::Display for ArrayTextStateResidenceConsumerCapability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DirectArrayTextStateResidence => f.write_str("direct_array_text_state_residence"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArrayTextStateResidencePublicationBoundary {
    None,
}

impl std::fmt::Display for ArrayTextStateResidencePublicationBoundary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => f.write_str("none"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArrayTextStateResidenceContract {
    pub observer_kind: ArrayTextStateResidenceKind,
    pub residence: ArrayTextStateResidence,
    pub result_repr: ArrayTextStateResidenceResultRepr,
    pub consumer_capability: ArrayTextStateResidenceConsumerCapability,
    pub publication_boundary: ArrayTextStateResidencePublicationBoundary,
}

impl ArrayTextStateResidenceContract {
    fn indexof_loop_local_pointer_array() -> Self {
        Self {
            observer_kind: ArrayTextStateResidenceKind::IndexOf,
            residence: ArrayTextStateResidence::LoopLocalPointerArray,
            result_repr: ArrayTextStateResidenceResultRepr::ScalarI64,
            consumer_capability:
                ArrayTextStateResidenceConsumerCapability::DirectArrayTextStateResidence,
            publication_boundary: ArrayTextStateResidencePublicationBoundary::None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArrayTextStateResidenceIndexOfSeedPayload {
    pub variant: IndexOfSearchMicroSeedVariant,
    pub rows: i64,
    pub ops: i64,
    pub flip_period: Option<i64>,
    pub line_seed: String,
    pub line_seed_len: i64,
    pub none_seed: String,
    pub none_seed_len: i64,
    pub needle: String,
    pub needle_len: i64,
    pub proof: IndexOfSearchMicroSeedProof,
    pub result_use: IndexOfSearchResultUse,
    pub backend_action: IndexOfSearchBackendAction,
    pub line_seed_outcome: IndexOfSearchCandidateOutcome,
    pub none_seed_outcome: IndexOfSearchCandidateOutcome,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArrayTextStateResidenceRoute {
    pub contract: ArrayTextStateResidenceContract,
    pub temporary_indexof_seed_payload: Option<ArrayTextStateResidenceIndexOfSeedPayload>,
}

impl ArrayTextStateResidenceRoute {
    pub fn from_indexof_search_route(route: &IndexOfSearchMicroSeedRoute) -> Self {
        Self {
            contract: ArrayTextStateResidenceContract::indexof_loop_local_pointer_array(),
            temporary_indexof_seed_payload: Some(ArrayTextStateResidenceIndexOfSeedPayload {
                variant: route.variant,
                rows: route.rows,
                ops: route.ops,
                flip_period: route.flip_period,
                line_seed: route.line_seed.clone(),
                line_seed_len: route.line_seed_len,
                none_seed: route.none_seed.clone(),
                none_seed_len: route.none_seed_len,
                needle: route.needle.clone(),
                needle_len: route.needle_len,
                proof: route.proof,
                result_use: route.result_use,
                backend_action: route.backend_action,
                line_seed_outcome: route.line_seed_outcome,
                none_seed_outcome: route.none_seed_outcome,
            }),
        }
    }
}

pub fn refresh_module_array_text_state_residence_routes(module: &mut MirModule) {
    for function in module.functions.values_mut() {
        refresh_function_array_text_state_residence_route(function);
    }
}

pub fn refresh_function_array_text_state_residence_route(function: &mut MirFunction) {
    function.metadata.array_text_state_residence_route = function
        .metadata
        .indexof_search_micro_seed_route
        .as_ref()
        .map(ArrayTextStateResidenceRoute::from_indexof_search_route);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn derives_residence_route_from_indexof_search_route() {
        let exact = IndexOfSearchMicroSeedRoute {
            variant: IndexOfSearchMicroSeedVariant::Line,
            rows: 64,
            ops: 400000,
            flip_period: Some(16),
            line_seed: "line-seed".to_string(),
            line_seed_len: 9,
            none_seed: "none-seed".to_string(),
            none_seed_len: 9,
            needle: "line".to_string(),
            needle_len: 4,
            proof: IndexOfSearchMicroSeedProof::KiloMicroIndexOfLine15Block,
            result_use: IndexOfSearchResultUse::FoundPredicate,
            backend_action: IndexOfSearchBackendAction::LiteralMembershipPredicate,
            line_seed_outcome: IndexOfSearchCandidateOutcome::Found,
            none_seed_outcome: IndexOfSearchCandidateOutcome::NotFound,
        };

        let route = ArrayTextStateResidenceRoute::from_indexof_search_route(&exact);
        let payload = route
            .temporary_indexof_seed_payload
            .as_ref()
            .expect("temporary payload");

        assert_eq!(payload.variant, IndexOfSearchMicroSeedVariant::Line);
        assert_eq!(
            route.contract.residence,
            ArrayTextStateResidence::LoopLocalPointerArray
        );
        assert_eq!(
            route.contract.observer_kind,
            ArrayTextStateResidenceKind::IndexOf
        );
        assert_eq!(
            route.contract.result_repr,
            ArrayTextStateResidenceResultRepr::ScalarI64
        );
        assert_eq!(
            route.contract.consumer_capability,
            ArrayTextStateResidenceConsumerCapability::DirectArrayTextStateResidence
        );
        assert_eq!(
            route.contract.publication_boundary,
            ArrayTextStateResidencePublicationBoundary::None
        );
        assert_eq!(
            payload.proof,
            IndexOfSearchMicroSeedProof::KiloMicroIndexOfLine15Block
        );
    }
}
