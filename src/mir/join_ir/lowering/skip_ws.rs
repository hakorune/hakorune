//! Phase 27.1: minimal_ssa_skip_ws 専用の MIR → JoinIR 変換
//!
//! 目的: apps/tests/minimal_ssa_skip_ws.hako の MIR を JoinIR に変換する実装
//!
//! 期待される変換:
//! ```text
//! // MIR (元):
//! static box Main {
//!   skip(s) {
//!     local i = 0
//!     local n = s.length()
//!     loop(1 == 1) {
//!       if i >= n { break }
//!       local ch = s.substring(i, i + 1)
//!       if ch == " " { i = i + 1 } else { break }
//!     }
//!     return i
//!   }
//! }
//!
//! // JoinIR (変換後):
//! fn skip(s_param, k_exit) {
//!     i_init = 0
//!     n = s_param.length()
//!     loop_step(s_param, i_init, n, k_exit)
//! }
//!
//! fn loop_step(s, i, n, k_exit) {
//!     if i >= n {
//!         k_exit(i)  // break
//!     } else {
//!         ch = s.substring(i, i + 1)
//!         if ch == " " {
//!             loop_step(s, i + 1, n, k_exit)  // continue
//!         } else {
//!             k_exit(i)  // break
//!         }
//!     }
//! }
//! ```

use crate::mir::join_ir::{
    BinOpKind, CompareOp, ConstValue, JoinContId, JoinFuncId, JoinFunction, JoinInst, JoinModule,
    LoopExitShape, LoopHeaderShape, MirLikeInst,
};
use crate::mir::query::MirQuery;
use crate::mir::ValueId;
use crate::runtime::get_global_ring0;

/// Phase 27.11.1: Common JoinIR builder for Main.skip/1
///
/// This function generates the JoinIR for skip/1, shared by both:
/// - lower_skip_ws_handwritten (always uses this)
/// - lower_skip_ws_from_mir (uses this after CFG sanity checks pass)
fn build_skip_ws_joinir(module: &crate::mir::MirModule) -> Option<JoinModule> {
    // Step 1: "Main.skip/1" を探す
    let target_func = module.functions.get("Main.skip/1")?;

    if crate::config::env::joinir_dev::debug_enabled() {
        let ring0 = get_global_ring0();
        ring0.log.debug("[joinir/skip_ws/build] Found Main.skip/1");
        ring0.log.debug(&format!(
            "[joinir/skip_ws/build] MIR blocks: {}",
            target_func.blocks.len()
        ));
    }

    // Step 2: JoinModule を構築
    let mut join_module = JoinModule::new();

    // Phase 27.1: 固定的な JoinIR を生成（実際の MIR 解析は Phase 28 以降）

    // skip 関数: i_init = 0, n = s.length(), loop_step(s, 0, n, k_exit)
    let skip_id = JoinFuncId::new(0);
    let s_param = ValueId(3000);
    let mut skip_func = JoinFunction::new(skip_id, "skip".to_string(), vec![s_param]);

    let i_init = ValueId(3001);
    let n = ValueId(3002);

    // i_init = 0
    skip_func.body.push(JoinInst::Compute(MirLikeInst::Const {
        dst: i_init,
        value: ConstValue::Integer(0),
    }));

    // n = s.length() (BoxCall でメソッド呼び出し)
    skip_func.body.push(JoinInst::Compute(MirLikeInst::BoxCall {
        dst: Some(n),
        box_name: "StringBox".to_string(),
        method: "length".to_string(),
        args: vec![s_param],
    }));

    // loop_step(s, i_init, n, k_exit)
    let loop_step_id = JoinFuncId::new(1);
    skip_func.body.push(JoinInst::Call {
        func: loop_step_id,
        args: vec![s_param, i_init, n],
        k_next: None,
        dst: None,
    });

    join_module.entry = Some(skip_id);
    join_module.add_function(skip_func);

    // Phase 27.4-A: loop_step の Pinned/Carrier 構造を明示
    // skip_ws ループの場合:
    //   - Pinned: s (文字列), n (長さ) - ループ中で不変
    //   - Carrier: i (現在位置) - ループで更新される
    let s_loop = ValueId(4000); // Pinned
    let i_loop = ValueId(4001); // Carrier
    let n_loop = ValueId(4002); // Pinned

    let _header_shape = LoopHeaderShape::new_manual(
        vec![s_loop, n_loop], // Pinned: s, n
        vec![i_loop],         // Carrier: i
    );
    // 将来: LoopHeaderShape.to_loop_step_params() は [pinned..., carriers...] の順を返す。
    // 現在は既存 JoinIR テストとの互換性のため、手動で [s, i, n] の順を維持している。

    // loop_step 関数: if i >= n { return i } else if ch == " " { loop_step(i + 1) } else { return i }
    let mut loop_step_func = JoinFunction::new(
        loop_step_id,
        "loop_step".to_string(),
        vec![s_loop, i_loop, n_loop], // [pinned, carrier, pinned] の順（現行実装）
    );

    let cmp1_result = ValueId(4003);
    let ch = ValueId(4004);
    let cmp2_result = ValueId(4005);
    let i_plus_1 = ValueId(4006);
    let const_1 = ValueId(4007);
    let const_space = ValueId(4010);
    let bool_false = ValueId(4011);
    let cmp2_is_false = ValueId(4012);

    // cmp1_result = (i >= n)
    loop_step_func
        .body
        .push(JoinInst::Compute(MirLikeInst::Compare {
            dst: cmp1_result,
            op: CompareOp::Ge,
            lhs: i_loop,
            rhs: n_loop,
        }));

    // Phase 27.5: Exit φ の意味を LoopExitShape で明示
    // skip_ws のループ脱出時は i の値だけを返す（先頭空白の文字数）
    let _exit_shape = LoopExitShape::new_manual(vec![i_loop]); // exit_args = [i]

    // if i >= n { return i }
    loop_step_func.body.push(JoinInst::Jump {
        cont: JoinContId::new(0),
        args: vec![i_loop], // ← LoopExitShape.exit_args に対応
        cond: Some(cmp1_result),
    });

    // const 1
    loop_step_func
        .body
        .push(JoinInst::Compute(MirLikeInst::Const {
            dst: const_1,
            value: ConstValue::Integer(1),
        }));

    // i_plus_1 = i + 1 (再利用: substring end / continue path)
    loop_step_func
        .body
        .push(JoinInst::Compute(MirLikeInst::BinOp {
            dst: i_plus_1,
            op: BinOpKind::Add,
            lhs: i_loop,
            rhs: const_1,
        }));

    // ch = s.substring(i, i + 1)
    loop_step_func
        .body
        .push(JoinInst::Compute(MirLikeInst::BoxCall {
            dst: Some(ch),
            box_name: "StringBox".to_string(),
            method: "substring".to_string(),
            args: vec![s_loop, i_loop, i_plus_1],
        }));

    // const " " (space)
    loop_step_func
        .body
        .push(JoinInst::Compute(MirLikeInst::Const {
            dst: const_space,
            value: ConstValue::String(" ".to_string()),
        }));

    // cmp2_result = (ch == " ")
    loop_step_func
        .body
        .push(JoinInst::Compute(MirLikeInst::Compare {
            dst: cmp2_result,
            op: CompareOp::Eq,
            lhs: ch,
            rhs: const_space,
        }));

    // bool false (for negation)
    loop_step_func
        .body
        .push(JoinInst::Compute(MirLikeInst::Const {
            dst: bool_false,
            value: ConstValue::Bool(false),
        }));

    // cmp2_is_false = (cmp2_result == false)
    loop_step_func
        .body
        .push(JoinInst::Compute(MirLikeInst::Compare {
            dst: cmp2_is_false,
            op: CompareOp::Eq,
            lhs: cmp2_result,
            rhs: bool_false,
        }));

    // Phase 27.5: 2箇所目の exit パス（同じく exit_args = [i]）
    // if ch != " " { return i }
    loop_step_func.body.push(JoinInst::Jump {
        cont: JoinContId::new(1),
        args: vec![i_loop], // ← LoopExitShape.exit_args に対応（1箇所目と同じ）
        cond: Some(cmp2_is_false),
    });

    // continue path: loop_step(s, i + 1, n)
    loop_step_func.body.push(JoinInst::Call {
        func: loop_step_id,
        args: vec![s_loop, i_plus_1, n_loop],
        k_next: None,
        dst: None,
    });

    join_module.add_function(loop_step_func);

    if crate::config::env::joinir_dev::debug_enabled() {
        get_global_ring0().log.debug(&format!(
            "[joinir/skip_ws] Generated {} JoinIR functions",
            join_module.functions.len()
        ));
    }

    Some(join_module)
}

/// Phase 27.8: Main.skip/1 の JoinIR lowering（MIR 自動解析版）
///
/// MIR 構造を解析して自動的に JoinIR を生成する実装。
/// Phase 27.8 で導入、将来的に hand-written 版を置き換える予定。
///
/// ## 環境変数:
/// - `NYASH_JOINIR_LOWER_FROM_MIR=1`: この実装を有効化
///
/// ## 実装状況:
/// - Phase 27.8: 基本実装（MirQuery を使用した MIR 解析）
fn lower_skip_ws_from_mir(module: &crate::mir::MirModule) -> Option<JoinModule> {
    use super::common::{ensure_entry_has_succs, has_const_int, has_string_method, log_fallback};
    use crate::mir::query::MirQueryBox;

    // Step 1: "Main.skip/1" を探す
    let target_func = module.functions.get("Main.skip/1")?;

    if crate::config::env::joinir_dev::debug_enabled() {
        let ring0 = get_global_ring0();
        ring0
            .log
            .debug("[joinir/skip_ws/mir] Found Main.skip/1 (MIR-based lowering)");
        ring0.log.debug(&format!(
            "[joinir/skip_ws/mir] MIR blocks: {}",
            target_func.blocks.len()
        ));
    }

    // NOTE:
    // このフェーズでは minimal_ssa_skip_ws.hako 固定のパターンを前提に、
    // MIR の CFG を軽く確認した上で JoinIR を組み立てる。
    // （完全一般化は次フェーズ以降で行う）

    // 簡易チェック: ブロック数が最低限あるか確認
    if target_func.blocks.len() < 3 {
        log_fallback(
            "skip_ws",
            &format!("insufficient blocks ({})", target_func.blocks.len()),
        );
        return lower_skip_ws_handwritten(module);
    }

    // Phase 27.10: Lightweight CFG sanity checks using common utilities
    let query = MirQueryBox::new(target_func);
    let entry_id = target_func.entry_block;

    // Check 1: Entry block has at least 1 successor
    if !ensure_entry_has_succs(&query, entry_id) {
        log_fallback("skip_ws", "entry has no successors");
        return lower_skip_ws_handwritten(module);
    }

    // Check 2: Entry block contains Const(0) and BoxCall(String.length)
    if !has_const_int(&query, entry_id, 0) || !has_string_method(&query, entry_id, "length") {
        log_fallback("skip_ws", "entry block missing Const(0) or String.length");
        return lower_skip_ws_handwritten(module);
    }

    if crate::config::env::joinir_dev::debug_enabled() {
        get_global_ring0()
            .log
            .debug("[joinir/skip_ws/mir] CFG sanity checks passed ✅");
    }

    // Phase 27.11.1: Generate JoinIR using shared builder
    // CFG checks passed, so we can use build_skip_ws_joinir() directly
    if crate::config::env::joinir_dev::debug_enabled() {
        get_global_ring0()
            .log
            .debug("[joinir/skip_ws/mir] Calling build_skip_ws_joinir() after CFG validation");
    }
    return build_skip_ws_joinir(module);
}

/// Phase 27.11.1: Handwritten lowering wrapper for Main.skip/1
///
/// This is a thin wrapper that calls the shared build_skip_ws_joinir() function.
/// Maintains the handwritten lowering path as the baseline reference.
fn lower_skip_ws_handwritten(module: &crate::mir::MirModule) -> Option<JoinModule> {
    if crate::config::env::joinir_dev::debug_enabled() {
        get_global_ring0()
            .log
            .debug("[joinir/skip_ws/handwritten] Using handwritten lowering path");
    }
    build_skip_ws_joinir(module)
}

/// Phase 27.8: Main.skip/1 の JoinIR lowering（トグル対応ディスパッチャー）
///
/// 環境変数 `NYASH_JOINIR_LOWER_FROM_MIR=1` に応じて、
/// hand-written 版または MIR 自動解析版を選択する。
///
/// ## トグル制御:
/// - **OFF (デフォルト)**: `lower_skip_ws_handwritten()` を使用
/// - **ON**: `lower_skip_ws_from_mir()` を使用
///
/// ## 使用例:
/// ```bash
/// # 手書き版（既定）
/// ./target/release/hakorune program.hako
///
/// # MIR 自動解析版
/// NYASH_JOINIR_LOWER_FROM_MIR=1 ./target/release/hakorune program.hako
/// ```
pub fn lower_skip_ws_to_joinir(module: &crate::mir::MirModule) -> Option<JoinModule> {
    // Phase 28: Generic Case A トグル（minimal_ssa_skip_ws 限定）
    if crate::mir::join_ir::env_flag_is_1("NYASH_JOINIR_LOWER_GENERIC") {
        if let Some(jm) = try_lower_skip_ws_generic_case_a(module) {
            return Some(jm);
        }
        if crate::config::env::joinir_dev::debug_enabled() {
            get_global_ring0()
                .log
                .debug("[joinir/skip_ws] generic_case_a fallback → existing dispatcher");
        }
    }

    lower_skip_ws_handwritten_or_mir(module)
}

/// 既存の hand-written / MIR-based dispatcher をラップしただけの関数
fn lower_skip_ws_handwritten_or_mir(module: &crate::mir::MirModule) -> Option<JoinModule> {
    super::common::dispatch_lowering(
        "skip_ws",
        module,
        lower_skip_ws_from_mir,
        lower_skip_ws_handwritten,
    )
}

/// トグル ON 時にだけ試す generic Case A ロワー（minimal_ssa_skip_ws 限定）
///
/// Phase 31: LoopToJoinLowerer 統一箱経由に移行
fn try_lower_skip_ws_generic_case_a(module: &crate::mir::MirModule) -> Option<JoinModule> {
    use crate::mir::join_ir::lowering::loop_to_join::LoopToJoinLowerer;
    use crate::mir::loop_form::LoopForm;
    use crate::mir::query::MirQueryBox;

    let target_func = module.functions.get("Main.skip/1")?;
    let query = MirQueryBox::new(target_func);

    // 最小限の LoopForm 形状推定（Case A/constant-true ループ想定）
    let preheader = target_func.entry_block;
    let header = query.succs(preheader).get(0).copied().unwrap_or(preheader);
    let succs_header = query.succs(header);
    let body = succs_header.get(0).copied().unwrap_or(header);
    let exit = succs_header.get(1).copied().unwrap_or(header);
    let latch = body;

    let loop_form = LoopForm {
        preheader,
        header,
        body,
        latch,
        exit,
        continue_targets: vec![body],
        break_targets: vec![exit],
    };

    // Phase 31: LoopToJoinLowerer 経由で JoinModule 生成
    let lowerer = LoopToJoinLowerer::new();
    lowerer.lower_case_a_for_skip_ws(target_func, &loop_form)
}
