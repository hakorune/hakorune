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
        plan: Option<crate::mir::resolved_semantics::home_new_prefix::ComparisonExecutionPlanV1>,
    ) -> Result<CompletedOrdinaryComparisonV1, String> {
        if plan.is_some_and(|plan| plan.operator() != op) {
            return Err("[freeze:contract][bool-condition/operator-kind]".into());
        }
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
        let original = match plan {
            Some(plan) => crate::mir::builder::emission::compare::emit_planned_recorded(
                self, plan, dst, lhs2, rhs2,
            )?,
            None => {
                crate::mir::builder::emission::compare::emit_to_recorded(self, dst, op, lhs2, rhs2)?
            }
        };

        Ok(CompletedOrdinaryComparisonV1 {
            value: dst,
            original,
        })
    }

    pub(in crate::mir::builder) fn build_borrowed_integer_comparison(
        &mut self,
        source: crate::mir::resolved_semantics::borrowed_integer_comparison::BorrowedIntegerComparisonSourceV1,
        tagged: ValueId,
        observed: ValueId,
        other: ValueId,
        frame: ValueId,
        unwind: Vec<crate::mir::instruction::InvokeOperation>,
    ) -> Result<CompletedOrdinaryComparisonV1, String> {
        use crate::mir::builder::ordinary_new_admission::selected::{
            emit_scalar_operation_to_landing_v1,
            root_graph::{prepare_operation_cleanup_v1, RawRootGraphWriterV1},
        };
        let outward = self.next_block_id();
        let normal = self.next_block_id();
        let mut bindings = Vec::new();
        let (fault, _) = prepare_operation_cleanup_v1(
            self,
            frame,
            outward,
            Vec::new(),
            unwind.clone(),
            &[],
            &mut bindings,
        )?;
        self.metadata_ctx.set_current_span(source.span());
        let (integer, _, _) = emit_scalar_operation_to_landing_v1(
            self,
            &mut RawRootGraphWriterV1,
            crate::mir::instruction::InvokeOperation::DynamicBorrowedInteger { tagged },
            frame,
            normal,
            fault,
            &mut bindings,
        )?;
        let dst = self.next_value_id();
        let (lhs, rhs) = if source.tagged_on_left() {
            (integer, other)
        } else {
            (other, integer)
        };
        let original = crate::mir::builder::emission::compare::emit_to_recorded(
            self,
            dst,
            source.operator(),
            lhs,
            rhs,
        )?;
        let function = self
            .function_state
            .current_function
            .as_mut()
            .ok_or("[freeze:contract][borrowed-integer/function]")?;
        function.metadata.borrowed_integer_comparisons.push(
            crate::mir::verification::borrowed_integer_comparison::ComparisonBinding {
                source,
                tagged,
                observed,
                extracted: integer,
                other,
                result: dst,
                support: bindings,
                unwind,
            },
        );
        Ok(CompletedOrdinaryComparisonV1 {
            value: dst,
            original,
        })
    }
}

/// Immutable physical append observation; not a semantic or source receipt.
#[derive(Debug)]
pub(in crate::mir) struct CompletedOrdinaryComparisonV1 {
    pub(in crate::mir::builder) value: ValueId,
    pub(in crate::mir::builder) original: (crate::mir::BasicBlockId, MirInstruction),
}
impl CompletedOrdinaryComparisonV1 {
    pub(in crate::mir::builder) fn original(&self) -> &(crate::mir::BasicBlockId, MirInstruction) {
        &self.original
    }
}
