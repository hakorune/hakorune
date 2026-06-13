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

use crate::mir::join_ir::JoinModule;

mod builder;
mod dispatch;

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
    super::common::dispatch_lowering(
        "stage1_using_resolver",
        module,
        dispatch::lower_from_mir,
        dispatch::lower_handwritten,
    )
}
