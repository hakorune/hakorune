//! Type operation helpers (TypeOp emission)
//!
//! Dead code helpers for type checking and casting.
//! Currently unused but kept for future type system work.

use crate::mir::TypeOpKind;

impl super::super::MirBuilder {
    #[allow(dead_code)]
    pub(super) fn emit_type_check(
        &mut self,
        value: super::super::ValueId,
        expected_type: String,
    ) -> Result<super::super::ValueId, String> {
        let dst = self.next_value_id();
        self.emit_instruction(super::super::MirInstruction::TypeOp {
            dst,
            op: TypeOpKind::Check,
            value,
            ty: super::super::MirType::Box(expected_type),
        })?;
        Ok(dst)
    }

    #[allow(dead_code)]
    pub(super) fn emit_cast(
        &mut self,
        value: super::super::ValueId,
        target_type: super::super::MirType,
    ) -> Result<super::super::ValueId, String> {
        let dst = self.next_value_id();
        self.emit_instruction(super::super::MirInstruction::TypeOp {
            dst,
            op: TypeOpKind::Cast,
            value,
            ty: target_type.clone(),
        })?;
        Ok(dst)
    }
}
