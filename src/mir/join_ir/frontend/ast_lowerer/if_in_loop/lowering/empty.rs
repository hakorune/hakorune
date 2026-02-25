//! Phase P1: 空の If - ケース 1
//!
//! 条件チェックのみで then/else 両方が空の場合。
//! 条件式の副作用のみを保持し、効果なしを返す。

use super::super::super::{JoinInst, StatementEffect};

/// ケース 1: 空の If
///
/// 条件式は評価されるが、then/else が空なので効果なし。
pub fn lower(insts: Vec<JoinInst>) -> (Vec<JoinInst>, StatementEffect) {
    (insts, StatementEffect::None)
}
