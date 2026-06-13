//! Route-local MIR-vs-handwritten dispatch arms for `Main.skip/1`.

use crate::mir::join_ir::JoinModule;
use crate::mir::query::MirQueryBox;
use crate::runtime::get_global_ring0;

use super::builder::build_skip_ws_joinir;

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
pub(super) fn lower_skip_ws_from_mir(module: &crate::mir::MirModule) -> Option<JoinModule> {
    use crate::mir::join_ir::lowering::common::{
        ensure_entry_has_succs, has_const_int, has_string_method, log_fallback,
    };

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
    build_skip_ws_joinir(module)
}

/// Phase 27.11.1: Handwritten lowering wrapper for Main.skip/1
///
/// This is a thin wrapper that calls the shared build_skip_ws_joinir() function.
/// Maintains the handwritten lowering path as the baseline reference.
pub(super) fn lower_skip_ws_handwritten(module: &crate::mir::MirModule) -> Option<JoinModule> {
    if crate::config::env::joinir_dev::debug_enabled() {
        get_global_ring0()
            .log
            .debug("[joinir/skip_ws/handwritten] Using handwritten lowering path");
    }
    build_skip_ws_joinir(module)
}
