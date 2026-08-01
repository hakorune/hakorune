use super::convert::convert_mir_like_inst;
use crate::mir::{BinaryOp, CompareOp as MirCompareOp, Effect, MirInstruction, ValueId};

#[test]
fn test_convert_const_inst() {
    let join_const = crate::mir::join_ir::MirLikeInst::Const {
        dst: ValueId(10),
        value: crate::mir::join_ir::ConstValue::Integer(42),
    };

    let mir_inst = convert_mir_like_inst(&join_const).unwrap();

    match mir_inst {
        MirInstruction::Const { dst, value } => {
            assert_eq!(dst, ValueId(10));
            assert!(matches!(value, crate::mir::ConstValue::Integer(42)));
        }
        _ => panic!("Expected Const instruction"),
    }
}

#[test]
fn test_convert_binop_inst() {
    let join_binop = crate::mir::join_ir::MirLikeInst::BinOp {
        dst: ValueId(20),
        op: crate::mir::join_ir::BinOpKind::Add,
        lhs: ValueId(10),
        rhs: ValueId(11),
    };

    let mir_inst = convert_mir_like_inst(&join_binop).unwrap();

    match mir_inst {
        MirInstruction::BinOp { dst, op, lhs, rhs } => {
            assert_eq!(dst, ValueId(20));
            assert_eq!(op, BinaryOp::Add);
            assert_eq!(lhs, ValueId(10));
            assert_eq!(rhs, ValueId(11));
        }
        _ => panic!("Expected BinOp instruction"),
    }
}

#[test]
fn test_convert_compare_inst() {
    let join_cmp = crate::mir::join_ir::MirLikeInst::Compare {
        dst: ValueId(30),
        op: crate::mir::join_ir::CompareOp::Ge,
        lhs: ValueId(10),
        rhs: ValueId(11),
    };

    let mir_inst = convert_mir_like_inst(&join_cmp).unwrap();

    match mir_inst {
        MirInstruction::Compare { dst, op, lhs, rhs } => {
            assert_eq!(dst, ValueId(30));
            assert_eq!(op, MirCompareOp::Ge);
            assert_eq!(lhs, ValueId(10));
            assert_eq!(rhs, ValueId(11));
        }
        _ => panic!("Expected Compare instruction"),
    }
}

#[test]
fn test_convert_print_inst_to_externcall() {
    let join_print = crate::mir::join_ir::MirLikeInst::Print { value: ValueId(7) };

    let mir_inst = convert_mir_like_inst(&join_print).unwrap();

    // Should now emit canonical Call with Callee::Extern
    match mir_inst {
        MirInstruction::Call {
            dst,
            callee: Some(crate::mir::Callee::Extern(name)),
            args,
            effects,
            ..
        } => {
            assert_eq!(dst, None);
            assert_eq!(name, "env.console.log");
            assert_eq!(args, vec![ValueId(7)]);
            assert!(effects.contains(Effect::Io));
        }
        _ => panic!("Expected Call(callee=Extern) instruction"),
    }
}

/// Phase 45: String 定数の MIR 変換テスト
#[test]
fn test_convert_string_const_inst() {
    let join_const = crate::mir::join_ir::MirLikeInst::Const {
        dst: ValueId(50),
        value: crate::mir::join_ir::ConstValue::String("\"".to_string()),
    };

    let mir_inst = convert_mir_like_inst(&join_const).unwrap();

    match mir_inst {
        MirInstruction::Const { dst, value } => {
            assert_eq!(dst, ValueId(50));
            match value {
                crate::mir::ConstValue::String(s) => assert_eq!(s, "\""),
                _ => panic!("Expected String value"),
            }
        }
        _ => panic!("Expected Const instruction"),
    }
}
