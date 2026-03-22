/*!
 * EdgeStub - 未配線の脱出エッジ（Phase 264: EdgeCFG Fragment）
 *
 * Frag 合成時に「どこへ飛ぶべきか未確定」な edge を表現。
 * 最終的に EdgeCFG の terminator edge に解決される。
 */

use super::exit_kind::ExitKind;
use crate::mir::{BasicBlockId, EdgeArgs};
use crate::mir::join_ir::lowering::inline_boundary::JumpArgsLayout;

/// 未配線の脱出エッジ
///
/// # 責務
/// - `from`: 脱出元のブロックID
/// - `kind`: 脱出の種別（配線先の決定に使う）
/// - `target`: 配線先ブロック（Phase 265 P1 で追加）
/// - `args`: edge-args（target が未確定でも「役割」は確定）
///
/// # Phase 265 P1: target フィールド追加
/// - `None`: 未配線（上位へ伝搬）
/// - `Some(block_id)`: 配線済み（Continue → header, Break → after 等）
///
/// # 既存型の使用
/// - `BasicBlockId`: `crate::mir::BasicBlockId` を使用
///   - 定義場所: `src/mir/basic_block.rs:16`
/// - `EdgeArgs`: `crate::mir::EdgeArgs` を使用（**MIR側のEdgeArgs**）
///   - 定義場所: `src/mir/basic_block.rs:46-51`（Phase 260 P0）
///   - EdgeCFG の terminator operand で使ってる型と同じ（混線回避）
///   - 構成: `{ layout: JumpArgsLayout, values: Vec<ValueId> }`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EdgeStub {
    /// 脱出元ブロック
    pub from: BasicBlockId,

    /// 脱出種別（配線ルール決定に使用）
    pub kind: ExitKind,

    /// 配線先ブロック（Phase 265 P1）
    ///
    /// - `None`: 未配線（上位へ伝搬する exit）
    /// - `Some(block_id)`: 配線済み（compose::loop_ 等で確定）
    pub target: Option<BasicBlockId>,

    /// エッジ引数（Phase 260 EdgeArgs SSOT）
    ///
    /// target が未確定でも、このエッジが運ぶ値の「役割」は
    /// ここで確定する（合成則で args を写像する設計）
    pub args: EdgeArgs,
}

impl EdgeStub {
    /// Generic constructor for edge stubs (wired/unwired).
    pub fn new(
        from: BasicBlockId,
        kind: ExitKind,
        target: Option<BasicBlockId>,
        args: EdgeArgs,
    ) -> Self {
        Self {
            from,
            kind,
            target,
            args,
        }
    }

    /// EdgeArgs を持たない EdgeStub を生成（利便性）
    pub fn without_args(from: BasicBlockId, kind: ExitKind) -> Self {
        Self::new(
            from,
            kind,
            None, // P1: 未配線で生成
            EdgeArgs {
                layout: JumpArgsLayout::CarriersOnly,
                values: vec![],
            },
        )
    }

    /// 既に配線先が確定している EdgeStub を生成（テスト/配線済み用途）
    #[cfg(test)]
    pub fn with_target(
        from: BasicBlockId,
        kind: ExitKind,
        target: BasicBlockId,
        args: EdgeArgs,
    ) -> Self {
        Self::new(from, kind, Some(target), args)
    }
}
