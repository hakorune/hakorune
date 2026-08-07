//! Portable producer provenance, independent of legacy route selection.
//!
//! This ID is a diagnostic/product receipt only. It must not be used to
//! select a family, schedule a route, or dispatch a physicalizer.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum LoopRecipeProducerIdV1 {
    DirectAccumV1,
    LoopTrueBreakContinueV1,
    NestedPredicateV1,
    GenericG0,
    CallableSingleLoopV1,
}

impl LoopRecipeProducerIdV1 {
    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::DirectAccumV1 => "direct_accum_v1",
            Self::LoopTrueBreakContinueV1 => "loop_true_break_continue_v1",
            Self::NestedPredicateV1 => "nested_predicate_v1",
            Self::GenericG0 => "generic_g0",
            Self::CallableSingleLoopV1 => "callable_single_loop_v1",
        }
    }
}

impl std::fmt::Display for LoopRecipeProducerIdV1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
