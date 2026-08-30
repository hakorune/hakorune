/// Comparison Operations Module
///
/// **Purpose**: Build comparison operations (Eq, Ne, Lt, Le, Gt, Ge) in MIR.
///
/// **Responsibilities**:
/// - IntegerBox cast detection and TypeOp insertion for safe comparison
/// - LocalSSA finalization (finalize_compare) for operand correctness
///
/// **Integration**:
/// - Called from `build_binary_op_from_values()` in parent module
/// - Uses `emission::compare::emit_to()` for final MIR emission
/// - Uses `ssa::local::finalize_compare()` for SSA correctness
///
/// **Related Phases**:
/// - Phase 196: TypeFacts SSOT - comparison result is always Bool
/// - Phase 29bq+: Cleanliness campaign - extraction from ops/mod.rs
use super::super::{MirInstruction, MirType, ValueId};
use crate::mir::CompareOp;

impl super::super::MirBuilder {
    /// Build a comparison operation with IntegerBox cast handling.
    ///
    /// **Algorithm**:
    /// 1. Emit the direct MIR comparison path:
    ///    - Detect IntegerBox operands → insert TypeOp::Cast
    ///    - Finalize operands via LocalSSA (finalize_compare)
    ///    - Emit Compare instruction via emission::compare::emit_to
    ///
    /// **Parameters**:
    /// - `op`: Comparison operator (Eq, Ne, Lt, Le, Gt, Ge)
    /// - `lhs`, `rhs`: Operand ValueIds (already slotified by caller if needed)
    ///
    /// **Returns**: ValueId of comparison result (typed as Bool)
    pub(in crate::mir::builder) fn build_comparison_op(
        &mut self,
        op: CompareOp,
        lhs: ValueId,
        rhs: ValueId,
    ) -> Result<ValueId, String> {
        let dst = self.next_value_id();

        // The legacy Builder operator-call route is rejected at compiler
        // ingress before this function is entered.
        let (lhs2_raw, rhs2_raw) = if self
            .function_state
            .type_ctx
            .value_origin_newbox
            .get(&lhs)
            .map(|s| s == "IntegerBox")
            .unwrap_or(false)
            && self
                .function_state
                .type_ctx
                .value_origin_newbox
                .get(&rhs)
                .map(|s| s == "IntegerBox")
                .unwrap_or(false)
        {
            let li = self.next_value_id();
            let ri = self.next_value_id();
            self.emit_instruction(MirInstruction::TypeOp {
                dst: li,
                op: crate::mir::TypeOpKind::Cast,
                value: lhs,
                ty: MirType::Integer,
            })?;
            self.emit_instruction(MirInstruction::TypeOp {
                dst: ri,
                op: crate::mir::TypeOpKind::Cast,
                value: rhs,
                ty: MirType::Integer,
            })?;
            (li, ri)
        } else {
            (lhs, rhs)
        };
        // Finalize compare operands in current block via LocalSSA
        let mut lhs2 = lhs2_raw;
        let mut rhs2 = rhs2_raw;
        crate::mir::builder::ssa::local::finalize_compare(self, &mut lhs2, &mut rhs2)?;
        crate::mir::builder::emission::compare::emit_to(self, dst, op, lhs2, rhs2)?;

        Ok(dst)
    }
}
