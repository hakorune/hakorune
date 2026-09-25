//! Public compatibility entrypoints kept separate from the unified call core.
//!
//! These methods preserve the existing API and MIR emission order. They do
//! not participate in CallTarget resolution or introduce a second call owner.

use super::UnifiedCallEmitterBox;
use crate::mir::builder::{EffectMask, MirBuilder, MirInstruction, ValueId};
use crate::mir::definitions::call_unified::Callee;

impl UnifiedCallEmitterBox {
    /// Emit a first-class function call (public compatibility entry).
    pub fn emit_value_unified(
        builder: &mut MirBuilder,
        dst: Option<ValueId>,
        func_val: ValueId,
        args: Vec<ValueId>,
    ) -> Result<(), String> {
        let mut args = args;
        crate::mir::builder::ssa::local::finalize_args(builder, &mut args)?;
        builder.emit_instruction(MirInstruction::LegacyCallV0 {
            dst,
            func: func_val,
            callee: Some(Callee::Value(func_val)),
            args,
            effects: EffectMask::IO,
        })
    }
}
