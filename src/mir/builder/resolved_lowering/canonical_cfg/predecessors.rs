use super::error::{CanonicalCfgBlockRoleV1, CanonicalCfgErrorV1};
use crate::mir::{BasicBlockId, MirFunction, MirInstruction};
use std::collections::{BTreeMap, BTreeSet};

pub(super) type TerminatorPredecessorsV1 = BTreeMap<BasicBlockId, BTreeSet<BasicBlockId>>;

fn ids(values: &BTreeSet<BasicBlockId>) -> Box<[BasicBlockId]> {
    values
        .iter()
        .copied()
        .collect::<Vec<_>>()
        .into_boxed_slice()
}

fn reject_duplicate_branch_edge(
    source: BasicBlockId,
    terminator: Option<&MirInstruction>,
) -> Result<(), CanonicalCfgErrorV1> {
    let Some(MirInstruction::Branch {
        then_bb, else_bb, ..
    }) = terminator
    else {
        return Ok(());
    };
    if then_bb == else_bb {
        return Err(CanonicalCfgErrorV1::DuplicateEdge {
            source,
            target: *then_bb,
        });
    }
    Ok(())
}

pub(super) fn derive_and_verify_predecessors(
    function: &MirFunction,
) -> Result<TerminatorPredecessorsV1, CanonicalCfgErrorV1> {
    let block_ids = function.block_ids();
    let mut predecessors = block_ids
        .iter()
        .copied()
        .map(|block| (block, BTreeSet::new()))
        .collect::<TerminatorPredecessorsV1>();

    for source in block_ids.iter().copied() {
        let block = function
            .get_block(source)
            .ok_or(CanonicalCfgErrorV1::MissingBlock {
                block: source,
                role: CanonicalCfgBlockRoleV1::Source,
            })?;
        reject_duplicate_branch_edge(source, block.terminator.as_ref())?;

        let terminator_successors = block.successors_from_terminator();
        if terminator_successors != block.successors {
            return Err(CanonicalCfgErrorV1::CachedSuccessorsMismatch {
                block: source,
                terminator: ids(&terminator_successors),
                cached: ids(&block.successors),
            });
        }

        for target in terminator_successors {
            let Some(target_predecessors) = predecessors.get_mut(&target) else {
                return Err(CanonicalCfgErrorV1::DanglingTerminatorTarget { source, target });
            };
            target_predecessors.insert(source);
        }
    }

    for block_id in block_ids {
        let block = function
            .get_block(block_id)
            .ok_or(CanonicalCfgErrorV1::MissingBlock {
                block: block_id,
                role: CanonicalCfgBlockRoleV1::Target,
            })?;
        let expected = predecessors
            .get(&block_id)
            .expect("all function blocks were initialized");
        if expected != &block.predecessors {
            return Err(CanonicalCfgErrorV1::CachedPredecessorsMismatch {
                block: block_id,
                terminator: ids(expected),
                cached: ids(&block.predecessors),
            });
        }
    }

    Ok(predecessors)
}
