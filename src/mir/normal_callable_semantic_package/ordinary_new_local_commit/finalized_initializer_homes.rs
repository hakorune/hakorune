//! Exact Text lifecycle events lent by their original binding and Return owners.
use super::*;

impl FinalizedRootSourceHandoffV1 {
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn initializer_lifecycle_owners_v1(&self) -> std::collections::BTreeSet<FunctionOwnerIdV1> {
        let mut owners: std::collections::BTreeSet<_> = self.local_calls.iter().filter_map(|(owner, roots)|
            RootLifecyclePrefixBindingGroupV1::snapshot_tree_v2(roots).iter()
                .any(|group| group.text_residence_v1().is_some() || group.dynamic_carrier_residence_v1().is_some())
                .then_some(*owner)).collect();
        owners.extend(self.ledger.terminal_relation_index.iter().filter_map(|(owner, terminals)|
            ((*owner == self.owner || self.ledger.child_physical_validation.borrow().contains_key(owner))
                && terminals.values().any(|relation| matches!(relation, TerminalRelationV1::Value(value)
                    if matches!(value.returned(), TerminalReturnedSourceV1::TextExpression(_)))))
                .then_some(*owner)));
        owners
    }

    pub(in crate::mir) fn finished_initializer_installs_v1(
        &self, module: &crate::mir::MirModule, function: &MirFunction,
    ) -> Result<Vec<Binding>, String> {
        if !module.functions.get(&function.signature.name).is_some_and(|stored| std::ptr::eq(stored, function)) {
            return Err(freeze("finished-initializer-home/function-membership"));
        }
        let context = CallValidationContextV1::Finalized(self);
        // Presence is navigation only; validate a tree only after selecting
        // its Text owner and the exact function's finished projection.
        let lifecycle_owners = self.initializer_lifecycle_owners_v1();
        if !lifecycle_owners.is_empty() { self.validate_finalized_root_cleanup_v1(module)?; }
        let mut result = Vec::new();
        for owner in lifecycle_owners {
            self.ledger.with_finished_projection(owner, |symbol, projection| {
                if symbol != function.signature.name { return Ok(()); }
                context.with_local_call_group_tree_v2(owner, |groups| {
                    for group in groups {
                        if let Some(packet) = group.dynamic_carrier_residence_v1() {
                            if packet.owner() != owner {
                                return Err(freeze("finished-initializer-home/source-owner"));
                            }
                            group.validate_recorded_in_context(CallValidationContextV1::Finalized(self), function, Some(projection))?;
                            self.ledger.validate_dynamic_carrier_residence_home_v1(
                                &packet,
                                CallValidationContextV1::Finalized(self),
                            )?;
                        }
                        let Some(packet) = group.text_residence_v1() else { continue };
                        if packet.owner() != owner { return Err(freeze("finished-initializer-home/source-owner")); }
                        group.validate_recorded_in_context(CallValidationContextV1::Finalized(self), function, Some(projection))?;
                        self.ledger.validate_text_residence_home_v1(&packet, CallValidationContextV1::Finalized(self))?;
                        let original = packet.install_binding().ok_or_else(|| freeze("finished-initializer-home/install-missing"))?;
                        let finished = projection.binding(original.0, &original.1)?
                            .ok_or_else(|| freeze("finished-initializer-home/install-removed"))?;
                        let count = function.blocks.iter().flat_map(|(block, body)|
                            body.all_instructions().map(move |row| (*block, row)))
                            .filter(|(block, row)| *block == finished.0 && **row == finished.1).count();
                        if count != 1 || projection.recorded().iter().filter(|row| **row == finished).count() != 1
                            || result.contains(&finished) {
                            return Err(freeze("finished-initializer-home/install-membership"));
                        }
                        result.push(finished);
                    }
                    Ok(())
                })?;
                let exits = self.ledger.root_exits.borrow();
                for ((row_owner, exit), progress) in exits.iter() {
                    if *row_owner != owner { continue; }
                    let RootHomeExitProgress::Emitted { order, entry, bindings, .. } = progress else { continue };
                    let events: Vec<_> = order.text_expression_install_v1().into_iter()
                        .chain(order.text_return_hold_binding()).collect();
                    if events.is_empty() { continue; }
                    let site = exit.explicit_site().ok_or_else(|| freeze("expression/residence-original-pair"))?;
                    super::finalized_root_cleanup::validate_finished_cleanup_entry(context, owner, site,
                        function, projection, order, entry, bindings, Some(projection))?;
                    for original in events {
                    let finished = projection.binding(original.0, &original.1)?
                        .ok_or_else(|| freeze("root-exit-binding-drift"))?;
                    let count = function.blocks.iter().flat_map(|(block, body)|
                        body.all_instructions().map(move |row| (*block, row)))
                        .filter(|(block, row)| *block == finished.0 && **row == finished.1).count();
                    if count != 1 || projection.recorded().iter().filter(|row| **row == finished).count() != 1
                        || result.contains(&finished) {
                        return Err(freeze("finished-initializer-home/install-membership"));
                    }
                    result.push(finished);
                    }

                }
                Ok(())
            })?;
        }
        Ok(result)
    }
}
