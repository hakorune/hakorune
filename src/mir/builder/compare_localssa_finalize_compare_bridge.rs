//! Compare LocalSSA finalize bridge.
//!
//! This owner calls the existing LocalSSA finalize_compare owner for already
//! resolved lhs/rhs ValueIds. It does not emit MIR Compare or Branch.

use super::{ssa, MirBuilder};
use crate::mir::ValueId;

pub(in crate::mir::builder) const REASON_OK: u8 = 0;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) struct CompareLocalSsaFinalizeCompareResponse {
    pub ok: bool,
    pub reason_code: u8,
    pub lhs_value_id_present: bool,
    pub rhs_value_id_present: bool,
    pub lhs_value_id: ValueId,
    pub rhs_value_id: ValueId,
    pub localssa_finalize_compare_executed: bool,
    pub lhs_changed: bool,
    pub rhs_changed: bool,
    pub valueid_allocated_delta: u32,
    pub instruction_count_delta: usize,
    pub mir_compare_emitted: bool,
    pub mir_branch_emitted: bool,
    pub bool_result_type_publication: bool,
    pub route_selection: bool,
    pub runtime_route_switch: bool,
    pub programjson_runtime_authority: bool,
    pub source_selfhost_claim: bool,
}

pub(in crate::mir::builder) struct CompareLocalSsaFinalizeCompareBridge;

impl CompareLocalSsaFinalizeCompareBridge {
    pub(in crate::mir::builder) fn finalize_operands(
        builder: &mut MirBuilder,
        lhs: ValueId,
        rhs: ValueId,
    ) -> Result<CompareLocalSsaFinalizeCompareResponse, String> {
        let before_next = current_next_value_id(builder);
        let before_instruction_count = instruction_count(builder);

        let mut lhs_final = lhs;
        let mut rhs_final = rhs;
        ssa::local::finalize_compare(builder, &mut lhs_final, &mut rhs_final)?;

        let after_next = current_next_value_id(builder);
        let after_instruction_count = instruction_count(builder);

        Ok(CompareLocalSsaFinalizeCompareResponse {
            ok: true,
            reason_code: REASON_OK,
            lhs_value_id_present: true,
            rhs_value_id_present: true,
            lhs_value_id: lhs_final,
            rhs_value_id: rhs_final,
            localssa_finalize_compare_executed: true,
            lhs_changed: lhs_final != lhs,
            rhs_changed: rhs_final != rhs,
            valueid_allocated_delta: after_next.saturating_sub(before_next),
            instruction_count_delta: after_instruction_count
                .saturating_sub(before_instruction_count),
            mir_compare_emitted: false,
            mir_branch_emitted: false,
            bool_result_type_publication: false,
            route_selection: false,
            runtime_route_switch: false,
            programjson_runtime_authority: false,
            source_selfhost_claim: false,
        })
    }
}

fn current_next_value_id(builder: &MirBuilder) -> u32 {
    builder
        .scope_ctx
        .current_function
        .as_ref()
        .map(|function| function.next_value_id)
        .unwrap_or(0)
}

fn instruction_count(builder: &MirBuilder) -> usize {
    builder
        .scope_ctx
        .current_function
        .as_ref()
        .map(|function| {
            function
                .blocks
                .values()
                .map(|block| block.instructions.len())
                .sum()
        })
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::{ConstValue, MirInstruction};

    fn compare_instruction_count(builder: &MirBuilder) -> usize {
        let function = builder.scope_ctx.current_function.as_ref().unwrap();
        function
            .blocks
            .values()
            .flat_map(|block| block.instructions.iter())
            .filter(|inst| matches!(inst, MirInstruction::Compare { .. }))
            .count()
    }

    fn branch_instruction_count(builder: &MirBuilder) -> usize {
        let function = builder.scope_ctx.current_function.as_ref().unwrap();
        function
            .blocks
            .values()
            .filter(|block| block.terminator.is_some())
            .count()
    }

    #[test]
    fn compare_localssa_finalize_compare_bridge_finalizes_operands_without_compare_emission() {
        let mut builder = MirBuilder::new();
        builder.enter_function_for_test("compare_localssa_finalize_bridge/0".to_string());

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

        let before_next = current_next_value_id(&builder);
        let before_instruction_count = instruction_count(&builder);
        let before_compare_count = compare_instruction_count(&builder);
        let before_branch_count = branch_instruction_count(&builder);

        let response =
            CompareLocalSsaFinalizeCompareBridge::finalize_operands(&mut builder, lhs, rhs)
                .unwrap();

        assert!(response.ok);
        assert_eq!(response.reason_code, REASON_OK);
        assert!(response.lhs_value_id_present);
        assert!(response.rhs_value_id_present);
        assert!(response.localssa_finalize_compare_executed);
        assert!(response.lhs_value_id.0 > 0);
        assert!(response.rhs_value_id.0 > 0);
        assert_ne!(response.lhs_value_id, response.rhs_value_id);

        let after_next = current_next_value_id(&builder);
        let after_instruction_count = instruction_count(&builder);
        assert_eq!(
            response.valueid_allocated_delta,
            after_next.saturating_sub(before_next)
        );
        assert_eq!(
            response.instruction_count_delta,
            after_instruction_count.saturating_sub(before_instruction_count)
        );
        assert_eq!(compare_instruction_count(&builder), before_compare_count);
        assert_eq!(branch_instruction_count(&builder), before_branch_count);

        assert!(!response.mir_compare_emitted);
        assert!(!response.mir_branch_emitted);
        assert!(!response.bool_result_type_publication);
        assert!(!response.route_selection);
        assert!(!response.runtime_route_switch);
        assert!(!response.programjson_runtime_authority);
        assert!(!response.source_selfhost_claim);
    }
}
