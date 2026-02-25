/// Comparison Operations Module
///
/// **Purpose**: Build comparison operations (Eq, Ne, Lt, Le, Gt, Ge) in MIR.
///
/// **Responsibilities**:
/// - Operator Box routing (CompareOperator.apply/3) when enabled
/// - IntegerBox cast detection and TypeOp insertion for safe comparison
/// - LocalSSA finalization (finalize_compare) for operand correctness
/// - Guard detection (in_cmp_op) to prevent infinite recursion
///
/// **Integration**:
/// - Called from `build_binary_op()` in parent module
/// - Uses `emission::compare::emit_to()` for final MIR emission
/// - Uses `ssa::local::finalize_compare()` for SSA correctness
/// - Checks `NYASH_BUILDER_OPERATOR_BOX_COMPARE_CALL` and `NYASH_BUILDER_OPERATOR_BOX_ALL_CALL` env vars
///
/// **Related Phases**:
/// - Phase 196: TypeFacts SSOT - comparison result is always Bool
/// - Phase 29bq+: Cleanliness campaign - extraction from ops/mod.rs

use super::super::{MirInstruction, MirType, ValueId};
use crate::mir::CompareOp;

impl super::super::MirBuilder {
    /// Build a comparison operation with operator box support and IntegerBox cast handling.
    ///
    /// **Algorithm**:
    /// 1. Check if inside CompareOperator.apply/* (guard detection)
    /// 2. If operator box enabled & not in guard:
    ///    - Emit CompareOperator.apply/3(op_string, lhs, rhs)
    ///    - Annotate result as Bool
    /// 3. Otherwise (legacy path):
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

        let all_call = crate::config::env::builder_operator_box_all_call();

        // Dev: Lower 比較 を演算子ボックス呼び出しに置換（既定OFF）
        let in_cmp_op = self
            .scope_ctx
            .current_function
            .as_ref()
            .map(|f| f.signature.name.starts_with("CompareOperator.apply/"))
            .unwrap_or(false);

        if !in_cmp_op
            && (all_call || crate::config::env::builder_operator_box_compare_call())
        {
            // op名の文字列化
            let opname = match op {
                CompareOp::Eq => "Eq",
                CompareOp::Ne => "Ne",
                CompareOp::Lt => "Lt",
                CompareOp::Le => "Le",
                CompareOp::Gt => "Gt",
                CompareOp::Ge => "Ge",
            };
            let op_const = crate::mir::builder::emission::constant::emit_string(self, opname)?;
            // そのまま値を渡す（型変換/slot化は演算子内orVMで行う）
            let name = "CompareOperator.apply/3".to_string();
            self.emit_legacy_call(
                Some(dst),
                super::super::builder_calls::CallTarget::Global(name),
                vec![op_const, lhs, rhs],
            )?;
            self.type_ctx.value_types.insert(dst, MirType::Bool);
        } else {
            // 既存の比較経路（安全のための型注釈/slot化含む）
            let (lhs2_raw, rhs2_raw) = if self
                .type_ctx
                .value_origin_newbox
                .get(&lhs)
                .map(|s| s == "IntegerBox")
                .unwrap_or(false)
                && self
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
        }

        Ok(dst)
    }
}
