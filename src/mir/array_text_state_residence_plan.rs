/*!
 * MIR-owned route plan for array/text state residence.
 *
 * This route is the generic residence-side owner for current indexOf keeper
 * fronts. MIR JSON and backend consumers read this field as the only active
 * residence contract; the exact-shape payload remains an explicit temporary
 * child field until the emitter no longer needs it.
 */

use super::{
    indexof_search_micro_seed_plan::{
        match_indexof_search_micro_seed_route, IndexOfSearchBackendAction,
        IndexOfSearchCandidateOutcome, IndexOfSearchMicroSeedProof, IndexOfSearchMicroSeedRoute,
        IndexOfSearchMicroSeedVariant, IndexOfSearchResultUse,
    },
    MirFunction, MirModule,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ArrayTextStateResidenceKind {
    IndexOf,
}

impl std::fmt::Display for ArrayTextStateResidenceKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl ArrayTextStateResidenceKind {
    fn as_str(self) -> &'static str {
        match self {
            Self::IndexOf => "indexof",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ArrayTextStateResidence {
    LoopLocalPointerArray,
}

impl std::fmt::Display for ArrayTextStateResidence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl ArrayTextStateResidence {
    fn as_str(self) -> &'static str {
        match self {
            Self::LoopLocalPointerArray => "loop_local_pointer_array",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ArrayTextStateResidenceResultRepr {
    ScalarI64,
}

impl std::fmt::Display for ArrayTextStateResidenceResultRepr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl ArrayTextStateResidenceResultRepr {
    fn as_str(self) -> &'static str {
        match self {
            Self::ScalarI64 => "scalar_i64",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ArrayTextStateResidenceConsumerCapability {
    DirectArrayTextStateResidence,
}

impl std::fmt::Display for ArrayTextStateResidenceConsumerCapability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl ArrayTextStateResidenceConsumerCapability {
    fn as_str(&self) -> &'static str {
        match self {
            Self::DirectArrayTextStateResidence => "direct_array_text_state_residence",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ArrayTextStateResidencePublicationBoundary {
    None,
}

impl std::fmt::Display for ArrayTextStateResidencePublicationBoundary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl ArrayTextStateResidencePublicationBoundary {
    fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArrayTextStateResidenceContract {
    observer_kind: ArrayTextStateResidenceKind,
    residence: ArrayTextStateResidence,
    result_repr: ArrayTextStateResidenceResultRepr,
    consumer_capability: ArrayTextStateResidenceConsumerCapability,
    publication_boundary: ArrayTextStateResidencePublicationBoundary,
}

impl ArrayTextStateResidenceContract {
    pub fn observer_kind(&self) -> &'static str {
        self.observer_kind.as_str()
    }

    pub fn residence(&self) -> &'static str {
        self.residence.as_str()
    }

    pub fn result_repr(&self) -> &'static str {
        self.result_repr.as_str()
    }

    pub fn consumer_capability(&self) -> &'static str {
        self.consumer_capability.as_str()
    }

    pub fn publication_boundary(&self) -> &'static str {
        self.publication_boundary.as_str()
    }

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
    contract: ArrayTextStateResidenceContract,
    temporary_indexof_seed_payload: Option<ArrayTextStateResidenceIndexOfSeedPayload>,
}

impl ArrayTextStateResidenceRoute {
    pub fn contract(&self) -> &ArrayTextStateResidenceContract {
        &self.contract
    }

    pub fn temporary_indexof_seed_payload(
        &self,
    ) -> Option<&ArrayTextStateResidenceIndexOfSeedPayload> {
        self.temporary_indexof_seed_payload.as_ref()
    }

    pub fn from_indexof_search_route(route: &IndexOfSearchMicroSeedRoute) -> Self {
        Self {
            contract: ArrayTextStateResidenceContract::indexof_loop_local_pointer_array(),
            temporary_indexof_seed_payload: Some(ArrayTextStateResidenceIndexOfSeedPayload {
                variant: route.variant(),
                rows: route.rows(),
                ops: route.ops(),
                flip_period: route.flip_period(),
                line_seed: route.line_seed().to_string(),
                line_seed_len: route.line_seed_len(),
                none_seed: route.none_seed().to_string(),
                none_seed_len: route.none_seed_len(),
                needle: route.needle().to_string(),
                needle_len: route.needle_len(),
                proof: route.proof(),
                result_use: route.result_use(),
                backend_action: route.backend_action(),
                line_seed_outcome: route.line_seed_outcome(),
                none_seed_outcome: route.none_seed_outcome(),
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
    function.metadata.array_text_state_residence_route =
        match_indexof_search_micro_seed_route(function)
            .as_ref()
            .map(ArrayTextStateResidenceRoute::from_indexof_search_route);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn derives_residence_route_from_indexof_search_route() {
        let exact = crate::mir::indexof_search_micro_seed_plan::test_support::line_route();

        let route = ArrayTextStateResidenceRoute::from_indexof_search_route(&exact);
        let payload = route
            .temporary_indexof_seed_payload()
            .expect("temporary payload");
        let contract = route.contract();

        assert_eq!(payload.variant, IndexOfSearchMicroSeedVariant::Line);
        assert_eq!(contract.residence(), "loop_local_pointer_array");
        assert_eq!(contract.observer_kind(), "indexof");
        assert_eq!(contract.result_repr(), "scalar_i64");
        assert_eq!(
            contract.consumer_capability(),
            "direct_array_text_state_residence"
        );
        assert_eq!(contract.publication_boundary(), "none");
        assert_eq!(
            payload.proof,
            IndexOfSearchMicroSeedProof::KiloMicroIndexOfLine15Block
        );
    }
}
