//! Phase P2/56: Filter パターン lowering
//!
//! ## 責務（1行で表現）
//! **pred が true のときだけ push するループを ConditionalMethodCall に落とす**
//!
//! ## パターン例
//! ```nyash
//! box ArrayExtBox {
//!     filter(pred) {
//!         local out = new ArrayBox()
//!         local i = 0
//!         loop(i < me.size()) {
//!             local v = me.get(i)
//!             if pred(v) {
//!                 out.push(v)  // ← ConditionalMethodCall に変換
//!             }
//!             i = i + 1
//!         }
//!         return out
//!     }
//! }
//! ```
//!
//! ## 生成する JoinIR 構造
//! - entry 関数: Call(loop_step)
//! - loop_step 関数:
//!   - 条件チェック: `i < tokens.size()`
//!   - true: v取得 + ConditionalMethodCall(if pred(v) then push) + i++ + 再帰
//!   - false: Jump(k_exit)
//! - k_exit 関数: Return(out)
//!
//! ## 現在の実装
//! Simple パターンに委譲（stmt_handlers が if 内 MethodCall を適切に処理）

use super::{AstToJoinIrLowerer, JoinModule, LoweringError};

/// Filter パターンを JoinModule に変換
///
/// Phase 56 で実装済みの filter ループを JoinIR に変換する。
/// 現在は Simple パターンに委譲し、stmt_handlers が if 内の処理を適切に行う。
///
/// # Arguments
/// * `lowerer` - AstToJoinIrLowerer インスタンス
/// * `program_json` - Program(JSON v0)
pub fn lower(
    lowerer: &mut AstToJoinIrLowerer,
    program_json: &serde_json::Value,
) -> Result<JoinModule, LoweringError> {
    // Phase 56: Filter パターンは Simple パターンと同じ構造
    // 差分は stmt_handlers での if 内 MethodCall 処理のみ
    // → Simple に委譲
    super::simple::lower(lowerer, program_json)
}
