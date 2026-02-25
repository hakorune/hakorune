//! Common helpers for simple Case A loop detection (single header/body/exit with minimal branches).
//!
//! 目的:
//! - minimal 系ロワーで個別に持っていた「Case A っぽいか？」チェックを一箇所に集約する。
//! - generic_case_a を噛ませる候補判定を共通化し、責務を明示する。

use crate::mir::loop_form::LoopForm;

/// Detects a minimal single-header Case A loop shape.
///
/// 条件:
/// - header と exit が異なる
/// - latch が body か header のどちらか
/// - continue/break の出発点がそれぞれ高々1
pub fn is_simple_case_a_loop(loop_form: &LoopForm) -> bool {
    loop_form.header != loop_form.exit
        && (loop_form.latch == loop_form.body || loop_form.latch == loop_form.header)
        && loop_form.continue_targets.len() <= 1
        && loop_form.break_targets.len() <= 1
}
