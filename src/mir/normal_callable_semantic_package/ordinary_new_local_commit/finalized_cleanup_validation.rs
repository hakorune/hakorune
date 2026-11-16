//! Finished root exit source, binding, and cleanup validation.
use super::*;
impl FinalizedRootSourceHandoffV1 {
    pub(in crate::mir) fn validate_finalized_root_cleanup_v1(
        &self,
        module: &MirModule,
    ) -> Result<(), String> {
        self.ledger
            .with_finished_projection(self.owner, |symbol, _| {
                let root = module
                    .functions
                    .get(symbol)
                    .ok_or_else(|| freeze("main-arguments/root-missing"))?;
                self.main_arguments_bindings_v1(module, root).map(|_| ())
            })?;
        let context = CallValidationContextV1::Finalized(self);
        let lifecycle_owners = self.initializer_lifecycle_owners_v1();
        for owner in &lifecycle_owners {
            context.with_local_call_group_tree_v2(*owner, |groups| {
                for packet in groups.iter().filter_map(|group| group.text_residence_v1()) {
                    self.ledger
                        .validate_text_residence_home_v1(&packet, context)?;
                }
                Ok(())
            })?;
        }
        let exits = self.ledger.root_exits.borrow();
        // A published Home owes every original exit, not just rows that happen
        // to remain in the physical ledger. Source-only declarations add none.
        for owner in &lifecycle_owners {
            let completion = self
                .ledger
                .completion_for_owner(*owner)
                .ok_or_else(|| freeze("final-cleanup/initializer-completion-missing"))?;
            for exit in completion
                .explicit_sites()
                .iter()
                .cloned()
                .map(RootHomeExitKeyV1::Explicit)
                .chain(
                    completion
                        .implicit_body_end()
                        .map(|_| RootHomeExitKeyV1::ImplicitBodyEnd),
                )
            {
                if !matches!(
                    exits.get(&(*owner, exit.clone())),
                    Some(RootHomeExitProgress::Emitted { .. })
                ) && !matches!(exits.get(&(*owner, exit)), Some(RootHomeExitProgress::Finalized(_)) if *owner == self.owner)
                {
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
                let RootHomeExitProgress::Emitted {
                    order,
                    origins,
                    bindings,
                    entry,
                } = progress
                else {
                    return Err(freeze("implicit-exit/unconsumed"));
                };
                self.ledger
                    .with_finished_projection(*owner, |symbol, projection| {
                        let function = module
                            .functions
                            .get(symbol)
                            .ok_or_else(|| freeze("final-cleanup/function-missing"))?;
                        validate_finished_implicit_cleanup(
                            &self.ledger,
                            self,
                            *owner,
                            function,
                            projection,
                            order,
                            origins,
                            bindings,
                            entry,
                        )
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
                    if !self.checks_explicit_exit_origin_v1(*owner, entry, &lifecycle_owners) {
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
pub(in super::super) fn validate_finished_cleanup_entry(
    context: CallValidationContextV1<'_>,
    owner: FunctionOwnerIdV1,
    exit: &SourceStmtSiteV1,
    function: &MirFunction,
    projection: &physical_boundary::FinishedBindings,
    order: &super::super::root_home::RootHomeCleanupOrderV1,
    entry: &RootHomeExitEntry,
    bindings: &[(BasicBlockId, MirInstruction)],
    finishing: Option<&physical_boundary::FinishedBindings>,
) -> Result<(), String> {
    let ledger = context.ledger();
    let declared = ledger
        .terminal_relation_for_owner_at(owner, exit)
        .and_then(|terminal| {
            let TerminalRelationV1::Value(terminal) = terminal else {
                return None;
            };
            let TerminalReturnedSourceV1::BorrowedNullableLocal(source) = terminal.returned()
            else {
                return None;
            };
            source.read_v1()
        });
    match (declared, &order.return_read) {
        (Some(source), Some(read)) => {
            context.declared_result_read_v1(source, read)?;
            context.validate_declared_read_transport_v1(
                source.source(),
                read.binding_observation_v2(),
                function,
                finishing,
            )?;
        }
        (None, None) => {}
        (None, Some(_)) if order.scalar_return_read.is_some() => {}
        _ => return Err(freeze("borrowed-return/source-kind")),
    }
    if context.is_finalized() {
        ledger.validate_local_call_binding_groups_for_key_v1(
            owner,
            &exit.clone().into(),
            entry.local_call_groups(),
            &[],
        )?;
    }
    if let RootHomeExitEntry::Call {
        row: super::super::super::super::RootCallDispositionV1::Lexical(packet),
        arguments,
        invoke,
        projection: result,
        frame,
        ..
    } = entry
    {
        let mut recorded = arguments.clone();
        recorded.extend_from_slice(bindings);
        recorded.extend([invoke.clone(), result.clone()]);
        // Expression actuals already retain the same borrowed FaultFrame.
        // This inventory lends its sole definition; it does not emit another.
        if !recorded.contains(frame) {
            recorded.push(frame.clone());
        }
        packet.validate_expression_arguments_with_cancellation_v1(
            context,
            function,
            Some(projection),
            &recorded,
            finishing.is_none().then_some(projection),
            None,
            &order.result_cancellation_v1()?,
        )?;
    }
    let mut mandatory = match finishing {
        Some(finishing) => finishing.bindings(bindings)?,
        None => bindings.to_vec(),
    };
    if let Some(original) = &order.original_frame {
        let frame = projection
            .binding(original.0, &original.1)?
            .ok_or_else(|| freeze("root-frame-missing"))?;
        let MirInstruction::FaultFrameEnter { dst, .. } = frame.1 else {
            return Err(freeze("root-frame-missing"));
        };
        if !physical_boundary::check_binding(function, None, frame.0, &frame.1)?
            || function
                .blocks
                .values()
                .flat_map(|block| block.all_instructions())
                .filter(|instruction| matches!(instruction, MirInstruction::FaultFrameEnter { .. }))
                .count()
                != 1
            || mandatory.iter().any(|(_, instruction)| {
                matches!(instruction,
                MirInstruction::Invoke { fault_frame, .. } if *fault_frame != dst)
            })
        {
            return Err(freeze("root-frame-missing"));
        }
        mandatory.push(frame);
    }
    if let Some(packet) = order.installed_map_return() {
        packet.validate_return(entry, bindings, finishing)?;
        packet.validate(context, function, Some(projection), entry)?;
        mandatory.push(
            projection
                .relation_binding(function, packet.alias())?
                .ok_or_else(|| freeze("borrowed-return/return-missing"))?,
        );
    }
    if let Some(original) = order.null_return_binding() {
        let finished = projection
            .binding(original.0, &original.1)?
            .ok_or_else(|| freeze("terminal-null/producer-removed"))?;
        mandatory.push(finished);
    }
    if let Some(producers) = ledger.validate_return_expression_producer_v1(
        context,
        owner,
        exit,
        order,
        entry,
        bindings,
        Some(projection),
        finishing,
        Some(function),
    )? {
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
        function,
        bindings,
        entry,
        order,
        finishing,
        Some(projection),
    )?;
    match finishing {
        Some(finishing) => root_cleanup_graph::ordered_structure::validate_projected(
            function,
            bindings,
            entry,
            finishing,
            order.ingress_result_kind(),
            order.borrowed_expression_fault_v1(),
        ),
        None => root_cleanup_graph::ordered_structure::validate_finished_call(
            function,
            bindings,
            entry,
            order.ingress_result_kind(),
        ),
    }
}

pub(super) fn validate_finished_implicit_cleanup(
    ledger: &OrdinaryNewClaimLedgerV1,
    source: &FinalizedRootSourceHandoffV1,
    owner: FunctionOwnerIdV1,
    function: &MirFunction,
    projection: &physical_boundary::FinishedBindings,
    order: &super::super::root_home::RootHomeCleanupOrderV1,
    origins: &[super::super::root_home::RootHomeReleaseEmissionV1],
    bindings: &[(BasicBlockId, MirInstruction)],
    entry: &RootHomeExitEntry,
) -> Result<(), String> {
    ledger.validate_implicit_root_graph_in_context_v1(
        owner,
        function,
        Some(projection),
        order,
        origins,
        bindings,
        entry,
        CallValidationContextV1::Finalized(source),
    )?;
    let mut mandatory = bindings.to_vec();
    entry.append_bindings(&mut mandatory);
    let (_, producer) = order
        .implicit_void
        .as_ref()
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
pub(super) fn lend_conditional_end_bindings(
    result: &mut Vec<(BasicBlockId, MirInstruction)>,
    function: &MirFunction,
    projection: &physical_boundary::FinishedBindings,
    bindings: &[(BasicBlockId, MirInstruction)],
    finishing: Option<&physical_boundary::FinishedBindings>,
) -> Result<(), String> {
    let finished = match finishing {
        Some(projection) => projection.bindings(bindings)?,
        None => bindings.to_vec(),
    };
    for binding in finished {
        if !matches!(
            binding.1,
            MirInstruction::Invoke {
                operation: crate::mir::instruction::InvokeOperation::ObjectEndIfOwned { .. }
                    | crate::mir::instruction::InvokeOperation::Shared(
                        crate::mir::instruction::SharedInvokeOperation::Release { .. }
                    ),
                ..
            }
        ) {
            continue;
        }
        if !projection.recorded().contains(&binding)
            || !physical_boundary::check_binding(function, None, binding.0, &binding.1)?
        {
            return Err(freeze("conditional-end/binding-unrecorded"));
        }
        if !result.contains(&binding) {
            result.push(binding);
        }
    }
    Ok(())
}
