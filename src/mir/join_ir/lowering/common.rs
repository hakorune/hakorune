//! Phase 27.10: Common utilities for JoinIR lowering
//!
//! CFG sanity checks and dispatcher helpers for MIR-based lowering.

pub mod balanced_depth_scan_emitter; // Phase 107: Balanced depth-scan (find_balanced_* recipe)
pub mod body_local_derived_emitter; // Phase 94: Derived body-local (P5b escape "ch" reassignment)
pub mod body_local_derived_slot_emitter; // Phase 29ab P4: Derived slot for seg
pub mod case_a;
pub mod condition_only_emitter; // Phase 93 P0: ConditionOnly (Derived Slot) recalculation
pub mod conditional_step_emitter; // Phase 92 P1-1: ConditionalStep emission module
pub mod dual_value_rewriter; // Phase 246-EX/247-EX: name-based dual-value rewrites

use crate::mir::loop_form::LoopForm;
use crate::mir::query::{MirQuery, MirQueryBox};
use crate::mir::{BasicBlockId, BinaryOp, ConstValue, MirInstruction};
use crate::runtime::get_global_ring0;

// ============================================================================
// Phase 32: LoopForm construction helpers
// ============================================================================

/// 単純な while ループの LoopForm を CFG から構築する
///
/// # Arguments
/// - `entry`: 関数エントリブロック
/// - `query`: MIR クエリ
/// - `entry_is_preheader`: true なら entry を preheader として使う（trim 用）
///                          false なら entry の succ を preheader とする（stage1 用）
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

/// Check if entry block has at least one successor
///
/// Returns `true` if the entry block has at least one successor, `false` otherwise.
/// This is a basic sanity check to ensure the MIR CFG is well-formed.
pub fn ensure_entry_has_succs(query: &MirQueryBox, entry: BasicBlockId) -> bool {
    !query.succs(entry).is_empty()
}

/// Check if a basic block contains `Const { value: Integer(value) }`
///
/// Returns `true` if the block contains a constant integer instruction with the specified value.
///
/// # Example
/// ```ignore
/// // Check if entry block contains Const(0)
/// if has_const_int(&query, entry_id, 0) {
///     // ...
/// }
/// ```
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

/// Check if a basic block contains `Const { value: String(value) }`
///
/// Returns `true` if the block contains a constant string instruction with the specified value.
///
/// # Example
/// ```ignore
/// // Check if entry block contains Const("")
/// if has_const_string(&query, entry_id, "") {
///     // ...
/// }
/// ```
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

/// Check if a basic block contains method call `Call { callee: Method { method } }`
///
/// Returns `true` if the block contains a method call instruction with the specified method name.
///
/// # Example
/// ```ignore
/// // Check if entry block contains String.length()
/// if has_string_method(&query, entry_id, "length") {
///     // ...
/// }
/// ```
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

/// Check if a basic block contains `BinOp { op: operation }`
///
/// Returns `true` if the block contains a binary operation instruction with the specified operation.
///
/// # Example
/// ```ignore
/// // Check if entry block contains BinOp(Add)
/// if has_binop(&query, entry_id, BinaryOp::Add) {
///     // ...
/// }
/// ```
pub fn has_binop(query: &MirQueryBox, bb: BasicBlockId, op: BinaryOp) -> bool {
    query.insts_in_block(bb).iter().any(|inst| {
        matches!(
            inst,
            MirInstruction::BinOp { op: o, .. } if *o == op
        )
    })
}

/// Check if a basic block contains method call `Call { callee: Method { method } }`
///
/// Returns `true` if the block contains a method call instruction with the specified method name.
/// Note: This cannot distinguish between different box types (e.g., ArrayBox.get vs StringBox.get)
/// when callers use runtime-dynamic callee metadata.
///
/// For more precise type checking, use TypeRegistry (future enhancement).
///
/// # Example
/// ```ignore
/// // Check if entry block contains ArrayBox.length()
/// // (Note: will also match StringBox.length() if it exists)
/// if has_array_method(&query, entry_id, "length") {
///     // ...
/// }
/// ```
pub fn has_array_method(query: &MirQueryBox, bb: BasicBlockId, method: &str) -> bool {
    // Note: This is intentionally the same as has_string_method().
    // Future enhancement: Use TypeRegistry to check box_val's type
    has_string_method(query, bb, method)
}

/// Check if a basic block contains a loop increment pattern (`i + 1`)
///
/// Returns `true` if the block contains a `BinOp::Add` instruction.
/// This is a convenience wrapper for `has_binop(query, bb, BinaryOp::Add)`.
///
/// # Example
/// ```ignore
/// // Check if loop body contains i + 1
/// if has_loop_increment(&query, loop_body_id) {
///     // ...
/// }
/// ```
pub fn has_loop_increment(query: &MirQueryBox, bb: BasicBlockId) -> bool {
    has_binop(query, bb, BinaryOp::Add)
}

/// Log fallback to handwritten lowering with reason
///
/// Prints a diagnostic message when MIR-based lowering falls back to handwritten version.
///
/// # Example
/// ```ignore
/// log_fallback("skip_ws", "entry has no successors");
/// // Output: [joinir/skip_ws/mir] unexpected MIR shape: entry has no successors, falling back to handwritten
/// ```
pub fn log_fallback(tag: &str, reason: &str) {
    if crate::config::env::joinir_dev::debug_enabled() {
        get_global_ring0().log.debug(&format!(
            "[joinir/{}/mir] unexpected MIR shape: {}, falling back to handwritten",
            tag, reason
        ));
    }
}

/// Dispatch between MIR-based and handwritten lowering based on environment variable
///
/// Checks `NYASH_JOINIR_LOWER_FROM_MIR` and dispatches to the appropriate lowering function.
/// This consolidates the toggle pattern used across all JoinIR lowering implementations.
///
/// # Arguments
/// - `tag`: Identifier for logging (e.g., "skip_ws", "trim")
/// - `module`: MIR module to lower
/// - `mir_based`: Closure for MIR-based lowering (returns `Option<JoinModule>`)
/// - `handwritten`: Closure for handwritten lowering (returns `Option<JoinModule>`)
///
/// # Returns
/// `Option<JoinModule>` - Both implementations may return None on errors
///
/// # Example
/// ```ignore
/// pub fn lower_skip_ws_to_joinir(module: &MirModule) -> Option<JoinModule> {
///     dispatch_lowering(
///         "skip_ws",
///         module,
///         lower_skip_ws_from_mir,
///         lower_skip_ws_handwritten,
///     )
/// }
/// ```
pub fn dispatch_lowering<F1, F2>(
    tag: &str,
    module: &crate::mir::MirModule,
    mir_based: F1,
    handwritten: F2,
) -> Option<crate::mir::join_ir::JoinModule>
where
    F1: FnOnce(&crate::mir::MirModule) -> Option<crate::mir::join_ir::JoinModule>,
    F2: FnOnce(&crate::mir::MirModule) -> Option<crate::mir::join_ir::JoinModule>,
{
    if crate::config::env::joinir_dev::lower_from_mir_enabled() {
        if crate::config::env::joinir_dev::debug_enabled() {
            get_global_ring0().log.debug(&format!(
                "[joinir/{}] Using MIR-based lowering (NYASH_JOINIR_LOWER_FROM_MIR=1)",
                tag
            ));
        }
        mir_based(module)
    } else {
        if crate::config::env::joinir_dev::debug_enabled() {
            get_global_ring0().log.debug(&format!(
                "[joinir/{}] Using handwritten lowering (default)",
                tag
            ));
        }
        handwritten(module)
    }
}

// ============================================================================
// Phase 185: Type inference utilities (shared by if_select / if_merge)
// ============================================================================

use crate::mir::{MirFunction, MirType, ValueId};

/// Phase 185: MIR から ValueId の型を推論（共通化）
///
/// Const 命令を探して、ValueId に対応する MirType を返す。
/// Select/IfMerge の then_val / else_val から型ヒントを埋めるために使用。
///
/// # Usage
/// - `if_select.rs`: Select の型ヒント埋め込み
/// - `if_merge.rs`: IfMerge の型ヒント埋め込み
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
