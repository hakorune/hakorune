//! Original borrowed writes through the existing finished-binding owner.
//! This visitor lends correspondence, never permission for generic FieldSet.
use super::*;
use crate::mir::normal_callable_semantic_package::BorrowedFieldWriteSourceLoanV1;
use std::collections::BTreeSet;
use super::null_guard::{LocatedBinding, GuardBindings};
#[path = "finalized_owned_field_writes.rs"]
mod owned;

impl FinalizedRootSourceHandoffV1 {
    /// Enumerate only original retained groups of this selected source entry.
    /// Both final consumers must still check source guards and operand roles.
    pub(in crate::mir) fn with_finished_borrowed_field_writes_v1(
        &self, owner: FunctionOwnerIdV1, function: &MirFunction,
        mut visit: impl FnMut(&BorrowedFieldWriteSourceLoanV1,
            &LocatedBinding, &[Vec<LocatedBinding>; 2], &GuardBindings) -> Result<(), String>,
    ) -> Result<(), String> {
        self.borrowed_ordinary_entry_source_for_function_v1(owner, function)?;
        self.ledger.with_finished_projection(owner, |symbol, projection| {
            if function.signature.name != symbol {
                return Err(freeze("finished-borrowed-write/function"));
            }
            let mut sites = BTreeSet::new();
            let mut coordinates = BTreeSet::new();
            for group in self.local_calls.get(&owner).into_iter().flatten() {
                let Some(packet) = group.borrowed_field_write() else { continue };
                group.require_sealed_condition_v1()?;
                let Some(source) = packet.source().borrowed() else { continue };
                if source.frame().source().owner() != owner || group.site() != source.value().2
                    || !sites.insert(group.site().clone())
                { return Err(freeze("finished-borrowed-write/source-membership")); }
                group.validate_recorded_in_context(CallValidationContextV1::Finalized(self),
                    function, Some(projection))?;
                let original = packet.invoke_binding();
                let invoke = projection.binding(original.0, &original.1)?
                    .ok_or_else(|| freeze("finished-borrowed-write/invoke-removed"))?;
                let coordinate = find_finished_producer(symbol, &invoke, function)?;
                if !coordinates.insert(coordinate)
                    || projection.recorded().iter().filter(|row| **row == invoke).count() != 1
                { return Err(freeze("finished-borrowed-write/invoke-membership")); }
                let mut pins = [Vec::new(), Vec::new()];
                for (index, pin) in packet.pins().iter().enumerate() {
                    pin.require_completed_v1()?;
                    let mut originals = Vec::new();
                    pin.append_observed_bindings_v1(&mut originals);
                    for original in originals {
                        let original = pin.resolve_observed_binding_v1(&original, true)?.unwrap_or(original);
                        let Some(finished) = projection.relation_binding(function, &original)? else { continue };
                        if !matches!(finished.1, MirInstruction::Copy { .. }) { continue; }
                        let position = find_finished_producer(symbol, &finished, function)?;
                        if !pins[index].iter().any(|(prior, _)| prior == &finished) {
                            pins[index].push((finished, position));
                        }
                    }
                }
                let guard = super::null_guard::project(&self.ledger, owner, source.null_guard_v1(),
                    symbol, function, projection)?;
                visit(source, &(invoke.clone(), coordinate), &pins, &guard)?;
            }
            Ok(())
        })
    }
}
