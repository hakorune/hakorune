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
        let nested_releases: usize = stores
            .values()
            .map(|store| match &store.progress {
                StoreProgress::Emitted {
                    provider_birth: Some(birth),
                    ..
                } => birth.owned_fields.len(),
                _ => 0,
            })
            .sum();
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
        // on each of the two cleanup chains.
        if actual_count
            != stores.len() + provider_count + 3 * birth_providers + 2 * nested_releases
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
                        // Only a proven provider `new` may land a normal
                        // result — its store records the exact pair, and the
                        // projection sits in the allocation's normal
                        // landing (the `birth_call` block for a user-class
                        // provider, the store block otherwise).
                        let proven = stores.values().any(|store| {
                            matches!(
                                &store.progress,
                                StoreProgress::Emitted {
                                    block: home,
                                    value,
                                    provider: Some(origin),
                                    provider_birth,
                                    ..
                                } if provider_birth
                                    .as_ref()
                                    .map(|birth| birth.birth_call)
                                    .unwrap_or(*home)
                                    == block.id
                                    && *origin == *invoke_block
                                    && *value == *dst
                            )
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
            } = progress
            else {
                return Err(fault("store-residual"));
            };
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
                    if actual == &field && b == base && v == value && normal_landing == normal && Some((*fault_frame, *fault_landing)) == *frame),
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
                            && Some((*fault_frame, *fault_landing)) == *frame)
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
                            && *normal_landing == birth.birth_call
                            && *fault_landing == frame_landing
                            && frame.is_some_and(|(id, _)| *fault_frame == id))
                    {
                        return Err(fault("provider-emission-drift"));
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
                    // Birth-fault cleanup: each sealed `ArrayBox` residence
                    // releases newest-first on the in-flight child handle,
                    // then the unpublished storage is reclaimed.
                    let reclaim_tail = residence_chain(
                        function,
                        birth.reclaim,
                        &birth.owned_fields,
                        *value,
                        *frame,
                    )
                    .ok_or_else(|| fault("provider-reclaim-chain"))?;
                    if !matches!(function.blocks.get(&reclaim_tail).and_then(|b| b.terminator.as_ref()),
                        Some(MirInstruction::Invoke {
                            operation: InvokeOperation::ReclaimUnpublished { object, value: reclaimed },
                            fault_frame,
                            fault_landing,
                            normal_landing,
                        }) if *object == *child && *reclaimed == *value
                            && normal_landing != fault_landing
                            && lands_on(function, *normal_landing, frame_landing)
                            && lands_on(function, *fault_landing, frame_landing)
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
                            && lands_on(function, *normal_landing, frame_landing)
                            && lands_on(function, *fault_landing, frame_landing)
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
