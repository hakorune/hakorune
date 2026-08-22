//! Production strict writer for one prepared canonical Loop CompareI64 row.
//!
//! This writer is independent of the test-only generic Loop physicalizer. It
//! consumes canonical CFG/SSA witnesses and a prepared Bool fact, then reaches
//! the one shared physical append core exactly once.

use crate::mir::builder::builder_emit::{
    CanonicalCompareAppendRejectV1, CanonicalCompareDefinitionSourceV1,
    PreparedCanonicalCompareAppendV1,
};
use crate::mir::builder::emission::compare_type::PreparedCanonicalCompareBoolTypeV1;
use crate::mir::builder::resolved_lowering::canonical_cfg::VerifiedCanonicalOpenInstructionTargetV1;
use crate::mir::builder::resolved_lowering::canonical_ssa::{
    ReservedCanonicalCompareDestinationV1, VerifiedCanonicalSameBlockIntegerOperandV1,
};
use crate::mir::builder::MirBuilder;
use crate::mir::CompareOp;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder::resolved_lowering) struct CanonicalLoopCompareI64WriterV1;

impl CanonicalLoopCompareI64WriterV1 {
    /// Prepare the strict Compare append without retaining a Builder borrow.
    /// This allows the same transaction to prepare its Branch before result
    /// reservation.
    pub(in crate::mir::builder::resolved_lowering) fn prepare(
        builder: &MirBuilder,
        target: VerifiedCanonicalOpenInstructionTargetV1,
        lhs: VerifiedCanonicalSameBlockIntegerOperandV1,
        rhs: VerifiedCanonicalSameBlockIntegerOperandV1,
        destination: ReservedCanonicalCompareDestinationV1,
        op: CompareOp,
        bool_plan: PreparedCanonicalCompareBoolTypeV1,
    ) -> Result<PreparedCanonicalLoopCompareI64V1, CanonicalCompareAppendRejectV1> {
        let append = builder.prepare_canonical_compare_append(target, lhs, rhs, destination, op)?;
        Ok(PreparedCanonicalLoopCompareI64V1 { append, bool_plan })
    }

    /// Commit one prepared Compare and its already-decided Bool fact.
    /// Preparation owns every fallible check; this suffix has no Result path.
    pub(in crate::mir::builder::resolved_lowering) fn commit(
        prepared: PreparedCanonicalLoopCompareI64V1,
        builder: &mut MirBuilder,
    ) -> CanonicalCompareDefinitionSourceV1 {
        let PreparedCanonicalLoopCompareI64V1 { append, bool_plan } = prepared;
        let definition = append.commit(builder);
        bool_plan.commit(
            definition.physical_value(),
            &mut builder.function_state.type_ctx,
        );
        definition
    }

    /// Compatibility/test facade. Selected Dynamic does not use this entry;
    /// it consumes the explicit prepare/commit pair above.
    #[cfg(test)]
    pub(in crate::mir::builder::resolved_lowering) fn emit(
        builder: &mut MirBuilder,
        target: VerifiedCanonicalOpenInstructionTargetV1,
        lhs: VerifiedCanonicalSameBlockIntegerOperandV1,
        rhs: VerifiedCanonicalSameBlockIntegerOperandV1,
        destination: ReservedCanonicalCompareDestinationV1,
        op: CompareOp,
        bool_plan: PreparedCanonicalCompareBoolTypeV1,
    ) -> Result<CanonicalCompareDefinitionSourceV1, CanonicalCompareAppendRejectV1> {
        let prepared = Self::prepare(builder, target, lhs, rhs, destination, op, bool_plan)?;
        Ok(Self::commit(prepared, builder))
    }
}

#[derive(Debug)]
pub(in crate::mir::builder::resolved_lowering) struct PreparedCanonicalLoopCompareI64V1 {
    append: PreparedCanonicalCompareAppendV1,
    bool_plan: PreparedCanonicalCompareBoolTypeV1,
}

impl PreparedCanonicalLoopCompareI64V1 {
    pub(in crate::mir::builder::resolved_lowering) fn commit(
        self,
        builder: &mut MirBuilder,
    ) -> CanonicalCompareDefinitionSourceV1 {
        CanonicalLoopCompareI64WriterV1::commit(self, builder)
    }
}
