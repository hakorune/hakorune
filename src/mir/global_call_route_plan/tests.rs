use super::*;
use crate::mir::definitions::call_unified::{CalleeBoxKind, TypeCertainty};
use crate::mir::{BasicBlock, CompareOp, EffectMask, FunctionSignature, MirType};

fn make_function_with_global_call_args(
    name: &str,
    dst: Option<ValueId>,
    args: Vec<ValueId>,
) -> MirFunction {
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "main".to_string(),
            params: vec![],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    let block = function
        .blocks
        .entry(BasicBlockId::new(0))
        .or_insert_with(|| BasicBlock::new(BasicBlockId::new(0)));
    block.instructions.push(MirInstruction::Call {
        dst,
        func: ValueId::INVALID,
        callee: Some(Callee::Global(name.to_string())),
        args,
        effects: EffectMask::PURE,
    });
    function
}

fn make_function_with_global_call(name: &str, dst: Option<ValueId>) -> MirFunction {
    make_function_with_global_call_args(name, dst, vec![ValueId::new(1), ValueId::new(2)])
}

#[test]
fn refresh_function_global_call_routes_records_unsupported_global_call() {
    let mut function = make_function_with_global_call(
        "Stage1ModeContractBox.resolve_mode/0",
        Some(ValueId::new(7)),
    );
    refresh_function_global_call_routes(&mut function);

    assert_eq!(function.metadata.global_call_routes.len(), 1);
    let route = &function.metadata.global_call_routes[0];
    assert_eq!(route.block(), BasicBlockId::new(0));
    assert_eq!(route.instruction_index(), 0);
    assert_eq!(route.callee_name(), "Stage1ModeContractBox.resolve_mode/0");
    assert_eq!(route.arity(), 2);
    assert_eq!(route.result_value(), Some(ValueId::new(7)));
    assert_eq!(route.tier(), "Unsupported");
    assert!(!route.target_exists());
    assert_eq!(route.target_arity(), None);
    assert_eq!(route.target_return_type(), None);
    assert_eq!(route.target_shape(), None);
    assert_eq!(route.reason(), Some("unknown_global_callee"));
}

#[test]
fn refresh_function_global_call_routes_skips_print_surface() {
    let mut function = make_function_with_global_call("print", None);
    refresh_function_global_call_routes(&mut function);
    assert!(function.metadata.global_call_routes.is_empty());
}

#[test]
fn refresh_module_global_call_routes_records_target_facts() {
    let mut module = MirModule::new("global_call_target_test".to_string());
    let caller = make_function_with_global_call(
        "Stage1ModeContractBox.resolve_mode/0",
        Some(ValueId::new(7)),
    );
    let callee = MirFunction::new(
        FunctionSignature {
            name: "Stage1ModeContractBox.resolve_mode/0".to_string(),
            params: vec![MirType::Integer, MirType::Integer],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("Stage1ModeContractBox.resolve_mode/0".to_string(), callee);

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert!(route.target_exists());
    assert_eq!(
        route.target_symbol(),
        Some("Stage1ModeContractBox.resolve_mode/0")
    );
    assert_eq!(route.target_arity(), Some(2));
    assert_eq!(route.target_return_type(), Some("i64".to_string()));
    assert_eq!(route.arity_matches(), Some(true));
    assert_eq!(route.target_shape(), None);
    assert_eq!(
        route.target_shape_reason(),
        Some("generic_string_no_string_surface")
    );
    assert_eq!(route.reason(), Some("missing_multi_function_emitter"));
}

#[test]
fn refresh_module_global_call_routes_marks_string_or_void_sentinel_body_direct_target() {
    let mut module = MirModule::new("global_call_void_sentinel_test".to_string());
    let caller =
        make_function_with_global_call_args("Helper.maybe_text/0", Some(ValueId::new(7)), vec![]);
    let mut callee = MirFunction::new(
        FunctionSignature {
            name: "Helper.maybe_text/0".to_string(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    let entry = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    entry.instructions.push(MirInstruction::Const {
        dst: ValueId::new(1),
        value: ConstValue::Bool(true),
    });
    entry.set_terminator(MirInstruction::Branch {
        condition: ValueId::new(1),
        then_bb: BasicBlockId::new(1),
        else_bb: BasicBlockId::new(2),
        then_edge_args: None,
        else_edge_args: None,
    });
    let mut text_block = BasicBlock::new(BasicBlockId::new(1));
    text_block.instructions.push(MirInstruction::Const {
        dst: ValueId::new(2),
        value: ConstValue::String("ok".to_string()),
    });
    text_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(2)),
    });
    let mut void_block = BasicBlock::new(BasicBlockId::new(2));
    void_block.instructions.push(MirInstruction::Const {
        dst: ValueId::new(3),
        value: ConstValue::Void,
    });
    void_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(3)),
    });
    callee.blocks.insert(BasicBlockId::new(1), text_block);
    callee.blocks.insert(BasicBlockId::new(2), void_block);
    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("Helper.maybe_text/0".to_string(), callee);

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert!(route.target_exists());
    assert_eq!(route.target_symbol(), Some("Helper.maybe_text/0"));
    assert_eq!(route.target_return_type(), Some("void".to_string()));
    assert_eq!(
        route.target_shape(),
        Some("generic_string_or_void_sentinel_body"),
        "reason={:?}",
        route.target_shape_reason()
    );
    assert_eq!(route.target_shape_reason(), None);
    assert_eq!(
        route.proof(),
        "typed_global_call_generic_string_or_void_sentinel"
    );
    assert_eq!(route.tier(), "DirectAbi");
    assert_eq!(route.return_shape(), Some("string_handle_or_null"));
    assert_eq!(route.value_demand(), "runtime_i64_or_handle");
    assert_eq!(route.reason(), None);
}

#[test]
fn refresh_module_global_call_routes_accepts_substring_void_sentinel_body() {
    let mut module = MirModule::new("global_call_substring_void_sentinel_test".to_string());
    let caller = make_function_with_global_call_args(
        "Helper.slice_or_null/1",
        Some(ValueId::new(7)),
        vec![ValueId::new(1)],
    );
    let mut callee = MirFunction::new(
        FunctionSignature {
            name: "Helper.slice_or_null/1".to_string(),
            params: vec![MirType::String],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    callee.params = vec![ValueId::new(1)];
    let entry = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    entry.instructions.extend([
        MirInstruction::Const {
            dst: ValueId::new(2),
            value: ConstValue::Bool(true),
        },
        MirInstruction::Const {
            dst: ValueId::new(3),
            value: ConstValue::Integer(0),
        },
        MirInstruction::Const {
            dst: ValueId::new(4),
            value: ConstValue::Integer(4),
        },
    ]);
    entry.set_terminator(MirInstruction::Branch {
        condition: ValueId::new(2),
        then_bb: BasicBlockId::new(1),
        else_bb: BasicBlockId::new(2),
        then_edge_args: None,
        else_edge_args: None,
    });
    let mut text_block = BasicBlock::new(BasicBlockId::new(1));
    text_block.instructions.push(MirInstruction::Call {
        dst: Some(ValueId::new(5)),
        func: ValueId::INVALID,
        callee: Some(Callee::Method {
            box_name: "RuntimeDataBox".to_string(),
            method: "substring".to_string(),
            receiver: Some(ValueId::new(1)),
            certainty: TypeCertainty::Union,
            box_kind: CalleeBoxKind::RuntimeData,
        }),
        args: vec![ValueId::new(3), ValueId::new(4)],
        effects: EffectMask::PURE,
    });
    text_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(5)),
    });
    let mut void_block = BasicBlock::new(BasicBlockId::new(2));
    void_block.instructions.push(MirInstruction::Const {
        dst: ValueId::new(6),
        value: ConstValue::Void,
    });
    void_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(6)),
    });
    callee.blocks.insert(BasicBlockId::new(1), text_block);
    callee.blocks.insert(BasicBlockId::new(2), void_block);
    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("Helper.slice_or_null/1".to_string(), callee);

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(
        route.target_shape(),
        Some("generic_string_or_void_sentinel_body"),
        "reason={:?}",
        route.target_shape_reason()
    );
    assert_eq!(route.target_shape_reason(), None);
    assert_eq!(
        route.proof(),
        "typed_global_call_generic_string_or_void_sentinel"
    );
    assert_eq!(route.return_shape(), Some("string_handle_or_null"));
}

#[test]
fn refresh_module_global_call_routes_accepts_mixed_param_substring_void_sentinel_body() {
    let mut module = MirModule::new("global_call_mixed_substring_void_sentinel_test".to_string());
    let caller = make_function_with_global_call_args(
        "Helper.slice_or_null/2",
        Some(ValueId::new(7)),
        vec![ValueId::new(1), ValueId::new(2)],
    );
    let mut callee = MirFunction::new(
        FunctionSignature {
            name: "Helper.slice_or_null/2".to_string(),
            params: vec![MirType::Unknown, MirType::Unknown],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    callee.params = vec![ValueId::new(1), ValueId::new(2)];
    let entry = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    entry.instructions.extend([
        MirInstruction::Const {
            dst: ValueId::new(3),
            value: ConstValue::String(String::new()),
        },
        MirInstruction::BinOp {
            dst: ValueId::new(4),
            op: BinaryOp::Add,
            lhs: ValueId::new(3),
            rhs: ValueId::new(1),
        },
        MirInstruction::Const {
            dst: ValueId::new(5),
            value: ConstValue::Integer(0),
        },
        MirInstruction::Copy {
            dst: ValueId::new(12),
            src: ValueId::new(2),
        },
        MirInstruction::Compare {
            dst: ValueId::new(6),
            op: CompareOp::Lt,
            lhs: ValueId::new(12),
            rhs: ValueId::new(5),
        },
    ]);
    entry.set_terminator(MirInstruction::Branch {
        condition: ValueId::new(6),
        then_bb: BasicBlockId::new(1),
        else_bb: BasicBlockId::new(2),
        then_edge_args: None,
        else_edge_args: None,
    });
    let mut text_block = BasicBlock::new(BasicBlockId::new(1));
    text_block.instructions.extend([
        MirInstruction::Const {
            dst: ValueId::new(7),
            value: ConstValue::Integer(1),
        },
        MirInstruction::Copy {
            dst: ValueId::new(13),
            src: ValueId::new(2),
        },
        MirInstruction::BinOp {
            dst: ValueId::new(8),
            op: BinaryOp::Add,
            lhs: ValueId::new(13),
            rhs: ValueId::new(7),
        },
        MirInstruction::Const {
            dst: ValueId::new(9),
            value: ConstValue::Integer(4),
        },
        MirInstruction::Call {
            dst: Some(ValueId::new(10)),
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: "RuntimeDataBox".to_string(),
                method: "substring".to_string(),
                receiver: Some(ValueId::new(4)),
                certainty: TypeCertainty::Union,
                box_kind: CalleeBoxKind::RuntimeData,
            }),
            args: vec![ValueId::new(13), ValueId::new(8)],
            effects: EffectMask::PURE,
        },
    ]);
    text_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(10)),
    });
    let mut void_block = BasicBlock::new(BasicBlockId::new(2));
    void_block.instructions.push(MirInstruction::Const {
        dst: ValueId::new(11),
        value: ConstValue::Void,
    });
    void_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(11)),
    });
    callee.blocks.insert(BasicBlockId::new(1), text_block);
    callee.blocks.insert(BasicBlockId::new(2), void_block);
    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("Helper.slice_or_null/2".to_string(), callee);

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(
        route.target_shape(),
        Some("generic_string_or_void_sentinel_body"),
        "reason={:?}",
        route.target_shape_reason()
    );
    assert_eq!(route.target_shape_reason(), None);
    assert_eq!(
        route.proof(),
        "typed_global_call_generic_string_or_void_sentinel"
    );
    assert_eq!(route.return_shape(), Some("string_handle_or_null"));
}

#[test]
fn refresh_module_global_call_routes_marks_void_sentinel_child_blocker() {
    let mut module = MirModule::new("global_call_void_sentinel_child_test".to_string());
    let caller =
        make_function_with_global_call_args("Helper.maybe_text/0", Some(ValueId::new(7)), vec![]);
    let mut callee = MirFunction::new(
        FunctionSignature {
            name: "Helper.maybe_text/0".to_string(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    let entry = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    entry.instructions.push(MirInstruction::Call {
        dst: Some(ValueId::new(1)),
        func: ValueId::INVALID,
        callee: Some(Callee::Global("Helper.flag/0".to_string())),
        args: vec![],
        effects: EffectMask::PURE,
    });
    entry.set_terminator(MirInstruction::Branch {
        condition: ValueId::new(1),
        then_bb: BasicBlockId::new(1),
        else_bb: BasicBlockId::new(2),
        then_edge_args: None,
        else_edge_args: None,
    });
    let mut text_block = BasicBlock::new(BasicBlockId::new(1));
    text_block.instructions.push(MirInstruction::Const {
        dst: ValueId::new(2),
        value: ConstValue::String("ok".to_string()),
    });
    text_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(2)),
    });
    let mut void_block = BasicBlock::new(BasicBlockId::new(2));
    void_block.instructions.push(MirInstruction::Const {
        dst: ValueId::new(3),
        value: ConstValue::Void,
    });
    void_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(3)),
    });
    let flag = MirFunction::new(
        FunctionSignature {
            name: "Helper.flag/0".to_string(),
            params: vec![],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    callee.blocks.insert(BasicBlockId::new(1), text_block);
    callee.blocks.insert(BasicBlockId::new(2), void_block);
    module.functions.insert("main".to_string(), caller);
    module.functions.insert("Helper.flag/0".to_string(), flag);
    module
        .functions
        .insert("Helper.maybe_text/0".to_string(), callee);

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert!(route.target_exists());
    assert_eq!(route.target_symbol(), Some("Helper.maybe_text/0"));
    assert_eq!(route.target_return_type(), Some("void".to_string()));
    assert_eq!(route.target_shape(), None);
    assert_eq!(
        route.target_shape_reason(),
        Some("generic_string_global_target_shape_unknown")
    );
    assert_eq!(route.target_shape_blocker_symbol(), Some("Helper.flag/0"));
    assert_eq!(
        route.target_shape_blocker_reason(),
        Some("generic_string_no_string_surface")
    );
    assert_eq!(route.tier(), "Unsupported");
    assert_eq!(route.reason(), Some("missing_multi_function_emitter"));
}

#[test]
fn refresh_module_global_call_routes_marks_void_sentinel_return_child_blocker() {
    let mut module = MirModule::new("global_call_void_sentinel_return_child_test".to_string());
    let caller =
        make_function_with_global_call_args("Helper.maybe_text/0", Some(ValueId::new(7)), vec![]);
    let mut callee = MirFunction::new(
        FunctionSignature {
            name: "Helper.maybe_text/0".to_string(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    let entry = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    entry.instructions.push(MirInstruction::Const {
        dst: ValueId::new(1),
        value: ConstValue::Bool(true),
    });
    entry.set_terminator(MirInstruction::Branch {
        condition: ValueId::new(1),
        then_bb: BasicBlockId::new(1),
        else_bb: BasicBlockId::new(2),
        then_edge_args: None,
        else_edge_args: None,
    });
    let mut text_block = BasicBlock::new(BasicBlockId::new(1));
    text_block.instructions.push(MirInstruction::Call {
        dst: Some(ValueId::new(2)),
        func: ValueId::INVALID,
        callee: Some(Callee::Global("Helper.pending/0".to_string())),
        args: vec![],
        effects: EffectMask::PURE,
    });
    text_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(2)),
    });
    let mut void_block = BasicBlock::new(BasicBlockId::new(2));
    void_block.instructions.push(MirInstruction::Const {
        dst: ValueId::new(3),
        value: ConstValue::Void,
    });
    void_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(3)),
    });
    let pending = MirFunction::new(
        FunctionSignature {
            name: "Helper.pending/0".to_string(),
            params: vec![],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    callee.blocks.insert(BasicBlockId::new(1), text_block);
    callee.blocks.insert(BasicBlockId::new(2), void_block);
    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("Helper.pending/0".to_string(), pending);
    module
        .functions
        .insert("Helper.maybe_text/0".to_string(), callee);

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert!(route.target_exists());
    assert_eq!(route.target_symbol(), Some("Helper.maybe_text/0"));
    assert_eq!(route.target_return_type(), Some("void".to_string()));
    assert_eq!(route.target_shape(), None);
    assert_eq!(
        route.target_shape_reason(),
        Some("generic_string_global_target_shape_unknown")
    );
    assert_eq!(
        route.target_shape_blocker_symbol(),
        Some("Helper.pending/0")
    );
    assert_eq!(
        route.target_shape_blocker_reason(),
        Some("generic_string_no_string_surface")
    );
}

#[test]
fn string_return_blocker_ignores_direct_string_child_targets() {
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "Helper.maybe_text/0".to_string(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    let entry = function.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    entry.instructions.push(MirInstruction::Const {
        dst: ValueId::new(1),
        value: ConstValue::Bool(true),
    });
    entry.set_terminator(MirInstruction::Branch {
        condition: ValueId::new(1),
        then_bb: BasicBlockId::new(1),
        else_bb: BasicBlockId::new(2),
        then_edge_args: None,
        else_edge_args: None,
    });
    let mut text_block = BasicBlock::new(BasicBlockId::new(1));
    text_block.instructions.push(MirInstruction::Call {
        dst: Some(ValueId::new(2)),
        func: ValueId::INVALID,
        callee: Some(Callee::Global("Helper.text/0".to_string())),
        args: vec![],
        effects: EffectMask::PURE,
    });
    text_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(2)),
    });
    let mut void_block = BasicBlock::new(BasicBlockId::new(2));
    void_block.instructions.push(MirInstruction::Const {
        dst: ValueId::new(3),
        value: ConstValue::Void,
    });
    void_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(3)),
    });
    function.blocks.insert(BasicBlockId::new(1), text_block);
    function.blocks.insert(BasicBlockId::new(2), void_block);
    let mut targets = BTreeMap::new();
    targets.insert(
        "Helper.text/0".to_string(),
        GlobalCallTargetFacts::present_with_shape(0, GlobalCallTargetShape::GenericPureStringBody),
    );

    assert_eq!(
        generic_string_void_sentinel_return_global_blocker(&function, &targets),
        None
    );
}

#[test]
fn refresh_module_global_call_routes_accepts_void_typed_direct_sentinel_child_return() {
    let mut module = MirModule::new("global_call_void_typed_sentinel_child_test".to_string());
    let caller =
        make_function_with_global_call_args("Helper.parent/0", Some(ValueId::new(7)), vec![]);
    let mut child = MirFunction::new(
        FunctionSignature {
            name: "Helper.child/0".to_string(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    child
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .unwrap()
        .instructions
        .push(MirInstruction::Const {
            dst: ValueId::new(1),
            value: ConstValue::Bool(true),
        });
    child
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .unwrap()
        .set_terminator(MirInstruction::Branch {
            condition: ValueId::new(1),
            then_bb: BasicBlockId::new(1),
            else_bb: BasicBlockId::new(2),
            then_edge_args: None,
            else_edge_args: None,
        });
    let mut child_text_block = BasicBlock::new(BasicBlockId::new(1));
    child_text_block.instructions.push(MirInstruction::Const {
        dst: ValueId::new(2),
        value: ConstValue::String("ok".to_string()),
    });
    child_text_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(2)),
    });
    let mut child_void_block = BasicBlock::new(BasicBlockId::new(2));
    child_void_block.instructions.push(MirInstruction::Const {
        dst: ValueId::new(3),
        value: ConstValue::Void,
    });
    child_void_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(3)),
    });
    child.blocks.insert(BasicBlockId::new(1), child_text_block);
    child.blocks.insert(BasicBlockId::new(2), child_void_block);

    let mut parent = MirFunction::new(
        FunctionSignature {
            name: "Helper.parent/0".to_string(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    parent
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .unwrap()
        .instructions
        .push(MirInstruction::Const {
            dst: ValueId::new(1),
            value: ConstValue::Bool(true),
        });
    parent
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .unwrap()
        .set_terminator(MirInstruction::Branch {
            condition: ValueId::new(1),
            then_bb: BasicBlockId::new(1),
            else_bb: BasicBlockId::new(2),
            then_edge_args: None,
            else_edge_args: None,
        });
    let mut parent_text_block = BasicBlock::new(BasicBlockId::new(1));
    parent_text_block.instructions.push(MirInstruction::Call {
        dst: Some(ValueId::new(2)),
        func: ValueId::INVALID,
        callee: Some(Callee::Global("Helper.child/0".to_string())),
        args: vec![],
        effects: EffectMask::PURE,
    });
    parent_text_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(2)),
    });
    let mut parent_void_block = BasicBlock::new(BasicBlockId::new(2));
    parent_void_block.instructions.push(MirInstruction::Const {
        dst: ValueId::new(3),
        value: ConstValue::Void,
    });
    parent_void_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(3)),
    });
    parent
        .blocks
        .insert(BasicBlockId::new(1), parent_text_block);
    parent
        .blocks
        .insert(BasicBlockId::new(2), parent_void_block);
    parent
        .metadata
        .value_types
        .insert(ValueId::new(2), MirType::Void);
    module.functions.insert("main".to_string(), caller);
    module.functions.insert("Helper.child/0".to_string(), child);
    module
        .functions
        .insert("Helper.parent/0".to_string(), parent);

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(
        route.target_shape(),
        Some("generic_string_or_void_sentinel_body"),
        "reason={:?} blocker={:?}/{:?}",
        route.target_shape_reason(),
        route.target_shape_blocker_symbol(),
        route.target_shape_blocker_reason()
    );
    assert_eq!(route.target_shape_reason(), None);
    assert_eq!(route.return_shape(), Some("string_handle_or_null"));
}

#[test]
fn refresh_module_global_call_routes_uses_direct_child_route_over_void_metadata() {
    let mut module =
        MirModule::new("global_call_direct_child_route_over_void_metadata_test".to_string());
    let caller = make_function_with_global_call_args(
        "Helper.resolve/1",
        Some(ValueId::new(30)),
        vec![ValueId::new(1)],
    );

    let mut child = MirFunction::new(
        FunctionSignature {
            name: "Helper.child/0".to_string(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    let child_entry = child.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    child_entry.instructions.push(MirInstruction::Const {
        dst: ValueId::new(1),
        value: ConstValue::Bool(true),
    });
    child_entry.set_terminator(MirInstruction::Branch {
        condition: ValueId::new(1),
        then_bb: BasicBlockId::new(1),
        else_bb: BasicBlockId::new(2),
        then_edge_args: None,
        else_edge_args: None,
    });
    let mut child_text_block = BasicBlock::new(BasicBlockId::new(1));
    child_text_block.instructions.push(MirInstruction::Const {
        dst: ValueId::new(2),
        value: ConstValue::String("body".to_string()),
    });
    child_text_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(2)),
    });
    let mut child_void_block = BasicBlock::new(BasicBlockId::new(2));
    child_void_block.instructions.push(MirInstruction::Const {
        dst: ValueId::new(3),
        value: ConstValue::Void,
    });
    child_void_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(3)),
    });
    child.blocks.insert(BasicBlockId::new(1), child_text_block);
    child.blocks.insert(BasicBlockId::new(2), child_void_block);

    let mut parent = MirFunction::new(
        FunctionSignature {
            name: "Helper.resolve/1".to_string(),
            params: vec![MirType::Unknown],
            return_type: MirType::Unknown,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    parent.params = vec![ValueId::new(10)];
    let parent_entry = parent.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    parent_entry.instructions.extend([
        MirInstruction::Call {
            dst: Some(ValueId::new(11)),
            func: ValueId::INVALID,
            callee: Some(Callee::Global("Helper.child/0".to_string())),
            args: vec![],
            effects: EffectMask::PURE,
        },
        MirInstruction::Copy {
            dst: ValueId::new(12),
            src: ValueId::new(11),
        },
        MirInstruction::Const {
            dst: ValueId::new(13),
            value: ConstValue::Void,
        },
        MirInstruction::Compare {
            dst: ValueId::new(14),
            op: CompareOp::Ne,
            lhs: ValueId::new(12),
            rhs: ValueId::new(13),
        },
    ]);
    parent_entry.set_terminator(MirInstruction::Branch {
        condition: ValueId::new(14),
        then_bb: BasicBlockId::new(1),
        else_bb: BasicBlockId::new(2),
        then_edge_args: None,
        else_edge_args: None,
    });
    let mut parent_child_block = BasicBlock::new(BasicBlockId::new(1));
    parent_child_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(12)),
    });
    let mut parent_fallback_block = BasicBlock::new(BasicBlockId::new(2));
    parent_fallback_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(10)),
    });
    parent
        .blocks
        .insert(BasicBlockId::new(1), parent_child_block);
    parent
        .blocks
        .insert(BasicBlockId::new(2), parent_fallback_block);
    parent
        .metadata
        .value_types
        .insert(ValueId::new(11), MirType::Void);
    parent
        .metadata
        .value_types
        .insert(ValueId::new(12), MirType::Void);

    module.functions.insert("main".to_string(), caller);
    module.functions.insert("Helper.child/0".to_string(), child);
    module
        .functions
        .insert("Helper.resolve/1".to_string(), parent);

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(route.target_shape(), None);
    assert_eq!(
        route.target_shape_reason(),
        Some("generic_string_return_not_string"),
        "reason={:?} blocker={:?}/{:?}",
        route.target_shape_reason(),
        route.target_shape_blocker_symbol(),
        route.target_shape_blocker_reason()
    );
}

#[test]
fn refresh_module_global_call_routes_propagates_return_child_blocker_transitively() {
    let mut module = MirModule::new("global_call_void_sentinel_transitive_child_test".to_string());
    let caller =
        make_function_with_global_call_args("Helper.maybe_text/0", Some(ValueId::new(7)), vec![]);
    let mut callee = MirFunction::new(
        FunctionSignature {
            name: "Helper.maybe_text/0".to_string(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    let entry = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    entry.instructions.push(MirInstruction::Const {
        dst: ValueId::new(1),
        value: ConstValue::Bool(true),
    });
    entry.set_terminator(MirInstruction::Branch {
        condition: ValueId::new(1),
        then_bb: BasicBlockId::new(1),
        else_bb: BasicBlockId::new(2),
        then_edge_args: None,
        else_edge_args: None,
    });
    let mut text_block = BasicBlock::new(BasicBlockId::new(1));
    text_block.instructions.push(MirInstruction::Call {
        dst: Some(ValueId::new(2)),
        func: ValueId::INVALID,
        callee: Some(Callee::Global("Helper.wrapper/0".to_string())),
        args: vec![],
        effects: EffectMask::PURE,
    });
    text_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(2)),
    });
    let mut void_block = BasicBlock::new(BasicBlockId::new(2));
    void_block.instructions.push(MirInstruction::Const {
        dst: ValueId::new(3),
        value: ConstValue::Void,
    });
    void_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(3)),
    });
    let mut wrapper = MirFunction::new(
        FunctionSignature {
            name: "Helper.wrapper/0".to_string(),
            params: vec![],
            return_type: MirType::Unknown,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    let wrapper_block = wrapper.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    wrapper_block.instructions.push(MirInstruction::Call {
        dst: Some(ValueId::new(1)),
        func: ValueId::INVALID,
        callee: Some(Callee::Global("Helper.map/0".to_string())),
        args: vec![],
        effects: EffectMask::PURE,
    });
    wrapper_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(1)),
    });
    let map = MirFunction::new(
        FunctionSignature {
            name: "Helper.map/0".to_string(),
            params: vec![],
            return_type: MirType::Box("MapBox".to_string()),
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    callee.blocks.insert(BasicBlockId::new(1), text_block);
    callee.blocks.insert(BasicBlockId::new(2), void_block);
    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("Helper.wrapper/0".to_string(), wrapper);
    module.functions.insert("Helper.map/0".to_string(), map);
    module
        .functions
        .insert("Helper.maybe_text/0".to_string(), callee);

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(
        route.target_shape_reason(),
        Some("generic_string_global_target_shape_unknown")
    );
    assert_eq!(route.target_shape_blocker_symbol(), Some("Helper.map/0"));
    assert_eq!(
        route.target_shape_blocker_reason(),
        Some("generic_string_return_object_abi_not_handle_compatible")
    );
}

#[test]
fn refresh_module_global_call_routes_marks_void_sentinel_const_reason() {
    let mut module = MirModule::new("global_call_void_const_reason_test".to_string());
    let caller = make_function_with_global_call_args(
        "Helper.flag/1",
        Some(ValueId::new(7)),
        vec![ValueId::new(1)],
    );
    let mut callee = MirFunction::new(
        FunctionSignature {
            name: "Helper.flag/1".to_string(),
            params: vec![MirType::String],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    callee.params = vec![ValueId::new(1)];
    let block = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    block.instructions.push(MirInstruction::Const {
        dst: ValueId::new(2),
        value: ConstValue::Void,
    });
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(2)),
    });
    module.functions.insert("main".to_string(), caller);
    module.functions.insert("Helper.flag/1".to_string(), callee);

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(route.target_shape(), None);
    assert_eq!(
        route.target_shape_reason(),
        Some("generic_string_unsupported_void_sentinel_const")
    );
    assert_eq!(route.target_shape_blocker_symbol(), None);
    assert_eq!(route.target_shape_blocker_reason(), None);
}

#[test]
fn refresh_module_global_call_routes_marks_object_return_abi_reason() {
    let mut module = MirModule::new("global_call_object_return_reason_test".to_string());
    let caller = make_function_with_global_call_args("Helper.map/0", Some(ValueId::new(7)), vec![]);
    let callee = MirFunction::new(
        FunctionSignature {
            name: "Helper.map/0".to_string(),
            params: vec![],
            return_type: MirType::Box("MapBox".to_string()),
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    module.functions.insert("main".to_string(), caller);
    module.functions.insert("Helper.map/0".to_string(), callee);

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(route.target_shape(), None);
    assert_eq!(
        route.target_shape_reason(),
        Some("generic_string_return_object_abi_not_handle_compatible")
    );
    assert_eq!(route.target_shape_blocker_symbol(), None);
    assert_eq!(route.target_shape_blocker_reason(), None);
}

#[test]
fn refresh_module_global_call_routes_allows_null_guard_before_method_blocker() {
    let mut module = MirModule::new("global_call_null_guard_method_test".to_string());
    let caller = make_function_with_global_call_args(
        "Helper.preview/1",
        Some(ValueId::new(7)),
        vec![ValueId::new(1)],
    );
    let mut callee = MirFunction::new(
        FunctionSignature {
            name: "Helper.preview/1".to_string(),
            params: vec![MirType::String],
            return_type: MirType::String,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    callee.params = vec![ValueId::new(1)];
    let entry = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    entry.instructions.extend([
        MirInstruction::Const {
            dst: ValueId::new(2),
            value: ConstValue::Void,
        },
        MirInstruction::Compare {
            dst: ValueId::new(3),
            op: CompareOp::Eq,
            lhs: ValueId::new(1),
            rhs: ValueId::new(2),
        },
    ]);
    entry.set_terminator(MirInstruction::Branch {
        condition: ValueId::new(3),
        then_bb: BasicBlockId::new(1),
        else_bb: BasicBlockId::new(2),
        then_edge_args: None,
        else_edge_args: None,
    });
    let mut null_block = BasicBlock::new(BasicBlockId::new(1));
    null_block.instructions.push(MirInstruction::Const {
        dst: ValueId::new(4),
        value: ConstValue::String("<null>".to_string()),
    });
    null_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(4)),
    });
    let mut method_block = BasicBlock::new(BasicBlockId::new(2));
    method_block.instructions.push(MirInstruction::Call {
        dst: Some(ValueId::new(5)),
        func: ValueId::INVALID,
        callee: Some(Callee::Method {
            box_name: "RuntimeDataBox".to_string(),
            method: "debugPreview".to_string(),
            receiver: Some(ValueId::new(1)),
            certainty: TypeCertainty::Union,
            box_kind: CalleeBoxKind::RuntimeData,
        }),
        args: vec![],
        effects: EffectMask::PURE,
    });
    method_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(5)),
    });
    callee.blocks.insert(BasicBlockId::new(1), null_block);
    callee.blocks.insert(BasicBlockId::new(2), method_block);
    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("Helper.preview/1".to_string(), callee);

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(route.target_shape(), None);
    assert_eq!(
        route.target_shape_reason(),
        Some("generic_string_unsupported_method_call")
    );
    assert_eq!(route.target_shape_blocker_symbol(), None);
    assert_eq!(route.target_shape_blocker_reason(), None);
}

#[test]
fn refresh_module_global_call_routes_accepts_runtime_data_string_length_method() {
    let mut module = MirModule::new("global_call_string_len_method_test".to_string());
    let caller = make_function_with_global_call_args(
        "Helper.debug_len/1",
        Some(ValueId::new(7)),
        vec![ValueId::new(1)],
    );
    let mut coerce = MirFunction::new(
        FunctionSignature {
            name: "Helper.coerce/1".to_string(),
            params: vec![MirType::Integer],
            return_type: MirType::String,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    coerce.params = vec![ValueId::new(1)];
    let coerce_block = coerce.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    coerce_block.instructions.extend([
        MirInstruction::Const {
            dst: ValueId::new(2),
            value: ConstValue::String(String::new()),
        },
        MirInstruction::BinOp {
            dst: ValueId::new(3),
            op: BinaryOp::Add,
            lhs: ValueId::new(2),
            rhs: ValueId::new(1),
        },
    ]);
    coerce_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(3)),
    });

    let mut debug_len = MirFunction::new(
        FunctionSignature {
            name: "Helper.debug_len/1".to_string(),
            params: vec![MirType::String],
            return_type: MirType::String,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    debug_len.params = vec![ValueId::new(1)];
    let block = debug_len.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    block.instructions.extend([
        MirInstruction::Call {
            dst: Some(ValueId::new(2)),
            func: ValueId::INVALID,
            callee: Some(Callee::Global("Helper.coerce/1".to_string())),
            args: vec![ValueId::new(1)],
            effects: EffectMask::PURE,
        },
        MirInstruction::Call {
            dst: Some(ValueId::new(3)),
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: "RuntimeDataBox".to_string(),
                method: "length".to_string(),
                receiver: Some(ValueId::new(2)),
                certainty: TypeCertainty::Union,
                box_kind: CalleeBoxKind::RuntimeData,
            }),
            args: vec![],
            effects: EffectMask::PURE,
        },
        MirInstruction::Call {
            dst: Some(ValueId::new(4)),
            func: ValueId::INVALID,
            callee: Some(Callee::Global("Helper.coerce/1".to_string())),
            args: vec![ValueId::new(3)],
            effects: EffectMask::PURE,
        },
    ]);
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(4)),
    });
    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("Helper.coerce/1".to_string(), coerce);
    module
        .functions
        .insert("Helper.debug_len/1".to_string(), debug_len);

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(route.target_shape(), Some("generic_pure_string_body"));
    assert_eq!(route.target_shape_reason(), None);
    assert_eq!(route.proof(), "typed_global_call_generic_pure_string");
}

#[test]
fn refresh_module_global_call_routes_accepts_runtime_data_string_substring_method() {
    let mut module = MirModule::new("global_call_string_substring_method_test".to_string());
    let caller = make_function_with_global_call_args(
        "Helper.debug_preview/1",
        Some(ValueId::new(7)),
        vec![ValueId::new(1)],
    );
    let mut callee = MirFunction::new(
        FunctionSignature {
            name: "Helper.debug_preview/1".to_string(),
            params: vec![MirType::String],
            return_type: MirType::String,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    callee.params = vec![ValueId::new(1)];
    let block = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    block.instructions.extend([
        MirInstruction::Const {
            dst: ValueId::new(2),
            value: ConstValue::Integer(0),
        },
        MirInstruction::Const {
            dst: ValueId::new(3),
            value: ConstValue::Integer(64),
        },
        MirInstruction::Call {
            dst: Some(ValueId::new(4)),
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: "RuntimeDataBox".to_string(),
                method: "substring".to_string(),
                receiver: Some(ValueId::new(1)),
                certainty: TypeCertainty::Union,
                box_kind: CalleeBoxKind::RuntimeData,
            }),
            args: vec![ValueId::new(2), ValueId::new(3)],
            effects: EffectMask::PURE,
        },
    ]);
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(4)),
    });
    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("Helper.debug_preview/1".to_string(), callee);

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(route.target_shape(), Some("generic_pure_string_body"));
    assert_eq!(route.target_shape_reason(), None);
    assert_eq!(route.proof(), "typed_global_call_generic_pure_string");
}

#[test]
fn refresh_module_global_call_routes_accepts_print_in_generic_pure_string_body() {
    let mut module = MirModule::new("global_call_string_print_method_test".to_string());
    let caller = make_function_with_global_call_args(
        "Helper.debug_print/1",
        Some(ValueId::new(7)),
        vec![ValueId::new(1)],
    );
    let mut callee = MirFunction::new(
        FunctionSignature {
            name: "Helper.debug_print/1".to_string(),
            params: vec![MirType::String],
            return_type: MirType::String,
            effects: EffectMask::IO,
        },
        BasicBlockId::new(0),
    );
    callee.params = vec![ValueId::new(1)];
    let block = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    block.instructions.extend([
        MirInstruction::Const {
            dst: ValueId::new(2),
            value: ConstValue::String("[debug] ".to_string()),
        },
        MirInstruction::BinOp {
            dst: ValueId::new(3),
            op: BinaryOp::Add,
            lhs: ValueId::new(2),
            rhs: ValueId::new(1),
        },
        MirInstruction::Call {
            dst: None,
            func: ValueId::INVALID,
            callee: Some(Callee::Global("print".to_string())),
            args: vec![ValueId::new(3)],
            effects: EffectMask::IO,
        },
    ]);
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(1)),
    });
    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("Helper.debug_print/1".to_string(), callee);

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(route.target_shape(), Some("generic_pure_string_body"));
    assert_eq!(route.target_shape_reason(), None);
    assert_eq!(route.proof(), "typed_global_call_generic_pure_string");
}

#[test]
fn refresh_module_global_call_routes_marks_generic_i64_body_direct_target() {
    let mut module = MirModule::new("global_call_generic_i64_test".to_string());
    let caller =
        make_function_with_global_call_args("Helper.debug/0", Some(ValueId::new(7)), vec![]);
    let mut wrapper = MirFunction::new(
        FunctionSignature {
            name: "Helper.debug/0".to_string(),
            params: vec![],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    let wrapper_block = wrapper.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    wrapper_block.instructions.extend([
        MirInstruction::Const {
            dst: ValueId::new(1),
            value: ConstValue::String("DEBUG".to_string()),
        },
        MirInstruction::Call {
            dst: Some(ValueId::new(2)),
            func: ValueId::INVALID,
            callee: Some(Callee::Global("Helper.flag/1".to_string())),
            args: vec![ValueId::new(1)],
            effects: EffectMask::PURE,
        },
    ]);
    wrapper_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(2)),
    });

    let mut flag = MirFunction::new(
        FunctionSignature {
            name: "Helper.flag/1".to_string(),
            params: vec![MirType::String],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    flag.params = vec![ValueId::new(1)];
    let entry = flag.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    entry.instructions.extend([
        MirInstruction::Call {
            dst: Some(ValueId::new(2)),
            func: ValueId::INVALID,
            callee: Some(Callee::Extern("env.get/1".to_string())),
            args: vec![ValueId::new(1)],
            effects: EffectMask::PURE,
        },
        MirInstruction::Const {
            dst: ValueId::new(3),
            value: ConstValue::Void,
        },
        MirInstruction::Compare {
            dst: ValueId::new(4),
            op: CompareOp::Ne,
            lhs: ValueId::new(2),
            rhs: ValueId::new(3),
        },
    ]);
    entry.set_terminator(MirInstruction::Branch {
        condition: ValueId::new(4),
        then_bb: BasicBlockId::new(1),
        else_bb: BasicBlockId::new(2),
        then_edge_args: None,
        else_edge_args: None,
    });
    let mut yes_block = BasicBlock::new(BasicBlockId::new(1));
    yes_block.instructions.push(MirInstruction::Const {
        dst: ValueId::new(5),
        value: ConstValue::Integer(1),
    });
    yes_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(5)),
    });
    let mut no_block = BasicBlock::new(BasicBlockId::new(2));
    no_block.instructions.push(MirInstruction::Const {
        dst: ValueId::new(6),
        value: ConstValue::Integer(0),
    });
    no_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(6)),
    });
    flag.blocks.insert(BasicBlockId::new(1), yes_block);
    flag.blocks.insert(BasicBlockId::new(2), no_block);

    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("Helper.debug/0".to_string(), wrapper);
    module.functions.insert("Helper.flag/1".to_string(), flag);

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(
        route.target_shape(),
        Some("generic_i64_body"),
        "reason={:?} blocker={:?}/{:?}",
        route.target_shape_reason(),
        route.target_shape_blocker_symbol(),
        route.target_shape_blocker_reason()
    );
    assert_eq!(route.target_shape_reason(), None);
    assert_eq!(route.tier(), "DirectAbi");
    assert_eq!(route.proof(), "typed_global_call_generic_i64");
    assert_eq!(route.return_shape(), Some("ScalarI64"));
    assert_eq!(route.value_demand(), "scalar_i64");

    let wrapper_route = &module.functions["Helper.debug/0"]
        .metadata
        .global_call_routes[0];
    assert_eq!(wrapper_route.target_shape(), Some("generic_i64_body"));
    assert_eq!(wrapper_route.proof(), "typed_global_call_generic_i64");
}

#[test]
fn refresh_module_global_call_routes_marks_string_scan_generic_i64_body() {
    let mut module = MirModule::new("global_call_string_scan_i64_body_test".to_string());
    let caller = make_function_with_global_call_args(
        "Helper.find/3",
        Some(ValueId::new(20)),
        vec![ValueId::new(1), ValueId::new(2), ValueId::new(3)],
    );
    let mut callee = MirFunction::new(
        FunctionSignature {
            name: "Helper.find/3".to_string(),
            params: vec![MirType::Unknown, MirType::Unknown, MirType::Unknown],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    callee.params = vec![ValueId::new(1), ValueId::new(2), ValueId::new(3)];
    callee
        .metadata
        .value_types
        .insert(ValueId::new(3), MirType::Integer);
    let entry = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    entry.instructions.extend([
        MirInstruction::Call {
            dst: Some(ValueId::new(4)),
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: "RuntimeDataBox".to_string(),
                method: "length".to_string(),
                receiver: Some(ValueId::new(1)),
                certainty: TypeCertainty::Union,
                box_kind: CalleeBoxKind::RuntimeData,
            }),
            args: vec![],
            effects: EffectMask::PURE,
        },
        MirInstruction::Call {
            dst: Some(ValueId::new(5)),
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: "RuntimeDataBox".to_string(),
                method: "length".to_string(),
                receiver: Some(ValueId::new(2)),
                certainty: TypeCertainty::Union,
                box_kind: CalleeBoxKind::RuntimeData,
            }),
            args: vec![],
            effects: EffectMask::PURE,
        },
        MirInstruction::BinOp {
            dst: ValueId::new(6),
            op: BinaryOp::Add,
            lhs: ValueId::new(3),
            rhs: ValueId::new(5),
        },
        MirInstruction::Call {
            dst: Some(ValueId::new(7)),
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: "RuntimeDataBox".to_string(),
                method: "substring".to_string(),
                receiver: Some(ValueId::new(1)),
                certainty: TypeCertainty::Union,
                box_kind: CalleeBoxKind::RuntimeData,
            }),
            args: vec![ValueId::new(3), ValueId::new(6)],
            effects: EffectMask::PURE,
        },
        MirInstruction::Compare {
            dst: ValueId::new(8),
            op: CompareOp::Eq,
            lhs: ValueId::new(7),
            rhs: ValueId::new(2),
        },
    ]);
    entry.set_terminator(MirInstruction::Branch {
        condition: ValueId::new(8),
        then_bb: BasicBlockId::new(1),
        else_bb: BasicBlockId::new(2),
        then_edge_args: None,
        else_edge_args: None,
    });
    let mut found_block = BasicBlock::new(BasicBlockId::new(1));
    found_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(3)),
    });
    let mut missing_block = BasicBlock::new(BasicBlockId::new(2));
    missing_block.instructions.push(MirInstruction::Const {
        dst: ValueId::new(9),
        value: ConstValue::Integer(-1),
    });
    missing_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(9)),
    });
    callee.blocks.insert(BasicBlockId::new(1), found_block);
    callee.blocks.insert(BasicBlockId::new(2), missing_block);
    module.functions.insert("main".to_string(), caller);
    module.functions.insert("Helper.find/3".to_string(), callee);

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(
        route.target_shape(),
        Some("generic_i64_body"),
        "reason={:?} blocker={:?}/{:?}",
        route.target_shape_reason(),
        route.target_shape_blocker_symbol(),
        route.target_shape_blocker_reason()
    );
    assert_eq!(route.target_shape_reason(), None);
    assert_eq!(route.proof(), "typed_global_call_generic_i64");
    assert_eq!(route.return_shape(), Some("ScalarI64"));
}

#[test]
fn refresh_module_global_call_routes_marks_method_call_shape_reason() {
    let mut module = MirModule::new("global_call_method_reason_test".to_string());
    let caller = make_function_with_global_call_args(
        "Helper.slice/1",
        Some(ValueId::new(7)),
        vec![ValueId::new(1)],
    );
    let mut callee = MirFunction::new(
        FunctionSignature {
            name: "Helper.slice/1".to_string(),
            params: vec![MirType::String],
            return_type: MirType::String,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    callee.params = vec![ValueId::new(1)];
    let block = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    block.instructions.extend([
        MirInstruction::Const {
            dst: ValueId::new(2),
            value: ConstValue::Integer(0),
        },
        MirInstruction::Const {
            dst: ValueId::new(3),
            value: ConstValue::Integer(1),
        },
        MirInstruction::Call {
            dst: Some(ValueId::new(4)),
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: "RuntimeDataBox".to_string(),
                method: "debugPreview".to_string(),
                receiver: Some(ValueId::new(1)),
                certainty: TypeCertainty::Known,
                box_kind: CalleeBoxKind::RuntimeData,
            }),
            args: vec![ValueId::new(2), ValueId::new(3)],
            effects: EffectMask::PURE,
        },
    ]);
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(4)),
    });
    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("Helper.slice/1".to_string(), callee);

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(route.target_shape(), None);
    assert_eq!(
        route.target_shape_reason(),
        Some("generic_string_unsupported_method_call")
    );
    assert_eq!(route.target_shape_blocker_symbol(), None);
    assert_eq!(route.target_shape_blocker_reason(), None);
}

#[test]
fn refresh_module_global_call_routes_marks_unknown_child_target_shape_reason() {
    let mut module = MirModule::new("global_call_child_reason_test".to_string());
    let caller =
        make_function_with_global_call_args("Helper.wrapper/0", Some(ValueId::new(7)), vec![]);
    let mut wrapper = MirFunction::new(
        FunctionSignature {
            name: "Helper.wrapper/0".to_string(),
            params: vec![],
            return_type: MirType::String,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    let block = wrapper.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    block.instructions.push(MirInstruction::Call {
        dst: Some(ValueId::new(1)),
        func: ValueId::INVALID,
        callee: Some(Callee::Global("Helper.pending/0".to_string())),
        args: vec![],
        effects: EffectMask::PURE,
    });
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(1)),
    });
    let pending = MirFunction::new(
        FunctionSignature {
            name: "Helper.pending/0".to_string(),
            params: vec![],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("Helper.wrapper/0".to_string(), wrapper);
    module
        .functions
        .insert("Helper.pending/0".to_string(), pending);

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(route.target_shape(), None);
    assert_eq!(
        route.target_shape_reason(),
        Some("generic_string_global_target_shape_unknown")
    );
    assert_eq!(
        route.target_shape_blocker_symbol(),
        Some("Helper.pending/0")
    );
    assert_eq!(
        route.target_shape_blocker_reason(),
        Some("generic_string_no_string_surface")
    );
}

#[test]
fn refresh_module_global_call_routes_marks_numeric_i64_leaf_direct_target() {
    let mut module = MirModule::new("global_call_leaf_test".to_string());
    let caller = make_function_with_global_call("Helper.add/2", Some(ValueId::new(7)));
    let mut callee = MirFunction::new(
        FunctionSignature {
            name: "Helper.add/2".to_string(),
            params: vec![MirType::Integer, MirType::Integer],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    callee.params = vec![ValueId::new(1), ValueId::new(2)];
    let block = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    block.instructions.push(MirInstruction::BinOp {
        dst: ValueId::new(3),
        op: BinaryOp::Add,
        lhs: ValueId::new(1),
        rhs: ValueId::new(2),
    });
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(3)),
    });
    module.functions.insert("main".to_string(), caller);
    module.functions.insert("Helper.add/2".to_string(), callee);

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert!(route.target_exists());
    assert_eq!(route.target_symbol(), Some("Helper.add/2"));
    assert_eq!(route.target_return_type(), Some("i64".to_string()));
    assert_eq!(route.target_shape(), Some("numeric_i64_leaf"));
    assert_eq!(route.target_shape_reason(), None);
    assert_eq!(route.target_arity(), Some(2));
    assert_eq!(route.arity_matches(), Some(true));
    assert_eq!(route.tier(), "DirectAbi");
    assert_eq!(route.emit_kind(), "direct_function_call");
    assert_eq!(route.proof(), "typed_global_call_leaf_numeric_i64");
    assert_eq!(route.return_shape(), Some("ScalarI64"));
    assert_eq!(route.value_demand(), "scalar_i64");
    assert_eq!(route.reason(), None);
}

#[test]
fn refresh_module_global_call_routes_resolves_static_entry_alias_to_target_symbol() {
    let mut module = MirModule::new("global_call_static_entry_alias_test".to_string());
    let caller =
        make_function_with_global_call_args("main._helper/0", Some(ValueId::new(7)), vec![]);
    let mut callee = MirFunction::new(
        FunctionSignature {
            name: "Main._helper/0".to_string(),
            params: vec![],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    let block = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    block.instructions.push(MirInstruction::Const {
        dst: ValueId::new(1),
        value: ConstValue::Integer(42),
    });
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(1)),
    });
    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("Main._helper/0".to_string(), callee);

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(route.callee_name(), "main._helper/0");
    assert!(route.target_exists());
    assert_eq!(route.target_symbol(), Some("Main._helper/0"));
    assert_eq!(route.target_arity(), Some(0));
    assert_eq!(route.target_return_type(), Some("i64".to_string()));
    assert_eq!(route.arity_matches(), Some(true));
    assert_eq!(route.target_shape(), Some("numeric_i64_leaf"));
    assert_eq!(route.target_shape_reason(), None);
    assert_eq!(route.tier(), "DirectAbi");
    assert_eq!(route.reason(), None);
}

#[test]
fn refresh_module_global_call_routes_marks_generic_pure_string_body_direct_target() {
    let mut module = MirModule::new("global_call_generic_string_test".to_string());
    let caller = make_function_with_global_call("Helper.normalize/2", Some(ValueId::new(7)));
    let mut callee = MirFunction::new(
        FunctionSignature {
            name: "Helper.normalize/2".to_string(),
            params: vec![MirType::String, MirType::String],
            return_type: MirType::String,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    callee.params = vec![ValueId::new(1), ValueId::new(9)];
    let entry = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    entry.instructions.extend([
        MirInstruction::Const {
            dst: ValueId::new(2),
            value: ConstValue::String("dev".to_string()),
        },
        MirInstruction::Compare {
            dst: ValueId::new(3),
            op: CompareOp::Eq,
            lhs: ValueId::new(1),
            rhs: ValueId::new(2),
        },
    ]);
    entry.set_terminator(MirInstruction::Branch {
        condition: ValueId::new(3),
        then_bb: BasicBlockId::new(1),
        else_bb: BasicBlockId::new(2),
        then_edge_args: None,
        else_edge_args: None,
    });

    let mut then_block = BasicBlock::new(BasicBlockId::new(1));
    then_block.instructions.push(MirInstruction::Const {
        dst: ValueId::new(4),
        value: ConstValue::String("vm".to_string()),
    });
    then_block.set_terminator(MirInstruction::Jump {
        target: BasicBlockId::new(3),
        edge_args: None,
    });

    let mut else_block = BasicBlock::new(BasicBlockId::new(2));
    else_block.instructions.push(MirInstruction::Copy {
        dst: ValueId::new(5),
        src: ValueId::new(1),
    });
    else_block.set_terminator(MirInstruction::Jump {
        target: BasicBlockId::new(3),
        edge_args: None,
    });

    let mut merge_block = BasicBlock::new(BasicBlockId::new(3));
    merge_block.instructions.push(MirInstruction::Phi {
        dst: ValueId::new(6),
        inputs: vec![
            (BasicBlockId::new(1), ValueId::new(4)),
            (BasicBlockId::new(2), ValueId::new(5)),
        ],
        type_hint: Some(MirType::String),
    });
    merge_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(6)),
    });

    callee.blocks.insert(BasicBlockId::new(1), then_block);
    callee.blocks.insert(BasicBlockId::new(2), else_block);
    callee.blocks.insert(BasicBlockId::new(3), merge_block);
    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("Helper.normalize/2".to_string(), callee);

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert!(route.target_exists());
    assert_eq!(route.target_symbol(), Some("Helper.normalize/2"));
    assert_eq!(route.target_return_type(), Some("str".to_string()));
    assert_eq!(route.target_shape(), Some("generic_pure_string_body"));
    assert_eq!(route.target_shape_reason(), None);
    assert_eq!(route.target_arity(), Some(2));
    assert_eq!(route.arity_matches(), Some(true));
    assert_eq!(route.tier(), "DirectAbi");
    assert_eq!(route.emit_kind(), "direct_function_call");
    assert_eq!(route.proof(), "typed_global_call_generic_pure_string");
    assert_eq!(route.return_shape(), Some("string_handle"));
    assert_eq!(route.value_demand(), "runtime_i64_or_handle");
    assert_eq!(route.reason(), None);
}
