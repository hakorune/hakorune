use super::*;
use std::collections::HashSet;
use std::sync::Arc;

impl MirInterpreter {
    /// Phase 287: Release strong references for all values (including SSA aliases)
    /// This is called by ReleaseStrong instruction for variable overwrite semantics.
    pub(super) fn release_strong_refs(&mut self, values: &[ValueId]) {
        let mut arc_ptrs: HashSet<*const dyn NyashBox> = HashSet::new();

        for value_id in values {
            if let Some(VMValue::BoxRef(arc)) = self.reg_peek_resolved(*value_id) {
                arc_ptrs.insert(Arc::as_ptr(arc));
            }
        }

        // Only BoxRef values participate in "strong ref" release.
        // Do not remove immediate values (Integer/Bool/String/etc): they have no strong refs,
        // and removing them can create use-after-release crashes.
        for value_id in values {
            if matches!(self.reg_peek_resolved(*value_id), Some(VMValue::BoxRef(_))) {
                let _ = self.take_reg(*value_id);
            }
        }

        if arc_ptrs.is_empty() {
            return;
        }

        let mut to_remove: HashSet<ValueId> = HashSet::new();

        for (idx, value) in self.reg_fast_slots.iter().enumerate() {
            if let Some(VMValue::BoxRef(arc)) = value {
                if arc_ptrs.contains(&Arc::as_ptr(arc)) {
                    to_remove.insert(ValueId(idx as u32));
                }
            }
        }

        for (value_id, value) in self.regs.iter() {
            if let VMValue::BoxRef(arc) = value {
                if arc_ptrs.contains(&Arc::as_ptr(arc)) {
                    to_remove.insert(*value_id);
                }
            }
        }

        for value_id in to_remove {
            let _ = self.take_reg(value_id);
        }
    }
}
