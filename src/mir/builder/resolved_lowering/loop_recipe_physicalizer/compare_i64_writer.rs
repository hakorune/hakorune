//! Strict writer boundary for one canonical Loop CompareI64 row.
//!
//! This child only co-seals already-issued witnesses and delegates the actual
//! MIR mutation to `builder_emit`'s prepared writer. It does not select a
//! target, infer a type, inspect a ledger, or retry through the legacy emitter.

use super::operation_ledger::LoopOperationValueDefinitionSourceV1;
use crate::mir::builder::builder_emit::{
    CanonicalCompareAppendRejectV1, CanonicalCompareDefinitionSourceV1,
};
use crate::mir::builder::emission::compare_type::PreparedCanonicalCompareBoolTypeV1;
use crate::mir::builder::resolved_lowering::canonical_cfg::VerifiedCanonicalOpenInstructionTargetV1;
use crate::mir::builder::resolved_lowering::canonical_ssa::{
    ReservedCanonicalCompareDestinationV1, VerifiedCanonicalSameBlockIntegerOperandV1,
};
use crate::mir::builder::MirBuilder;
use crate::mir::CompareOp;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct CanonicalLoopCompareI64WriterV1;

impl CanonicalLoopCompareI64WriterV1 {
    /// Consume the precomputed Bool plan and append exactly one Compare.
    /// Everything that can reject is prepared before `commit()`.
    pub(super) fn emit(
        builder: &mut MirBuilder,
        target: VerifiedCanonicalOpenInstructionTargetV1,
        lhs: VerifiedCanonicalSameBlockIntegerOperandV1,
        rhs: VerifiedCanonicalSameBlockIntegerOperandV1,
        destination: ReservedCanonicalCompareDestinationV1,
        op: CompareOp,
        bool_plan: PreparedCanonicalCompareBoolTypeV1,
    ) -> Result<CanonicalCompareDefinitionSourceV1, CanonicalCompareAppendRejectV1> {
        let prepared =
            builder.prepare_canonical_compare_append(target, lhs, rhs, destination, op)?;
        let definition = prepared.commit();
        bool_plan.commit(
            definition.physical_value(),
            &mut builder.function_state.type_ctx,
        );
        Ok(definition)
    }
}

impl LoopOperationValueDefinitionSourceV1 for CanonicalCompareDefinitionSourceV1 {
    fn physical_value(&self) -> crate::mir::ValueId {
        CanonicalCompareDefinitionSourceV1::physical_value(self)
    }
}
