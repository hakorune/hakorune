//! Behavior-neutral call-argument descent port.
//!
//! This module owns the ordered argument preflight and descent boundary. It
//! extends the one recursive child-lowering port instead of creating a second
//! expression-lowering authority. It owns no call route, receiver, result,
//! callable-result location, or ledger policy.

use std::collections::BTreeMap;

use crate::ast::ASTNode;
use crate::mir::{MirBuilder, ValueId};

use super::super::recursive_child_lowering::{
    drive_legacy_expression_v1, RawAstChildLoweringPortV1, RawLegacyChildLoweringPortV1,
    RecursiveChildLoweringPortV1,
};

pub(in crate::mir::builder) trait CallArgumentDescentPortV1:
    RecursiveChildLoweringPortV1
{
    type ArgumentsInput: ?Sized;

    fn argument_count(&self, input: &Self::ArgumentsInput) -> usize;

    fn argument_syntax<'input>(
        &self,
        input: &'input Self::ArgumentsInput,
        index: usize,
    ) -> Option<&'input ASTNode>;

    fn argument_expression_input(
        &self,
        input: &Self::ArgumentsInput,
        index: usize,
    ) -> Result<Self::ExpressionInput, String>;
}

pub(in crate::mir::builder) fn drive_call_arguments_v1<Port>(
    builder: &mut MirBuilder,
    port: &mut Port,
    input: &Port::ArgumentsInput,
) -> Result<Vec<ValueId>, String>
where
    Port: CallArgumentDescentPortV1,
{
    validate_argument_inputs(port, input)?;
    enforce_moved_same_call_args_contract(port, input)?;

    let mut values = Vec::with_capacity(port.argument_count(input));
    for index in 0..port.argument_count(input) {
        preflight_record_value_argument(builder, port, input, index)?;
        let expression_input = port.argument_expression_input(input, index)?;
        let value = drive_legacy_expression_v1(builder, port, expression_input)?;
        observe_undefined_argument_value(builder, port, input, index, value);
        values.push(value);
    }
    Ok(values)
}

fn validate_argument_inputs<Port>(port: &Port, input: &Port::ArgumentsInput) -> Result<(), String>
where
    Port: CallArgumentDescentPortV1,
{
    for index in 0..port.argument_count(input) {
        if port.argument_syntax(input, index).is_none() {
            return Err(format!(
                "[call-argument-descent/missing-input] index={} count={}",
                index,
                port.argument_count(input)
            ));
        }
    }
    Ok(())
}

fn enforce_moved_same_call_args_contract<Port>(
    port: &Port,
    input: &Port::ArgumentsInput,
) -> Result<(), String>
where
    Port: CallArgumentDescentPortV1,
{
    if !crate::config::env::joinir_dev::strict_planner_required_enabled() {
        return Ok(());
    }

    let mut first_seen: BTreeMap<&str, usize> = BTreeMap::new();
    for index in 0..port.argument_count(input) {
        let syntax = port
            .argument_syntax(input, index)
            .expect("argument inputs were validated before moved-state preflight");
        let ASTNode::Variable { name, .. } = syntax else {
            continue;
        };
        if let Some(previous) = first_seen.insert(name.as_str(), index) {
            return Err(format!(
                "[freeze:contract][moved/use_after_move_same_call] var={} first_arg={} reused_arg={}",
                name, previous, index
            ));
        }
    }
    Ok(())
}

fn preflight_record_value_argument<Port>(
    builder: &MirBuilder,
    port: &Port,
    input: &Port::ArgumentsInput,
    index: usize,
) -> Result<(), String>
where
    Port: CallArgumentDescentPortV1,
{
    let syntax = port
        .argument_syntax(input, index)
        .expect("argument inputs were validated before record-value preflight");
    if let ASTNode::Variable { name, .. } = syntax {
        if let Some(value) = builder
            .function_state
            .variable_ctx
            .variable_map
            .get(name)
            .copied()
        {
            builder.fail_if_record_value_call_arg_by_name(name, value)?;
        }
    }
    Ok(())
}

fn observe_undefined_argument_value<Port>(
    builder: &MirBuilder,
    port: &Port,
    input: &Port::ArgumentsInput,
    index: usize,
    value: ValueId,
) where
    Port: CallArgumentDescentPortV1,
{
    if !crate::config::env::joinir_dev::debug_enabled() {
        return;
    }
    let Some(function) = builder.function_state.current_function.as_ref() else {
        return;
    };
    let def_blocks = crate::mir::verification::utils::compute_def_blocks(function);
    if def_blocks.contains_key(&value) {
        return;
    }

    let syntax = port
        .argument_syntax(input, index)
        .expect("argument inputs were validated before debug observation");
    let ring0 = crate::runtime::get_global_ring0();
    ring0.log.debug(&format!(
        "[call/arg_build:undefined_value] fn={} bb={:?} arg_idx={} v=%{} ast={} span={:?} next={}",
        function.signature.name,
        builder.function_state.current_block,
        index,
        value.0,
        syntax.node_type(),
        syntax.span(),
        function.next_value_id
    ));
}

impl<Port> CallArgumentDescentPortV1 for Port
where
    Port: RawAstChildLoweringPortV1,
{
    type ArgumentsInput = [ASTNode];

    fn argument_count(&self, input: &Self::ArgumentsInput) -> usize {
        input.len()
    }

    fn argument_syntax<'input>(
        &self,
        input: &'input Self::ArgumentsInput,
        index: usize,
    ) -> Option<&'input ASTNode> {
        input.get(index)
    }

    fn argument_expression_input(
        &self,
        input: &Self::ArgumentsInput,
        index: usize,
    ) -> Result<Self::ExpressionInput, String> {
        input
            .get(index)
            .cloned()
            .ok_or_else(|| format!("[call-argument-descent/missing-input] index={index}"))
    }
}

pub(in crate::mir::builder) fn drive_raw_call_arguments_v1(
    builder: &mut MirBuilder,
    arguments: &[ASTNode],
) -> Result<Vec<ValueId>, String> {
    let mut port = RawLegacyChildLoweringPortV1;
    drive_call_arguments_v1(builder, &mut port, arguments)
}
