//! Dead Code Elimination (pure instruction DCE)
//!
//! Extracted from the monolithic optimizer to enable modular pass composition.

#[path = "dce/control_anchor.rs"]
mod control_anchor;
#[path = "dce/elimination.rs"]
mod elimination;
#[path = "dce/local_fields.rs"]
pub(crate) mod local_fields;
#[path = "dce/memory.rs"]
pub(crate) mod memory;
#[cfg(test)]
#[path = "dce/tests/mod.rs"]
mod tests;

use crate::mir::{MirFunction, MirModule, ValueId};
use std::collections::HashSet;

#[derive(Clone, Default)]
pub(super) struct LiveValueSet {
    bits: Vec<bool>,
    values: Vec<ValueId>,
}

#[derive(Default)]
struct UsesByDst {
    entries: Vec<Option<Vec<ValueId>>>,
}

impl UsesByDst {
    fn insert(&mut self, dst: ValueId, uses: Vec<ValueId>) {
        let index = dst.to_usize();
        if index >= self.entries.len() {
            self.entries.resize(index + 1, None);
        }
        self.entries[index] = Some(uses);
    }

    fn get(&self, value: ValueId) -> Option<&[ValueId]> {
        self.entries
            .get(value.to_usize())
            .and_then(|entry| entry.as_deref())
    }
}

impl LiveValueSet {
    fn insert(&mut self, value: ValueId) -> bool {
        let index = value.to_usize();
        if index >= self.bits.len() {
            self.bits.resize(index + 1, false);
        }
        if self.bits[index] {
            return false;
        }
        self.bits[index] = true;
        self.values.push(value);
        true
    }

    fn contains(&self, value: &ValueId) -> bool {
        self.bits.get(value.to_usize()).copied().unwrap_or(false)
    }

    fn extend(&mut self, values: impl IntoIterator<Item = ValueId>) {
        for value in values {
            self.insert(value);
        }
    }

    fn iter(&self) -> impl Iterator<Item = &ValueId> {
        self.values.iter()
    }
}

/// Eliminate dead code (unused results of pure instructions) across the module
/// and prune unreachable blocks as structural CFG cleanup.
///
/// This pass also removes pure no-dst calls plus dead field reads and writes on
/// definitely non-escaping local boxes when they are otherwise unused.
///
/// Returns the number of eliminated instructions.
pub fn eliminate_dead_code(module: &mut MirModule) -> usize {
    let mut eliminated_total = 0usize;
    for (_func_name, func) in &mut module.functions {
        eliminated_total += elimination::eliminate_dead_code_in_function(func);
    }
    eliminated_total
}

/// Eliminate dead code in a single function.
pub(crate) fn eliminate_dead_code_in_function(function: &mut MirFunction) -> usize {
    elimination::eliminate_dead_code_in_function(function)
}

fn propagate_used_values(
    function: &MirFunction,
    reachable_blocks: &HashSet<crate::mir::BasicBlockId>,
    used_values: &mut LiveValueSet,
) {
    let mut uses_by_dst = UsesByDst::default();
    for (bid, block) in &function.blocks {
        if !reachable_blocks.contains(bid) {
            continue;
        }
        for instruction in &block.instructions {
            let Some(dst) = instruction.dst_value() else {
                continue;
            };
            uses_by_dst.insert(dst, instruction.used_values());
        }
    }

    let mut worklist: Vec<_> = used_values.iter().copied().collect();
    while let Some(value) = worklist.pop() {
        let Some(used_by_definition) = uses_by_dst.get(value) else {
            continue;
        };
        for used in used_by_definition {
            if used_values.insert(*used) {
                worklist.push(*used);
            }
        }
    }
}

fn is_removable_no_dst_pure_instruction(inst: &crate::mir::MirInstruction) -> bool {
    matches!(
        inst,
        crate::mir::MirInstruction::Safepoint | crate::mir::MirInstruction::Call { dst: None, .. }
    )
}
