//! Public compatibility entrypoints kept separate from the unified call core.
//!
//! These methods preserve the existing API and MIR emission order. They do
//! not participate in CallTarget resolution or introduce a second call owner.

use super::UnifiedCallEmitterBox;
use crate::mir::builder::{EffectMask, MirBuilder, MirInstruction, ValueId};
use crate::mir::definitions::call_unified::Callee;

impl UnifiedCallEmitterBox {
    /// Emit a global call with a name constant (public compatibility entry).
    pub fn emit_global_unified(
        builder: &mut MirBuilder,
        dst: Option<ValueId>,
        name: String,
        args: Vec<ValueId>,
    ) -> Result<(), String> {
        let name_const = crate::mir::builder::name_const::make_name_const_result(builder, &name)?;
        let actual_dst = dst.unwrap_or_else(|| builder.next_value_id());
        let mut args = args;
        crate::mir::builder::ssa::local::finalize_args(builder, &mut args)?;
        builder.emit_instruction(MirInstruction::Call {
            dst: Some(actual_dst),
            func: name_const,
            callee: Some(Callee::Global(name.clone())),
            args,
            effects: EffectMask::IO,
        })?;
        builder.annotate_call_result_from_func_name(actual_dst, name);
        Ok(())
    }

    /// Emit a first-class function call (public compatibility entry).
    pub fn emit_value_unified(
        builder: &mut MirBuilder,
        dst: Option<ValueId>,
        func_val: ValueId,
        args: Vec<ValueId>,
    ) -> Result<(), String> {
        let mut args = args;
        crate::mir::builder::ssa::local::finalize_args(builder, &mut args)?;
        builder.emit_instruction(MirInstruction::Call {
            dst,
            func: func_val,
            callee: Some(Callee::Value(func_val)),
            args,
            effects: EffectMask::IO,
        })
    }
}
