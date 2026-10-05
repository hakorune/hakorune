//! Emitted-shape validation for construction stores: the retained transport
//! checks and the exact block/terminator census against the physical draft.
//! No source acceptance and no new predicate live here — only comparison of
//! recorded progress rows with the emitted function.

use super::*;
use crate::mir::instruction::InvokeOperation;
use crate::mir::MirInstruction;

impl RetainedConstructionValidation {
    pub(in crate::mir::builder) fn validate_artifact_after_compiler_finishing(
        self,
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

    pub(in crate::mir::builder) fn validate_after_compiler_finishing(
        self,
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
        let ConstructionState::Selected { stores, frame, .. } = self else {
            return Ok(());
        };
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
                        object: Some(_),
                        ..
                    }
                )
            })
            .count();
        let (nested_releases, call_args): (usize, usize) = stores
            .values()
            .map(|store| match &store.progress {
                StoreProgress::Emitted {
                    provider_birth: Some(birth),
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
        // A user-class provider adds three more invokes on the frame:
        // `birth_call`, the `reclaim_unpublished` on its fault edge, and the
        // `home_release` discharge on the object-field store's fault edge —
        // plus one `field_residence_release` per sealed nested residence
        // on the completed child store-Fault chain, prior-field discharge and
        // one `Call{Global, I64}`
        // per sealed qualified-static argument row.
        if actual_count
            != stores.len()
                + provider_count
                + 3 * birth_providers
                + nested_releases
                + stores
                    .values()
                    .map(|store| store.fault_discharge.len())
                    .sum::<usize>()
                + call_args
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
                                provider_birth,
                                ..
                            } = &store.progress
                            else {
                                return false;
                            };
                            match provider_birth {
                                Some(birth) => {
                                    (birth.entry == block.id
                                        && *origin == *invoke_block
                                        && *value == *dst)
                                        || birth.call_args.iter().any(|arg| {
                                            arg.landing == block.id
                                                && arg.invoke_block == *invoke_block
                                                && arg.value == *dst
                                        })
                                }
                                None => {
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
                provider_birth,
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
            let store_ok = match (&store.rhs, provider_birth) {
                (
                    ConstructionStoreRhsV1::ProviderConstruction {
                        object: Some(child),
                        ..
                    },
                    Some(birth),
                ) => matches!(function.blocks.get(block).and_then(|b| b.terminator.as_ref()),
                    Some(MirInstruction::Invoke { operation: InvokeOperation::ObjectFieldSet { field: actual, base: b, value: v, child: stored }, fault_frame, fault_landing, normal_landing })
                    if actual == &field && b == base && v == value && stored == child
                        && normal_landing == normal
                        && *fault_landing == birth.store_discharge
                        && frame.is_some_and(|(id, _)| *fault_frame == id)),
                _ => matches!(function.blocks.get(block).and_then(|b| b.terminator.as_ref()),
                    Some(MirInstruction::Invoke { operation: InvokeOperation::FieldSet { field: actual, base: b, value: v }, fault_frame, fault_landing, normal_landing })
                    if actual == &field && b == base && v == value && normal_landing == normal && *fault_frame == frame_id && fault_landing == discharge),
            };
            if !store_ok {
                return Err(fault("emission-drift"));
            }
            match (provider, &store.rhs, provider_birth) {
                (
                    Some(provider_origin),
                    ConstructionStoreRhsV1::ProviderConstruction {
                        object: None, ..
                    },
                    None,
                ) => {
                    if !matches!(function.blocks.get(provider_origin).and_then(|b| b.terminator.as_ref()),
                        Some(MirInstruction::Invoke {
                            operation: InvokeOperation::IntrinsicArrayNew,
                            fault_frame,
                            fault_landing,
                            normal_landing,
                        }) if *normal_landing == *block
                            && *fault_frame == frame_id && fault_landing == discharge)
                    {
                        return Err(fault("provider-emission-drift"));
                    }
                }
                (
                    Some(provider_origin),
                    ConstructionStoreRhsV1::ProviderConstruction {
                        object: Some(child),
                        ..
                    },
                    Some(birth),
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
                    let mut cursor = birth.entry;
                    for arg in birth.call_args.iter() {
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
                    let discharge_tail = residence_chain(
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

/// Walk a nested-residence cleanup chain starting at `head`: exactly one
/// `OwnedFieldResidenceRelease` invoke per expected field in emitted
/// order, each releasing `base` on the shared fault frame and landing
/// through distinct jump blocks onto the next link. Returns the tail
/// block — the caller validates its terminator (`ReclaimUnpublished` or
/// `HomeRelease`) — or `None` on any missing, foreign, duplicated, or
/// reordered link.
fn residence_chain(
    function: &crate::mir::MirFunction,
    head: BasicBlockId,
    expected: &[hakorune_mir_defs::CanonicalFieldRefV1],
    base: crate::mir::ValueId,
    frame: Option<(crate::mir::ValueId, BasicBlockId)>,
) -> Option<BasicBlockId> {
    let mut cursor = head;
    for field in expected {
        let MirInstruction::Invoke {
            operation:
                InvokeOperation::OwnedFieldResidenceRelease {
                    field: actual,
                    base: released,
                },
            fault_frame,
            normal_landing,
            fault_landing,
        } = function.blocks.get(&cursor)?.terminator.as_ref()?
        else {
            return None;
        };
        if actual != field
            || *released != base
            || normal_landing == fault_landing
            || !frame.is_some_and(|(id, _)| *fault_frame == id)
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

/// Compare per-store cleanup against its retained source-sealed inventory.
fn field_discharge_chain(
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
                OwnedFieldChildKindV1::Array,
                InvokeOperation::OwnedFieldResidenceRelease {
                    field,
                    base: released,
                },
            ) => *field == child.field && *released == base,
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
