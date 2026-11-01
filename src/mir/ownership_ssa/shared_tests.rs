//! Source lifecycle contracts over the shared ownership test fixtures.
use super::*;

#[test]
fn shared_new_transfers_payload_only_on_normal_and_release_consumes_once() {
    use crate::mir::instruction::{InvokeOperation, ObjectEndStepV1, SharedDescriptorV1, SharedInvokeOperation as Shared};
    let object = hakorune_mir_defs::CanonicalObjectIdV1::from_declaration_index(0).unwrap();
    let descriptor = SharedDescriptorV1 { payload: object, end: vec![ObjectEndStepV1::ReleaseOuter].into_boxed_slice() };
    let (mut f, _) = function(&[], 7);
    let owner = crate::mir::resolved_semantics::FunctionOwnerIssuerV1::new_for_compilation().unwrap().issue().unwrap();
    let abi = OwnershipFunctionAbiV1::new(OwnershipFunctionOwnerV1::source(owner), vec![], FunctionResultOwnershipV1::None)
        .with_initially_published([value(10)].into_iter().collect())
        .with_source_shared_descriptors(vec![descriptor.clone()]);
    f.get_block_mut(bb(0)).unwrap().add_instruction(MirInstruction::FaultFrameEnter {
        dst: value(9), mode: crate::mir::instruction::FaultFrameMode::RootOwned,
    });
    f.get_block_mut(bb(0)).unwrap().set_terminator(MirInstruction::Invoke {
        operation: InvokeOperation::NewBox { object }, fault_frame: value(9), normal_landing: bb(1), fault_landing: bb(2),
    });
    f.get_block_mut(bb(1)).unwrap().add_instruction(MirInstruction::InvokeNormalResult { invoke_block: bb(0), dst: value(10) });
    f.get_block_mut(bb(1)).unwrap().set_terminator(MirInstruction::Invoke {
        operation: InvokeOperation::Shared(Shared::New { descriptor: descriptor.clone(), payload: value(10) }),
        fault_frame: value(9), normal_landing: bb(3), fault_landing: bb(4),
    });
    f.get_block_mut(bb(3)).unwrap().add_instruction(MirInstruction::InvokeNormalResult { invoke_block: bb(1), dst: value(11) });
    f.get_block_mut(bb(3)).unwrap().set_terminator(MirInstruction::Invoke {
        operation: InvokeOperation::Shared(Shared::Release { descriptor: descriptor.clone(), handle: value(11) }),
        fault_frame: value(9), normal_landing: bb(5), fault_landing: bb(6),
    });
    f.get_block_mut(bb(4)).unwrap().set_terminator(MirInstruction::Invoke {
        operation: InvokeOperation::HomeRelease { object, value: value(10) },
        fault_frame: value(9), normal_landing: bb(5), fault_landing: bb(6),
    });
    for block in [2, 5, 6] { f.get_block_mut(bb(block)).unwrap().set_terminator(MirInstruction::Return { value: None }); }
    assert!(verify_ownership_ssa_v1(&f, &abi).is_ok());
    // The same live handle may be renamed across mutually exclusive edges;
    // the PHI transfers its one token and does not clone the cell.
    let mut joined = f.clone();
    for id in 7..=9 { joined.add_block(BasicBlock::new(bb(id))); }
    joined.get_block_mut(bb(3)).unwrap().add_instruction(MirInstruction::Const { dst: value(14), value: ConstValue::Bool(true) });
    branch(&mut joined, 3, 14, 7, 8);
    jump(&mut joined, 7, 9);
    jump(&mut joined, 8, 9);
    phi(&mut joined, 9, 15, &[(7, 11), (8, 11)]);
    joined.get_block_mut(bb(9)).unwrap().set_terminator(MirInstruction::Invoke {
        operation: InvokeOperation::Shared(Shared::Release { descriptor: descriptor.clone(), handle: value(15) }),
        fault_frame: value(9), normal_landing: bb(5), fault_landing: bb(6),
    });
    assert!(verify_ownership_ssa_v1(&joined, &abi).is_ok(), "one Shared token survives a PHI rename");
    let mut duplicate_phi = joined;
    phi(&mut duplicate_phi, 9, 16, &[(7, 11), (8, 11)]);
    assert!(verify_ownership_ssa_v1(&duplicate_phi, &abi).is_err(), "one token cannot feed two PHIs");
    let mut forged = f.clone();
    let wrong_descriptor = SharedDescriptorV1 { payload: object, end: Box::new([]) };
    forged.get_block_mut(bb(1)).unwrap().set_terminator(MirInstruction::Invoke {
        operation: InvokeOperation::Shared(Shared::New { descriptor: wrong_descriptor, payload: value(10) }),
        fault_frame: value(9), normal_landing: bb(3), fault_landing: bb(4),
    });
    assert!(matches!(verify_ownership_ssa_v1(&forged, &abi), Err(OwnershipSsaErrorV1::InvalidSharedLifecycle { .. })));
    let mut same_edge = f.clone();
    same_edge.get_block_mut(bb(1)).unwrap().set_terminator(MirInstruction::Invoke {
        operation: InvokeOperation::Shared(Shared::New { descriptor: descriptor.clone(), payload: value(10) }),
        fault_frame: value(9), normal_landing: bb(3), fault_landing: bb(3),
    });
    assert!(matches!(verify_ownership_ssa_v1(&same_edge, &abi), Err(OwnershipSsaErrorV1::InvalidSharedCfg { .. })));
    let mut duplicate_definition = f.clone();
    duplicate_definition.get_block_mut(bb(0)).unwrap().add_instruction(MirInstruction::Const { dst: value(11), value: ConstValue::Integer(0) });
    assert!(matches!(verify_ownership_ssa_v1(&duplicate_definition, &abi), Err(OwnershipSsaErrorV1::InvalidSharedCfg { .. })));
    let mut wrong_fault = f.clone();
    wrong_fault.get_block_mut(bb(4)).unwrap().set_terminator(MirInstruction::Invoke {
        operation: InvokeOperation::Shared(Shared::Release { descriptor, handle: value(11) }),
        fault_frame: value(9), normal_landing: bb(5), fault_landing: bb(6),
    });
    assert!(verify_ownership_ssa_v1(&wrong_fault, &abi).is_err(), "Fault cannot use the Normal result");
    let mut leaked = f;
    leaked.get_block_mut(bb(3)).unwrap().set_terminator(MirInstruction::Return { value: Some(value(11)) });
    assert!(verify_ownership_ssa_v1(&leaked, &abi).is_err(), "Shared escape is not admitted");
}

#[test]
fn shared_borrow_requires_its_original_live_support() {
    use crate::mir::instruction::{InvokeOperation, ObjectEndStepV1, SharedDescriptorV1, SharedInvokeOperation as Shared};
    let object = hakorune_mir_defs::CanonicalObjectIdV1::from_declaration_index(0).unwrap();
    let field = hakorune_mir_defs::CanonicalFieldRefV1::from_declaration_ordinal(object, 0).unwrap();
    let descriptor = SharedDescriptorV1 { payload: object, end: vec![ObjectEndStepV1::ReleaseOuter].into_boxed_slice() };
    let (mut f, _) = function(&[], 11);
    let owner = crate::mir::resolved_semantics::FunctionOwnerIssuerV1::new_for_compilation().unwrap().issue().unwrap();
    let abi = OwnershipFunctionAbiV1::new(OwnershipFunctionOwnerV1::source(owner), vec![], FunctionResultOwnershipV1::None)
        .with_initially_published([value(10)].into_iter().collect())
        .with_source_shared_descriptors(vec![descriptor.clone()]);
    f.get_block_mut(bb(0)).unwrap().add_instruction(MirInstruction::FaultFrameEnter { dst: value(9), mode: crate::mir::instruction::FaultFrameMode::RootOwned });
    f.get_block_mut(bb(0)).unwrap().set_terminator(MirInstruction::Invoke { operation: InvokeOperation::NewBox { object }, fault_frame: value(9), normal_landing: bb(1), fault_landing: bb(2) });
    f.get_block_mut(bb(1)).unwrap().add_instruction(MirInstruction::InvokeNormalResult { invoke_block: bb(0), dst: value(10) });
    f.get_block_mut(bb(1)).unwrap().set_terminator(MirInstruction::Invoke { operation: InvokeOperation::Shared(Shared::New { descriptor: descriptor.clone(), payload: value(10) }), fault_frame: value(9), normal_landing: bb(3), fault_landing: bb(4) });
    f.get_block_mut(bb(3)).unwrap().add_instruction(MirInstruction::InvokeNormalResult { invoke_block: bb(1), dst: value(11) });
    f.get_block_mut(bb(3)).unwrap().set_terminator(MirInstruction::Invoke { operation: InvokeOperation::Shared(Shared::Borrow { descriptor: descriptor.clone(), handle: value(11) }), fault_frame: value(9), normal_landing: bb(7), fault_landing: bb(8) });
    f.get_block_mut(bb(7)).unwrap().add_instruction(MirInstruction::InvokeNormalResult { invoke_block: bb(3), dst: value(12) });
    f.get_block_mut(bb(7)).unwrap().set_terminator(MirInstruction::Invoke { operation: InvokeOperation::FieldGet { field, base: value(12) }, fault_frame: value(9), normal_landing: bb(9), fault_landing: bb(10) });
    f.get_block_mut(bb(9)).unwrap().add_instruction(MirInstruction::InvokeNormalResult { invoke_block: bb(7), dst: value(13) });
    for block in [8, 9, 10] {
        f.get_block_mut(bb(block)).unwrap().set_terminator(MirInstruction::Invoke { operation: InvokeOperation::Shared(Shared::Release { descriptor: descriptor.clone(), handle: value(11) }), fault_frame: value(9), normal_landing: bb(5), fault_landing: bb(6) });
    }
    f.get_block_mut(bb(4)).unwrap().set_terminator(MirInstruction::Invoke { operation: InvokeOperation::HomeRelease { object, value: value(10) }, fault_frame: value(9), normal_landing: bb(5), fault_landing: bb(6) });
    for block in [2, 5, 6] { f.get_block_mut(bb(block)).unwrap().set_terminator(MirInstruction::Return { value: None }); }
    assert!(verify_ownership_ssa_v1(&f, &abi).is_ok());
    f.get_block_mut(bb(7)).unwrap().set_terminator(MirInstruction::Invoke { operation: InvokeOperation::Shared(Shared::Release { descriptor, handle: value(11) }), fault_frame: value(9), normal_landing: bb(9), fault_landing: bb(10) });
    f.get_block_mut(bb(9)).unwrap().instructions.clear();
    assert!(matches!(verify_ownership_ssa_v1(&f, &abi), Err(OwnershipSsaErrorV1::InvalidSharedLifecycle { .. })));
}
