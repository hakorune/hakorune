use crate::mir::{function::MirFunction, BasicBlockId, ValueId};
use std::collections::{HashMap, HashSet};

pub fn compute_predecessors(function: &MirFunction) -> HashMap<BasicBlockId, Vec<BasicBlockId>> {
    let mut preds: HashMap<BasicBlockId, Vec<BasicBlockId>> = HashMap::new();
    for (bid, block) in &function.blocks {
        for succ in &block.successors {
            preds.entry(*succ).or_default().push(*bid);
        }
    }
    preds
}

pub fn compute_def_blocks(function: &MirFunction) -> HashMap<ValueId, BasicBlockId> {
    let mut def_block: HashMap<ValueId, BasicBlockId> = HashMap::new();
    for pid in &function.params {
        def_block.insert(*pid, function.entry_block);
    }
    for (bid, block) in &function.blocks {
        for sp in block.all_spanned_instructions() {
            if let Some(dst) = sp.inst.dst_value() {
                def_block.insert(dst, *bid);
            }
        }
    }
    def_block
}

#[derive(Debug, Clone)]
pub struct DominatorTree {
    index_of: HashMap<BasicBlockId, usize>,
    tin: Vec<u32>,
    tout: Vec<u32>,
}

impl DominatorTree {
    /// Membership in the reachable CFG used by this same dominator computation.
    pub(in crate::mir) fn is_reachable(&self, block: BasicBlockId) -> bool {
        self.index_of.contains_key(&block)
    }

    #[inline]
    pub fn dominates(&self, a: BasicBlockId, b: BasicBlockId) -> bool {
        // Preserve legacy behavior: unreachable blocks are treated as "dominated by everything"
        // because older compute_dominators initialized unreachable dominator sets as all_blocks.
        let Some(&b_idx) = self.index_of.get(&b) else {
            return true;
        };
        let Some(&a_idx) = self.index_of.get(&a) else {
            return false;
        };
        self.tin[a_idx] <= self.tin[b_idx] && self.tout[b_idx] <= self.tout[a_idx]
    }
}

fn compute_reachable_rpo(function: &MirFunction) -> Vec<BasicBlockId> {
    let entry = function.entry_block;
    let mut visited: HashSet<BasicBlockId> = HashSet::new();
    let mut postorder: Vec<BasicBlockId> = Vec::new();
    let mut stack: Vec<(BasicBlockId, bool)> = Vec::new();
    stack.push((entry, false));

    while let Some((node, expanded)) = stack.pop() {
        if expanded {
            postorder.push(node);
            continue;
        }
        if !visited.insert(node) {
            continue;
        }
        stack.push((node, true));

        let Some(block) = function.blocks.get(&node) else {
            continue;
        };
        for succ in block.successors.iter().rev().copied() {
            if !visited.contains(&succ) {
                stack.push((succ, false));
            }
        }
    }

    postorder.reverse();
    postorder
}

pub fn compute_dominators(function: &MirFunction) -> DominatorTree {
    let preds = compute_predecessors(function);
    let reachable_rpo = compute_reachable_rpo(function);
    let mut index_of: HashMap<BasicBlockId, usize> = HashMap::new();
    for (i, bb) in reachable_rpo.iter().enumerate() {
        index_of.insert(*bb, i);
    }

    let n = reachable_rpo.len();
    let mut idom: Vec<Option<usize>> = vec![None; n];
    if n > 0 {
        idom[0] = Some(0);
    }

    fn intersect(mut f1: usize, mut f2: usize, idom: &[Option<usize>]) -> usize {
        while f1 != f2 {
            while f1 > f2 {
                f1 = idom[f1].expect("idom must be initialized for reachable blocks");
            }
            while f2 > f1 {
                f2 = idom[f2].expect("idom must be initialized for reachable blocks");
            }
        }
        f1
    }

    let mut changed = true;
    while changed {
        changed = false;
        for i in 1..n {
            let b = reachable_rpo[i];
            let Some(p_list) = preds.get(&b) else {
                continue;
            };

            let mut new_idom: Option<usize> = None;
            for p in p_list {
                let Some(&p_i) = index_of.get(p) else {
                    continue; // unreachable predecessor
                };
                if idom[p_i].is_some() {
                    new_idom = Some(p_i);
                    break;
                }
            }

            let Some(mut candidate) = new_idom else {
                continue;
            };

            for p in p_list {
                let Some(&p_i) = index_of.get(p) else {
                    continue;
                };
                if idom[p_i].is_some() {
                    candidate = intersect(p_i, candidate, &idom);
                }
            }

            if idom[i] != Some(candidate) {
                idom[i] = Some(candidate);
                changed = true;
            }
        }
    }

    let mut children: Vec<Vec<usize>> = vec![Vec::new(); n];
    for i in 1..n {
        if let Some(p) = idom[i] {
            if p != i {
                children[p].push(i);
            }
        }
    }

    let mut tin = vec![0u32; n];
    let mut tout = vec![0u32; n];
    let mut time: u32 = 1;
    let mut dfs_stack: Vec<(usize, usize)> = Vec::new();
    if n > 0 {
        tin[0] = time;
        time += 1;
        dfs_stack.push((0, 0));
    }

    while let Some((node, next_child_idx)) = dfs_stack.pop() {
        if next_child_idx < children[node].len() {
            dfs_stack.push((node, next_child_idx + 1));
            let child = children[node][next_child_idx];
            tin[child] = time;
            time += 1;
            dfs_stack.push((child, 0));
        } else {
            tout[node] = time;
            time += 1;
        }
    }

    DominatorTree {
        index_of,
        tin,
        tout,
    }
}

pub fn compute_reachable_blocks(function: &MirFunction) -> HashSet<BasicBlockId> {
    let mut reachable = HashSet::new();
    let mut worklist = vec![function.entry_block];
    while let Some(current) = worklist.pop() {
        if reachable.insert(current) {
            if let Some(block) = function.blocks.get(&current) {
                for successor in &block.successors {
                    if !reachable.contains(successor) {
                        worklist.push(*successor);
                    }
                }
                for sp in block.iter_spanned() {
                    if let crate::mir::MirInstruction::Catch { handler_bb, .. } = sp.inst {
                        if !reachable.contains(handler_bb) {
                            worklist.push(*handler_bb);
                        }
                    }
                }
                if let Some(ref terminator) = block.terminator {
                    if let crate::mir::MirInstruction::Catch { handler_bb, .. } = terminator {
                        if !reachable.contains(handler_bb) {
                            worklist.push(*handler_bb);
                        }
                    }
                }
            }
        }
    }
    reachable
}

/// Prove the exact integer operand at a use through unique ordered Copy defs.
/// The same check serves source append correspondence and finished MIR.
pub(in crate::mir) fn exact_integer_operand_origin(
    function: &MirFunction,
    value: ValueId,
    use_site: &(BasicBlockId, crate::mir::MirInstruction),
    expected: i64,
) -> Option<(BasicBlockId, crate::mir::MirInstruction)> {
    let (use_block, use_index) = {
        let body = function.blocks.get(&use_site.0)?;
        let mut matches = body
            .all_instructions()
            .enumerate()
            .filter(|(_, row)| *row == &use_site.1);
        let (index, _) = matches.next()?;
        if matches.next().is_some() {
            return None;
        }
        (use_site.0, index)
    };
    exact_integer_operand_origin_at(function, value, use_block, use_index, expected)
}

/// The same proof at a writer's insertion point, before any instruction is emitted.
pub(in crate::mir) fn exact_integer_operand_origin_at(
    function: &MirFunction,
    mut value: ValueId,
    mut use_block: BasicBlockId,
    mut use_index: usize,
    expected: i64,
) -> Option<(BasicBlockId, crate::mir::MirInstruction)> {
    use crate::mir::{ConstValue, MirInstruction};
    let mut seen = HashSet::new();
    let mut dominators = None;
    while seen.insert(value) {
        let mut definitions = function
            .blocks
            .iter()
            .flat_map(|(block, body)| {
                body.all_instructions()
                    .enumerate()
                    .map(move |(index, row)| (*block, index, row))
            })
            .filter(|(_, _, row)| row.dst_value() == Some(value));
        let (block, index, definition) = definitions.next()?;
        if definitions.next().is_some() {
            return None;
        }
        if block == use_block {
            if index >= use_index {
                return None;
            }
        } else {
            let tree = dominators.get_or_insert_with(|| compute_dominators(function));
            if !tree.is_reachable(block)
                || !tree.is_reachable(use_block)
                || !tree.dominates(block, use_block)
            {
                return None;
            }
        }
        match definition {
            MirInstruction::Const {
                value: ConstValue::Integer(actual),
                ..
            } if *actual == expected => return Some((block, definition.clone())),
            MirInstruction::Copy { src, .. } => {
                value = *src;
                use_block = block;
                use_index = index;
            }
            _ => return None,
        }
    }
    None
}
