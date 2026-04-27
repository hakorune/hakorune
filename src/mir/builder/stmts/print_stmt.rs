//! Print Statement Builder - Handle print statement with TypeOp support
//!
//! Purpose: Build MIR instructions for print statements with early TypeOp detection
//!
//! Responsibilities:
//! - Detect isType/asType patterns in print expressions
//! - Emit TypeOp instructions before ExternCall/Call
//! - Support both function call and method call patterns
//! - Handle fallback to expression builder for complex cases
//!
//! Called by: `build_expression()` in expressions.rs (Print pattern)

use super::super::{MirBuilder, MirInstruction, ValueId};
use crate::ast::{ASTNode, CallExpr};
use crate::mir::TypeOpKind;

/// Print statement: env.console.log(value) with early TypeOp handling
///
/// Handles three patterns:
/// 1. `print(isType(val, "Type"))` / `print(asType(val, "Type"))` - function call pattern
/// 2. `print(obj.is("Type"))` / `print(obj.as("Type"))` - method call pattern
/// 3. `print(expression)` - fallback to expression builder
///
/// # Arguments
/// - `builder`: MirBuilder for instruction emission
/// - `expression`: AST expression node to print
///
/// # Returns
/// ValueId of the printed value (for chaining)
///
/// # Phase Comments
/// - Phase 3.2: Unified call support for print statements
pub(in crate::mir::builder) fn build_print_statement(
    builder: &mut MirBuilder,
    expression: ASTNode,
) -> Result<ValueId, String> {
    super::super::utils::builder_debug_log("enter build_print_statement");
    // Prefer wrapper for simple function-call pattern (non-breaking refactor)
    if let Ok(call) = CallExpr::try_from(expression.clone()) {
        if (call.name == "isType" || call.name == "asType") && call.arguments.len() == 2 {
            super::super::utils::builder_debug_log(
                "pattern: print(FunctionCall isType|asType) [via wrapper]",
            );
            if let Some(type_name) =
                super::super::MirBuilder::extract_string_literal(&call.arguments[1])
            {
                super::super::utils::builder_debug_log(&format!(
                    "extract_string_literal OK: {}",
                    type_name
                ));
                let val = builder.build_expression(call.arguments[0].clone())?;
                let ty = super::super::MirBuilder::parse_type_name_to_mir(&type_name);
                let dst = builder.next_value_id();
                let op = if call.name == "isType" {
                    TypeOpKind::Check
                } else {
                    TypeOpKind::Cast
                };
                super::super::utils::builder_debug_log(&format!(
                    "emit TypeOp {:?} value={} dst= {}",
                    op, val, dst
                ));
                builder.emit_instruction(MirInstruction::TypeOp {
                    dst,
                    op,
                    value: val,
                    ty,
                })?;
                builder.emit_extern_call("env.console", "log", vec![dst], None)?;
                return Ok(dst);
            } else {
                super::super::utils::builder_debug_log("extract_string_literal FAIL [via wrapper]");
            }
        }
    }

    match &expression {
        // print(isType(val, "Type")) / print(asType(...))
        ASTNode::FunctionCall {
            name, arguments, ..
        } if (name == "isType" || name == "asType") && arguments.len() == 2 => {
            super::super::utils::builder_debug_log("pattern: print(FunctionCall isType|asType)");
            if let Some(type_name) = super::super::MirBuilder::extract_string_literal(&arguments[1])
            {
                super::super::utils::builder_debug_log(&format!(
                    "extract_string_literal OK: {}",
                    type_name
                ));
                let val = builder.build_expression(arguments[0].clone())?;
                let ty = super::super::MirBuilder::parse_type_name_to_mir(&type_name);
                let dst = builder.next_value_id();
                let op = if name == "isType" {
                    TypeOpKind::Check
                } else {
                    TypeOpKind::Cast
                };
                super::super::utils::builder_debug_log(&format!(
                    "emit TypeOp {:?} value={} dst= {}",
                    op, val, dst
                ));
                builder.emit_instruction(MirInstruction::TypeOp {
                    dst,
                    op,
                    value: val,
                    ty,
                })?;
                builder.emit_extern_call("env.console", "log", vec![dst], None)?;
                return Ok(dst);
            } else {
                super::super::utils::builder_debug_log("extract_string_literal FAIL");
            }
        }
        // print(obj.is("Type")) / print(obj.as("Type"))
        ASTNode::MethodCall {
            object,
            method,
            arguments,
            ..
        } if (method == "is" || method == "as") && arguments.len() == 1 => {
            super::super::utils::builder_debug_log("pattern: print(MethodCall is|as)");
            if let Some(type_name) = super::super::MirBuilder::extract_string_literal(&arguments[0])
            {
                super::super::utils::builder_debug_log(&format!(
                    "extract_string_literal OK: {}",
                    type_name
                ));
                let obj_val = builder.build_expression(*object.clone())?;
                let ty = super::super::MirBuilder::parse_type_name_to_mir(&type_name);
                let dst = builder.next_value_id();
                let op = if method == "is" {
                    TypeOpKind::Check
                } else {
                    TypeOpKind::Cast
                };
                super::super::utils::builder_debug_log(&format!(
                    "emit TypeOp {:?} obj={} dst= {}",
                    op, obj_val, dst
                ));
                builder.emit_instruction(MirInstruction::TypeOp {
                    dst,
                    op,
                    value: obj_val,
                    ty,
                })?;
                builder.emit_extern_call("env.console", "log", vec![dst], None)?;
                return Ok(dst);
            } else {
                super::super::utils::builder_debug_log("extract_string_literal FAIL");
            }
        }
        _ => {}
    }

    let value = builder.build_expression(expression)?;
    super::super::utils::builder_debug_log(&format!("fallback print value={}", value));

    // Phase 3.2: Use unified call for print statements
    let use_unified = super::super::calls::call_unified::is_unified_call_enabled();

    if use_unified {
        // New unified path - treat print as global function
        builder.emit_unified_call(
            None, // print returns nothing
            super::super::CallTarget::Global("print".to_string()),
            vec![value],
        )?;
    } else {
        // Legacy path - use ExternCall
        builder.emit_extern_call("env.console", "log", vec![value], None)?;
    }
    Ok(value)
}
