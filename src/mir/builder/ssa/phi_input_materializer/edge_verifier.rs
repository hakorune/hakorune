//! FINALIZE0-PHI-SPLIT0-S0 disconnected PHI-edge verification products.
//!
//! This module reads terminators directly and owns no CFG-cache update, MIR
//! rewrite, ValueId allocation, type/origin publication, or Builder state.

use crate::ast::Span;
use crate::mir::{BasicBlockId, MirFunction, MirInstruction, ValueId};
use std::collections::{BTreeMap, BTreeSet, VecDeque};

/// Stable all-build PHI-edge failure vocabulary.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::mir::builder) enum PhiEdgeVerificationErrorV1 {
    MissingEntryBlock {
        entry: BasicBlockId,
    },
    MissingSuccessorBlock {
        block: BasicBlockId,
        successor: BasicBlockId,
    },
    SuccessorCacheMismatch {
        block: BasicBlockId,
    },
    DuplicatePredecessor {
        block: BasicBlockId,
        phi_ordinal: u32,
        predecessor: BasicBlockId,
    },
    PhantomPredecessor {
        block: BasicBlockId,
        phi_ordinal: u32,
        predecessor: BasicBlockId,
    },
    MissingPredecessor {
        block: BasicBlockId,
        phi_ordinal: u32,
        predecessor: BasicBlockId,
    },
    UndefinedIncoming {
        block: BasicBlockId,
        phi_ordinal: u32,
        predecessor: BasicBlockId,
        value: ValueId,
    },
    NonDominatingIncoming {
        block: BasicBlockId,
        phi_ordinal: u32,
        predecessor: BasicBlockId,
        value: ValueId,
        definition: BasicBlockId,
    },
}

impl PhiEdgeVerificationErrorV1 {
    fn sort_key(&self) -> (u8, u32, u32, u32, u32) {
        match self {
            Self::MissingEntryBlock { entry } => (0, entry.0, 0, 0, 0),
            Self::MissingSuccessorBlock { block, successor } => (1, block.0, 0, successor.0, 0),
            Self::SuccessorCacheMismatch { block } => (2, block.0, 0, 0, 0),
            Self::DuplicatePredecessor {
                block,
                phi_ordinal,
                predecessor,
            } => (3, block.0, *phi_ordinal, predecessor.0, 0),
            Self::PhantomPredecessor {
                block,
                phi_ordinal,
                predecessor,
            } => (4, block.0, *phi_ordinal, predecessor.0, 0),
            Self::MissingPredecessor {
                block,
                phi_ordinal,
                predecessor,
            } => (5, block.0, *phi_ordinal, predecessor.0, 0),
            Self::UndefinedIncoming {
                block,
                phi_ordinal,
                predecessor,
                value,
            } => (6, block.0, *phi_ordinal, predecessor.0, value.0),
            Self::NonDominatingIncoming {
                block,
                phi_ordinal,
                predecessor,
                value,
                ..
            } => (7, block.0, *phi_ordinal, predecessor.0, value.0),
        }
    }
}

impl std::fmt::Display for PhiEdgeVerificationErrorV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "[freeze:contract][phi_edge/verification] {self:?}"
        )
    }
}

impl std::error::Error for PhiEdgeVerificationErrorV1 {}

/// Read-only candidate for an unused Phi deletion.
///
/// It intentionally exposes no commit. A later side-artifact closure owner
/// must prove that positional and value-indexed artifacts can be refreshed
/// before it may add a mutation surface.
#[derive(Clone, Debug, PartialEq)]
pub(in crate::mir::builder) struct UnusedPhiCandidateV1 {
    block: BasicBlockId,
    instruction_index: u32,
    destination: ValueId,
    span: Span,
}

/// Private non-Clone unused-Phi candidate plan.
#[derive(Debug, PartialEq)]
pub(in crate::mir::builder) struct PreparedUnusedPhiNormalizationV1 {
    candidates: Box<[UnusedPhiCandidateV1]>,
    _seal: UnusedPhiNormalizationSealV1,
}

#[derive(Debug, PartialEq)]
struct UnusedPhiNormalizationSealV1;

/// A deletion remains blocked until a positional-artifact owner closes it.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::mir::builder) enum UnusedPhiNormalizationErrorV1 {
    MissingInstructionSpan {
        block: BasicBlockId,
        instruction_index: u32,
        destination: ValueId,
    },
    BlockedByArtifactReference {
        block: BasicBlockId,
        instruction_index: u32,
        destination: ValueId,
    },
}

impl PreparedUnusedPhiNormalizationV1 {
    /// Collects candidate rows only; it performs no live mutation.
    pub(in crate::mir::builder) fn collect(
        function: &MirFunction,
    ) -> Result<Self, UnusedPhiNormalizationErrorV1> {
        let mut used = BTreeSet::new();
        for block_id in sorted_block_ids(function) {
            let block = &function.blocks[&block_id];
            for instruction in block.all_instructions() {
                used.extend(instruction.used_values());
            }
        }

        let mut candidates = Vec::new();
        for block_id in sorted_block_ids(function) {
            let block = &function.blocks[&block_id];
            for (instruction_index, instruction) in block.instructions.iter().enumerate() {
                let MirInstruction::Phi { dst, .. } = instruction else {
                    continue;
                };
                if !used.contains(dst) {
                    let Some(span) = block.instruction_span(instruction_index) else {
                        return Err(UnusedPhiNormalizationErrorV1::MissingInstructionSpan {
                            block: block_id,
                            instruction_index: instruction_index as u32,
                            destination: *dst,
                        });
                    };
                    candidates.push(UnusedPhiCandidateV1 {
                        block: block_id,
                        instruction_index: instruction_index as u32,
                        destination: *dst,
                        span,
                    });
                }
            }
        }
        Ok(Self {
            candidates: candidates.into_boxed_slice(),
            _seal: UnusedPhiNormalizationSealV1,
        })
    }

    /// Refuses to delete until a future artifact-closure owner proves safety.
    pub(in crate::mir::builder) fn require_artifact_closure(
        &self,
    ) -> Result<(), UnusedPhiNormalizationErrorV1> {
        let Some(candidate) = self.candidates.first() else {
            return Ok(());
        };
        Err(UnusedPhiNormalizationErrorV1::BlockedByArtifactReference {
            block: candidate.block,
            instruction_index: candidate.instruction_index,
            destination: candidate.destination,
        })
    }

    #[cfg(test)]
    fn candidates(&self) -> &[UnusedPhiCandidateV1] {
        &self.candidates
    }
}

/// Verifies reachable PHI edges without reading or updating CFG caches.
pub(in crate::mir::builder) fn verify_phi_edges_v1(
    function: &MirFunction,
) -> Result<(), Vec<PhiEdgeVerificationErrorV1>> {
    let (predecessors, reachable, mut errors) = terminator_cfg(function);
    let definitions = definitions(function);
    let dominators = terminator_dominators(function, &predecessors, &reachable);

    for block_id in sorted_block_ids(function) {
        if !reachable.contains(&block_id) {
            continue;
        }
        let block = &function.blocks[&block_id];
        let expected_predecessors = predecessors.get(&block_id).cloned().unwrap_or_default();
        let mut phi_ordinal = 0u32;
        for instruction in &block.instructions {
            let MirInstruction::Phi { inputs, .. } = instruction else {
                continue;
            };
            let mut seen = BTreeSet::new();
            for (predecessor, incoming) in inputs {
                if !seen.insert(*predecessor) {
                    errors.push(PhiEdgeVerificationErrorV1::DuplicatePredecessor {
                        block: block_id,
                        phi_ordinal,
                        predecessor: *predecessor,
                    });
                }
                if !expected_predecessors.contains(predecessor) {
                    errors.push(PhiEdgeVerificationErrorV1::PhantomPredecessor {
                        block: block_id,
                        phi_ordinal,
                        predecessor: *predecessor,
                    });
                    continue;
                }
                let Some(definition) = definitions.get(incoming).copied() else {
                    errors.push(PhiEdgeVerificationErrorV1::UndefinedIncoming {
                        block: block_id,
                        phi_ordinal,
                        predecessor: *predecessor,
                        value: *incoming,
                    });
                    continue;
                };
                if definition != *predecessor
                    && !dominators
                        .get(predecessor)
                        .is_some_and(|set| set.contains(&definition))
                {
                    errors.push(PhiEdgeVerificationErrorV1::NonDominatingIncoming {
                        block: block_id,
                        phi_ordinal,
                        predecessor: *predecessor,
                        value: *incoming,
                        definition,
                    });
                }
            }
            for predecessor in &expected_predecessors {
                if reachable.contains(predecessor) && !seen.contains(predecessor) {
                    errors.push(PhiEdgeVerificationErrorV1::MissingPredecessor {
                        block: block_id,
                        phi_ordinal,
                        predecessor: *predecessor,
                    });
                }
            }
            phi_ordinal += 1;
        }
    }

    errors.sort_by_key(PhiEdgeVerificationErrorV1::sort_key);
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn terminator_cfg(
    function: &MirFunction,
) -> (
    BTreeMap<BasicBlockId, BTreeSet<BasicBlockId>>,
    BTreeSet<BasicBlockId>,
    Vec<PhiEdgeVerificationErrorV1>,
) {
    let mut predecessors = BTreeMap::new();
    let mut errors = Vec::new();
    for block_id in sorted_block_ids(function) {
        let block = &function.blocks[&block_id];
        let successors = block.successors_from_terminator();
        if successors != block.successors {
            errors.push(PhiEdgeVerificationErrorV1::SuccessorCacheMismatch { block: block_id });
        }
        for successor in successors {
            if !function.blocks.contains_key(&successor) {
                errors.push(PhiEdgeVerificationErrorV1::MissingSuccessorBlock {
                    block: block_id,
                    successor,
                });
                continue;
            }
            predecessors
                .entry(successor)
                .or_insert_with(BTreeSet::new)
                .insert(block_id);
        }
    }

    if !function.blocks.contains_key(&function.entry_block) {
        errors.push(PhiEdgeVerificationErrorV1::MissingEntryBlock {
            entry: function.entry_block,
        });
        return (predecessors, BTreeSet::new(), errors);
    }

    let mut reachable = BTreeSet::new();
    let mut queue = VecDeque::from([function.entry_block]);
    while let Some(block_id) = queue.pop_front() {
        if !reachable.insert(block_id) {
            continue;
        }
        let block = &function.blocks[&block_id];
        for successor in block.successors_from_terminator() {
            if function.blocks.contains_key(&successor) {
                queue.push_back(successor);
            }
        }
    }
    (predecessors, reachable, errors)
}

fn definitions(function: &MirFunction) -> BTreeMap<ValueId, BasicBlockId> {
    let mut definitions = BTreeMap::new();
    for parameter in &function.params {
        definitions.insert(*parameter, function.entry_block);
    }
    for block_id in sorted_block_ids(function) {
        let block = &function.blocks[&block_id];
        for instruction in block.all_instructions() {
            if let Some(destination) = instruction.dst_value() {
                definitions.insert(destination, block_id);
            }
        }
    }
    definitions
}

fn terminator_dominators(
    function: &MirFunction,
    predecessors: &BTreeMap<BasicBlockId, BTreeSet<BasicBlockId>>,
    reachable: &BTreeSet<BasicBlockId>,
) -> BTreeMap<BasicBlockId, BTreeSet<BasicBlockId>> {
    let mut dominators = BTreeMap::new();
    for block_id in reachable {
        let initial = if *block_id == function.entry_block {
            BTreeSet::from([*block_id])
        } else {
            reachable.clone()
        };
        dominators.insert(*block_id, initial);
    }

    let mut changed = true;
    while changed {
        changed = false;
        for block_id in reachable {
            if *block_id == function.entry_block {
                continue;
            }
            let incoming = predecessors
                .get(block_id)
                .into_iter()
                .flatten()
                .filter(|predecessor| reachable.contains(predecessor));
            let mut intersection: Option<BTreeSet<BasicBlockId>> = None;
            for predecessor in incoming {
                let predecessor_dominators = &dominators[predecessor];
                intersection = Some(match intersection {
                    Some(current) => current
                        .intersection(predecessor_dominators)
                        .copied()
                        .collect(),
                    None => predecessor_dominators.clone(),
                });
            }
            let mut next = intersection.unwrap_or_default();
            next.insert(*block_id);
            if dominators.get(block_id) != Some(&next) {
                dominators.insert(*block_id, next);
                changed = true;
            }
        }
    }
    dominators
}

fn sorted_block_ids(function: &MirFunction) -> Vec<BasicBlockId> {
    let mut block_ids = function.blocks.keys().copied().collect::<Vec<_>>();
    block_ids.sort_by_key(|block_id| block_id.0);
    block_ids
}

#[cfg(test)]
mod tests {
    use super::{
        verify_phi_edges_v1, PhiEdgeVerificationErrorV1, PreparedUnusedPhiNormalizationV1,
        UnusedPhiNormalizationErrorV1,
    };
    use crate::mir::{
        BasicBlock, BasicBlockId, ConstValue, EffectMask, FunctionSignature, MirFunction,
        MirInstruction, MirType,
    };

    fn function() -> MirFunction {
        MirFunction::new(
            FunctionSignature {
                name: "phi-edge-verifier/0".to_string(),
                params: Vec::new(),
                return_type: MirType::Void,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        )
    }

    #[test]
    fn valid_self_carried_phi_verifies_without_cfg_mutation() {
        let mut function = function();
        function.add_block(BasicBlock::new(BasicBlockId::new(1)));
        let seed = function.next_value_id();
        let phi = function.next_value_id();
        function
            .get_block_mut(BasicBlockId::new(0))
            .unwrap()
            .add_instruction(MirInstruction::Const {
                dst: seed,
                value: ConstValue::Integer(0),
            });
        function
            .get_block_mut(BasicBlockId::new(0))
            .unwrap()
            .set_terminator(MirInstruction::Jump {
                target: BasicBlockId::new(1),
                edge_args: None,
            });
        function
            .get_block_mut(BasicBlockId::new(1))
            .unwrap()
            .add_instruction(MirInstruction::Phi {
                dst: phi,
                inputs: vec![(BasicBlockId::new(0), seed), (BasicBlockId::new(1), phi)],
                type_hint: None,
            });
        function
            .get_block_mut(BasicBlockId::new(1))
            .unwrap()
            .set_terminator(MirInstruction::Jump {
                target: BasicBlockId::new(1),
                edge_args: None,
            });

        let cached_predecessors = function
            .get_block(BasicBlockId::new(1))
            .unwrap()
            .predecessors
            .clone();
        assert_eq!(verify_phi_edges_v1(&function), Ok(()));
        assert_eq!(
            function
                .get_block(BasicBlockId::new(1))
                .unwrap()
                .predecessors,
            cached_predecessors
        );
    }

    #[test]
    fn duplicate_phi_predecessor_is_a_stable_error() {
        let mut function = function();
        function.add_block(BasicBlock::new(BasicBlockId::new(1)));
        let seed = function.next_value_id();
        let phi = function.next_value_id();
        function
            .get_block_mut(BasicBlockId::new(0))
            .unwrap()
            .add_instruction(MirInstruction::Const {
                dst: seed,
                value: ConstValue::Integer(0),
            });
        function
            .get_block_mut(BasicBlockId::new(0))
            .unwrap()
            .set_terminator(MirInstruction::Jump {
                target: BasicBlockId::new(1),
                edge_args: None,
            });
        function
            .get_block_mut(BasicBlockId::new(1))
            .unwrap()
            .add_instruction(MirInstruction::Phi {
                dst: phi,
                inputs: vec![(BasicBlockId::new(0), seed), (BasicBlockId::new(0), seed)],
                type_hint: None,
            });

        assert_eq!(
            verify_phi_edges_v1(&function),
            Err(vec![PhiEdgeVerificationErrorV1::DuplicatePredecessor {
                block: BasicBlockId::new(1),
                phi_ordinal: 0,
                predecessor: BasicBlockId::new(0),
            }])
        );
    }

    #[test]
    fn unused_phi_plan_is_blocked_without_a_mutation_surface() {
        let mut function = function();
        let phi = function.next_value_id();
        function
            .get_block_mut(BasicBlockId::new(0))
            .unwrap()
            .add_instruction(MirInstruction::Phi {
                dst: phi,
                inputs: Vec::new(),
                type_hint: None,
            });
        let instruction_count = function
            .get_block(BasicBlockId::new(0))
            .unwrap()
            .instructions
            .len();

        let prepared = PreparedUnusedPhiNormalizationV1::collect(&function).unwrap();
        assert_eq!(prepared.candidates().len(), 1);
        assert_eq!(
            prepared.require_artifact_closure(),
            Err(UnusedPhiNormalizationErrorV1::BlockedByArtifactReference {
                block: BasicBlockId::new(0),
                instruction_index: 0,
                destination: phi,
            })
        );
        assert_eq!(
            function
                .get_block(BasicBlockId::new(0))
                .unwrap()
                .instructions
                .len(),
            instruction_count
        );
    }

    #[test]
    fn unused_phi_plan_rejects_a_missing_aligned_span() {
        let mut function = function();
        let phi = function.next_value_id();
        let block = function.get_block_mut(BasicBlockId::new(0)).unwrap();
        block.add_instruction(MirInstruction::Phi {
            dst: phi,
            inputs: Vec::new(),
            type_hint: None,
        });
        block.instruction_spans.clear();

        assert_eq!(
            PreparedUnusedPhiNormalizationV1::collect(&function),
            Err(UnusedPhiNormalizationErrorV1::MissingInstructionSpan {
                block: BasicBlockId::new(0),
                instruction_index: 0,
                destination: phi,
            })
        );
    }
}
