//! Phase 90 P0: ParseStringComposite パターン lowering
//!
//! ## 責務（1行で表現）
//! **continue(escape) + early return(close quote) + 可変ステップを持つループを JoinIR に落とす**
//!
//! ## パターン例
//! ```nyash
//! loop(i < n) {
//!     if i == 7 { return acc }  // Early return (close quote)
//!     if i == 3 {
//!         i = i + 2  // Variable step (escape)
//!         continue
//!     }
//!     acc = acc + 1
//!     i = i + 1
//! }
//! ```
//!
//! ## 期待動作 (n=10)
//! - i=0,1,2: acc++ (acc=3)
//! - i=3: escape → i=5, continue (acc=3)
//! - i=5,6: acc++ (acc=5)
//! - i=7: close quote → return acc=5
//!
//! ## 設計方針
//! - **Phase 90 P0**: dev-only fixture、ContinueReturn パターンを再利用
//! - **StepCalculator 活用**: 可変ステップ計算（i+=1 vs i+=2）
//! - **Fail-Fast**: 複数 Return/Continue、非対応形式は明示エラー

use super::continue_return_pattern;
use super::{AstToJoinIrLowerer, JoinModule, LoweringError};

/// ParseStringComposite パターンを JoinModule に変換
///
/// # Arguments
/// * `lowerer` - AstToJoinIrLowerer インスタンス
/// * `program_json` - Program(JSON v0)
///
/// # Notes
/// Phase 90 P0: ContinueReturn パターンを再利用
/// - ParseStringComposite は ContinueReturn の特殊ケース（可変ステップ）
/// - 構造は同じなので continue_return_pattern::lower を呼び出すだけ
pub fn lower(
    lowerer: &mut AstToJoinIrLowerer,
    program_json: &serde_json::Value,
) -> Result<JoinModule, LoweringError> {
    // Phase 90 P0: ContinueReturn パターンを再利用
    // ParseStringComposite は構造的に ContinueReturn と同じ
    // 唯一の違いは i+=2 vs i+=1 だが、StepCalculator が自動検出
    continue_return_pattern::lower(lowerer, program_json)
}
