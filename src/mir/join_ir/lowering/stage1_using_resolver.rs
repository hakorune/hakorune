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
use crate::mir::join_ir::JoinModule;
use crate::mir::query::MirQueryBox;
use crate::runtime::get_global_ring0;

mod builder;

use builder::build_stage1_using_resolver_joinir;

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
                "[joinir/stage1_using_resolver/generic-hook] constructed LoopForm: \
                 preheader={:?} header={:?} body={:?} latch={:?} exit={:?} break={:?}",
                loop_form.preheader,
                loop_form.header,
                loop_form.body,
                loop_form.latch,
                loop_form.exit,
                loop_form.break_targets
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
                            "[joinir/stage1_using_resolver/generic-hook] \
                             LoopToJoinLowerer produced JoinIR, returning early",
                        );
                    }
                    return Some(jm);
                }
            }
            if crate::config::env::joinir_dev::debug_enabled() {
                get_global_ring0().log.debug(
                    "[joinir/stage1_using_resolver/generic-hook] LoopToJoinLowerer returned None \
                     or params mismatch, falling back to handwritten/MIR path",
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
