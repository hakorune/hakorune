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
            return if builder.binding_ctx.contains(var_name)
                || builder.scope_ctx.function_param_names.contains(var_name)
            {
                Ok(())
            } else {
                Err(format!(
                    "[type/local_contract_binding_missing] name={} boundary=assignment",
                    var_name
                ))
            };
        }

        let mut msg = builder.undefined_variable_message(var_name);
        msg.push_str("\nHint: Nyash requires explicit local declaration. Use `local <name>` before assignment.");
        Err(msg)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::{BindingId, ValueId};

    #[test]
    fn rejects_value_publication_without_lexical_identity() {
        let mut builder = MirBuilder::new();
        builder
            .variable_ctx
            .variable_map
            .insert("x".to_string(), ValueId::new(1));

        let error = AssignmentResolverBox::ensure_declared(&builder, "x")
            .expect_err("missing BindingId must fail");
        assert!(error.starts_with("[type/local_contract_binding_missing]"));
    }

    #[test]
    fn reassignment_keeps_existing_lexical_identity() {
        let mut builder = MirBuilder::new();
        builder
            .variable_ctx
            .variable_map
            .insert("x".to_string(), ValueId::new(1));
        builder
            .binding_ctx
            .insert("x".to_string(), BindingId::new(3));

        AssignmentResolverBox::ensure_declared(&builder, "x").expect("declared local");
        builder
            .variable_ctx
            .variable_map
            .insert("x".to_string(), ValueId::new(2));

        assert_eq!(builder.binding_ctx.lookup("x"), Some(BindingId::new(3)));
    }

    #[test]
    fn function_parameters_use_their_entry_identity_lane() {
        let mut builder = MirBuilder::new();
        builder
            .variable_ctx
            .variable_map
            .insert("arg".to_string(), ValueId::new(0));
        builder
            .scope_ctx
            .function_param_names
            .insert("arg".to_string());

        assert!(AssignmentResolverBox::ensure_declared(&builder, "arg").is_ok());
        assert!(!builder.binding_ctx.contains("arg"));
    }
}
