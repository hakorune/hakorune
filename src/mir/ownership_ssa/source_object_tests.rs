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
    function.get_block_mut(bb(0)).unwrap().set_terminator(MirInstruction::Invoke {
        operation: InvokeOperation::NewBox { object }, fault_frame: value(9), normal_landing: bb(1), fault_landing: bb(2),
    });
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
    function.get_block_mut(bb(4)).unwrap().set_terminator(MirInstruction::Invoke {
        operation: InvokeOperation::ReclaimUnpublished { object, value: value(10) }, fault_frame: value(9), normal_landing: bb(7), fault_landing: bb(8),
    });
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
    wrong_fault_end.get_block_mut(bb(4)).unwrap().set_terminator(MirInstruction::Invoke {
        operation: InvokeOperation::HomeRelease { object, value: value(10) }, fault_frame: value(9), normal_landing: bb(7), fault_landing: bb(8),
    });
    assert!(matches!(verify_ownership_ssa_v1(&wrong_fault_end, &abi), Err(OwnershipSsaErrorV1::InvalidObjectLifecycle { .. })));
    let mut scalar_end = function.clone();
    scalar_end.get_block_mut(bb(3)).unwrap().set_terminator(MirInstruction::Invoke {
        operation: InvokeOperation::HomeRelease { object, value: value(9) }, fault_frame: value(9), normal_landing: bb(5), fault_landing: bb(6),
    });
    assert!(matches!(verify_ownership_ssa_v1(&scalar_end, &abi), Err(OwnershipSsaErrorV1::InvalidObjectLifecycle { .. })));
    let mut escaped = function.clone();
    escaped.get_block_mut(bb(3)).unwrap().add_instruction(MirInstruction::Store { value: value(11), ptr: value(9) });
    assert!(matches!(verify_ownership_ssa_v1(&escaped, &abi), Err(OwnershipSsaErrorV1::UnsupportedManagedInstruction { .. })));
    function.get_block_mut(bb(5)).unwrap().set_terminator(MirInstruction::Invoke {
        operation: InvokeOperation::HomeRelease { object, value: value(10) }, fault_frame: value(9), normal_landing: bb(6), fault_landing: bb(8),
    });
    assert!(matches!(verify_ownership_ssa_v1(&function, &abi), Err(OwnershipSsaErrorV1::InvalidObjectLifecycle { .. })));
}
