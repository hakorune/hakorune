//! Source lifecycle contracts over the shared ownership test fixtures.
use super::*;

#[test]
fn new_home_alias_ends_once_across_normal_and_fault_edges() {
    use crate::mir::instruction::{BirthInvokeV1, InvokeOperation};
    let object = hakorune_mir_defs::CanonicalObjectIdV1::from_declaration_index(0).unwrap();
    let (mut function, _) = function(&[], 9);
    let owner = crate::mir::resolved_semantics::FunctionOwnerIssuerV1::new_for_compilation().unwrap().issue().unwrap();
    let abi = OwnershipFunctionAbiV1::new(OwnershipFunctionOwnerV1::source(owner), vec![], FunctionResultOwnershipV1::None)
        .with_source_aliases([(value(11), value(10))].into_iter().collect());
    function.get_block_mut(bb(0)).unwrap().add_instruction(MirInstruction::FaultFrameEnter {
        dst: value(9), mode: crate::mir::instruction::FaultFrameMode::RootOwned,
    });
    invoke(&mut function, 0, InvokeOperation::NewBox { object }, 1, 2);
    let normal = function.get_block_mut(bb(1)).unwrap();
    normal.add_instruction(MirInstruction::InvokeNormalResult { invoke_block: bb(0), dst: value(10) });
    normal.set_terminator(MirInstruction::Invoke {
        operation: InvokeOperation::Birth(BirthInvokeV1::from_value_arguments(
            hakorune_mir_defs::CanonicalSameModuleCallableKeyV1::birth_constructor("Pair", 0), value(10), vec![], EffectMask::PURE)),
        fault_frame: value(9), normal_landing: bb(3), fault_landing: bb(4),
    });
    let normal = function.get_block_mut(bb(3)).unwrap();
    normal.add_instruction(MirInstruction::Copy { dst: value(11), src: value(10) });
    normal.set_terminator(MirInstruction::Invoke {
        operation: InvokeOperation::HomeRelease { object, value: value(11) }, fault_frame: value(9), normal_landing: bb(5), fault_landing: bb(6),
    });
    invoke(&mut function, 4, InvokeOperation::ReclaimUnpublished { object, value: value(10) }, 7, 8);
    for id in [2, 5, 6, 7, 8] {
        function.get_block_mut(bb(id)).unwrap().set_terminator(MirInstruction::Return { value: None });
    }
    assert!(verify_ownership_ssa_v1(&function, &abi).is_ok());
    let mut same_edge = function.clone();
    if let Some(MirInstruction::Invoke { fault_landing, .. }) = &mut same_edge.get_block_mut(bb(1)).unwrap().terminator {
        *fault_landing = bb(3);
    }
    assert!(matches!(verify_ownership_ssa_v1(&same_edge, &abi), Err(OwnershipSsaErrorV1::InvalidObjectLifecycle { .. })));
    let mut wrong_fault_end = function.clone();
    invoke(&mut wrong_fault_end, 4, InvokeOperation::HomeRelease { object, value: value(10) }, 7, 8);
    assert!(matches!(verify_ownership_ssa_v1(&wrong_fault_end, &abi), Err(OwnershipSsaErrorV1::InvalidObjectLifecycle { .. })));
    let mut scalar_end = function.clone();
    invoke(&mut scalar_end, 3, InvokeOperation::HomeRelease { object, value: value(9) }, 5, 6);
    assert!(matches!(verify_ownership_ssa_v1(&scalar_end, &abi), Err(OwnershipSsaErrorV1::InvalidObjectLifecycle { .. })));
    let mut escaped = function.clone();
    escaped.get_block_mut(bb(3)).unwrap().add_instruction(MirInstruction::Store { value: value(11), ptr: value(9) });
    assert!(matches!(verify_ownership_ssa_v1(&escaped, &abi), Err(OwnershipSsaErrorV1::UnsupportedManagedInstruction { .. })));
    invoke(&mut function, 5, InvokeOperation::HomeRelease { object, value: value(10) }, 6, 8);
    assert!(matches!(verify_ownership_ssa_v1(&function, &abi), Err(OwnershipSsaErrorV1::InvalidObjectLifecycle { .. })));
    // A take parameter starts as a published object; no synthetic allocation.
    let (mut entry, _) = self::function(&[MirOwnershipKindV1::Owned], 3);
    let parameter = entry.params[0];
    let entry_abi = OwnershipFunctionAbiV1::new(OwnershipFunctionOwnerV1::source(owner),
        vec![MirOwnershipKindV1::Owned], FunctionResultOwnershipV1::None)
        .with_source_owned_parameters([(parameter, object)].into_iter().collect());
    entry.get_block_mut(bb(0)).unwrap().add_instruction(MirInstruction::FaultFrameEnter {
        dst: value(9), mode: crate::mir::instruction::FaultFrameMode::RootOwned,
    });
    invoke(&mut entry, 0, InvokeOperation::HomeRelease { object, value: parameter }, 1, 2);
    for block in [1, 2] { entry.get_block_mut(bb(block)).unwrap().set_terminator(MirInstruction::Return { value: None }); }
    assert!(verify_ownership_ssa_v1(&entry, &entry_abi).is_ok());
    let target = hakorune_mir_defs::CanonicalSameModuleCallableKeyV1::static_box_method("Consumer", "run", 1);
    let call_abi = entry_abi.clone().with_source_call_takes([(target.clone(), [(0, object)].into_iter().collect())].into_iter().collect());
    let mut caller = entry.clone();
    invoke(&mut caller, 0, InvokeOperation::Call {
            call: crate::mir::definitions::call_unified::MirCall::global(None, target.canonical_global_target_v1().unwrap(), vec![parameter]),
            result: crate::mir::instruction::InvokeCallResultKind::I64,
        }, 1, 2);
    assert!(verify_ownership_ssa_v1(&caller, &call_abi).is_ok());
    let r = verify_ownership_ssa_v1(&caller, &entry_abi);
    assert!(matches!(r, Err(OwnershipSsaErrorV1::ManagedCallOwnershipUnsupported { .. })), "a call without a declared take cannot move the parameter: {r:?}");
    // Entry transfer consumes on Fault as well: the caller cannot End again.
    invoke(&mut caller, 2, InvokeOperation::HomeRelease { object, value: parameter }, 1, 1);
    let r = verify_ownership_ssa_v1(&caller, &call_abi);
    assert!(matches!(r, Err(OwnershipSsaErrorV1::InvalidObjectLifecycle { .. })), "a moved parameter cannot be Ended on the Fault edge: {r:?}");
    let missing_class = entry_abi.clone().with_source_owned_parameters(Default::default());
    assert!(matches!(verify_ownership_ssa_v1(&entry, &missing_class), Err(OwnershipSsaErrorV1::InvalidObjectLifecycle { .. })));
    let mut wrong_class = entry.clone();
    if let Some(MirInstruction::Invoke { operation: InvokeOperation::HomeRelease { object, .. }, .. }) =
        &mut wrong_class.get_block_mut(bb(0)).unwrap().terminator {
        *object = hakorune_mir_defs::CanonicalObjectIdV1::from_declaration_index(1).unwrap();
    }
    assert!(matches!(verify_ownership_ssa_v1(&wrong_class, &entry_abi), Err(OwnershipSsaErrorV1::InvalidObjectLifecycle { .. })));
    entry.get_block_mut(bb(0)).unwrap().set_terminator(MirInstruction::Return { value: None });
    entry.blocks.retain(|id, _| *id == bb(0));
    assert!(matches!(verify_ownership_ssa_v1(&entry, &entry_abi), Err(OwnershipSsaErrorV1::MissingDispositionAtExit { .. })));

}

/// Hand-assembled MIR (not the real pipeline): the verifier admits a published object into a Birth only through a declared `take` ordinal; the permission is
/// the declared ordinal, a single transfer, and nothing may End the same value twice.
#[test]
fn hand_assembled_birth_take_permission_is_the_declared_ordinal_only() {
    use crate::mir::instruction::{BirthArgumentV1, BirthInvokeV1, InvokeOperation};
    let object = hakorune_mir_defs::CanonicalObjectIdV1::from_declaration_index(0).unwrap();
    let holder = hakorune_mir_defs::CanonicalObjectIdV1::from_declaration_index(1).unwrap();
    let key = hakorune_mir_defs::CanonicalSameModuleCallableKeyV1::birth_constructor("Holder", 1);
    let owner = crate::mir::resolved_semantics::FunctionOwnerIssuerV1::new_for_compilation().unwrap().issue().unwrap();
    let (mut function, _) = self::function(&[MirOwnershipKindV1::Owned], 9);
    let parameter = function.params[0];
    let base_abi = OwnershipFunctionAbiV1::new(OwnershipFunctionOwnerV1::source(owner),
        vec![MirOwnershipKindV1::Owned], FunctionResultOwnershipV1::None)
        .with_source_owned_parameters([(parameter, object)].into_iter().collect())
        .with_source_aliases([(value(11), value(10))].into_iter().collect());
    function.get_block_mut(bb(0)).unwrap().add_instruction(MirInstruction::FaultFrameEnter {
        dst: value(9), mode: crate::mir::instruction::FaultFrameMode::RootOwned,
    });
    invoke(&mut function, 0, InvokeOperation::NewBox { object: holder }, 1, 2);
    function.get_block_mut(bb(1)).unwrap().add_instruction(MirInstruction::InvokeNormalResult { invoke_block: bb(0), dst: value(10) });
    invoke(&mut function, 1, InvokeOperation::Birth(BirthInvokeV1 {
        key: key.clone(), receiver: value(10), arguments: vec![BirthArgumentV1::Value(parameter)].into_boxed_slice(),
        effects: EffectMask::PURE,
    }), 3, 4);
    // Normal: the holder (which owns the moved object through its field) is Ended once.
    function.get_block_mut(bb(3)).unwrap().add_instruction(MirInstruction::Copy { dst: value(11), src: value(10) });
    invoke(&mut function, 3, InvokeOperation::HomeRelease { object: holder, value: value(11) }, 5, 6);
    // Birth Fault: the unpublished holder is reclaimed and the caller-owned actual is Ended.
    invoke(&mut function, 4, InvokeOperation::ReclaimUnpublished { object: holder, value: value(10) }, 7, 7);
    invoke(&mut function, 7, InvokeOperation::HomeRelease { object, value: parameter }, 8, 8);
    // The allocation Fault precedes any move: the caller still owns the actual and Ends it.
    invoke(&mut function, 2, InvokeOperation::HomeRelease { object, value: parameter }, 5, 6);
    for id in [5, 6, 8] {
        function.get_block_mut(bb(id)).unwrap().set_terminator(MirInstruction::Return { value: None });
    }
    let take_abi = base_abi.clone().with_source_birth_takes([(key.clone(), [0u32].into_iter().collect())].into_iter().collect());
    let accepted = verify_ownership_ssa_v1(&function, &take_abi);
    // The declared `take` ordinal admits the transfer; without the declaration it is refused.
    assert!(accepted.is_ok(), "declared take must admit the transfer: {accepted:?}");
    assert!(matches!(verify_ownership_ssa_v1(&function, &base_abi), Err(OwnershipSsaErrorV1::InvalidObjectLifecycle { .. })),
        "an undeclared formal gets no take permission");
    let wrong_ordinal = base_abi.clone().with_source_birth_takes([(key.clone(), [1u32].into_iter().collect())].into_iter().collect());
    assert!(matches!(verify_ownership_ssa_v1(&function, &wrong_ordinal), Err(OwnershipSsaErrorV1::InvalidObjectLifecycle { .. })),
        "a take on another ordinal does not cover this formal");
    // Ending the moved value again on the Normal path is a double End.
    let mut double_end = function.clone();
    invoke(&mut double_end, 5, InvokeOperation::HomeRelease { object, value: parameter }, 6, 6);
    assert!(matches!(verify_ownership_ssa_v1(&double_end, &take_abi), Err(OwnershipSsaErrorV1::InvalidObjectLifecycle { .. })), "a moved value cannot be Ended again");
    // The same value in two take positions is moved twice.
    let mut twice = function.clone();
    if let Some(MirInstruction::Invoke { operation: InvokeOperation::Birth(row), .. }) = &mut twice.get_block_mut(bb(1)).unwrap().terminator {
        row.arguments = vec![BirthArgumentV1::Value(parameter), BirthArgumentV1::Value(parameter)].into_boxed_slice();
    }
    let both = base_abi.with_source_birth_takes([(key, [0u32, 1].into_iter().collect())].into_iter().collect());
    assert!(matches!(verify_ownership_ssa_v1(&twice, &both), Err(OwnershipSsaErrorV1::InvalidObjectLifecycle { .. })), "one value cannot be taken twice");
}
