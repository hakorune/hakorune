//! BranchEmissionBox — 分岐/ジャンプ命令発行の薄いヘルパ（仕様不変）

use crate::mir::builder::MirBuilder;
use crate::mir::{BasicBlockId, MirInstruction};

#[inline]
pub fn emit_conditional(
    b: &mut MirBuilder,
    cond: crate::mir::ValueId,
    then_bb: BasicBlockId,
    else_bb: BasicBlockId,
) -> Result<(), String> {
    if let (Some(func), Some(cur_bb)) = (b.scope_ctx.current_function.as_mut(), b.current_block) {
        crate::mir::ssot::cf_common::set_branch(func, cur_bb, cond, then_bb, else_bb);
        Ok(())
    } else {
        b.emit_instruction(MirInstruction::Branch {
            condition: cond,
            then_bb,
            else_bb,
            then_edge_args: None,
            else_edge_args: None,
        })
    }
}

#[inline]
pub fn emit_jump(b: &mut MirBuilder, target: BasicBlockId) -> Result<(), String> {
    if let (Some(func), Some(cur_bb)) = (b.scope_ctx.current_function.as_mut(), b.current_block) {
        crate::mir::ssot::cf_common::set_jump(func, cur_bb, target);
        Ok(())
    } else {
        b.emit_instruction(MirInstruction::Jump {
            target,
            edge_args: None,
        })
    }
}

/// Phase 268 P0: EdgeCFG Fragment ベースの if 条件分岐 emit
///
/// # 責務
/// - Frag 構築（then/else/join の3断片）
/// - compose::if_() で合成
/// - emit_frag() で MIR terminator に変換
///
/// # 引数
/// - `b`: MirBuilder への可変参照
/// - `pre_branch_bb`: 分岐前のヘッダーブロック
/// - `condition_val`: 分岐条件（ValueId）
/// - `then_block`: then 側の entry ブロック
/// - `then_exit_block`: then 側の exit ブロック（merge への飛び元）
/// - `then_reaches_merge`: then が merge に到達するか
/// - `else_block`: else 側の entry ブロック
/// - `else_exit_block`: else 側の exit ブロック（merge への飛び元）
/// - `else_reaches_merge`: else が merge に到達するか
/// - `merge_block`: merge ブロック
///
/// # 注意
/// - then_exit_block/else_exit_block は「実際に merge へ飛ぶブロック」と一致必須
/// - P0 では edge-args は空（CarriersOnly, values=[]）
///
/// # Phase 268 アーキテクチャ
/// ```text
/// if_form.rs (MirBuilder 層)
///   ↓ 呼び出し
/// emission/branch.rs::emit_conditional_edgecfg() (emission 層: 薄ラッパー)
///   ↓ 内部で使用
/// Frag 構築 + compose::if_() + emit_frag() (EdgeCFG Fragment API)
///   ↓ 最終的に呼び出し
/// set_branch_with_edge_args() / set_jump_with_edge_args() (Phase 260 SSOT)
/// ```
pub fn emit_conditional_edgecfg(
    b: &mut MirBuilder,
    pre_branch_bb: BasicBlockId,
    condition_val: crate::mir::ValueId,
    then_block: BasicBlockId,
    then_exit_block: BasicBlockId,
    then_reaches_merge: bool,
    else_block: BasicBlockId,
    else_exit_block: BasicBlockId,
    else_reaches_merge: bool,
    merge_block: BasicBlockId,
) -> Result<(), String> {
    use crate::mir::builder::control_flow::edgecfg::api::{
        compose, EdgeStub, ExitKind, Frag,
    };

    // Then Frag 構築（from = then_exit_block: 実際の merge 飛び元）
    let then_frag = if then_reaches_merge {
        let stub = EdgeStub::without_args(then_exit_block, ExitKind::Normal);
        Frag::with_single_exit(then_block, stub)
    } else {
        // Early return: no Normal exit
        Frag::new(then_block)
    };

    // Else Frag 構築（from = else_exit_block: 実際の merge 飛び元）
    let else_frag = if else_reaches_merge {
        let stub = EdgeStub::without_args(else_exit_block, ExitKind::Normal);
        Frag::with_single_exit(else_block, stub)
    } else {
        // Early return: no Normal exit
        Frag::new(else_block)
    };

    // Join Frag 構築
    let join_frag = Frag::new(merge_block);

    // Compose if_ (Phase 268 P1: entry edge-args from caller)
    let if_frag = compose::if_(
        pre_branch_bb,
        condition_val,
        then_frag,
        crate::mir::basic_block::EdgeArgs {
            layout: crate::mir::join_ir::lowering::inline_boundary::JumpArgsLayout::CarriersOnly,
            values: vec![],
        },
        else_frag,
        crate::mir::basic_block::EdgeArgs {
            layout: crate::mir::join_ir::lowering::inline_boundary::JumpArgsLayout::CarriersOnly,
            values: vec![],
        },
        join_frag,
    );

    // Emit to MIR (Phase 29bq+: session 経由で sealing enforce)
    if let Some(ref mut func) = b.scope_ctx.current_function {
        b.frag_emit_session.emit_and_seal(func, &if_frag)?;
    } else {
        return Err("[emit_conditional_edgecfg] current_function is None".to_string());
    }

    Ok(())
}
