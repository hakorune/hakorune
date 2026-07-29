/*!
 * Control Flow Utilities - 制御フロー処理の共通ユーティリティ
 *
 * PHI incoming修正とブロック終端検出の汎用関数群
 * フェーズS（即効止血）からフェーズL（根本解決）まで共通利用
 */

use super::super::MirBuilder;

/// **外部関数**: 現在のブロックが終端済みかチェック
///
/// loop_builder.rsで3箇所重複していた処理を統一
///
/// # 使用例
/// ```rust
/// if is_current_block_terminated(builder)? {
///     break; // 早期終了
/// }
/// ```
pub fn is_current_block_terminated(builder: &MirBuilder) -> Result<bool, String> {
    builder.checked_current_block_terminated()
}
