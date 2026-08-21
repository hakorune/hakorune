//! Production strict writer for one prepared canonical Loop CompareI64 row.
//!
//! This writer is independent of the test-only generic Loop physicalizer. It
//! consumes canonical CFG/SSA witnesses and a prepared Bool fact, then reaches
//! the one shared physical append core exactly once.

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
pub(in crate::mir::builder::resolved_lowering) struct CanonicalLoopCompareI64WriterV1;

impl CanonicalLoopCompareI64WriterV1 {
    /// Consume the precomputed Bool plan and append exactly one Compare.
    /// Everything that can reject is prepared before `commit()`.
    pub(in crate::mir::builder::resolved_lowering) fn emit(
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
