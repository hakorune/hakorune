//! Corroborate the retained terminal source/disposition against finished MIR.
//! This child issues no call target, result contract or cleanup obligation.
use super::*;
use crate::mir::instruction::InvokeCallResultKind;
use crate::mir::ConstValue;

impl OrdinaryNewClaimLedgerV1 {
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn validate_call_entry(
        &self,
        owner: FunctionOwnerIdV1,
        exit: &SourceStmtSiteV1,
        function: &MirFunction,
        finishing: Option<&super::super::super::physical_boundary::FinishedBindings>,
        entry: &RootHomeExitEntry,
        cleanup: &[(BasicBlockId, MirInstruction)],
    ) -> Result<(), String> {
        self.validate_call_entry_storage(owner, exit, function, finishing, None, entry, cleanup)
    }

    /// Root Call storage has already been rebound; its packet keeps original coordinates.
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn validate_rebound_call_entry(
        &self,
        owner: FunctionOwnerIdV1,
        exit: &SourceStmtSiteV1,
        function: &MirFunction,
        original_projection: &super::super::super::physical_boundary::FinishedBindings,
        entry: &RootHomeExitEntry,
        cleanup: &[(BasicBlockId, MirInstruction)],
    ) -> Result<(), String> {
        self.validate_call_entry_storage(
            owner,
            exit,
            function,
            None,
            Some(original_projection),
            entry,
            cleanup,
        )
    }

    fn validate_call_entry_storage(
        &self,
        owner: FunctionOwnerIdV1,
        exit: &SourceStmtSiteV1,
        function: &MirFunction,
        finishing: Option<&super::super::super::physical_boundary::FinishedBindings>,
        packet_finishing: Option<&super::super::super::physical_boundary::FinishedBindings>,
        entry: &RootHomeExitEntry,
        cleanup: &[(BasicBlockId, MirInstruction)],
    ) -> Result<(), String> {
        let Some(source) = self.verified_terminal_call_source_v1(owner, exit)? else {
            use crate::mir::resolved_semantics::home_new_prefix::{
                ObjectReturnAcquisitionV1, TerminalRelationV1, TerminalReturnedSourceV1,
            };
            if matches!(self.terminal_relation_for_owner_at(owner, exit),
                Some(TerminalRelationV1::Value(value))
                    if matches!(value.returned(), TerminalReturnedSourceV1::OwnedCall(call)
                        if matches!(call.acquisition(), ObjectReturnAcquisitionV1::Direct { .. })))
            {
                return Err(freeze("direct-result-source-missing"));
            }
            return match entry {
                RootHomeExitEntry::Plain { local_bindings } => {
                    // A source MapGet terminal owes its checked-read entry;
                    // a Plain exit can never carry it.
                    if self
                        .terminal_map_get_return_for_owner_at(owner, exit)
                        .is_some()
                    {
                        return Err(freeze("map-get-entry-missing"));
                    }
                    self.check_local_call_binding_groups(
                        owner,
                        exit,
                        function,
                        finishing,
                        local_bindings,
                    )
                }
                RootHomeExitEntry::Call { .. } => Err(freeze("call-source-missing")),
                RootHomeExitEntry::MapGet {
                    local_bindings,
                    invoke,
                    projection,
                    frame,
                } => self.validate_map_get_entry(
                    owner,
                    exit,
                    function,
                    finishing,
                    local_bindings,
                    invoke,
                    projection,
                    frame,
                    cleanup,
                ),
            };
        };
        let expected_kind = source.result();
        let RootHomeExitEntry::Call {
            local_bindings,
            row,
            arguments,
            invoke,
            projection,
            frame,
        } = entry
        else {
            let RootHomeExitEntry::Plain { local_bindings } = entry else {
                return Err(freeze("call-entry-missing"));
            };
            if source.legacy_terminal().is_none() {
                return Err(freeze("direct-result-entry-missing"));
            }
            self.check_local_call_binding_groups(owner, exit, function, finishing, local_bindings)?;
            return if self.root_instance_call_expected(owner) {
                // The source method is known, but its target result contract
                // is unavailable. Preserve the existing artifact stop rather
                // than manufacturing a direct-call entry.
                Ok(())
            } else {
                Err(freeze("call-entry-missing"))
            };
        };
        let expected = if let RootCallDispositionV1::Lexical(packet) = row {
            let source_arguments = self.validate_lexical_terminal_packet(owner, exit, packet)?;
            let mut recorded = arguments.clone();
            recorded.extend([invoke.clone(), projection.clone()]);
            packet.validate_recorded_projected(&recorded, packet_finishing)?;
            let (original_invoke, original_projection) = packet.outer_bindings();
            let mapped = |original: &(BasicBlockId, MirInstruction)| match packet_finishing {
                Some(finishing) => finishing
                    .binding(original.0, &original.1)?
                    .ok_or_else(|| freeze("lexical-terminal/original-outer-drift")),
                None => Ok(original.clone()),
            };
            if mapped(original_invoke)? != *invoke || mapped(original_projection)? != *projection {
                return Err(freeze("lexical-terminal/original-outer-drift"));
            }
            packet.call_with_ledger(owner, &source_arguments, self)?
        } else {
            let terminal = source
                .legacy_terminal()
                .ok_or_else(|| freeze("direct-result-legacy-row"))?;
            let row_argument_count = match row {
                RootCallDispositionV1::Direct(row) => row.argument_sites().len(),
                RootCallDispositionV1::Instance(row) => row.argument_sites().len(),
                RootCallDispositionV1::Lexical(_) => {
                    return Err(freeze("lexical-terminal/strict-row"))
                }
            };
            if arguments.len() != terminal.arguments().len()
                || row_argument_count != arguments.len()
            {
                return Err(freeze("call-argument-count"));
            }
            let mut values = Vec::with_capacity(arguments.len());
            for (ordinal, ((_, instruction), argument)) in
                arguments.iter().zip(terminal.arguments()).enumerate()
            {
                match (instruction, argument) {
                (
                    MirInstruction::Const {
                        dst,
                        value: ConstValue::Integer(value),
                    },
                    crate::mir::resolved_semantics::home_new_prefix::TerminalCallArgumentV1::I64(
                        literal,
                    ),
                ) if value == literal => values.push(*dst),
                (
                    MirInstruction::InvokeNormalResult { dst, .. },
                    crate::mir::resolved_semantics::home_new_prefix::TerminalCallArgumentV1::Map(
                        site,
                    ),
                ) => {
                    // The recorded projection must produce the exact lease
                    // the sealed CallArgument flow row emitted into this
                    // argument slot — never a re-read of the map site.
                    let rows = self.local_commits.borrow();
                    let Some(LocalCommitV1::Map(row)) = rows.get(site) else {
                        return Err(freeze("call-argument-drift"));
                    };
                    let flow = self.map_flow(site)?;
                    if site.owner() != owner
                        || row.binding.is_some()
                        || row.emitted_value() != Some(*dst)
                        || !matches!(
                            flow.destination(),
                            crate::mir::resolved_semantics::home_new_prefix::MapDestinationV1::CallArgument {
                                call,
                                ordinal: expected,
                            } if call.site() == terminal.call_site() && *expected == ordinal as u32
                        )
                    {
                        return Err(freeze("call-argument-drift"));
                    }
                    values.push(*dst);
                }
                _ => return Err(freeze("call-argument-drift")),
            }
            }
            let typed: Vec<_> = values
            .into_iter()
            .zip(terminal.arguments())
            .map(|(value, argument)| {
                let class = match argument {
                    crate::mir::resolved_semantics::home_new_prefix::TerminalCallArgumentV1::I64(_) => {
                        crate::mir::resolved_semantics::ExactCallableParamAbiV1::I64
                    }
                    crate::mir::resolved_semantics::home_new_prefix::TerminalCallArgumentV1::Map(_) => {
                        crate::mir::resolved_semantics::ExactCallableParamAbiV1::Map
                    }
                    crate::mir::resolved_semantics::home_new_prefix::TerminalCallArgumentV1::Lexical(_) => return Err(freeze("call-lexical-argument")),
                };
                Ok((value, class))
            })
            .collect::<Result<_, String>>()?;
            match row {
                RootCallDispositionV1::Direct(row) => row
                    .physical_emission()
                    .materialize_call_typed(None, typed)
                    .map_err(|_| freeze("call-projection-failed"))?,
                RootCallDispositionV1::Lexical(_) => {
                    return Err(freeze("lexical-terminal/strict-row"))
                }
                RootCallDispositionV1::Instance(row) => {
                    if !typed.is_empty() || row.argument_sites().iter().next().is_some() {
                        return Err(freeze("instance-call-arguments"));
                    }
                    let receiver = self.resolve_instance_receiver(owner, row)?;
                    let MirInstruction::Invoke {
                        operation:
                            InvokeOperation::Call {
                                call,
                                result: InvokeCallResultKind::I64,
                            },
                        ..
                    } = &invoke.1
                    else {
                        return Err(freeze("instance-call-shape"));
                    };
                    if let crate::mir::definitions::Callee::SameModuleInstance {
                        receiver: actual_receiver,
                        ..
                    } = &call.callee
                    {
                        if *actual_receiver != receiver {
                            return Err(freeze("instance-call-receiver"));
                        }
                    }
                    if *call
                        != crate::mir::definitions::MirCall::new(
                            None,
                            crate::mir::definitions::Callee::SameModuleInstance {
                                key: row.target().clone(),
                                receiver,
                            },
                            Vec::new(),
                        )
                    {
                        return Err(freeze("instance-call-target"));
                    }
                    call.clone()
                }
            }
        };
        if !matches!(&invoke.1, MirInstruction::Invoke {
            operation: InvokeOperation::Call { call, result }, ..
        } if *call == expected && *result == expected_kind)
        {
            return Err(freeze("call-target-drift"));
        }
        // The original root completion selects the owned frame at entry;
        // child callables borrow that same invocation frame through their ABI.
        let expected_mode = if self.root_owner() == Some(owner) {
            crate::mir::instruction::FaultFrameMode::RootOwned
        } else {
            crate::mir::instruction::FaultFrameMode::Borrowed
        };
        if !matches!((&invoke.1, &frame.1), (MirInstruction::Invoke { fault_frame, .. },
            MirInstruction::FaultFrameEnter { dst, mode })
                if fault_frame == dst && *mode == expected_mode)
        {
            return Err(freeze("call-frame-drift"));
        }
        for (id, instruction) in arguments.iter().chain([invoke, projection, frame]) {
            if !super::super::super::physical_boundary::check_binding(
                function,
                finishing,
                *id,
                instruction,
            )? {
                return Err(freeze("call-binding-drift"));
            }
        }
        self.check_local_call_binding_groups(owner, exit, function, finishing, local_bindings)?;
        let mapped = |binding: &(BasicBlockId, MirInstruction)| match finishing {
            Some(p) => p
                .binding(binding.0, &binding.1)?
                .ok_or_else(|| freeze("call-binding-missing")),
            None => Ok(binding.clone()),
        };
        let cleanup = match finishing {
            Some(p) => p.bindings(cleanup)?,
            None => cleanup.to_vec(),
        };
        super::super::super::root_cleanup_graph::call::ingress(
            function,
            &cleanup,
            &mapped(invoke)?,
            &mapped(projection)?,
            expected_kind,
        )?;
        Ok(())
    }

    fn resolve_instance_receiver(
        &self,
        owner: FunctionOwnerIdV1,
        row: &crate::mir::normal_callable_semantic_package::RootInstanceCallDispositionRowV1,
    ) -> Result<ValueId, String> {
        if row.receiver_initializer().owner() != owner || row.receiver_binding().owner() != owner {
            return Err(freeze("instance-call-receiver-owner"));
        }
        let commits = self.local_commits.borrow();
        let local = commits
            .get(row.receiver_initializer())
            .ok_or_else(|| freeze("instance-call-new-local-missing"))?;
        if !local.installs(row.receiver_binding()) {
            return Err(freeze("instance-call-new-local-binding"));
        }
        let object = local
            .ordinary_object()
            .ok_or_else(|| freeze("instance-call-new-local-kind"))?;
        if object != row.receiver_object() {
            return Err(freeze("instance-call-receiver-object"));
        }
        local
            .local()
            .ok_or_else(|| freeze("instance-call-new-local-value"))
    }

    #[cfg(test)]
    pub(crate) fn resolve_root_instance_receiver_for_test(
        &self,
        owner: FunctionOwnerIdV1,
        row: &crate::mir::normal_callable_semantic_package::RootInstanceCallDispositionRowV1,
        actual: ValueId,
    ) -> Result<(), String> {
        if self.resolve_instance_receiver(owner, row)? != actual {
            return Err(freeze("instance-call-receiver"));
        }
        Ok(())
    }
}
