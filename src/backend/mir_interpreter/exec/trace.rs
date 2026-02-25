use super::super::MirInterpreter;
use crate::mir::{BasicBlockId, MirInstruction};

impl MirInterpreter {
    pub(crate) fn record_step_trace(
        &mut self,
        bb: BasicBlockId,
        inst_idx: Option<usize>,
        inst: Option<&MirInstruction>,
    ) {
        if !self.joinir_debug_enabled {
            return;
        }
        let inst_str = inst.map(|i| {
            let s = format!("{:?}", i);
            if s.len() > 120 {
                format!("{}...", &s[..117])
            } else {
                s
            }
        });
        if self.recent_steps.len() == super::super::VM_RECENT_STEP_LIMIT {
            self.recent_steps.pop_front();
        }
        self.recent_steps.push_back(super::super::StepTrace {
            bb,
            inst_idx,
            inst: inst_str,
        });
    }
}
