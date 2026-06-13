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

use crate::mir::join_ir::JoinModule;
use crate::mir::query::MirQuery;
use crate::runtime::get_global_ring0;

mod builder;

use builder::build_skip_ws_joinir;

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
    if crate::config::env::joinir_dev::lower_generic_enabled() {
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
