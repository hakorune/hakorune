//! Type hint utilities shared by IfSelect and IfMerge lowering.

use crate::mir::{ConstValue, MirFunction, MirInstruction, MirType, ValueId};

/// Phase 185: MIR から ValueId の型を推論（共通化）
///
/// Const 命令を探して、ValueId に対応する MirType を返す。
/// Select/IfMerge の then_val / else_val から型ヒントを埋めるために使用。
pub fn infer_type_from_mir_pattern(func: &MirFunction, val_id: ValueId) -> Option<MirType> {
    // 全ブロックの全命令を走査して Const 命令を探す
    for block in func.blocks.values() {
        for inst in &block.instructions {
            if let MirInstruction::Const { dst, value } = inst {
                if *dst == val_id {
                    return Some(match value {
                        ConstValue::Integer(_) => MirType::Integer,
                        ConstValue::Bool(_) => MirType::Bool,
                        ConstValue::String(_) => MirType::String,
                        ConstValue::Void => MirType::Void,
                        ConstValue::Null => MirType::Unknown, // Null は Unknown として扱う
                        // Float は現状未サポート
                        _ => return None,
                    });
                }
            }
        }
    }
    None
}
