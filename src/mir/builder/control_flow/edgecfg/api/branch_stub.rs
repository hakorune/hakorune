use crate::mir::ValueId;
use crate::mir::{BasicBlockId, EdgeArgs};

/// 条件分岐の未配線エッジペア（Phase 267 P0）
///
/// # 責務
/// - header → then/else の分岐を表現
/// - set_branch_with_edge_args() へのマッピング用
///
/// # 制約
/// - 1 block = 1 terminator: from は重複禁止
/// - then_target, else_target は必ず Some（未配線 Branch は存在しない）
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BranchStub {
    /// 分岐元ブロック
    pub from: BasicBlockId,

    /// 分岐条件の値
    pub cond: ValueId,

    /// then ブランチの配線先
    pub then_target: BasicBlockId,

    /// then ブランチの引数
    pub then_args: EdgeArgs,

    /// else ブランチの配線先
    pub else_target: BasicBlockId,

    /// else ブランチの引数
    pub else_args: EdgeArgs,
}

impl BranchStub {
    /// Build a fully-wired conditional branch stub.
    pub fn new(
        from: BasicBlockId,
        cond: ValueId,
        then_target: BasicBlockId,
        then_args: EdgeArgs,
        else_target: BasicBlockId,
        else_args: EdgeArgs,
    ) -> Self {
        Self {
            from,
            cond,
            then_target,
            then_args,
            else_target,
            else_args,
        }
    }
}
