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
use crate::runtime::get_global_ring0;

mod builder;
mod dispatch;
mod generic_probe;

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
        if let Some(jm) = generic_probe::try_lower_skip_ws_generic_case_a(module) {
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
        dispatch::lower_skip_ws_from_mir,
        dispatch::lower_skip_ws_handwritten,
    )
}
