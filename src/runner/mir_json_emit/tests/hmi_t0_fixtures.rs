use super::super::build_mir_json_root;
use crate::mir::ownership_ssa::{
    verify_ownership_ssa_v1, FunctionResultOwnershipV1, MirOwnershipKindV1, OwnershipFunctionAbiV1,
    OwnershipFunctionOwnerV1,
};
use crate::mir::{
    storage_class::StorageClass, BasicBlock, BasicBlockId, BinaryOp, ConstValue, EffectMask,
    FunctionSignature, MirFunction, MirInstruction, MirModule, MirType, ValueId,
};

const SCALAR_FIXTURE: &str =
    include_str!("../../../../tools/hako_shared/hmi/tests/fixtures/scalar_suite_v1.json");
const OWNERSHIP_FIXTURE: &str =
    include_str!("../../../../tools/hako_shared/hmi/tests/fixtures/ownership_transport_v1.json");

fn id(value: u32) -> ValueId {
    ValueId::new(value)
}

fn block(value: u32) -> BasicBlockId {
    BasicBlockId::new(value)
}

fn signature(name: &str, params: Vec<MirType>, result: MirType) -> FunctionSignature {
    FunctionSignature {
        name: name.to_string(),
        params,
        return_type: result,
        effects: EffectMask::PURE,
    }
}

fn set_type(function: &mut MirFunction, value: u32, ty: MirType) {
    function.metadata.value_types.insert(id(value), ty);
}

fn add_block(function: &mut MirFunction, mut basic_block: BasicBlock, reachable: bool) {
    basic_block.reachable = reachable;
    function.add_block(basic_block);
}

fn build_scalar_cfg_function() -> MirFunction {
    let mut function = MirFunction::new(
        signature("hmi_scalar_cfg", vec![], MirType::Integer),
        block(7),
    );
    for value in [2, 3, 4, 5, 6, 7, 8] {
        set_type(&mut function, value, MirType::Integer);
    }
    set_type(&mut function, 1, MirType::Bool);

    let entry = function.get_block_mut(block(7)).expect("entry");
    entry.reachable = true;
    entry.add_instruction(MirInstruction::Const {
        dst: id(1),
        value: ConstValue::Bool(true),
    });
    entry.set_terminator(MirInstruction::Branch {
        condition: id(1),
        then_bb: block(8),
        else_bb: block(9),
        then_edge_args: None,
        else_edge_args: None,
    });

    let mut left = BasicBlock::new(block(8));
    left.add_instruction(MirInstruction::Const {
        dst: id(2),
        value: ConstValue::Integer(2),
    });
    left.set_terminator(MirInstruction::Jump {
        target: block(10),
        edge_args: None,
    });
    add_block(&mut function, left, true);

    let mut right = BasicBlock::new(block(9));
    right.add_instruction(MirInstruction::Const {
        dst: id(3),
        value: ConstValue::Integer(3),
    });
    right.set_terminator(MirInstruction::Jump {
        target: block(10),
        edge_args: None,
    });
    add_block(&mut function, right, true);

    let mut merge = BasicBlock::new(block(10));
    merge.add_instruction(MirInstruction::Phi {
        dst: id(4),
        inputs: vec![(block(8), id(2)), (block(9), id(3))],
        type_hint: Some(MirType::Integer),
    });
    merge.add_instruction(MirInstruction::Const {
        dst: id(5),
        value: ConstValue::Integer(1),
    });
    merge.add_instruction(MirInstruction::BinOp {
        dst: id(6),
        op: BinaryOp::Add,
        lhs: id(4),
        rhs: id(5),
    });
    merge.add_instruction(MirInstruction::Copy {
        dst: id(7),
        src: id(6),
    });
    merge.set_terminator(MirInstruction::Return { value: Some(id(7)) });
    add_block(&mut function, merge, true);

    let mut lower_unreachable = BasicBlock::new(block(6));
    lower_unreachable.add_instruction(MirInstruction::Const {
        dst: id(8),
        value: ConstValue::Integer(0),
    });
    lower_unreachable.set_terminator(MirInstruction::Return { value: Some(id(8)) });
    add_block(&mut function, lower_unreachable, false);
    function
}

fn build_parameter_cross_block_function() -> MirFunction {
    let mut function = MirFunction::new(
        signature(
            "parameter_cross_block",
            vec![MirType::Integer],
            MirType::Integer,
        ),
        block(3),
    );
    set_type(&mut function, 0, MirType::Integer);
    set_type(&mut function, 1, MirType::Integer);
    let entry = function.get_block_mut(block(3)).expect("entry");
    entry.reachable = true;
    entry.set_terminator(MirInstruction::Jump {
        target: block(4),
        edge_args: None,
    });
    let mut exit = BasicBlock::new(block(4));
    exit.add_instruction(MirInstruction::Copy {
        dst: id(1),
        src: id(0),
    });
    exit.set_terminator(MirInstruction::Return { value: Some(id(1)) });
    add_block(&mut function, exit, true);
    function
}

fn build_same_successor_function() -> MirFunction {
    let mut function =
        MirFunction::new(signature("same_successor", vec![], MirType::Void), block(0));
    set_type(&mut function, 0, MirType::Bool);
    let entry = function.get_block_mut(block(0)).expect("entry");
    entry.reachable = true;
    entry.add_instruction(MirInstruction::Const {
        dst: id(0),
        value: ConstValue::Bool(true),
    });
    entry.set_terminator(MirInstruction::Branch {
        condition: id(0),
        then_bb: block(1),
        else_bb: block(1),
        then_edge_args: None,
        else_edge_args: None,
    });
    let mut exit = BasicBlock::new(block(1));
    exit.set_terminator(MirInstruction::Return { value: None });
    add_block(&mut function, exit, true);
    function
}

fn build_scalar_ops_function() -> MirFunction {
    let mut function =
        MirFunction::new(signature("scalar_ops", vec![], MirType::Integer), block(0));
    for value in 0..=6 {
        set_type(&mut function, value, MirType::Integer);
    }
    let entry = function.get_block_mut(block(0)).expect("entry");
    entry.reachable = true;
    entry.add_instruction(MirInstruction::Const {
        dst: id(0),
        value: ConstValue::Integer(20),
    });
    entry.add_instruction(MirInstruction::Const {
        dst: id(1),
        value: ConstValue::Integer(3),
    });
    for (dst, op, lhs) in [
        (2, BinaryOp::Sub, 0),
        (3, BinaryOp::Mul, 2),
        (4, BinaryOp::Div, 3),
        (5, BinaryOp::Mod, 4),
    ] {
        entry.add_instruction(MirInstruction::BinOp {
            dst: id(dst),
            op,
            lhs: id(lhs),
            rhs: id(1),
        });
    }
    entry.add_instruction(MirInstruction::Copy {
        dst: id(6),
        src: id(5),
    });
    entry.set_terminator(MirInstruction::Return { value: Some(id(6)) });
    function
}

fn build_scalar_suite_module() -> MirModule {
    let mut bool_return =
        MirFunction::new(signature("bool_return", vec![], MirType::Bool), block(0));
    set_type(&mut bool_return, 0, MirType::Bool);
    let bool_entry = bool_return.get_block_mut(block(0)).expect("entry");
    bool_entry.reachable = true;
    bool_entry.add_instruction(MirInstruction::Const {
        dst: id(0),
        value: ConstValue::Bool(true),
    });
    bool_entry.set_terminator(MirInstruction::Return { value: Some(id(0)) });

    let mut no_value = MirFunction::new(signature("no_value", vec![], MirType::Void), block(0));
    let no_value_entry = no_value.get_block_mut(block(0)).expect("entry");
    no_value_entry.reachable = true;
    no_value_entry.set_terminator(MirInstruction::Return { value: None });

    let mut module = MirModule::new("hmi_t0_scalar_suite".to_string());
    for function in [
        build_scalar_cfg_function(),
        build_parameter_cross_block_function(),
        build_same_successor_function(),
        build_scalar_ops_function(),
        bool_return,
        no_value,
    ] {
        module.add_function(function);
    }
    module
}

fn build_ownership_module() -> MirModule {
    let mut function = MirFunction::new(
        signature(
            "ownership_transport",
            vec![MirType::Box("WidgetBox".to_string())],
            MirType::Void,
        ),
        block(0),
    );
    let src = id(0);
    let dst = id(1);
    set_type(&mut function, 0, MirType::Box("WidgetBox".to_string()));
    set_type(&mut function, 1, MirType::Box("WidgetBox".to_string()));
    function
        .metadata
        .value_storage_classes
        .insert(src, StorageClass::BoxRef);
    function
        .metadata
        .value_storage_classes
        .insert(dst, StorageClass::BoxRef);
    let entry = function.get_block_mut(block(0)).expect("entry");
    entry.reachable = true;
    entry.add_instruction(MirInstruction::CopyOwned { dst, src });
    entry.add_instruction(MirInstruction::DestroyOwned { value: dst });
    entry.set_terminator(MirInstruction::Return { value: None });
    let abi = OwnershipFunctionAbiV1::new(
        OwnershipFunctionOwnerV1::new(41),
        vec![MirOwnershipKindV1::Borrowed],
        FunctionResultOwnershipV1::None,
    );
    function.metadata.ownership_ssa_v1 =
        Some(verify_ownership_ssa_v1(&function, &abi).expect("ownership witness"));
    let mut module = MirModule::new("hmi_t0_ownership".to_string());
    module.add_function(function);
    module
}

fn emitted_fixture(module: &MirModule) -> String {
    let root = build_mir_json_root(module).expect("build HMI T0 MIR JSON");
    format!(
        "{}\n",
        serde_json::to_string(&root).expect("serialize HMI T0 MIR JSON")
    )
}

#[test]
#[ignore = "explicit checked-in HMI T0 fixture regeneration"]
fn regenerate_checked_in_hmi_t0_fixtures() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tools/hako_shared/hmi/tests/fixtures");
    std::fs::write(
        root.join("scalar_suite_v1.json"),
        emitted_fixture(&build_scalar_suite_module()),
    )
    .expect("write scalar HMI T0 fixture");
    std::fs::write(
        root.join("ownership_transport_v1.json"),
        emitted_fixture(&build_ownership_module()),
    )
    .expect("write ownership HMI T0 fixture");
}

#[test]
fn scalar_suite_matches_checked_in_hmi_t0_fixture() {
    let emitted = emitted_fixture(&build_scalar_suite_module());
    assert_eq!(emitted, SCALAR_FIXTURE);
}

#[test]
fn ownership_transport_matches_checked_in_hmi_t0_fixture() {
    let emitted = emitted_fixture(&build_ownership_module());
    assert_eq!(emitted, OWNERSHIP_FIXTURE);
}
