//! Same ordinary/result New validation used before and after handoff.
//! Other local-commit families retain their own pool and validation protocols.
use super::*;
use crate::mir::normal_callable_semantic_package::instance_construction::{new_execution_profile_v1, NewExecutionProfileRefV1};

impl OrdinaryNewClaimLedgerV1 {
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn validate_ordinary_new_row_emission_projected(
        &self, owner: FunctionOwnerIdV1, site: &OwnedExprSiteV1, row: &LocalCommitV1,
        function: &MirFunction,
        projection: Option<&super::super::physical_boundary::FinishedBindings>,
        context: crate::mir::normal_callable_semantic_package::CallValidationContextV1<'_>,
    ) -> Result<(), String> {
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
            _ => return Err(freeze("ordinary-emission/row-kind")),
        };
        let row_emission = match row {
            LocalCommitV1::Ordinary(row) => &row.emission,
            LocalCommitV1::Result(row) => &row.emission,
            LocalCommitV1::Map(_) | LocalCommitV1::CallReceived(_) | LocalCommitV1::OwnedArray(_) | LocalCommitV1::TextResidence(_) => {
                unreachable!("map and call-received rows returned above")
            }
        };
        match row_emission {
            NewEmissionProgress::RetainedUnavailable { .. } => {}
            NewEmissionProgress::Emitted {
                result,
                arguments,
                argument_cleanup,
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
                    if source != &source_arguments[index] || source.owner() != owner
                        || source.new_site() != site
                        || source.ordinal()
                            != u32::try_from(index)
                                .map_err(|_| freeze("argument-ordinal-overflow"))?
                    {
                        return Err(freeze("argument-source-drift"));
                    }
                    emitted.validate_field_array_read(function, projection)?;
                    if let Some(expression) = emitted.expression_v1() {
                        let inherited = crate::mir::normal_callable_semantic_package::ExpressionCallTransientPrefixV1::for_prepared_new_argument_v1(
                            source, &arguments[..index], source_arguments)?;
                        expression.validate_v1(source, emitted.payload(),
                            context, function, projection, Some(&inherited))?;
                    } else { self.validate_argument_definition(function, source, emitted.value_only_v1()?)?; }
                }
                let cleanup_covered = match argument_cleanup {
                    Some(cleanup) => cleanup.validate_v1(context, site, *result, arguments, function, projection)?,
                    None if arguments.iter().all(|argument| argument.expression_v1().is_none()) => std::collections::BTreeSet::new(),
                    None => return Err(freeze("argument-source-unavailable")),
                };
                let allocation = match row {
                    LocalCommitV1::Ordinary(row) => {
                        if row.allocation_execution.as_ref().is_some_and(|lease| !lease.matches_source(&row.construction_source)) {
                            return Err(freeze("new-execution/source-drift"));
                        }
                        row.allocation_execution.as_ref()
                    },
                    _ => None,
                };
                let execution = new_execution_profile_v1(row_construction, allocation, row_object)
                    .map_err(|_| freeze("new-execution/unavailable"))?;
                let expected_reclaim = match (row_birth_target, &execution) {
                    (None, NewExecutionProfileRefV1::Initialized(plan)) if plan.constructor().is_none() => None,
                    (None, NewExecutionProfileRefV1::AllocationOnly(lease)) if lease.source().constructor().is_none() => None,
                    (Some(_), NewExecutionProfileRefV1::Initialized(plan)) => {
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
                        if !super::super::physical_boundary::check_binding(
                            function,
                            projection,
                            emitted.block,
                            &emitted.instruction,
                        )? {
                            return Err(freeze("reclaim-origin-binding-drift"));
                        }
                        // These selected New bindings may reclaim storage,
                        // but never repeat Birth's field-residence discharge.
                        if bindings.iter().any(|(block, instruction)| !cleanup_covered.contains(&projection.and_then(|boundary| boundary.destination(*block)).unwrap_or(*block)) && matches!(instruction,
                            MirInstruction::Invoke {
                                operation: crate::mir::instruction::InvokeOperation::OwnedFieldResidenceRelease { base, .. }
                                    | crate::mir::instruction::InvokeOperation::OwnedObjectFieldRelease { base, .. },
                                ..
                            } if base == result))
                        {
                            return Err(freeze("reclaim-duplicate-field-cleanup"));
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
                            operation: crate::mir::instruction::InvokeOperation::Birth(call), ..
                        } if call.receiver == *result && row_birth_target.as_ref() == Some(&call.key)
                            && call.arguments.iter().eq(arguments.iter().map(|argument| argument.payload()))
                    ))
                    .count();
                let expected_birth_calls = usize::from(row_birth_target.is_some());
                if birth_calls != expected_birth_calls {
                    return Err(freeze("argument-call-drift"));
                }
                for (block, expected) in bindings {
                    if !super::super::physical_boundary::check_binding(
                        function, projection, *block, expected,
                    )? {
                        return Err(format!(
                            "{} function={} source={site:?} block={block:?} binding={expected:?}",
                            freeze("emission-binding-drift"),
                            function.signature.name,
                        ));
                    }
                }
            }
            _ => return Err(freeze("emission-residual")),
        }
        Ok(())
    }
}
