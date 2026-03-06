//! Phase P2/55: PrintTokens route lowering
//!
//! ## 責務（1行で表現）
//! **token を順番に取り出して print するループを Jump/Call/MethodCall に落とす**
//!
//! ## route 例
//! ```nyash
//! box JsonTokenizer {
//!     print_tokens() {
//!         local i = 0
//!         loop(i < me.tokens.size()) {
//!             local token = me.tokens.get(i)
//!             print(token)  // ← MethodCall に変換
//!             i = i + 1
//!         }
//!     }
//! }
//! ```
//!
//! ## 生成する JoinIR 構造
//! - entry 関数: Call(loop_step)
//! - loop_step 関数:
//!   - 条件チェック: `i < tokens.size()`
//!   - true: token取得 + print + i++ + 再帰
//!   - false: Jump(k_exit)
//! - k_exit 関数: Return(void)
//!
//! ## 現在の実装
//! Simple routeに委譲（stmt_handlers が MethodCall を適切に処理）

use super::{AstToJoinIrLowerer, JoinModule, LoweringError};

/// PrintTokens routeを JoinModule に変換
///
/// Phase 55 で実装済みの print_tokens ループを JoinIR に変換する。
/// 現在は Simple routeに委譲し、stmt_handlers が MethodCall を適切に処理。
///
/// # Arguments
/// * `lowerer` - AstToJoinIrLowerer インスタンス
/// * `program_json` - Program(JSON v0)
pub fn lower(
    lowerer: &mut AstToJoinIrLowerer,
    program_json: &serde_json::Value,
) -> Result<JoinModule, LoweringError> {
    // Phase 55: PrintTokens routeは Simple routeと同じ構造
    // 差分は stmt_handlers での MethodCall 処理のみ
    // → Simple に委譲
    super::simple::lower(lowerer, program_json)
}
