//! Source-owned initializer lifecycle and finalized New relations.
use super::*;

impl FinalizedRootSourceHandoffV1 {
    /// Lend only the finished physical Shared initializers retained by the
    /// exact source group and its existing cleanup proof.
    pub(in crate::mir) fn finished_shared_initializers_v1(
        &self, module: &crate::mir::MirModule, function: &MirFunction,
    ) -> Result<Vec<(BasicBlockId, MirInstruction)>, String> {
        if !module.functions.get(&function.signature.name).is_some_and(|stored| std::ptr::eq(stored, function)) {
            return Err(freeze("shared-initializer/function-membership"));
        }
        // Source remains authoritative even if both a physical instruction
        // and its packet were accidentally removed together.
        for completion in self.ledger.completion_index.values().filter_map(|row| row.as_ref().ok())
            .chain(self.ledger.root_completion.iter().filter_map(|row| row.as_ref().ok())) {
            if completion.owner() != self.owner && !self.ledger.child_physical_validation.borrow().contains_key(&completion.owner()) {
                continue;
            }
            let Some(flow) = completion.cleanup().root_flow() else { continue };
            let sources: Vec<_> = flow.expression_plans().iter().filter_map(|plan|
                plan.shared_initializer_source_v1()).collect();
            if sources.is_empty() { continue; }
            let groups = RootLifecyclePrefixBindingGroupV1::snapshot_tree_v2(
                self.local_calls.get(&completion.owner()).map(Vec::as_slice).unwrap_or(&[]));
            let packets: Vec<_> = groups.iter().filter_map(|group| group.shared_initializer_v1()).collect();
            if packets.len() != sources.len() || sources.iter().any(|source|
                packets.iter().filter(|packet| std::rc::Rc::ptr_eq(&packet.source, source)).count() != 1) {
                return Err(freeze("shared-initializer/source-coverage"));
            }
        }
        let mut result = Vec::new();
        for owner in self.local_calls.keys() {
            self.ledger.with_finished_projection(*owner, |symbol, projection| {
                if symbol != function.signature.name { return Ok(()); }
                CallValidationContextV1::Finalized(self).with_local_call_group_tree_v2(*owner, |groups| {
                for group in groups {
                    if group.shared_initializer_v1().is_none() { continue; }
                    group.check_physical_bindings_v1(function, Some(projection))?;
                    group.validate_recorded_in_context(CallValidationContextV1::Finalized(self), function, Some(projection))?;
                    for binding in projection.bindings(group.bindings())? {
                        if !matches!(binding.1, MirInstruction::Invoke {
                            operation: crate::mir::instruction::InvokeOperation::Shared(crate::mir::instruction::SharedInvokeOperation::New { .. } | crate::mir::instruction::SharedInvokeOperation::Clone { .. }), .. }) { continue; }
                        if !projection.recorded().contains(&binding)
                            || !physical_boundary::check_binding(function, None, binding.0, &binding.1)?
                            || result.contains(&binding) { return Err(freeze("shared-initializer/finished-membership")); }
                        result.push(binding);
                    }
                }
                Ok(())
                })
            })?;
        }
        Ok(result)
    }

    /// Exact scalar and receiver lanes from the original selected callable;
    /// physical types or names never decide whether a parameter is owned.
    pub(crate) fn move_parameter_kinds_v1(
        &self, owner: FunctionOwnerIdV1,
        key: &hakorune_mir_defs::CanonicalSameModuleCallableKeyV1,
        receiver_object: Option<CanonicalObjectIdV1>, count: usize,
    ) -> Result<Vec<crate::mir::ownership_ssa::MirOwnershipKindV1>, String> {
        use crate::mir::ownership_ssa::MirOwnershipKindV1 as Kind;
        use crate::mir::callable_parameter_contract::CallableParameterContractKindV1 as Contract;
        use crate::mir::normal_callable_semantic_package::physical_signature::PhysicalCallableLaneRoleV1 as Role;
        let unsupported = || "[ownership/move-abi-not-supported]".to_owned();
        let (selected, signatures) = self.ledger.original_instance_input_issuance.as_ref().ok_or_else(unsupported)?;
        let selected_key = crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(key.clone());
        let slot = selected.batch_slot(&selected_key).ok_or_else(unsupported)?;
        let signature = signatures.row(slot).ok_or_else(unsupported)?;
        let contracts = self.ledger.original_parameter_contracts.as_deref().ok_or_else(unsupported)?;
        let mut rows = contracts.iter().filter(|row| row.batch_slot == slot);
        let row = rows.next().ok_or_else(unsupported)?;
        if rows.next().is_some() || row.owner != owner || signature.owner() != owner || row.mode != signature.mode()
            || !selected.identity_for_batch_slot(slot).is_some_and(|identity| identity.same_as(signature.identity()))
            || signature.lanes().len() != count || signature.physical_callable_lane_count() as usize != count {
            return Err(unsupported());
        }
        let mut kinds = Vec::with_capacity(count);
        let mut seen = std::collections::BTreeSet::new();
        let mut receivers = 0;
        for (index, lane) in signature.lanes().iter().enumerate() {
            if lane.index() as usize != index || lane.binding().owner() != owner { return Err(unsupported()); }
            match lane.role() {
                Role::InstanceReceiver if index == 0 && receiver_object.is_some() && lane.logical_ordinal().is_none() => {
                    receivers += 1;
                    kinds.push(Kind::Borrowed);
                }
                Role::OrdinaryScalar => {
                    let ordinal = lane.logical_ordinal().ok_or_else(unsupported)?;
                    let mut matching = row.parameters.iter().filter(|p| p.ordinal == ordinal && p.binding == lane.binding());
                    let parameter = matching.next().ok_or_else(unsupported)?;
                    if matching.next().is_some() || !seen.insert(ordinal)
                        || !matches!(&parameter.kind, Contract::ExactTrivial(abi) if abi.is_i64()) { return Err(unsupported()); }
                    kinds.push(Kind::None);
                }
                _ => return Err(unsupported()),
            }
        }
        if receivers != signature.receiver_lane_count() || seen.len() != row.parameters.len() { return Err(unsupported()); }
        Ok(kinds)
    }
    pub(crate) fn move_ownership_owners_v1(&self) -> std::collections::BTreeSet<FunctionOwnerIdV1> {
        self.ledger.completion_index.values().filter_map(|row| row.as_ref().ok())
            .chain(self.ledger.root_completion.iter().filter_map(|row| row.as_ref().ok()))
            .filter(|completion| completion.cleanup().root_flow().is_some_and(|flow|
                flow.expression_plans().iter().any(|plan| plan.move_initializer_v1().is_some()
                    || plan.shared_initializer_source_v1().is_some())))
            .map(|completion| completion.owner()).collect()
    }
    /// Lend the finished, source-issued New/local relations only when this
    /// exact function contains a checked move. These are handle aliases,
    /// never extra ownership tokens or a representation of move itself.
    pub(crate) fn move_ownership_aliases_v1(
        &self, owner: FunctionOwnerIdV1, function: &MirFunction,
    ) -> Result<Option<(std::collections::BTreeMap<ValueId, ValueId>, std::collections::BTreeSet<ValueId>)>, String> {
        let Some(flow) = self.ledger.completion_for_owner(owner).and_then(|c| c.cleanup().root_flow()) else {
            return Ok(None);
        };
        let moves: Vec<_> = flow.expression_plans().iter().filter_map(|p| p.move_initializer_v1()).collect();
        let shared: Vec<_> = flow.expression_plans().iter().filter_map(|p| p.shared_move_initializer_v1()).collect();
        let clones: Vec<_> = flow.expression_plans().iter().filter_map(|p| p.shared_clone_initializer_v1()).collect();
        let fresh: Vec<_> = flow.expression_plans().iter().filter_map(|p| p.shared_fresh_initializer_v1()).collect();
        if moves.is_empty() && shared.is_empty() && clones.is_empty() && fresh.is_empty() { return Ok(None); }
        self.ledger.with_finished_projection(owner, |symbol, projection| {
            if symbol != function.signature.name { return Err(freeze("move-verifier/function-drift")); }
            let commits = self.ledger.local_commits.borrow();
            for (_, _, acquisition) in &moves {
                if !matches!(commits.get(*acquisition), Some(LocalCommitV1::Ordinary(_) | LocalCommitV1::Shared(_))) {
                    return Err(freeze("move-verifier/acquisition-missing"));
                }
            }
            for (_, _, acquisition, _) in &shared {
                if !matches!(commits.get(*acquisition), Some(LocalCommitV1::Ordinary(_))) {
                    return Err(freeze("shared-verifier/acquisition-missing"));
                }
            }
            for (_, _, acquisition, _) in &clones {
                if !matches!(commits.get(*acquisition), Some(LocalCommitV1::Shared(_))) {
                    return Err(freeze("shared-verifier/handle-acquisition-missing"));
                }
            }
            for (_, site, _) in &fresh {
                if !matches!(commits.get(*site), Some(LocalCommitV1::Result(NewResultCommitV1 {
                    emission: NewEmissionProgress::Emitted { progress: EmittedLocalProgress::Checked { .. }, .. }, ..
                }))) {
                    return Err(freeze("shared-verifier/fresh-acquisition-missing"));
                }
            }
            let mut aliases = std::collections::BTreeMap::new();
            let mut initially_published = std::collections::BTreeSet::new();
            for (_, site, _) in &fresh {
                let Some(LocalCommitV1::Result(row)) = commits.get(*site) else { return Err(freeze("shared-verifier/fresh-acquisition-missing")); };
                if row.birth_target.is_none() && row.birth_abi.is_none() {
                    let NewEmissionProgress::Emitted { result, .. } = &row.emission else { return Err(freeze("shared-verifier/fresh-acquisition-missing")); };
                    initially_published.insert(*result);
                }
            }
            for row in commits.values().filter(|row| row.owner() == owner) {
                let LocalCommitV1::Ordinary(row) = row else { continue };
                let NewEmissionProgress::Emitted { result, progress: EmittedLocalProgress::Checked { local }, .. } = &row.emission else {
                    return Err(freeze("move-verifier/acquisition-unfinished"));
                };
                if row.birth_target.is_none() && row.birth_abi.is_none() {
                    initially_published.insert(*result);
                }
                if let Some(copy) = projection.source_local_copy_binding_v1(function, *local, *result)? {
                    let (_, actual) = projection.relation_binding(function, &copy)?
                        .ok_or_else(|| freeze("move-verifier/source-copy-omitted"))?;
                    if !matches!(actual, MirInstruction::Copy { dst, src } if dst == *local && src == *result)
                        || aliases.insert(*local, *result).is_some() {
                        return Err(freeze("move-verifier/source-copy-drift"));
                    }
                }
            }
            Ok(Some((aliases, initially_published)))
        })
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn initializer_lifecycle_owners_v1(&self) -> std::collections::BTreeSet<FunctionOwnerIdV1> {
        let mut owners: std::collections::BTreeSet<_> = self.local_calls.iter().filter_map(|(owner, roots)|
            RootLifecyclePrefixBindingGroupV1::snapshot_tree_v2(roots).iter()
                .any(|group| group.text_residence_v1().is_some() || group.dynamic_carrier_residence_v1().is_some()
                    || group.shared_initializer_v1().is_some())
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
