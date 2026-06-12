use super::*;

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
