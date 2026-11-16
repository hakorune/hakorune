use super::*;
use crate::mir::compiler::common_v2_physical_function_entry_input::PhysicalCallableLaneCarrierV1;
use crate::mir::{BasicBlock, ConstValue, EffectMask, FunctionSignature};
use SameI64Definition::{Different, Same, Unresolved};

fn function() -> MirFunction {
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "origin/2".into(),
            params: vec![MirType::Integer; 2],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId(0),
    );
    function.metadata.physical_param_carriers =
        Some(vec![PhysicalCallableLaneCarrierV1::ExistingCallableI64; 2].into());
    function.metadata.declared_param_decls = (0..2)
        .map(|i| crate::mir::function::MirParamDecl {
            name: format!("p{i}"),
            declared_type_name: Some("i64".into()),
            implicit_receiver: false,
        })
        .collect();
    function
}
fn block(function: &mut MirFunction, id: u32, rows: Vec<MirInstruction>, end: MirInstruction) {
    let mut block = BasicBlock::new(BasicBlockId(id));
    for row in rows {
        block.add_instruction(row);
    }
    if id == 0 {
        block.add_instruction(MirInstruction::Const {
            dst: ValueId(90),
            value: ConstValue::Bool(true),
        });
    }
    block.set_terminator(end);
    function.add_block(block);
    function.update_cfg();
}
fn copy(dst: u32, src: u32) -> MirInstruction {
    MirInstruction::Copy {
        dst: ValueId(dst),
        src: ValueId(src),
    }
}
fn phi(dst: u32, inputs: &[(u32, u32)]) -> MirInstruction {
    MirInstruction::Phi {
        dst: ValueId(dst),
        inputs: inputs
            .iter()
            .map(|(b, v)| (BasicBlockId(*b), ValueId(*v)))
            .collect(),
        type_hint: Some(MirType::Integer),
    }
}
fn jump(to: u32) -> MirInstruction {
    MirInstruction::Jump {
        target: BasicBlockId(to),
        edge_args: None,
    }
}
fn ret(v: u32) -> MirInstruction {
    MirInstruction::Return {
        value: Some(ValueId(v)),
    }
}
fn branch(a: u32, b: u32) -> MirInstruction {
    MirInstruction::Branch {
        condition: ValueId(90),
        then_bb: BasicBlockId(a),
        else_bb: BasicBlockId(b),
        then_edge_args: None,
        else_edge_args: None,
    }
}
fn answer(function: &MirFunction, v: u32, expected: u32) -> SameI64Definition {
    same_i64_definition(function, None, ValueId(v), ValueId(expected)).unwrap()
}
fn diamond() -> MirFunction {
    let mut f = function();
    block(&mut f, 0, vec![], branch(1, 2));
    block(&mut f, 1, vec![phi(2, &[(0, 0)]), copy(3, 2)], jump(3));
    block(&mut f, 2, vec![copy(4, 0)], jump(3));
    block(&mut f, 3, vec![phi(5, &[(1, 3), (2, 4)])], ret(5));
    f
}

#[test]
fn copies_and_all_incoming_nested_phi_preserve_definition() {
    let mut f = diamond();
    assert_eq!(answer(&f, 3, 0), Same); // one predecessor and multiple Copy/PHI levels
    assert_eq!(answer(&f, 5, 0), Same);
    block(&mut f, 3, vec![phi(5, &[(1, 3), (2, 4)])], branch(4, 5));
    block(&mut f, 4, vec![copy(6, 5)], jump(6));
    block(&mut f, 5, vec![phi(7, &[(3, 5)])], jump(6));
    block(
        &mut f,
        6,
        vec![phi(8, &[(4, 6), (5, 7)]), copy(9, 8)],
        ret(9),
    );
    assert_eq!(answer(&f, 9, 0), Same);
}
#[test]
fn one_changed_incoming_or_binding_is_not_same_definition() {
    let mut f = diamond();
    {
        let geometry = FunctionVerificationGeometry::new(&f);
        let analysis = FinishedI64OriginsV1::new(&geometry, None);
        assert_eq!(
            analysis.same_definition(ValueId(5), ValueId(0)).unwrap(),
            Same
        );
        assert_eq!(
            analysis.same_definition(ValueId(5), ValueId(1)).unwrap(),
            Different
        );
        assert_eq!(
            analysis.same_definition(ValueId(3), ValueId(0)).unwrap(),
            Same
        );
    }
    block(&mut f, 2, vec![copy(4, 1)], jump(3));
    assert_eq!(answer(&f, 5, 0), Different);
    assert_eq!(answer(&f, 3, 1), Different);
    // Fresh query after a rewrite cannot reuse the old successful geometry.
    block(&mut f, 2, vec![copy(4, 0)], jump(3));
    assert_eq!(answer(&f, 5, 0), Same);
    block(&mut f, 2, vec![copy(4, 1)], jump(3));
    assert_eq!(answer(&f, 5, 0), Different);
}
#[test]
fn malformed_phi_edges_are_errors() {
    for inputs in [
        vec![(1, 3)],
        vec![(1, 3), (0, 0)],
        vec![(1, 3), (1, 3), (2, 4)],
    ] {
        let mut f = diamond();
        block(&mut f, 3, vec![phi(5, &inputs)], ret(5));
        assert!(same_i64_definition(&f, None, ValueId(5), ValueId(0))
            .unwrap_err()
            .iter()
            .any(|e| matches!(e, VerificationError::InvalidPhi { .. })));
    }
}
#[test]
fn dominance_and_same_block_future_definitions_are_errors() {
    let mut f = diamond();
    block(&mut f, 3, vec![phi(5, &[(1, 3), (2, 3)])], ret(5));
    assert!(same_i64_definition(&f, None, ValueId(5), ValueId(0))
        .unwrap_err()
        .iter()
        .any(|e| matches!(e, VerificationError::DominatorViolation { .. })));
    let mut f = function();
    block(&mut f, 0, vec![copy(2, 3), copy(3, 0)], ret(2));
    assert!(same_i64_definition(&f, None, ValueId(2), ValueId(0))
        .unwrap_err()
        .iter()
        .any(|e| matches!(e, VerificationError::DominatorViolation { .. })));
}
#[test]
fn duplicate_definitions_and_parameter_collision_are_errors() {
    for rows in [vec![copy(2, 0), copy(2, 1)], vec![copy(0, 1)]] {
        let mut f = function();
        block(&mut f, 0, rows, ret(0));
        assert!(same_i64_definition(&f, None, ValueId(0), ValueId(0))
            .unwrap_err()
            .iter()
            .any(|e| matches!(e, VerificationError::MultipleDefinition { .. })));
    }
}
#[test]
fn grounded_cycle_and_distinct_integer_definitions_keep_their_identity() {
    let mut f = function();
    block(&mut f, 0, vec![], jump(1));
    block(&mut f, 1, vec![phi(2, &[(0, 0), (2, 3)])], branch(2, 3));
    block(&mut f, 2, vec![copy(3, 2)], jump(1));
    block(&mut f, 3, vec![], ret(2));
    assert_eq!(answer(&f, 2, 0), Same);
    let mut f = function();
    block(
        &mut f,
        0,
        vec![MirInstruction::Const {
            dst: ValueId(2),
            value: ConstValue::Integer(7),
        }],
        ret(2),
    );
    assert_eq!(answer(&f, 2, 0), Different);
    assert_eq!(answer(&f, 2, 2), Same);
    let mut second = f.clone();
    second
        .blocks
        .get_mut(&BasicBlockId(0))
        .unwrap()
        .instructions
        .push(MirInstruction::Const {
            dst: ValueId(3),
            value: ConstValue::Integer(7),
        });
    second
        .blocks
        .get_mut(&BasicBlockId(0))
        .unwrap()
        .set_terminator(ret(3));
    assert_eq!(answer(&second, 3, 2), Different); // equal numbers, distinct definitions
    block(&mut f, 1, vec![copy(3, 0)], ret(3));
    assert_eq!(answer(&f, 3, 0), Unresolved);
}
#[test]
fn integer_wire_is_not_tagged_unsigned_or_object_identity() {
    for carrier in [
        PhysicalCallableLaneCarrierV1::BorrowedTaggedValue,
        PhysicalCallableLaneCarrierV1::U64BitsOnI64,
        PhysicalCallableLaneCarrierV1::CheckedMapStorage,
    ] {
        let mut f = function();
        block(&mut f, 0, vec![copy(2, 0)], ret(2));
        f.metadata.physical_param_carriers.as_mut().unwrap()[0] = carrier;
        assert_eq!(answer(&f, 2, 0), Unresolved);
    }
    let mut f = function();
    block(&mut f, 0, vec![copy(2, 0)], ret(2));
    f.metadata
        .value_types
        .insert(ValueId(2), MirType::Box("Thing".into()));
    assert_eq!(answer(&f, 2, 0), Unresolved);
    f.metadata.value_types.clear();
    f.metadata.physical_param_carriers = None;
    assert_eq!(answer(&f, 2, 0), Unresolved);
}

fn call_module() -> (MirModule, crate::mir::definitions::MirCall) {
    let key = hakorune_mir_defs::CanonicalSameModuleCallableKeyV1::free_function("identity", 2);
    let mut callee = function();
    callee.signature.name = key.mir_symbol_projection();
    callee.metadata.declared_return_type_name = Some("i64".into());
    block(&mut callee, 0, vec![], ret(0));
    let mut module = MirModule::new("origins".into());
    module
        .add_cataloged_box_method(key.clone(), callee)
        .unwrap();
    let call = crate::mir::definitions::MirCall::global(
        Some(ValueId(2)),
        key.canonical_global_target_v1().unwrap(),
        vec![ValueId(0), ValueId(1)],
    );
    (module, call)
}
#[test]
fn cataloged_raw_call_is_a_distinct_actual_definition() {
    let (module, call) = call_module();
    let mut f = function();
    let mut second = call.clone();
    second.dst = Some(ValueId(3));
    block(
        &mut f,
        0,
        vec![
            MirInstruction::Call(call),
            MirInstruction::Call(second),
            copy(4, 2),
        ],
        ret(4),
    );
    assert_eq!(
        same_i64_definition(&f, Some(&module), ValueId(4), ValueId(2)).unwrap(),
        Same
    );
    assert_eq!(
        same_i64_definition(&f, Some(&module), ValueId(3), ValueId(2)).unwrap(),
        Different
    );
    assert_eq!(
        same_i64_definition(&f, None, ValueId(4), ValueId(2)).unwrap(),
        Unresolved
    );
    let mut inferred = module.clone();
    inferred
        .functions
        .get_mut("identity/2")
        .unwrap()
        .metadata
        .declared_return_type_name = None;
    assert_eq!(
        same_i64_definition(&f, Some(&inferred), ValueId(4), ValueId(2)).unwrap(),
        Same
    );
}
#[test]
fn invoke_normal_result_uses_actual_normal_edge_and_callee() {
    use crate::mir::instruction::{FaultFrameMode, InvokeCallResultKind, InvokeOperation};
    let (mut module, mut call) = call_module();
    call.dst = None;
    let mut f = function();
    block(
        &mut f,
        0,
        vec![MirInstruction::FaultFrameEnter {
            dst: ValueId(9),
            mode: FaultFrameMode::RootOwned,
        }],
        jump(1),
    );
    block(
        &mut f,
        1,
        vec![],
        MirInstruction::Invoke {
            operation: InvokeOperation::Call {
                call,
                result: InvokeCallResultKind::I64,
            },
            fault_frame: ValueId(9),
            normal_landing: BasicBlockId(2),
            fault_landing: BasicBlockId(3),
        },
    );
    block(
        &mut f,
        2,
        vec![
            MirInstruction::InvokeNormalResult {
                invoke_block: BasicBlockId(1),
                dst: ValueId(2),
            },
            copy(3, 2),
        ],
        ret(3),
    );
    block(
        &mut f,
        3,
        vec![],
        MirInstruction::ReturnFault {
            fault_frame: ValueId(9),
        },
    );
    assert_eq!(
        same_i64_definition(&f, Some(&module), ValueId(3), ValueId(2)).unwrap(),
        Same
    );
    // Inferred scalar callees keep an Integer signature without an annotation.
    module
        .functions
        .get_mut("identity/2")
        .unwrap()
        .metadata
        .declared_return_type_name = None;
    assert_eq!(
        same_i64_definition(&f, Some(&module), ValueId(3), ValueId(2)).unwrap(),
        Same
    );
    module
        .functions
        .get_mut("identity/2")
        .unwrap()
        .signature
        .return_type = MirType::Box("Thing".into());
    assert_eq!(
        same_i64_definition(&f, Some(&module), ValueId(3), ValueId(2)).unwrap(),
        Unresolved
    );
    module
        .functions
        .get_mut("identity/2")
        .unwrap()
        .signature
        .return_type = MirType::Integer;
    module
        .functions
        .get_mut("identity/2")
        .unwrap()
        .metadata
        .declared_return_type_name = Some("u64".into());
    assert_eq!(
        same_i64_definition(&f, Some(&module), ValueId(3), ValueId(2)).unwrap(),
        Unresolved
    );
    module
        .functions
        .get_mut("identity/2")
        .unwrap()
        .metadata
        .declared_return_type_name = Some("i64".into());
    f.blocks
        .get_mut(&BasicBlockId(3))
        .unwrap()
        .instructions
        .push(copy(4, 2));
    assert!(same_i64_definition(&f, Some(&module), ValueId(3), ValueId(2)).is_err());
}

#[test]
fn incomplete_cfg_and_stale_successors_are_rejected() {
    let mut f = function();
    assert!(same_i64_definition(&f, None, ValueId(0), ValueId(0)).is_err());
    block(&mut f, 0, vec![copy(2, 0)], ret(2));
    let mut malformed = f.clone();
    malformed
        .blocks
        .get_mut(&BasicBlockId(0))
        .unwrap()
        .terminator = Some(copy(3, 0));
    assert!(same_i64_definition(&malformed, None, ValueId(0), ValueId(0)).is_err());
    let mut malformed = f.clone();
    malformed
        .blocks
        .get_mut(&BasicBlockId(0))
        .unwrap()
        .instructions
        .push(ret(0));
    assert!(same_i64_definition(&malformed, None, ValueId(0), ValueId(0)).is_err());
    f.blocks
        .get_mut(&BasicBlockId(0))
        .unwrap()
        .successors
        .insert(BasicBlockId(1));
    assert!(same_i64_definition(&f, None, ValueId(2), ValueId(0)).is_err());
}

#[test]
fn nested_loop_and_inner_if_carry_only_the_original_base() {
    let mut f = function();
    block(&mut f, 0, vec![], jump(1));
    block(&mut f, 1, vec![phi(2, &[(0, 0), (5, 8)])], branch(2, 6));
    block(&mut f, 2, vec![phi(3, &[(1, 2), (4, 7)])], branch(3, 5));
    block(&mut f, 3, vec![], branch(7, 8));
    block(&mut f, 7, vec![copy(4, 3)], jump(9));
    block(&mut f, 8, vec![copy(5, 3)], jump(9));
    block(&mut f, 9, vec![phi(6, &[(7, 4), (8, 5)])], jump(4));
    block(&mut f, 4, vec![copy(7, 6)], jump(2));
    block(&mut f, 5, vec![copy(8, 3)], jump(1));
    block(&mut f, 6, vec![copy(9, 2)], ret(9));
    for value in [2, 3, 6, 9] {
        assert_eq!(answer(&f, value, 0), Same);
    }
    block(&mut f, 4, vec![copy(7, 1)], jump(2));
    assert_eq!(answer(&f, 9, 0), Different); // only the backedge changes
    block(
        &mut f,
        4,
        vec![MirInstruction::BinOp {
            dst: ValueId(7),
            op: crate::mir::BinaryOp::Add,
            lhs: ValueId(3),
            rhs: ValueId(1),
        }],
        jump(2),
    );
    assert_eq!(answer(&f, 9, 0), Unresolved); // no arithmetic equivalence
    block(&mut f, 2, vec![phi(3, &[(1, 2)])], branch(3, 5));
    assert!(same_i64_definition(&f, None, ValueId(9), ValueId(0)).is_err());
}

#[test]
fn every_external_entry_of_a_cycle_must_have_the_same_base() {
    let mut f = function();
    block(&mut f, 0, vec![], branch(1, 2));
    block(&mut f, 1, vec![copy(2, 0)], jump(3));
    block(&mut f, 2, vec![copy(3, 1)], jump(3));
    block(
        &mut f,
        3,
        vec![phi(4, &[(1, 2), (2, 3), (4, 5)])],
        branch(4, 5),
    );
    block(&mut f, 4, vec![copy(5, 4)], jump(3));
    block(&mut f, 5, vec![], ret(4));
    assert_eq!(answer(&f, 4, 0), Different);
    block(&mut f, 2, vec![copy(3, 0)], jump(3));
    assert_eq!(answer(&f, 4, 0), Same);
}

#[test]
fn an_ungrounded_cycle_stays_unresolved_even_beside_a_valid_base() {
    let mut f = function();
    block(&mut f, 0, vec![phi(2, &[(0, 2)])], branch(0, 1));
    block(&mut f, 1, vec![], ret(2));
    assert_eq!(answer(&f, 2, 0), Unresolved);
    block(&mut f, 1, vec![], branch(2, 3));
    block(&mut f, 2, vec![], jump(4));
    block(&mut f, 3, vec![], jump(4));
    block(&mut f, 4, vec![phi(3, &[(2, 2), (3, 0)])], ret(3));
    assert_eq!(answer(&f, 3, 0), Unresolved);
}
