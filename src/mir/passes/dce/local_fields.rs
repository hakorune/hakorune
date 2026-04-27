use crate::mir::classify_escape_uses;
use crate::mir::phi_query::{
    collect_passthrough_phi_parents, infer_phi_base_query_with_anchors, PhiBaseRelation,
};
use crate::mir::value_origin::{
    build_value_def_map, resolve_value_origin_from_parent_map, ParentMap,
};
use crate::mir::{MirFunction, ValueId};
use std::collections::{BTreeSet, HashMap, HashSet};

#[derive(Default)]
pub(crate) struct LocalReadInfo {
    local_boxes: HashSet<ValueId>,
    escaping: HashSet<ValueId>,
    field_read_roots: HashSet<ValueId>,
    alias_parents: ParentMap,
    local_root_overrides: HashMap<ValueId, ValueId>,
}

impl LocalReadInfo {
    pub(crate) fn resolve_local_root(&self, value: ValueId) -> Option<ValueId> {
        let root = resolve_value_origin_from_parent_map(value, &self.alias_parents);
        if self.local_boxes.contains(&root) {
            return Some(root);
        }
        self.local_root_overrides.get(&root).copied()
    }

    pub(crate) fn is_non_escaping_local(&self, value: ValueId) -> bool {
        self.resolve_local_root(value)
            .is_some_and(|root| !self.escaping.contains(&root))
    }

    pub(crate) fn is_unobserved_local(&self, value: ValueId) -> bool {
        self.resolve_local_root(value).is_some_and(|root| {
            !self.escaping.contains(&root) && !self.field_read_roots.contains(&root)
        })
    }
}

pub(crate) fn analyze_local_reads(
    function: &MirFunction,
    reachable_blocks: &HashSet<crate::mir::BasicBlockId>,
) -> LocalReadInfo {
    let mut info = LocalReadInfo::default();

    for (bid, block) in &function.blocks {
        if !reachable_blocks.contains(bid) {
            continue;
        }

        for instruction in &block.instructions {
            if let crate::mir::MirInstruction::NewBox { dst, .. } = instruction {
                info.local_boxes.insert(*dst);
            }
            if let crate::mir::MirInstruction::Copy { dst, src } = instruction {
                info.alias_parents.insert(*dst, *src);
            }
        }
    }

    info.alias_parents
        .extend(collect_passthrough_phi_parents(function));

    let anchors: BTreeSet<ValueId> = info.local_boxes.iter().copied().collect();
    let def_map = build_value_def_map(function);
    for (bid, block) in &function.blocks {
        if !reachable_blocks.contains(bid) {
            continue;
        }

        for instruction in &block.instructions {
            let Some(dst) = instruction.dst_value() else {
                continue;
            };
            let root = resolve_value_origin_from_parent_map(dst, &info.alias_parents);
            if info.local_boxes.contains(&root) || info.local_root_overrides.contains_key(&root) {
                continue;
            }
            let query = infer_phi_base_query_with_anchors(function, &def_map, root, &anchors);
            if let PhiBaseRelation::SameBase(base) = query.relation {
                if info.local_boxes.contains(&base) {
                    info.local_root_overrides.insert(root, base);
                }
            }
        }
    }

    for (bid, block) in &function.blocks {
        if !reachable_blocks.contains(bid) {
            continue;
        }

        for instruction in &block.instructions {
            let allow_same_root_phi_merge = matches!(
                instruction,
                crate::mir::MirInstruction::Phi { dst, inputs, .. }
                    if inputs.len() > 1 && info.resolve_local_root(*dst).is_some()
            );
            if !allow_same_root_phi_merge {
                for use_site in classify_escape_uses(instruction) {
                    if let Some(root) = info.resolve_local_root(use_site.value) {
                        info.escaping.insert(root);
                    }
                }
            }
            if let crate::mir::MirInstruction::FieldGet { base, .. } = instruction {
                if let Some(root) = info.resolve_local_root(*base) {
                    info.field_read_roots.insert(root);
                }
            }
        }

        if let Some(term) = &block.terminator {
            for use_site in classify_escape_uses(term) {
                if let Some(root) = info.resolve_local_root(use_site.value) {
                    info.escaping.insert(root);
                }
            }
        }
    }

    info
}

pub(super) fn is_removable_effect_sensitive_read_instruction(
    inst: &crate::mir::MirInstruction,
    local_reads: &LocalReadInfo,
) -> bool {
    matches!(
        inst,
        crate::mir::MirInstruction::FieldGet { base, .. } if local_reads.is_non_escaping_local(*base)
    )
}

pub(super) fn is_removable_effect_sensitive_write_instruction(
    inst: &crate::mir::MirInstruction,
    local_reads: &LocalReadInfo,
) -> bool {
    matches!(
        inst,
        crate::mir::MirInstruction::FieldSet { base, .. } if local_reads.is_unobserved_local(*base)
    )
}

pub(super) fn collect_overwritten_local_field_sets(
    function: &MirFunction,
    reachable_blocks: &HashSet<crate::mir::BasicBlockId>,
    local_reads: &LocalReadInfo,
) -> HashSet<(crate::mir::BasicBlockId, usize)> {
    let mut removable = HashSet::new();
    let dominators = crate::mir::verification::utils::compute_dominators(function);
    let mut propagation_successors: HashMap<crate::mir::BasicBlockId, crate::mir::BasicBlockId> =
        HashMap::new();

    for (bid, block) in &function.blocks {
        if !reachable_blocks.contains(bid) {
            continue;
        }
        let reachable_successors: Vec<_> = block
            .successors
            .iter()
            .copied()
            .filter(|succ| reachable_blocks.contains(succ))
            .collect();
        if reachable_successors.len() != 1 {
            continue;
        }
        let succ = reachable_successors[0];
        if dominators.dominates(succ, *bid) {
            continue;
        }
        propagation_successors.insert(*bid, succ);
    }

    let mut pending_entry_writes: HashMap<crate::mir::BasicBlockId, HashSet<(ValueId, String)>> =
        HashMap::new();
    let mut changed = true;
    while changed {
        changed = false;
        for (bid, block) in &function.blocks {
            if !reachable_blocks.contains(bid) {
                continue;
            }

            let mut pending_writes = propagation_successors
                .get(bid)
                .and_then(|succ| pending_entry_writes.get(succ).cloned())
                .unwrap_or_default();

            for instruction in block.instructions.iter().rev() {
                match instruction {
                    crate::mir::MirInstruction::FieldSet { base, field, .. } => {
                        let Some(root) = local_reads.resolve_local_root(*base) else {
                            continue;
                        };
                        pending_writes.insert((root, field.clone()));
                    }
                    crate::mir::MirInstruction::FieldGet { base, field, .. } => {
                        if let Some(root) = local_reads.resolve_local_root(*base) {
                            pending_writes.remove(&(root, field.clone()));
                        }
                    }
                    _ => {}
                }

                for use_site in classify_escape_uses(instruction) {
                    if let Some(root) = local_reads.resolve_local_root(use_site.value) {
                        pending_writes.retain(|(pending_root, _)| *pending_root != root);
                    }
                }
            }

            let entry = pending_entry_writes.entry(*bid).or_default();
            if *entry != pending_writes {
                *entry = pending_writes;
                changed = true;
            }
        }
    }

    for (bid, block) in &function.blocks {
        if !reachable_blocks.contains(bid) {
            continue;
        }

        let mut pending_writes = propagation_successors
            .get(bid)
            .and_then(|succ| pending_entry_writes.get(succ).cloned())
            .unwrap_or_default();
        for (idx, instruction) in block.instructions.iter().enumerate().rev() {
            match instruction {
                crate::mir::MirInstruction::FieldSet { base, field, .. } => {
                    let Some(root) = local_reads.resolve_local_root(*base) else {
                        continue;
                    };
                    let key = (root, field.clone());
                    if pending_writes.contains(&key) {
                        removable.insert((*bid, idx));
                    }
                    pending_writes.insert(key);
                }
                crate::mir::MirInstruction::FieldGet { base, field, .. } => {
                    if let Some(root) = local_reads.resolve_local_root(*base) {
                        pending_writes.remove(&(root, field.clone()));
                    }
                }
                _ => {}
            }

            for use_site in classify_escape_uses(instruction) {
                if let Some(root) = local_reads.resolve_local_root(use_site.value) {
                    pending_writes.retain(|(pending_root, _)| *pending_root != root);
                }
            }
        }
    }

    removable.extend(collect_loop_roundtrip_overwritten_local_field_sets(
        function,
        reachable_blocks,
        local_reads,
        &dominators,
    ));

    removable
}

fn collect_loop_header_entry_overwrites(
    header: &crate::mir::BasicBlock,
    local_reads: &LocalReadInfo,
) -> HashSet<(ValueId, String)> {
    let mut confirmed = HashSet::new();
    let mut blocked_roots = HashSet::new();
    let mut blocked_keys = HashSet::new();

    for instruction in &header.instructions {
        match instruction {
            crate::mir::MirInstruction::FieldGet { base, field, .. } => {
                if let Some(root) = local_reads.resolve_local_root(*base) {
                    blocked_keys.insert((root, field.clone()));
                }
            }
            crate::mir::MirInstruction::FieldSet { base, field, .. } => {
                let Some(root) = local_reads.resolve_local_root(*base) else {
                    continue;
                };
                let key = (root, field.clone());
                if !blocked_roots.contains(&root) && !blocked_keys.contains(&key) {
                    confirmed.insert(key);
                }
            }
            _ => {}
        }

        let allow_same_root_phi_merge = matches!(
            instruction,
            crate::mir::MirInstruction::Phi { dst, inputs, .. }
                if inputs.len() > 1 && local_reads.resolve_local_root(*dst).is_some()
        );
        if !allow_same_root_phi_merge {
            for use_site in classify_escape_uses(instruction) {
                if let Some(root) = local_reads.resolve_local_root(use_site.value) {
                    blocked_roots.insert(root);
                }
            }
        }
    }

    confirmed
}

fn collect_loop_roundtrip_overwritten_local_field_sets(
    function: &MirFunction,
    reachable_blocks: &HashSet<crate::mir::BasicBlockId>,
    local_reads: &LocalReadInfo,
    dominators: &crate::mir::verification::utils::DominatorTree,
) -> HashSet<(crate::mir::BasicBlockId, usize)> {
    let mut removable = HashSet::new();

    for (header_id, header) in &function.blocks {
        if !reachable_blocks.contains(header_id) {
            continue;
        }

        let header_entry_overwrites = collect_loop_header_entry_overwrites(header, local_reads);
        if header_entry_overwrites.is_empty() {
            continue;
        }

        for predecessor in &header.predecessors {
            if !reachable_blocks.contains(predecessor) {
                continue;
            }
            if !dominators.dominates(*header_id, *predecessor) {
                continue;
            }

            let Some(pred_block) = function.blocks.get(predecessor) else {
                continue;
            };
            let reachable_successors: Vec<_> = pred_block
                .successors
                .iter()
                .copied()
                .filter(|succ| reachable_blocks.contains(succ))
                .collect();
            if reachable_successors.len() != 1 || reachable_successors[0] != *header_id {
                continue;
            }

            let mut pending_writes = header_entry_overwrites.clone();
            if let Some(term) = &pred_block.terminator {
                for use_site in classify_escape_uses(term) {
                    if let Some(root) = local_reads.resolve_local_root(use_site.value) {
                        pending_writes.retain(|(pending_root, _)| *pending_root != root);
                    }
                }
            }

            for (idx, instruction) in pred_block.instructions.iter().enumerate().rev() {
                match instruction {
                    crate::mir::MirInstruction::FieldSet { base, field, .. } => {
                        let Some(root) = local_reads.resolve_local_root(*base) else {
                            continue;
                        };
                        let key = (root, field.clone());
                        if pending_writes.contains(&key) {
                            removable.insert((*predecessor, idx));
                        }
                        pending_writes.insert(key);
                    }
                    crate::mir::MirInstruction::FieldGet { base, field, .. } => {
                        if let Some(root) = local_reads.resolve_local_root(*base) {
                            pending_writes.remove(&(root, field.clone()));
                        }
                    }
                    _ => {}
                }

                for use_site in classify_escape_uses(instruction) {
                    if let Some(root) = local_reads.resolve_local_root(use_site.value) {
                        pending_writes.retain(|(pending_root, _)| *pending_root != root);
                    }
                }
            }
        }
    }

    removable
}
