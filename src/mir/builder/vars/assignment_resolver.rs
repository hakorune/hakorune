use super::super::MirBuilder;

/// AssignmentResolverBox
///
/// Responsibility:
/// - Enforce "explicit local declaration" policy for assignments.
/// - Produce consistent diagnostics shared with variable access errors.
pub(in crate::mir::builder) struct AssignmentResolverBox;

impl AssignmentResolverBox {
    pub(in crate::mir::builder) fn ensure_declared(
        builder: &MirBuilder,
        var_name: &str,
    ) -> Result<(), String> {
        // Phase 288 P2: REPL mode allows implicit local declarations
        if builder.repl_mode {
            return Ok(());
        }

        // Compiler-generated temporaries are not part of the user variable namespace.
        if var_name.starts_with("__pin$") {
            return Ok(());
        }

        if builder.variable_ctx.variable_map.contains_key(var_name) {
            return Ok(());
        }

        let mut msg = builder.undefined_variable_message(var_name);
        msg.push_str("\nHint: Nyash requires explicit local declaration. Use `local <name>` before assignment.");
        Err(msg)
    }
}
