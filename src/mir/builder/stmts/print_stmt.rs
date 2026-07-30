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
use crate::mir::builder::raw_structured_child_scope::{
    PreparedRawChildSourceV1, RawStructuredChildScopePortV1,
};
use crate::mir::builder::recursive_child_lowering::{
    drive_legacy_expression_v1, RecursiveChildLoweringPortV1,
};
use crate::mir::resolved_semantics::ExprChildRoleV1;
use crate::mir::TypeOpKind;

pub(in crate::mir::builder) struct PreparedRawPrintV1 {
    statement: ASTNode,
    route: PreparedRawPrintRouteV1,
}

enum PreparedRawPrintRouteV1 {
    TypeOp {
        origin: PreparedRawPrintTypeOpOriginV1,
        raw_type_name: String,
        op: TypeOpKind,
    },
    General {
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
    pub(in crate::mir::builder) fn prepare(statement: ASTNode) -> Result<Self, String> {
        let ASTNode::Print { expression, .. } = &statement else {
            return Err("[freeze:contract][raw-print/source-drift]".to_owned());
        };
        let observation = match &**expression {
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
                PreparedRawPrintRouteV1::TypeOp {
                    origin: PreparedRawPrintTypeOpOriginV1::FunctionWrapper,
                    raw_type_name,
                    op,
                }
            }
            PreparedRawPrintSourceObservationV1::MethodTypeOp { raw_type_name, op } => {
                PreparedRawPrintRouteV1::TypeOp {
                    origin: PreparedRawPrintTypeOpOriginV1::Method,
                    raw_type_name,
                    op,
                }
            }
            PreparedRawPrintSourceObservationV1::General(trace) => {
                PreparedRawPrintRouteV1::General { trace }
            }
        };
        Ok(Self { statement, route })
    }

    fn expression(&self) -> &ASTNode {
        let ASTNode::Print { expression, .. } = &self.statement else {
            unreachable!("prepared Print retains its exact statement")
        };
        expression
    }

    fn prepare_value_source_v1<Port>(
        &self,
        port: &mut Port,
    ) -> Result<PreparedRawChildSourceV1, String>
    where
        Port: RecursiveChildLoweringPortV1<
            BodyInput = Vec<ASTNode>,
            StatementInput = ASTNode,
            ExpressionInput = ASTNode,
        >,
    {
        let value =
            port.prepare_expression_child_source_v1(&self.statement, ExprChildRoleV1::PrintValue)?;
        let role = match self.route {
            PreparedRawPrintRouteV1::TypeOp {
                origin: PreparedRawPrintTypeOpOriginV1::FunctionWrapper,
                ..
            } => Some(ExprChildRoleV1::CallArgument(0)),
            PreparedRawPrintRouteV1::TypeOp {
                origin: PreparedRawPrintTypeOpOriginV1::Method,
                ..
            } => Some(ExprChildRoleV1::Receiver),
            PreparedRawPrintRouteV1::General { .. } => None,
        };
        match role {
            Some(role) => port.with_prepared_child_source_v1(value, |port| {
                port.prepare_expression_child_source_v1(self.expression(), role)
            }),
            None => Ok(value),
        }
    }
}

/// Prepare and consume the one exact Print value demand without route replay.
pub(in crate::mir::builder) fn lower_raw_print_statement_with_port_v1<Port>(
    builder: &mut MirBuilder,
    port: &mut Port,
    statement: ASTNode,
) -> Result<ValueId, String>
where
    Port: RecursiveChildLoweringPortV1<
        BodyInput = Vec<ASTNode>,
        StatementInput = ASTNode,
        ExpressionInput = ASTNode,
    >,
{
    let prepared = PreparedRawPrintV1::prepare(statement)?;
    let value_source = prepared.prepare_value_source_v1(port)?;
    let mut scoped = RawStructuredChildScopePortV1::new(port, vec![value_source], Vec::new());
    let value = lower_prepared_raw_print_with_port_v1(builder, &mut scoped, prepared)?;
    scoped.complete_exact_demands_v1()?;
    Ok(value)
}

fn lower_prepared_raw_print_with_port_v1<Port>(
    builder: &mut MirBuilder,
    port: &mut Port,
    prepared: PreparedRawPrintV1,
) -> Result<ValueId, String>
where
    Port: RecursiveChildLoweringPortV1<ExpressionInput = ASTNode>,
{
    super::super::utils::builder_debug_log("enter build_print_statement");
    let ASTNode::Print { expression, .. } = prepared.statement else {
        unreachable!("prepared Print retains its exact statement")
    };
    match prepared.route {
        PreparedRawPrintRouteV1::TypeOp {
            origin,
            raw_type_name,
            op,
        } => {
            let operand = match (origin, *expression) {
                (
                    PreparedRawPrintTypeOpOriginV1::FunctionWrapper,
                    ASTNode::FunctionCall { arguments, .. },
                ) => arguments
                    .into_iter()
                    .next()
                    .expect("Function TypeOp arity checked during preparation"),
                (PreparedRawPrintTypeOpOriginV1::Method, ASTNode::MethodCall { object, .. }) => {
                    *object
                }
                _ => unreachable!("prepared Print TypeOp route matches retained source"),
            };
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
        PreparedRawPrintRouteV1::General { trace } => {
            replay_general_print_trace(trace);
            let value = drive_legacy_expression_v1(builder, port, *expression)?;
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
    use std::cell::RefCell;

    use super::{
        lower_raw_print_statement_with_port_v1, PreparedRawPrintGeneralTraceV1,
        PreparedRawPrintRouteV1, PreparedRawPrintTypeOpOriginV1, PreparedRawPrintV1,
    };
    use crate::ast::{ASTNode, LiteralValue, Span};
    use crate::mir::builder::raw_structured_child_scope::PreparedRawChildSourceV1;
    use crate::mir::builder::recursive_child_lowering::{
        RawLegacyChildLoweringPortV1, RecursiveChildLoweringPortV1,
    };
    use crate::mir::builder::MirBuilder;
    use crate::mir::definitions::call_unified::Callee;
    use crate::mir::resolved_semantics::ExprChildRoleV1;
    use crate::mir::{MirInstruction, TypeOpKind, ValueId};

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

    fn print(expression: ASTNode) -> ASTNode {
        ASTNode::Print {
            expression: Box::new(expression),
            span: span(),
        }
    }

    #[derive(Default)]
    struct RecordingPrintSourcePortV1 {
        roles: RefCell<Vec<(&'static str, ExprChildRoleV1)>>,
        lowered: Vec<ASTNode>,
    }

    impl RecursiveChildLoweringPortV1 for RecordingPrintSourcePortV1 {
        type BodyInput = Vec<ASTNode>;
        type StatementInput = ASTNode;
        type ExpressionInput = ASTNode;

        fn lower_body(
            &mut self,
            _builder: &mut MirBuilder,
            _input: Self::BodyInput,
        ) -> Result<ValueId, String> {
            Err("unexpected Print body demand".to_owned())
        }

        fn lower_statement(
            &mut self,
            _builder: &mut MirBuilder,
            _input: Self::StatementInput,
        ) -> Result<ValueId, String> {
            Err("unexpected Print statement demand".to_owned())
        }

        fn lower_expression(
            &mut self,
            builder: &mut MirBuilder,
            input: Self::ExpressionInput,
        ) -> Result<ValueId, String> {
            self.lowered.push(input);
            crate::mir::builder::emission::constant::emit_integer(builder, 1)
        }

        fn prepare_expression_child_source_v1(
            &self,
            parent: &ASTNode,
            role: ExprChildRoleV1,
        ) -> Result<PreparedRawChildSourceV1, String> {
            self.roles.borrow_mut().push((parent.node_type(), role));
            Ok(PreparedRawChildSourceV1::Preserve)
        }
    }

    #[test]
    fn print_source_route_is_total_and_preserves_legacy_trace_profile() {
        let function =
            PreparedRawPrintV1::prepare(print(function_type_op(string("Integer")))).unwrap();
        assert!(matches!(
            function.route,
            PreparedRawPrintRouteV1::TypeOp {
                origin: PreparedRawPrintTypeOpOriginV1::FunctionWrapper,
                op: TypeOpKind::Check,
                ..
            }
        ));

        let method = PreparedRawPrintV1::prepare(print(method_type_op(string("Integer")))).unwrap();
        assert!(matches!(
            method.route,
            PreparedRawPrintRouteV1::TypeOp {
                origin: PreparedRawPrintTypeOpOriginV1::Method,
                op: TypeOpKind::Cast,
                ..
            }
        ));

        let function_miss =
            PreparedRawPrintV1::prepare(print(function_type_op(integer(2)))).unwrap();
        assert!(matches!(
            function_miss.route,
            PreparedRawPrintRouteV1::General {
                trace: PreparedRawPrintGeneralTraceV1::FunctionTypeOpMissingLiteral,
                ..
            }
        ));
        let method_miss = PreparedRawPrintV1::prepare(print(method_type_op(integer(2)))).unwrap();
        assert!(matches!(
            method_miss.route,
            PreparedRawPrintRouteV1::General {
                trace: PreparedRawPrintGeneralTraceV1::MethodTypeOpMissingLiteral,
                ..
            }
        ));
        let ordinary = PreparedRawPrintV1::prepare(print(integer(3))).unwrap();
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
        let mut port = RawLegacyChildLoweringPortV1;

        lower_raw_print_statement_with_port_v1(
            &mut builder,
            &mut port,
            print(function_type_op(string("Integer"))),
        )
        .unwrap();

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

    #[test]
    fn print_routes_consume_one_exact_source_demand() {
        let cases = [
            (
                print(integer(3)),
                vec![("Print", ExprChildRoleV1::PrintValue)],
                "Literal",
            ),
            (
                print(function_type_op(string("Integer"))),
                vec![
                    ("Print", ExprChildRoleV1::PrintValue),
                    ("FunctionCall", ExprChildRoleV1::CallArgument(0)),
                ],
                "Literal",
            ),
            (
                print(function_type_op(integer(2))),
                vec![("Print", ExprChildRoleV1::PrintValue)],
                "FunctionCall",
            ),
            (
                print(method_type_op(string("Integer"))),
                vec![
                    ("Print", ExprChildRoleV1::PrintValue),
                    ("MethodCall", ExprChildRoleV1::Receiver),
                ],
                "Literal",
            ),
        ];

        for (statement, expected_roles, expected_lowered) in cases {
            let mut builder = MirBuilder::new();
            builder.enter_function_for_test("print_source_demand/0".to_owned());
            let mut port = RecordingPrintSourcePortV1::default();
            lower_raw_print_statement_with_port_v1(&mut builder, &mut port, statement).unwrap();
            assert_eq!(*port.roles.borrow(), expected_roles);
            assert_eq!(port.lowered.len(), 1);
            assert_eq!(port.lowered[0].node_type(), expected_lowered);
            assert!(
                !matches!(
                    &port.lowered[0],
                    ASTNode::Literal {
                        value: LiteralValue::String(_),
                        ..
                    }
                ),
                "TypeOp type literal must remain syntax-only"
            );
        }
    }
}
