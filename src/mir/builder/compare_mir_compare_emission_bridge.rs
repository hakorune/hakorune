//! Compare MIR emission bridge.
//!
//! This owner emits a MIR Compare from already-finalized lhs/rhs ValueIds. It
//! does not emit Branch or make route/runtime authority decisions.

use super::{emission, MirBuilder};
use crate::mir::{CompareOp, ValueId};

pub(in crate::mir::builder) const REASON_OK: u8 = 0;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) struct CompareMirCompareEmissionResponse {
    pub ok: bool,
    pub reason_code: u8,
    pub compare_result_value_id_present: bool,
    pub compare_result_value_id: ValueId,
    pub lhs_value_id: ValueId,
    pub rhs_value_id: ValueId,
    pub compare_op: CompareOp,
    pub compare_result_valueid_allocated: bool,
    pub mir_compare_emitted: bool,
    pub bool_result_type_publication: bool,
    pub mir_branch_emitted: bool,
    pub route_selection: bool,
    pub runtime_route_switch: bool,
    pub programjson_runtime_authority: bool,
    pub source_selfhost_claim: bool,
}

pub(in crate::mir::builder) struct CompareMirCompareEmissionBridge;

impl CompareMirCompareEmissionBridge {
    pub(in crate::mir::builder) fn emit_compare_from_finalized_operands(
        builder: &mut MirBuilder,
        op: CompareOp,
        lhs: ValueId,
        rhs: ValueId,
    ) -> Result<CompareMirCompareEmissionResponse, String> {
        let dst = builder.next_value_id();
        emission::compare::emit_to(builder, dst, op, lhs, rhs)?;
        Ok(CompareMirCompareEmissionResponse {
            ok: true,
            reason_code: REASON_OK,
            compare_result_value_id_present: true,
            compare_result_value_id: dst,
            lhs_value_id: lhs,
            rhs_value_id: rhs,
            compare_op: op,
            compare_result_valueid_allocated: true,
            mir_compare_emitted: true,
            bool_result_type_publication: true,
            mir_branch_emitted: false,
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
    use crate::mir::{ConstValue, MirInstruction, MirType};

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
    fn compare_mir_compare_emission_bridge_emits_compare_only() {
        let mut builder = MirBuilder::new();
        builder.enter_function_for_test("compare_mir_compare_emission_bridge/0".to_string());

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

        let before_next = builder
            .scope_ctx
            .current_function
            .as_ref()
            .unwrap()
            .next_value_id;
        let before_compare_count = compare_instruction_count(&builder);
        let before_branch_count = branch_instruction_count(&builder);

        let response = CompareMirCompareEmissionBridge::emit_compare_from_finalized_operands(
            &mut builder,
            CompareOp::Lt,
            lhs,
            rhs,
        )
        .unwrap();

        let function = builder.scope_ctx.current_function.as_ref().unwrap();
        assert!(response.ok);
        assert_eq!(response.reason_code, REASON_OK);
        assert!(response.compare_result_value_id_present);
        assert_eq!(response.lhs_value_id, lhs);
        assert_eq!(response.rhs_value_id, rhs);
        assert_eq!(response.compare_op, CompareOp::Lt);
        assert!(response.compare_result_valueid_allocated);
        assert_eq!(function.next_value_id - before_next, 1);
        assert_eq!(
            compare_instruction_count(&builder) - before_compare_count,
            1
        );
        assert_eq!(branch_instruction_count(&builder), before_branch_count);
        assert_eq!(
            builder
                .type_ctx
                .value_types
                .get(&response.compare_result_value_id),
            Some(&MirType::Bool)
        );

        assert!(response.mir_compare_emitted);
        assert!(response.bool_result_type_publication);
        assert!(!response.mir_branch_emitted);
        assert!(!response.route_selection);
        assert!(!response.runtime_route_switch);
        assert!(!response.programjson_runtime_authority);
        assert!(!response.source_selfhost_claim);
    }
}
