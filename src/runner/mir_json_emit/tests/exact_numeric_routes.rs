use super::super::build_mir_json_root;
use super::make_function;
use crate::mir::exact_numeric_value_facts::{
    ExactNumericBinaryOpRouteFact, ExactNumericCompareRouteFact, ExactNumericShiftRouteFact,
};
use crate::mir::function::{
    ExactNumericRuntimeCheckContract, ExactNumericRuntimeCheckContractKind,
};
use crate::mir::{BasicBlockId, BinaryOp, CompareOp, MirInstruction, MirModule, ValueId};

#[test]
fn mir_json_exact_numeric_routes_emit_operation_facts() {
    let mut function = make_function("main", true);
    function
        .metadata
        .exact_numeric_binary_op_route_facts
        .push(ExactNumericBinaryOpRouteFact {
            block: BasicBlockId::new(7),
            instruction_index: 3,
            dst: ValueId::new(30),
            op: BinaryOp::Add,
            lhs: ValueId::new(10),
            rhs: ValueId::new(11),
            declared_type_name: "usize".to_string(),
        });
    function
        .metadata
        .exact_numeric_compare_route_facts
        .push(ExactNumericCompareRouteFact {
            block: BasicBlockId::new(7),
            instruction_index: 4,
            dst: ValueId::new(31),
            op: CompareOp::Lt,
            lhs: ValueId::new(30),
            rhs: ValueId::new(12),
            declared_type_name: "usize".to_string(),
        });
    function
        .metadata
        .exact_numeric_shift_route_facts
        .push(ExactNumericShiftRouteFact {
            block: BasicBlockId::new(7),
            instruction_index: 5,
            dst: ValueId::new(32),
            op: BinaryOp::Shr,
            lhs: ValueId::new(30),
            rhs: ValueId::new(13),
            declared_type_name: "usize".to_string(),
        });

    let mut module = MirModule::new("json_exact_numeric_routes_test".to_string());
    module.add_function(function);

    let root = build_mir_json_root(&module).expect("mir json root");
    let metadata = &root["functions"][0]["metadata"];
    let binary_route = &metadata["exact_numeric_binary_op_routes"][0];
    assert_eq!(binary_route["block"], 7);
    assert_eq!(binary_route["instruction_index"], 3);
    assert_eq!(binary_route["dst"], 30);
    assert_eq!(binary_route["operation"], "+");
    assert_eq!(binary_route["lhs"], 10);
    assert_eq!(binary_route["rhs"], 11);
    assert_eq!(binary_route["declared_type"], "usize");

    let compare_route = &metadata["exact_numeric_compare_routes"][0];
    assert_eq!(compare_route["operation"], "<");
    assert_eq!(compare_route["dst"], 31);
    assert_eq!(compare_route["declared_type"], "usize");

    let shift_route = &metadata["exact_numeric_shift_routes"][0];
    assert_eq!(shift_route["operation"], ">>");
    assert_eq!(shift_route["dst"], 32);
    assert_eq!(shift_route["declared_type"], "usize");
}

#[test]
fn mir_json_exact_numeric_runtime_check_marks_field_set_instruction() {
    let mut function = make_function("main", true);
    function
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .unwrap()
        .instructions
        .push(MirInstruction::FieldSet {
            base: ValueId::new(10),
            field: "capacity".to_string(),
            value: ValueId::new(20),
            declared_type: None,
        });
    function
        .metadata
        .exact_numeric_runtime_check_contracts
        .push(ExactNumericRuntimeCheckContract {
            block: BasicBlockId::new(0),
            instruction_index: 0,
            field: "capacity".to_string(),
            value: ValueId::new(20),
            declared_type_name: "usize".to_string(),
            kind: ExactNumericRuntimeCheckContractKind::DynamicIntegerRange,
        });

    let mut module = MirModule::new("json_exact_numeric_runtime_check_test".to_string());
    module.add_function(function);

    let root = build_mir_json_root(&module).expect("mir json root");
    let inst = &root["functions"][0]["blocks"][0]["instructions"][0];
    assert_eq!(inst["op"], "field_set");
    assert_eq!(
        inst["exact_numeric_runtime_check"]["kind"],
        "dynamic_integer_range"
    );
    assert_eq!(
        inst["exact_numeric_runtime_check"]["declared_type"],
        "usize"
    );
}
