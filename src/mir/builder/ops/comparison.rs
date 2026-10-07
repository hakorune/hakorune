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
    /// **Returns**: Bool value with its immutable original append observation.
    /// Retain the final SSA operands and exact shared-emitter tuple for the
    /// source-scoped Binary consumer. The Binary value facade borrows this core.
    pub(in crate::mir::builder) fn build_comparison_op_recorded(
        &mut self,
        op: CompareOp,
        lhs: ValueId,
        rhs: ValueId,
    ) -> Result<CompletedOrdinaryComparisonV1, String> {
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
        let original =
            crate::mir::builder::emission::compare::emit_to_recorded(self, dst, op, lhs2, rhs2)?;

        Ok(CompletedOrdinaryComparisonV1 {
            value: dst,
            original,
        })
    }
}

/// Immutable physical append observation; not a semantic or source receipt.
#[derive(Debug)]
pub(in crate::mir) struct CompletedOrdinaryComparisonV1 {
    pub(super) value: ValueId,
    original: (crate::mir::BasicBlockId, MirInstruction),
}
impl CompletedOrdinaryComparisonV1 {
    pub(in crate::mir::builder) fn original(&self) -> &(crate::mir::BasicBlockId, MirInstruction) {
        &self.original
    }
}
