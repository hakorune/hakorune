use super::canonicalize_string_concat3;
use crate::ast::Span;
use crate::mir::{
    BasicBlockId, BinaryOp, Callee, ConstValue, EffectMask, FunctionSignature, MirFunction,
    MirInstruction, MirModule, MirType, ValueId,
};

fn build_concat3_chain_module(right_assoc: bool) -> MirModule {
    let mut module = MirModule::new("concat3".to_string());
    let signature = FunctionSignature {
        name: "main".to_string(),
        params: vec![],
        return_type: MirType::String,
        effects: EffectMask::PURE,
    };
    let mut func = MirFunction::new(signature, BasicBlockId(0));
    let block = func
        .blocks
        .get_mut(&BasicBlockId(0))
        .expect("entry block exists");

    block.instructions.push(MirInstruction::Const {
        dst: ValueId(1),
        value: ConstValue::String("ha".to_string()),
    });
    block.instruction_spans.push(Span::unknown());

    block.instructions.push(MirInstruction::Const {
        dst: ValueId(2),
        value: ConstValue::String("ko".to_string()),
    });
    block.instruction_spans.push(Span::unknown());

    block.instructions.push(MirInstruction::Const {
        dst: ValueId(3),
        value: ConstValue::String("run".to_string()),
    });
    block.instruction_spans.push(Span::unknown());

    if right_assoc {
        // %4 = %2 + %3
        block.instructions.push(MirInstruction::BinOp {
            dst: ValueId(4),
            op: BinaryOp::Add,
            lhs: ValueId(2),
            rhs: ValueId(3),
        });
        block.instruction_spans.push(Span::unknown());
        // %5 = %1 + %4
        block.instructions.push(MirInstruction::BinOp {
            dst: ValueId(5),
            op: BinaryOp::Add,
            lhs: ValueId(1),
            rhs: ValueId(4),
        });
        block.instruction_spans.push(Span::unknown());
    } else {
        // %4 = %1 + %2
        block.instructions.push(MirInstruction::BinOp {
            dst: ValueId(4),
            op: BinaryOp::Add,
            lhs: ValueId(1),
            rhs: ValueId(2),
        });
        block.instruction_spans.push(Span::unknown());
        // %5 = %4 + %3
        block.instructions.push(MirInstruction::BinOp {
            dst: ValueId(5),
            op: BinaryOp::Add,
            lhs: ValueId(4),
            rhs: ValueId(3),
        });
        block.instruction_spans.push(Span::unknown());
    }

    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId(5)),
    });

    func.metadata
        .value_types
        .insert(ValueId(1), MirType::String);
    func.metadata
        .value_types
        .insert(ValueId(2), MirType::String);
    func.metadata
        .value_types
        .insert(ValueId(3), MirType::String);
    func.metadata
        .value_types
        .insert(ValueId(4), MirType::String);
    func.metadata
        .value_types
        .insert(ValueId(5), MirType::String);

    module.add_function(func);
    module
}

#[test]
fn rewrites_left_assoc_chain_to_concat3() {
    let mut module = build_concat3_chain_module(false);
    let rewritten = canonicalize_string_concat3(&mut module);
    assert_eq!(rewritten, 1);

    let block = &module
        .get_function("main")
        .expect("main exists")
        .blocks
        .get(&BasicBlockId(0))
        .expect("entry exists");
    assert_eq!(block.instructions.len(), block.instruction_spans.len());

    let mut saw_concat3 = false;
    let mut saw_add = false;
    for inst in &block.instructions {
        match inst {
            MirInstruction::Call {
                dst,
                callee: Some(Callee::Extern(name)),
                args,
                ..
            } if *dst == Some(ValueId(5)) && name == "nyash.string.concat3_hhh" => {
                saw_concat3 = true;
                assert_eq!(args, &vec![ValueId(1), ValueId(2), ValueId(3)]);
            }
            MirInstruction::BinOp {
                op: BinaryOp::Add, ..
            } => saw_add = true,
            _ => {}
        }
    }
    assert!(saw_concat3);
    assert!(!saw_add, "inner/outer Add should both be eliminated");
}

#[test]
fn rewrites_right_assoc_chain_to_concat3() {
    let mut module = build_concat3_chain_module(true);
    let rewritten = canonicalize_string_concat3(&mut module);
    assert_eq!(rewritten, 1);

    let block = &module
        .get_function("main")
        .expect("main exists")
        .blocks
        .get(&BasicBlockId(0))
        .expect("entry exists");
    assert_eq!(block.instructions.len(), block.instruction_spans.len());

    let mut saw_concat3 = false;
    let mut saw_add = false;
    for inst in &block.instructions {
        match inst {
            MirInstruction::Call {
                dst,
                callee: Some(Callee::Extern(name)),
                args,
                ..
            } if *dst == Some(ValueId(5)) && name == "nyash.string.concat3_hhh" => {
                saw_concat3 = true;
                assert_eq!(args, &vec![ValueId(1), ValueId(2), ValueId(3)]);
            }
            MirInstruction::BinOp {
                op: BinaryOp::Add, ..
            } => saw_add = true,
            _ => {}
        }
    }
    assert!(saw_concat3);
    assert!(!saw_add, "inner/outer Add should both be eliminated");
}

#[test]
fn keeps_non_chain_add_unchanged() {
    let mut module = MirModule::new("concat3_non_chain".to_string());
    let signature = FunctionSignature {
        name: "main".to_string(),
        params: vec![],
        return_type: MirType::String,
        effects: EffectMask::PURE,
    };
    let mut func = MirFunction::new(signature, BasicBlockId(0));
    let block = func
        .blocks
        .get_mut(&BasicBlockId(0))
        .expect("entry block exists");

    block.instructions.push(MirInstruction::Const {
        dst: ValueId(1),
        value: ConstValue::String("ha".to_string()),
    });
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(MirInstruction::Const {
        dst: ValueId(2),
        value: ConstValue::String("ko".to_string()),
    });
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(MirInstruction::BinOp {
        dst: ValueId(3),
        op: BinaryOp::Add,
        lhs: ValueId(1),
        rhs: ValueId(2),
    });
    block.instruction_spans.push(Span::unknown());
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId(3)),
    });

    module.add_function(func);

    let rewritten = canonicalize_string_concat3(&mut module);
    assert_eq!(rewritten, 0);
}
