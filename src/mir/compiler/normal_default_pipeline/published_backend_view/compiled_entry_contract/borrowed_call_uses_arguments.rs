//! Original call actuals joined to the caller's sealed entry, not metadata.
use super::*;

impl BorrowedCallUses {
    pub(in crate::mir::compiler::normal_default_pipeline::published_backend_view::compiled_entry_contract::borrowed_call_incoming) fn call(
        &mut self,
        caller: &MirFunction,
        coordinate: Coordinate,
        actuals: &[PreparedBorrowedFormalActualV1],
        copies: &[(Binding, Coordinate)],
        caller_entry: &[(u32, BindingRefV1, ValueId)],
    ) -> Result<(), String> {
        let state = self
            .functions
            .entry(caller.signature.name.clone())
            .or_default();
        for (original, finished) in copies {
            state.copy(original, Some(*finished))?;
        }
        for actual in actuals {
            if let Some((_, formal)) = actual.source.caller_formal_origin_v1() {
                // DeclaredObject also exists outside the selected borrowed
                // transport. Only the original finished caller entry can
                // connect it to this operand closure; visitation order cannot.
                if matches!(actual.source, BorrowedFormalActualSourceV1::DeclaredFormal { .. })
                    && !caller_entry.iter().any(|(_, root, _)| *root == formal)
                {
                    continue;
                }
                let key = (coordinate.0, coordinate.1, actual.ordinal as usize);
                if let Some(previous) = state.arguments.insert(key, formal) {
                    if previous != formal {
                        return Err(fault("borrowed-use/conflicting-argument"));
                    }
                }
            }
        }
        Ok(())
    }
}
