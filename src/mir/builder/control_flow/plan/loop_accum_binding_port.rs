//! Named SSA/PHI capabilities for the DirectAccum physicalizer.
//!
//! The core emitter borrows this port and never owns a second Binding SSA
//! authority.  Production supplies the resolved identity adapter; caller-zero
//! tests use the raw wrapper below.

#[cfg(test)]
use crate::mir::builder::emission::phi_lifecycle::PhiToken;
use crate::mir::builder::emission::phi_lifecycle::PhiTxn;
use crate::mir::builder::resolved_lowering::canonical_cfg::VerifiedPredecessorsV1;
#[cfg(test)]
use crate::mir::builder::ssa::binding::{BindingSsaBuilderV1, MirBindingSsaAdapterV1};
use crate::mir::builder::MirBuilder;
use crate::mir::resolved_semantics::BindingRefV1;
use crate::mir::{BasicBlockId, ValueId};

pub(in crate::mir::builder) trait DirectAccumBindingPortV1 {
    fn seed_input(
        &mut self,
        builder: &mut MirBuilder,
        binding: BindingRefV1,
        block: BasicBlockId,
        value: ValueId,
    ) -> Result<(), String>;

    fn read_binding(
        &mut self,
        builder: &mut MirBuilder,
        phis: &mut PhiTxn,
        binding: BindingRefV1,
        block: BasicBlockId,
    ) -> Result<ValueId, String>;

    fn read_condition_induction(
        &mut self,
        builder: &mut MirBuilder,
        phis: &mut PhiTxn,
        binding: BindingRefV1,
        block: BasicBlockId,
    ) -> Result<ValueId, String>;

    fn read_update_accumulator(
        &mut self,
        builder: &mut MirBuilder,
        phis: &mut PhiTxn,
        binding: BindingRefV1,
        block: BasicBlockId,
    ) -> Result<ValueId, String>;

    fn read_step_induction(
        &mut self,
        builder: &mut MirBuilder,
        phis: &mut PhiTxn,
        binding: BindingRefV1,
        block: BasicBlockId,
    ) -> Result<ValueId, String>;

    fn write_update_accumulator(
        &mut self,
        binding: BindingRefV1,
        block: BasicBlockId,
        value: ValueId,
    ) -> Result<(), String>;

    fn write_step_induction(
        &mut self,
        binding: BindingRefV1,
        block: BasicBlockId,
        value: ValueId,
    ) -> Result<(), String>;

    fn seal(
        &mut self,
        builder: &mut MirBuilder,
        phis: &mut PhiTxn,
        block: BasicBlockId,
        witness: &VerifiedPredecessorsV1,
    ) -> Result<(), String>;
}

/// Caller-zero wrapper.  Production code must use a resolved adapter instead
/// of exposing this raw owner to the physicalizer core.
#[cfg(test)]
pub(in crate::mir::builder) struct RawDirectAccumBindingPort<'ssa> {
    pub(in crate::mir::builder) ssa: &'ssa mut BindingSsaBuilderV1<PhiToken>,
}

#[cfg(test)]
impl DirectAccumBindingPortV1 for RawDirectAccumBindingPort<'_> {
    fn seed_input(
        &mut self,
        _builder: &mut MirBuilder,
        binding: BindingRefV1,
        block: BasicBlockId,
        value: ValueId,
    ) -> Result<(), String> {
        self.ssa
            .define(binding, block, value)
            .map_err(|error| error.to_string())
    }

    fn read_binding(
        &mut self,
        builder: &mut MirBuilder,
        phis: &mut PhiTxn,
        binding: BindingRefV1,
        block: BasicBlockId,
    ) -> Result<ValueId, String> {
        let mut adapter = MirBindingSsaAdapterV1::new(builder, phis);
        self.ssa
            .read(&mut adapter, binding, block)
            .map_err(|error| error.to_string())
    }

    fn read_condition_induction(
        &mut self,
        builder: &mut MirBuilder,
        phis: &mut PhiTxn,
        binding: BindingRefV1,
        block: BasicBlockId,
    ) -> Result<ValueId, String> {
        self.read_binding(builder, phis, binding, block)
    }

    fn read_update_accumulator(
        &mut self,
        builder: &mut MirBuilder,
        phis: &mut PhiTxn,
        binding: BindingRefV1,
        block: BasicBlockId,
    ) -> Result<ValueId, String> {
        self.read_binding(builder, phis, binding, block)
    }

    fn read_step_induction(
        &mut self,
        builder: &mut MirBuilder,
        phis: &mut PhiTxn,
        binding: BindingRefV1,
        block: BasicBlockId,
    ) -> Result<ValueId, String> {
        self.read_binding(builder, phis, binding, block)
    }

    fn write_update_accumulator(
        &mut self,
        binding: BindingRefV1,
        block: BasicBlockId,
        value: ValueId,
    ) -> Result<(), String> {
        self.ssa
            .define(binding, block, value)
            .map_err(|error| error.to_string())
    }

    fn write_step_induction(
        &mut self,
        binding: BindingRefV1,
        block: BasicBlockId,
        value: ValueId,
    ) -> Result<(), String> {
        self.write_update_accumulator(binding, block, value)
    }

    fn seal(
        &mut self,
        builder: &mut MirBuilder,
        phis: &mut PhiTxn,
        block: BasicBlockId,
        witness: &VerifiedPredecessorsV1,
    ) -> Result<(), String> {
        let mut adapter = MirBindingSsaAdapterV1::new(builder, phis);
        self.ssa
            .seal(&mut adapter, block, witness)
            .map_err(|error| error.to_string())
    }
}
