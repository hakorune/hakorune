use super::*;
use crate::mir::definitions::call_unified::{CalleeBoxKind, TypeCertainty};
use crate::mir::edge_args::JumpArgsLayout;
use crate::mir::{
    BasicBlock, BasicBlockId, Callee, ConstValue, EdgeArgs, EffectMask, FunctionSignature,
    MirFunction, MirInstruction, MirType, ValueId,
};

fn bb(raw: u32) -> BasicBlockId {
    BasicBlockId::new(raw)
}

fn value(raw: u32) -> ValueId {
    ValueId::new(raw)
}

fn function(
    parameter_kinds: &[MirOwnershipKindV1],
    blocks: u32,
) -> (MirFunction, OwnershipFunctionAbiV1) {
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "ownership_ssa_test/0".into(),
            params: parameter_kinds
                .iter()
                .map(|_| MirType::Box("OwnedTestBox".into()))
                .collect(),
            return_type: MirType::Box("OwnedTestBox".into()),
            effects: EffectMask::PURE,
        },
        bb(0),
    );
    for raw in 1..blocks {
        function.add_block(BasicBlock::new(bb(raw)));
    }
    let abi = OwnershipFunctionAbiV1::new(
        OwnershipFunctionOwnerV1::new(7),
        parameter_kinds.to_vec(),
        FunctionResultOwnershipV1::Owned,
    );
    (function, abi)
}

fn branch(function: &mut MirFunction, source: u32, condition: u32, then_bb: u32, else_bb: u32) {
    function
        .get_block_mut(bb(source))
        .unwrap()
        .set_terminator(MirInstruction::Branch {
            condition: value(condition),
            then_bb: bb(then_bb),
            else_bb: bb(else_bb),
            then_edge_args: None,
            else_edge_args: None,
        });
    function
        .get_block_mut(bb(then_bb))
        .unwrap()
        .add_predecessor(bb(source));
    function
        .get_block_mut(bb(else_bb))
        .unwrap()
        .add_predecessor(bb(source));
}

fn jump(function: &mut MirFunction, source: u32, target: u32) {
    function
        .get_block_mut(bb(source))
        .unwrap()
        .set_terminator(MirInstruction::Jump {
            target: bb(target),
            edge_args: None,
        });
    function
        .get_block_mut(bb(target))
        .unwrap()
        .add_predecessor(bb(source));
}

fn ret(function: &mut MirFunction, block: u32, result: u32) {
    function
        .get_block_mut(bb(block))
        .unwrap()
        .set_terminator(MirInstruction::Return {
            value: Some(value(result)),
        });
}

fn none_result_call(
    callee: Option<Callee>,
    func: ValueId,
    args: Vec<ValueId>,
    dst: Option<ValueId>,
) -> (MirFunction, OwnershipFunctionAbiV1) {
    let (mut function, abi) = function(&[], 1);
    let abi = OwnershipFunctionAbiV1::new(abi.owner(), Vec::new(), FunctionResultOwnershipV1::None);
    function
        .get_block_mut(bb(0))
        .unwrap()
        .add_instruction(MirInstruction::Call {
            dst,
            func,
            callee,
            args,
            effects: EffectMask::PURE,
        });
    function
        .get_block_mut(bb(0))
        .unwrap()
        .set_terminator(MirInstruction::Return { value: None });
    (function, abi)
}

fn define_trivial_value(function: &mut MirFunction, raw: u32) {
    function
        .get_block_mut(bb(0))
        .unwrap()
        .add_instruction(MirInstruction::Const {
            dst: value(raw),
            value: ConstValue::Integer(0),
        });
}

fn phi(function: &mut MirFunction, block: u32, dst: u32, inputs: &[(u32, u32)]) {
    function
        .get_block_mut(bb(block))
        .unwrap()
        .add_instruction(MirInstruction::Phi {
            dst: value(dst),
            inputs: inputs
                .iter()
                .map(|(pred, src)| (bb(*pred), value(*src)))
                .collect(),
            type_hint: Some(MirType::Box("OwnedTestBox".into())),
        });
}

#[test]
fn alternative_branch_tokens_forward_into_one_owned_phi_result() {
    let (mut function, abi) = function(
        &[
            MirOwnershipKindV1::Borrowed,
            MirOwnershipKindV1::Borrowed,
            MirOwnershipKindV1::None,
        ],
        4,
    );
    branch(&mut function, 0, 2, 1, 2);
    function
        .get_block_mut(bb(1))
        .unwrap()
        .add_instruction(MirInstruction::CopyOwned {
            dst: value(10),
            src: value(0),
        });
    function
        .get_block_mut(bb(2))
        .unwrap()
        .add_instruction(MirInstruction::CopyOwned {
            dst: value(11),
            src: value(1),
        });
    jump(&mut function, 1, 3);
    jump(&mut function, 2, 3);
    phi(&mut function, 3, 12, &[(1, 10), (2, 11)]);
    ret(&mut function, 3, 12);

    let verified = verify_ownership_ssa_v1(&function, &abi).unwrap();
    assert_eq!(verified.owner(), OwnershipFunctionOwnerV1::new(7));
    assert_eq!(verified.kind(value(12)), Some(MirOwnershipKindV1::Owned));
    assert_eq!(verified.dispositions(value(10)).len(), 1);
}

#[test]
fn one_prebranch_token_can_forward_on_mutually_exclusive_edges() {
    let (mut function, abi) = function(&[MirOwnershipKindV1::Owned, MirOwnershipKindV1::None], 4);
    branch(&mut function, 0, 1, 2, 3);
    jump(&mut function, 2, 1);
    jump(&mut function, 3, 1);
    phi(&mut function, 1, 10, &[(2, 0), (3, 0)]);
    ret(&mut function, 1, 10);
    assert!(verify_ownership_ssa_v1(&function, &abi).is_ok());
}

#[test]
fn one_branch_forwards_while_the_other_destroys_and_replaces() {
    let (mut function, abi) = function(
        &[
            MirOwnershipKindV1::Owned,
            MirOwnershipKindV1::Borrowed,
            MirOwnershipKindV1::None,
        ],
        4,
    );
    branch(&mut function, 0, 2, 1, 2);
    function
        .get_block_mut(bb(2))
        .unwrap()
        .add_instruction(MirInstruction::DestroyOwned { value: value(0) });
    function
        .get_block_mut(bb(2))
        .unwrap()
        .add_instruction(MirInstruction::CopyOwned {
            dst: value(10),
            src: value(1),
        });
    jump(&mut function, 1, 3);
    jump(&mut function, 2, 3);
    phi(&mut function, 3, 11, &[(1, 0), (2, 10)]);
    ret(&mut function, 3, 11);
    assert!(verify_ownership_ssa_v1(&function, &abi).is_ok());
}

#[test]
fn branch_without_a_disposition_is_rejected() {
    let (mut function, mut abi) =
        function(&[MirOwnershipKindV1::Owned, MirOwnershipKindV1::None], 4);
    abi = OwnershipFunctionAbiV1::new(
        abi.owner(),
        abi.parameter_kinds().to_vec(),
        FunctionResultOwnershipV1::None,
    );
    branch(&mut function, 0, 1, 2, 3);
    function
        .get_block_mut(bb(2))
        .unwrap()
        .add_instruction(MirInstruction::DestroyOwned { value: value(0) });
    jump(&mut function, 2, 1);
    jump(&mut function, 3, 1);
    function
        .get_block_mut(bb(1))
        .unwrap()
        .set_terminator(MirInstruction::Return { value: None });
    assert!(matches!(
        verify_ownership_ssa_v1(&function, &abi),
        Err(OwnershipSsaErrorV1::LiveSetMismatch { .. })
    ));
}

#[test]
fn same_source_forwarded_twice_on_one_edge_is_rejected() {
    let (mut function, abi) = function(&[MirOwnershipKindV1::Owned], 2);
    jump(&mut function, 0, 1);
    phi(&mut function, 1, 10, &[(0, 0)]);
    phi(&mut function, 1, 11, &[(0, 0)]);
    ret(&mut function, 1, 10);
    assert!(matches!(
        verify_ownership_ssa_v1(&function, &abi),
        Err(OwnershipSsaErrorV1::DuplicateConsumeOnEdge { .. })
    ));
}

#[test]
fn phi_source_use_after_forwarding_is_rejected() {
    let (mut function, abi) = function(&[MirOwnershipKindV1::Owned], 2);
    jump(&mut function, 0, 1);
    phi(&mut function, 1, 10, &[(0, 0)]);
    function
        .get_block_mut(bb(1))
        .unwrap()
        .add_instruction(MirInstruction::DestroyOwned { value: value(0) });
    ret(&mut function, 1, 10);
    assert!(
        matches!(verify_ownership_ssa_v1(&function, &abi), Err(OwnershipSsaErrorV1::OwnedUseAfterConsume { value: v, .. }) if v == value(0))
    );
}

#[test]
fn loop_header_phi_forwards_entry_and_backedge_tokens() {
    let (mut function, abi) = function(
        &[
            MirOwnershipKindV1::Owned,
            MirOwnershipKindV1::Borrowed,
            MirOwnershipKindV1::None,
        ],
        4,
    );
    jump(&mut function, 0, 1);
    phi(&mut function, 1, 10, &[(0, 0), (2, 11)]);
    branch(&mut function, 1, 2, 2, 3);
    function
        .get_block_mut(bb(2))
        .unwrap()
        .add_instruction(MirInstruction::DestroyOwned { value: value(10) });
    function
        .get_block_mut(bb(2))
        .unwrap()
        .add_instruction(MirInstruction::CopyOwned {
            dst: value(11),
            src: value(1),
        });
    jump(&mut function, 2, 1);
    ret(&mut function, 3, 10);
    assert!(verify_ownership_ssa_v1(&function, &abi).is_ok());
}

#[test]
fn multiple_owned_phis_transfer_in_parallel() {
    let (mut function, abi) = function(&[MirOwnershipKindV1::Owned, MirOwnershipKindV1::Owned], 2);
    jump(&mut function, 0, 1);
    phi(&mut function, 1, 10, &[(0, 1)]);
    phi(&mut function, 1, 11, &[(0, 0)]);
    function
        .get_block_mut(bb(1))
        .unwrap()
        .add_instruction(MirInstruction::DestroyOwned { value: value(10) });
    ret(&mut function, 1, 11);
    assert!(verify_ownership_ssa_v1(&function, &abi).is_ok());
}

#[test]
fn unreachable_block_is_rejected() {
    let (mut function, abi) = function(&[MirOwnershipKindV1::Owned], 2);
    ret(&mut function, 0, 0);
    ret(&mut function, 1, 0);
    assert!(
        matches!(verify_ownership_ssa_v1(&function, &abi), Err(OwnershipSsaErrorV1::UnreachableBlock { block }) if block == bb(1))
    );
}

#[test]
fn phi_input_from_non_predecessor_is_rejected() {
    let (mut function, abi) = function(&[MirOwnershipKindV1::Owned], 2);
    jump(&mut function, 0, 1);
    phi(&mut function, 1, 10, &[(99, 0)]);
    ret(&mut function, 1, 10);
    assert!(matches!(
        verify_ownership_ssa_v1(&function, &abi),
        Err(OwnershipSsaErrorV1::PhiPredecessorMismatch { .. })
    ));
}

#[test]
fn borrowed_phi_is_rejected() {
    let (mut function, abi) = function(&[MirOwnershipKindV1::Borrowed], 2);
    jump(&mut function, 0, 1);
    phi(&mut function, 1, 10, &[(0, 0)]);
    ret(&mut function, 1, 10);
    assert!(matches!(
        verify_ownership_ssa_v1(&function, &abi),
        Err(OwnershipSsaErrorV1::BorrowedPhiForbidden { .. })
    ));
}

#[test]
fn borrowed_return_is_rejected() {
    let (mut function, abi) = function(&[MirOwnershipKindV1::Borrowed], 1);
    ret(&mut function, 0, 0);
    assert!(matches!(
        verify_ownership_ssa_v1(&function, &abi),
        Err(OwnershipSsaErrorV1::BorrowedReturnForbidden { .. })
    ));
}

#[test]
fn ordinary_copy_of_owned_value_is_rejected() {
    let (mut function, abi) = function(&[MirOwnershipKindV1::Owned], 1);
    function
        .get_block_mut(bb(0))
        .unwrap()
        .add_instruction(MirInstruction::Copy {
            dst: value(10),
            src: value(0),
        });
    ret(&mut function, 0, 0);
    assert!(matches!(
        verify_ownership_ssa_v1(&function, &abi),
        Err(OwnershipSsaErrorV1::CopyOnOwned { .. })
    ));
}

#[test]
fn duplicate_destroy_is_rejected_as_use_after_consume() {
    let (mut function, abi) = function(&[MirOwnershipKindV1::Owned], 1);
    let abi = OwnershipFunctionAbiV1::new(
        abi.owner(),
        vec![MirOwnershipKindV1::Owned],
        FunctionResultOwnershipV1::None,
    );
    for _ in 0..2 {
        function
            .get_block_mut(bb(0))
            .unwrap()
            .add_instruction(MirInstruction::DestroyOwned { value: value(0) });
    }
    function
        .get_block_mut(bb(0))
        .unwrap()
        .set_terminator(MirInstruction::Return { value: None });
    assert!(matches!(
        verify_ownership_ssa_v1(&function, &abi),
        Err(OwnershipSsaErrorV1::OwnedUseAfterConsume { .. })
    ));
}

#[test]
fn canonical_edge_arguments_are_rejected() {
    let (mut function, abi) = function(&[MirOwnershipKindV1::Owned], 2);
    function
        .get_block_mut(bb(0))
        .unwrap()
        .set_terminator(MirInstruction::Jump {
            target: bb(1),
            edge_args: Some(EdgeArgs {
                layout: JumpArgsLayout::CarriersOnly,
                values: vec![value(0)],
            }),
        });
    function
        .get_block_mut(bb(1))
        .unwrap()
        .add_predecessor(bb(0));
    ret(&mut function, 1, 0);
    assert!(matches!(
        verify_ownership_ssa_v1(&function, &abi),
        Err(OwnershipSsaErrorV1::EdgeArgumentsForbidden { .. })
    ));
}

#[test]
fn managed_call_shape_without_abi_witness_is_rejected() {
    let (mut function, abi) = function(&[MirOwnershipKindV1::Borrowed], 1);
    let abi = OwnershipFunctionAbiV1::new(
        abi.owner(),
        vec![MirOwnershipKindV1::Borrowed],
        FunctionResultOwnershipV1::None,
    );
    function
        .get_block_mut(bb(0))
        .unwrap()
        .add_instruction(MirInstruction::Call {
            dst: None,
            func: value(0),
            callee: None,
            args: vec![],
            effects: EffectMask::PURE,
        });
    function
        .get_block_mut(bb(0))
        .unwrap()
        .set_terminator(MirInstruction::Return { value: None });
    assert!(matches!(
        verify_ownership_ssa_v1(&function, &abi),
        Err(OwnershipSsaErrorV1::ManagedCallOwnershipUnsupported { .. })
    ));
}

#[test]
fn typed_targetless_call_ignores_legacy_func() {
    let shapes = [
        Callee::Global(crate::mir::test_global_target("owner/0".to_string())),
        Callee::Extern("env.owner/0".to_string()),
        Callee::Constructor {
            box_type: "OwnerBox".to_string(),
        },
        Callee::Method {
            box_name: "OwnerBox".to_string(),
            method: "owner".to_string(),
            receiver: None,
            certainty: TypeCertainty::Known,
            box_kind: CalleeBoxKind::UserDefined,
        },
    ];
    for callee in shapes {
        let (function, abi) = none_result_call(Some(callee), ValueId::new(999), vec![], None);
        assert!(verify_ownership_ssa_v1(&function, &abi).is_ok());
    }
}

#[test]
fn typed_method_and_value_targets_accept_known_trivial_values() {
    let shapes = [
        Callee::Method {
            box_name: "OwnerBox".to_string(),
            method: "owner".to_string(),
            receiver: Some(value(10)),
            certainty: TypeCertainty::Known,
            box_kind: CalleeBoxKind::RuntimeData,
        },
        Callee::Value(value(10)),
    ];
    for callee in shapes {
        let (mut function, abi) = none_result_call(Some(callee), ValueId::INVALID, vec![], None);
        define_trivial_value(&mut function, 10);
        assert!(verify_ownership_ssa_v1(&function, &abi).is_ok());
    }
}

#[test]
fn typed_method_and_value_targets_reject_managed_or_unknown_values() {
    let (mut borrowed_function, borrowed_abi) = function(&[MirOwnershipKindV1::Borrowed], 1);
    let borrowed_abi = OwnershipFunctionAbiV1::new(
        borrowed_abi.owner(),
        vec![MirOwnershipKindV1::Borrowed],
        FunctionResultOwnershipV1::None,
    );
    borrowed_function
        .get_block_mut(bb(0))
        .unwrap()
        .add_instruction(MirInstruction::Call {
            dst: None,
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: "OwnerBox".to_string(),
                method: "owner".to_string(),
                receiver: Some(value(0)),
                certainty: TypeCertainty::Known,
                box_kind: CalleeBoxKind::RuntimeData,
            }),
            args: vec![],
            effects: EffectMask::PURE,
        });
    borrowed_function
        .get_block_mut(bb(0))
        .unwrap()
        .set_terminator(MirInstruction::Return { value: None });
    assert!(matches!(
        verify_ownership_ssa_v1(&borrowed_function, &borrowed_abi),
        Err(OwnershipSsaErrorV1::ManagedCallOwnershipUnsupported { .. })
    ));

    let (unknown_function, unknown_abi) = none_result_call(
        Some(Callee::Value(value(999))),
        ValueId::INVALID,
        vec![],
        None,
    );
    assert!(matches!(
        verify_ownership_ssa_v1(&unknown_function, &unknown_abi),
        Err(OwnershipSsaErrorV1::ManagedCallOwnershipUnsupported { .. })
    ));
}

#[test]
fn typed_managed_target_fails_before_generic_liveness() {
    let (mut function, abi) = function(&[MirOwnershipKindV1::Owned], 1);
    let abi = OwnershipFunctionAbiV1::new(
        abi.owner(),
        vec![MirOwnershipKindV1::Owned],
        FunctionResultOwnershipV1::None,
    );
    function
        .get_block_mut(bb(0))
        .unwrap()
        .add_instruction(MirInstruction::Call {
            dst: None,
            func: ValueId::INVALID,
            callee: Some(Callee::Value(value(0))),
            args: vec![],
            effects: EffectMask::PURE,
        });
    function
        .get_block_mut(bb(0))
        .unwrap()
        .set_terminator(MirInstruction::Return { value: None });
    assert!(matches!(
        verify_ownership_ssa_v1(&function, &abi),
        Err(OwnershipSsaErrorV1::ManagedCallOwnershipUnsupported { .. })
    ));
}

#[test]
fn typed_call_arguments_keep_the_existing_trivial_only_policy() {
    let (mut function, abi) = function(&[MirOwnershipKindV1::Borrowed], 1);
    let abi = OwnershipFunctionAbiV1::new(
        abi.owner(),
        vec![MirOwnershipKindV1::Borrowed],
        FunctionResultOwnershipV1::None,
    );
    function
        .get_block_mut(bb(0))
        .unwrap()
        .add_instruction(MirInstruction::Call {
            dst: None,
            func: ValueId::INVALID,
            callee: Some(Callee::Global(crate::mir::test_global_target(
                "owner/1".to_string(),
            ))),
            args: vec![value(0)],
            effects: EffectMask::PURE,
        });
    function
        .get_block_mut(bb(0))
        .unwrap()
        .set_terminator(MirInstruction::Return { value: None });
    assert!(matches!(
        verify_ownership_ssa_v1(&function, &abi),
        Err(OwnershipSsaErrorV1::ManagedCallOwnershipUnsupported { .. })
    ));
}

#[test]
fn typed_closure_operands_use_generic_liveness_not_managed_call_policy() {
    let (mut function, abi) = none_result_call(
        Some(Callee::Closure {
            params: vec!["x".to_string()],
            captures: vec![("capture".to_string(), value(10))],
            me_capture: Some(value(11)),
        }),
        ValueId::new(999),
        vec![],
        None,
    );
    define_trivial_value(&mut function, 10);
    define_trivial_value(&mut function, 11);
    assert!(verify_ownership_ssa_v1(&function, &abi).is_ok());
}

#[test]
fn legacy_call_still_requires_a_known_trivial_func() {
    let (function, abi) = none_result_call(None, ValueId::new(999), vec![], None);
    assert!(matches!(
        verify_ownership_ssa_v1(&function, &abi),
        Err(OwnershipSsaErrorV1::ManagedCallOwnershipUnsupported { .. })
    ));
}

#[test]
fn copy_owned_from_trivial_value_is_rejected() {
    let (mut function, abi) = function(&[MirOwnershipKindV1::None], 1);
    function
        .get_block_mut(bb(0))
        .unwrap()
        .add_instruction(MirInstruction::CopyOwned {
            dst: value(10),
            src: value(0),
        });
    ret(&mut function, 0, 10);
    assert!(matches!(
        verify_ownership_ssa_v1(&function, &abi),
        Err(OwnershipSsaErrorV1::CopyOwnedSourceNotStrong { .. })
    ));
}
