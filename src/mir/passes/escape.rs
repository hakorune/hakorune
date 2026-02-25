//! Escape Analysis (VM-only footing)
//! Conservative analysis to elide write/read barriers for definitely non-escaping boxes.
//! Enabled for VM backend as a staging step before LLVM.

use crate::mir::{MirFunction, MirInstruction, MirModule, ValueId};
use std::collections::{HashMap, HashSet};

/// Run a conservative escape analysis and remove Barrier(Read/Write) for non-escaping boxes.
/// Returns the number of barriers removed.
pub fn escape_elide_barriers_vm(module: &mut MirModule) -> usize {
    let mut removed = 0usize;
    let mut analysis: HashMap<String, EscapeInfo> = HashMap::new();

    // 1) Analyze each function
    for (name, func) in module.functions.iter() {
        analysis.insert(name.clone(), analyze_function(func));
    }

    // 2) Apply in-place edits per function
    for (name, info) in analysis.into_iter() {
        if let Some(func) = module.functions.get_mut(&name) {
            removed += elide_barriers_in_function(func, &info);
        }
    }
    removed
}

#[derive(Default)]
struct EscapeInfo {
    local_boxes: HashSet<ValueId>,
    escaping: HashSet<ValueId>,
}

impl EscapeInfo {
    fn is_non_escaping(&self, v: &ValueId) -> bool {
        self.local_boxes.contains(v) && !self.escaping.contains(v)
    }
}

fn analyze_function(func: &MirFunction) -> EscapeInfo {
    let mut info = EscapeInfo::default();
    // Collect local boxes: results of NewBox in this function
    for block in func.blocks.values() {
        for sp in block.iter_spanned() {
            if let MirInstruction::NewBox { dst, .. } = sp.inst {
                info.local_boxes.insert(*dst);
            }
        }
        if let Some(term) = &block.terminator {
            if let MirInstruction::NewBox { dst, .. } = term {
                info.local_boxes.insert(*dst);
            }
        }
    }
    // Conservative escape marking
    for block in func.blocks.values() {
        for sp in block.all_spanned_instructions() {
            match sp.inst {
                MirInstruction::Return { value: Some(v) } => {
                    if info.local_boxes.contains(v) {
                        info.escaping.insert(*v);
                    }
                }
                MirInstruction::Call { args, .. } => {
                    for a in args {
                        if info.local_boxes.contains(a) {
                            info.escaping.insert(*a);
                        }
                    }
                }
                MirInstruction::Store { value, .. } => {
                    if info.local_boxes.contains(value) {
                        info.escaping.insert(*value);
                    }
                }
                _ => {}
            }
        }
    }
    info
}

fn elide_barriers_in_function(func: &mut MirFunction, info: &EscapeInfo) -> usize {
    let mut removed = 0usize;
    for block in func.blocks.values_mut() {
        block.instructions.retain(|ins| match ins {
            MirInstruction::Barrier { ptr, .. } if info.is_non_escaping(ptr) => {
                removed += 1;
                false
            }
            _ => true,
        });

        if let Some(MirInstruction::Barrier { ptr, .. }) = block.terminator.as_ref() {
            if info.is_non_escaping(ptr) {
                block.terminator = None;
                block.terminator_span = None;
                removed += 1;
            }
        }
    }
    if removed > 0 {
        func.update_cfg();
    }
    removed
}
