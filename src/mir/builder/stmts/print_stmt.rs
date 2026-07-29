//! Print Statement Builder - Handle print statement with TypeOp support
//!
//! Purpose: Build MIR instructions for print statements with early TypeOp detection
//!
//! Responsibilities:
//! - Detect isType/asType patterns in print expressions
//! - Emit TypeOp instructions before ExternCall/Call
//! - Support both function call and method call patterns
//! - Lower general print expressions through the expression builder
//!
//! Called by the raw expression dispatcher for the Print pattern.

use super::super::{CallTarget, MirBuilder, MirInstruction, ValueId};
use crate::ast::ASTNode;
use crate::mir::builder::recursive_child_lowering::{
    drive_legacy_expression_v1, RecursiveChildLoweringPortV1,
};
use crate::mir::TypeOpKind;

pub(in crate::mir::builder) struct PreparedRawPrintV1 {
    route: PreparedRawPrintRouteV1,
}

enum PreparedRawPrintRouteV1 {
    TypeOp {
        origin: PreparedRawPrintTypeOpOriginV1,
        operand: ASTNode,
        raw_type_name: String,
        op: TypeOpKind,
    },
    General {
        expression: ASTNode,
        trace: PreparedRawPrintGeneralTraceV1,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PreparedRawPrintTypeOpOriginV1 {
    FunctionWrapper,
    Method,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PreparedRawPrintGeneralTraceV1 {
    None,
    FunctionTypeOpMissingLiteral,
    MethodTypeOpMissingLiteral,
}

enum PreparedRawPrintSourceObservationV1 {
    FunctionTypeOp {
        raw_type_name: String,
        op: TypeOpKind,
    },
    MethodTypeOp {
        raw_type_name: String,
        op: TypeOpKind,
    },
    General(PreparedRawPrintGeneralTraceV1),
}

impl PreparedRawPrintV1 {
    pub(in crate::mir::builder) fn prepare(expression: ASTNode) -> Self {
        let observation = match &expression {
            ASTNode::FunctionCall {
                name, arguments, ..
            } if (name == "isType" || name == "asType") && arguments.len() == 2 => {
                match MirBuilder::extract_string_literal(&arguments[1]) {
                    Some(raw_type_name) => PreparedRawPrintSourceObservationV1::FunctionTypeOp {
                        raw_type_name,
                        op: if name == "isType" {
                            TypeOpKind::Check
                        } else {
                            TypeOpKind::Cast
                        },
                    },
                    None => PreparedRawPrintSourceObservationV1::General(
                        PreparedRawPrintGeneralTraceV1::FunctionTypeOpMissingLiteral,
                    ),
                }
            }
            ASTNode::MethodCall {
                method, arguments, ..
            } if (method == "is" || method == "as") && arguments.len() == 1 => {
                match MirBuilder::extract_string_literal(&arguments[0]) {
                    Some(raw_type_name) => PreparedRawPrintSourceObservationV1::MethodTypeOp {
                        raw_type_name,
                        op: if method == "is" {
                            TypeOpKind::Check
                        } else {
                            TypeOpKind::Cast
                        },
                    },
                    None => PreparedRawPrintSourceObservationV1::General(
                        PreparedRawPrintGeneralTraceV1::MethodTypeOpMissingLiteral,
                    ),
                }
            }
            _ => PreparedRawPrintSourceObservationV1::General(PreparedRawPrintGeneralTraceV1::None),
        };

        let route = match observation {
            PreparedRawPrintSourceObservationV1::FunctionTypeOp { raw_type_name, op } => {
                let ASTNode::FunctionCall { arguments, .. } = expression else {
                    unreachable!("Function TypeOp observation must project FunctionCall")
                };
                let operand = arguments
                    .into_iter()
                    .next()
                    .expect("Function TypeOp arity checked during observation");
                PreparedRawPrintRouteV1::TypeOp {
                    origin: PreparedRawPrintTypeOpOriginV1::FunctionWrapper,
                    operand,
                    raw_type_name,
                    op,
                }
            }
            PreparedRawPrintSourceObservationV1::MethodTypeOp { raw_type_name, op } => {
                let ASTNode::MethodCall { object, .. } = expression else {
                    unreachable!("Method TypeOp observation must project MethodCall")
                };
                PreparedRawPrintRouteV1::TypeOp {
                    origin: PreparedRawPrintTypeOpOriginV1::Method,
                    operand: *object,
                    raw_type_name,
                    op,
                }
            }
            PreparedRawPrintSourceObservationV1::General(trace) => {
                PreparedRawPrintRouteV1::General { expression, trace }
            }
        };
        Self { route }
    }
}

/// Lower one prepared Print route without re-observing its source.
pub(in crate::mir::builder) fn lower_prepared_raw_print_with_port_v1<Port>(
    builder: &mut MirBuilder,
    port: &mut Port,
    prepared: PreparedRawPrintV1,
) -> Result<ValueId, String>
where
    Port: RecursiveChildLoweringPortV1<ExpressionInput = ASTNode>,
{
    super::super::utils::builder_debug_log("enter build_print_statement");
    match prepared.route {
        PreparedRawPrintRouteV1::TypeOp {
            origin,
            operand,
            raw_type_name,
            op,
        } => {
            match origin {
                PreparedRawPrintTypeOpOriginV1::FunctionWrapper => {
                    super::super::utils::builder_debug_log(
                        "pattern: print(FunctionCall isType|asType) [via wrapper]",
                    );
                }
                PreparedRawPrintTypeOpOriginV1::Method => {
                    super::super::utils::builder_debug_log("pattern: print(MethodCall is|as)");
                }
            }
            super::super::utils::builder_debug_log(&format!(
                "extract_string_literal OK: {}",
                raw_type_name
            ));
            let value = drive_legacy_expression_v1(builder, port, operand)?;
            let ty = MirBuilder::parse_type_name_to_mir(&raw_type_name);
            let dst = builder.next_value_id();
            let value_label = match origin {
                PreparedRawPrintTypeOpOriginV1::FunctionWrapper => "value",
                PreparedRawPrintTypeOpOriginV1::Method => "obj",
            };
            super::super::utils::builder_debug_log(&format!(
                "emit TypeOp {:?} {}={} dst= {}",
                op, value_label, value, dst
            ));
            builder.emit_instruction(MirInstruction::TypeOp { dst, op, value, ty })?;
            builder.emit_extern_call("env.console", "log", vec![dst], None)?;
            Ok(dst)
        }
        PreparedRawPrintRouteV1::General { expression, trace } => {
            replay_general_print_trace(trace);
            let value = drive_legacy_expression_v1(builder, port, expression)?;
            super::super::utils::builder_debug_log(&format!("general print value={}", value));
            build_print_from_value(builder, value)
        }
    }
}

fn replay_general_print_trace(trace: PreparedRawPrintGeneralTraceV1) {
    match trace {
        PreparedRawPrintGeneralTraceV1::None => {}
        PreparedRawPrintGeneralTraceV1::FunctionTypeOpMissingLiteral => {
            super::super::utils::builder_debug_log(
                "pattern: print(FunctionCall isType|asType) [via wrapper]",
            );
            super::super::utils::builder_debug_log("extract_string_literal FAIL [via wrapper]");
            super::super::utils::builder_debug_log("pattern: print(FunctionCall isType|asType)");
            super::super::utils::builder_debug_log("extract_string_literal FAIL");
        }
        PreparedRawPrintGeneralTraceV1::MethodTypeOpMissingLiteral => {
            super::super::utils::builder_debug_log("pattern: print(MethodCall is|as)");
            super::super::utils::builder_debug_log("extract_string_literal FAIL");
        }
    }
}

/*
 * General Print emission is also used by the typed root-body owner, so it
 * deliberately remains separate from the raw source-route product above.
 */
pub(in crate::mir::builder) fn build_print_from_value(
    builder: &mut MirBuilder,
    value: ValueId,
) -> Result<ValueId, String> {
    // Phase 3.2: Use unified call for print statements
    let use_unified = super::super::calls::call_unified::is_unified_call_enabled();

    if use_unified {
        // Unified path: treat print as global function.
        builder.emit_unified_call(None, CallTarget::Global("print".to_string()), vec![value])?;
    } else {
        // Compatibility path when unified calls are disabled.
        builder.emit_extern_call("env.console", "log", vec![value], None)?;
    }
    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::{
        lower_prepared_raw_print_with_port_v1, PreparedRawPrintGeneralTraceV1,
        PreparedRawPrintRouteV1, PreparedRawPrintTypeOpOriginV1, PreparedRawPrintV1,
    };
    use crate::ast::{ASTNode, LiteralValue, Span};
    use crate::mir::builder::recursive_child_lowering::RawLegacyChildLoweringPortV1;
    use crate::mir::builder::MirBuilder;
    use crate::mir::definitions::call_unified::Callee;
    use crate::mir::{MirInstruction, TypeOpKind};

    fn span() -> Span {
        Span::unknown()
    }

    fn integer(value: i64) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::Integer(value),
            span: span(),
        }
    }

    fn string(value: &str) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::String(value.to_string()),
            span: span(),
        }
    }

    fn function_type_op(type_name: ASTNode) -> ASTNode {
        ASTNode::FunctionCall {
            name: "isType".to_string(),
            arguments: vec![integer(1), type_name],
            span: span(),
        }
    }

    fn method_type_op(type_name: ASTNode) -> ASTNode {
        ASTNode::MethodCall {
            object: Box::new(integer(1)),
            method: "as".to_string(),
            arguments: vec![type_name],
            span: span(),
        }
    }

    #[test]
    fn print_source_route_is_total_and_preserves_legacy_trace_profile() {
        let function = PreparedRawPrintV1::prepare(function_type_op(string("Integer")));
        assert!(matches!(
            function.route,
            PreparedRawPrintRouteV1::TypeOp {
                origin: PreparedRawPrintTypeOpOriginV1::FunctionWrapper,
                op: TypeOpKind::Check,
                ..
            }
        ));

        let method = PreparedRawPrintV1::prepare(method_type_op(string("Integer")));
        assert!(matches!(
            method.route,
            PreparedRawPrintRouteV1::TypeOp {
                origin: PreparedRawPrintTypeOpOriginV1::Method,
                op: TypeOpKind::Cast,
                ..
            }
        ));

        let function_miss = PreparedRawPrintV1::prepare(function_type_op(integer(2)));
        assert!(matches!(
            function_miss.route,
            PreparedRawPrintRouteV1::General {
                trace: PreparedRawPrintGeneralTraceV1::FunctionTypeOpMissingLiteral,
                ..
            }
        ));
        let method_miss = PreparedRawPrintV1::prepare(method_type_op(integer(2)));
        assert!(matches!(
            method_miss.route,
            PreparedRawPrintRouteV1::General {
                trace: PreparedRawPrintGeneralTraceV1::MethodTypeOpMissingLiteral,
                ..
            }
        ));
        let ordinary = PreparedRawPrintV1::prepare(integer(3));
        assert!(matches!(
            ordinary.route,
            PreparedRawPrintRouteV1::General {
                trace: PreparedRawPrintGeneralTraceV1::None,
                ..
            }
        ));
    }

    #[test]
    fn prepared_function_type_op_emits_typeop_then_extern_print() {
        let mut builder = MirBuilder::new();
        builder.enter_function_for_test("prepared_print_typeop/0".to_string());
        let prepared = PreparedRawPrintV1::prepare(function_type_op(string("Integer")));
        let mut port = RawLegacyChildLoweringPortV1;

        lower_prepared_raw_print_with_port_v1(&mut builder, &mut port, prepared).unwrap();

        let function = builder.function_state.current_function.as_ref().unwrap();
        let instructions = function
            .blocks
            .values()
            .flat_map(|block| block.all_instructions())
            .collect::<Vec<_>>();
        let typeop = instructions
            .iter()
            .position(|inst| matches!(inst, MirInstruction::TypeOp { .. }))
            .expect("prepared Print TypeOp");
        let print = instructions
            .iter()
            .position(|inst| {
                matches!(
                    inst,
                    MirInstruction::Call {
                        callee: Some(Callee::Extern(name)),
                        ..
                    } if name == "env.console.log"
                )
            })
            .expect("prepared Print extern terminal");
        assert!(
            typeop < print,
            "TypeOp must precede the extern Print terminal"
        );
    }
}
