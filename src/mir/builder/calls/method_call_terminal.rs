//! Value-only terminal boundary for source MethodCall lowering.
//!
//! Route selection, syntax preflight, and child descent must be complete before
//! this port is called. The port owns no route, target, effect, result, located
//! source, or caller-ledger authority. Its raw implementation only preserves
//! the existing legacy terminal operations.

use super::extern_calls::EnvMethodSpec;
use super::method_call_descent::MethodCallDescentPortV1;
use super::CallTarget;
use crate::mir::builder::recursive_child_lowering::RawLegacyChildLoweringPortV1;
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

impl MethodCallValueTerminalPortV1 for RawLegacyChildLoweringPortV1 {
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
        emit_global_value_terminal_raw_v1(builder, owner, method, checked_source_arity, arguments)
            .map(|(value, _)| value)
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
        let (value, target) = emit_global_value_terminal_raw_v1(
            builder,
            owner,
            method,
            checked_source_arity,
            arguments,
        )?;
        builder.annotate_call_result_from_func_name(value, &target);
        Ok(value)
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
        emit_standard_value_terminal_raw_v1(builder, receiver, method, arguments)
    }
}

fn emit_typeop_value_terminal_raw_v1(
    builder: &mut MirBuilder,
    value: ValueId,
    op: TypeOpKind,
    ty: MirType,
) -> Result<ValueId, String> {
    let dst = builder.next_value_id();
    builder.emit_instruction(MirInstruction::TypeOp { dst, op, value, ty })?;
    Ok(dst)
}

fn emit_global_value_terminal_raw_v1(
    builder: &mut MirBuilder,
    owner: &str,
    method: &str,
    checked_source_arity: u32,
    arguments: Vec<ValueId>,
) -> Result<(ValueId, String), String> {
    let target = format!("{owner}.{method}/{checked_source_arity}");
    let dst = builder.next_value_id();
    builder.emit_unified_call(Some(dst), CallTarget::Global(target.clone()), arguments)?;
    Ok((dst, target))
}

fn emit_env_value_terminal_raw_v1(
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
    let dst = builder.next_value_id();
    builder.emit_unified_call(
        Some(dst),
        CallTarget::Method {
            box_type: None,
            method,
            receiver,
        },
        arguments,
    )?;
    Ok(dst)
}
