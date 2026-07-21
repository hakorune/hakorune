//! Value-only terminal boundary for source MethodCall lowering.
//!
//! Route selection, syntax preflight, and child descent must be complete before
//! this port is called. The port owns no route, target, effect, result, located
//! source, or caller-ledger authority. Its raw implementation only preserves
//! the existing legacy terminal operations.

use super::extern_calls::EnvMethodSpec;
use super::method_call_descent::{AssociatedMethodCallArgumentsV1, MethodCallDescentPortV1};
use super::CallTarget;
use crate::mir::builder::recursive_child_lowering::{
    RawAstChildLoweringPortV1, RawFunctionHeaderLookupPortV1,
};
use crate::mir::{MirBuilder, MirInstruction, MirType, TypeOpKind, ValueId};

pub(in crate::mir::builder) trait MethodCallValueTerminalPortV1:
    MethodCallDescentPortV1
{
    fn emit_typeop_value_terminal(
        &mut self,
        builder: &mut MirBuilder,
        input: &Self::MethodCallInput,
        value: ValueId,
        op: TypeOpKind,
        ty: MirType,
    ) -> Result<ValueId, String>;

    fn emit_static_global_value_terminal(
        &mut self,
        builder: &mut MirBuilder,
        input: &Self::MethodCallInput,
        owner: &str,
        method: &str,
        checked_source_arity: u32,
        arguments: Vec<ValueId>,
    ) -> Result<ValueId, String>;

    fn emit_me_lowered_global_value_terminal(
        &mut self,
        builder: &mut MirBuilder,
        input: &Self::MethodCallInput,
        owner: &str,
        method: &str,
        checked_source_arity: u32,
        arguments: Vec<ValueId>,
    ) -> Result<ValueId, String>;

    fn emit_env_value_terminal(
        &mut self,
        builder: &mut MirBuilder,
        input: &Self::MethodCallInput,
        spec: &EnvMethodSpec,
        arguments: Vec<ValueId>,
    ) -> Result<ValueId, String>;

    fn emit_standard_value_terminal(
        &mut self,
        builder: &mut MirBuilder,
        input: &Self::MethodCallInput,
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
        _input: &Self::MethodCallInput,
        value: ValueId,
        op: TypeOpKind,
        ty: MirType,
    ) -> Result<ValueId, String> {
        emit_typeop_value_terminal_raw_v1(builder, value, op, ty)
    }

    fn emit_static_global_value_terminal(
        &mut self,
        builder: &mut MirBuilder,
        _input: &Self::MethodCallInput,
        owner: &str,
        method: &str,
        checked_source_arity: u32,
        arguments: Vec<ValueId>,
    ) -> Result<ValueId, String> {
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
        _input: &Self::MethodCallInput,
        owner: &str,
        method: &str,
        checked_source_arity: u32,
        arguments: Vec<ValueId>,
    ) -> Result<ValueId, String> {
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
        _input: &Self::MethodCallInput,
        spec: &EnvMethodSpec,
        arguments: Vec<ValueId>,
    ) -> Result<ValueId, String> {
        emit_env_value_terminal_raw_v1(builder, spec, arguments)
    }

    fn emit_standard_value_terminal(
        &mut self,
        builder: &mut MirBuilder,
        _input: &Self::MethodCallInput,
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
    Port: MethodCallValueTerminalPortV1,
{
    pub(in crate::mir::builder) fn finish_typeop_value_terminal(
        &mut self,
        builder: &mut MirBuilder,
        value: ValueId,
        op: TypeOpKind,
        ty: MirType,
    ) -> Result<ValueId, String> {
        let (port, input) = self.terminal_parts();
        port.emit_typeop_value_terminal(builder, input, value, op, ty)
    }

    pub(in crate::mir::builder) fn finish_static_global_value_terminal(
        &mut self,
        builder: &mut MirBuilder,
        owner: &str,
        method: &str,
        checked_source_arity: u32,
        arguments: Vec<ValueId>,
    ) -> Result<ValueId, String> {
        let (port, input) = self.terminal_parts();
        port.emit_static_global_value_terminal(
            builder,
            input,
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
        let (port, input) = self.terminal_parts();
        port.emit_me_lowered_global_value_terminal(
            builder,
            input,
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
        let (port, input) = self.terminal_parts();
        port.emit_env_value_terminal(builder, input, spec, arguments)
    }

    pub(in crate::mir::builder) fn finish_standard_value_terminal(
        &mut self,
        builder: &mut MirBuilder,
        receiver: ValueId,
        method: String,
        arguments: Vec<ValueId>,
    ) -> Result<ValueId, String> {
        let (port, input) = self.terminal_parts();
        port.emit_standard_value_terminal(builder, input, receiver, method, arguments)
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
    let target = format!("{owner}.{method}/{checked_source_arity}");
    let dst = builder.next_value_id();
    builder.emit_unified_call_with_lookup(
        Some(dst),
        CallTarget::Global(target.clone()),
        arguments,
        lookup,
    )?;
    Ok((dst, target))
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
    let dst = builder.next_value_id();
    builder.emit_unified_call_with_lookup(
        Some(dst),
        CallTarget::Method {
            box_type: None,
            method,
            receiver,
        },
        arguments,
        lookup,
    )?;
    Ok(dst)
}
