//! One use walk for SSA, ordinary/edge dominance and merge diagnostics.
//! Buckets retain each verifier's historical order and span-selection policy.
use crate::mir::value_origin::ValueDefMap;
use crate::mir::verification_types::VerificationError as Error;
use crate::mir::{BasicBlockId, MirFunction, MirInstruction};
use std::collections::{BTreeMap, HashMap, HashSet};

#[derive(Debug)]
pub(super) struct FunctionUseErrors {
    pub(super) ssa: Vec<Error>,
    pub(super) dominance: Vec<Error>,
    pub(super) merge: Vec<Error>,
}

pub(super) fn check(
    function: &MirFunction,
    definitions: &ValueDefMap,
    initial_ssa: Vec<Error>,
    dominators: &super::utils::DominatorTree,
    predecessors: &HashMap<BasicBlockId, Vec<BasicBlockId>>,
) -> FunctionUseErrors {
    let mut result = FunctionUseErrors {
        ssa: initial_ssa,
        dominance: Vec::new(),
        merge: Vec::new(),
    };
    let mut undefined = BTreeMap::<_, Vec<_>>::new();
    let mut phi_dominance = Vec::new();
    let violation = |value, target| {
        definitions
            .get(&value)
            .map(|(block, _)| *block)
            .filter(|block| *block != target && !dominators.dominates(*block, target))
    };
    for (block_id, block) in &function.blocks {
        let mut phi_destinations = HashSet::new();
        let mut merge_candidates = Vec::new();
        let merge = predecessors
            .get(block_id)
            .is_some_and(|rows| rows.len() >= 2);
        crate::mir::compile_timing::trace_refresh_block_visit();
        for (index, instruction) in block.all_instructions().enumerate() {
            crate::mir::compile_timing::trace_refresh_instruction_visit();
            let values = instruction.used_values();
            for value in &values {
                if !definitions.contains_key(value) {
                    undefined
                        .entry(*block_id)
                        .or_default()
                        .push(Error::UndefinedValue {
                            value: *value,
                            block: *block_id,
                            instruction_index: index,
                        });
                }
            }
            // SSA historically saw every row; dom/merge used iter_spanned's
            // zip, plus the terminator regardless of its optional span.
            if index < block.instructions.len() && index >= block.instruction_spans.len() {
                continue;
            }
            if let MirInstruction::Phi { dst, inputs, .. } = instruction {
                phi_destinations.insert(*dst);
                for (predecessor, value) in inputs {
                    if let Some(def_block) = violation(*value, *predecessor) {
                        phi_dominance.push(Error::DominatorViolation {
                            value: *value,
                            use_block: *predecessor,
                            def_block,
                        });
                    }
                }
            } else {
                for value in values {
                    if let Some(def_block) = violation(value, *block_id) {
                        result.dominance.push(Error::DominatorViolation {
                            value,
                            use_block: *block_id,
                            def_block,
                        });
                        if merge {
                            merge_candidates.push((value, def_block));
                        }
                    }
                }
            }
        }
        // A later Phi in malformed MIR still suppresses only the merge error,
        // exactly as the previous precomputed per-block Phi-destination set.
        result.merge.extend(
            merge_candidates
                .into_iter()
                .filter(|(value, _)| !phi_destinations.contains(value))
                .map(|(value, pred_block)| Error::MergeUsesPredecessorValue {
                    value,
                    merge_block: *block_id,
                    pred_block,
                }),
        );
    }
    for (block, errors) in undefined {
        for error in errors {
            if crate::config::env::joinir_dev::debug_enabled() {
                if let Error::UndefinedValue {
                    value,
                    instruction_index,
                    ..
                } = &error
                {
                    let instruction = function.blocks[&block]
                        .all_instructions()
                        .nth(*instruction_index)
                        .unwrap();
                    crate::runtime::get_global_ring0().log.debug(&format!(
                        "[ssa-undef-debug] fn={} bb={:?} inst_idx={} used={:?} inst={:?}",
                        function.signature.name, block, instruction_index, value, instruction
                    ));
                }
            }
            result.ssa.push(error);
        }
    }
    result.dominance.extend(phi_dominance);
    result
}
