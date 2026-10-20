//! Emitted-shape validation for construction stores: the retained transport
//! checks and the exact block/terminator census against the physical draft.
//! No source acceptance and no new predicate live here — only comparison of
//! recorded progress rows with the emitted function.

use super::*;
use crate::mir::instruction::InvokeOperation;
use crate::mir::MirInstruction;

impl RetainedConstructionValidation {
    pub(in crate::mir) fn validate_artifact_after_compiler_finishing(
        &self,
        function: &MirFunction,
    ) -> Result<(), String> {
        match &self.construction {
            ConstructionState::Selected {
                completed: true, ..
            } => {}
            ConstructionState::RetainedUnavailable(reason) => {
                return Err(format!(
                    "{} reason={reason:?}",
                    fault("artifact-source-unavailable")
                ))
            }
            _ => return Err(fault("artifact-construction-not-complete")),
        }
        self.validate_after_compiler_finishing(function)
    }

    /// Corroborate the retained original Map Birth assignment against this
    /// SAME emitted construction, then replay every provider/store/Fault edge.
    pub(in crate::mir) fn require_map_field_source_v1(
        &self, owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
        source: &crate::mir::normal_callable_semantic_package::ConstructionStoreV1,
        function: &MirFunction,
    ) -> Result<(), String> {
        let ConstructionState::Selected { stores, .. } = &self.construction
            else { return Err(fault("artifact-construction-not-complete")) };
        let store = stores.get(source.assignment().statement_site().node()).ok_or_else(|| fault("emission-drift"))?;
        if self.owner != owner || store.field != source.field() || store.receiver_site != *source.receiver_site()
            || store.receiver_binding != source.receiver_binding() || store.rhs != *source.rhs()
            || source.mode() != Ok(crate::mir::instruction::FieldSetModeV1::DynamicMapInitialize)
            || store.mode != crate::mir::instruction::FieldSetModeV1::DynamicMapInitialize {
            return Err(fault("emission-drift"));
        }
        self.validate_artifact_after_compiler_finishing(function)
    }

    /// Lend only original source-issued store coordinates after full replay.
    pub(in crate::mir) fn visit_finished_field_writes_v1(
        &self, key: &hakorune_mir_defs::CanonicalSameModuleCallableKeyV1,
        function: &MirFunction,
        mut visit: impl FnMut((BasicBlockId, MirInstruction)) -> Result<(), String>,
    ) -> Result<(), String> {
        if key.namespace() != hakorune_mir_defs::SameModuleCallableNamespaceV1::BirthConstructor
            || function.signature.name != key.mir_symbol_projection() {
            return Err(fault("unit-call-birth-owner"));
        }
        let ConstructionState::Selected { stores, .. } = &self.construction else { return Ok(()); };
        self.validate_artifact_after_compiler_finishing(function)?;
        for store in stores.values() {
            if store.mode == crate::mir::instruction::FieldSetModeV1::ExactSlot { continue; }
            let StoreProgress::Emitted { block, .. } = store.progress else { return Err(fault("store-residual")); };
            let instruction = function.blocks.get(&block).and_then(|body| body.terminator.as_ref())
                .ok_or_else(|| fault("emission-drift"))?;
            visit((block, instruction.clone()))?;
        }
        Ok(())
    }

    pub(in crate::mir::builder) fn validate_after_compiler_finishing(
        &self,
        function: &MirFunction,
    ) -> Result<(), String> {
        self.fault_frame.validate(function)?;
        self.construction.finish()?;
        self.construction
            .validate_bindings(function)
            .map_err(|error| format!("{error} owner={:?}", self.owner))
    }
}

impl ConstructionState {
    pub(super) fn validate_bindings(&self, function: &MirFunction) -> Result<(), String> {
        let ConstructionState::Selected { stores, frame, calls, .. } = self else {
            return Ok(());
        };
        self.validate_step_coverage()?;
        for block in function.blocks.values() {
            for instruction in block.all_instructions() {
                if matches!(instruction, MirInstruction::ObjectFieldGet { .. })
                    && !calls.iter().any(|call| call.owns_exact_field_read(block.id, instruction)) {
                    return Err(fault("unowned-exact-field-read"));
                }
            }
        }

        let provider_count = stores
            .values()
            .filter(|store| {
                matches!(
                    store.rhs,
                    ConstructionStoreRhsV1::ProviderConstruction { .. }
                )
            })
            .count();
        let birth_providers = stores
            .values()
            .filter(|store| {
                matches!(
                    &store.rhs,
                    ConstructionStoreRhsV1::ProviderConstruction {
                        object:
                            Some(
                                crate::mir::normal_callable_semantic_package
                                    ::ProviderConstructionChildV1::BirthIndexed(_),
                            ),
                        ..
                    }
                )
            })
            .count();
        let nobirth_providers = stores
            .values()
            .filter(|store| {
                matches!(
                    &store.rhs,
                    ConstructionStoreRhsV1::ProviderConstruction {
                        object:
                            Some(
                                crate::mir::normal_callable_semantic_package
                                    ::ProviderConstructionChildV1::NoBirthZero(_),
                            ),
                        ..
                    }
                )
            })
            .count();
        let (nested_releases, call_args): (usize, usize) = stores
            .values()
            .map(|store| match &store.progress {
                StoreProgress::Emitted {
                    provider_child:
                        Some(super::ProviderChildEmissionV1::Birth(birth)),
                    ..
                } => (birth.owned_fields.len(), birth.call_args.len()),
                _ => (0, 0),
            })
            .fold((0, 0), |(fields, calls), (f, c)| (fields + f, calls + c));
        let actual_count = function
            .blocks
            .values()
            .flat_map(|block| block.all_instructions())
            .filter(|instruction| matches!(instruction, MirInstruction::Invoke { .. }))
            .count();
        // A `BirthIndexed` provider adds three more invokes on the frame:
        // `birth_call`, the `reclaim_unpublished` on its fault edge, and the
        // `home_release` discharge on the object-field store's fault edge.
        // A `NoBirthZero` provider emits no call — only the `home_release`
        // tail — plus one `field_residence_release` per sealed nested
        // residence on the completed child store-Fault chain, prior-field
        // discharge and one `Call{Global, I64}`
        // per sealed qualified-static argument row. Provided arguments
        // remain caller-owned until Birth Normal and add no release tail.
        if actual_count
            != stores.len()
                + provider_count
                + stores.values().filter(|store| store.mode == crate::mir::instruction::FieldSetModeV1::DynamicMapInitialize).count()
                + 3 * birth_providers
                + nobirth_providers
                + nested_releases
                + stores
                    .values()
                    .map(|store| store.fault_discharge.len())
                    .sum::<usize>()
                + call_args
                + calls.len()
                + calls.iter().map(|call| call.discharge_fields.len()).sum::<usize>()
        {
            return Err(fault("emission-count"));
        }
        let mut fault_returns = 0;
        for block in function.blocks.values() {
            for instruction in block.all_instructions() {
                match instruction {
                    MirInstruction::ReturnFault { fault_frame } => {
                        if *frame != Some((*fault_frame, block.id)) {
                            return Err(fault("fault-return-drift"));
                        }
                        fault_returns += 1;
                    }
                    MirInstruction::InvokeNormalResult { invoke_block, dst } => {
                        // Only a proven provider `new` or a recorded
                        // qualified-static argument call may land a normal
                        // result — the store records each exact pair, and
                        // the projection sits in the matching landing (the
                        // `entry` block for the allocation, the call's own
                        // landing for an argument, the store block for a
                        // builtin provider).
                        let proven = stores.values().any(|store| {
                            let StoreProgress::Emitted {
                                block: home,
                                value,
                                provider: Some(origin),
                                provider_child,
                                ..
                            } = &store.progress
                            else {
                                return false;
                            };
                            match provider_child {
                                Some(super::ProviderChildEmissionV1::Birth(birth)) => {
                                    (birth.entry == block.id
                                        && *origin == *invoke_block
                                        && *value == *dst)
                                        || birth.call_args.iter().any(|arg| {
                                            arg.landing == block.id
                                                && arg.invoke_block == *invoke_block
                                                && arg.value == *dst
                                        })
                                }
                                _ => {
                                    // `NoBirthZero` and builtin providers
                                    // land the allocation projection in the
                                    // store's own block — the same shape as
                                    // `object: None`.
                                    *home == block.id && *origin == *invoke_block && *value == *dst
                                }
                            }
                        });
                        if !proven {
                            return Err(fault("unexpected-normal-result"));
                        }
                    }
                    MirInstruction::Call(call)
                        if matches!(call.callee, crate::mir::Callee::BirthConstructor { .. }) =>
                    {
                        return Err(fault("unowned-birth-call"));
                    }
                    _ => {}
                }
            }
        }
        if fault_returns != usize::from(frame.is_some()) {
            return Err(fault("fault-return-count"));
        }
        for store in stores.values() {
            let field = store.field;
            let progress = &store.progress;
            let StoreProgress::Emitted {
                block,
                normal,
                base,
                value,
                provider,
                provider_child,
                discharge,
            } = progress
            else {
                return Err(fault("store-residual"));
            };
            let Some((frame_id, frame_landing)) = *frame else {
                return Err(fault("emission-state"));
            };
            if field_discharge_chain(function, *discharge, &store.fault_discharge, *base, *frame)
                != Some(frame_landing)
            {
                return Err(fault("store-discharge-chain"));
            }
            let store_ok = match (&store.rhs, provider_child) {
                (
                    ConstructionStoreRhsV1::ProviderConstruction {
                        object:
                            Some(
                                crate::mir::normal_callable_semantic_package
                                    ::ProviderConstructionChildV1::BirthIndexed(child),
                            ),
                        ..
                    },
                    Some(super::ProviderChildEmissionV1::Birth(birth)),
                ) => matches!(function.blocks.get(block).and_then(|b| b.terminator.as_ref()),
                    Some(MirInstruction::Invoke { operation: InvokeOperation::ObjectFieldSet { field: actual, base: b, value: v, child: stored }, fault_frame, fault_landing, normal_landing })
                    if actual == &field && b == base && v == value && stored == child
                        && normal_landing == normal
                        && *fault_landing == birth.store_discharge
                        && frame.is_some_and(|(id, _)| *fault_frame == id)),
                (
                    ConstructionStoreRhsV1::ProviderConstruction {
                        object:
                            Some(
                                crate::mir::normal_callable_semantic_package
                                    ::ProviderConstructionChildV1::NoBirthZero(child),
                            ),
                        ..
                    },
                    Some(super::ProviderChildEmissionV1::NoBirthZero { store_discharge }),
                ) => matches!(function.blocks.get(block).and_then(|b| b.terminator.as_ref()),
                    Some(MirInstruction::Invoke { operation: InvokeOperation::ObjectFieldSet { field: actual, base: b, value: v, child: stored }, fault_frame, fault_landing, normal_landing })
                    if actual == &field && b == base && v == value && stored == child
                        && normal_landing == normal
                        && *fault_landing == *store_discharge
                        && frame.is_some_and(|(id, _)| *fault_frame == id)),
                (
                    ConstructionStoreRhsV1::Parameter {
                        provided: Some(child),
                        ..
                    },
                    None,
                ) => matches!(function.blocks.get(block).and_then(|b| b.terminator.as_ref()),
                    Some(MirInstruction::Invoke { operation: InvokeOperation::ObjectFieldSet { field: actual, base: b, value: v, child: stored }, fault_frame, fault_landing, normal_landing })
                    if actual == &field && b == base && v == value && stored == child
                        && normal_landing == normal
                        && fault_landing == discharge
                        && frame.is_some_and(|(id, _)| *fault_frame == id)),
                _ => matches!(function.blocks.get(block).and_then(|b| b.terminator.as_ref()),
                    Some(MirInstruction::Invoke { operation: InvokeOperation::FieldSet { mode, field: actual, base: b, value: v }, fault_frame, fault_landing, normal_landing })
                    if *mode == store.mode && actual == &field && b == base && v == value
                        && normal_landing == normal && *fault_frame == frame_id
                        && if store.mode == crate::mir::instruction::FieldSetModeV1::DynamicMapInitialize {
                            matches!(function.blocks.get(fault_landing).and_then(|b| b.terminator.as_ref()),
                                Some(MirInstruction::Invoke {
                                    operation: InvokeOperation::Map(crate::mir::instruction::MapInvokeOperation::End { map }),
                                    fault_frame: f, normal_landing: n, fault_landing: e })
                                if map == value && *f == frame_id && n != e
                                    && jump_target(function, *n) == Some(*discharge)
                                    && jump_target(function, *e) == Some(*discharge))
                        } else { fault_landing == discharge }),
            };
            if !store_ok {
                return Err(fault("emission-drift"));
            }
            match (provider, &store.rhs, provider_child) {
                (
                    Some(provider_origin),
                    ConstructionStoreRhsV1::ProviderConstruction {
                        object: None, ..
                    },
                    None,
                ) => {
                    if !matches!(function.blocks.get(provider_origin).and_then(|b| b.terminator.as_ref()),
                        Some(MirInstruction::Invoke {
                            operation,
                            fault_frame,
                            fault_landing,
                            normal_landing,
                        }) if (if store.mode == crate::mir::instruction::FieldSetModeV1::DynamicMapInitialize {
                                *operation == InvokeOperation::Map(crate::mir::instruction::MapInvokeOperation::New)
                            } else { *operation == InvokeOperation::IntrinsicArrayNew })
                            && *normal_landing == *block
                            && *fault_frame == frame_id && fault_landing == discharge)
                    {
                        return Err(fault("provider-emission-drift"));
                    }
                }
                (
                    Some(provider_origin),
                    ConstructionStoreRhsV1::ProviderConstruction {
                        object:
                            Some(
                                crate::mir::normal_callable_semantic_package
                                    ::ProviderConstructionChildV1::NoBirthZero(child),
                            ),
                        ..
                    },
                    Some(super::ProviderChildEmissionV1::NoBirthZero { store_discharge }),
                ) => {
                    // `NoBirthZero`: `new_box` lands in the store's own
                    // block — no `birth_call`/argument chain exists. The
                    // store-fault discharge tail still discharges the
                    // in-flight child through `HomeRelease`.
                    if !matches!(function.blocks.get(provider_origin).and_then(|b| b.terminator.as_ref()),
                        Some(MirInstruction::Invoke {
                            operation: InvokeOperation::NewBox { object },
                            fault_frame,
                            fault_landing,
                            normal_landing,
                        }) if *object == *child
                            && *normal_landing == *block
                            && fault_landing == discharge
                            && frame.is_some_and(|(id, _)| *fault_frame == id))
                    {
                        return Err(fault("provider-emission-drift"));
                    }
                    if !matches!(function.blocks.get(store_discharge).and_then(|b| b.terminator.as_ref()),
                        Some(MirInstruction::Invoke {
                            operation: InvokeOperation::HomeRelease { object, value: released },
                            fault_frame,
                            fault_landing,
                            normal_landing,
                        }) if *object == *child && *released == *value
                            && normal_landing != fault_landing
                            && lands_on(function, *normal_landing, *discharge)
                            && lands_on(function, *fault_landing, *discharge)
                            && frame.is_some_and(|(id, _)| *fault_frame == id))
                    {
                        return Err(fault("provider-discharge-drift"));
                    }
                }
                (
                    Some(provider_origin),
                    ConstructionStoreRhsV1::ProviderConstruction {
                        object:
                            Some(
                                crate::mir::normal_callable_semantic_package
                                    ::ProviderConstructionChildV1::BirthIndexed(child),
                            ),
                        ..
                    },
                    Some(super::ProviderChildEmissionV1::Birth(birth)),
                ) => {
                    let Some((_, frame_landing)) = *frame else {
                        return Err(fault("emission-state"));
                    };
                    if !matches!(function.blocks.get(provider_origin).and_then(|b| b.terminator.as_ref()),
                        Some(MirInstruction::Invoke {
                            operation: InvokeOperation::NewBox { object },
                            fault_frame,
                            fault_landing,
                            normal_landing,
                        }) if *object == *child
                            && *normal_landing == birth.entry
                            && fault_landing == discharge
                            && frame.is_some_and(|(id, _)| *fault_frame == id))
                    {
                        return Err(fault("provider-emission-drift"));
                    }
                    // Argument chain: `entry` terminates in the first
                    // recorded `Call{Global, I64}` invoke, each landing
                    // projects the i64 result and terminates in the next,
                    // and the last landing ends on `birth_call`. Every
                    // invoke faults onto the reclaim head — the same
                    // cleanup the birth call owns.
                    let ConstructionStoreRhsV1::ProviderConstruction { arguments, .. } = &store.rhs
                        else { return Err(fault("provider-argument-source-kind")); };
                    let expected: Vec<_> = arguments.iter().filter(|argument|
                        matches!(argument.kind(), crate::mir::normal_callable_semantic_package::OrdinaryNewTrivialArgumentKindV1::QualifiedStaticCall { .. }))
                        .map(|argument| argument.ordinal()).collect();
                    if !expected.iter().copied().eq(birth.call_args.iter().map(|argument| argument.ordinal)) {
                        return Err(fault("provider-argument-source-coverage"));
                    }
                    let mut cursor = birth.entry;
                    for arg in birth.call_args.iter() {
                        arg.validate_source_v1(function, store)?;
                        if arg.invoke_block != cursor { return Err(fault("provider-argument-origin")); }
                        if !matches!(function.blocks.get(&cursor).and_then(|b| b.terminator.as_ref()),
                            Some(MirInstruction::Invoke {
                                operation: InvokeOperation::Call {
                                    call,
                                    result: crate::mir::instruction::InvokeCallResultKind::I64,
                                },
                                fault_frame,
                                fault_landing,
                                normal_landing,
                            }) if matches!(call.callee, crate::mir::Callee::Global(_))
                                && call.dst.is_none()
                                && *normal_landing == arg.landing
                                && *fault_landing == birth.reclaim
                                && frame.is_some_and(|(id, _)| *fault_frame == id))
                        {
                            return Err(fault("provider-argument-drift"));
                        }
                        cursor = arg.landing;
                    }
                    if cursor != birth.birth_call {
                        return Err(fault("provider-argument-chain"));
                    }
                    if !matches!(function.blocks.get(&birth.birth_call).and_then(|b| b.terminator.as_ref()),
                        Some(MirInstruction::Invoke {
                            operation: InvokeOperation::Call {
                                call,
                                result: crate::mir::instruction::InvokeCallResultKind::Unit,
                            },
                            fault_frame,
                            fault_landing,
                            normal_landing,
                        }) if matches!(call.callee, crate::mir::Callee::BirthConstructor { ref receiver, .. } if *receiver == *value)
                            && birth.call_args.iter().all(|argument|
                                call.args.get(argument.ordinal as usize) == Some(&argument.value))
                            && *normal_landing == *block
                            && *fault_landing == birth.reclaim
                            && frame.is_some_and(|(id, _)| *fault_frame == id))
                    {
                        return Err(fault("provider-birth-drift"));
                    }
                    // Child Birth discharged its own partial fields;
                    // this caller reclaims only that child's storage.
                    let reclaim_tail = birth.reclaim;
                    if !matches!(function.blocks.get(&reclaim_tail).and_then(|b| b.terminator.as_ref()),
                        Some(MirInstruction::Invoke {
                            operation: InvokeOperation::ReclaimUnpublished { object, value: reclaimed },
                            fault_frame,
                            fault_landing,
                            normal_landing,
                        }) if *object == *child && *reclaimed == *value
                            && normal_landing != fault_landing
                            && lands_on(function, *normal_landing, *discharge)
                            && lands_on(function, *fault_landing, *discharge)
                            && frame.is_some_and(|(id, _)| *fault_frame == id))
                    {
                        return Err(fault("provider-reclaim-drift"));
                    }
                    // Store-fault discharge: the same residence chain on
                    // the in-flight lease, then the child's own Home.
                    let discharge_tail = field_discharge_chain(
                        function,
                        birth.store_discharge,
                        &birth.owned_fields,
                        *value,
                        *frame,
                    )
                    .ok_or_else(|| fault("provider-discharge-chain"))?;
                    if !matches!(function.blocks.get(&discharge_tail).and_then(|b| b.terminator.as_ref()),
                        Some(MirInstruction::Invoke {
                            operation: InvokeOperation::HomeRelease { object, value: released },
                            fault_frame,
                            fault_landing,
                            normal_landing,
                        }) if *object == *child && *released == *value
                            && normal_landing != fault_landing
                            && lands_on(function, *normal_landing, *discharge)
                            && lands_on(function, *fault_landing, *discharge)
                            && frame.is_some_and(|(id, _)| *fault_frame == id))
                    {
                        return Err(fault("provider-discharge-drift"));
                    }
                }
                (None, ConstructionStoreRhsV1::ProviderConstruction { .. }, _) => {
                    return Err(fault("provider-emission-missing"));
                }
                (
                    Some(_),
                    ConstructionStoreRhsV1::ProviderConstruction { .. },
                    _,
                ) => return Err(fault("provider-birth-missing")),
                (Some(_), _, _) => return Err(fault("provider-emission-foreign")),
                (None, _, Some(_)) => return Err(fault("provider-birth-foreign")),
                (None, _, None) => {}
            }
            if let ConstructionStoreRhsV1::LiteralI64(expected) = &store.rhs {
                let literal_matches = function
                    .blocks
                    .get(block)
                    .into_iter()
                    .flat_map(|block| block.all_instructions())
                    .any(|instruction| {
                        matches!(instruction,
                            MirInstruction::Const {
                                dst,
                                value: crate::mir::ConstValue::Integer(actual),
                            } if *dst == *value && *actual == *expected)
                    });
                if !literal_matches {
                    return Err(fault("literal-value-drift"));
                }
            }
        }
        let ConstructionState::Selected { steps, next_step, .. } = self else { unreachable!() };
        if *next_step != steps.len() { return Err(fault("step-residual")); }
        let mut previous_normal = None;
        let mut actual_calls = calls.iter();
        for step in steps.iter() {
            let (entry, normal) = match step {
                ConstructionSourceStepV1::Store(site) => {
                    let store = stores.get(site.node()).ok_or_else(|| fault("step-store-missing"))?;
                    let StoreProgress::Emitted { block, normal, provider, .. } = store.progress else {
                        return Err(fault("store-residual"));
                    };
                    (provider.unwrap_or(block), normal)
                }
                ConstructionSourceStepV1::StoredChildCall(_) => {
                    let call = actual_calls.next().ok_or_else(|| fault("step-call-unconsumed"))?;
                    call.validate(function, step, *frame)?
                }
            };
            if previous_normal.is_some_and(|previous| previous != entry) {
                return Err(fault("physical-step-order-drift"));
            }
            previous_normal = Some(normal);
        }
        if actual_calls.next().is_some() { return Err(fault("foreign-call-emission")); }
        Ok(())
    }
}

/// True when `landing` is a dedicated jump block forwarding to `target`.
fn lands_on(
    function: &crate::mir::MirFunction,
    landing: BasicBlockId,
    target: BasicBlockId,
) -> bool {
    jump_target(function, landing) == Some(target)
}

/// The unique jump target of a dedicated landing block, or `None` when
/// the block is missing or carries any other terminator shape.
fn jump_target(
    function: &crate::mir::MirFunction,
    landing: BasicBlockId,
) -> Option<BasicBlockId> {
    match function
        .blocks
        .get(&landing)
        .and_then(|b| b.terminator.as_ref())
    {
        Some(MirInstruction::Jump {
            target,
            edge_args: None,
        }) => Some(*target),
        _ => None,
    }
}

/// Compare per-store cleanup against its retained source-sealed inventory.
pub(super) fn field_discharge_chain(
    function: &MirFunction,
    head: BasicBlockId,
    expected: &[crate::mir::normal_callable_semantic_package::OwnedFieldChildV1],
    base: ValueId,
    frame: Option<(ValueId, BasicBlockId)>,
) -> Option<BasicBlockId> {
    use crate::mir::normal_callable_semantic_package::OwnedFieldChildKindV1;
    let mut cursor = head;
    for child in expected {
        let MirInstruction::Invoke {
            operation,
            fault_frame,
            normal_landing,
            fault_landing,
        } = function.blocks.get(&cursor)?.terminator.as_ref()?
        else {
            return None;
        };
        let matches_source = match (child.kind, operation) {
            (
                OwnedFieldChildKindV1::Residence(expected_mode),
                InvokeOperation::OwnedFieldResidenceRelease {
                mode,
                    field,
                    base: released,
                },
            ) => *mode == expected_mode && *field == child.field && *released == base,
            (
                OwnedFieldChildKindV1::Object(object),
                InvokeOperation::OwnedObjectFieldRelease {
                    field,
                    base: released,
                    child: released_child,
                },
            ) => *field == child.field && *released == base && *released_child == object,
            _ => false,
        };
        if !matches_source
            || normal_landing == fault_landing
            || !frame.is_some_and(|(id, _)| id == *fault_frame)
        {
            return None;
        }
        let next = jump_target(function, *normal_landing)?;
        if jump_target(function, *fault_landing)? != next {
            return None;
        }
        cursor = next;
    }
    Some(cursor)
}

#[cfg(test)]
#[path = "fault_cleanup_tests.rs"]
mod fault_cleanup_tests;
