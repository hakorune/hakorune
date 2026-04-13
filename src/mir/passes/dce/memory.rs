//! Memory-effect analysis helpers shared with the top-level memory-effect owner seam.

use super::local_fields::LocalReadInfo;
use crate::mir::{resolve_value_origin_from_parent_map, MirFunction, ParentMap, ValueId};
use std::collections::HashSet;

#[derive(Default)]
pub(crate) struct PrivateCarrierInfo {
    private_roots: HashSet<ValueId>,
    escaping_roots: HashSet<ValueId>,
    load_blocked_roots: HashSet<ValueId>,
    alias_parents: ParentMap,
}

impl PrivateCarrierInfo {
    fn resolve_private_root(&self, value: ValueId) -> Option<ValueId> {
        let root = resolve_value_origin_from_parent_map(value, &self.alias_parents);
        self.private_roots.contains(&root).then_some(root)
    }

    pub(crate) fn resolve_private_store_root(&self, value: ValueId) -> Option<ValueId> {
        self.resolve_private_root(value)
            .filter(|root| !self.escaping_roots.contains(root))
    }

    pub(super) fn is_private_carrier(&self, value: ValueId) -> bool {
        self.resolve_private_root(value).is_some_and(|root| {
            !self.escaping_roots.contains(&root) && !self.load_blocked_roots.contains(&root)
        })
    }
}

pub(crate) fn analyze_private_carriers(
    function: &MirFunction,
    reachable_blocks: &HashSet<crate::mir::BasicBlockId>,
    local_reads: &LocalReadInfo,
) -> PrivateCarrierInfo {
    let mut info = PrivateCarrierInfo::default();

    for (bid, block) in &function.blocks {
        if !reachable_blocks.contains(bid) {
            continue;
        }

        for instruction in &block.instructions {
            match instruction {
                crate::mir::MirInstruction::Copy { dst, src } => {
                    info.alias_parents.insert(*dst, *src);
                }
                crate::mir::MirInstruction::RefNew { dst, box_val } => {
                    if local_reads.is_non_escaping_local(*box_val) {
                        info.private_roots.insert(*dst);
                    }
                }
                _ => {}
            }
        }
    }

    for (bid, block) in &function.blocks {
        if !reachable_blocks.contains(bid) {
            continue;
        }

        for instruction in &block.instructions {
            match instruction {
                crate::mir::MirInstruction::Copy { src, .. } => {
                    if let Some(root) = info.resolve_private_root(*src) {
                        if info.escaping_roots.contains(&root) {
                            continue;
                        }
                    }
                }
                crate::mir::MirInstruction::Load { ptr, .. } => {
                    if let Some(root) = info.resolve_private_root(*ptr) {
                        if info.escaping_roots.contains(&root) {
                            continue;
                        }
                    }
                }
                crate::mir::MirInstruction::Store { ptr, value } => {
                    if let Some(root) = info.resolve_private_root(*ptr) {
                        info.load_blocked_roots.insert(root);
                        if info.escaping_roots.contains(&root) {
                            continue;
                        }
                    }
                    if let Some(root) = info.resolve_private_root(*value) {
                        info.escaping_roots.insert(root);
                    }
                }
                _ => {
                    for value in instruction.used_values() {
                        if let Some(root) = info.resolve_private_root(value) {
                            info.escaping_roots.insert(root);
                        }
                    }
                }
            }
        }

        if let Some(term) = &block.terminator {
            for value in term.used_values() {
                if let Some(root) = info.resolve_private_root(value) {
                    info.escaping_roots.insert(root);
                }
            }
        }
    }

    info
}

pub(crate) fn is_removable_effect_sensitive_memory_read_instruction(
    inst: &crate::mir::MirInstruction,
    private_carriers: &PrivateCarrierInfo,
) -> bool {
    matches!(
        inst,
        crate::mir::MirInstruction::Load { ptr, .. } if private_carriers.is_private_carrier(*ptr)
    )
}

pub(crate) fn collect_overwritten_private_stores(
    function: &MirFunction,
    reachable_blocks: &HashSet<crate::mir::BasicBlockId>,
    private_carriers: &PrivateCarrierInfo,
) -> HashSet<(crate::mir::BasicBlockId, usize)> {
    let mut removable = HashSet::new();

    for (bid, block) in &function.blocks {
        if !reachable_blocks.contains(bid) {
            continue;
        }

        let mut pending_store_roots = HashSet::new();
        for (idx, instruction) in block.instructions.iter().enumerate().rev() {
            match instruction {
                crate::mir::MirInstruction::Store { ptr, .. } => {
                    let Some(root) = private_carriers.resolve_private_store_root(*ptr) else {
                        continue;
                    };
                    if pending_store_roots.contains(&root) {
                        removable.insert((*bid, idx));
                    }
                    pending_store_roots.insert(root);
                }
                crate::mir::MirInstruction::Load { ptr, .. } => {
                    if let Some(root) = private_carriers.resolve_private_store_root(*ptr) {
                        pending_store_roots.remove(&root);
                    }
                }
                crate::mir::MirInstruction::Copy { src, .. } => {
                    if let Some(root) = private_carriers.resolve_private_store_root(*src) {
                        if pending_store_roots.contains(&root) {
                            continue;
                        }
                    }
                }
                _ => {
                    for value in instruction.used_values() {
                        if let Some(root) = private_carriers.resolve_private_store_root(value) {
                            pending_store_roots.remove(&root);
                        }
                    }
                }
            }
        }
    }

    removable
}
