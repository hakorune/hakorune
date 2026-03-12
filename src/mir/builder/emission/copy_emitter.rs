//! CopyEmissionBox — Copy命令発行の薄いヘルパ（仕様不変・dev補助）
//!
//! 直接 Copy を作る箇所を “許可された窓口” に寄せるための最小入口。
//! strict/dev+planner_required のときだけ dominance を検査して fail-fast する。

use crate::mir::verification::utils::{compute_def_blocks, compute_dominators};
use crate::mir::{BasicBlock, BasicBlockId, MirFunction, MirInstruction, ValueId};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CopyEmitReason {
    JoinIrBridgeConditionalMethodCall,
    JoinIrBridgeIfMergeThen,
    JoinIrBridgeIfMergeElse,
    JoinIrBridgeNestedIfMergeThen,
    JoinIrBridgeNestedIfMergeElse,
    JoinIrBridgeJoinirBlockConverterConditionalMethodCall,
    JoinIrMergeRewriterTailCallParamsContinuation,
    JoinIrMergeRewriterTailCallParamsTailCall,
    JsonV0BridgeLoopformEmitCopy,
    #[cfg(test)]
    TestDceEdgeArgCopy,
    #[cfg(test)]
    TestMirOptimizerDceKeepsEdgeArgsValues,
    #[cfg(test)]
    TestCopyTypePropagatorSingle,
    #[cfg(test)]
    TestCopyTypePropagatorChain1,
    #[cfg(test)]
    TestCopyTypePropagatorChain2,
    #[cfg(test)]
    TestCopyTypePropagatorDstHasType,
    #[cfg(test)]
    TestCopyTypePropagatorSrcUnknown,
    #[cfg(test)]
    TestPhiTypeResolverCopy,
    #[cfg(test)]
    TestPhiTypeResolverPhiCopy,
    #[cfg(test)]
    TestMergeVariableHandler,
}

impl CopyEmitReason {
    #[inline]
    pub const fn as_str(self) -> &'static str {
        match self {
            CopyEmitReason::JoinIrBridgeConditionalMethodCall => {
                "joinir_bridge/conditional_method_call"
            }
            CopyEmitReason::JoinIrBridgeIfMergeThen => "joinir_bridge/if_merge/then",
            CopyEmitReason::JoinIrBridgeIfMergeElse => "joinir_bridge/if_merge/else",
            CopyEmitReason::JoinIrBridgeNestedIfMergeThen => "joinir_bridge/nested_if_merge/then",
            CopyEmitReason::JoinIrBridgeNestedIfMergeElse => "joinir_bridge/nested_if_merge/else",
            CopyEmitReason::JoinIrBridgeJoinirBlockConverterConditionalMethodCall => {
                "joinir_bridge/joinir_block_converter:conditional_method_call"
            }
            CopyEmitReason::JoinIrMergeRewriterTailCallParamsContinuation => {
                "joinir_merge_rewriter/tail_call_params:continuation"
            }
            CopyEmitReason::JoinIrMergeRewriterTailCallParamsTailCall => {
                "joinir_merge_rewriter/tail_call_params:tail_call"
            }
            CopyEmitReason::JsonV0BridgeLoopformEmitCopy => "json_v0_bridge/loopform/emit_copy",
            #[cfg(test)]
            CopyEmitReason::TestDceEdgeArgCopy => "test/dce:edge_arg_copy",
            #[cfg(test)]
            CopyEmitReason::TestMirOptimizerDceKeepsEdgeArgsValues => {
                "test/mir_optimizer:dce_keeps_edge_args_values"
            }
            #[cfg(test)]
            CopyEmitReason::TestCopyTypePropagatorSingle => "test/copy_type_propagator:single",
            #[cfg(test)]
            CopyEmitReason::TestCopyTypePropagatorChain1 => "test/copy_type_propagator:chain1",
            #[cfg(test)]
            CopyEmitReason::TestCopyTypePropagatorChain2 => "test/copy_type_propagator:chain2",
            #[cfg(test)]
            CopyEmitReason::TestCopyTypePropagatorDstHasType => {
                "test/copy_type_propagator:dst_has_type"
            }
            #[cfg(test)]
            CopyEmitReason::TestCopyTypePropagatorSrcUnknown => {
                "test/copy_type_propagator:src_unknown"
            }
            #[cfg(test)]
            CopyEmitReason::TestPhiTypeResolverCopy => "test/phi_type_resolver:copy",
            #[cfg(test)]
            CopyEmitReason::TestPhiTypeResolverPhiCopy => "test/phi_type_resolver:phi_copy",
            #[cfg(test)]
            CopyEmitReason::TestMergeVariableHandler => "test",
        }
    }
}

fn strict_planner_required() -> bool {
    crate::config::env::joinir_dev::strict_planner_required_debug_enabled()
}

#[inline]
#[track_caller]
pub(crate) fn emit_copy_in_block(
    func: &mut MirFunction,
    bb: BasicBlockId,
    dst: ValueId,
    src: ValueId,
    reason: CopyEmitReason,
) -> Result<(), String> {
    if crate::config::env::joinir_dev::debug_enabled() {
        let caller = std::panic::Location::caller();
        func.metadata.value_origin_callers.insert(
            dst,
            format!("{}:{}:{}", caller.file(), caller.line(), caller.column()),
        );
    }
    if strict_planner_required() {
        let def_blocks = compute_def_blocks(func);
        if let Some(def_block) = def_blocks.get(&src).copied() {
            let dominators = compute_dominators(func);
            let dominates = dominators.dominates(def_block, bb);
            if !dominates {
                return Err(format!(
                    "[freeze:contract][copy/non_dominating] fn={} bb={:?} src=%{} def_block={:?} reason={}",
                    func.signature.name,
                    bb,
                    src.0,
                    def_block,
                    reason.as_str()
                ));
            }
        }
    }

    let block = func
        .get_block_mut(bb)
        .ok_or_else(|| format!("copy_emitter: missing block {:?}", bb))?;
    block.add_instruction(MirInstruction::Copy { dst, src });
    Ok(())
}

/// Emit Copy into a detached block that isn't inserted into `MirFunction` yet.
///
/// This is a stop-gap for code paths that construct `BasicBlock` values and
/// attach them to a function later (e.g. block rewrites). Dominance checks are
/// not possible here; defer to verifier / later in-function checks.
#[inline]
pub(crate) fn emit_copy_into_detached_block(
    block: &mut BasicBlock,
    dst: ValueId,
    src: ValueId,
    _reason: CopyEmitReason,
) -> Result<(), String> {
    block.add_instruction(MirInstruction::Copy { dst, src });
    Ok(())
}
