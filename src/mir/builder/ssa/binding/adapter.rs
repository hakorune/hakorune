use crate::mir::{BasicBlockId, ValueId};

/// Narrow MIR boundary needed by sealed-block Binding SSA.
pub(in crate::mir::builder) trait BindingSsaIrV1 {
    type PhiToken: Copy + Eq + std::fmt::Debug;

    fn define_provisional_phi(
        &mut self,
        block: BasicBlockId,
    ) -> Result<(ValueId, Self::PhiToken), String>;

    fn patch_phi_inputs(
        &mut self,
        token: Self::PhiToken,
        inputs: &[(BasicBlockId, ValueId)],
    ) -> Result<(), String>;

    fn verify_phi_input(&self, predecessor: BasicBlockId, value: ValueId) -> Result<(), String>;

    fn rollback_phi(&mut self, token: Self::PhiToken) -> Result<(), String>;
}
