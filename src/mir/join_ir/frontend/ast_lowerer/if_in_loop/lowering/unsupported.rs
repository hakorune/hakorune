//! Phase P1: 複雑なケース（未対応） - ケース 5
//!
//! 複数ステートメント、異なる変数更新など、
//! 現在サポートされていないパターン。
//! Phase 54+ で対応予定。

use super::super::super::{JoinInst, StatementEffect};

/// ケース 5: 複雑なケース（未対応）
///
/// # Panics
/// 常にパニックする（未対応）
pub fn panic_unsupported(then_count: usize, else_count: usize) -> (Vec<JoinInst>, StatementEffect) {
    panic!(
        "Complex If statement in loop body not yet supported (Phase 54). \
         then: {} stmts, else: {} stmts",
        then_count, else_count
    )
}
