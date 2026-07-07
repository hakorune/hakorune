//! Compare branch emission bridge.
//!
//! This owner consumes an already-emitted Compare result as a branch condition.
//! It finalizes the condition through LocalSSA and emits a conditional Branch
//! terminator only. It does not select routes or switch runtime authority.

use super::{emission, ssa, MirBuilder};
use crate::mir::{BasicBlockId, ValueId};

pub(in crate::mir::builder) const REASON_OK: u8 = 0;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) struct CompareBranchEmissionResponse {
    pub ok: bool,
    pub reason_code: u8,
    pub branch_condition_consumed: bool,
    pub branch_condition_value_id: ValueId,
    pub then_block: BasicBlockId,
    pub else_block: BasicBlockId,
    pub localssa_finalize_branch_cond_executed: bool,
    pub branch_emission_executed: bool,
    pub route_selection: bool,
    pub runtime_route_switch: bool,
    pub programjson_runtime_authority: bool,
    pub source_selfhost_claim: bool,
}

pub(in crate::mir::builder) struct CompareBranchEmissionBridge;

impl CompareBranchEmissionBridge {
    pub(in crate::mir::builder) fn emit_branch_from_compare_result(
        builder: &mut MirBuilder,
        condition: ValueId,
        then_block: BasicBlockId,
        else_block: BasicBlockId,
    ) -> Result<CompareBranchEmissionResponse, String> {
        let mut finalized_condition = condition;
        ssa::local::finalize_branch_cond(builder, &mut finalized_condition)?;
        emission::branch::emit_conditional(builder, finalized_condition, then_block, else_block)?;
        Ok(CompareBranchEmissionResponse {
            ok: true,
            reason_code: REASON_OK,
            branch_condition_consumed: true,
            branch_condition_value_id: finalized_condition,
            then_block,
            else_block,
            localssa_finalize_branch_cond_executed: true,
            branch_emission_executed: true,
            route_selection: false,
            runtime_route_switch: false,
            programjson_runtime_authority: false,
            source_selfhost_claim: false,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::{CompareOp, ConstValue, MirInstruction};

    #[test]
    fn compare_branch_emission_bridge_emits_branch_only() {
        let mut builder = MirBuilder::new();
        builder.enter_function_for_test("compare_branch_emission_bridge/0".to_string());
        let branch_block = builder.current_block_for_test().unwrap();

        let lhs = builder.alloc_value_for_test();
        builder
            .emit_for_test(MirInstruction::Const {
                dst: lhs,
                value: ConstValue::Integer(1),
            })
            .unwrap();
        let rhs = builder.alloc_value_for_test();
        builder
            .emit_for_test(MirInstruction::Const {
                dst: rhs,
                value: ConstValue::Integer(3),
            })
            .unwrap();
        let compare_result = builder.alloc_value_for_test();
        builder
            .emit_for_test(MirInstruction::Compare {
                dst: compare_result,
                op: CompareOp::Lt,
                lhs,
                rhs,
            })
            .unwrap();

        let then_block = builder.core_ctx.next_block();
        builder.start_new_block(then_block).unwrap();
        let else_block = builder.core_ctx.next_block();
        builder.start_new_block(else_block).unwrap();
        builder.start_new_block(branch_block).unwrap();

        let response = CompareBranchEmissionBridge::emit_branch_from_compare_result(
            &mut builder,
            compare_result,
            then_block,
            else_block,
        )
        .unwrap();

        assert!(response.ok);
        assert_eq!(response.reason_code, REASON_OK);
        assert!(response.branch_condition_consumed);
        assert!(response.branch_condition_value_id.0 > 0);
        assert_eq!(response.then_block, then_block);
        assert_eq!(response.else_block, else_block);
        assert!(response.localssa_finalize_branch_cond_executed);
        assert!(response.branch_emission_executed);

        let function = builder.scope_ctx.current_function.as_ref().unwrap();
        let block = function.blocks.get(&branch_block).unwrap();
        match block.terminator.as_ref().expect("branch terminator") {
            MirInstruction::Branch {
                condition,
                then_bb,
                else_bb,
                ..
            } => {
                assert_eq!(*condition, response.branch_condition_value_id);
                assert_eq!(*then_bb, then_block);
                assert_eq!(*else_bb, else_block);
            }
            other => panic!("expected Branch terminator, got {other:?}"),
        }

        assert!(!response.route_selection);
        assert!(!response.runtime_route_switch);
        assert!(!response.programjson_runtime_authority);
        assert!(!response.source_selfhost_claim);
    }
}
