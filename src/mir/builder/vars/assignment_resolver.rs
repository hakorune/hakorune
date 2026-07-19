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
        // Pin temporaries are direct SSA values. Reaching named assignment is
        // a compiler routing error in every mode, including the REPL.
        if var_name.starts_with("__pin$") {
            return Err(format!(
                "[type/pin_named_assignment_forbidden] name={}",
                var_name
            ));
        }

        // Phase 288 P2: REPL mode allows implicit user local declarations.
        if builder.repl_mode {
            return Ok(());
        }

        if builder
            .function_state
            .variable_ctx
            .variable_map
            .contains_key(var_name)
        {
            return if builder.function_state.binding_ctx.contains(var_name) {
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
            .function_state
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
            .function_state
            .variable_ctx
            .variable_map
            .insert("x".to_string(), ValueId::new(1));
        builder
            .function_state
            .binding_ctx
            .insert("x".to_string(), BindingId::new(3));

        AssignmentResolverBox::ensure_declared(&builder, "x").expect("declared local");
        builder
            .function_state
            .variable_ctx
            .variable_map
            .insert("x".to_string(), ValueId::new(2));

        assert_eq!(
            builder.function_state.binding_ctx.lookup("x"),
            Some(BindingId::new(3))
        );
    }

    #[test]
    fn parameter_observation_without_identity_is_rejected() {
        let mut builder = MirBuilder::new();
        builder
            .function_state
            .variable_ctx
            .variable_map
            .insert("arg".to_string(), ValueId::new(0));
        builder
            .function_state
            .scope
            .function_param_names
            .insert("arg".to_string());

        let error = AssignmentResolverBox::ensure_declared(&builder, "arg")
            .expect_err("observation inventory is not assignment authority");
        assert!(error.starts_with("[type/local_contract_binding_missing]"));
        assert!(!builder.function_state.binding_ctx.contains("arg"));
    }

    #[test]
    fn synthetic_pin_named_assignment_is_rejected() {
        let mut builder = MirBuilder::new();
        builder.repl_mode = true;
        let error = AssignmentResolverBox::ensure_declared(&builder, "__pin$1$recv")
            .expect_err("pin names are direct SSA values");
        assert!(error.starts_with("[type/pin_named_assignment_forbidden]"));
    }
}
