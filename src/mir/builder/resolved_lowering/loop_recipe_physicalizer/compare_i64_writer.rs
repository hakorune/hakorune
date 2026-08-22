//! Test-only compatibility module for the generic Loop physicalizer tests.
//!
//! The production strict writer lives in the non-test `resolved_lowering`
//! module. This module only keeps the existing test ledger trait impl local to
//! the caller-zero generic lane.

use super::operation_ledger::LoopOperationValueDefinitionSourceV1;
use crate::mir::builder::builder_emit::CanonicalCompareDefinitionSourceV1;

pub(in crate::mir::builder::resolved_lowering) use crate::mir::builder::resolved_lowering::CanonicalLoopCompareI64WriterV1;

impl LoopOperationValueDefinitionSourceV1 for CanonicalCompareDefinitionSourceV1 {
    fn physical_value(&self) -> crate::mir::ValueId {
        CanonicalCompareDefinitionSourceV1::physical_value(self)
    }
}
