use super::super::schema::LoopExitKindV1;
use super::model::{LoopJoinEdgeRoleV1, LoopJoinEdgeV1, LoopJoinPayloadV1, LoopJoinPortV1};

pub(super) fn loop_exit_edge(
    exit: LoopExitKindV1,
    payload: Vec<LoopJoinPayloadV1>,
) -> LoopJoinEdgeV1 {
    match exit {
        LoopExitKindV1::Break { .. } => LoopJoinEdgeV1 {
            from: LoopJoinPortV1::Body,
            to: LoopJoinPortV1::After,
            role: LoopJoinEdgeRoleV1::Break,
            payload,
        },
        LoopExitKindV1::Continue { .. } => LoopJoinEdgeV1 {
            from: LoopJoinPortV1::Body,
            to: LoopJoinPortV1::Header,
            role: LoopJoinEdgeRoleV1::Continue,
            payload,
        },
        LoopExitKindV1::Return { .. } => unreachable!("direct branch rows reject Return"),
    }
}
