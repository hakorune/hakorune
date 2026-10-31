//! Same-owner final cleanup corroboration, retaining original source order.
use crate::mir::normal_callable_semantic_package::CallValidationContextV1;
use super::*;
use crate::mir::MirModule;
use crate::mir::resolved_semantics::home_new_prefix::RootHomeExitKeyV1;

impl FinalizedRootSourceHandoffV1 {
    pub(super) fn validate_finalized_text_participant_v1(
        &self, owner: FunctionOwnerIdV1, module: &MirModule,
        contract: crate::mir::normal_callable_semantic_package::result_contract::CallableResultContractRefV1<'_>,
    ) -> Result<bool, String> {
        let completion = self.ledger.completion_for_owner(owner)
            .ok_or_else(|| freeze("callable-participant/completion-missing"))?;
        let terminals = self.ledger.terminal_relation_index.get(&owner)
            .ok_or_else(|| freeze("callable-participant/terminals-missing"))?;
        if !terminals.values().any(|relation| matches!(relation,
            TerminalRelationV1::Value(value) if matches!(value.returned(), TerminalReturnedSourceV1::TextResidence { .. } | TerminalReturnedSourceV1::TextExpression(_)))) {
            return Ok(false);
        }
        let exits: std::collections::BTreeSet<_> = completion.explicit_sites().iter().cloned().collect();
        if contract.owner() != owner || !std::ptr::eq(contract.completion(), completion)
            || !std::ptr::eq(contract.terminal_relations(), terminals.as_ref())
            || !contract.has_complete_text_result_v1() {
            return Err(freeze("callable-participant/result-unavailable"));
        }
        self.ledger.with_finished_projection(owner, |symbol, projection| {
            let function = module.functions.get(symbol).filter(|function| function.signature.name == symbol
                && function.signature.return_type == crate::mir::MirType::String)
                .ok_or_else(|| freeze("final-cleanup/function-missing"))?;
            let rows = self.ledger.root_exits.borrow();
            for site in &exits {
                let source = self.ledger.normal_exit_projection_v1(owner, site)?
                    .ok_or_else(|| freeze("root-exit-source-missing"))?;
                let Some(RootHomeExitProgress::Emitted { order, origins, bindings, entry }) = rows.get(&(owner, site.clone().into()))
                    else { return Err(freeze("root-exit-unconsumed")) };
                let mut expected = super::root_home::RootHomeCleanupOrderV1::from_home_end_plans_for_exit(
                    &self.ledger.local_commits.borrow(), &site.clone().into(), source.fault_homes(), source.homes())?
                    .ok_or_else(|| freeze("root-home-end-unavailable"))?;
                self.ledger.adopt_loop_return_text_ends_v1(owner, &site.clone().into(), &mut expected)?;
                expected.include_fresh_result_prefix_v1(&self.ledger, owner, site, order)?;
                if order.full() != expected.full() || order.normal() != expected.normal()
                    || order.normal() != origins.iter().map(|origin| origin.origin().clone()).collect::<Vec<_>>() {
                    return Err(freeze("root-exit-origin-order"));
                }
                validate_finished_cleanup_entry(CallValidationContextV1::Finalized(self), owner, site,
                    function, projection, order, entry, bindings, Some(projection))?;
            }
            Ok(true)
        })
    }

    /// Validate every original Unit endpoint against the retained final pool.
    /// Both explicit and implicit exits keep their source order and bindings;
    /// no caller returns to the affine lowering pool after handoff.
    pub(super) fn validate_finalized_unit_participant_v1(
        &self, owner: FunctionOwnerIdV1, module: &MirModule,
    ) -> Result<(), String> {
        let completion = self.ledger.completion_for_owner(owner)
            .ok_or_else(|| freeze("callable-participant/completion-missing"))?;
        self.ledger.with_finished_projection(owner, |symbol, projection| {
            let function = module.functions.get(symbol)
                .filter(|function| function.signature.name == symbol)
                .ok_or_else(|| freeze("final-cleanup/function-missing"))?;
            let context = CallValidationContextV1::Finalized(self);
            let exits = self.ledger.root_exits.borrow();
            for site in completion.explicit_sites() {
                let source = self.ledger.normal_exit_projection_v1(owner, site)?
                    .ok_or_else(|| freeze("root-exit-source-missing"))?;
                let Some(RootHomeExitProgress::Emitted { order, origins, bindings, entry }) =
                    exits.get(&(owner, site.clone().into()))
                else { return Err(freeze("root-exit-unconsumed")); };
                let mut expected = super::root_home::RootHomeCleanupOrderV1::from_home_end_plans_for_exit(
                    &self.ledger.local_commits.borrow(), &site.clone().into(),
                    source.homes(), source.homes(),
                )?.ok_or_else(|| freeze("root-home-end-unavailable"))?;
                self.ledger.adopt_loop_return_text_ends_v1(owner, &site.clone().into(), &mut expected)?;
                if order.full() != expected.full() || order.normal() != expected.normal()
                    || order.normal() != origins.iter().map(|origin| origin.origin().clone()).collect::<Vec<_>>() {
                    return Err(freeze("root-exit-origin-order"));
                }
                if !matches!(entry, RootHomeExitEntry::Plain { .. }) {
                    return Err(freeze("call-source-missing"));
                }
                for origin in origins {
                    let emitted = origin.binding();
                    if !matches!(&emitted.1, MirInstruction::Invoke { operation, .. }
                        if operation == origin.origin().operation())
                        || bindings.iter().filter(|binding| **binding == emitted).count() != 1 {
                        return Err(freeze("root-exit-origin-binding-drift"));
                    }
                }
                self.ledger.validate_local_call_binding_groups_for_key_v1(owner,
                    &site.clone().into(), entry.local_call_groups(), &[])?;
                for group in entry.local_call_groups() {
                    group.validate_recorded_in_context(context, function, Some(projection))?;
                    group.check_physical_bindings_v1(function, Some(projection))?;
                }
                validate_finished_cleanup_entry(context, owner, site, function, projection,
                    order, entry, bindings, Some(projection))?;
            }
            if completion.implicit_body_end().is_some() {
                let Some(RootHomeExitProgress::Emitted { order, origins, bindings, entry }) =
                    exits.get(&(owner, RootHomeExitKeyV1::ImplicitBodyEnd))
                else { return Err(freeze("implicit-exit/unconsumed")); };
                validate_finished_implicit_cleanup(&self.ledger, self, owner, function,
                    projection, order, origins, bindings, entry)?;
            }
            Ok(())
        })
    }

    /// Lend the original complete conditional End operations only after the
    /// same source cleanup and finished-binding correspondence checks.
    pub(in crate::mir) fn object_end_bindings_v1(
        &self, module: &MirModule, function: &MirFunction,
    ) -> Result<Vec<(BasicBlockId, MirInstruction)>, String> {
        if !module.functions.get(&function.signature.name)
            .is_some_and(|stored| std::ptr::eq(stored, function))
        { return Err(freeze("conditional-end/function-membership")); }
        let mut result = Vec::new();
        let exits = self.ledger.root_exits.borrow();
        for ((owner, exit), progress) in exits.iter() {
            let (order, entry, bindings, original) = match progress {
                RootHomeExitProgress::Emitted { order, entry, bindings, .. } =>
                    (order, entry, bindings.as_slice(), true),
                RootHomeExitProgress::Finalized(order) => {
                    if *owner != self.owner { return Err(freeze("conditional-end/foreign-finalized")); }
                    let site = exit.explicit_site().ok_or_else(|| freeze("conditional-end/finalized-implicit"))?;
                    let (entry, bindings) = self.call_entries.get(site)
                        .ok_or_else(|| freeze("conditional-end/entry-missing"))?;
                    (order, entry, bindings.as_slice(), false)
                }
                _ => continue, // No uncompleted row can lend an operation.
            };
            let mut candidates = bindings.to_vec();
            entry.append_bindings(&mut candidates);
            if !candidates.iter().any(|(_, row)| matches!(row,
                MirInstruction::Invoke { operation: crate::mir::instruction::InvokeOperation::ObjectEndIfOwned { .. }
                    | crate::mir::instruction::InvokeOperation::Shared(crate::mir::instruction::SharedInvokeOperation::Release { .. }), .. }))
            { continue; }
            self.ledger.with_finished_projection(*owner, |symbol, projection| {
                if symbol != function.signature.name { return Ok(()); }
                if !module.functions.get(symbol).is_some_and(|stored| std::ptr::eq(stored, function)) {
                    return Err(freeze("conditional-end/function-membership"));
                }
                let finishing = original.then_some(projection);
                if let Some(site) = exit.explicit_site() {
                    validate_finished_cleanup_entry(CallValidationContextV1::Finalized(self), *owner, site, function,
                        projection, order, entry, bindings, finishing)?;
                } else {
                    let RootHomeExitProgress::Emitted { origins, .. } = progress else {
                        return Err(freeze("conditional-end/implicit-unfinished"));
                    };
                    validate_finished_implicit_cleanup(&self.ledger, self, *owner,
                        function, projection, order, origins, bindings, entry)?;
                }
                for group in entry.local_call_groups() {
                    group.validate_recorded_in_context(CallValidationContextV1::Finalized(self),
                        function, Some(projection))?;
                }
                lend_conditional_end_bindings(&mut result, function, projection, &candidates, finishing)?;
                Ok(())
            })?;
        }
        // The retained final pool also owns Call-Fault cleanup groups.
        // Never return to the moved lowering packet pool after handoff.
        for (owner, groups) in &self.local_calls {
            self.ledger.with_finished_projection(*owner, |symbol, projection| {
                if symbol != function.signature.name { return Ok(()); }
                for group in groups {
                    group.validate_recorded_in_context(CallValidationContextV1::Finalized(self),
                        function, Some(projection))?;
                    lend_conditional_end_bindings(&mut result, function, projection,
                        group.bindings(), Some(projection))?;
                }
                Ok(())
            })?;
        }
        // New's allocation/Birth Fault paths have the same previous-Home End
        // obligations. Reuse only the original Ordinary/Result row verifier.
        for (site, row) in self.ledger.local_commits.borrow().iter() {
            if !matches!(row, LocalCommitV1::Ordinary(_) | LocalCommitV1::Result(_)) { continue; }
            let Some(NewEmissionProgress::Emitted { bindings, .. }) = row.new_emission() else { continue; };
            if !bindings.iter().any(|(_, row)| matches!(row, MirInstruction::Invoke {
                operation: crate::mir::instruction::InvokeOperation::ObjectEndIfOwned { .. }
                    | crate::mir::instruction::InvokeOperation::Shared(crate::mir::instruction::SharedInvokeOperation::Release { .. }), .. }))
            { continue; }
            self.ledger.with_finished_projection(row.owner(), |symbol, projection| {
                if symbol != function.signature.name { return Ok(()); }
                self.ledger.validate_ordinary_new_row_emission_projected(row.owner(), site,
                    row, function, Some(projection), CallValidationContextV1::Finalized(self))?;
                lend_conditional_end_bindings(&mut result, function, projection,
                    bindings, Some(projection))
            })?;
        }
        // Checked field reads already retain their source-validated Fault
        // cleanup. Lend its actual End rows to the same physical census.
        for owner in self.ledger.checked_field_read_owners_v1() {
            self.ledger.with_finished_projection(owner, |symbol, projection| {
                if symbol != function.signature.name { return Ok(()); }
                let context = CallValidationContextV1::Finalized(self);
                for site in self.ledger.checked_local_field_sites_v1(owner) {
                    self.ledger.validate_checked_field_cleanup_v1(&site, function, context, &mut |original| {
                        projection.binding(original.0, &original.1)
                    })?;
                    let bindings = self.ledger.checked_field_cleanup_bindings_v1(&site)?;
                    lend_conditional_end_bindings(&mut result, function, projection, &bindings, Some(projection))?;
                }
                Ok(())
            })?;
        }
        Ok(result)
    }

    /// Lend Array operations from their checked local construction rows.
    /// Empty Arrays transferred directly into a field have no element-holder
    /// row, but their Claim-fault release still belongs to this source owner.
    pub(in crate::mir) fn finished_array_construction_bindings_v1(
        &self, module: &MirModule, function: &MirFunction,
    ) -> Result<Vec<(BasicBlockId, MirInstruction)>, String> {
        if !module.functions.get(&function.signature.name)
            .is_some_and(|stored| std::ptr::eq(stored, function))
        { return Err(freeze("array-construction/function-membership")); }
        let constructions: Vec<_> = self.ledger.local_commits.borrow().iter()
            .filter_map(|(site, row)| matches!(row, LocalCommitV1::OwnedArray(_))
                .then_some((row.owner(), site.clone())))
            .collect();
        let is_array_operation = |instruction: &MirInstruction| matches!(instruction,
            MirInstruction::ArrayResidenceRelease { .. }
                | MirInstruction::Invoke {
                    operation: crate::mir::instruction::InvokeOperation::OwnedArray(_)
                        | crate::mir::instruction::InvokeOperation::DynamicCarrierArray(_),
                    ..
                });
        let has_root_array_exit = self.call_entries.values()
            .any(|(_, bindings)| bindings.iter().any(|(_, row)| is_array_operation(row)))
            || self.ledger.root_exits.borrow().values().any(|progress| matches!(progress,
                RootHomeExitProgress::Emitted { bindings, .. }
                    if bindings.iter().any(|(_, row)| is_array_operation(row))));
        if !constructions.is_empty() || has_root_array_exit {
            self.validate_finalized_root_cleanup_v1(module)?;
        }
        let mut result = Vec::new();
        for (owner, site) in constructions {
            self.ledger.with_finished_projection(owner, |symbol, projection| {
                if symbol != function.signature.name { return Ok(()); }
                self.ledger.validate_owned_array_construction(
                    &site, function, Some(projection),
                )?;
                let rows = self.ledger.local_commits.borrow();
                let Some(LocalCommitV1::OwnedArray(array)) = rows.get(&site) else {
                    return Err(freeze("array-construction/source-row-missing"));
                };
                for original in array.checked_bindings()? {
                    if !is_array_operation(&original.1) { continue; }
                    let binding = projection.binding(original.0, &original.1)?
                        .ok_or_else(|| freeze("array-construction/binding-removed"))?;
                    if !projection.recorded().contains(&binding) {
                        return Err(freeze("array-construction/binding-unrecorded"));
                    }
                    if !result.contains(&binding) { result.push(binding); }
                }
                Ok(())
            })?;
        }
        self.ledger.with_finished_projection(self.owner, |symbol, _| {
            if symbol != function.signature.name { return Ok(()); }
            for (_, bindings) in self.call_entries.values() {
                for binding in bindings.iter().filter(|(_, row)| is_array_operation(row)) {
                    if !result.contains(binding) { result.push(binding.clone()); }
                }
            }
            Ok(())
        })?;
        let exits = self.ledger.root_exits.borrow();
        for ((owner, _), progress) in exits.iter() {
            let RootHomeExitProgress::Emitted { bindings, entry, .. } = progress else {
                continue;
            };
            let mut candidates = bindings.clone();
            entry.append_bindings(&mut candidates);
            let candidates: Vec<_> = candidates.into_iter()
                .filter(|(_, row)| is_array_operation(row)).collect();
            if candidates.is_empty() { continue; }
            self.ledger.with_finished_projection(*owner, |symbol, projection| {
                if symbol != function.signature.name { return Ok(()); }
                for original in &candidates {
                    let binding = projection.binding(original.0, &original.1)?
                        .ok_or_else(|| freeze("array-construction/exit-binding-removed"))?;
                    if !projection.recorded().contains(&binding) {
                        return Err(freeze("array-construction/exit-binding-unrecorded"));
                    }
                    if !result.contains(&binding) { result.push(binding); }
                }
                Ok(())
            })?;
        }
        Ok(result)
    }

    pub(in crate::mir) fn validate_finalized_root_cleanup_v1(
        &self,
        module: &MirModule,
    ) -> Result<(), String> {
        self.ledger.with_finished_projection(self.owner, |symbol, _| {
            let root = module.functions.get(symbol).ok_or_else(|| freeze("main-arguments/root-missing"))?;
            self.main_arguments_bindings_v1(module, root).map(|_| ())
        })?;
        let context = CallValidationContextV1::Finalized(self);
        let lifecycle_owners = self.initializer_lifecycle_owners_v1();
        for owner in &lifecycle_owners {
            context.with_local_call_group_tree_v2(*owner, |groups| {
                for packet in groups.iter().filter_map(|group| group.text_residence_v1()) {
                    self.ledger.validate_text_residence_home_v1(&packet, context)?;
                }
                Ok(())
            })?;
        }
        let exits = self.ledger.root_exits.borrow();
        // A published Home owes every original exit, not just rows that happen
        // to remain in the physical ledger. Source-only declarations add none.
        for owner in &lifecycle_owners {
            let completion = self.ledger.completion_for_owner(*owner)
                .ok_or_else(|| freeze("final-cleanup/initializer-completion-missing"))?;
            for exit in completion.explicit_sites().iter().cloned().map(RootHomeExitKeyV1::Explicit)
                .chain(completion.implicit_body_end().map(|_| RootHomeExitKeyV1::ImplicitBodyEnd)) {
                if !matches!(exits.get(&(*owner, exit.clone())), Some(RootHomeExitProgress::Emitted { .. }))
                    && !matches!(exits.get(&(*owner, exit)), Some(RootHomeExitProgress::Finalized(_)) if *owner == self.owner) {
                    return Err(freeze("final-cleanup/initializer-exit-missing"));
                }
            }
        }
        for exit in self.call_entries.keys() {
            if !matches!(
                exits.get(&(self.owner, exit.clone().into())),
                Some(RootHomeExitProgress::Finalized(_))
            ) {
                return Err(freeze("final-cleanup/order-missing"));
            }
        }
        for ((owner, exit), progress) in exits.iter() {
            let Some(exit) = exit.explicit_site() else {
                let RootHomeExitProgress::Emitted { order, origins, bindings, entry } = progress else {
                    return Err(freeze("implicit-exit/unconsumed"));
                };
                self.ledger.with_finished_projection(*owner, |symbol, projection| {
                    let function = module.functions.get(symbol)
                        .ok_or_else(|| freeze("final-cleanup/function-missing"))?;
                    validate_finished_implicit_cleanup(&self.ledger, self, *owner,
                        function, projection, order, origins, bindings, entry)
                })?;
                continue;
            };
            match progress {
                RootHomeExitProgress::Finalized(order) => {
                    if *owner != self.owner {
                        return Err(freeze("final-cleanup/foreign-owner"));
                    }
                    let (entry, bindings) = self
                        .call_entries
                        .get(exit)
                        .ok_or_else(|| freeze("final-cleanup/entry-missing"))?;
                    self.ledger
                        .with_finished_projection(*owner, |symbol, projection| {
                            let function = module
                                .functions
                                .get(symbol)
                                .filter(|f| f.signature.name == symbol)
                                .ok_or_else(|| freeze("final-cleanup/function-missing"))?;
                            validate_finished_cleanup_entry(
                                CallValidationContextV1::Finalized(self),
                                *owner,
                                exit,
                                function,
                                projection,
                                order,
                                entry,
                                bindings,
                                None,
                            )
                        })?;
                }
                RootHomeExitProgress::Emitted {
                    order,
                    bindings,
                    entry,
                    ..
                } => {
                    // An original initializer Home is a cleanup participant
                    // even when this child returns a plain scalar, not a Call.
                    if *owner != self.owner && !matches!(entry, RootHomeExitEntry::Call { .. })
                        && !lifecycle_owners.contains(owner) {
                        continue;
                    }
                    self.ledger
                        .with_finished_projection(*owner, |symbol, projection| {
                            let function = module
                                .functions
                                .get(symbol)
                                .filter(|f| f.signature.name == symbol)
                                .ok_or_else(|| freeze("final-cleanup/function-missing"))?;
                            validate_finished_cleanup_entry(
                                CallValidationContextV1::Finalized(self),
                                *owner,
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
                RootHomeExitProgress::Unavailable => {}
                _ => {} // Unselected source-only rows retain their existing gate.
            }
        }
        Ok(())
    }
}

/// Borrow the existing final-binding and source-order checks without moving an
/// exit. `finishing` maps original storage; finalized Call storage is already
/// rebound by its existing owner and supplies `None`.
pub(super) fn validate_finished_cleanup_entry(
    context: CallValidationContextV1<'_>,
    owner: FunctionOwnerIdV1,
    exit: &SourceStmtSiteV1,
    function: &MirFunction,
    projection: &physical_boundary::FinishedBindings,
    order: &super::root_home::RootHomeCleanupOrderV1,
    entry: &RootHomeExitEntry,
    bindings: &[(BasicBlockId, MirInstruction)],
    finishing: Option<&physical_boundary::FinishedBindings>,
) -> Result<(), String> {
    let ledger = context.ledger();
    if matches!(context, CallValidationContextV1::Finalized(_)) {
        ledger.validate_local_call_binding_groups_for_key_v1(owner,
            &exit.clone().into(), entry.local_call_groups(), &[])?;
    }
    if let RootHomeExitEntry::Call { row: super::super::RootCallDispositionV1::Lexical(packet), arguments, invoke, projection: result, frame, .. } = entry {
        let mut recorded = arguments.clone();
        recorded.extend_from_slice(bindings);
        recorded.extend([invoke.clone(), result.clone()]);
        // Expression actuals already retain the same borrowed FaultFrame.
        // This inventory lends its sole definition; it does not emit another.
        if !recorded.contains(frame) {
            recorded.push(frame.clone());
        }
        packet.validate_expression_arguments_with_cancellation_v1(context, function,
            Some(projection), &recorded, finishing.is_none().then_some(projection), None,
            &order.result_cancellation_v1()?)?;
    }
    let mut mandatory = match finishing {
        Some(finishing) => finishing.bindings(bindings)?,
        None => bindings.to_vec(),
    };
    if let Some(packet) = order.installed_map_return() {
        packet.validate_return(entry, bindings, finishing)?;
        packet.validate(context, function, Some(projection), entry)?;
        mandatory.push(projection.relation_binding(function, packet.alias())?
            .ok_or_else(|| freeze("borrowed-return/return-missing"))?);
    }
    if let Some(original) = order.null_return_binding() {
        let finished = projection
            .binding(original.0, &original.1)?
            .ok_or_else(|| freeze("terminal-null/producer-removed"))?;
        mandatory.push(finished);
    }
    if let Some(producers) = ledger.validate_return_expression_producer_v1(context, owner, exit, order, entry, bindings, Some(projection), finishing, Some(function))? {
        mandatory.extend(producers);
    }
    if let Some(producer) = ledger.validate_received_return_producer_v1(
        owner,
        exit,
        entry,
        bindings,
        Some(projection),
        finishing,
    )? {
        mandatory.push(producer);
    }
    if let Some(producer) = ledger.validate_fresh_return_producer_v1(
        owner,
        exit,
        entry,
        bindings,
        Some(projection),
        finishing,
        Some(function),
    )? {
        mandatory.extend(producer);
    }
    match (finishing, entry) {
        (
            finishing,
            RootHomeExitEntry::Call {
                invoke,
                projection: result,
                frame,
                ..
            },
        )
        | (
            finishing @ Some(_),
            RootHomeExitEntry::MapGet {
                invoke,
                projection: result,
                frame,
                ..
            },
        )
        | (
            finishing @ Some(_),
            RootHomeExitEntry::FieldGet {
                invoke,
                projection: result,
                frame,
                ..
            },
        ) => {
            for original in [invoke, result, frame] {
                mandatory.push(match finishing {
                    Some(finishing) => finishing
                        .binding(original.0, &original.1)?
                        .ok_or_else(|| freeze("final-cleanup/entry-binding-missing"))?,
                    None => original.clone(),
                });
            }
        }
        (Some(_), RootHomeExitEntry::Plain { .. }) => {}
        (None, _) => return Err(freeze("final-cleanup/entry-kind")),
    }
    for binding in mandatory {
        if !projection.recorded().contains(&binding)
            || !physical_boundary::check_binding(function, None, binding.0, &binding.1)?
        {
            return Err(freeze("final-cleanup/binding-unrecorded"));
        }
    }
    order.validate_direct_entry(ledger, owner, exit, entry, bindings, finishing)?;
    order.validate_null_return(entry, bindings, Some(projection), finishing)?;
    root_cleanup_graph::ordered_paths::validate_with_packet_projection_v1(
        function, bindings, entry, order, finishing, Some(projection))?;
    match finishing {
        Some(finishing) => root_cleanup_graph::ordered_structure::validate_projected(
            function,
            bindings,
            entry,
            finishing,
            order.ingress_result_kind(),
        ),
        None => root_cleanup_graph::ordered_structure::validate_finished_call(
            function,
            bindings,
            entry,
            order.ingress_result_kind(),
        ),
    }
}

fn validate_finished_implicit_cleanup(
    ledger: &OrdinaryNewClaimLedgerV1,
    source: &FinalizedRootSourceHandoffV1,
    owner: FunctionOwnerIdV1,
    function: &MirFunction,
    projection: &physical_boundary::FinishedBindings,
    order: &super::root_home::RootHomeCleanupOrderV1,
    origins: &[super::root_home::RootHomeReleaseEmissionV1],
    bindings: &[(BasicBlockId, MirInstruction)],
    entry: &RootHomeExitEntry,
) -> Result<(), String> {
    ledger.validate_implicit_root_graph_in_context_v1(owner, function, Some(projection),
        order, origins, bindings, entry, CallValidationContextV1::Finalized(source))?;
    let mut mandatory = bindings.to_vec();
    entry.append_bindings(&mut mandatory);
    let (_, producer) = order.implicit_void.as_ref()
        .ok_or_else(|| freeze("implicit-exit/producer-missing"))?;
    mandatory.push(producer.clone());
    for binding in projection.bindings(&mandatory)? {
        if !projection.recorded().contains(&binding) {
            return Err(freeze("final-cleanup/binding-unrecorded"));
        }
    }
    Ok(())
}

/// Corroborate only bindings supplied by a validated source family above.
fn lend_conditional_end_bindings(
    result: &mut Vec<(BasicBlockId, MirInstruction)>, function: &MirFunction,
    projection: &physical_boundary::FinishedBindings,
    bindings: &[(BasicBlockId, MirInstruction)],
    finishing: Option<&physical_boundary::FinishedBindings>,
) -> Result<(), String> {
    let finished = match finishing { Some(projection) => projection.bindings(bindings)?, None => bindings.to_vec() };
    for binding in finished {
        if !matches!(binding.1, MirInstruction::Invoke {
            operation: crate::mir::instruction::InvokeOperation::ObjectEndIfOwned { .. }
                | crate::mir::instruction::InvokeOperation::Shared(crate::mir::instruction::SharedInvokeOperation::Release { .. }), .. })
        { continue; }
        if !projection.recorded().contains(&binding)
            || !physical_boundary::check_binding(function, None, binding.0, &binding.1)?
        { return Err(freeze("conditional-end/binding-unrecorded")); }
        if !result.contains(&binding) { result.push(binding); }
    }
    Ok(())
}
