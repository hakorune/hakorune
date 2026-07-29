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
//! - `build_local_statement_from_values*` - Local binding publication after descent
//! - `build_me_expression` - Receiver resolution (me/this)
//!
//! **Phase Context**:
//! - Phase 135 P0: Function-level ValueId allocation (SSOT)
//! - Phase 269 P1.2: Fail-Fast principle for receiver resolution
//!
//! **Shared Patterns**:
//! This module shares variable binding and registration patterns with:
//! - Program declaration facts - module declaration inventory before lowering
//! - Consider consolidating variable registration logic in future refactoring

use crate::ast::ASTNode;
use crate::mir::builder::MirBuilder;
use crate::mir::builder::ValueId;

pub(in crate::mir::builder) fn observe_preflighted_local_statement(
    variables: &[String],
    initial_values: &[Option<Box<ASTNode>>],
) {
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
}

pub(in crate::mir::builder) fn preflight_exact_numeric_local_initializers(
    variables: &[String],
    initial_values: &[Option<Box<ASTNode>>],
    declared_type_names: &[Option<String>],
) -> Result<(), String> {
    for (index, name) in variables.iter().enumerate() {
        let declared_type = declared_type_names
            .get(index)
            .and_then(|value| value.as_deref());
        let typed_array = declared_type
            .map(crate::typed_array_contract_spec::parse_annotation)
            .transpose()?
            .flatten()
            .is_some();
        if (crate::mir::type_contracts::local_slot::is_exact_numeric_local_type(declared_type)
            || typed_array)
            && initial_values
                .get(index)
                .and_then(|value| value.as_ref())
                .is_none()
        {
            return Err(format!(
                "[type/local_contract_uninitialized_forbidden] name={} declared_type={}",
                name,
                declared_type.unwrap_or("<missing>")
            ));
        }
    }
    Ok(())
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
    build_local_statement_from_values_with_types(builder, variables, initial_values, Vec::new())
}

pub(in crate::mir::builder) fn build_local_statement_from_values_with_types(
    builder: &mut MirBuilder,
    variables: Vec<String>,
    initial_values: Vec<ValueId>,
    declared_type_names: Vec<Option<String>>,
) -> Result<ValueId, String> {
    build_local_statement_from_values_with_types_and_preclaims(
        builder,
        variables,
        initial_values,
        declared_type_names,
        Vec::new(),
    )
}

pub(in crate::mir::builder) fn build_local_statement_from_values_with_types_and_preclaims(
    builder: &mut MirBuilder,
    variables: Vec<String>,
    initial_values: Vec<ValueId>,
    declared_type_names: Vec<Option<String>>,
    preclaimed_arrays: Vec<
        Option<(
            String,
            crate::typed_array_contract_spec::ArrayElementContractSpec,
        )>,
    >,
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

        let declared_type_name = declared_type_names
            .get(index)
            .and_then(|value| value.as_deref());
        let exact_contract =
            crate::mir::type_contracts::local_slot::is_exact_numeric_local_type(declared_type_name);
        let local_slot_id = builder.declare_local_in_current_scope(var_name, var_id)?;
        let typed_array_contract = declared_type_name
            .map(crate::typed_array_contract_spec::parse_annotation)
            .transpose()?
            .flatten()
            .is_some();
        if exact_contract {
            let function = builder
                .function_state
                .current_function
                .as_mut()
                .ok_or_else(|| {
                    "[type/local_contract_carrier_missing] function=<none>".to_string()
                })?;
            crate::mir::type_contracts::local_slot::register_local_slot_contract(
                function,
                local_slot_id,
                var_name,
                declared_type_name.expect("exact local has declared type"),
            )?;
            builder.emit_instruction(crate::mir::MirInstruction::LocalContractWrite {
                dst: var_id,
                src: init_val,
                local_slot_id,
                write_kind: crate::mir::function::LocalContractWriteKind::Init,
            })?;
        } else if typed_array_contract {
            let function = builder
                .function_state
                .current_function
                .as_mut()
                .ok_or_else(|| {
                    "[type/typed_array_contract_carrier_missing] function=<none>".to_string()
                })?;
            if let Some((contract_id, element_spec)) =
                preclaimed_arrays.get(index).and_then(|entry| entry.clone())
            {
                crate::mir::type_contracts::typed_array::register_source_with_id(
                    function,
                    contract_id,
                    crate::mir::function::TypedArrayContractBoundary::LocalInit,
                    crate::mir::function::TypedArrayContractSourceIdentity::LocalSlot(
                        local_slot_id,
                    ),
                    init_val,
                    element_spec,
                );
            } else {
                let contract_id =
                    crate::mir::type_contracts::typed_array::register_instruction_source(
                        function,
                        crate::mir::function::TypedArrayContractBoundary::LocalInit,
                        crate::mir::function::TypedArrayContractSourceIdentity::LocalSlot(
                            local_slot_id,
                        ),
                        init_val,
                        declared_type_name,
                        &format!(
                            "local:{}:init:{}",
                            local_slot_id.binding_id().raw(),
                            var_id.as_u32()
                        ),
                    )?
                    .expect("typed Array local has a source contract");
                builder.emit_instruction(crate::mir::MirInstruction::ArrayStateContractClaim {
                    contract_id,
                    array: init_val,
                })?;
            }
            builder.emit_instruction(crate::mir::MirInstruction::Copy {
                dst: var_id,
                src: init_val,
            })?;
        } else {
            builder.emit_instruction(crate::mir::MirInstruction::Copy {
                dst: var_id,
                src: init_val,
            })?;
        }
        crate::mir::builder::metadata::propagate::propagate(builder, init_val, var_id);
        builder
            .function_state
            .compilation
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
        if let Some(reg) = builder.comp_ctx.current_slot_registry.as_mut() {
            let ty = builder
                .function_state
                .type_ctx
                .value_types
                .get(&var_id)
                .cloned();
            reg.ensure_slot(&var_name, ty);
        }
        last_value = Some(var_id);
    }
    Ok(last_value.unwrap_or_else(|| builder.next_value_id()))
}

/// Build a narrow outbox declaration statement.
///
/// This is the smallest explicit transfer-surface slice:
/// - materialize the declared names as Void-typed local bindings
/// - record the outbox binding names in function metadata
/// - do not introduce a richer ownership checker
pub(in crate::mir::builder) fn build_outbox_statement(
    builder: &mut MirBuilder,
    variables: Vec<String>,
) -> Result<ValueId, String> {
    let values = variables
        .iter()
        .map(|_| crate::mir::builder::emission::constant::emit_void(builder))
        .collect::<Result<Vec<_>, _>>()?;

    let result = build_local_statement_from_values(builder, variables.clone(), values)?;

    if let Some(function) = builder.function_state.current_function.as_mut() {
        function.metadata.outbox_bindings.extend(variables);
    }

    Ok(result)
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
    if let Some(id) = builder
        .function_state
        .variable_ctx
        .variable_map
        .get(ME_VAR)
        .cloned()
    {
        return Ok(id);
    }

    // ✅ Fail-Fast: "me" must be in variable_map (no string fallback)
    // This is a contract violation - caller must initialize "me" before use

    let function_context = builder
        .function_state
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

#[cfg(test)]
mod local_contract_tests {
    use super::*;
    use crate::ast::{LiteralValue, Span};
    use crate::mir::builder::recursive_child_lowering::RawLegacyChildLoweringPortV1;
    use crate::mir::builder::stmts::{drive_local_statement_v1, RawLegacyLocalInputV1};
    use crate::mir::builder::vars::lexical_scope::LexicalScopeGuard;
    use crate::mir::function::LocalContractWriteKind;
    use crate::mir::MirInstruction;

    fn integer(value: i64) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::Integer(value),
            span: Span::unknown(),
        }
    }

    fn lower_local(builder: &mut MirBuilder, node: ASTNode) -> Result<ValueId, String> {
        let ASTNode::Local {
            variables,
            initial_values,
            declared_type_names,
            ..
        } = node
        else {
            return Err("local contract fixture requires Local".to_string());
        };
        let input = RawLegacyLocalInputV1::new(variables, initial_values, declared_type_names);
        let mut port = RawLegacyChildLoweringPortV1;
        drive_local_statement_v1(builder, &mut port, &input)
    }

    #[test]
    fn typed_local_init_and_reassignment_share_one_slot() {
        let mut builder = MirBuilder::new();
        builder.enter_function_for_test("typed_local".to_string());
        let _scope = LexicalScopeGuard::new(&mut builder);
        lower_local(
            &mut builder,
            ASTNode::Local {
                variables: vec!["x".to_string()],
                initial_values: vec![Some(Box::new(integer(1)))],
                declared_type_names: vec![Some("u8".to_string())],
                span: Span::unknown(),
            },
        )
        .unwrap();
        let slot =
            crate::mir::LocalSlotId::from(builder.function_state.binding_ctx.lookup("x").unwrap());
        let rhs = crate::mir::builder::emission::constant::emit_integer(&mut builder, 2).unwrap();
        builder
            .build_assignment_from_value("x".to_string(), rhs)
            .unwrap();

        let function = builder.function_state.current_function.as_ref().unwrap();
        assert_eq!(function.metadata.local_slot_contracts.len(), 1);
        assert_eq!(
            function.metadata.local_slot_contracts[0].local_slot_id,
            slot
        );
        let writes = function
            .blocks
            .values()
            .flat_map(|block| block.instructions.iter())
            .filter_map(|instruction| match instruction {
                MirInstruction::LocalContractWrite {
                    local_slot_id,
                    write_kind,
                    ..
                } => Some((*local_slot_id, *write_kind)),
                _ => None,
            })
            .collect::<Vec<_>>();
        assert_eq!(
            writes,
            vec![
                (slot, LocalContractWriteKind::Init),
                (slot, LocalContractWriteKind::Reassign),
            ]
        );
    }

    #[test]
    fn exact_local_without_initializer_rejects_before_lowering() {
        let mut builder = MirBuilder::new();
        builder.enter_function_for_test("typed_local_uninitialized".to_string());
        let _scope = LexicalScopeGuard::new(&mut builder);
        let error = lower_local(
            &mut builder,
            ASTNode::Local {
                variables: vec!["x".to_string()],
                initial_values: vec![None],
                declared_type_names: vec![Some("i64".to_string())],
                span: Span::unknown(),
            },
        )
        .unwrap_err();
        assert!(error.contains("[type/local_contract_uninitialized_forbidden]"));
        assert!(!builder
            .function_state
            .variable_ctx
            .variable_map
            .contains_key("x"));
        assert!(!builder.function_state.binding_ctx.contains("x"));
    }

    #[test]
    fn typed_array_literal_claim_precedes_appends_and_reassignment_reuses_slot_identity() {
        let mut builder = MirBuilder::new();
        builder.enter_function_for_test("typed_array_local".to_string());
        let _scope = LexicalScopeGuard::new(&mut builder);
        let literal = ASTNode::ArrayLiteral {
            elements: vec![integer(1), integer(2)],
            span: Span::unknown(),
        };
        lower_local(
            &mut builder,
            ASTNode::Local {
                variables: vec!["xs".to_string()],
                initial_values: vec![Some(Box::new(literal))],
                declared_type_names: vec![Some("Array<u8>".to_string())],
                span: Span::unknown(),
            },
        )
        .unwrap();
        let slot =
            crate::mir::LocalSlotId::from(builder.function_state.binding_ctx.lookup("xs").unwrap());

        let mut port = RawLegacyChildLoweringPortV1;
        let replacement = builder
            .build_array_literal_with_port_v1(&mut port, vec![integer(3)])
            .unwrap();
        builder
            .build_assignment_from_value("xs".to_string(), replacement)
            .unwrap();

        let function = builder.function_state.current_function.as_ref().unwrap();
        let instructions = function
            .blocks
            .values()
            .flat_map(|block| block.instructions.iter())
            .collect::<Vec<_>>();
        let first_claim = instructions
            .iter()
            .position(|instruction| {
                matches!(instruction, MirInstruction::ArrayStateContractClaim { .. })
            })
            .unwrap();
        let first_append = instructions
            .iter()
            .position(|instruction| {
                matches!(
                    instruction,
                    MirInstruction::ArrayElementWrite {
                        kind: crate::mir::ArrayElementWriteKind::LiteralAppend,
                        ..
                    }
                )
            })
            .unwrap();
        assert!(first_claim < first_append);

        let local_sources = function
            .metadata
            .typed_array_contract_sources
            .iter()
            .filter_map(|source| match source.source_identity {
                crate::mir::function::TypedArrayContractSourceIdentity::LocalSlot(source_slot) => {
                    Some((source.boundary, source_slot))
                }
                _ => None,
            })
            .collect::<Vec<_>>();
        assert_eq!(local_sources.len(), 2);
        assert!(local_sources
            .iter()
            .all(|(_, source_slot)| *source_slot == slot));
        assert!(local_sources.iter().any(|(boundary, _)| {
            *boundary == crate::mir::function::TypedArrayContractBoundary::LocalInit
        }));
        assert!(local_sources.iter().any(|(boundary, _)| {
            *boundary == crate::mir::function::TypedArrayContractBoundary::LocalReassign
        }));
    }
}
