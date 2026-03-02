use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

use crate::mir::{BasicBlock, BasicBlockId, BinaryOp, MirFunction, MirInstruction, ValueId};

#[derive(Debug, Clone)]
pub(super) struct Concat3Plan {
    pub(super) outer_idx: usize,
    pub(super) inner_idx: usize,
    pub(super) inner_dst: ValueId,
    pub(super) outer_dst: ValueId,
    pub(super) args: [ValueId; 3],
}

pub(super) fn collect_plans(
    function: &MirFunction,
    stringish: &HashSet<ValueId>,
    def_map: &HashMap<ValueId, (BasicBlockId, usize)>,
    use_counts: &HashMap<ValueId, usize>,
) -> BTreeMap<BasicBlockId, Vec<Concat3Plan>> {
    let mut plans_by_block: BTreeMap<BasicBlockId, Vec<Concat3Plan>> = BTreeMap::new();
    let mut block_ids: Vec<_> = function.blocks.keys().copied().collect();
    block_ids.sort();

    for bbid in block_ids {
        let Some(block) = function.blocks.get(&bbid) else {
            continue;
        };
        let mut claimed: BTreeSet<usize> = BTreeSet::new();

        for outer_idx in 0..block.instructions.len() {
            if claimed.contains(&outer_idx) {
                continue;
            }

            let (outer_dst, outer_lhs, outer_rhs) = match &block.instructions[outer_idx] {
                MirInstruction::BinOp {
                    dst,
                    op: BinaryOp::Add,
                    lhs,
                    rhs,
                } => (*dst, *lhs, *rhs),
                _ => continue,
            };

            if !is_string_add_candidate(outer_dst, outer_lhs, outer_rhs, stringish) {
                continue;
            }

            let picked = pick_inner_concat_candidate(
                block, bbid, outer_idx, outer_lhs, outer_rhs, stringish, def_map, use_counts,
                &claimed,
            );

            let Some((inner_idx, inner_dst, args)) = picked else {
                continue;
            };

            claimed.insert(inner_idx);
            claimed.insert(outer_idx);
            plans_by_block.entry(bbid).or_default().push(Concat3Plan {
                outer_idx,
                inner_idx,
                inner_dst,
                outer_dst,
                args,
            });
        }
    }

    plans_by_block
}

fn pick_inner_concat_candidate(
    block: &BasicBlock,
    bbid: BasicBlockId,
    outer_idx: usize,
    outer_lhs: ValueId,
    outer_rhs: ValueId,
    stringish: &HashSet<ValueId>,
    def_map: &HashMap<ValueId, (BasicBlockId, usize)>,
    use_counts: &HashMap<ValueId, usize>,
    claimed: &BTreeSet<usize>,
) -> Option<(usize, ValueId, [ValueId; 3])> {
    // Left-assoc first: (a + b) + c
    if let Some(candidate) = inspect_inner_add(
        block, bbid, outer_idx, outer_lhs, stringish, def_map, use_counts, claimed,
    ) {
        let (inner_idx, inner_dst, il, ir) = candidate;
        return Some((inner_idx, inner_dst, [il, ir, outer_rhs]));
    }

    // Right-assoc: a + (b + c)
    if let Some(candidate) = inspect_inner_add(
        block, bbid, outer_idx, outer_rhs, stringish, def_map, use_counts, claimed,
    ) {
        let (inner_idx, inner_dst, il, ir) = candidate;
        return Some((inner_idx, inner_dst, [outer_lhs, il, ir]));
    }

    None
}

fn inspect_inner_add(
    block: &BasicBlock,
    bbid: BasicBlockId,
    outer_idx: usize,
    inner_vid: ValueId,
    stringish: &HashSet<ValueId>,
    def_map: &HashMap<ValueId, (BasicBlockId, usize)>,
    use_counts: &HashMap<ValueId, usize>,
    claimed: &BTreeSet<usize>,
) -> Option<(usize, ValueId, ValueId, ValueId)> {
    let (def_bbid, inner_idx) = *def_map.get(&inner_vid)?;
    if def_bbid != bbid || inner_idx >= outer_idx || claimed.contains(&inner_idx) {
        return None;
    }

    let (inner_dst, il, ir) = match &block.instructions[inner_idx] {
        MirInstruction::BinOp {
            dst,
            op: BinaryOp::Add,
            lhs,
            rhs,
        } => (*dst, *lhs, *rhs),
        _ => return None,
    };

    if inner_dst != inner_vid {
        return None;
    }
    if use_counts.get(&inner_dst).copied().unwrap_or(0) != 1 {
        return None;
    }
    if !is_string_add_candidate(inner_dst, il, ir, stringish) {
        return None;
    }

    Some((inner_idx, inner_dst, il, ir))
}

fn is_string_add_candidate(
    dst: ValueId,
    lhs: ValueId,
    rhs: ValueId,
    stringish: &HashSet<ValueId>,
) -> bool {
    stringish.contains(&dst) || stringish.contains(&lhs) || stringish.contains(&rhs)
}
