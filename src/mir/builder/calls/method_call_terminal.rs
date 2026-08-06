//! Value-only terminal boundary for source MethodCall lowering.
//!
//! Route selection, syntax preflight, and child descent must be complete before
//! this port is called. The port owns no route, target, effect, result, located
//! source, or caller-ledger authority. Its raw implementation only preserves
//! the existing legacy terminal operations.

use super::extern_calls::EnvMethodSpec;
use super::method_call_descent::{
    AssociatedMethodCallArgumentsV1, MethodCallArgumentDescentV1, MethodCallDescentPortV1,
};
use super::unified_emitter::{
    CompletedUnifiedValueCallEmissionV1, UnifiedCallEmitterBox, UnifiedValueCallReceiptErrorV1,
};
use super::CallTarget;
use crate::mir::builder::recursive_child_lowering::{
    RawAstChildLoweringPortV1, RawFunctionHeaderLookupPortV1,
};
use crate::mir::{MirBuilder, MirInstruction, MirType, TypeOpKind, ValueId};

pub(in crate::mir::builder) trait MethodCallValueTerminalPortV1 {
    fn emit_typeop_value_terminal(
        &mut self,
        builder: &mut MirBuilder,
        value: ValueId,
        op: TypeOpKind,
        ty: MirType,
    ) -> Result<ValueId, String>;

    fn emit_static_global_value_terminal(
        &mut self,
        builder: &mut MirBuilder,
        owner: &str,
        method: &str,
        checked_source_arity: u32,
        arguments: Vec<ValueId>,
    ) -> Result<ValueId, String>;

    fn emit_me_lowered_global_value_terminal(
        &mut self,
        builder: &mut MirBuilder,
        owner: &str,
        method: &str,
        checked_source_arity: u32,
        arguments: Vec<ValueId>,
    ) -> Result<ValueId, String>;

    fn emit_env_value_terminal(
        &mut self,
        builder: &mut MirBuilder,
        spec: &EnvMethodSpec,
        arguments: Vec<ValueId>,
    ) -> Result<ValueId, String>;

    fn emit_standard_value_terminal(
        &mut self,
        builder: &mut MirBuilder,
        receiver: ValueId,
        method: String,
        arguments: Vec<ValueId>,
    ) -> Result<ValueId, String>;
}

/// Source-neutral completion capability for the one existing static-call
/// handler.
///
/// It combines the existing argument-descent authority with only the static
/// value terminal needed by that handler. Route selection and source identity
/// stay outside this interface.
pub(in crate::mir::builder) trait StaticMethodCallCompletionV1:
    MethodCallArgumentDescentV1
{
    fn finish_static_global_value_terminal(
        &mut self,
        builder: &mut MirBuilder,
        owner: &str,
        method: &str,
        checked_source_arity: u32,
        arguments: Vec<ValueId>,
    ) -> Result<ValueId, String>;
}

/// Source-neutral completion capability for standard value calls.
///
/// Associated calls retain header-aware completion; materialized property
/// getters retain the raw `lookup=None` terminal.
pub(in crate::mir::builder) trait StandardMethodCallCompletionV1:
    MethodCallArgumentDescentV1
{
    fn finish_standard_value_terminal(
        &mut self,
        builder: &mut MirBuilder,
        receiver: ValueId,
        method: String,
        arguments: Vec<ValueId>,
    ) -> Result<ValueId, String>;
}

impl<Port> MethodCallValueTerminalPortV1 for Port
where
    Port: RawAstChildLoweringPortV1 + RawFunctionHeaderLookupPortV1,
{
    fn emit_typeop_value_terminal(
        &mut self,
        builder: &mut MirBuilder,
        value: ValueId,
        op: TypeOpKind,
        ty: MirType,
    ) -> Result<ValueId, String> {
        emit_typeop_value_terminal_raw_v1(builder, value, op, ty)
    }

    fn emit_static_global_value_terminal(
        &mut self,
        builder: &mut MirBuilder,
        owner: &str,
        method: &str,
        checked_source_arity: u32,
        arguments: Vec<ValueId>,
    ) -> Result<ValueId, String> {
        if let Some(value) = self.try_emit_source_bound_static_call_result_v1(
            builder,
            owner,
            method,
            checked_source_arity,
            &arguments,
        )? {
            return Ok(value);
        }
        self.with_function_headers(|lookup| {
            emit_global_value_terminal_with_lookup_v1(
                builder,
                owner,
                method,
                checked_source_arity,
                arguments,
                lookup,
            )
            .map(|(value, _)| value)
        })
    }

    fn emit_me_lowered_global_value_terminal(
        &mut self,
        builder: &mut MirBuilder,
        owner: &str,
        method: &str,
        checked_source_arity: u32,
        arguments: Vec<ValueId>,
    ) -> Result<ValueId, String> {
        if let Some(value) = self.try_emit_source_bound_static_call_result_v1(
            builder,
            owner,
            method,
            checked_source_arity,
            &arguments,
        )? {
            return Ok(value);
        }
        self.with_function_headers(|lookup| {
            let (value, target) = emit_global_value_terminal_with_lookup_v1(
                builder,
                owner,
                method,
                checked_source_arity,
                arguments,
                lookup,
            )?;
            if let Some(view) = lookup {
                crate::mir::builder::calls::annotation::annotate_call_result_from_func_name_with_lookup(
                    builder,
                    value,
                    &target,
                    Some(view),
                );
            } else {
                builder.annotate_call_result_from_func_name(value, &target);
            }
            Ok(value)
        })
    }

    fn emit_env_value_terminal(
        &mut self,
        builder: &mut MirBuilder,
        spec: &EnvMethodSpec,
        arguments: Vec<ValueId>,
    ) -> Result<ValueId, String> {
        emit_env_value_terminal_raw_v1(builder, spec, arguments)
    }

    fn emit_standard_value_terminal(
        &mut self,
        builder: &mut MirBuilder,
        receiver: ValueId,
        method: String,
        arguments: Vec<ValueId>,
    ) -> Result<ValueId, String> {
        self.with_function_headers(|lookup| {
            emit_standard_value_terminal_with_lookup_v1(
                builder, receiver, method, arguments, lookup,
            )
        })
    }
}

impl<Port> AssociatedMethodCallArgumentsV1<'_, '_, Port>
where
    Port: MethodCallDescentPortV1 + MethodCallValueTerminalPortV1,
{
    pub(in crate::mir::builder) fn finish_typeop_value_terminal(
        &mut self,
        builder: &mut MirBuilder,
        value: ValueId,
        op: TypeOpKind,
        ty: MirType,
    ) -> Result<ValueId, String> {
        self.terminal_port()
            .emit_typeop_value_terminal(builder, value, op, ty)
    }

    pub(in crate::mir::builder) fn finish_static_global_value_terminal(
        &mut self,
        builder: &mut MirBuilder,
        owner: &str,
        method: &str,
        checked_source_arity: u32,
        arguments: Vec<ValueId>,
    ) -> Result<ValueId, String> {
        self.terminal_port().emit_static_global_value_terminal(
            builder,
            owner,
            method,
            checked_source_arity,
            arguments,
        )
    }

    pub(in crate::mir::builder) fn finish_me_lowered_global_value_terminal(
        &mut self,
        builder: &mut MirBuilder,
        owner: &str,
        method: &str,
        checked_source_arity: u32,
        arguments: Vec<ValueId>,
    ) -> Result<ValueId, String> {
        self.terminal_port().emit_me_lowered_global_value_terminal(
            builder,
            owner,
            method,
            checked_source_arity,
            arguments,
        )
    }

    pub(in crate::mir::builder) fn finish_env_value_terminal(
        &mut self,
        builder: &mut MirBuilder,
        spec: &EnvMethodSpec,
        arguments: Vec<ValueId>,
    ) -> Result<ValueId, String> {
        self.terminal_port()
            .emit_env_value_terminal(builder, spec, arguments)
    }
}

impl<Port> StaticMethodCallCompletionV1 for AssociatedMethodCallArgumentsV1<'_, '_, Port>
where
    Port: MethodCallDescentPortV1 + MethodCallValueTerminalPortV1,
{
    fn finish_static_global_value_terminal(
        &mut self,
        builder: &mut MirBuilder,
        owner: &str,
        method: &str,
        checked_source_arity: u32,
        arguments: Vec<ValueId>,
    ) -> Result<ValueId, String> {
        self.terminal_port().emit_static_global_value_terminal(
            builder,
            owner,
            method,
            checked_source_arity,
            arguments,
        )
    }
}

impl<Port> StandardMethodCallCompletionV1 for AssociatedMethodCallArgumentsV1<'_, '_, Port>
where
    Port: MethodCallDescentPortV1 + MethodCallValueTerminalPortV1,
{
    fn finish_standard_value_terminal(
        &mut self,
        builder: &mut MirBuilder,
        receiver: ValueId,
        method: String,
        arguments: Vec<ValueId>,
    ) -> Result<ValueId, String> {
        self.terminal_port()
            .emit_standard_value_terminal(builder, receiver, method, arguments)
    }
}

pub(in crate::mir::builder) fn emit_typeop_value_terminal_raw_v1(
    builder: &mut MirBuilder,
    value: ValueId,
    op: TypeOpKind,
    ty: MirType,
) -> Result<ValueId, String> {
    let dst = builder.next_value_id();
    builder.emit_instruction(MirInstruction::TypeOp { dst, op, value, ty })?;
    Ok(dst)
}

pub(in crate::mir::builder) fn emit_global_value_terminal_raw_v1(
    builder: &mut MirBuilder,
    owner: &str,
    method: &str,
    checked_source_arity: u32,
    arguments: Vec<ValueId>,
) -> Result<(ValueId, String), String> {
    emit_global_value_terminal_with_lookup_v1(
        builder,
        owner,
        method,
        checked_source_arity,
        arguments,
        None,
    )
}

fn emit_global_value_terminal_with_lookup_v1(
    builder: &mut MirBuilder,
    owner: &str,
    method: &str,
    checked_source_arity: u32,
    arguments: Vec<ValueId>,
    lookup: Option<&dyn crate::mir::builder::function_signature_lookup::FunctionSignatureLookupV1>,
) -> Result<(ValueId, String), String> {
    let request = PreparedGlobalValueCallRequestV1::prepare(
        builder,
        owner,
        method,
        checked_source_arity,
        arguments,
    );
    let target = request.symbol.clone();
    builder.emit_unified_call_with_lookup(
        Some(request.destination),
        request.target,
        request.arguments,
        lookup,
    )?;
    Ok((request.destination, target))
}

/// Receipt-required sibling for a source-neutral static/global value request.
///
/// The ordinary terminal keeps its compatibility behavior. This sibling
/// accepts only the existing generic physical Call terminal, so rewrites,
/// BoxCall, legacy compatibility, a missing destination, and failed emission
/// cannot produce a receipt.
pub(in crate::mir::builder) fn emit_static_global_value_terminal_with_receipt_v1<Port>(
    builder: &mut MirBuilder,
    port: &mut Port,
    owner: &str,
    method: &str,
    checked_source_arity: u32,
    arguments: Vec<ValueId>,
) -> Result<CompletedUnifiedValueCallEmissionV1, UnifiedValueCallReceiptErrorV1>
where
    Port: RawFunctionHeaderLookupPortV1,
{
    let request = PreparedGlobalValueCallRequestV1::prepare(
        builder,
        owner,
        method,
        checked_source_arity,
        arguments,
    );
    port.with_function_headers(|lookup| {
        UnifiedCallEmitterBox::emit_unified_value_call_with_lookup_receipt_v1(
            builder,
            request.destination,
            request.target,
            request.arguments,
            lookup,
        )
    })
}

struct PreparedGlobalValueCallRequestV1 {
    destination: ValueId,
    symbol: String,
    target: CallTarget,
    arguments: Vec<ValueId>,
}

impl PreparedGlobalValueCallRequestV1 {
    fn prepare(
        builder: &mut MirBuilder,
        owner: &str,
        method: &str,
        checked_source_arity: u32,
        arguments: Vec<ValueId>,
    ) -> Self {
        let symbol = format!("{owner}.{method}/{checked_source_arity}");
        Self {
            destination: builder.next_value_id(),
            target: CallTarget::Global(symbol.clone()),
            symbol,
            arguments,
        }
    }
}

pub(in crate::mir::builder) fn emit_env_value_terminal_raw_v1(
    builder: &mut MirBuilder,
    spec: &EnvMethodSpec,
    arguments: Vec<ValueId>,
) -> Result<ValueId, String> {
    let result_id = builder.next_value_id();
    let dst = spec.returns.then_some(result_id);
    builder.emit_extern_call_with_effects(
        &spec.iface_name,
        &spec.method_name,
        arguments,
        dst,
        spec.effects,
    )?;
    if spec.returns {
        Ok(result_id)
    } else {
        crate::mir::builder::emission::constant::emit_void(builder)
    }
}

pub(in crate::mir::builder) fn emit_standard_value_terminal_raw_v1(
    builder: &mut MirBuilder,
    receiver: ValueId,
    method: String,
    arguments: Vec<ValueId>,
) -> Result<ValueId, String> {
    emit_standard_value_terminal_with_lookup_v1(builder, receiver, method, arguments, None)
}

fn emit_standard_value_terminal_with_lookup_v1(
    builder: &mut MirBuilder,
    receiver: ValueId,
    method: String,
    arguments: Vec<ValueId>,
    lookup: Option<&dyn crate::mir::builder::function_signature_lookup::FunctionSignatureLookupV1>,
) -> Result<ValueId, String> {
    let request = PreparedStandardValueCallRequestV1::prepare(builder, receiver, method, arguments);
    builder.emit_unified_call_with_lookup(
        Some(request.destination),
        request.target,
        request.arguments,
        lookup,
    )?;
    Ok(request.destination)
}

/// Receipt-required sibling for the bounded pre-loop candidate only.
///
/// The ordinary terminal retains its compatibility facade. This sibling shares
/// the same prepared standard Method request but requires the existing generic
/// physical Call terminal and therefore never accepts a rewrite, BoxCall, or
/// legacy fallback as success.
pub(super) fn emit_standard_value_terminal_with_receipt_v1<Port>(
    builder: &mut MirBuilder,
    port: &mut Port,
    receiver: ValueId,
    method: String,
    arguments: Vec<ValueId>,
) -> Result<CompletedUnifiedValueCallEmissionV1, UnifiedValueCallReceiptErrorV1>
where
    Port: RawFunctionHeaderLookupPortV1,
{
    let request = PreparedStandardValueCallRequestV1::prepare(builder, receiver, method, arguments);
    port.with_function_headers(|lookup| {
        UnifiedCallEmitterBox::emit_unified_value_call_with_lookup_receipt_v1(
            builder,
            request.destination,
            request.target,
            request.arguments,
            lookup,
        )
    })
}

struct PreparedStandardValueCallRequestV1 {
    destination: ValueId,
    target: CallTarget,
    arguments: Vec<ValueId>,
}

impl PreparedStandardValueCallRequestV1 {
    fn prepare(
        builder: &mut MirBuilder,
        receiver: ValueId,
        method: String,
        arguments: Vec<ValueId>,
    ) -> Self {
        Self {
            destination: builder.next_value_id(),
            target: CallTarget::Method {
                box_type: None,
                method,
                receiver,
            },
            arguments,
        }
    }
}
