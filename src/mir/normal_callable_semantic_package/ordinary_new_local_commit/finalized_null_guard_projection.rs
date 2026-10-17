//! One finished projection algorithm for the existing entry null-guard observer.
use super::*;
use crate::mir::resolved_semantics::SourceNodeSiteV1;
pub(super) type LocatedBinding = (Binding, (BasicBlockId, usize));
pub(super) type GuardBindings = (LocatedBinding, LocatedBinding, BasicBlockId, Vec<LocatedBinding>);

pub(super) fn project(ledger: &OrdinaryNewClaimLedgerV1, owner: FunctionOwnerIdV1,
    guard: &SourceNodeSiteV1, symbol: &str, function: &MirFunction,
    projection: &physical_boundary::FinishedBindings,
) -> Result<GuardBindings, String> {
    let mut result = None;
    ledger.with_borrowed_entry_null_guard_original_v1(owner, guard, |compare, operands, condition, branch, originals| {
        for relation in operands.iter().chain(std::iter::once(condition.relation())) {
            relation.require_completed_v1()?;
            relation.verify_projected(function, &mut |row| projection.relation_binding(function, row))?;
        }
        let compare = projection.relation_binding(function, compare)?
            .ok_or_else(|| freeze("finished-borrowed-write/guard-removed"))?;
        let branch = projection.binding(branch.0, &branch.1)?
            .ok_or_else(|| freeze("finished-borrowed-write/branch-removed"))?;
        let nonnull = projection.destination(condition.arms().1)
            .ok_or_else(|| freeze("finished-borrowed-write/nonnull-removed"))?;
        let mut dependencies = originals.to_vec();
        dependencies.extend_from_slice(condition.bindings());
        condition.relation().append_observed_bindings_v1(&mut dependencies);
        let mut finished_dependencies = Vec::new();
        for original in dependencies {
            let mut resolved = original.clone();
            for relation in operands.iter().chain(std::iter::once(condition.relation())) {
                if let Some(binding) = relation.resolve_observed_binding_v1(&original, true)? { resolved = binding; }
            }
            let finished = projection.relation_binding(function, &resolved)?
                .ok_or_else(|| freeze("finished-borrowed-write/guard-dependency-removed"))?;
            if !finished_dependencies.iter().any(|(prior, _)| prior == &finished) {
                let position = find_finished_producer(symbol, &finished, function)?;
                finished_dependencies.push((finished, position));
            }
        }
        let guard = ((compare.clone(), find_finished_producer(symbol, &compare, function)?),
            (branch.clone(), find_finished_producer(symbol, &branch, function)?), nonnull, finished_dependencies);
        result = Some(guard);
        Ok(())
    })?;
    result.ok_or_else(|| freeze("finished-borrowed-write/guard-projection-missing"))
}
