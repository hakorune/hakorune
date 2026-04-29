//! EdgeCFG stub builders (BranchStub/EdgeStub + EdgeArgs layouts).

use crate::mir::builder::control_flow::edgecfg::api::{BranchStub, EdgeStub, ExitKind};
use crate::mir::builder::control_flow::plan::normalizer::common::empty_args;
use crate::mir::EdgeArgs;
use crate::mir::{BasicBlockId, ValueId};

pub(in crate::mir::builder) fn build_branch_stub(
    from: BasicBlockId,
    cond: ValueId,
    then_target: BasicBlockId,
    then_args: EdgeArgs,
    else_target: BasicBlockId,
    else_args: EdgeArgs,
) -> BranchStub {
    BranchStub {
        from,
        cond,
        then_target,
        then_args,
        else_target,
        else_args,
    }
}

pub(in crate::mir::builder) fn build_normal_edge_stub(
    from: BasicBlockId,
    target: BasicBlockId,
    args: EdgeArgs,
) -> EdgeStub {
    build_edge_stub(from, ExitKind::Normal, Some(target), args)
}

pub(in crate::mir::builder) fn build_edge_stub(
    from: BasicBlockId,
    kind: ExitKind,
    target: Option<BasicBlockId>,
    args: EdgeArgs,
) -> EdgeStub {
    EdgeStub {
        from,
        kind,
        target,
        args,
    }
}

pub(in crate::mir::builder) fn build_loop_header_branch_with_args(
    header_bb: BasicBlockId,
    cond: ValueId,
    body_bb: BasicBlockId,
    body_args: EdgeArgs,
    after_bb: BasicBlockId,
    after_args: EdgeArgs,
) -> BranchStub {
    build_branch_stub(header_bb, cond, body_bb, body_args, after_bb, after_args)
}

#[cfg(test)]
pub(in crate::mir::builder) fn build_loop_cond_branch(
    from: BasicBlockId,
    cond: ValueId,
    then_target: BasicBlockId,
    else_target: BasicBlockId,
) -> BranchStub {
    let args = empty_args();
    build_branch_stub(from, cond, then_target, args.clone(), else_target, args)
}

pub(in crate::mir::builder) fn build_loop_back_edge(
    from: BasicBlockId,
    target: BasicBlockId,
) -> EdgeStub {
    build_normal_edge_stub(from, target, empty_args())
}

pub(in crate::mir::builder) fn build_loop_back_edge_with_args(
    from: BasicBlockId,
    target: BasicBlockId,
    args: EdgeArgs,
) -> EdgeStub {
    build_normal_edge_stub(from, target, args)
}
