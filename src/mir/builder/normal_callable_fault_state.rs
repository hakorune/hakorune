//! Callable source entry permission over the shared physical frame owner.

use super::CallableSemanticLoweringState;
use crate::mir::{MirBuilder, MirFunction, ValueId};

impl CallableSemanticLoweringState {
    /// Only the source-identity-checked App Main root entry calls this.
    pub(in crate::mir::builder) fn select_root_fault_frame(&mut self) -> Result<(), String> {
        self.fault_frame
            .as_mut()
            .ok_or_else(|| freeze("state-transferred"))?
            .select_root()
    }

    pub(in crate::mir::builder) fn borrow_fault_frame(
        &mut self,
        builder: &mut MirBuilder,
    ) -> Result<ValueId, String> {
        self.fault_frame
            .as_mut()
            .ok_or_else(|| freeze("state-transferred"))?
            .materialize(builder)
    }

    pub(in crate::mir::builder) fn validate_fault_frame(
        &self,
        function: &MirFunction,
    ) -> Result<(), String> {
        self.fault_frame
            .as_ref()
            .ok_or_else(|| freeze("state-transferred"))?
            .validate(function)
    }
}

fn freeze(reason: &str) -> String {
    format!("[freeze:contract][callable-fault-frame/{reason}]")
}
