//! Phase 27.14: FuncScannerBox._append_defs loop の JoinIR lowering
//!
//! 目的: FuncScanner の最も簡単な配列結合ループを JoinIR に変換
//!
//! ## 対象ループ
//! - ファイル: `lang/src/compiler/entry/func_scanner.hako`
//! - 関数: `FuncScannerBox._append_defs(dst, defs_box)`
//! - 行数: 293-300
//!
//! ## ループ構造
//! ```hako
//! method _append_defs(dst, defs_box) {
//!     if defs_box == null { return }
//!     local i = 0
//!     loop(i < defs_box.length()) {
//!         dst.push(defs_box.get(i))
//!         i = i + 1
//!     }
//! }
//! ```
//!
//! ## LoopForm ケース: Case A (動的条件 `i < defs_box.length()`)
//!
//! ## Pinned / Carrier / Exit
//! - **Pinned**: `dst` (ArrayBox), `defs_box` (ArrayBox), `n` (Integer = defs_box.length())
//! - **Carrier**: `i` (Integer)
//! - **Exit**: none (void return, dst は破壊的変更)
//!
//! ## 想定 JoinIR 構造
//! ```text
//! fn append_defs_entry(dst, defs_box, n) -> void {
//!     let i_init = 0;
//!     loop_step(dst, defs_box, n, i_init)
//! }
//!
//! fn loop_step(dst, defs_box, n, i) -> void {
//!     if i >= n { return }
//!     let item = defs_box.get(i)
//!     dst.push(item)
//!     let next_i = i + 1
//!     loop_step(dst, defs_box, n, next_i)
//! }
//! ```

use crate::mir::join_ir::lowering::common::{
    dispatch_lowering, ensure_entry_has_succs, has_array_method, has_const_int, has_loop_increment,
    log_fallback,
};
use crate::mir::join_ir::lowering::value_id_ranges::funcscanner_append_defs as vid;
use crate::mir::join_ir::JoinModule;
use crate::mir::loop_form::LoopForm;
use crate::mir::query::{MirQuery, MirQueryBox};
use crate::runtime::get_global_ring0;

/// Phase 27.14: FuncScannerBox._append_defs の JoinIR lowering（public dispatcher）
///
/// 環境変数 `NYASH_JOINIR_LOWER_FROM_MIR=1` に応じて、
/// MIR-based 版または handwritten 版を選択する。
///
/// ## トグル制御:
/// - **OFF (デフォルト)**: `lower_handwritten()` を使用
/// - **ON**: `lower_from_mir()` を使用
///
/// ## Shared Builder Pattern
/// 両方の実装が `build_funcscanner_append_defs_joinir()` を呼び出す共通パターン。
pub fn lower_funcscanner_append_defs_to_joinir(
    module: &crate::mir::MirModule,
) -> Option<JoinModule> {
    dispatch_lowering(
        "funcscanner_append_defs",
        module,
        lower_from_mir,
        lower_handwritten,
    )
}

/// Phase 27.14: Common JoinIR builder for FuncScannerBox._append_defs
///
/// This function generates the JoinIR for the append_defs loop, shared by both:
/// - lower_handwritten (always uses this)
/// - lower_from_mir (uses this after CFG sanity checks pass)
///
/// ## 簡略化方針
/// Phase 27.14 の最小実装として、最も単純な JoinIR を生成する：
/// - ArrayBox.length() → dst.push(item) の基本パターン
/// - null チェックは省略（MIR 側で既に処理済み前提）
fn build_funcscanner_append_defs_joinir(module: &crate::mir::MirModule) -> Option<JoinModule> {
    use crate::mir::join_ir::*;

    // Phase 27.14: ターゲット関数が存在するかチェック
    let _target_func = module.functions.get("FuncScannerBox._append_defs/2")?;

    if crate::config::env::joinir_dev::debug_enabled() {
        let ring0 = get_global_ring0();
        ring0
            .log
            .debug("[joinir/funcscanner_append_defs/build] Phase 27.14 implementation");
        ring0
            .log
            .debug("[joinir/funcscanner_append_defs/build] Generating JoinIR for _append_defs loop");
        ring0.log.debug(
            "[joinir/funcscanner_append_defs/build] Using ValueId range: 9000-10999 (via value_id_ranges)",
        );
    }

    // Step 1: JoinModule を構築
    let mut join_module = JoinModule::new();

    // append_defs_entry 関数（entry）:
    // fn append_defs_entry(dst, defs_box, n) -> void {
    //     let i_init = 0;
    //     loop_step(dst, defs_box, n, i_init)
    // }
    let entry_id = JoinFuncId::new(0);
    let dst_param = vid::entry(0); // 9000
    let defs_box_param = vid::entry(1); // 9001
    let n_param = vid::entry(2); // 9002

    let mut entry_func = JoinFunction::new(
        entry_id,
        "append_defs_entry".to_string(),
        vec![dst_param, defs_box_param, n_param],
    );

    let i_init = vid::entry(10); // 9010

    // i_init = 0
    entry_func.body.push(JoinInst::Compute(MirLikeInst::Const {
        dst: i_init,
        value: ConstValue::Integer(0),
    }));

    // loop_step(dst, defs_box, n, i_init)
    let loop_step_id = JoinFuncId::new(1);
    entry_func.body.push(JoinInst::Call {
        func: loop_step_id,
        args: vec![dst_param, defs_box_param, n_param, i_init],
        k_next: None,
        dst: None,
    });

    join_module.entry = Some(entry_id);
    join_module.add_function(entry_func);

    // Phase 27.14: loop_step の Pinned/Carrier 構造を明示
    // FuncScanner _append_defs ループの場合:
    //   - Pinned: dst (ArrayBox), defs_box (ArrayBox), n (Integer)
    //   - Carrier: i (Integer)
    //   - Exit: none (void return)
    let dst_loop = vid::loop_step(0); // 10000 - Pinned
    let defs_box_loop = vid::loop_step(1); // 10001 - Pinned
    let n_loop = vid::loop_step(2); // 10002 - Pinned
    let i_loop = vid::loop_step(3); // 10003 - Carrier

    let _header_shape = LoopHeaderShape::new_manual(
        vec![dst_loop, defs_box_loop, n_loop], // Pinned
        vec![i_loop],                          // Carrier
    );

    // loop_step 関数:
    // fn loop_step(dst, defs_box, n, i) -> void {
    //     if i >= n { return }
    //     let item = defs_box.get(i)
    //     dst.push(item)
    //     let next_i = i + 1
    //     loop_step(dst, defs_box, n, next_i)
    // }
    let mut loop_step_func = JoinFunction::new(
        loop_step_id,
        "loop_step".to_string(),
        vec![dst_loop, defs_box_loop, n_loop, i_loop],
    );

    let cmp_result = vid::loop_step(10); // 10010
    let item_value = vid::loop_step(11); // 10011
    let next_i = vid::loop_step(12); // 10012
    let const_1 = vid::loop_step(13); // 10013

    // cmp_result = (i >= n)
    loop_step_func
        .body
        .push(JoinInst::Compute(MirLikeInst::Compare {
            dst: cmp_result,
            op: CompareOp::Ge,
            lhs: i_loop,
            rhs: n_loop,
        }));

    // Phase 27.14: Exit φ の意味を LoopExitShape で明示
    // FuncScanner _append_defs ループ脱出時は void 返却（dst は破壊的変更済み）
    let _exit_shape = LoopExitShape::new_manual(vec![]); // exit_args = [] (void)

    // if i >= n { return } (void)
    loop_step_func.body.push(JoinInst::Jump {
        cont: JoinContId::new(0),
        args: vec![], // ← LoopExitShape.exit_args に対応 (void)
        cond: Some(cmp_result),
    });

    // item = defs_box.get(i)
    loop_step_func
        .body
        .push(JoinInst::Compute(MirLikeInst::BoxCall {
            dst: Some(item_value),
            box_name: "ArrayBox".to_string(),
            method: "get".to_string(),
            args: vec![defs_box_loop, i_loop],
        }));

    // dst.push(item) - 破壊的変更（戻り値なし）
    loop_step_func
        .body
        .push(JoinInst::Compute(MirLikeInst::BoxCall {
            dst: None, // push は戻り値なし
            box_name: "ArrayBox".to_string(),
            method: "push".to_string(),
            args: vec![dst_loop, item_value],
        }));

    // const_1 = 1
    loop_step_func
        .body
        .push(JoinInst::Compute(MirLikeInst::Const {
            dst: const_1,
            value: ConstValue::Integer(1),
        }));

    // next_i = i + 1
    loop_step_func
        .body
        .push(JoinInst::Compute(MirLikeInst::BinOp {
            dst: next_i,
            op: BinOpKind::Add,
            lhs: i_loop,
            rhs: const_1,
        }));

    // loop_step(dst, defs_box, n, next_i) - tail recursion
    loop_step_func.body.push(JoinInst::Call {
        func: loop_step_id,
        args: vec![dst_loop, defs_box_loop, n_loop, next_i],
        k_next: None,
        dst: None,
    });

    join_module.add_function(loop_step_func);

    if crate::config::env::joinir_dev::debug_enabled() {
        let ring0 = get_global_ring0();
        ring0
            .log
            .debug("[joinir/funcscanner_append_defs/build] ✅ JoinIR construction completed");
        ring0.log.debug(&format!(
            "[joinir/funcscanner_append_defs/build] Functions: {}",
            join_module.functions.len()
        ));
    }

    Some(join_module)
}

/// Phase 27.14: MIR-based lowering for FuncScannerBox._append_defs
///
/// CFG sanity checks + MIR パターンマッチング → 成功なら `build_funcscanner_append_defs_joinir()` 呼び出し
///
/// ## CFG Sanity Checks (軽量パターンマッチ):
/// 1. Entry block に後続がある
/// 2. Entry block 付近に以下の命令がある:
///    - `Const { value: Integer(0) }` (初期 i = 0)
///    - `BoxCall { box_name: "ArrayBox", method: "length" }` (n = defs_box.length())
/// 3. ループ本体付近に:
///    - `BoxCall { box_name: "ArrayBox", method: "get" }` (defs_box.get(i))
///    - `BoxCall { box_name: "ArrayBox", method: "push" }` (dst.push(item))
///    - `BinOp { op: Add }` (next_i = i + 1)
///
/// ## Graceful Degradation
/// 上記パターンが検出できない場合は `log_fallback()` → `lower_handwritten()` に戻る。
fn lower_from_mir(module: &crate::mir::MirModule) -> Option<JoinModule> {
    if crate::config::env::joinir_dev::debug_enabled() {
        get_global_ring0()
            .log
            .debug("[joinir/funcscanner_append_defs/mir] Starting MIR-based lowering");
    }

    // Step 1: FuncScannerBox._append_defs/2 を探す
    let target_func = module.functions.get("FuncScannerBox._append_defs/2")?;

    if crate::config::env::joinir_dev::debug_enabled() {
        let ring0 = get_global_ring0();
        ring0
            .log
            .debug("[joinir/funcscanner_append_defs/mir] Found FuncScannerBox._append_defs/2");
        ring0.log.debug(&format!(
            "[joinir/funcscanner_append_defs/mir] MIR blocks: {}",
            target_func.blocks.len()
        ));
    }

    // Step 2: MirQueryBox を作成
    let query = MirQueryBox::new(target_func);
    let entry = target_func.entry_block;

    // CFG Check 1: Entry block has successors
    if !ensure_entry_has_succs(&query, entry) {
        log_fallback("funcscanner_append_defs", "entry block has no successors");
        return lower_handwritten(module);
    }

    // CFG Check 2: Entry block contains expected route-shape signals
    // Signal A: i = 0 (初期化)
    if !has_const_int(&query, entry, 0) {
        log_fallback(
            "funcscanner_append_defs",
            "Const(0) not found in entry block",
        );
        return lower_handwritten(module);
    }

    // Signal B: defs_box.length() の検出
    // Check entry block and its immediate successors for length() call
    let has_length_call = has_array_method(&query, entry, "length")
        || query
            .succs(entry)
            .iter()
            .any(|&succ| has_array_method(&query, succ, "length"));

    if !has_length_call {
        log_fallback(
            "funcscanner_append_defs",
            "ArrayBox.length() not found in entry or successors",
        );
        return lower_handwritten(module);
    }

    // Signal C: ループ本体での配列操作検出
    // Check all blocks for array operations (get/push) and loop increment
    let all_blocks: Vec<_> = target_func.blocks.keys().copied().collect();

    let has_get_call = all_blocks
        .iter()
        .any(|&bb| has_array_method(&query, bb, "get"));
    if !has_get_call {
        log_fallback(
            "funcscanner_append_defs",
            "ArrayBox.get() not found in function body",
        );
        return lower_handwritten(module);
    }

    let has_push_call = all_blocks
        .iter()
        .any(|&bb| has_array_method(&query, bb, "push"));
    if !has_push_call {
        log_fallback(
            "funcscanner_append_defs",
            "ArrayBox.push() not found in function body",
        );
        return lower_handwritten(module);
    }

    let has_increment = all_blocks.iter().any(|&bb| has_loop_increment(&query, bb));
    if !has_increment {
        log_fallback(
            "funcscanner_append_defs",
            "loop increment (i + 1) not found in function body",
        );
        return lower_handwritten(module);
    }

    if crate::config::env::joinir_dev::debug_enabled() {
        let ring0 = get_global_ring0();
        ring0
            .log
            .debug("[joinir/funcscanner_append_defs/mir] CFG sanity checks passed ✅");
        ring0
            .log
            .debug("[joinir/funcscanner_append_defs/mir] Found: length(), get(), push(), i+1");
    }

    // Phase 31: LoopToJoinLowerer 統一箱経由に移行
    if crate::mir::join_ir::env_flag_is_1("NYASH_JOINIR_LOWER_GENERIC") {
        use crate::mir::join_ir::lowering::loop_to_join::LoopToJoinLowerer;

        let header = query.succs(entry).get(0).copied().unwrap_or(entry);
        let succs_header = query.succs(header);
        let body = succs_header.get(0).copied().unwrap_or(header);
        let exit = succs_header.get(1).copied().unwrap_or(header);
        let loop_form = LoopForm {
            preheader: entry,
            header,
            body,
            latch: body,
            exit,
            continue_targets: vec![body],
            break_targets: vec![exit],
        };
        if crate::mir::join_ir::lowering::common::case_a::is_simple_case_a_loop(&loop_form) {
            if crate::config::env::joinir_dev::debug_enabled() {
                get_global_ring0().log.debug(
                    "[joinir/funcscanner_append_defs/generic-hook] detected simple Case A loop (LoopToJoinLowerer)",
                );
            }
            let lowerer = LoopToJoinLowerer::new();
            if let Some(jm) = lowerer.lower_case_a_for_append_defs(target_func, &loop_form) {
                if crate::config::env::joinir_dev::debug_enabled() {
                    get_global_ring0().log.debug(
                        "[joinir/funcscanner_append_defs/generic-hook] LoopToJoinLowerer produced JoinIR, returning early",
                    );
                }
                return Some(jm);
            }
            if crate::config::env::joinir_dev::debug_enabled() {
                get_global_ring0().log.debug(
                    "[joinir/funcscanner_append_defs/generic-hook] LoopToJoinLowerer returned None, falling back to handwritten/MIR path",
                );
            }
        }
    }

    // Phase 27.14: Generate JoinIR using shared builder
    // CFG checks passed, so we can use build_funcscanner_append_defs_joinir() directly
    if crate::config::env::joinir_dev::debug_enabled() {
        get_global_ring0().log.debug(
            "[joinir/funcscanner_append_defs/mir] Calling build_funcscanner_append_defs_joinir() after CFG validation",
        );
    }
    build_funcscanner_append_defs_joinir(module)
}

/// Phase 27.14: Handwritten lowering wrapper for FuncScannerBox._append_defs
///
/// This is a thin wrapper that calls the shared build_funcscanner_append_defs_joinir() function.
/// Maintains the handwritten lowering path as the baseline reference.
fn lower_handwritten(module: &crate::mir::MirModule) -> Option<JoinModule> {
    if crate::config::env::joinir_dev::debug_enabled() {
        get_global_ring0()
            .log
            .debug("[joinir/funcscanner_append_defs/handwritten] Using handwritten lowering path");
    }
    build_funcscanner_append_defs_joinir(module)
}
