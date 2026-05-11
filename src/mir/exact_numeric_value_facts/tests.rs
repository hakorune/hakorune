use super::*;
use crate::mir::function::MirParamDecl;
use crate::mir::{
    BinaryOp, CompareOp, ConstValue, EffectMask, FunctionSignature, MirModule, UserBoxFieldDecl,
};

fn module_with_fields(function: MirFunction) -> MirModule {
    let mut module = MirModule::new("exact_numeric_value_facts_test".to_string());
    module.metadata.user_box_field_decls.insert(
        "Page".to_string(),
        vec![
            UserBoxFieldDecl {
                name: "capacity".to_string(),
                declared_type_name: Some("usize".to_string()),
                is_weak: false,
            },
            UserBoxFieldDecl {
                name: "count".to_string(),
                declared_type_name: Some("u64".to_string()),
                is_weak: false,
            },
            UserBoxFieldDecl {
                name: "delta".to_string(),
                declared_type_name: Some("i64".to_string()),
                is_weak: false,
            },
        ],
    );
    module.add_function(function);
    module
}

fn page_function() -> MirFunction {
    MirFunction::new(
        FunctionSignature {
            name: "main".to_string(),
            params: vec![MirType::Box("Page".to_string())],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    )
}

fn numeric_param_function() -> MirFunction {
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "takes_size".to_string(),
            params: vec![MirType::Integer],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    function.metadata.declared_param_decls = vec![MirParamDecl {
        name: "size".to_string(),
        declared_type_name: Some("usize".to_string()),
    }];
    function.metadata.declared_return_type_name = Some("u64".to_string());
    function
}

#[test]
fn publishes_exact_numeric_param_fact_from_declared_signature_metadata() {
    let function = numeric_param_function();
    let param = function.params[0];
    let mut module = module_with_fields(function);

    refresh_module_exact_numeric_value_facts(&mut module);

    let fact = module
        .get_function("takes_size")
        .unwrap()
        .metadata
        .exact_numeric_value_facts
        .get(&param)
        .unwrap();
    assert_eq!(fact.declared_type_name, "usize");
    assert_eq!(
        fact.source,
        ExactNumericValueFactSource::Param {
            index: 0,
            name: "size".to_string(),
        }
    );
}

#[test]
fn publishes_exact_numeric_return_fact_from_declared_signature_metadata() {
    let function = numeric_param_function();
    let mut module = module_with_fields(function);

    refresh_module_exact_numeric_value_facts(&mut module);

    assert_eq!(
        module
            .get_function("takes_size")
            .unwrap()
            .metadata
            .exact_numeric_return_fact,
        Some(ExactNumericReturnFact {
            declared_type_name: "u64".to_string(),
        })
    );
}

#[test]
fn publishes_field_get_exact_numeric_fact_from_box_param() {
    let mut function = page_function();
    let page = function.params[0];
    let capacity = function.next_value_id();
    function
        .get_block_mut(BasicBlockId::new(0))
        .unwrap()
        .add_instruction(MirInstruction::FieldGet {
            dst: capacity,
            base: page,
            field: "capacity".to_string(),
            declared_type: Some(MirType::Integer),
        });
    let mut module = module_with_fields(function);

    refresh_module_exact_numeric_value_facts(&mut module);

    let fact = module
        .get_function("main")
        .unwrap()
        .metadata
        .exact_numeric_value_facts
        .get(&capacity)
        .unwrap();
    assert_eq!(fact.declared_type_name, "usize");
    assert_eq!(
        fact.source,
        ExactNumericValueFactSource::FieldGet {
            box_name: "Page".to_string(),
            field: "capacity".to_string(),
        }
    );
}

#[test]
fn propagates_copy_exact_numeric_fact() {
    let mut function = page_function();
    let page = function.params[0];
    let capacity = function.next_value_id();
    let copied = function.next_value_id();
    let block = function.get_block_mut(BasicBlockId::new(0)).unwrap();
    block.add_instruction(MirInstruction::FieldGet {
        dst: capacity,
        base: page,
        field: "capacity".to_string(),
        declared_type: Some(MirType::Integer),
    });
    block.add_instruction(MirInstruction::Copy {
        dst: copied,
        src: capacity,
    });
    let mut module = module_with_fields(function);

    refresh_module_exact_numeric_value_facts(&mut module);

    let fact = module
        .get_function("main")
        .unwrap()
        .metadata
        .exact_numeric_value_facts
        .get(&copied)
        .unwrap();
    assert_eq!(fact.declared_type_name, "usize");
    assert_eq!(
        fact.source,
        ExactNumericValueFactSource::Copy { src: capacity }
    );
}

#[test]
fn propagates_select_when_inputs_share_exact_type() {
    let mut function = page_function();
    let page = function.params[0];
    let left = function.next_value_id();
    let right = function.next_value_id();
    let cond = function.next_value_id();
    let selected = function.next_value_id();
    let block = function.get_block_mut(BasicBlockId::new(0)).unwrap();
    block.add_instruction(MirInstruction::FieldGet {
        dst: left,
        base: page,
        field: "capacity".to_string(),
        declared_type: Some(MirType::Integer),
    });
    block.add_instruction(MirInstruction::FieldGet {
        dst: right,
        base: page,
        field: "capacity".to_string(),
        declared_type: Some(MirType::Integer),
    });
    block.add_instruction(MirInstruction::Const {
        dst: cond,
        value: ConstValue::Bool(true),
    });
    block.add_instruction(MirInstruction::Select {
        dst: selected,
        cond,
        then_val: left,
        else_val: right,
    });
    let mut module = module_with_fields(function);

    refresh_module_exact_numeric_value_facts(&mut module);

    let metadata = &module.get_function("main").unwrap().metadata;
    assert_eq!(
        metadata
            .exact_numeric_value_facts
            .get(&selected)
            .unwrap()
            .declared_type_name,
        "usize"
    );
    assert!(metadata.exact_numeric_value_fact_rejections.is_empty());
}

#[test]
fn publishes_binop_add_route_and_result_fact_for_same_exact_operands() {
    let mut function = page_function();
    let page = function.params[0];
    let left = function.next_value_id();
    let right = function.next_value_id();
    let sum = function.next_value_id();
    let block = function.get_block_mut(BasicBlockId::new(0)).unwrap();
    block.add_instruction(MirInstruction::FieldGet {
        dst: left,
        base: page,
        field: "capacity".to_string(),
        declared_type: Some(MirType::Integer),
    });
    block.add_instruction(MirInstruction::FieldGet {
        dst: right,
        base: page,
        field: "capacity".to_string(),
        declared_type: Some(MirType::Integer),
    });
    block.add_instruction(MirInstruction::BinOp {
        dst: sum,
        op: BinaryOp::Add,
        lhs: left,
        rhs: right,
    });
    let mut module = module_with_fields(function);

    refresh_module_exact_numeric_value_facts(&mut module);

    let metadata = &module.get_function("main").unwrap().metadata;
    assert_eq!(
        metadata.exact_numeric_value_facts.get(&sum).unwrap(),
        &ExactNumericValueFact {
            declared_type_name: "usize".to_string(),
            source: ExactNumericValueFactSource::BinaryOp {
                op: BinaryOp::Add,
                lhs: left,
                rhs: right,
            },
        }
    );
    assert_eq!(
        metadata.exact_numeric_binary_op_route_facts,
        vec![ExactNumericBinaryOpRouteFact {
            block: BasicBlockId::new(0),
            instruction_index: 2,
            dst: sum,
            op: BinaryOp::Add,
            lhs: left,
            rhs: right,
            declared_type_name: "usize".to_string(),
        }]
    );
    assert!(metadata.exact_numeric_binary_op_route_rejections.is_empty());
}

#[test]
fn publishes_sub_and_mul_routes_for_same_exact_operands() {
    let mut function = page_function();
    let page = function.params[0];
    let left = function.next_value_id();
    let right = function.next_value_id();
    let difference = function.next_value_id();
    let product = function.next_value_id();
    let block = function.get_block_mut(BasicBlockId::new(0)).unwrap();
    block.add_instruction(MirInstruction::FieldGet {
        dst: left,
        base: page,
        field: "capacity".to_string(),
        declared_type: Some(MirType::Integer),
    });
    block.add_instruction(MirInstruction::FieldGet {
        dst: right,
        base: page,
        field: "capacity".to_string(),
        declared_type: Some(MirType::Integer),
    });
    block.add_instruction(MirInstruction::BinOp {
        dst: difference,
        op: BinaryOp::Sub,
        lhs: left,
        rhs: right,
    });
    block.add_instruction(MirInstruction::BinOp {
        dst: product,
        op: BinaryOp::Mul,
        lhs: left,
        rhs: right,
    });
    let mut module = module_with_fields(function);

    refresh_module_exact_numeric_value_facts(&mut module);

    let metadata = &module.get_function("main").unwrap().metadata;
    assert_eq!(
        metadata.exact_numeric_value_facts.get(&difference).unwrap(),
        &ExactNumericValueFact {
            declared_type_name: "usize".to_string(),
            source: ExactNumericValueFactSource::BinaryOp {
                op: BinaryOp::Sub,
                lhs: left,
                rhs: right,
            },
        }
    );
    assert_eq!(
        metadata.exact_numeric_value_facts.get(&product).unwrap(),
        &ExactNumericValueFact {
            declared_type_name: "usize".to_string(),
            source: ExactNumericValueFactSource::BinaryOp {
                op: BinaryOp::Mul,
                lhs: left,
                rhs: right,
            },
        }
    );
    assert_eq!(
        metadata.exact_numeric_binary_op_route_facts,
        vec![
            ExactNumericBinaryOpRouteFact {
                block: BasicBlockId::new(0),
                instruction_index: 2,
                dst: difference,
                op: BinaryOp::Sub,
                lhs: left,
                rhs: right,
                declared_type_name: "usize".to_string(),
            },
            ExactNumericBinaryOpRouteFact {
                block: BasicBlockId::new(0),
                instruction_index: 3,
                dst: product,
                op: BinaryOp::Mul,
                lhs: left,
                rhs: right,
                declared_type_name: "usize".to_string(),
            },
        ]
    );
}

#[test]
fn publishes_compare_route_for_same_exact_operands() {
    let mut function = page_function();
    let page = function.params[0];
    let left = function.next_value_id();
    let right = function.next_value_id();
    let result = function.next_value_id();
    let block = function.get_block_mut(BasicBlockId::new(0)).unwrap();
    block.add_instruction(MirInstruction::FieldGet {
        dst: left,
        base: page,
        field: "capacity".to_string(),
        declared_type: Some(MirType::Integer),
    });
    block.add_instruction(MirInstruction::FieldGet {
        dst: right,
        base: page,
        field: "capacity".to_string(),
        declared_type: Some(MirType::Integer),
    });
    block.add_instruction(MirInstruction::Compare {
        dst: result,
        op: CompareOp::Lt,
        lhs: left,
        rhs: right,
    });
    let mut module = module_with_fields(function);

    refresh_module_exact_numeric_value_facts(&mut module);

    let metadata = &module.get_function("main").unwrap().metadata;
    assert!(!metadata.exact_numeric_value_facts.contains_key(&result));
    assert_eq!(
        metadata.exact_numeric_compare_route_facts,
        vec![ExactNumericCompareRouteFact {
            block: BasicBlockId::new(0),
            instruction_index: 2,
            dst: result,
            op: CompareOp::Lt,
            lhs: left,
            rhs: right,
            declared_type_name: "usize".to_string(),
        }]
    );
    assert!(metadata.exact_numeric_compare_route_rejections.is_empty());
}

#[test]
fn publishes_logical_shift_route_for_exact_unsigned_lhs() {
    let mut function = page_function();
    let page = function.params[0];
    let value = function.next_value_id();
    let shift = function.next_value_id();
    let shifted = function.next_value_id();
    let block = function.get_block_mut(BasicBlockId::new(0)).unwrap();
    block.add_instruction(MirInstruction::FieldGet {
        dst: value,
        base: page,
        field: "capacity".to_string(),
        declared_type: Some(MirType::Integer),
    });
    block.add_instruction(MirInstruction::Const {
        dst: shift,
        value: ConstValue::Integer(3),
    });
    block.add_instruction(MirInstruction::BinOp {
        dst: shifted,
        op: BinaryOp::Shr,
        lhs: value,
        rhs: shift,
    });
    let mut module = module_with_fields(function);

    refresh_module_exact_numeric_value_facts(&mut module);

    let metadata = &module.get_function("main").unwrap().metadata;
    assert_eq!(
        metadata.exact_numeric_value_facts.get(&shifted).unwrap(),
        &ExactNumericValueFact {
            declared_type_name: "usize".to_string(),
            source: ExactNumericValueFactSource::BinaryOp {
                op: BinaryOp::Shr,
                lhs: value,
                rhs: shift,
            },
        }
    );
    assert_eq!(
        metadata.exact_numeric_shift_route_facts,
        vec![ExactNumericShiftRouteFact {
            block: BasicBlockId::new(0),
            instruction_index: 2,
            dst: shifted,
            op: BinaryOp::Shr,
            lhs: value,
            rhs: shift,
            declared_type_name: "usize".to_string(),
        }]
    );
    assert!(metadata.exact_numeric_shift_route_rejections.is_empty());
}

#[test]
fn records_select_rejection_for_exact_dynamic_mix() {
    let mut function = page_function();
    let page = function.params[0];
    let exact = function.next_value_id();
    let dynamic = function.next_value_id();
    let cond = function.next_value_id();
    let selected = function.next_value_id();
    let block = function.get_block_mut(BasicBlockId::new(0)).unwrap();
    block.add_instruction(MirInstruction::FieldGet {
        dst: exact,
        base: page,
        field: "capacity".to_string(),
        declared_type: Some(MirType::Integer),
    });
    block.add_instruction(MirInstruction::Const {
        dst: dynamic,
        value: ConstValue::Integer(7),
    });
    block.add_instruction(MirInstruction::Const {
        dst: cond,
        value: ConstValue::Bool(true),
    });
    block.add_instruction(MirInstruction::Select {
        dst: selected,
        cond,
        then_val: exact,
        else_val: dynamic,
    });
    let mut module = module_with_fields(function);

    refresh_module_exact_numeric_value_facts(&mut module);

    let metadata = &module.get_function("main").unwrap().metadata;
    assert!(!metadata.exact_numeric_value_facts.contains_key(&selected));
    assert_eq!(
        metadata.exact_numeric_value_fact_rejections,
        vec![ExactNumericValueFactRejection {
            block: BasicBlockId::new(0),
            instruction_index: 3,
            dst: selected,
            site: ExactNumericValueFactMergeSite::Select,
            kind: ExactNumericValueFactRejectionKind::MixedExactAndDynamic {
                exact_source_name: "usize".to_string(),
            },
        }]
    );
}

#[test]
fn records_binop_add_rejection_for_exact_dynamic_mix() {
    let mut function = page_function();
    let page = function.params[0];
    let exact = function.next_value_id();
    let dynamic = function.next_value_id();
    let sum = function.next_value_id();
    let block = function.get_block_mut(BasicBlockId::new(0)).unwrap();
    block.add_instruction(MirInstruction::FieldGet {
        dst: exact,
        base: page,
        field: "capacity".to_string(),
        declared_type: Some(MirType::Integer),
    });
    block.add_instruction(MirInstruction::Const {
        dst: dynamic,
        value: ConstValue::Integer(7),
    });
    block.add_instruction(MirInstruction::BinOp {
        dst: sum,
        op: BinaryOp::Add,
        lhs: exact,
        rhs: dynamic,
    });
    let mut module = module_with_fields(function);

    refresh_module_exact_numeric_value_facts(&mut module);

    let metadata = &module.get_function("main").unwrap().metadata;
    assert!(!metadata.exact_numeric_value_facts.contains_key(&sum));
    assert!(metadata.exact_numeric_binary_op_route_facts.is_empty());
    assert_eq!(
        metadata.exact_numeric_binary_op_route_rejections,
        vec![ExactNumericBinaryOpRouteRejection {
            block: BasicBlockId::new(0),
            instruction_index: 2,
            dst: sum,
            op: BinaryOp::Add,
            lhs: exact,
            rhs: dynamic,
            kind: ExactNumericBinaryOpRouteRejectionKind::MixedExactAndDynamic {
                exact_source_name: "usize".to_string(),
            },
        }]
    );
}

#[test]
fn records_phi_rejection_for_exact_type_mismatch() {
    let mut function = page_function();
    let page = function.params[0];
    let left = function.next_value_id();
    let right = function.next_value_id();
    let merged = function.next_value_id();
    let block = function.get_block_mut(BasicBlockId::new(0)).unwrap();
    block.add_instruction(MirInstruction::FieldGet {
        dst: left,
        base: page,
        field: "capacity".to_string(),
        declared_type: Some(MirType::Integer),
    });
    block.add_instruction(MirInstruction::FieldGet {
        dst: right,
        base: page,
        field: "count".to_string(),
        declared_type: Some(MirType::Integer),
    });
    block.add_instruction(MirInstruction::Phi {
        dst: merged,
        inputs: vec![(BasicBlockId::new(0), left), (BasicBlockId::new(0), right)],
        type_hint: None,
    });
    let mut module = module_with_fields(function);

    refresh_module_exact_numeric_value_facts(&mut module);

    let metadata = &module.get_function("main").unwrap().metadata;
    assert!(!metadata.exact_numeric_value_facts.contains_key(&merged));
    assert_eq!(
        metadata.exact_numeric_value_fact_rejections,
        vec![ExactNumericValueFactRejection {
            block: BasicBlockId::new(0),
            instruction_index: 2,
            dst: merged,
            site: ExactNumericValueFactMergeSite::Phi,
            kind: ExactNumericValueFactRejectionKind::TypeMismatch {
                left_source_name: "usize".to_string(),
                right_source_name: "u64".to_string(),
            },
        }]
    );
}

#[test]
fn records_compare_rejection_for_exact_dynamic_mix() {
    let mut function = page_function();
    let page = function.params[0];
    let exact = function.next_value_id();
    let dynamic = function.next_value_id();
    let result = function.next_value_id();
    let block = function.get_block_mut(BasicBlockId::new(0)).unwrap();
    block.add_instruction(MirInstruction::FieldGet {
        dst: exact,
        base: page,
        field: "capacity".to_string(),
        declared_type: Some(MirType::Integer),
    });
    block.add_instruction(MirInstruction::Const {
        dst: dynamic,
        value: ConstValue::Integer(7),
    });
    block.add_instruction(MirInstruction::Compare {
        dst: result,
        op: CompareOp::Ge,
        lhs: exact,
        rhs: dynamic,
    });
    let mut module = module_with_fields(function);

    refresh_module_exact_numeric_value_facts(&mut module);

    let metadata = &module.get_function("main").unwrap().metadata;
    assert!(metadata.exact_numeric_compare_route_facts.is_empty());
    assert_eq!(
        metadata.exact_numeric_compare_route_rejections,
        vec![ExactNumericCompareRouteRejection {
            block: BasicBlockId::new(0),
            instruction_index: 2,
            dst: result,
            op: CompareOp::Ge,
            lhs: exact,
            rhs: dynamic,
            kind: ExactNumericCompareRouteRejectionKind::MixedExactAndDynamic {
                exact_source_name: "usize".to_string(),
            },
        }]
    );
}

#[test]
fn records_logical_shift_rejection_for_exact_signed_lhs() {
    let mut function = page_function();
    let page = function.params[0];
    let value = function.next_value_id();
    let shift = function.next_value_id();
    let shifted = function.next_value_id();
    let block = function.get_block_mut(BasicBlockId::new(0)).unwrap();
    block.add_instruction(MirInstruction::FieldGet {
        dst: value,
        base: page,
        field: "delta".to_string(),
        declared_type: Some(MirType::Integer),
    });
    block.add_instruction(MirInstruction::Const {
        dst: shift,
        value: ConstValue::Integer(3),
    });
    block.add_instruction(MirInstruction::BinOp {
        dst: shifted,
        op: BinaryOp::Shr,
        lhs: value,
        rhs: shift,
    });
    let mut module = module_with_fields(function);

    refresh_module_exact_numeric_value_facts(&mut module);

    let metadata = &module.get_function("main").unwrap().metadata;
    assert!(!metadata.exact_numeric_value_facts.contains_key(&shifted));
    assert!(metadata.exact_numeric_shift_route_facts.is_empty());
    assert_eq!(
        metadata.exact_numeric_shift_route_rejections,
        vec![ExactNumericShiftRouteRejection {
            block: BasicBlockId::new(0),
            instruction_index: 2,
            dst: shifted,
            op: BinaryOp::Shr,
            lhs: value,
            rhs: shift,
            kind: ExactNumericShiftRouteRejectionKind::SignedLogicalShift {
                source_name: "i64".to_string(),
            },
        }]
    );
}

#[test]
fn records_binop_add_rejection_for_exact_type_mismatch() {
    let mut function = page_function();
    let page = function.params[0];
    let left = function.next_value_id();
    let right = function.next_value_id();
    let sum = function.next_value_id();
    let block = function.get_block_mut(BasicBlockId::new(0)).unwrap();
    block.add_instruction(MirInstruction::FieldGet {
        dst: left,
        base: page,
        field: "capacity".to_string(),
        declared_type: Some(MirType::Integer),
    });
    block.add_instruction(MirInstruction::FieldGet {
        dst: right,
        base: page,
        field: "count".to_string(),
        declared_type: Some(MirType::Integer),
    });
    block.add_instruction(MirInstruction::BinOp {
        dst: sum,
        op: BinaryOp::Add,
        lhs: left,
        rhs: right,
    });
    let mut module = module_with_fields(function);

    refresh_module_exact_numeric_value_facts(&mut module);

    let metadata = &module.get_function("main").unwrap().metadata;
    assert!(!metadata.exact_numeric_value_facts.contains_key(&sum));
    assert!(metadata.exact_numeric_binary_op_route_facts.is_empty());
    assert_eq!(
        metadata.exact_numeric_binary_op_route_rejections,
        vec![ExactNumericBinaryOpRouteRejection {
            block: BasicBlockId::new(0),
            instruction_index: 2,
            dst: sum,
            op: BinaryOp::Add,
            lhs: left,
            rhs: right,
            kind: ExactNumericBinaryOpRouteRejectionKind::TypeMismatch {
                left_source_name: "usize".to_string(),
                right_source_name: "u64".to_string(),
            },
        }]
    );
}
