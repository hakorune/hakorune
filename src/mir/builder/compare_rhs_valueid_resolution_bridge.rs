//! Compare RHS ValueId resolution bridge.
//!
//! This is the first narrow bridge from read-only compare RHS plans into
//! mutation. The initial owner only permits LiteralI64 constant emission.

use super::{emission, MirBuilder};
use crate::mir::ValueId;

pub(in crate::mir::builder) const REASON_OK: u8 = 0;
pub(in crate::mir::builder) const CONSTANT_KIND_INTEGER: u8 = 1;
pub(in crate::mir::builder) const MUTATION_KIND_CONST_INSTRUCTION_ONLY: u8 = 1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) struct CompareRhsValueIdResolutionResponse {
    pub ok: bool,
    pub reason_code: u8,
    pub rhs_value_id_present: bool,
    pub rhs_value_id: Option<ValueId>,
    pub emitted_constant: bool,
    pub constant_kind_code: u8,
    pub constant_i64: i64,
    pub used_symbol_lookup: bool,
    pub symbol_id: u32,
    pub valueid_allocated: bool,
    pub mutation_performed: bool,
    pub mutation_kind_code: u8,
    pub local_ssa_finalize_compare_executed: bool,
    pub mir_compare_emitted: bool,
    pub mir_branch_emitted: bool,
    pub route_selection: bool,
    pub runtime_route_switch: bool,
    pub programjson_runtime_authority: bool,
    pub source_selfhost_claim: bool,
}

pub(in crate::mir::builder) struct CompareRhsConstantEmissionBridge;

impl CompareRhsConstantEmissionBridge {
    pub(in crate::mir::builder) fn resolve_literal_i64(
        builder: &mut MirBuilder,
        value: i64,
    ) -> Result<CompareRhsValueIdResolutionResponse, String> {
        let rhs = emission::constant::emit_integer(builder, value)?;
        Ok(CompareRhsValueIdResolutionResponse {
            ok: true,
            reason_code: REASON_OK,
            rhs_value_id_present: true,
            rhs_value_id: Some(rhs),
            emitted_constant: true,
            constant_kind_code: CONSTANT_KIND_INTEGER,
            constant_i64: value,
            used_symbol_lookup: false,
            symbol_id: 0,
            valueid_allocated: true,
            mutation_performed: true,
            mutation_kind_code: MUTATION_KIND_CONST_INSTRUCTION_ONLY,
            local_ssa_finalize_compare_executed: false,
            mir_compare_emitted: false,
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

    fn integer_const_count(builder: &MirBuilder) -> usize {
        let function = builder.scope_ctx.current_function.as_ref().unwrap();
        function
            .blocks
            .values()
            .flat_map(|block| block.instructions.iter())
            .filter(|inst| {
                matches!(
                    inst,
                    MirInstruction::Const {
                        value: ConstValue::Integer(_),
                        ..
                    }
                )
            })
            .count()
    }

    fn integer_const_values(builder: &MirBuilder) -> Vec<(ValueId, i64)> {
        let function = builder.scope_ctx.current_function.as_ref().unwrap();
        let mut values = function
            .blocks
            .values()
            .flat_map(|block| block.instructions.iter())
            .filter_map(|inst| match inst {
                MirInstruction::Const {
                    dst,
                    value: ConstValue::Integer(value),
                } => Some((*dst, *value)),
                _ => None,
            })
            .collect::<Vec<_>>();
        values.sort_by_key(|(value_id, _)| value_id.0);
        values
    }

    #[test]
    fn compare_rhs_literal_i64_bridge_emits_const_only() {
        let mut builder = MirBuilder::new();
        builder.enter_function_for_test("compare_rhs_literal_i64_bridge/0".to_string());

        let before_next = builder
            .scope_ctx
            .current_function
            .as_ref()
            .unwrap()
            .next_value_id;
        let before_const_count = integer_const_count(&builder);

        let response =
            CompareRhsConstantEmissionBridge::resolve_literal_i64(&mut builder, 3).unwrap();

        let function = builder.scope_ctx.current_function.as_ref().unwrap();
        let after_next = function.next_value_id;
        let after_const_count = integer_const_count(&builder);
        let rhs = response.rhs_value_id.expect("rhs ValueId");

        assert!(response.ok);
        assert!(response.rhs_value_id_present);
        assert!(rhs.0 > 0);
        assert!(response.emitted_constant);
        assert_eq!(response.constant_kind_code, CONSTANT_KIND_INTEGER);
        assert_eq!(response.constant_i64, 3);
        assert!(response.valueid_allocated);
        assert!(response.mutation_performed);
        assert_eq!(
            response.mutation_kind_code,
            MUTATION_KIND_CONST_INSTRUCTION_ONLY
        );

        assert_eq!(after_next - before_next, 1);
        assert_eq!(after_const_count - before_const_count, 1);
        assert_eq!(integer_const_values(&builder), vec![(rhs, 3)]);
        assert_eq!(
            builder.type_ctx.value_types.get(&rhs),
            Some(&MirType::Integer)
        );

        assert!(!response.used_symbol_lookup);
        assert!(!response.local_ssa_finalize_compare_executed);
        assert!(!response.mir_compare_emitted);
        assert!(!response.mir_branch_emitted);
        assert!(!response.route_selection);
        assert!(!response.runtime_route_switch);
        assert!(!response.programjson_runtime_authority);
        assert!(!response.source_selfhost_claim);
    }
}
