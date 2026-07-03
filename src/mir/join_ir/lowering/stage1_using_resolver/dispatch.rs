//! Route-local dispatch for `Stage1UsingResolverBox.resolve_for_source/5`.
//!
//! The lower-resolver route keeps its params-length guard and diagnostics local. Do not widen the
//! shared target adapter to absorb this route-specific policy.

use crate::mir::join_ir::lowering::common::{ensure_entry_has_succs, has_const_int, log_fallback};
use crate::mir::join_ir::JoinModule;
use crate::mir::query::MirQueryBox;
use crate::runtime::get_global_ring0;

use super::builder::build_stage1_using_resolver_joinir;

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
pub(super) fn lower_from_mir(module: &crate::mir::MirModule) -> Option<JoinModule> {
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

    try_stage1_generic_case_a(module, target_func, entry, &query)
        .or_else(|| build_after_cfg_validation(module))
}

fn try_stage1_generic_case_a(
    module: &crate::mir::MirModule,
    target_func: &crate::mir::MirFunction,
    entry: crate::mir::BasicBlockId,
    query: &MirQueryBox<'_>,
) -> Option<JoinModule> {
    // Phase 31: LoopToJoinLowerer 統一箱経由に移行
    // Phase 32 L-2.1: CFG から正確な LoopForm を構築
    // Phase 32: construct_simple_while_loopform 共通ヘルパーを使用
    if !crate::config::env::joinir_dev::lower_generic_enabled() {
        return None;
    }

    use crate::mir::join_ir::lowering::common::construct_simple_while_loopform;
    use crate::mir::join_ir::lowering::loop_to_join::LoopToJoinLowerer;

    // lower-resolver route: entry_is_preheader=false (entry の succ が preheader)
    //                       has_break=false (このループに break はない)
    let Some(loop_form) = construct_simple_while_loopform(entry, query, false, false) else {
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
            if let Some(jm) = lowerer.lower_case_a_for_stage1_resolver(target_func, &loop_form) {
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
    } else if crate::config::env::joinir_dev::debug_enabled() {
        get_global_ring0().log.debug(
            "[joinir/stage1_using_resolver/generic-hook] NOT simple Case A loop, falling back",
        );
    }

    None
}

fn build_after_cfg_validation(module: &crate::mir::MirModule) -> Option<JoinModule> {
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
pub(super) fn lower_handwritten(module: &crate::mir::MirModule) -> Option<JoinModule> {
    if crate::config::env::joinir_dev::debug_enabled() {
        get_global_ring0()
            .log
            .debug("[joinir/stage1_using_resolver/handwritten] Using handwritten lowering path");
    }
    build_stage1_using_resolver_joinir(module)
}
