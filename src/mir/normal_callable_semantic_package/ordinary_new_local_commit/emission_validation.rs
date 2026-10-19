//! Validation of recorded physical New emission remains owned by local commit.

use super::*;
use crate::mir::instruction::InvokeCallResultKind;
#[path = "ordinary_new_emission_row.rs"]
mod ordinary_row;

impl OrdinaryNewClaimLedgerV1 {
    pub(crate) fn validate_new_emissions(
        &self,
        owner: FunctionOwnerIdV1,
        function: &MirFunction,
    ) -> Result<(), String> {
        self.validate_new_emissions_view(owner, function, None, None)
    }

    pub(super) fn validate_new_emissions_with_pending(
        &self,
        owner: FunctionOwnerIdV1,
        function: &MirFunction,
        pending: &super::PendingSelectedStaticLoopViewV1<'_, '_>,
    ) -> Result<(), String> {
        self.validate_new_emissions_view(owner, function, None, Some(pending))
    }

    pub(super) fn validate_new_emissions_projected(
        &self,
        owner: FunctionOwnerIdV1,
        function: &MirFunction,
        projection: Option<&super::physical_boundary::FinishedBindings>,
    ) -> Result<(), String> {
        self.validate_new_emissions_view(owner, function, projection, None)
    }

    fn validate_new_emissions_view(
        &self,
        owner: FunctionOwnerIdV1,
        function: &MirFunction,
        projection: Option<&super::physical_boundary::FinishedBindings>,
        pending: Option<&super::PendingSelectedStaticLoopViewV1<'_, '_>>,
    ) -> Result<(), String> {
        let intervals = super::read_argument::IntervalViewV1::new(function);
        for (site, row) in self
            .local_commits
            .borrow()
            .iter()
            .filter(|(_, row)| row.owner() == owner)
        {
            match row {
                LocalCommitV1::TextResidence(packet) => {
                    self.validate_text_residence_home_v1(packet, CallValidationContextV1::Lowering(self))?;
                    packet.validate_paths(CallValidationContextV1::Lowering(self), function, projection)?;
                    continue;
                }
                LocalCommitV1::OwnedArray(_) => {
                    self.validate_owned_array_construction(site, function, projection)?;
                    continue;
                }
                LocalCommitV1::Map(_) => {
                    self.validate_map_emission(site, function, projection)?;
                    continue;
                }
                LocalCommitV1::CallReceived(_) => {
                    self.validate_call_received_emission_with_intervals_v1(site, &intervals, projection, pending,
                        CallValidationContextV1::Lowering(self))?;
                    continue;
                }
                LocalCommitV1::Ordinary(_) | LocalCommitV1::Result(_) => {}
            }
            self.validate_ordinary_new_row_emission_projected(owner, site, row, function, projection)?;
        }
        Ok(())
    }

    /// A received-call local emits exactly one `Invoke{Call}` whose normal
    /// result the receiving local keeps, and the caller's terminal cleanup
    /// discharges that value through the release the commit sealed: a
    /// definite `Handle` result owes one unconditional `HomeRelease`, while
    /// a `Nullable` result may carry the `Void` sentinel and owes exactly
    /// one checked `HomeReleaseIfLive` — never an unconditional release.
    pub(super) fn validate_call_received_emission_in_context_v1(
        &self, site: &OwnedExprSiteV1, function: &MirFunction,
        projection: Option<&super::physical_boundary::FinishedBindings>,
        pending: Option<&super::PendingSelectedStaticLoopViewV1<'_, '_>>,
        context: CallValidationContextV1<'_>,
    ) -> Result<(), String> {
        let intervals = super::read_argument::IntervalViewV1::new(function);
        self.validate_call_received_emission_with_intervals_v1(site, &intervals, projection, pending, context)
    }

    fn validate_call_received_emission_with_intervals_v1(
        &self, site: &OwnedExprSiteV1, intervals: &super::read_argument::IntervalViewV1<'_>,
        projection: Option<&super::physical_boundary::FinishedBindings>,
        pending: Option<&super::PendingSelectedStaticLoopViewV1<'_, '_>>,
        context: CallValidationContextV1<'_>,
    ) -> Result<(), String> {
        let function = intervals.function();
        context.require_ledger(self)?;
        let rows = self.local_commits.borrow();
        let Some(LocalCommitV1::CallReceived(row)) = rows.get(site) else {
            return Err(freeze("handle-progress-missing"));
        };
        let CallReceivedProgress::Emitted {
            result,
            bindings,
            packet,
            ..
        } = &row.progress
        else {
            return Err(freeze("handle-emission-incomplete"));
        };
        let packet_arguments = if let Some(packet) = packet {
            if packet.call_site() != site {
                return Err(freeze("receiver-object/record-source-drift"));
            }
            let arguments = if matches!(row.release, CallReceivedReleaseV1::Mixed { .. }) {
                self.checked_mixed_received_packet_in_context_v1(site, row, packet, context)?
            } else {
                self.object_packet_arguments_v1(packet.original_row()?)?
            };
            if packet.value_in_context_v1(site.owner(), arguments, context)? != *result {
                return Err(freeze("receiver-object/record-result-drift"));
            }
            packet.validate_recorded(bindings)?;
            Some(arguments)
        } else {
            if matches!(row.release, CallReceivedReleaseV1::Mixed { .. }) {
                return Err(freeze("mixed-call/record-packet-missing"));
            }
            if self.receiver_call_observation(site).is_some()
                && self
                    .lifecycle_local_call_sites
                    .borrow()
                    .get(&site.owner())
                    .is_some_and(|sites| sites.contains(site))
            {
                return Err(freeze("receiver-object/record-packet-missing"));
            }
            None
        };
        // Flow membership and argument evidence share one sealed edge:
        // Handle rows carry literal arguments on the flow row itself, while
        // the Nullable row's typed arguments live on the receiver-call
        // observation — the expected arity comes from whichever authority
        // sealed the class.
        let (call_destination, expected_arity) = match row.release {
            CallReceivedReleaseV1::Mixed { .. } => {
                let observation = self.mixed_call_source_v1(site).ok_or_else(|| freeze("mixed-call/observation-missing"))?;
                (observation.local_binding().ok_or_else(|| freeze("mixed-call/destination-missing"))?.1,
                    packet_arguments.ok_or_else(|| freeze("mixed-call/record-packet-missing"))?.len())
            }
            CallReceivedReleaseV1::Handle => {
                let call = self
                    .handle_call_source(site)
                    .ok_or_else(|| freeze("handle-call-source-missing"))?;
                (
                    call.local_binding()
                        .ok_or_else(|| freeze("handle-call-destination-drift"))?
                        .1,
                    packet_arguments
                        .map_or_else(|| call.arguments().len(), |arguments| arguments.len()),
                )
            }
            CallReceivedReleaseV1::Nullable => {
                let call = self
                    .nullable_call_source(site)
                    .ok_or_else(|| freeze("nullable-call-source-missing"))?;
                // `me.` receiver calls keep their typed-argument evidence
                // on the package observation; lexical `local x = recv.m(..)`
                // calls seal the same arguments on the flow row itself —
                // exactly one of the two shapes seals each site, so the
                // sealed one names the expected arity.
                let arity = packet_arguments.map_or_else(
                    || {
                        self.receiver_call_observation(site).map_or_else(
                            || call.arguments().len(),
                            |observation| observation.arguments().len(),
                        )
                    },
                    |arguments| arguments.len(),
                );
                (
                    call.local_binding()
                        .ok_or_else(|| freeze("nullable-call-destination-drift"))?
                        .1,
                    arity,
                )
            }
        };
        let expected_result_kind = match row.release {
            CallReceivedReleaseV1::Handle => InvokeCallResultKind::Handle,
            CallReceivedReleaseV1::Nullable => InvokeCallResultKind::NullableHandle,
            CallReceivedReleaseV1::Mixed { formal_ordinal } => InvokeCallResultKind::MixedHandle { formal_ordinal },
        };
        if row.binding != call_destination || row.local().is_none() {
            return Err(freeze("handle-local-incomplete"));
        }
        let invokes: Vec<_> = bindings
            .iter()
            .filter(|(_, instruction)| {
                matches!(instruction, MirInstruction::Invoke {
                    operation: InvokeOperation::Call {
                        call: emitted,
                        result: emitted_result,
                    },
                    ..
                } if *emitted_result == expected_result_kind
                    && emitted.args.len() == expected_arity)
            })
            .collect();
        if invokes.len() != 1 {
            return Err(freeze("handle-call-invoke-drift"));
        }
        if !bindings.iter().any(|(_, instruction)| {
            matches!(instruction, MirInstruction::InvokeNormalResult { invoke_block, dst }
                if *invoke_block == invokes[0].0 && *dst == *result)
        }) {
            return Err(freeze("handle-call-projection-drift"));
        }
        for (block, expected) in bindings {
            if !super::physical_boundary::check_binding(function, projection, *block, expected)? {
                return Err(format!(
                    "{} function={} source={site:?} block={block:?} binding={expected:?}",
                    freeze("emission-binding-drift"),
                    function.signature.name,
                ));
            }
        }
        // An owned object whose residences were never proven has no
        // teardown plan at all; emitting one is a contract violation,
        // not a plain release.
        if matches!(row.end_children, Some(None)) {
            return Err(freeze("handle-release-unproven"));
        }
        if let Some(packet) = packet {
            let operations: Vec<_> = row.end_plan().into_vec().into_iter().map(|(_, operation)| operation).collect();
            let mut ends = Vec::new();
            for (id, block) in &function.blocks {
                for instruction in block.all_instructions() {
                    if matches!(instruction, MirInstruction::Invoke { operation, .. } if operations.contains(operation)) {
                        ends.push((*id, instruction.clone()));
                    }
                }
            }
            if !matches!(row.release, CallReceivedReleaseV1::Mixed { .. })
                && packet.validate_received_field_support_v1(context, Some((intervals, projection, &ends)))? != row.field_support_parents {
                return Err(freeze("field-end/final-support-drift"));
            }
        } else if row.field_support_parents.is_some() { return Err(freeze("field-end/packet-missing")); }
        // The binding owes every operation of its sealed teardown plan —
        // owned field residences then the Home release — once per exit
        // where it stays live, plus once per recorded fault-unwind chain
        // that discharges it. The owed count comes from recorded
        // emissions — never from recounting MIR shape.
        let shared_received = self.shared_received_call_sites_v1(row.owner, context, projection)?;
        for (_, operation) in row.end_plan().iter() {
            let actual = function
                .blocks
                .values()
                .flat_map(|block| block.all_instructions())
                .filter(|instruction| {
                    matches!(
                        instruction,
                        MirInstruction::Invoke {
                            operation: emitted,
                            ..
                        } if emitted == operation
                    )
                })
                .count();
            let mut owed = 0usize;
            context.visit_exit_bindings_v1(row.owner, |bindings| {
                owed += bindings.iter().filter(|(_, emitted)| matches!(emitted,
                    MirInstruction::Invoke { operation: emitted, .. } if emitted == operation)).count();
            })?;
            if let Some(view) = pending {
                owed += view
                    .roots()
                    .into_iter()
                    .filter(|root| root.owner() == row.owner)
                    .flat_map(|root| root.bindings().iter())
                    .filter(|(_, emitted)| {
                        matches!(
                            emitted,
                            MirInstruction::Invoke {
                                operation: emitted,
                                ..
                            } if emitted == operation
                        )
                    })
                    .count();
            }
            owed += rows
                .iter()
                .filter(|(site, other)| other.owner() == row.owner && !shared_received.contains(site))
                .flat_map(|(_, other)| match other {
                    LocalCommitV1::Ordinary(_) | LocalCommitV1::Result(_) => {
                        match other.new_emission() {
                            Some(NewEmissionProgress::Emitted { bindings, .. }) => {
                                bindings.as_slice()
                            }
                            _ => &[],
                        }
                    }
                    LocalCommitV1::CallReceived(other) => match &other.progress {
                        CallReceivedProgress::Emitted { bindings, .. } => bindings.as_slice(),
                        _ => &[],
                    },
                    LocalCommitV1::Map(other) => other.emitted_bindings(),
                    LocalCommitV1::OwnedArray(other) => other.emitted_bindings(),
                    LocalCommitV1::TextResidence(_) => &[],
                })
                .filter(|(_, instruction)| {
                    matches!(
                        instruction,
                        MirInstruction::Invoke {
                            operation: emitted,
                            ..
                        } if emitted == operation
                    )
                })
                .count();
            // A recorded lexical-call binding group's fault-unwind chain
            // discharges the binding exactly like a commit row's own
            // emitted unwind — both are recorded emissions, never
            // recounts of MIR shape.
            owed += context.with_local_call_groups_v1(row.owner, |groups| Ok(groups.iter()
                .flat_map(|group| group.bindings())
                .filter(|(_, instruction)| matches!(instruction,
                    MirInstruction::Invoke { operation: emitted, .. } if emitted == operation)).count()))?;
            // A release of the wrong shape is just as fatal as a missing
            // one: unconditional `HomeRelease` on a `Void`-carrying value
            // would release the sentinel, and an extra field-residence op
            // would double-free.
            if actual != owed {
                return Err(freeze(if actual < owed {
                    match row.release {
                        CallReceivedReleaseV1::Handle => "handle-release-missing",
                        CallReceivedReleaseV1::Nullable => "nullable-release-missing",
                        CallReceivedReleaseV1::Mixed { .. } => "mixed-release-missing",
                    }
                } else {
                    "handle-release-shape-drift"
                }));
            }
        }
        Ok(())
    }

    fn validate_argument_definition(
        &self,
        function: &MirFunction,
        source: &super::super::OrdinaryNewTrivialArgumentV1,
        value: crate::mir::ValueId,
    ) -> Result<(), String> {
        use super::super::OrdinaryNewTrivialArgumentKindV1;

        let matching = match source.kind() {
            OrdinaryNewTrivialArgumentKindV1::Integer(expected) => function
                .blocks
                .values()
                .flat_map(|block| block.all_instructions())
                .filter(|instruction| {
                    matches!(instruction, MirInstruction::Const {
                        dst,
                        value: crate::mir::ConstValue::Integer(actual),
                    } if *dst == value && actual == expected)
                })
                .count(),
            OrdinaryNewTrivialArgumentKindV1::Bool(expected) => function
                .blocks
                .values()
                .flat_map(|block| block.all_instructions())
                .filter(|instruction| {
                    matches!(instruction, MirInstruction::Const {
                        dst,
                        value: crate::mir::ConstValue::Bool(actual),
                    } if *dst == value && actual == expected)
                })
                .count(),
            OrdinaryNewTrivialArgumentKindV1::Null => function
                .blocks
                .values()
                .flat_map(|block| block.all_instructions())
                .filter(|instruction| {
                    matches!(instruction, MirInstruction::Const {
                        dst,
                        value: crate::mir::ConstValue::Null,
                    } if *dst == value)
                })
                .count(),
            // `QualifiedStaticCall` emits one `Call{Global, I64}` invoke
            // whose normal landing projects `value` through
            // `InvokeNormalResult`. Validation re-walks the chain against
            // the sealed row: exactly one invoke carries the sealed global
            // target with literal-only actuals in source order, and its
            // normal landing owns the projection.
            OrdinaryNewTrivialArgumentKindV1::QualifiedStaticCall { target, arguments } => {
                let global = target
                    .canonical_global_target_v1()
                    .map_err(|_| freeze("argument-call-target"))?;
                let matching =
                    function
                        .blocks
                        .iter()
                        .filter(|(block_id, block)| {
                            let Some(crate::mir::MirInstruction::Invoke {
                                operation:
                                    crate::mir::instruction::InvokeOperation::Call {
                                        call,
                                        result: crate::mir::instruction::InvokeCallResultKind::I64,
                                    },
                                normal_landing,
                                ..
                            }) = block.terminator.as_ref()
                            else {
                                return false;
                            };
                            if call.dst.is_some()
                                || call.callee != crate::mir::Callee::Global(global.clone())
                                || call.args.len() != arguments.len()
                            {
                                return false;
                            }
                            let literals_match = call.args.iter().zip(arguments.iter()).all(
                                |(argument_value, kind)| {
                                    let expected = match kind {
                                        super::super::QualifiedStaticCallArgumentKindV1::Integer(
                                            literal,
                                        ) => crate::mir::ConstValue::Integer(*literal),
                                        super::super::QualifiedStaticCallArgumentKindV1::Bool(
                                            literal,
                                        ) => crate::mir::ConstValue::Bool(*literal),
                                    };
                                    function
                                        .blocks
                                        .values()
                                        .flat_map(|candidate| candidate.all_instructions())
                                        .filter(|instruction| {
                                            matches!(
                                                instruction,
                                                crate::mir::MirInstruction::Const {
                                                    dst,
                                                    value: actual,
                                                } if dst == argument_value && *actual == expected
                                            )
                                        })
                                        .count()
                                        == 1
                                },
                            );
                            if !literals_match {
                                return false;
                            }
                            function
                                .blocks
                                .get(normal_landing)
                                .map(|landing| {
                                    landing
                                        .all_instructions()
                                        .filter(|instruction| {
                                            matches!(
                                                instruction,
                                                crate::mir::MirInstruction::InvokeNormalResult {
                                                    dst,
                                                    invoke_block,
                                                } if *dst == value && invoke_block == *block_id
                                            )
                                        })
                                        .count()
                                        == 1
                                })
                                .unwrap_or(false)
                        })
                        .count();
                if matching != 1 {
                    return Err(freeze("argument-call-drift"));
                }
                return Ok(());
            }
            // `I64Field` emits an `ObjectFieldGet` — the argument field-read
            // ledger, not a literal shape, validates that emission.
            OrdinaryNewTrivialArgumentKindV1::Local { .. }
            | OrdinaryNewTrivialArgumentKindV1::Handle { .. }
            | OrdinaryNewTrivialArgumentKindV1::BoundValue { .. }
            | OrdinaryNewTrivialArgumentKindV1::I64Field { .. } => return Ok(()),
        };
        if matching != 1 {
            return Err(freeze("argument-literal-drift"));
        }
        Ok(())
    }
}

#[cfg(test)]
#[path = "reclaim_cleanup_tests.rs"]
mod reclaim_cleanup_tests;
