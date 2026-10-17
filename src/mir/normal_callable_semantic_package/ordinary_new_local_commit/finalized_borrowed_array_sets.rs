//! Original Set occurrences through the same finished-binding correspondence.
use super::*;
use super::null_guard::{LocatedBinding, GuardBindings};
use crate::mir::normal_callable_semantic_package::BorrowedArraySetSourceLoanV1;
use std::collections::BTreeSet;

impl FinalizedRootSourceHandoffV1 {
    pub(in crate::mir) fn with_finished_borrowed_array_sets_v1(
        &self, owner: FunctionOwnerIdV1, function: &MirFunction,
        mut visit: impl FnMut(&BorrowedArraySetSourceLoanV1, &LocatedBinding,
            &[LocatedBinding; 2], &[LocatedBinding], &GuardBindings) -> Result<(), String>,
    ) -> Result<(), String> {
        self.borrowed_ordinary_entry_source_for_function_v1(owner, function)?;
        self.ledger.with_finished_projection(owner, |symbol, projection| {
            if function.signature.name != symbol { return Err(freeze("finished-borrowed-set/function")); }
            let mut sites = BTreeSet::new();
            let mut coordinates = BTreeSet::new();
            self.ledger.with_borrowed_array_set_originals_v1(owner, |source, write, reads, operands| {
                source.validate_source(&self.ledger)?;
                if source.call().owner() != owner || !sites.insert(source.call().clone()) {
                    return Err(freeze("finished-borrowed-set/source-membership"));
                }
                let write = projection.binding(write.0, &write.1)?
                    .ok_or_else(|| freeze("finished-borrowed-set/write-removed"))?;
                let coordinate = find_finished_producer(symbol, &write, function)?;
                if !coordinates.insert(coordinate)
                    || projection.recorded().iter().filter(|row| **row == write).count() != 1 {
                    return Err(freeze("finished-borrowed-set/write-membership"));
                }
                let mut dependencies = Vec::new();
                for operand in operands {
                    operand.require_completed_v1()?;
                    operand.verify_projected(function, &mut |row| projection.relation_binding(function, row))?;
                    let mut originals = Vec::new();
                    operand.append_observed_bindings_v1(&mut originals);
                    for original in originals {
                        let original = operand.resolve_observed_binding_v1(&original, true)?.unwrap_or(original);
                        let Some(finished) = projection.relation_binding(function, &original)? else { continue };
                        if !dependencies.iter().any(|(prior, _)| prior == &finished) {
                            let position = find_finished_producer(symbol, &finished, function)?;
                            dependencies.push((finished, position));
                        }
                    }
                }
                let mut finished_reads = Vec::new();
                for read in reads {
                    let finished = projection.relation_binding(function, read)?
                        .ok_or_else(|| freeze("finished-borrowed-set/read-removed"))?;
                    let position = find_finished_producer(symbol, &finished, function)?;
                    finished_reads.push((finished, position));
                }
                let finished_reads: [LocatedBinding; 2] = finished_reads.try_into()
                    .map_err(|_| freeze("finished-borrowed-set/read-cardinality"))?;
                let guard = null_guard::project(&self.ledger, owner, source.null_guard(), symbol, function, projection)?;
                visit(source, &(write, coordinate), &finished_reads, &dependencies, &guard)
            })
        })
    }
}
