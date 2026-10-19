//! Original source-selected participants, distinct from executable call coordinates.
use super::*;
use super::finalized_borrowed_call_visit::FinalizedBorrowedCallPacketV1;
use crate::mir::instruction::InvokeCallResultKind;
use crate::mir::normal_callable_semantic_package::VerifiedCallableResultContractCohortV1;
use crate::mir::resolved_control_flow::{DeclaredFunctionResultContractV1, VerifiedFunctionCompletionV1};
use crate::mir::MirModule;
use hakorune_mir_defs::SameModuleCallableNamespaceV1;
use std::collections::{BTreeMap, BTreeSet};

impl FinalizedRootSourceHandoffV1 {
    /// Reuses the original pools and completed catalog; this map is a borrowed
    /// selection projection, never a call site, source issuer or ABI receipt.
    pub(in crate::mir) fn borrowed_callable_participants_v1(
        &self,
        module: &MirModule,
        callables: &VerifiedCallableResultContractCohortV1,
    ) -> Result<BTreeMap<CanonicalSameModuleCallableKeyV1, InvokeCallResultKind>, String> {
        // Scalar source membership does not imply a tagged borrowed entry.
        let scalar_source = |packet: FinalizedBorrowedCallPacketV1<'_>| matches!(packet,
            FinalizedBorrowedCallPacketV1::Lexical(physical)
            if matches!(physical.original_source(), CallPacketSourceLoanV1::Instance(original)
                if original.scalar_result_source_v1().is_some()));
        let mut selected_callees = BTreeSet::new();
        let mut real_targets = BTreeMap::new();
        let declarations: BTreeSet<_> = self
            .ledger
            .borrowed_declaration_owners_v1()?
            .into_iter()
            .collect();
        self.visit_finalized_borrowed_call_nodes_v1(module, |packet, arguments, _, _, _| {
            match real_targets.entry(packet.target().clone()) {
                std::collections::btree_map::Entry::Vacant(entry) => {
                    entry.insert((packet.callee_owner(), packet.result()));
                }
                std::collections::btree_map::Entry::Occupied(entry)
                    if entry.get() == &(packet.callee_owner(), packet.result()) => {}
                _ => return Err(freeze("callable-participant/real-result-drift")),
            }
            let unit = packet.result() == Some(InvokeCallResultKind::Unit);
            if unit {
                let super::finalized_borrowed_call_visit::FinalizedBorrowedCallPacketV1::Lexical(physical) = packet
                    else { return Err(freeze("callable-participant/unit-source-kind")); };
                let crate::mir::normal_callable_semantic_package::CallPacketSourceLoanV1::Instance(original) = physical.original_source()
                    else { return Err(freeze("callable-participant/unit-source-kind")); };
                self.ledger.corroborate_unit_discard_v1(original)?;
            }
            if unit || scalar_source(packet) || packet.selects_borrowed_source(arguments)
                || self.has_borrowed_ordinary_entry_v1(packet.callee_owner())
            {
                selected_callees.insert(packet.target().clone());
            }
            Ok(())
        })?;
        let mut owners = declarations.clone();
        // The original field-loop packet remains a validation participant even
        // when no executable Call happens to invoke its declared method.
        let field_owners: BTreeSet<_> = self.local_calls.iter()
            .filter(|(_, groups)| groups.iter().any(|group| group.field_array_append().is_some()))
            .map(|(owner, _)| *owner).collect();
        owners.extend(field_owners.iter().copied());
        self.visit_finalized_borrowed_call_nodes_v1(module, |packet, arguments, _, _, _| {
            if selected_callees.contains(packet.target()) {
                if packet.result() != Some(InvokeCallResultKind::Unit) {
                    let actuals = self.borrowed_finalized_call_actuals_v1(packet)?;
                    let scalar_only = scalar_source(packet)
                        && !packet.selects_borrowed_source(arguments)
                        && !self.has_borrowed_ordinary_entry_v1(packet.callee_owner());
                    if actuals.is_none() && !scalar_only {
                        return Err(freeze("callable-participant/actuals-missing"));
                    }
                }
                owners.insert(packet.call_site().owner());
                owners.insert(packet.callee_owner());
            }
            Ok(())
        })?;
        let mut result = BTreeMap::new();
        for owner in owners {
            if owner == self.owner() {
                continue;
            }
            let mut birth_owner = false;
            for (key, retained) in &self.construction {
                let symbol = module.canonical_callable_definition_symbol(key)
                    .ok_or_else(|| freeze("callable-participant/birth-definition"))?;
                let function = module.functions.get(symbol)
                    .ok_or_else(|| freeze("callable-participant/birth-function"))?;
                if retained.corroborates_birth_owner_v1(owner, key, function)? {
                    if birth_owner { return Err(freeze("callable-participant/birth-owner-duplicate")); }
                    birth_owner = true;
                }
            }
            if birth_owner { continue; }
            let Some(crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(key)) =
                callables.completed_key_for_owner_v1(owner)
            else {
                return Err(freeze("callable-participant/selected-key"));
            };
            if callables.owner_for_canonical_key(key) != Some(owner) {
                return Err(freeze("callable-participant/owner-key"));
            }
            if !matches!(
                key.namespace(),
                SameModuleCallableNamespaceV1::InstanceBoxMethod
                    | SameModuleCallableNamespaceV1::StaticBoxMethod
            ) {
                return Err(freeze("callable-participant/owner-key"));
            }
            let contract = callables
                .completed_result(
                    &crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(key.clone()),
                )
                .ok_or_else(|| freeze("callable-participant/result-missing"))?;
            let symbol = module
                .canonical_callable_definition_symbol(key)
                .ok_or_else(|| freeze("callable-participant/definition-missing"))?;
            let function = module
                .functions
                .get(symbol)
                .ok_or_else(|| freeze("callable-participant/function-missing"))?;
            self.ledger.with_finished_projection(owner, |finished, _| {
                if finished != symbol || function.signature.name != key.mir_symbol_projection() {
                    return Err(freeze("callable-participant/finished-symbol"));
                }
                Ok(())
            })?;
            if field_owners.contains(&owner) {
                self.ledger.with_finished_projection(owner, |_, projection| {
                    for group in &self.local_calls[&owner] {
                        let Some(packet) = group.field_array_append() else { continue; };
                        if group.site() != packet.source().site() || group.site().owner() != owner {
                            return Err(freeze("callable-participant/field-source-owner"));
                        }
                        group.require_sealed_condition_v1()?;
                        group.validate_recorded_in_context(CallValidationContextV1::Finalized(self),
                            function, Some(projection))?;
                    }
                    Ok(())
                })?;
            }
            let validate_unit = |completion: &VerifiedFunctionCompletionV1| -> Result<(), String> {
                if contract.owner() != owner || !std::ptr::eq(contract.completion(), completion)
                    || contract.result().is_some() || completion.returns_value()
                    || !matches!(completion.function_exit_contract().declared_result(), DeclaredFunctionResultContractV1::Unannotated)
                    || function.signature.return_type != crate::mir::MirType::Void
                    || !self.ledger.root_home_flow_ready_v1(completion)? {
                    return Err(freeze("callable-participant/unit-result-agreement"));
                }
                Ok(())
            };
            // An issued real call target retains its original result owner.
            // Source caller membership adds a seed, not a second Home result law.
            if let Some((target_owner, kind)) = real_targets.get(key) {
                if *target_owner != owner {
                    return Err(freeze("callable-participant/real-owner-drift"));
                }
                let kind =
                    kind.ok_or_else(|| freeze("callable-participant/real-result-unavailable"))?;
                if kind == InvokeCallResultKind::Unit {
                    let completion = self.ledger.completion_for_owner(owner)
                        .ok_or_else(|| freeze("callable-participant/completion-missing"))?;
                    validate_unit(completion)?;
                    self.validate_finalized_unit_participant_v1(owner, module)?;
                }
                result.insert(key.clone(), kind);
                continue;
            }
            if field_owners.contains(&owner) {
                if let Some(completion) = self.ledger.implicit_unit_completion_source_v1(owner)? {
                    validate_unit(&completion)?;
                    if !completion.explicit_sites().is_empty() || completion.implicit_body_end().is_none()
                        || !self.ledger.completion_for_owner(owner).is_some_and(|original| std::ptr::eq(original, completion.as_ref())) {
                        return Err(freeze("callable-participant/implicit-unit-completion"));
                    }
                    self.validate_finalized_unit_participant_v1(owner, module)?;
                    if result.insert(key.clone(), InvokeCallResultKind::Unit).is_some() {
                        return Err(freeze("callable-participant/duplicate"));
                    }
                    continue;
                }
            }
            if declarations.contains(&owner) {
                self.borrowed_declaration_for_function_v1(owner, key, function)?;
            }
            let completion = self
                .ledger
                .completion_for_owner(owner)
                .ok_or_else(|| freeze("callable-participant/completion-missing"))?;
            let terminals = self
                .ledger
                .terminal_relation_index
                .get(&owner)
                .ok_or_else(|| freeze("callable-participant/terminals-missing"))?;
            let exits: BTreeSet<_> = completion.explicit_sites().iter().cloned().collect();
            if contract.owner() != owner
                || !std::ptr::eq(contract.completion(), completion)
                || !std::ptr::eq(contract.terminal_relations(), terminals.as_ref())
                || !completion.returns_value()
                || completion.implicit_body_end().is_some()
                || exits.is_empty()
                || exits.len() != completion.explicit_sites().len()
                || terminals.keys().cloned().collect::<BTreeSet<_>>() != exits
            {
                return Err(freeze("callable-participant/completion-coverage"));
            }
            let kind = if let Some(descriptor) =
                self.ledger.checked_callable_object_result_v1(key, owner)?
            {
                if descriptor.owner != owner
                    || descriptor.terminals.len() != exits.len()
                    || !matches!(
                        completion.function_exit_contract().declared_result(),
                        DeclaredFunctionResultContractV1::Unannotated
                    )
                    || contract.result().is_some()
                {
                    return Err(freeze("callable-participant/object-result-agreement"));
                }
                for relation in terminals.values() {
                    if let TerminalRelationV1::Value(value) = relation {
                        if let TerminalReturnedSourceV1::OwnedCall(original) = value.returned() {
                            match original.acquisition() {
                                crate::mir::resolved_semantics::home_new_prefix::ObjectReturnAcquisitionV1::Direct { .. } => {
                                    let own = self.ledger.verified_direct_object_return_source_v1(owner, relation.return_site())?
                                        .ok_or_else(|| freeze("callable-participant/direct-result-missing"))?;
                                    if own.class() != descriptor.class || own.result() != descriptor.kind {
                                        return Err(freeze("callable-participant/direct-result-drift"));
                                    }
                                }
                                _ => return Err(freeze("callable-participant/return-acquisition")),
                            }
                        }
                    }
                }
                descriptor.kind
            } else if let Some(scalar) = self.ledger.callable_scalar_result_source_v1(owner)? {
                let (kind, agrees) = match scalar.kind() {
                    crate::mir::resolved_semantics::home_new_prefix::SourceScalarKind::Integer =>
                        (InvokeCallResultKind::I64, contract.declared_result_agrees_with_i64_source()),
                    crate::mir::resolved_semantics::home_new_prefix::SourceScalarKind::Bool =>
                        (InvokeCallResultKind::Bool, contract.declared_result_agrees_with_bool_source()),
                };
                if scalar.owner() != owner || !std::ptr::eq(scalar.completion(), completion) || !agrees
                    || terminals.values().any(|terminal| match (kind, terminal) {
                        (InvokeCallResultKind::Bool, TerminalRelationV1::BoolLiteral(_)) => false,
                        (_, TerminalRelationV1::Call(call)) => call.source_scalar().is_none()
                            || !matches!((kind, call.scalar_kind()),
                                (InvokeCallResultKind::I64, crate::mir::resolved_semantics::home_new_prefix::SourceScalarKind::Integer)
                                | (InvokeCallResultKind::Bool, crate::mir::resolved_semantics::home_new_prefix::SourceScalarKind::Bool)),
                        (InvokeCallResultKind::I64, TerminalRelationV1::I64Add(_) | TerminalRelationV1::IntegerLiteral(_)
                            | TerminalRelationV1::I64Field(_) | TerminalRelationV1::I64Scalar(_) | TerminalRelationV1::MapGet(_)) => false,
                        _ => true,
                    })
                { return Err(freeze("callable-participant/scalar-result-agreement")); }
                kind
            } else {
                if !contract.declared_result_agrees_with_i64_source()
                    || terminals.values().any(|terminal| {
                        !matches!(
                            terminal,
                            TerminalRelationV1::Call(_)
                                | TerminalRelationV1::I64Add(_)
                                | TerminalRelationV1::IntegerLiteral(_)
                                | TerminalRelationV1::I64Field(_)
                                | TerminalRelationV1::I64Scalar(_)
                                | TerminalRelationV1::MapGet(_)
                        )
                    })
                {
                    return Err(freeze("callable-participant/result-unavailable"));
                }
                InvokeCallResultKind::I64
            };
            for exit in &exits {
                let relation = &terminals[exit];
                if relation.owner() != owner
                    || relation.return_site() != exit
                    || self
                        .ledger
                        .normal_exit_projection_v1(owner, exit)?
                        .is_none()
                {
                    return Err(freeze("callable-participant/terminal-source"));
                }
                self.ledger
                    .with_finished_projection(owner, |finished, projection| {
                        if finished != function.signature.name {
                            return Err(freeze("callable-participant/finished-symbol"));
                        }
                        let saved = self.ledger.root_exits.borrow();
                        let Some(RootHomeExitProgress::Emitted {
                            order,
                            entry,
                            bindings,
                            ..
                        }) = saved.get(&(owner, exit.clone().into()))
                        else {
                            return Err(freeze("callable-participant/cleanup-missing"));
                        };
                        if matches!(
                            kind,
                            InvokeCallResultKind::Handle | InvokeCallResultKind::NullableHandle
                        ) {
                            let TerminalRelationV1::Value(value) = relation else {
                                return Err(freeze("callable-participant/object-terminal"));
                            };
                            match value.returned() {
                                TerminalReturnedSourceV1::Construction(_) => {
                                    if self
                                        .ledger
                                        .validate_fresh_return_producer_v1(
                                            owner,
                                            exit,
                                            entry,
                                            bindings,
                                            Some(projection),
                                            Some(projection),
                                            Some(function),
                                        )?
                                        .is_none()
                                    {
                                        return Err(freeze(
                                            "callable-participant/fresh-producer-unavailable",
                                        ));
                                    }
                                }
                                TerminalReturnedSourceV1::NullLiteral
                                    if order.null_return_binding().is_none() =>
                                {
                                    return Err(freeze(
                                        "callable-participant/null-producer-missing",
                                    ));
                                }
                                TerminalReturnedSourceV1::NullLiteral
                                | TerminalReturnedSourceV1::OwnedCall(_) => {}
                                _ => return Err(freeze("callable-participant/object-terminal")),
                            }
                        }
                        super::finalized_root_cleanup::validate_finished_cleanup_entry(
                            CallValidationContextV1::Finalized(self),
                            owner,
                            exit,
                            function,
                            projection,
                            order,
                            entry,
                            bindings,
                            Some(projection),
                        )
                    })?;
            }
            if result.insert(key.clone(), kind).is_some() {
                return Err(freeze("callable-participant/duplicate"));
            }
        }
        Ok(result)
    }
}
