//! CFG and instruction shape probes for target-specific JoinIR lowerers.

use crate::mir::loop_form::LoopForm;
use crate::mir::query::{MirQuery, MirQueryBox};
use crate::mir::{BasicBlockId, BinaryOp, ConstValue, MirInstruction};

/// 単純な while ループの LoopForm を CFG から構築する
///
/// # Arguments
/// - `entry`: 関数エントリブロック
/// - `query`: MIR クエリ
/// - `entry_is_preheader`: true なら entry を preheader として使う（trim 用）
///                          false なら entry の succ を preheader とする（lower-resolver 用）
/// - `has_break`: true なら exit を break_targets に含める
///
/// # Loop structure assumed
/// ```text
/// [entry] → [preheader] → [header] ─┬→ [body] → [latch] → [header]
///                                   └→ [exit]
/// ```
///
/// Note: latch は body と同じブロックとして扱う（is_simple_case_a_loop 対応）
pub fn construct_simple_while_loopform(
    entry: BasicBlockId,
    query: &MirQueryBox,
    entry_is_preheader: bool,
    has_break: bool,
) -> Option<LoopForm> {
    let preheader = if entry_is_preheader {
        entry
    } else {
        query.succs(entry).get(0).copied()?
    };

    let header = query.succs(preheader).get(0).copied().unwrap_or(preheader);
    let succs_header = query.succs(header);
    let body = succs_header.get(0).copied().unwrap_or(header);
    let exit = succs_header.get(1).copied().unwrap_or(header);

    Some(LoopForm {
        preheader,
        header,
        body,
        latch: body, // is_simple_case_a_loop 対応: latch == body
        exit,
        continue_targets: vec![body],
        break_targets: if has_break { vec![exit] } else { vec![] },
    })
}

/// Check if entry block has at least one successor.
pub fn ensure_entry_has_succs(query: &MirQueryBox, entry: BasicBlockId) -> bool {
    !query.succs(entry).is_empty()
}

/// Check if a basic block contains `Const { value: Integer(value) }`.
pub fn has_const_int(query: &MirQueryBox, bb: BasicBlockId, value: i64) -> bool {
    query.insts_in_block(bb).iter().any(|inst| {
        matches!(
            inst,
            MirInstruction::Const {
                value: ConstValue::Integer(v),
                ..
            } if *v == value
        )
    })
}

/// Check if a basic block contains `Const { value: String(value) }`.
pub fn has_const_string(query: &MirQueryBox, bb: BasicBlockId, value: &str) -> bool {
    query.insts_in_block(bb).iter().any(|inst| {
        matches!(
            inst,
            MirInstruction::Const {
                value: ConstValue::String(s),
                ..
            } if s == value
        )
    })
}

/// Check if a basic block contains method call `Call { callee: Method { method } }`.
pub fn has_string_method(query: &MirQueryBox, bb: BasicBlockId, method: &str) -> bool {
    query.insts_in_block(bb).iter().any(|inst| {
        matches!(
            inst,
            MirInstruction::Call {
                callee: Some(crate::mir::Callee::Method { method: m, .. }),
                ..
            } if m == method
        )
    })
}

/// Check if a basic block contains `BinOp { op: operation }`.
pub fn has_binop(query: &MirQueryBox, bb: BasicBlockId, op: BinaryOp) -> bool {
    query.insts_in_block(bb).iter().any(|inst| {
        matches!(
            inst,
            MirInstruction::BinOp { op: o, .. } if *o == op
        )
    })
}
