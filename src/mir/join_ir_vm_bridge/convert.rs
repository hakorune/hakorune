use crate::mir::join_ir::{BinOpKind, CompareOp, ConstValue, JoinModule, MirLikeInst};
use crate::mir::{
    BinaryOp, CompareOp as MirCompareOp, ConstValue as MirConstValue, Effect, EffectMask,
    MirInstruction, MirModule,
};
use crate::mir::ssot::extern_call::extern_call as build_extern_call;

use super::JoinIrVmBridgeError;

// Phase 190: Use modular converters from parent module
use super::joinir_function_converter::JoinIrFunctionConverter;

/// Phase 190: JoinIR → MIR 変換器（統合エントリーポイント）
///
/// Phase 32 L-2.2 Step-3: テストから呼び出し可能に `pub(crate)` 化
#[cfg_attr(not(test), allow(dead_code))]
pub(crate) fn convert_joinir_to_mir(
    join_module: &JoinModule,
) -> Result<MirModule, JoinIrVmBridgeError> {
    // Phase 190: Delegate to FunctionConverter
    JoinIrFunctionConverter::convert_joinir_to_mir(join_module)
}

/// MirLikeInst → MirInstruction 変換
/// Phase 190: 共有ユーティリティとして pub(crate) に変更
pub(crate) fn convert_mir_like_inst(
    mir_like: &MirLikeInst,
) -> Result<MirInstruction, JoinIrVmBridgeError> {
    match mir_like {
        MirLikeInst::Const { dst, value } => {
            let mir_const = match value {
                ConstValue::Integer(i) => MirConstValue::Integer(*i),
                ConstValue::Bool(b) => MirConstValue::Bool(*b),
                ConstValue::String(s) => MirConstValue::String(s.clone()),
                ConstValue::Null => MirConstValue::Null,
            };
            Ok(MirInstruction::Const {
                dst: *dst,
                value: mir_const,
            })
        }
        MirLikeInst::BinOp { dst, op, lhs, rhs } => {
            let mir_op = match op {
                BinOpKind::Add => BinaryOp::Add,
                BinOpKind::Sub => BinaryOp::Sub,
                BinOpKind::Mul => BinaryOp::Mul,
                BinOpKind::Div => BinaryOp::Div,
                BinOpKind::Mod => BinaryOp::Mod, // Phase 188-Impl-3
                BinOpKind::Or => BinaryOp::Or,
                BinOpKind::And => BinaryOp::And,
            };
            Ok(MirInstruction::BinOp {
                dst: *dst,
                op: mir_op,
                lhs: *lhs,
                rhs: *rhs,
            })
        }
        MirLikeInst::Compare { dst, op, lhs, rhs } => {
            let mir_cmp = match op {
                CompareOp::Lt => MirCompareOp::Lt,
                CompareOp::Le => MirCompareOp::Le,
                CompareOp::Gt => MirCompareOp::Gt,
                CompareOp::Ge => MirCompareOp::Ge,
                CompareOp::Eq => MirCompareOp::Eq,
                CompareOp::Ne => MirCompareOp::Ne,
            };
            Ok(MirInstruction::Compare {
                dst: *dst,
                op: mir_cmp,
                lhs: *lhs,
                rhs: *rhs,
            })
        }
        MirLikeInst::BoxCall {
            dst,
            box_name,
            method,
            args,
        } => {
            // Phase 27-shortterm S-4.3: BoxCall → MIR BoxCall
            // box_name は JoinIR で保持しているが、MIR BoxCall には receiver ValueId のみ
            // 暫定: args[0] を receiver として扱う
            if args.is_empty() {
                return Err(JoinIrVmBridgeError::new(format!(
                    "BoxCall requires at least one argument (receiver), got: box_name={}, method={}",
                    box_name, method
                )));
            }

            let receiver = args[0];
            let method_args = args[1..].to_vec();

            Ok(MirInstruction::Call {
                dst: *dst,
                func: crate::mir::ValueId::INVALID,
                callee: Some(crate::mir::Callee::Method {
                    box_name: box_name.clone(),
                    method: method.clone(),
                    receiver: Some(receiver),
                    certainty: crate::mir::definitions::call_unified::TypeCertainty::Known,
                    box_kind: crate::mir::definitions::call_unified::CalleeBoxKind::RuntimeData,
                }),
                args: method_args,
                effects: EffectMask::PURE, // Phase 27-shortterm: assume pure
            })
        }
        // Phase 56: UnaryOp
        MirLikeInst::UnaryOp { dst, op, operand } => {
            let mir_op = match op {
                crate::mir::join_ir::UnaryOp::Not => crate::mir::types::UnaryOp::Not,
                crate::mir::join_ir::UnaryOp::Neg => crate::mir::types::UnaryOp::Neg,
            };
            Ok(MirInstruction::UnaryOp {
                dst: *dst,
                op: mir_op,
                operand: *operand,
            })
        }
        // Phase C7-L3: Do not emit lowered-away Print op from JoinIR bridge.
        MirLikeInst::Print { value } => Ok(build_extern_call(
            None,
            "env.console",
            "log",
            vec![*value],
            EffectMask::PURE.add(Effect::Io),
        )),
        // Phase 188-Impl-3: Select
        // Select is a ternary operator: cond ? then_val : else_val
        // This should not be directly converted to a single MIR instruction
        // Instead, it should be handled by merge_joinir_mir_blocks which creates
        // proper control flow with branches and PHI nodes
        MirLikeInst::Select { .. } => {
            Err(JoinIrVmBridgeError::new(
                "Select instruction should be handled by merge_joinir_mir_blocks, not convert_mir_like_inst".to_string()
            ))
        }
    }
}
