//! One pre-effect route for a raw direct `FunctionCall`.
//!
//! This owner observes source plus the read-only Brand/FastMem context once.
//! It does not descend children or mutate the Builder while selecting a route.

use super::super::recursive_child_lowering::{
    drive_legacy_expression_v1, RawAstChildLoweringPortV1, RawFunctionHeaderLookupPortV1,
};
use super::super::{EffectMask, MirBuilder, MirInstruction, MirType, ValueId};
use crate::ast::ASTNode;
use crate::mir::instruction::FastMemRegionId;
use crate::mir::TypeOpKind;

pub(in crate::mir::builder) struct PreparedRawFunctionPreflightV1 {
    name: String,
    route: PreparedRawFunctionPreflightRouteV1,
}

enum PreparedRawFunctionPreflightRouteV1 {
    WeakReject,
    ExplicitExtern(PreparedRawExplicitExternCallV1),
    Brand(PreparedRawBrandConstructorV1),
    TypeOp {
        operand: ASTNode,
        raw_type_name: String,
        op: TypeOpKind,
    },
    Math {
        arguments: Vec<super::special_method_handlers::PreparedRawMathArgumentV1>,
    },
    FastMem {
        region: FastMemRegionId,
        arguments: Vec<ASTNode>,
        intrinsic: crate::mir::builder::fastmem::calls::PreparedFastMemIntrinsicV1,
    },
    Ordinary {
        completion: PreparedRawOrdinaryFunctionCompletionV1,
    },
}

pub(super) enum PreparedRawOrdinaryFunctionCompletionV1 {
    StrNormalization { argument: ASTNode },
    Resolved { arguments: Vec<ASTNode> },
}

enum PreparedRawExplicitExternCallV1 {
    MissingTarget,
    TargetMustBeString,
    Ready {
        iface_name: String,
        method_name: String,
        return_type: MirType,
        arguments: Vec<ASTNode>,
    },
}

enum PreparedRawBrandConstructorV1 {
    ArityMismatch { actual: usize },
    Ready { argument: ASTNode },
}

impl PreparedRawBrandConstructorV1 {
    fn prepare(arguments: Vec<ASTNode>) -> Self {
        if arguments.len() != 1 {
            return Self::ArityMismatch {
                actual: arguments.len(),
            };
        }
        Self::Ready {
            argument: arguments
                .into_iter()
                .next()
                .expect("exact-one Brand constructor retains one argument"),
        }
    }
}

impl PreparedRawExplicitExternCallV1 {
    fn prepare(arguments: Vec<ASTNode>) -> Self {
        let Some(target) = arguments.first() else {
            return Self::MissingTarget;
        };
        let Some(extern_name) = super::special_handlers::extract_string_literal(target) else {
            return Self::TargetMustBeString;
        };
        let return_type = super::extern_calls::explicit_extern_return_type(&extern_name);
        let (iface_name, method_name) =
            super::extern_calls::split_explicit_extern_name(&extern_name);
        Self::Ready {
            iface_name,
            method_name,
            return_type,
            arguments: arguments.into_iter().skip(1).collect(),
        }
    }
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
            PreparedRawFunctionPreflightRouteV1::ExplicitExtern(
                PreparedRawExplicitExternCallV1::prepare(arguments),
            )
        } else if builder.comp_ctx.is_brand_declared(&name) {
            PreparedRawFunctionPreflightRouteV1::Brand(PreparedRawBrandConstructorV1::prepare(
                arguments,
            ))
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
            PreparedRawFunctionPreflightRouteV1::Math {
                arguments: super::special_method_handlers::prepare_raw_math_arguments_v1(arguments),
            }
        } else if let Some(region) = builder.current_fastmem_region() {
            if name.starts_with("mem.") {
                let intrinsic =
                    crate::mir::builder::fastmem::calls::PreparedFastMemIntrinsicV1::prepare(
                        &name,
                        arguments.len(),
                    );
                PreparedRawFunctionPreflightRouteV1::FastMem {
                    region,
                    arguments,
                    intrinsic,
                }
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
        PreparedRawFunctionPreflightRouteV1::ExplicitExtern(explicit) => {
            lower_prepared_raw_explicit_extern_call_with_port_v1(builder, port, explicit)
        }
        PreparedRawFunctionPreflightRouteV1::Brand(brand) => {
            lower_prepared_raw_brand_constructor_with_port_v1(builder, port, prepared.name, brand)
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
        PreparedRawFunctionPreflightRouteV1::FastMem {
            region,
            arguments,
            intrinsic,
        } => {
            crate::mir::builder::fastmem::calls::lower_prepared_fastmem_function_call_with_port_v1(
                builder, region, intrinsic, arguments, port,
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

fn lower_prepared_raw_brand_constructor_with_port_v1<Port>(
    builder: &mut MirBuilder,
    port: &mut Port,
    name: String,
    prepared: PreparedRawBrandConstructorV1,
) -> Result<ValueId, String>
where
    Port: RawAstChildLoweringPortV1,
{
    let argument = match prepared {
        PreparedRawBrandConstructorV1::ArityMismatch { actual } => {
            return Err(format!(
                "[brand/constructor-arity] {} expects exactly one value, got {}",
                name, actual
            ));
        }
        PreparedRawBrandConstructorV1::Ready { argument } => argument,
    };
    drive_legacy_expression_v1(builder, port, argument)
}

fn lower_prepared_raw_explicit_extern_call_with_port_v1<Port>(
    builder: &mut MirBuilder,
    port: &mut Port,
    prepared: PreparedRawExplicitExternCallV1,
) -> Result<ValueId, String>
where
    Port: RawAstChildLoweringPortV1,
{
    let (iface_name, method_name, return_type, arguments) = match prepared {
        PreparedRawExplicitExternCallV1::MissingTarget => {
            return Err(
                "externcall requires a target string literal: externcall \"name\"(...)".to_string(),
            )
        }
        PreparedRawExplicitExternCallV1::TargetMustBeString => {
            return Err(
                "externcall target must be a string literal: externcall \"name\"(...)".to_string(),
            )
        }
        PreparedRawExplicitExternCallV1::Ready {
            iface_name,
            method_name,
            return_type,
            arguments,
        } => (iface_name, method_name, return_type, arguments),
    };
    let arg_values = super::drive_call_arguments_v1(builder, port, arguments.as_slice())?;
    let dst = builder.next_value_id();
    builder.emit_extern_call_with_effects(
        &iface_name,
        &method_name,
        arg_values,
        Some(dst),
        EffectMask::IO,
    )?;
    builder
        .function_state
        .type_ctx
        .value_types
        .insert(dst, return_type);
    Ok(dst)
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
#[path = "function_call_preflight_route_tests.rs"]
mod tests;
