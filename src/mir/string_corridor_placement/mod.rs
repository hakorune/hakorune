/*!
 * String corridor placement/effect scaffold.
 *
 * This module consumes canonical string corridor facts and emits no-op candidate
 * decisions for future placement/effect rewrites. It does not mutate MIR or
 * change runtime behavior in this wave.
 */

use super::value_origin::build_value_def_map;
use super::{MirFunction, MirModule};

mod candidate_infer;
mod plan_infer;
mod relation_carry;
#[cfg(test)]
mod tests;
mod types;

pub use types::{
    StringCorridorCandidate, StringCorridorCandidateKind, StringCorridorCandidatePlan,
    StringCorridorCandidateProof, StringCorridorCandidateState, StringCorridorPublicationBoundary,
    StringCorridorPublicationContract,
};

use candidate_infer::infer_candidates;
use relation_carry::refresh_function_string_corridor_relation_candidates;

/// Refresh placement/effect candidates across the module without changing behavior.
pub fn refresh_module_string_corridor_candidates(module: &mut MirModule) {
    for function in module.functions.values_mut() {
        refresh_function_string_corridor_candidates(function);
    }
}

/// Refresh a single function's placement/effect candidates from existing facts.
pub fn refresh_function_string_corridor_candidates(function: &mut MirFunction) {
    function.metadata.string_corridor_candidates.clear();
    let def_map = build_value_def_map(function);

    for (value, fact) in &function.metadata.string_corridor_facts {
        let candidates = infer_candidates(function, *value, fact, &def_map);
        if !candidates.is_empty() {
            function
                .metadata
                .string_corridor_candidates
                .insert(*value, candidates);
        }
    }

    refresh_function_string_corridor_relation_candidates(function, &def_map);
}
