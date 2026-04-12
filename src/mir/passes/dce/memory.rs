use super::local_fields::LocalReadInfo;
use crate::mir::{resolve_value_origin_from_parent_map, MirFunction, ParentMap, ValueId};
use std::collections::HashSet;

#[derive(Default)]
pub(super) struct PrivateCarrierInfo {
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

    pub(super) fn is_private_carrier(&self, value: ValueId) -> bool {
        self.resolve_private_root(value).is_some_and(|root| {
            !self.escaping_roots.contains(&root) && !self.load_blocked_roots.contains(&root)
        })
    }
}

pub(super) fn analyze_private_carriers(
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

pub(super) fn is_removable_effect_sensitive_memory_read_instruction(
    inst: &crate::mir::MirInstruction,
    private_carriers: &PrivateCarrierInfo,
) -> bool {
    matches!(
        inst,
        crate::mir::MirInstruction::Load { ptr, .. } if private_carriers.is_private_carrier(*ptr)
    )
}
