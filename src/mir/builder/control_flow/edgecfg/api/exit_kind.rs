/*!
 * ExitKind - 制御フローの脱出種別（Phase 264: EdgeCFG Fragment入口）
 *
 * 構造化制御（if/loop/catch/cleanup）から CFG への lowering において、
 * 脱出エッジの配線先を決定するための一次概念。
 */

use crate::mir::control_form::LoopId;

/// 制御フローの脱出種別
///
/// # 設計原則
/// - ExitKind は pattern番号より優先される一次概念
/// - 各 Frag は ExitKind ごとに未配線の EdgeStub を保持
/// - 合成則（seq/if_/loop_/cleanup）で配線ルールを統一
///
/// # 既存型の使用
/// - `LoopId`: `crate::mir::control_form::LoopId` を使用（既存の安定な型）
///   - 定義場所: `src/mir/control_form.rs:23`
///   - 後で差し替える必要なし（既に control_form で SSOT 化済み）
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ExitKind {
    /// 通常のフォールスルー（次の文・ループ継続など）
    Normal,

    /// 関数からのreturn
    Return,

    /// 指定ループからの脱出
    Break(LoopId),

    /// 指定ループの次回イテレーション
    Continue(LoopId),

    /// 例外のunwind（Invoke.err → catch）
    Unwind,

    /// 非同期タスクのキャンセル（予約：将来の async/drop 用）
    Cancel,
}

impl ExitKind {
    /// ループ関連の脱出か判定
    pub fn is_loop_exit(&self) -> bool {
        matches!(self, ExitKind::Break(_) | ExitKind::Continue(_))
    }

    /// 関数全体からの脱出か判定
    pub fn is_function_exit(&self) -> bool {
        matches!(self, ExitKind::Return | ExitKind::Unwind)
    }
}

impl std::fmt::Display for ExitKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExitKind::Normal => write!(f, "Normal"),
            ExitKind::Return => write!(f, "Return"),
            ExitKind::Break(id) => write!(f, "Break({})", id.0),
            ExitKind::Continue(id) => write!(f, "Continue({})", id.0),
            ExitKind::Unwind => write!(f, "Unwind"),
            ExitKind::Cancel => write!(f, "Cancel"),
        }
    }
}
