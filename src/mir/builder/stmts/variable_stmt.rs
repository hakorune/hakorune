//! Variable Statement Module - Variable lifecycle management
//!
//! **Purpose**: Handles variable declaration and receiver resolution
//!
//! **Responsibilities**:
//! - Local variable declaration with optional initialization (`local x`, `local x = expr`)
//! - Variable registration in variable_map
//! - Type propagation for initialized variables
//! - Receiver resolution (me/this)
//! - SlotRegistry integration for observation
//!
//! **Key Functions**:
//! - `build_local_statement` - Local variable declaration with optional initialization
//! - `build_me_expression` - Receiver resolution (me/this)
//!
//! **Phase Context**:
//! - Phase 135 P0: Function-level ValueId allocation (SSOT)
//! - Phase 269 P1.2: Fail-Fast principle for receiver resolution
//!
//! **Shared Patterns**:
//! This module shares variable binding and registration patterns with:
//! - `declaration_indexer.rs` - Variable binding in function signatures
//! - Consider consolidating variable registration logic in future refactoring

use crate::ast::ASTNode;
use crate::mir::builder::MirBuilder;
use crate::mir::builder::ValueId;

/// Build a local variable declaration statement.
///
/// Handles both forms:
/// - `local x` - Default initialization to null (sugar for `local x = null`)
/// - `local x = expr` - Explicit initialization
///
/// **Variable Registration**:
/// - Allocates new ValueId for each variable
/// - Registers in variable_map via declare_local_in_current_scope
/// - Registers in SlotRegistry for observation
/// - Propagates type metadata from initializer to variable
///
/// **Phase Context**:
/// - Phase 135 P0: Function-level ValueId allocation (SSOT)
/// - Always in function context (top-level variables forbidden)
///
/// # Arguments
/// * `builder` - MIR builder context
/// * `variables` - List of variable names to declare
/// * `initial_values` - Optional initializer expressions for each variable
///
/// # Returns
/// * `Ok(ValueId)` - Last declared variable's ValueId
/// * `Err(String)` - Error message if declaration fails
///
/// # Example
/// ```hako
/// local x              // ← Default to null
/// local y = 42         // ← Initialize with expression
/// local a, b = 1, 2    // ← Multiple variables
/// ```
pub(in crate::mir::builder) fn build_local_statement(
    builder: &mut MirBuilder,
    variables: Vec<String>,
    initial_values: Vec<Option<Box<ASTNode>>>,
) -> Result<ValueId, String> {
    if crate::config::env::builder_loopform_debug() {
        crate::mir::builder::control_flow::joinir::trace::trace().stderr_if(
            &format!(
                "[build_local_statement] ENTRY: variables={:?}, initial_values.len()={}",
                variables,
                initial_values.len()
            ),
            true,
        );
    }
    let mut evaluated_values = Vec::with_capacity(variables.len());
    for (i, _var_name) in variables.iter().enumerate() {
        let value_id = if i < initial_values.len() && initial_values[i].is_some() {
            let init_expr = initial_values[i].as_ref().unwrap();
            match init_expr.as_ref() {
                ASTNode::New {
                    class, arguments, ..
                } if builder.is_record_constructor_class(class) => {
                    builder.build_record_constructor_value(class.clone(), arguments.clone())?
                }
                _ => builder.build_expression(*init_expr.clone())?,
            }
        } else {
            // `local x` is sugar for `local x = null` (SSOT: docs/reference/language/types.md)
            // At runtime, `null` and `void` are the same "no value" concept, but we preserve `Null`
            // at the MIR-const level for consistency with surface syntax.
            crate::mir::builder::emission::constant::emit_null(builder)?
        };
        evaluated_values.push(value_id);
    }

    build_local_statement_from_values(builder, variables, evaluated_values)
}

/// Build local variable declaration from already-evaluated initializer values.
///
/// This is the shared shell used by ordinary lowering and fastmem lowering.
/// It preserves SSA behavior by copying each initializer into a fresh local
/// ValueId before registering the binding.
pub(in crate::mir::builder) fn build_local_statement_from_values(
    builder: &mut MirBuilder,
    variables: Vec<String>,
    initial_values: Vec<ValueId>,
) -> Result<ValueId, String> {
    let mut last_value = None;
    for (index, var_name) in variables.iter().enumerate() {
        let Some(init_val) = initial_values.get(index).copied() else {
            return Err(format!(
                "[freeze:contract][fastmem/local_missing_initializer] name={}",
                var_name
            ));
        };

        let var_id = builder.next_value_id();

        if crate::config::env::builder_loopform_debug() {
            crate::mir::builder::control_flow::joinir::trace::trace().stderr_if(
                &format!(
                    "[build_local_statement] '{}': init_val={:?}, allocated var_id={:?}",
                    var_name, init_val, var_id
                ),
                true,
            );
        }

        builder.emit_instruction(crate::mir::MirInstruction::Copy {
            dst: var_id,
            src: init_val,
        })?;
        crate::mir::builder::metadata::propagate::propagate(builder, init_val, var_id);
        builder
            .comp_ctx
            .propagate_record_local_value(init_val, var_id);

        if crate::config::env::builder_loopform_debug() {
            crate::mir::builder::control_flow::joinir::trace::trace().stderr_if(
                &format!(
                    "[build_local_statement] Inserting '{}' -> {:?} into variable_map",
                    var_name, var_id
                ),
                true,
            );
        }
        builder.declare_local_in_current_scope(var_name, var_id)?;
        if let Some(reg) = builder.comp_ctx.current_slot_registry.as_mut() {
            let ty = builder.type_ctx.value_types.get(&var_id).cloned();
            reg.ensure_slot(&var_name, ty);
        }
        last_value = Some(var_id);
    }
    Ok(last_value.unwrap_or_else(|| builder.next_value_id()))
}

/// MeResolverBox - SSOT for "me" resolution
///
/// **Purpose**: Resolve receiver reference (me/this) to ValueId
///
/// **Box Theory**: variable_map["me"] only, no string fallback (Fail-Fast principle)
///
/// **Phase Context**:
/// - Phase 269 P1.2: Removed string constant fallback (Fail-Fast principle)
/// - Contract: "me" must be initialized before use
///
/// # Arguments
/// * `builder` - MIR builder context
///
/// # Returns
/// * `Ok(ValueId)` - Receiver ValueId from variable_map
/// * `Err(String)` - Detailed error message if "me" not found
///
/// # Error Handling
/// **Fail-Fast**: Immediately errors if "me" not in variable_map
/// - No string fallback
/// - Provides detailed diagnostic message
/// - Hints at ReceiverNormalizeBox for static calls
///
/// # Example
/// ```hako
/// box Counter {
///     count: IntegerBox
///
///     inc() {
///         me.count = me.count + 1  // ← "me" must be in variable_map
///     }
/// }
/// ```
pub(in crate::mir::builder) fn build_me_expression(
    builder: &mut MirBuilder,
) -> Result<ValueId, String> {
    // Phase 269 P1.2: SSOT - variable_map["me"] only (no string fallback)
    const ME_VAR: &str = "me"; // Small constant SSOT

    // Fast path: return if "me" is in variable_map
    if let Some(id) = builder.variable_ctx.variable_map.get(ME_VAR).cloned() {
        return Ok(id);
    }

    // ✅ Fail-Fast: "me" must be in variable_map (no string fallback)
    // This is a contract violation - caller must initialize "me" before use

    let function_context = builder
        .scope_ctx
        .current_function
        .as_ref()
        .map(|f| f.signature.name.clone())
        .unwrap_or_else(|| "unknown".to_string());

    let static_box_context = builder
        .comp_ctx
        .current_static_box
        .as_ref()
        .map(|s| s.as_str())
        .unwrap_or("none");

    Err(format!(
        "[Phase269/P1.2/MeResolverBox] 'me'/'this' not found in variable_map\n\
         \n\
         Function: {}\n\
         Static box context: {}\n\
         \n\
         This is an **instance method** context error.\n\
         The legacy string constant fallback has been removed (Fail-Fast principle).\n\
         \n\
         Expected: variable_map contains 'me' → Box receiver ValueId (instance method)\n\
         Got: variable_map missing 'me' entry\n\
         \n\
         Possible causes:\n\
         1. Instance method called without proper 'me' initialization\n\
         2. Method called from incorrect context (instance method in static context)\n\
         \n\
         Note: For **static box this.method()** calls, use ReceiverNormalizeBox\n\
         (MethodCall common entry point handles static call normalization).\n\
         \n\
         Hint: Enable NYASH_TRACE_VARMAP=1 to trace variable_map changes.",
        function_context, static_box_context
    ))
}
