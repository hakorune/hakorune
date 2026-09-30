//! Validation of recorded physical New emission remains owned by local commit.

use super::*;
use crate::mir::instruction::InvokeCallResultKind;

impl OrdinaryNewClaimLedgerV1 {
    pub(crate) fn validate_new_emissions(
        &self,
        owner: FunctionOwnerIdV1,
        function: &MirFunction,
    ) -> Result<(), String> {
        self.validate_new_emissions_projected(owner, function, None)
    }

    pub(super) fn validate_new_emissions_projected(
        &self,
        owner: FunctionOwnerIdV1,
        function: &MirFunction,
        projection: Option<&super::physical_boundary::FinishedBindings>,
    ) -> Result<(), String> {
        for (site, row) in self
            .local_commits
            .borrow()
            .iter()
            .filter(|(_, row)| row.owner() == owner)
        {
            let (row_object, row_birth_target, row_construction, row_argument_rows) = match row {
                LocalCommitV1::Ordinary(row) => (
                    row.object,
                    &row.birth_target,
                    &row.construction,
                    &row.argument_rows,
                ),
                LocalCommitV1::Result(row) => (
                    row.object,
                    &row.birth_target,
                    &row.construction,
                    &row.argument_rows,
                ),
                LocalCommitV1::Map(_) => {
                    self.validate_map_emission(site, function, projection)?;
                    continue;
                }
                LocalCommitV1::CallReceived(_) => {
                    self.validate_call_received_emission(site, function, projection)?;
                    continue;
                }
            };
            let row_emission = match row {
                LocalCommitV1::Ordinary(row) => &row.emission,
                LocalCommitV1::Result(row) => &row.emission,
                LocalCommitV1::Map(_) | LocalCommitV1::CallReceived(_) => {
                    unreachable!("map and call-received rows returned above")
                }
            };
            match row_emission {
                NewEmissionProgress::RetainedUnavailable { .. } => {}
                NewEmissionProgress::Emitted {
                    result,
                    arguments,
                    reclaim,
                    bindings,
                    ..
                } => {
                    if let LocalCommitV1::Result(_) = row {
                        // Result exit contract: the emitted object identity
                        // reaches `Return { value }` exactly once — this is
                        // the transfer edge the claim stands for.
                        let returns = function
                            .blocks
                            .values()
                            .flat_map(|block| block.all_instructions())
                            .filter(|instruction| {
                                matches!(
                                    instruction,
                                    MirInstruction::Return {
                                        value: Some(value),
                                        ..
                                    } if value == result
                                )
                            })
                            .count();
                        if returns != 1 {
                            return Err(freeze("result-return-drift"));
                        }
                    } else {
                        let local = row_emission
                            .local()
                            .ok_or_else(|| freeze("emission-local-result-drift"))?;
                        let copy_valid = match projection {
                            Some(projection) => {
                                projection.check_source_local_copy(function, local, *result)?
                            }
                            None => {
                                let mut copies = function
                                    .blocks
                                    .values()
                                    .flat_map(|block| block.all_instructions())
                                    .filter(|instruction| {
                                        matches!(instruction, MirInstruction::Copy { dst, .. } if *dst == local)
                                    });
                                matches!(copies.next(), Some(MirInstruction::Copy { src, .. }) if src == result)
                                    && copies.next().is_none()
                            }
                        };
                        if !copy_valid {
                            return Err(freeze("emission-local-copy-drift"));
                        }
                    }
                    let source_arguments = row_argument_rows
                        .as_ref()
                        .map_err(|_| freeze("argument-source-unavailable"))?;
                    if source_arguments.len() != arguments.len() {
                        return Err(freeze("argument-count-drift"));
                    }
                    for (index, emitted) in arguments.iter().enumerate() {
                        let source = &emitted.source;
                        if source.owner() != owner
                            || source.new_site() != site
                            || source.ordinal()
                                != u32::try_from(index)
                                    .map_err(|_| freeze("argument-ordinal-overflow"))?
                        {
                            return Err(freeze("argument-source-drift"));
                        }
                        self.validate_argument_definition(function, source, emitted.value)?;
                    }
                    let expected_reclaim = match (row_birth_target, row_construction) {
                        (None, Ok(plan)) if plan.constructor().is_none() => None,
                        (Some(_), Ok(plan)) => {
                            let (constructor_source, constructor_owner) = plan
                                .constructor()
                                .ok_or_else(|| freeze("reclaim-origin-constructor-missing"))?;
                            if !plan.reclaims_unpublished_outer_storage()
                                || plan.object() != row_object
                            {
                                return Err(freeze("reclaim-origin-source-drift"));
                            }
                            Some((constructor_source, constructor_owner))
                        }
                        _ => return Err(freeze("reclaim-origin-source-drift")),
                    };
                    match (expected_reclaim, reclaim) {
                        (None, None) => {}
                        (Some((constructor_source, constructor_owner)), Some(emitted)) => {
                            if emitted.origin.site != *site
                                || emitted.origin.object != row_object
                                || !emitted
                                    .origin
                                    .constructor_source
                                    .same_as(constructor_source)
                                || emitted.origin.constructor_owner != *constructor_owner
                            {
                                return Err(freeze("reclaim-origin-source-drift"));
                            }
                            if !matches!(
                                emitted.instruction,
                                MirInstruction::Invoke {
                                    operation: crate::mir::instruction::InvokeOperation::ReclaimUnpublished {
                                        object,
                                        value,
                                    },
                                    ..
                                } if object == emitted.origin.object && value == *result
                            ) {
                                return Err(freeze("reclaim-origin-operation-drift"));
                            }
                            let matching = function
                                .blocks
                                .values()
                                .flat_map(|block| block.all_instructions())
                                .filter(|actual| matches!(
                                    actual,
                                    MirInstruction::Invoke {
                                        operation: crate::mir::instruction::InvokeOperation::ReclaimUnpublished {
                                            object,
                                            value,
                                        },
                                        ..
                                    } if *object == emitted.origin.object && *value == *result
                                ))
                                .count();
                            if matching != 1 {
                                return Err(freeze(if matching == 0 {
                                    "reclaim-origin-operation-drift"
                                } else {
                                    "reclaim-origin-duplicate"
                                }));
                            }
                            if !super::physical_boundary::check_binding(
                                function,
                                projection,
                                emitted.block,
                                &emitted.instruction,
                            )? {
                                return Err(freeze("reclaim-origin-binding-drift"));
                            }
                        }
                        _ => return Err(freeze("reclaim-origin-presence-drift")),
                    }
                    let birth_calls = function
                        .blocks
                        .values()
                        .flat_map(|block| block.all_instructions())
                        .filter(|instruction| matches!(
                            instruction,
                            MirInstruction::Invoke {
                                operation: crate::mir::instruction::InvokeOperation::Call { call, result: InvokeCallResultKind::Unit },
                                ..
                            } if matches!(&call.callee,
                                crate::mir::Callee::BirthConstructor { receiver, .. } if receiver == result)
                                && call.args
                                    == arguments
                                        .iter()
                                        .map(|argument| argument.value)
                                        .collect::<Vec<_>>()
                        ))
                        .count();
                    let expected_birth_calls = usize::from(row_birth_target.is_some());
                    if birth_calls != expected_birth_calls {
                        return Err(freeze("argument-call-drift"));
                    }
                    for (block, expected) in bindings {
                        if !super::physical_boundary::check_binding(
                            function, projection, *block, expected,
                        )? {
                            return Err(freeze("emission-binding-drift"));
                        }
                    }
                }
                _ => return Err(freeze("emission-residual")),
            }
        }
        Ok(())
    }

    /// A received-call local emits exactly one `Invoke{Call}` whose normal
    /// result the receiving local keeps, and the caller's terminal cleanup
    /// discharges that value through the release the commit sealed: a
    /// definite `Handle` result owes one unconditional `HomeRelease`, while
    /// a `Nullable` result may carry the `Void` sentinel and owes exactly
    /// one checked `HomeReleaseIfLive` — never an unconditional release.
    fn validate_call_received_emission(
        &self,
        site: &OwnedExprSiteV1,
        function: &MirFunction,
        projection: Option<&super::physical_boundary::FinishedBindings>,
    ) -> Result<(), String> {
        let rows = self.local_commits.borrow();
        let Some(LocalCommitV1::CallReceived(row)) = rows.get(site) else {
            return Err(freeze("handle-progress-missing"));
        };
        // Flow membership and argument evidence share one sealed edge:
        // Handle rows carry literal arguments on the flow row itself, while
        // the Nullable row's typed arguments live on the receiver-call
        // observation — the expected arity comes from whichever authority
        // sealed the class.
        let (call_destination, expected_arity) = match row.release {
            CallReceivedReleaseV1::Handle => {
                let call = self
                    .handle_call_source(site)
                    .ok_or_else(|| freeze("handle-call-source-missing"))?;
                (call.destination(), call.arguments().len())
            }
            CallReceivedReleaseV1::Nullable => {
                let call = self
                    .nullable_call_source(site)
                    .ok_or_else(|| freeze("nullable-call-source-missing"))?;
                let observation = self
                    .receiver_call_observation(site)
                    .ok_or_else(|| freeze("nullable-observation-missing"))?;
                (call.destination(), observation.arguments().len())
            }
        };
        let expected_result_kind = match row.release {
            CallReceivedReleaseV1::Handle => InvokeCallResultKind::Handle,
            CallReceivedReleaseV1::Nullable => InvokeCallResultKind::NullableHandle,
        };
        if row.binding != call_destination || row.local().is_none() {
            return Err(freeze("handle-local-incomplete"));
        }
        let CallReceivedProgress::Emitted {
            result, bindings, ..
        } = &row.progress
        else {
            return Err(freeze("handle-emission-incomplete"));
        };
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
                return Err(freeze("emission-binding-drift"));
            }
        }
        let releases = function
            .blocks
            .values()
            .flat_map(|block| block.all_instructions())
            .filter(|instruction| {
                matches!(
                    instruction,
                    MirInstruction::Invoke {
                        operation: InvokeOperation::HomeRelease { object, value }
                        | InvokeOperation::HomeReleaseIfLive { object, value },
                        ..
                    } if *object == row.object && value == result
                )
            })
            .collect::<Vec<_>>();
        let missing = match row.release {
            CallReceivedReleaseV1::Handle => "handle-release-missing",
            CallReceivedReleaseV1::Nullable => "nullable-release-missing",
        };
        let matching = releases
            .iter()
            .filter(|instruction| match (row.release, instruction) {
                (
                    CallReceivedReleaseV1::Handle,
                    MirInstruction::Invoke {
                        operation: InvokeOperation::HomeRelease { .. },
                        ..
                    },
                )
                | (
                    CallReceivedReleaseV1::Nullable,
                    MirInstruction::Invoke {
                        operation: InvokeOperation::HomeReleaseIfLive { .. },
                        ..
                    },
                ) => true,
                _ => false,
            })
            .count();
        // A release of the wrong shape is just as fatal as a missing one:
        // unconditional `HomeRelease` on a `Void`-carrying value would
        // release the sentinel.
        if releases.len() != 1 || matching != 1 {
            return Err(freeze(if releases.is_empty() {
                missing
            } else {
                "handle-release-shape-drift"
            }));
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
