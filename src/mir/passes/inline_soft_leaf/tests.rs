use super::*;
use crate::ast::RuneAttr;
use crate::mir::{
    BasicBlock, BasicBlockId, BinaryOp, ConstValue, EffectMask, FunctionSignature, MirType,
};

fn make_add1_inline_function() -> MirFunction {
    let signature = FunctionSignature {
        name: "Main.add1/1".to_string(),
        params: vec![MirType::Integer],
        return_type: MirType::Integer,
        effects: EffectMask::PURE,
    };
    let mut function = MirFunction::new(signature, BasicBlockId(0));
    let one = function.next_value_id();
    let sum = function.next_value_id();
    let mut entry = BasicBlock::new(BasicBlockId(0));
    entry.add_instruction(MirInstruction::Const {
        dst: one,
        value: ConstValue::Integer(1),
    });
    entry.add_instruction(MirInstruction::BinOp {
        dst: sum,
        op: BinaryOp::Add,
        lhs: ValueId(0),
        rhs: one,
    });
    entry.add_instruction(MirInstruction::Return { value: Some(sum) });
    function.blocks.insert(BasicBlockId(0), entry);
    function.metadata.runes = vec![RuneAttr {
        name: "Hint".to_string(),
        args: vec!["inline".to_string()],
    }];
    crate::mir::rune_plan_refresh::refresh_function_rune_plans(&mut function);
    function
}

fn make_main_calling_add1() -> MirFunction {
    let signature = FunctionSignature {
        name: "Main.main/0".to_string(),
        params: vec![],
        return_type: MirType::Integer,
        effects: EffectMask::PURE,
    };
    let mut function = MirFunction::new(signature, BasicBlockId(0));
    let arg = function.next_value_id();
    let result = function.next_value_id();
    let mut entry = BasicBlock::new(BasicBlockId(0));
    entry.add_instruction(MirInstruction::Const {
        dst: arg,
        value: ConstValue::Integer(41),
    });
    entry.add_instruction(MirInstruction::Call {
        dst: Some(result),
        func: ValueId::INVALID,
        callee: Some(Callee::Global("Main.add1/1".to_string())),
        args: vec![arg],
        effects: EffectMask::PURE,
    });
    entry.add_instruction(MirInstruction::Return {
        value: Some(result),
    });
    function.blocks.insert(BasicBlockId(0), entry);
    function
}

#[test]
fn inline_soft_leaf_rewrites_same_module_prefer_global_call() {
    let mut module = MirModule::new("inline_soft_leaf_test".to_string());
    module.add_function(make_add1_inline_function());
    module.add_function(make_main_calling_add1());

    assert_eq!(apply(&mut module), 1);

    let main = module.get_function("Main.main/0").expect("main function");
    let entry = main.entry_block();
    assert!(!entry
        .instructions
        .iter()
        .any(|inst| matches!(inst, MirInstruction::Call { .. })));
    assert!(entry.instructions.iter().any(|inst| matches!(
        inst,
        MirInstruction::Copy {
            dst: ValueId(2),
            ..
        }
    )));
}

#[test]
fn inline_soft_leaf_keeps_call_without_prefer_plan() {
    let mut module = MirModule::new("inline_soft_leaf_no_plan_test".to_string());
    let mut callee = make_add1_inline_function();
    callee.metadata.runes.clear();
    crate::mir::rune_plan_refresh::refresh_function_rune_plans(&mut callee);
    module.add_function(callee);
    module.add_function(make_main_calling_add1());

    assert_eq!(apply(&mut module), 0);

    let main = module.get_function("Main.main/0").expect("main function");
    assert!(main
        .entry_block()
        .instructions
        .iter()
        .any(|inst| matches!(inst, MirInstruction::Call { .. })));
}

#[test]
fn inline_soft_leaf_rewrites_verified_required_global_call() {
    let mut module = MirModule::new("inline_required_leaf_test".to_string());
    let mut callee = make_add1_inline_function();
    callee.metadata.runes = vec![RuneAttr {
        name: "Inline".to_string(),
        args: vec!["required".to_string()],
    }];
    crate::mir::rune_plan_refresh::refresh_function_rune_plans(&mut callee);
    assert!(callee.metadata.inline_plans.iter().any(|plan| matches!(
        plan.request,
        crate::mir::inline_plan::InlineRequest::Required
    ) && plan.verified));
    module.add_function(callee);
    module.add_function(make_main_calling_add1());

    assert_eq!(apply(&mut module), 1);

    let main = module.get_function("Main.main/0").expect("main function");
    assert!(!main
        .entry_block()
        .instructions
        .iter()
        .any(|inst| matches!(inst, MirInstruction::Call { .. })));
}

fn make_reset_inline_function() -> MirFunction {
    let signature = FunctionSignature {
        name: "Main.reset/1".to_string(),
        params: vec![MirType::Unknown],
        return_type: MirType::Void,
        effects: EffectMask::WRITE,
    };
    let mut function = MirFunction::new(signature, BasicBlockId(0));
    let minus_one = function.next_value_id();
    let mut entry = BasicBlock::new(BasicBlockId(0));
    entry.add_instruction(MirInstruction::Const {
        dst: minus_one,
        value: ConstValue::Integer(-1),
    });
    entry.add_instruction(MirInstruction::FieldSet {
        base: ValueId(0),
        field: "last_selected_index".to_string(),
        value: minus_one,
        declared_type: Some(MirType::Integer),
    });
    entry.add_instruction(MirInstruction::Return { value: None });
    function.blocks.insert(BasicBlockId(0), entry);
    function.metadata.runes = vec![RuneAttr {
        name: "Inline".to_string(),
        args: vec!["required".to_string()],
    }];
    crate::mir::rune_plan_refresh::refresh_function_rune_plans(&mut function);
    function
}

fn make_implicit_reset_inline_function() -> MirFunction {
    let signature = FunctionSignature {
        name: "Main.resetImplicit/0".to_string(),
        params: vec![],
        return_type: MirType::Void,
        effects: EffectMask::WRITE,
    };
    let mut function = MirFunction::new(signature, BasicBlockId(0));
    let minus_one = function.next_value_id();
    let mut entry = BasicBlock::new(BasicBlockId(0));
    entry.add_instruction(MirInstruction::Const {
        dst: minus_one,
        value: ConstValue::Integer(-1),
    });
    entry.add_instruction(MirInstruction::FieldSet {
        base: ValueId::INVALID,
        field: "last_selected_index".to_string(),
        value: minus_one,
        declared_type: Some(MirType::Integer),
    });
    entry.add_instruction(MirInstruction::Return { value: None });
    function.blocks.insert(BasicBlockId(0), entry);
    function.metadata.runes = vec![RuneAttr {
        name: "Inline".to_string(),
        args: vec!["required".to_string()],
    }];
    crate::mir::rune_plan_refresh::refresh_function_rune_plans(&mut function);
    function
}

fn make_main_calling_implicit_reset() -> MirFunction {
    let signature = FunctionSignature {
        name: "Main.main/0".to_string(),
        params: vec![],
        return_type: MirType::Void,
        effects: EffectMask::WRITE,
    };
    let mut function = MirFunction::new(signature, BasicBlockId(0));
    let mut entry = BasicBlock::new(BasicBlockId(0));
    entry.add_instruction(MirInstruction::Call {
        dst: None,
        func: ValueId::INVALID,
        callee: Some(Callee::Global("Main.resetImplicit/0".to_string())),
        args: vec![],
        effects: EffectMask::WRITE,
    });
    entry.add_instruction(MirInstruction::Return { value: None });
    function.blocks.insert(BasicBlockId(0), entry);
    function
}

fn make_main_calling_reset() -> MirFunction {
    let signature = FunctionSignature {
        name: "Main.main/0".to_string(),
        params: vec![],
        return_type: MirType::Void,
        effects: EffectMask::WRITE,
    };
    let mut function = MirFunction::new(signature, BasicBlockId(0));
    let receiver = function.next_value_id();
    let mut entry = BasicBlock::new(BasicBlockId(0));
    entry.add_instruction(MirInstruction::Const {
        dst: receiver,
        value: ConstValue::Null,
    });
    entry.add_instruction(MirInstruction::Call {
        dst: None,
        func: ValueId::INVALID,
        callee: Some(Callee::Global("Main.reset/1".to_string())),
        args: vec![receiver],
        effects: EffectMask::WRITE,
    });
    entry.add_instruction(MirInstruction::Return { value: None });
    function.blocks.insert(BasicBlockId(0), entry);
    function
}

fn make_main_calling_method_reset() -> MirFunction {
    let signature = FunctionSignature {
        name: "Main.main/0".to_string(),
        params: vec![],
        return_type: MirType::Void,
        effects: EffectMask::WRITE,
    };
    let mut function = MirFunction::new(signature, BasicBlockId(0));
    let receiver = function.next_value_id();
    let mut entry = BasicBlock::new(BasicBlockId(0));
    entry.add_instruction(MirInstruction::Const {
        dst: receiver,
        value: ConstValue::Null,
    });
    entry.add_instruction(MirInstruction::Call {
        dst: None,
        func: ValueId::INVALID,
        callee: Some(Callee::Method {
            box_name: "Main".to_string(),
            method: "reset".to_string(),
            receiver: Some(receiver),
            certainty: crate::mir::definitions::call_unified::TypeCertainty::Known,
            box_kind: crate::mir::definitions::call_unified::CalleeBoxKind::UserDefined,
        }),
        args: vec![],
        effects: EffectMask::WRITE,
    });
    entry.add_instruction(MirInstruction::Return { value: None });
    function.blocks.insert(BasicBlockId(0), entry);
    function
}

#[test]
fn inline_soft_leaf_rewrites_verified_required_receiver_fieldset_call() {
    let mut module = MirModule::new("inline_required_receiver_fieldset_test".to_string());
    let callee = make_reset_inline_function();
    assert!(callee.metadata.inline_plans.iter().any(|plan| matches!(
        plan.request,
        crate::mir::inline_plan::InlineRequest::Required
    ) && plan.verified));
    module.add_function(callee);
    module.add_function(make_main_calling_reset());

    assert_eq!(apply(&mut module), 1);

    let main = module.get_function("Main.main/0").expect("main function");
    let entry = main.entry_block();
    assert!(!entry
        .instructions
        .iter()
        .any(|inst| matches!(inst, MirInstruction::Call { .. })));
    assert!(entry.instructions.iter().any(|inst| matches!(
        inst,
        MirInstruction::FieldSet {
            base: ValueId(1),
            field,
            ..
        } if field == "last_selected_index"
    )));
}

#[test]
fn inline_soft_leaf_rewrites_verified_required_user_method_call() {
    let mut module =
        MirModule::new("inline_required_user_method_receiver_fieldset_test".to_string());
    let mut callee = make_reset_inline_function();
    callee.signature.name = "Main.reset/0".to_string();
    assert!(callee.metadata.inline_plans.iter().any(|plan| matches!(
        plan.request,
        crate::mir::inline_plan::InlineRequest::Required
    ) && plan.verified));
    module.add_function(callee);
    module.add_function(make_main_calling_method_reset());

    assert_eq!(apply(&mut module), 1);

    let main = module.get_function("Main.main/0").expect("main function");
    let entry = main.entry_block();
    assert!(!entry
        .instructions
        .iter()
        .any(|inst| matches!(inst, MirInstruction::Call { .. })));
    assert!(entry.instructions.iter().any(|inst| matches!(
        inst,
        MirInstruction::FieldSet {
            base: ValueId(1),
            field,
            ..
        } if field == "last_selected_index"
    )));
}

#[test]
fn inline_soft_leaf_rewrites_verified_required_implicit_receiver_fieldset_call() {
    let mut module = MirModule::new("inline_required_implicit_receiver_fieldset_test".to_string());
    let callee = make_implicit_reset_inline_function();
    assert!(callee.metadata.inline_plans.iter().any(|plan| matches!(
        plan.request,
        crate::mir::inline_plan::InlineRequest::Required
    ) && plan.verified));
    module.add_function(callee);
    module.add_function(make_main_calling_implicit_reset());

    assert_eq!(apply(&mut module), 1);

    let main = module.get_function("Main.main/0").expect("main function");
    let entry = main.entry_block();
    assert!(!entry
        .instructions
        .iter()
        .any(|inst| matches!(inst, MirInstruction::Call { .. })));
    assert!(entry.instructions.iter().any(|inst| matches!(
        inst,
        MirInstruction::FieldSet {
            base: ValueId::INVALID,
            field,
            ..
        } if field == "last_selected_index"
    )));
}

#[test]
fn inline_soft_leaf_keeps_recursive_call() {
    let mut module = MirModule::new("inline_soft_leaf_recursive_test".to_string());
    let mut function = make_add1_inline_function();
    function.signature.name = "Main.main/0".to_string();
    function
        .blocks
        .get_mut(&BasicBlockId(0))
        .expect("entry")
        .instructions
        .push(MirInstruction::Call {
            dst: None,
            func: ValueId::INVALID,
            callee: Some(Callee::Global("Main.main/0".to_string())),
            args: vec![],
            effects: EffectMask::PURE,
        });
    module.add_function(function);

    assert_eq!(apply(&mut module), 0);
}
