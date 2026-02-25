//! Throw statement implementation.
//!
//! This module implements the throw statement for exception raising,
//! with proper cleanup block validation.

use crate::ast::ASTNode;
use crate::mir::builder::{Effect, EffectMask, MirInstruction, ValueId};

/// Control-flow: throw
///
/// Raises an exception with the given expression value.
///
/// # Cleanup Block Validation
///
/// Throwing inside cleanup blocks is controlled by the
/// `NYASH_CLEANUP_ALLOW_THROW=1` environment variable.
pub(in crate::mir::builder) fn cf_throw(
    builder: &mut super::super::super::MirBuilder,
    expression: ASTNode,
) -> Result<ValueId, String> {
    if builder.in_cleanup_block && !builder.cleanup_allow_throw {
        return Err("throw is not allowed inside cleanup block (enable NYASH_CLEANUP_ALLOW_THROW=1 to permit)".to_string());
    }
    if crate::config::env::builder_disable_throw() {
        let v = builder.build_expression(expression)?;
        builder.emit_extern_call_with_effects(
            "env.debug",
            "trace",
            vec![v],
            None,
            EffectMask::PURE.add(Effect::Debug),
        )?;
        return Ok(v);
    }
    let exception_value = builder.build_expression(expression)?;
    builder.emit_instruction(MirInstruction::Throw {
        exception: exception_value,
        effects: EffectMask::PANIC,
    })?;
    Ok(exception_value)
}
