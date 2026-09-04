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
mod generic_probe;

/// Main.skip/1 の JoinIR lowering。
///
/// Generic Case A が選択され、形が受理される場合だけその結果を使う。
/// それ以外は既存の `build_skip_ws_joinir` が唯一のlowering ownerである。
///
pub fn lower_skip_ws_to_joinir(module: &crate::mir::MirModule) -> Option<JoinModule> {
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

    builder::build_skip_ws_joinir(module)
}
