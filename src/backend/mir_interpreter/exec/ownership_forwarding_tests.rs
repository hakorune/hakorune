use super::super::{MirInterpreter, VMError, VMValue};
use crate::box_trait::IntegerBox;
use crate::mir::ownership_ssa::{
    verify_ownership_ssa_v1, FunctionResultOwnershipV1, MirOwnershipKindV1, OwnershipFunctionAbiV1,
    OwnershipFunctionOwnerV1,
};
use crate::mir::{
    BasicBlock, BasicBlockId, EffectMask, FunctionSignature, MirFunction, MirInstruction, MirType,
    ValueId,
};
use std::sync::Arc;

fn bb(raw: u32) -> BasicBlockId {
    BasicBlockId::new(raw)
}

fn value(raw: u32) -> ValueId {
    ValueId::new(raw)
}

fn box_value(raw: i64) -> VMValue {
    VMValue::BoxRef(Arc::new(IntegerBox::new(raw)))
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

fn diamond() -> (
    MirFunction,
    OwnershipFunctionOwnerV1,
    crate::mir::ownership_ssa::VerifiedOwnershipSsaV1,
) {
    let owner = OwnershipFunctionOwnerV1::new(41);
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "Ownership.forward/2".into(),
            params: vec![MirType::Box("OwnedTestBox".into()), MirType::Bool],
            return_type: MirType::Box("OwnedTestBox".into()),
            effects: EffectMask::PURE,
        },
        bb(0),
    );
    for raw in 1..4 {
        function.add_block(BasicBlock::new(bb(raw)));
    }
    branch(&mut function, 0, 1, 1, 2);
    function
        .get_block_mut(bb(1))
        .unwrap()
        .add_instruction(MirInstruction::CopyOwned {
            dst: value(2),
            src: value(0),
        });
    function
        .get_block_mut(bb(2))
        .unwrap()
        .add_instruction(MirInstruction::CopyOwned {
            dst: value(3),
            src: value(0),
        });
    jump(&mut function, 1, 3);
    jump(&mut function, 2, 3);
    function
        .get_block_mut(bb(3))
        .unwrap()
        .add_instruction(MirInstruction::Phi {
            dst: value(4),
            inputs: vec![(bb(1), value(2)), (bb(2), value(3))],
            type_hint: Some(MirType::Box("OwnedTestBox".into())),
        });
    function
        .get_block_mut(bb(3))
        .unwrap()
        .set_terminator(MirInstruction::Return {
            value: Some(value(4)),
        });
    let abi = OwnershipFunctionAbiV1::new(
        owner,
        vec![MirOwnershipKindV1::Borrowed, MirOwnershipKindV1::None],
        FunctionResultOwnershipV1::Owned,
    );
    let witness = verify_ownership_ssa_v1(&function, &abi).unwrap();
    (function, owner, witness)
}

#[test]
fn verified_session_moves_owned_phi_and_return() {
    let (function, owner, witness) = diamond();
    for condition in [true, false] {
        let mut interpreter = MirInterpreter::new();
        let result = interpreter
            .exec_function_inner_with_verified_ownership(
                &function,
                vec![box_value(7), VMValue::Bool(condition)],
                owner,
                witness.clone(),
            )
            .unwrap();
        assert!(matches!(result, VMValue::BoxRef(_)));
        assert!(interpreter.active_ownership_ssa.is_none());
    }
}

#[test]
fn foreign_owner_is_rejected_before_frame_install() {
    let (function, _, witness) = diamond();
    let mut interpreter = MirInterpreter::new();
    let error = interpreter
        .exec_function_inner_with_verified_ownership(
            &function,
            vec![box_value(7), VMValue::Bool(true)],
            OwnershipFunctionOwnerV1::new(99),
            witness,
        )
        .unwrap_err();
    assert!(
        matches!(error, VMError::InvalidInstruction(message) if message.contains("foreign_function_owner"))
    );
    assert!(interpreter.active_ownership_ssa.is_none());
}

#[test]
fn verified_session_restores_prior_witness_on_error() {
    let (mut function, owner, witness) = diamond();
    function.get_block_mut(bb(1)).unwrap().instructions.clear();
    let mut interpreter = MirInterpreter::new();
    let error = interpreter
        .exec_function_inner_with_verified_ownership(
            &function,
            vec![box_value(7), VMValue::Bool(true)],
            owner,
            witness,
        )
        .unwrap_err();
    assert!(matches!(error, VMError::InvalidValue(_)));
    assert!(interpreter.active_ownership_ssa.is_none());
}
