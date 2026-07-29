//! Builder adapter for the shared source-method reserved-route policy.
//!
//! This module projects the active FastMem session into the neutral context,
//! consumes one typed decision, and delegates only selected execution.

use crate::ast::ASTNode;
use crate::mir::policies::source_method_reserved_route::{
    classify_source_method_reserved_route_v1, SourceMethodReservedRouteContextV1,
    SourceMethodReservedRouteDecisionV1, SourceMethodReservedRouteFailureV1,
};
use crate::mir::{MirBuilder, ValueId};

use super::method_call_descent::{
    lower_method_call_argument_v1, lower_method_call_arguments_v1, MethodCallDescentPortV1,
};

pub(super) enum ReservedMethodCallOutcomeV1 {
    Ordinary,
    Emitted(ValueId),
}

enum PreparedReservedMethodCallV1 {
    Ordinary,
    FastMem {
        region: crate::mir::instruction::FastMemRegionId,
        intrinsic: crate::mir::builder::fastmem::calls::PreparedFastMemIntrinsicV1,
    },
    MirDebug {
        method: crate::mir::policies::source_method_reserved_route::MirDebugMethodV1,
        label: Box<str>,
    },
    ReplIntrinsic {
        method: crate::mir::policies::source_method_reserved_route::ReplIntrinsicMethodV1,
    },
    ReservedFail(SourceMethodReservedRouteFailureV1),
}

fn prepare_reserved_method_call_v1(
    region: Option<crate::mir::instruction::FastMemRegionId>,
    object: &ASTNode,
    method: &str,
    arguments: &[ASTNode],
) -> Result<PreparedReservedMethodCallV1, String> {
    let context = if region.is_some() {
        SourceMethodReservedRouteContextV1::FastMemBody
    } else {
        SourceMethodReservedRouteContextV1::Ordinary
    };
    match classify_source_method_reserved_route_v1(context, object, method, arguments) {
        SourceMethodReservedRouteDecisionV1::Ordinary => Ok(PreparedReservedMethodCallV1::Ordinary),
        SourceMethodReservedRouteDecisionV1::FastMem => {
            let region = region.ok_or_else(|| {
                "[freeze:contract][source-method-route/fastmem-context-missing]".to_string()
            })?;
            let name = format!("mem.{method}");
            let intrinsic =
                crate::mir::builder::fastmem::calls::PreparedFastMemIntrinsicV1::prepare(
                    &name,
                    arguments.len(),
                );
            Ok(PreparedReservedMethodCallV1::FastMem { region, intrinsic })
        }
        SourceMethodReservedRouteDecisionV1::MirDebug { method, label } => {
            Ok(PreparedReservedMethodCallV1::MirDebug { method, label })
        }
        SourceMethodReservedRouteDecisionV1::ReplIntrinsic { method } => {
            Ok(PreparedReservedMethodCallV1::ReplIntrinsic { method })
        }
        SourceMethodReservedRouteDecisionV1::ReservedFail(reason) => {
            Ok(PreparedReservedMethodCallV1::ReservedFail(reason))
        }
    }
}

pub(super) fn build_reserved_method_call_v1<Port>(
    builder: &mut MirBuilder,
    port: &mut Port,
    input: &Port::MethodCallInput,
) -> Result<ReservedMethodCallOutcomeV1, String>
where
    Port: MethodCallDescentPortV1,
{
    let syntax = port.method_call_syntax(input)?;
    let prepared = prepare_reserved_method_call_v1(
        builder.current_fastmem_region(),
        syntax.receiver(),
        syntax.method(),
        syntax.arguments(),
    )?;
    match prepared {
        PreparedReservedMethodCallV1::Ordinary => Ok(ReservedMethodCallOutcomeV1::Ordinary),
        PreparedReservedMethodCallV1::FastMem { region, intrinsic } => {
            let value =
                crate::mir::builder::fastmem::calls::lower_prepared_fastmem_method_call_with_port_v1(
                    builder,
                    region,
                    intrinsic,
                    syntax.arguments(),
                    port,
                    input,
                )?;
            Ok(ReservedMethodCallOutcomeV1::Emitted(value))
        }
        PreparedReservedMethodCallV1::MirDebug { method, label } => {
            let mut values = Vec::new();
            if method == crate::mir::policies::source_method_reserved_route::MirDebugMethodV1::Log {
                for index in 1..syntax.arguments().len() {
                    let value = lower_method_call_argument_v1(builder, port, input, index)?;
                    builder.observe_selected_mir_debug_argument(
                        &syntax.arguments()[index],
                        index - 1,
                        value,
                    );
                    values.push(value);
                }
            }
            let value = builder.build_selected_mir_debug_call(method, &label, values)?;
            Ok(ReservedMethodCallOutcomeV1::Emitted(value))
        }
        PreparedReservedMethodCallV1::ReplIntrinsic { method } => {
            let arguments = lower_method_call_arguments_v1(builder, port, input)?;
            let value = builder.build_selected_repl_method_call(method, arguments)?;
            Ok(ReservedMethodCallOutcomeV1::Emitted(value))
        }
        PreparedReservedMethodCallV1::ReservedFail(reason) => match reason {
            SourceMethodReservedRouteFailureV1::MirDebugLabelRequired => {
                Err("__mir__.log/__mir__.mark requires at least a label argument".to_string())
            }
            SourceMethodReservedRouteFailureV1::UnsupportedReplMethod => Err(format!(
                "__repl.{} is not supported. Only __repl.get and __repl.set are allowed.",
                syntax.method()
            )),
        },
    }
}
