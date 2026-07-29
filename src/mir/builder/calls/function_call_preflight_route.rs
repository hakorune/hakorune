//! One pre-effect route for a raw direct `FunctionCall`.
//!
//! This owner observes source plus the read-only Brand/FastMem context once.
//! It does not descend children or mutate the Builder while selecting a route.

use super::super::recursive_child_lowering::{
    drive_legacy_expression_v1, RawAstChildLoweringPortV1, RawFunctionHeaderLookupPortV1,
};
use super::super::{MirBuilder, MirInstruction, ValueId};
use crate::ast::ASTNode;
use crate::mir::instruction::FastMemRegionId;
use crate::mir::TypeOpKind;

pub(in crate::mir::builder) struct PreparedRawFunctionPreflightV1 {
    name: String,
    route: PreparedRawFunctionPreflightRouteV1,
}

enum PreparedRawFunctionPreflightRouteV1 {
    WeakReject,
    ExplicitExtern {
        arguments: Vec<ASTNode>,
    },
    Brand {
        arguments: Vec<ASTNode>,
    },
    TypeOp {
        operand: ASTNode,
        raw_type_name: String,
        op: TypeOpKind,
    },
    Math {
        arguments: Vec<ASTNode>,
    },
    FastMem {
        region: FastMemRegionId,
        arguments: Vec<ASTNode>,
    },
    Ordinary {
        completion: PreparedRawOrdinaryFunctionCompletionV1,
    },
}

pub(super) enum PreparedRawOrdinaryFunctionCompletionV1 {
    StrNormalization { argument: ASTNode },
    Resolved { arguments: Vec<ASTNode> },
}

impl PreparedRawFunctionPreflightV1 {
    pub(in crate::mir::builder) fn prepare(
        builder: &MirBuilder,
        name: String,
        arguments: Vec<ASTNode>,
    ) -> Self {
        let route = if name == "weak" {
            PreparedRawFunctionPreflightRouteV1::WeakReject
        } else if name == "externcall" {
            PreparedRawFunctionPreflightRouteV1::ExplicitExtern { arguments }
        } else if builder.comp_ctx.is_brand_declared(&name) {
            PreparedRawFunctionPreflightRouteV1::Brand { arguments }
        } else if let Some((raw_type_name, op)) = prepare_typeop_route(&name, arguments.as_slice())
        {
            let mut arguments = arguments.into_iter();
            let operand = arguments
                .next()
                .expect("TypeOp route requires exactly two arguments");
            PreparedRawFunctionPreflightRouteV1::TypeOp {
                operand,
                raw_type_name,
                op,
            }
        } else if super::special_handlers::is_math_function(&name) {
            PreparedRawFunctionPreflightRouteV1::Math { arguments }
        } else if let Some(region) = builder.current_fastmem_region() {
            if name.starts_with("mem.") {
                PreparedRawFunctionPreflightRouteV1::FastMem { region, arguments }
            } else {
                PreparedRawFunctionPreflightRouteV1::Ordinary {
                    completion: prepare_ordinary_function_completion_v1(&name, arguments),
                }
            }
        } else {
            PreparedRawFunctionPreflightRouteV1::Ordinary {
                completion: prepare_ordinary_function_completion_v1(&name, arguments),
            }
        };
        Self { name, route }
    }
}

fn prepare_ordinary_function_completion_v1(
    name: &str,
    arguments: Vec<ASTNode>,
) -> PreparedRawOrdinaryFunctionCompletionV1 {
    if name == "str" && arguments.len() == 1 {
        PreparedRawOrdinaryFunctionCompletionV1::StrNormalization {
            argument: arguments
                .into_iter()
                .next()
                .expect("exact str/1 route must retain one argument"),
        }
    } else {
        PreparedRawOrdinaryFunctionCompletionV1::Resolved { arguments }
    }
}

fn prepare_typeop_route(name: &str, arguments: &[ASTNode]) -> Option<(String, TypeOpKind)> {
    if (name != "isType" && name != "asType") || arguments.len() != 2 {
        return None;
    }
    let raw_type_name = super::special_handlers::extract_string_literal(&arguments[1])?;
    let op = if name == "isType" {
        TypeOpKind::Check
    } else {
        TypeOpKind::Cast
    };
    Some((raw_type_name, op))
}

pub(in crate::mir::builder) fn lower_prepared_raw_function_preflight_with_port_v1<Port>(
    builder: &mut MirBuilder,
    port: &mut Port,
    prepared: PreparedRawFunctionPreflightV1,
) -> Result<ValueId, String>
where
    Port: RawAstChildLoweringPortV1 + RawFunctionHeaderLookupPortV1,
{
    replay_function_call_trace(builder, &prepared.name);
    match prepared.route {
        PreparedRawFunctionPreflightRouteV1::WeakReject => {
            let ring0 = crate::runtime::get_global_ring0();
            ring0
                .log
                .error("[Phase285W-0.1] Rejecting weak(...) function call");
            Err(
                "Invalid syntax: weak(...). Use unary operator: weak <expr>\n\
                 Help: Change 'weak(obj)' to 'weak obj' (unary operator, no parentheses)\n\
                 SSOT: docs/reference/language/lifecycle.md"
                    .to_string(),
            )
        }
        PreparedRawFunctionPreflightRouteV1::ExplicitExtern { arguments } => {
            builder.build_explicit_extern_call_with_port_v1(port, arguments)
        }
        PreparedRawFunctionPreflightRouteV1::Brand { arguments } => {
            builder.build_brand_constructor_call_with_port_v1(port, prepared.name, arguments)
        }
        PreparedRawFunctionPreflightRouteV1::TypeOp {
            operand,
            raw_type_name,
            op,
        } => {
            let value = drive_legacy_expression_v1(builder, port, operand)?;
            let ty = super::special_handlers::parse_type_name_to_mir(&raw_type_name);
            let dst = builder.next_value_id();
            builder.emit_instruction(MirInstruction::TypeOp { dst, op, value, ty })?;
            Ok(dst)
        }
        PreparedRawFunctionPreflightRouteV1::Math { arguments } => {
            builder.lower_math_function_with_port_v1(port, prepared.name, arguments)
        }
        PreparedRawFunctionPreflightRouteV1::FastMem { region, arguments } => {
            crate::mir::builder::fastmem::calls::lower_fastmem_function_call_with_port_v1(
                builder,
                region,
                prepared.name,
                arguments,
                port,
            )
        }
        PreparedRawFunctionPreflightRouteV1::Ordinary { completion } => builder
            .lower_prepared_raw_ordinary_function_completion_with_port_v1(
                port,
                prepared.name,
                completion,
            ),
    }
}

fn replay_function_call_trace(builder: &MirBuilder, name: &str) {
    if !crate::config::env::cli_verbose() {
        return;
    }
    let current_function = builder
        .function_state
        .current_function
        .as_ref()
        .map(|function| function.signature.name.as_str())
        .unwrap_or("<none>");
    let ring0 = crate::runtime::get_global_ring0();
    ring0.log.debug(&format!(
        "[builder] function-call name={} static_ctx={} in_fn={}",
        name,
        builder.comp_ctx.current_static_box.as_deref().unwrap_or(""),
        current_function
    ));
}

#[cfg(test)]
mod tests {
    use super::{
        lower_prepared_raw_function_preflight_with_port_v1, PreparedRawFunctionPreflightRouteV1,
        PreparedRawFunctionPreflightV1, PreparedRawOrdinaryFunctionCompletionV1,
    };
    use crate::ast::{ASTNode, LiteralValue, Span};
    use crate::mir::builder::recursive_child_lowering::{
        RawFunctionHeaderLookupPortV1, RecursiveChildLoweringPortV1,
    };
    use crate::mir::builder::MirBuilder;
    use crate::mir::instruction::FastMemRegionId;
    use crate::mir::{MirInstruction, TypeOpKind, ValueId};

    #[derive(Default)]
    struct RecordingPortV1 {
        expression_count: usize,
        events: Vec<&'static str>,
        fail_expression: bool,
    }

    impl RecursiveChildLoweringPortV1 for RecordingPortV1 {
        type BodyInput = Vec<ASTNode>;
        type StatementInput = ASTNode;
        type ExpressionInput = ASTNode;

        fn lower_body(
            &mut self,
            _builder: &mut MirBuilder,
            _input: Self::BodyInput,
        ) -> Result<ValueId, String> {
            unreachable!("FunctionCall route test does not lower a body")
        }

        fn lower_statement(
            &mut self,
            _builder: &mut MirBuilder,
            _input: Self::StatementInput,
        ) -> Result<ValueId, String> {
            unreachable!("FunctionCall route test does not lower a statement")
        }

        fn lower_expression(
            &mut self,
            builder: &mut MirBuilder,
            _input: Self::ExpressionInput,
        ) -> Result<ValueId, String> {
            self.expression_count += 1;
            self.events.push("child");
            if self.fail_expression {
                return Err("direct str child failed".to_owned());
            }
            crate::mir::builder::emission::constant::emit_integer(builder, 7)
        }
    }

    impl RawFunctionHeaderLookupPortV1 for RecordingPortV1 {
        fn with_function_headers<R>(
            &mut self,
            observe: impl for<'headers> FnOnce(
                Option<
                    &'headers dyn crate::mir::builder::function_signature_lookup::FunctionSignatureLookupV1,
                >,
            ) -> R,
        ) -> R {
            self.events.push("header");
            observe(None)
        }
    }

    fn literal(value: LiteralValue) -> ASTNode {
        ASTNode::Literal {
            value,
            span: Span::unknown(),
        }
    }

    fn integer(value: i64) -> ASTNode {
        literal(LiteralValue::Integer(value))
    }

    fn new_box(name: &str, arguments: Vec<ASTNode>) -> ASTNode {
        ASTNode::New {
            class: name.to_string(),
            type_arguments: Vec::new(),
            arguments,
            field_initializers: Vec::new(),
            span: Span::unknown(),
        }
    }

    #[test]
    fn direct_function_preflight_priority_is_total() {
        let mut builder = MirBuilder::new();
        builder
            .comp_ctx
            .register_brand_decl("sin".to_string(), "Integer".to_string());
        builder
            .comp_ctx
            .register_brand_decl("isType".to_string(), "Integer".to_string());
        builder
            .comp_ctx
            .register_brand_decl("mem.addr".to_string(), "Integer".to_string());
        builder
            .comp_ctx
            .register_brand_decl("str".to_string(), "Integer".to_string());
        builder.push_fastmem_region(FastMemRegionId::new(6));

        let weak =
            PreparedRawFunctionPreflightV1::prepare(&builder, "weak".to_string(), vec![integer(1)]);
        assert!(matches!(
            weak.route,
            PreparedRawFunctionPreflightRouteV1::WeakReject
        ));

        let explicit = PreparedRawFunctionPreflightV1::prepare(
            &builder,
            "externcall".to_string(),
            vec![integer(1)],
        );
        assert!(matches!(
            explicit.route,
            PreparedRawFunctionPreflightRouteV1::ExplicitExtern { .. }
        ));

        let brand =
            PreparedRawFunctionPreflightV1::prepare(&builder, "sin".to_string(), vec![integer(1)]);
        assert!(matches!(
            brand.route,
            PreparedRawFunctionPreflightRouteV1::Brand { .. }
        ));

        for (name, arguments) in [
            (
                "isType",
                vec![
                    integer(1),
                    literal(LiteralValue::String("Integer".to_string())),
                ],
            ),
            ("mem.addr", vec![integer(1)]),
            ("str", vec![integer(1)]),
        ] {
            let collision =
                PreparedRawFunctionPreflightV1::prepare(&builder, name.to_string(), arguments);
            assert!(matches!(
                collision.route,
                PreparedRawFunctionPreflightRouteV1::Brand { .. }
            ));
        }

        let mut builder = MirBuilder::new();
        let typeop = PreparedRawFunctionPreflightV1::prepare(
            &builder,
            "isType".to_string(),
            vec![
                integer(1),
                literal(LiteralValue::String("Integer".to_string())),
            ],
        );
        assert!(matches!(
            typeop.route,
            PreparedRawFunctionPreflightRouteV1::TypeOp { .. }
        ));

        let malformed_typeop = PreparedRawFunctionPreflightV1::prepare(
            &builder,
            "isType".to_string(),
            vec![integer(1), integer(2)],
        );
        assert!(matches!(
            malformed_typeop.route,
            PreparedRawFunctionPreflightRouteV1::Ordinary { .. }
        ));

        let math =
            PreparedRawFunctionPreflightV1::prepare(&builder, "sqrt".to_string(), vec![integer(4)]);
        assert!(matches!(
            math.route,
            PreparedRawFunctionPreflightRouteV1::Math { .. }
        ));

        let inactive_fastmem = PreparedRawFunctionPreflightV1::prepare(
            &builder,
            "mem.addr".to_string(),
            vec![integer(1)],
        );
        assert!(matches!(
            inactive_fastmem.route,
            PreparedRawFunctionPreflightRouteV1::Ordinary { .. }
        ));

        builder.push_fastmem_region(FastMemRegionId::new(7));
        let fastmem = PreparedRawFunctionPreflightV1::prepare(
            &builder,
            "mem.addr".to_string(),
            vec![integer(1)],
        );
        assert!(matches!(
            fastmem.route,
            PreparedRawFunctionPreflightRouteV1::FastMem { .. }
        ));

        let ordinary = PreparedRawFunctionPreflightV1::prepare(
            &builder,
            "user_function".to_string(),
            vec![integer(1)],
        );
        assert!(matches!(
            ordinary.route,
            PreparedRawFunctionPreflightRouteV1::Ordinary { .. }
        ));

        let str_one =
            PreparedRawFunctionPreflightV1::prepare(&builder, "str".to_string(), vec![integer(1)]);
        assert!(matches!(
            str_one.route,
            PreparedRawFunctionPreflightRouteV1::Ordinary {
                completion: PreparedRawOrdinaryFunctionCompletionV1::StrNormalization { .. }
            }
        ));
        for arguments in [Vec::new(), vec![integer(1), integer(2)]] {
            let wrong_arity =
                PreparedRawFunctionPreflightV1::prepare(&builder, "str".to_string(), arguments);
            assert!(matches!(
                wrong_arity.route,
                PreparedRawFunctionPreflightRouteV1::Ordinary {
                    completion: PreparedRawOrdinaryFunctionCompletionV1::Resolved { .. }
                }
            ));
        }
    }

    #[test]
    fn rejecting_routes_precede_children_and_typeop_uses_one_child() {
        let mut builder = MirBuilder::new();
        builder.enter_function_for_test("direct_preflight_order/0".to_string());
        builder
            .comp_ctx
            .register_brand_decl("Meter".to_string(), "Integer".to_string());
        let mut port = RecordingPortV1::default();

        for (name, arguments) in [
            ("externcall", vec![integer(1)]),
            ("Meter", vec![integer(1), integer(2)]),
        ] {
            let prepared =
                PreparedRawFunctionPreflightV1::prepare(&builder, name.to_string(), arguments);
            assert!(lower_prepared_raw_function_preflight_with_port_v1(
                &mut builder,
                &mut port,
                prepared,
            )
            .is_err());
            assert_eq!(port.expression_count, 0);
        }

        let typeop = PreparedRawFunctionPreflightV1::prepare(
            &builder,
            "asType".to_string(),
            vec![
                integer(1),
                literal(LiteralValue::String("Integer".to_string())),
            ],
        );
        lower_prepared_raw_function_preflight_with_port_v1(&mut builder, &mut port, typeop)
            .unwrap();
        assert_eq!(port.expression_count, 1);

        let malformed_typeop = PreparedRawFunctionPreflightV1::prepare(
            &builder,
            "isType".to_string(),
            vec![integer(1), integer(2)],
        );
        let _ = lower_prepared_raw_function_preflight_with_port_v1(
            &mut builder,
            &mut port,
            malformed_typeop,
        );
        assert_eq!(port.expression_count, 3);

        let inactive_fastmem = PreparedRawFunctionPreflightV1::prepare(
            &builder,
            "mem.addr".to_string(),
            vec![integer(1)],
        );
        let _ = lower_prepared_raw_function_preflight_with_port_v1(
            &mut builder,
            &mut port,
            inactive_fastmem,
        );
        assert_eq!(port.expression_count, 4);

        builder.push_fastmem_region(FastMemRegionId::new(8));
        let unknown_fastmem = PreparedRawFunctionPreflightV1::prepare(
            &builder,
            "mem.unknown".to_string(),
            vec![integer(1)],
        );
        let error = lower_prepared_raw_function_preflight_with_port_v1(
            &mut builder,
            &mut port,
            unknown_fastmem,
        )
        .unwrap_err();
        assert!(error.contains("[fastmem/forbidden_call]"));
        assert_eq!(port.expression_count, 4);
    }

    #[test]
    fn selected_math_and_ordinary_str_keep_child_and_completion_order() {
        let mut builder = MirBuilder::new();
        builder.enter_function_for_test("direct_preflight_completion/0".to_string());
        let mut port = RecordingPortV1::default();

        let math = PreparedRawFunctionPreflightV1::prepare(
            &builder,
            "sqrt".to_string(),
            vec![new_box("IntegerBox", vec![integer(9)])],
        );
        lower_prepared_raw_function_preflight_with_port_v1(&mut builder, &mut port, math).unwrap();
        assert_eq!(port.expression_count, 1);
        assert_eq!(port.events, vec!["child"]);
        assert!(builder
            .function_state
            .current_function
            .as_ref()
            .unwrap()
            .blocks
            .values()
            .flat_map(|block| block.all_instructions())
            .any(|instruction| matches!(
                instruction,
                MirInstruction::TypeOp {
                    op: TypeOpKind::Cast,
                    ..
                }
            )));

        let string =
            PreparedRawFunctionPreflightV1::prepare(&builder, "str".to_string(), vec![integer(1)]);
        lower_prepared_raw_function_preflight_with_port_v1(&mut builder, &mut port, string)
            .unwrap();
        assert_eq!(port.expression_count, 2);
        assert_eq!(port.events, vec!["child", "child"]);

        port.events.clear();
        let ordinary = PreparedRawFunctionPreflightV1::prepare(
            &builder,
            "user_function".to_string(),
            vec![integer(1)],
        );
        let _ =
            lower_prepared_raw_function_preflight_with_port_v1(&mut builder, &mut port, ordinary);
        assert_eq!(port.events, vec!["child", "header"]);
    }

    #[test]
    fn direct_str_child_failure_does_not_retry_or_observe_headers_and_reuses_builder() {
        let mut builder = MirBuilder::new();
        builder.enter_function_for_test("direct_str_failure_reuse/0".to_owned());
        let mut port = RecordingPortV1 {
            fail_expression: true,
            ..Default::default()
        };

        let failing =
            PreparedRawFunctionPreflightV1::prepare(&builder, "str".to_owned(), vec![integer(1)]);
        let error =
            lower_prepared_raw_function_preflight_with_port_v1(&mut builder, &mut port, failing)
                .unwrap_err();
        assert_eq!(error, "direct str child failed");
        assert_eq!(port.events, vec!["child"]);

        port.fail_expression = false;
        port.events.clear();
        let succeeding =
            PreparedRawFunctionPreflightV1::prepare(&builder, "str".to_owned(), vec![integer(2)]);
        lower_prepared_raw_function_preflight_with_port_v1(&mut builder, &mut port, succeeding)
            .unwrap();
        assert_eq!(port.events, vec!["child"]);
        assert_eq!(port.expression_count, 2);
    }
}
