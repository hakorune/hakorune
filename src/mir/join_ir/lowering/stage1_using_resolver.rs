//! Phase 27.12: Stage1UsingResolverBox.resolve_for_source entries ループの JoinIR lowering
//!
//! 目的: Stage-1 UsingResolver の最も簡単なループを JoinIR に変換
//!
//! ## 対象ループ
//! - ファイル: `lang/src/compiler/entry/using_resolver_box.hako`
//! - 関数: `Stage1UsingResolverBox.resolve_for_source(src)`
//! - 行数: 44-91
//!
//! ## ループ構造
//! ```hako
//! local i = 0
//! local n = entries.length()
//! loop(i < n) {
//!     local next_i = i + 1
//!     local entry = entries.get(i)
//!     // ... processing ...
//!     i = next_i
//! }
//! ```
//!
//! ## LoopForm ケース: Case A (動的条件 `i < n`)
//!
//! ## Pinned / Carrier / Exit
//! - **Pinned**: `entries` (ArrayBox), `n` (Integer), `modules` (MapBox), `seen` (MapBox)
//! - **Carrier**: `i` (Integer), `prefix` (String)
//! - **Exit**: `prefix` (String - 最終的な連結文字列)
//!
//! ## 想定 JoinIR 構造
//! ```text
//! fn resolve_entries(entries, n, modules, seen, prefix_init) -> String {
//!     let i_init = 0;
//!     loop_step(entries, n, modules, seen, prefix_init, i_init)
//! }
//!
//! fn loop_step(entries, n, modules, seen, prefix, i) -> String {
//!     if i >= n { return prefix }
//!     let entry = entries.get(i)
//!     let next_i = i + 1
//!     // ... processing ...
//!     loop_step(entries, n, modules, seen, new_prefix, next_i)
//! }
//! ```

use crate::mir::join_ir::lowering::common::{
    dispatch_lowering, ensure_entry_has_succs, has_const_int, log_fallback,
};
use crate::mir::join_ir::lowering::value_id_ranges::stage1_using_resolver as vid;
use crate::mir::join_ir::JoinModule;
use crate::mir::query::MirQueryBox;
use crate::runtime::get_global_ring0;

/// Phase 27.12: Stage1UsingResolverBox.resolve_for_source の JoinIR lowering（public dispatcher）
///
/// 環境変数 `NYASH_JOINIR_LOWER_FROM_MIR=1` に応じて、
/// MIR-based 版または handwritten 版を選択する。
///
/// ## トグル制御:
/// - **OFF (デフォルト)**: `lower_handwritten()` を使用
/// - **ON**: `lower_from_mir()` を使用
///
/// ## Shared Builder Pattern
/// 両方の実装が `build_stage1_using_resolver_joinir()` を呼び出す共通パターン。
pub fn lower_stage1_usingresolver_to_joinir(module: &crate::mir::MirModule) -> Option<JoinModule> {
    dispatch_lowering(
        "stage1_using_resolver",
        module,
        lower_from_mir,
        lower_handwritten,
    )
}

/// Phase 27.12: Common JoinIR builder for Stage1UsingResolverBox.resolve_for_source
///
/// This function generates the JoinIR for the entries loop, shared by both:
/// - lower_handwritten (always uses this)
/// - lower_from_mir (uses this after CFG sanity checks pass)
///
/// ## 簡略化方針
/// Phase 27.12 の最小実装として、まずは **最も単純な JoinIR** を生成する：
/// - ループ本体の複雑な処理（should_emit, path 解決等）は省略
/// - ArrayBox.get(i) → 文字列連結 のシンプルな形に固定
///
/// 将来的には MIR から実際の処理を抽出して精密化する。
fn build_stage1_using_resolver_joinir(module: &crate::mir::MirModule) -> Option<JoinModule> {
    use crate::mir::join_ir::*;

    // Phase 27.13: ターゲット関数が存在するかチェック
    let _target_func = module
        .functions
        .get("Stage1UsingResolverBox.resolve_for_source/5")?;

    if crate::config::env::joinir_dev::debug_enabled() {
        let ring0 = get_global_ring0();
        ring0
            .log
            .debug("[joinir/stage1_using_resolver/build] Phase 27.13 implementation");
        ring0
            .log
            .debug("[joinir/stage1_using_resolver/build] Generating JoinIR for entries loop");
        ring0.log.debug(
            "[joinir/stage1_using_resolver/build] Using ValueId range: 7000-8999 (via value_id_ranges)",
        );
    }

    // Step 1: JoinModule を構築
    let mut join_module = JoinModule::new();

    // resolve_entries 関数（entry）:
    // fn resolve_entries(entries, n, modules, seen, prefix_init) -> String {
    //     let i_init = 0;
    //     loop_step(entries, n, modules, seen, prefix_init, i_init)
    // }
    let resolve_id = JoinFuncId::new(0);
    let entries_param = vid::entry(0); // 7000
    let n_param = vid::entry(1); // 7001
    let modules_param = vid::entry(2); // 7002
    let seen_param = vid::entry(3); // 7003
    let prefix_init_param = vid::entry(4); // 7004

    let mut resolve_func = JoinFunction::new(
        resolve_id,
        "resolve_entries".to_string(),
        vec![
            entries_param,
            n_param,
            modules_param,
            seen_param,
            prefix_init_param,
        ],
    );

    let i_init = vid::entry(10); // 7010

    // i_init = 0
    resolve_func
        .body
        .push(JoinInst::Compute(MirLikeInst::Const {
            dst: i_init,
            value: ConstValue::Integer(0),
        }));

    // loop_step(entries, n, modules, seen, prefix_init, i_init)
    let loop_step_id = JoinFuncId::new(1);
    resolve_func.body.push(JoinInst::Call {
        func: loop_step_id,
        args: vec![
            entries_param,
            n_param,
            modules_param,
            seen_param,
            prefix_init_param,
            i_init,
        ],
        k_next: None,
        dst: None,
    });

    join_module.entry = Some(resolve_id);
    join_module.add_function(resolve_func);

    // Phase 27.13: loop_step の Pinned/Carrier 構造を明示
    // UsingResolver entries ループの場合:
    //   - Pinned: entries (ArrayBox), n (Integer), modules (MapBox), seen (MapBox)
    //   - Carrier: prefix (String), i (Integer)
    //   - Exit: prefix (String)
    let entries_loop = vid::loop_step(0); // 8000 - Pinned
    let n_loop = vid::loop_step(1); // 8001 - Pinned
    let modules_loop = vid::loop_step(2); // 8002 - Pinned
    let seen_loop = vid::loop_step(3); // 8003 - Pinned
    let prefix_loop = vid::loop_step(4); // 8004 - Carrier
    let i_loop = vid::loop_step(5); // 8005 - Carrier

    let _header_shape = LoopHeaderShape::new_manual(
        vec![entries_loop, n_loop, modules_loop, seen_loop], // Pinned
        vec![prefix_loop, i_loop],                           // Carrier
    );

    // loop_step 関数:
    // fn loop_step(entries, n, modules, seen, prefix, i) -> String {
    //     if i >= n { return prefix }
    //     let entry = entries.get(i)
    //     let next_i = i + 1
    //     // 簡略化: 文字列連結のみ（should_emit, path 解決等は省略）
    //     let new_prefix = prefix + entry  // 実際は "\n" + code + "\n"
    //     loop_step(entries, n, modules, seen, new_prefix, next_i)
    // }
    let mut loop_step_func = JoinFunction::new(
        loop_step_id,
        "loop_step".to_string(),
        vec![
            entries_loop,
            n_loop,
            modules_loop,
            seen_loop,
            prefix_loop,
            i_loop,
        ],
    );

    let cmp_result = vid::loop_step(10); // 8010
    let entry_value = vid::loop_step(11); // 8011
    let next_i = vid::loop_step(12); // 8012
    let const_1 = vid::loop_step(13); // 8013
    let new_prefix = vid::loop_step(14); // 8014

    // cmp_result = (i >= n)
    loop_step_func
        .body
        .push(JoinInst::Compute(MirLikeInst::Compare {
            dst: cmp_result,
            op: CompareOp::Ge,
            lhs: i_loop,
            rhs: n_loop,
        }));

    // Phase 27.13: Exit φ の意味を LoopExitShape で明示
    // UsingResolver entries ループ脱出時は prefix の値を返す（最終的な連結文字列）
    let _exit_shape = LoopExitShape::new_manual(vec![prefix_loop]); // exit_args = [prefix]

    // if i >= n { return prefix }
    loop_step_func.body.push(JoinInst::Jump {
        cont: JoinContId::new(0),
        args: vec![prefix_loop], // ← LoopExitShape.exit_args に対応
        cond: Some(cmp_result),
    });

    // entry = entries.get(i)
    loop_step_func
        .body
        .push(JoinInst::Compute(MirLikeInst::BoxCall {
            dst: Some(entry_value),
            box_name: "ArrayBox".to_string(),
            method: "get".to_string(),
            args: vec![entries_loop, i_loop],
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

    // 簡略化: 文字列連結のみ（実際の should_emit, path 解決等は省略）
    // new_prefix = prefix + entry (実際は "\n" + code + "\n" だが、ここでは簡略化)
    loop_step_func
        .body
        .push(JoinInst::Compute(MirLikeInst::BinOp {
            dst: new_prefix,
            op: BinOpKind::Add, // String concatenation uses Add
            lhs: prefix_loop,
            rhs: entry_value,
        }));

    // loop_step(entries, n, modules, seen, new_prefix, next_i) - tail recursion
    loop_step_func.body.push(JoinInst::Call {
        func: loop_step_id,
        args: vec![
            entries_loop,
            n_loop,
            modules_loop,
            seen_loop,
            new_prefix,
            next_i,
        ],
        k_next: None,
        dst: None,
    });

    join_module.add_function(loop_step_func);

    if crate::config::env::joinir_dev::debug_enabled() {
        let ring0 = get_global_ring0();
        ring0
            .log
            .debug("[joinir/stage1_using_resolver/build] ✅ JoinIR construction completed");
        ring0.log.debug(&format!(
            "[joinir/stage1_using_resolver/build] Functions: {}",
            join_module.functions.len()
        ));
    }

    Some(join_module)
}

/// Phase 27.12: MIR-based lowering for Stage1UsingResolverBox.resolve_for_source
///
/// CFG sanity checks + MIR パターンマッチング → 成功なら `build_stage1_using_resolver_joinir()` 呼び出し
///
/// ## CFG Sanity Checks (軽量パターンマッチ):
/// 1. Entry block に後続がある
/// 2. Entry block 付近に以下の命令がある:
///    - `Const { value: Integer(0) }` (初期 i = 0)
///    - `BoxCall { box_name: "ArrayBox", method: "length" }` (n = entries.length())
/// 3. ループ本体付近に:
///    - `BoxCall { box_name: "ArrayBox", method: "get" }` (entries.get(i))
///    - `BinOp { op: Add }` (next_i = i + 1)
///
/// ## Graceful Degradation
/// 上記パターンが検出できない場合は `log_fallback()` → `lower_handwritten()` に戻る。
fn lower_from_mir(module: &crate::mir::MirModule) -> Option<JoinModule> {
    if crate::config::env::joinir_dev::debug_enabled() {
        get_global_ring0()
            .log
            .debug("[joinir/stage1_using_resolver/mir] Starting MIR-based lowering");
    }

    // Step 1: Stage1UsingResolverBox.resolve_for_source/5 を探す
    let target_func = module
        .functions
        .get("Stage1UsingResolverBox.resolve_for_source/5")?;

    if crate::config::env::joinir_dev::debug_enabled() {
        let ring0 = get_global_ring0();
        ring0.log.debug(
            "[joinir/stage1_using_resolver/mir] Found Stage1UsingResolverBox.resolve_for_source/5",
        );
        ring0.log.debug(&format!(
            "[joinir/stage1_using_resolver/mir] MIR blocks: {}",
            target_func.blocks.len()
        ));
    }

    // Step 2: MirQueryBox を作成
    let query = MirQueryBox::new(target_func);
    let entry = target_func.entry_block;

    // CFG Check 1: Entry block has successors
    if !ensure_entry_has_succs(&query, entry) {
        log_fallback("stage1_using_resolver", "entry block has no successors");
        return lower_handwritten(module);
    }

    // CFG Check 2: Entry block contains expected route-shape signals
    // Signal A: i = 0 (初期化)
    if !has_const_int(&query, entry, 0) {
        log_fallback("stage1_using_resolver", "Const(0) not found in entry block");
        return lower_handwritten(module);
    }

    // Signal B: entries.length() の検出
    // Phase 27.13: 簡略化のため、複雑な BoxCall 検出は省略
    // 現時点では Const(0) の存在で最小限の sanity check とする

    // TODO (Phase 27.14+): より厳密な CFG パターンマッチング
    //   - has_binop(&query, loop_body, BinaryOp::Add) で i + 1 確認

    if crate::config::env::joinir_dev::debug_enabled() {
        get_global_ring0()
            .log
            .debug("[joinir/stage1_using_resolver/mir] CFG sanity checks passed ✅");
    }

    // Phase 31: LoopToJoinLowerer 統一箱経由に移行
    // Phase 32 L-2.1: CFG から正確な LoopForm を構築
    // Phase 32: construct_simple_while_loopform 共通ヘルパーを使用
    if crate::config::env::joinir_dev::lower_generic_enabled() {
        use crate::mir::join_ir::lowering::common::construct_simple_while_loopform;
        use crate::mir::join_ir::lowering::loop_to_join::LoopToJoinLowerer;

        // Stage-1: entry_is_preheader=false (entry の succ が preheader)
        //          has_break=false (このループに break はない)
        let Some(loop_form) = construct_simple_while_loopform(entry, &query, false, false) else {
            if crate::config::env::joinir_dev::debug_enabled() {
                get_global_ring0().log.debug(
                    "[joinir/stage1_using_resolver/generic-hook] failed to construct LoopForm from CFG",
                );
            }
            return lower_handwritten(module);
        };

        if crate::config::env::joinir_dev::debug_enabled() {
            get_global_ring0().log.debug(&format!(
                "[joinir/stage1_using_resolver/generic-hook] constructed LoopForm: preheader={:?} header={:?} body={:?} latch={:?} exit={:?} break={:?}",
                loop_form.preheader, loop_form.header, loop_form.body, loop_form.latch, loop_form.exit, loop_form.break_targets
            ));
        }

        if crate::mir::join_ir::lowering::common::case_a::is_simple_case_a_loop(&loop_form) {
            if crate::config::env::joinir_dev::debug_enabled() {
                get_global_ring0().log.debug(
                    "[joinir/stage1_using_resolver/generic-hook] simple Case A loop detected (LoopToJoinLowerer)",
                );
            }
            let params_len = target_func.params.len();
            if params_len == 5 {
                let lowerer = LoopToJoinLowerer::new();
                if let Some(jm) = lowerer.lower_case_a_for_stage1_resolver(target_func, &loop_form)
                {
                    if crate::config::env::joinir_dev::debug_enabled() {
                        get_global_ring0().log.debug(
                            "[joinir/stage1_using_resolver/generic-hook] LoopToJoinLowerer produced JoinIR, returning early",
                        );
                    }
                    return Some(jm);
                }
            }
            if crate::config::env::joinir_dev::debug_enabled() {
                get_global_ring0().log.debug(
                    "[joinir/stage1_using_resolver/generic-hook] LoopToJoinLowerer returned None or params mismatch, falling back to handwritten/MIR path",
                );
            }
        } else {
            if crate::config::env::joinir_dev::debug_enabled() {
                get_global_ring0().log.debug(
                    "[joinir/stage1_using_resolver/generic-hook] NOT simple Case A loop, falling back",
                );
            }
        }
    }

    // Phase 27.12: Generate JoinIR using shared builder
    // CFG checks passed, so we can use build_stage1_using_resolver_joinir() directly
    if crate::config::env::joinir_dev::debug_enabled() {
        get_global_ring0().log.debug(
            "[joinir/stage1_using_resolver/mir] Calling build_stage1_using_resolver_joinir() after CFG validation",
        );
    }
    build_stage1_using_resolver_joinir(module)
}

/// Phase 27.12: Handwritten lowering wrapper for Stage1UsingResolverBox.resolve_for_source
///
/// This is a thin wrapper that calls the shared build_stage1_using_resolver_joinir() function.
/// Maintains the handwritten lowering path as the baseline reference.
fn lower_handwritten(module: &crate::mir::MirModule) -> Option<JoinModule> {
    if crate::config::env::joinir_dev::debug_enabled() {
        get_global_ring0()
            .log
            .debug("[joinir/stage1_using_resolver/handwritten] Using handwritten lowering path");
    }
    build_stage1_using_resolver_joinir(module)
}
