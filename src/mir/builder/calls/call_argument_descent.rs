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
    drive_legacy_expression_v1, DirectCallDispositionPortV1, RawAstChildLoweringPortV1,
    RawLegacyChildLoweringPortV1, RecursiveChildLoweringPortV1,
};
use crate::mir::resolved_semantics::SourceExprSiteV1;
use crate::mir::resolved_semantics::home_new_prefix::{LocalCallArgumentV1, LocalCallObservationV1};
use crate::mir::normal_callable_semantic_package::LexicalCallArgumentProjectionV1 as ArgumentProjection;

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

    /// Projects one argument exactly once after the shared list preflight.
    /// Implementations may consume stack-scoped candidate state here, but
    /// must not add a second ordered driver or copy the preflight.
    fn argument_expression_input(
        &mut self,
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
        let value = port.with_call_argument_source_v1(index, |port| {
            let expression_input = port.argument_expression_input(input, index)?;
            drive_legacy_expression_v1(builder, port, expression_input)
        })?;
        observe_undefined_argument_value(builder, port, input, index, value);
        values.push(value);
    }
    Ok(values)
}

/// The exact direct-call consumer uses the same ordered argument descent,
/// with one additional check that the raw argument scope matches the source
/// sites sealed by the producer.  No second AST walk or target lookup is
/// introduced here.
/// One completed ordered descent; raw values cannot manufacture its source sites.
pub(in crate::mir::builder) struct CompletedDirectCallArgumentsV1 {
    values: Vec<ValueId>,
    observations: Vec<(u32, SourceExprSiteV1, ValueId)>,
    projections: Option<Vec<ArgumentProjection>>,
}
impl CompletedDirectCallArgumentsV1 {
    pub(in crate::mir::builder) fn into_parts(self) -> (Vec<ValueId>, Vec<(u32, SourceExprSiteV1, ValueId)>) {
        (self.values, self.observations)
    }
    pub(in crate::mir::builder) fn into_values(self) -> Vec<ValueId> { self.values }
    pub(in crate::mir::builder) fn bind_row(self,
        mut row: crate::mir::normal_callable_semantic_package::DirectCallDispositionRowV1,
    ) -> Result<(crate::mir::normal_callable_semantic_package::DirectCallDispositionRowV1, Vec<ValueId>), String> {
        row.retain_argument_descent(self.observations)?;
        if let Some(projections) = self.projections { row.retain_argument_projections_v1(projections)?; }
        Ok((row, self.values))
    }
}

pub(in crate::mir::builder) fn drive_call_arguments_with_expected_sites_v1<Port>(
    builder: &mut MirBuilder,
    port: &mut Port,
    input: &Port::ArgumentsInput,
    expected_sites: &[SourceExprSiteV1],
    original: Option<&LocalCallObservationV1>,
) -> Result<CompletedDirectCallArgumentsV1, String>
where
    Port: CallArgumentDescentPortV1 + DirectCallDispositionPortV1,
{
    validate_argument_inputs(port, input)?;
    if port.argument_count(input) != expected_sites.len() {
        return Err(format!(
            "[freeze:contract][direct-call/argument-site-cardinality] expected={} actual={}",
            expected_sites.len(),
            port.argument_count(input)
        ));
    }
    enforce_moved_same_call_args_contract(port, input)?;

    if original.is_some_and(|source| source.arguments().len() != expected_sites.len()) {
        return Err("[freeze:contract][static-execution/argument-source-cardinality]".into());
    }
    let mut values = Vec::with_capacity(port.argument_count(input));
    let mut observations = Vec::with_capacity(expected_sites.len());
    let mut projections = original.map(|_| Vec::with_capacity(expected_sites.len()));
    for (index, expected_site) in expected_sites.iter().enumerate() {
        preflight_record_value_argument(builder, port, input, index)?;
        let (value, projection) = port.with_call_argument_source_v1(index, |port| -> Result<(ValueId, Option<ArgumentProjection>), String> {
            port.validate_current_call_argument_site_v1(expected_site)?;
            let source = original.map(|source| &source.arguments()[index]);
            // An exact read owns this site's consumption. Generic variable
            // descent would consume it a second time, so it is not invoked.
            if let Some(LocalCallArgumentV1::Scalar(binding)) = source {
                let projection = ArgumentProjection::Scalar(port.take_exact_static_argument_read_v1(expected_site, *binding)?);
                let value = projection.value_for_source_v1(source.unwrap(), original.unwrap().owner(), expected_site, None, None)?;
                return Ok((value, Some(projection)));
            }
            if let Some(LocalCallArgumentV1::Expression { ordinal, site }) = source {
                if *ordinal as usize != index || site != expected_site {
                    return Err("[freeze:contract][static-execution/argument-expression-site]".into());
                }
                let (value, projection) = port.emit_static_argument_expression_v1(builder, original.unwrap().site(), *ordinal, site)?;
                return Ok((value, Some(projection)));
            }
            if source.is_some_and(|source| !matches!(source, LocalCallArgumentV1::Integer(_))) {
                return Err("[freeze:contract][static-execution/argument-child-proof-required]".into());
            }
            let expression_input = port.argument_expression_input(input, index)?;
            let value = drive_legacy_expression_v1(builder, port, expression_input)?;
            let projection = if let Some(source) = source {
                let function = builder.function_state.current_function.as_ref()
                    .ok_or("[freeze:contract][static-execution/argument-function]")?;
                let mut definitions = function.blocks.iter().flat_map(|(block, body)|
                    body.all_instructions().map(move |row| (*block, row)))
                    .filter(|(_, row)| row.dst_value() == Some(value));
                let (block, row) = definitions.next().ok_or("[freeze:contract][static-execution/argument-producer]")?;
                if definitions.next().is_some() || function.params.contains(&value) {
                    return Err("[freeze:contract][static-execution/argument-producer-duplicate]".into());
                }
                let projection = ArgumentProjection::Integer((block, row.clone()));
                if projection.value_for_source_v1(source, original.unwrap().owner(), expected_site, None, None)? != value {
                    return Err("[freeze:contract][static-execution/argument-value-drift]".into());
                }
                Some(projection)
            } else { None };
            Ok((value, projection))
        })?;
        if let (Some(projections), Some(projection)) = (&mut projections, projection) { projections.push(projection); }
        observe_undefined_argument_value(builder, port, input, index, value);
        observations.push((u32::try_from(index).map_err(|_| "[freeze:contract][direct-call/argument-ordinal-overflow]")?, expected_site.clone(), value));
        values.push(value);
    }
    Ok(CompletedDirectCallArgumentsV1 { values, observations, projections })
}

/// Lower one already-selected argument through the same projection and
/// recursive expression authority used by the ordered driver.
///
/// This helper intentionally does not repeat list-wide preflight. Callers use
/// it only after their existing route-specific indexed decision.
pub(in crate::mir::builder) fn lower_call_argument_v1<Port>(
    builder: &mut MirBuilder,
    port: &mut Port,
    input: &Port::ArgumentsInput,
    index: usize,
) -> Result<ValueId, String>
where
    Port: CallArgumentDescentPortV1,
{
    port.with_call_argument_source_v1(index, |port| {
        let expression_input = port.argument_expression_input(input, index)?;
        drive_legacy_expression_v1(builder, port, expression_input)
    })
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
        &mut self,
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
