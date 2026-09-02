//! Public compatibility entrypoints kept separate from the unified call core.
//!
//! These methods preserve the existing API and MIR emission order. They do
//! not participate in CallTarget resolution or introduce a second call owner.

use super::UnifiedCallEmitterBox;
use crate::mir::builder::{EffectMask, MirBuilder, MirInstruction, ValueId};
use crate::mir::definitions::call_unified::Callee;
use hakorune_mir_defs::CanonicalGlobalTargetV1;

impl UnifiedCallEmitterBox {
    /// Emit a global call with a name constant (public compatibility entry).
    pub fn emit_global_unified(
        builder: &mut MirBuilder,
        dst: Option<ValueId>,
        target: CanonicalGlobalTargetV1,
        args: Vec<ValueId>,
    ) -> Result<(), String> {
        let display_name = target.display_name();
        let name_const =
            crate::mir::builder::name_const::make_name_const_result(builder, &display_name)?;
        let actual_dst = dst.unwrap_or_else(|| builder.next_value_id());
        let mut args = args;
        crate::mir::builder::ssa::local::finalize_args(builder, &mut args)?;
        builder.emit_instruction(MirInstruction::LegacyCallV0 {
            dst: Some(actual_dst),
            func: name_const,
            callee: Some(Callee::Global(target)),
            args,
            effects: EffectMask::IO,
        })?;
        builder.annotate_call_result_from_func_name(actual_dst, display_name);
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
        builder.emit_instruction(MirInstruction::LegacyCallV0 {
            dst,
            func: func_val,
            callee: Some(Callee::Value(func_val)),
            args,
            effects: EffectMask::IO,
        })
    }
}
