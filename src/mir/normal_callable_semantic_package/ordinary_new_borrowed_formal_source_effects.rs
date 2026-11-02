//! Original read-only effect closure shared by storage and returned read loans.
use super::*;

/// Compute the reachable original-method effect closure once per query; an
/// unregistered call, Print, write, escape or unknown effect is never pure.
pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn readonly_call_closure_v1(root: FunctionOwnerIdV1, inventory: &BorrowedIncomingInventoryV1,
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    constructors: Option<&crate::mir::normal_callable_semantic_package::VerifiedInstanceConstructorSemanticBatchV1>,
) -> bool {
    use crate::mir::resolved_semantics::{BodyEffectKindV1 as Effect, BodyStatementShapeV1};
    let mut pending = vec![root]; let mut seen = BTreeSet::new();
    while let Some(owner) = pending.pop() {
        if !seen.insert(owner) { continue; }
        if !inventory.source_registered_owners_v1().contains(&owner)
            || inventory.sum_vetoed_owners_v1().contains(&owner) { return false; }
        let mut check = |input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>| {
                let Some(shape) = input.body_shape() else { return false; };
                if shape.statements().iter().any(|row| matches!(row, BodyStatementShapeV1::Print { .. })) { return false; }
                shape.effects().iter().all(|effect| match effect.kind {
                    Effect::Call => {
                        let mut rows = inventory.source_rows_with_method_v1().filter(|(row, _)|
                            row.call.owner() == owner && row.call.site() == &effect.site);
                        let Some((row, Some(original))) = rows.next() else { return false; };
                        if rows.next().is_some() || !input.function().method_call_source_v1(&effect.site)
                            .is_some_and(|current| std::sync::Arc::ptr_eq(current, original)) { return false; }
                        let Some(reference) = row.source.original_instance_reference_v1() else { return false; };
                        if reference.callee_owner != row.callee || !reference.matches_original_call_v1(input, original) { return false; }
                        pending.push(row.callee); true
                    }
                    Effect::NonLocalControl => shape.statements().iter().any(|row|
                        matches!(row, BodyStatementShapeV1::Return { site, .. } if site.node() == effect.site.node())),
                    _ => false,
                })
            };
        let checked = if let Some(constructors) = constructors {
            crate::mir::normal_callable_semantic_package::original_call_source::with_original_source_input_v1(
                batch, constructors, owner, |_, input| check(input)).map_err(|_| ())
        } else {
            let mut declarations = batch.declarations().filter(|row| row.owner() == owner);
            let Some(declaration) = declarations.next() else { return false; };
            if declarations.next().is_some() { return false; }
            batch.with_lowering_input(declaration.batch_slot(), check).map_err(|_| ())
        };
        if checked != Ok(true) { return false; }
    }
    true
}

impl PreparedBorrowedFormalIngressV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn call_preserves_read_loans_v1(&self,
        input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
        batch: &VerifiedResolvedCallableSemanticBatchV1, call: &OwnedExprSiteV1,
    ) -> bool {
        let mut rows = self.source_incoming.source_rows_with_method_v1().filter(|(row, _)| &row.call == call);
        let Some((row, Some(original))) = rows.next() else { return false; };
        let Some(reference) = row.source.original_instance_reference_v1() else { return false; };
        rows.next().is_none() && input.owner() == call.owner()
            && input.function().method_call_source_v1(call.site()).is_some_and(|same| std::sync::Arc::ptr_eq(same, original))
            && reference.callee_owner == row.callee && reference.matches_original_call_v1(input, original)
            && readonly_call_closure_v1(row.callee, &self.source_incoming, batch, None)
    }
}
