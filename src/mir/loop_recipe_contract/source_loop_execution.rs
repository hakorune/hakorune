//! Source execution/lifecycle obligations of the SAME checked Loop tree.
//! These are analysis templates, never runtime acquisitions or physical grants.
use crate::mir::resolved_semantics::home_new_prefix::{
    LiveHomeV1, LocalCallObservationV1, LocalCallResultClassV1, LocalCallResultSourceV1,
};
use super::source_loop_v2::{reject, Summary, Body, Statement, Evaluation, Source, SourceProducer, TextProducer};
use std::{rc::Rc, collections::{BTreeMap, BTreeSet}};
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::resolved_semantics::{OwnedExprSiteV1,
    CoreMethodHomeParameterRelationV1 as Parameter, CoreMethodHomeResultRelationV1 as ResultKind};
use crate::mir::resolved_semantics::source_value_flow::SourceValueEdgeV1 as Edge;
use crate::mir::resolved_value_profile::{expression_projection::ExpressionLeafClassV1 as Leaf,
    expression_class::ExpressionValueClassV1 as Class};

#[derive(Debug)]
pub(crate) struct SourceLoopFaultV2 {
    pub(crate) node: usize,
    pub(crate) continuation: crate::mir::resolved_control_flow::LoopExpressionFaultContinuationV1,
    /// Only acquisitions whose Normal already occurred before this operation.
    pub(crate) prior_acquisitions: Box<[usize]>,
    /// Text carrier state at this Fault point. A failed RHS keeps the old
    /// Header/Residence; it never publishes the not-yet-created result.
    pub(crate) text_carriers: Box<[SourceLoopTextCarrierStateV2]>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SourceLoopTextCarrierStateV2 {
    pub(crate) basis: Rc<crate::mir::resolved_semantics::home_new_prefix::LoopHeaderValueBasisV1>,
    pub(crate) producer: TextProducer,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SourceLoopTextOldResidenceV2 {
    ConditionalHeader,
    OwnedResidence,
}
#[derive(Debug)]
pub(crate) struct SourceLoopTextCarrierTransitionV2 {
    pub(crate) basis: Rc<crate::mir::resolved_semantics::home_new_prefix::LoopHeaderValueBasisV1>,
    pub(crate) evaluation: Rc<Source>,
    pub(crate) before: TextProducer,
    pub(crate) after: crate::mir::resolved_semantics::home_new_prefix::TextResidenceAcquisitionV1,
    pub(crate) old_residence: SourceLoopTextOldResidenceV2,
}
#[derive(Debug)]
pub(crate) struct SourceLoopEvaluationLifecycleV2 {
    pub(crate) source: Rc<Source>,
    pub(crate) normal_temporary_ends: Box<[usize]>,
    pub(crate) faults: Box<[SourceLoopFaultV2]>,
    /// Whole-evaluation entry state. Bool's own receipt owns conditional Fault
    /// reach; this snapshot grants no eager operation or acquisition.
    pub(crate) text_carriers: Box<[SourceLoopTextCarrierStateV2]>,
    pub(crate) text_carrier_transition: Option<SourceLoopTextCarrierTransitionV2>,
}
fn call_at_node<'a>(frame: &'a Evaluation, node: usize) -> Result<Option<&'a LocalCallObservationV1>, String> {
    let original = frame.source().nodes().get(node).ok_or_else(|| reject("execution-call-node"))?;
    if !matches!(original.source(), crate::mir::resolved_value_profile::expression_plan::ExpressionNodeSourceV1::MethodCall) {
        return Ok(None);
    }
    let mut calls = frame.calls().iter().filter(|call|
        call.site().owner() == frame.source().owner() && call.site().site() == original.site());
    let call = calls.next();
    if calls.next().is_some() {
        return Err(reject("execution-call-duplicate"));
    }
    Ok(call)
}
fn call_result_class(result: LocalCallResultClassV1) -> Option<Class> {
    Some(match result {
        LocalCallResultClassV1::Dynamic => Class::Dynamic,
        LocalCallResultClassV1::I64 => Class::I64,
        LocalCallResultClassV1::Bool => Class::Bool,
        LocalCallResultClassV1::Text => Class::String,
        _ => return None,
    })
}
fn transferred_by_expression_call(frame: &Evaluation, node: usize) -> bool {
    let Some(original) = frame.source().nodes().get(node) else { return false; };
    frame.calls().iter().any(|call| call.result_source().is_some_and(|loan| {
        std::iter::once(loan.receiver_site()).chain(loan.argument_sites().iter()).any(|input| {
            original.site().node().segments().starts_with(input.node().segments())
        })
    }))
}
fn fault(input: ResolvedFunctionLoweringInputV1<'_>, source: &Summary, frame: &Evaluation,
    node: usize, prior_acquisitions: &[usize],
    text_carriers: &BTreeMap<crate::mir::resolved_semantics::BindingRefV1, SourceLoopTextCarrierStateV2>,
) -> Result<SourceLoopFaultV2, String> {
    let site = OwnedExprSiteV1::new(input.owner(), frame.source().nodes()[node].site().clone());
    let continuation = crate::mir::resolved_control_flow::issue_loop_expression_fault_continuation_v1(
        input, source.source.site(), source.source.frame(), &site).map_err(reject)?;
    Ok(SourceLoopFaultV2 { node, continuation, prior_acquisitions: prior_acquisitions.to_vec().into_boxed_slice(),
        text_carriers: text_carriers.values().cloned().collect() })
}
fn evaluation(input: ResolvedFunctionLoweringInputV1<'_>, source: &Summary, frame: &Evaluation,
    homes: &[LiveHomeV1], retain_root: bool,
    text_carriers: &BTreeMap<crate::mir::resolved_semantics::BindingRefV1, SourceLoopTextCarrierStateV2>,
) -> Result<SourceLoopEvaluationLifecycleV2, String> {
    if frame.prior_homes() != homes { return Err(reject("execution-prior-homes")); }
    let mut acquisitions = Vec::new();
    let mut faults = Vec::new();
    let mut cores = 0;
    let mut text_comparisons = 0;
    let mut comparison_cores = 0;
    let mut owned_string_operations = 0;
    let mut dynamic_operator_faults = 0;
    let mut calls = 0;
    let mut normal_temporary_ends = Vec::new();
    for &node in frame.ordered_nodes() {
        if frame.transfers_node(node) {
            if !transferred_by_expression_call(frame, node) {
                return Err(reject("execution-transferred-source"));
            }
            continue;
        }
        if frame.node_is_identity(node) {
            return Err(reject("execution-identity-node"));
        }
        let call = call_at_node(frame, node)?;
        if let Some(core) = frame.cores().get(&node) {
            if call.is_some() { return Err(reject("execution-call-core-collision")); }
            let target = core.source().target();
            if !core.matches_plan(frame.source(), node) || target.checked_code_point_law_v1().is_none() {
                return Err(reject("execution-core-policy"));
            }
            let original = core.source().source();
            let leaf = |site| frame.source().nodes().iter().position(|row| row.site() == site)
                .and_then(|node| frame.class(node));
            if leaf(original.receiver_site()) != Some(Leaf::Value(Class::String))
                || target.parameters().len() != original.arguments().len()
                || target.parameters().iter().zip(original.arguments()).any(|(parameter, argument)| {
                    let expected = match parameter {
                        Parameter::I64Parameter if core.dynamic_i64_operand_contains(argument.site()) => Class::Dynamic,
                        Parameter::I64Parameter => Class::I64,
                        Parameter::TextParameter => Class::String,
                        Parameter::TextRetainedByReceiver => return true,
                    };
                    leaf(argument.site()) != Some(Leaf::Value(expected))
                }) {
                return Err(reject("execution-core-operands"));
            }
            let result = match target.result() {
                ResultKind::I64ToCaller => Class::I64,
                ResultKind::TextToCaller => Class::String,
                _ => return Err(reject("execution-core-result")),
            };
            if frame.class(node) != Some(Leaf::Value(result))
                || core.owns_result() != frame.owned_result_nodes().contains(&node) {
                return Err(reject("execution-core-ownership"));
            }
            faults.push(fault(input, source, frame, node, &acquisitions, text_carriers)?);
            cores += 1;
        } else if let Some(call) = call {
            let node_class = match frame.class(node) {
                Some(Leaf::Value(class)) => Some(class),
                _ => None,
            };
            let source_call = call.result_source().is_some()
                && call.owner() == frame.source().owner()
                && call.statement() == frame.source().root().statement()
                && call.is_expression_value()
                && call_result_class(call.result()) == node_class;
            if !source_call {
                return Err(reject("execution-call-source"));
            }
            faults.push(fault(input, source, frame, node, &acquisitions, text_carriers)?);
            calls += 1;
        } else if frame.operations().iter().any(|loan| loan.node() == node
            && loan.single_envelope().is_some_and(is_source_loop_dynamic_operator_v1)) {
            use crate::mir::dynamic_operator_contract::{DynamicOperatorFamilyV1 as Family,
                DynamicOperatorNormalResultV1 as ResultKind,
                DynamicOperatorValueClassV1 as Value};
            let loan = frame.operations().iter().find(|loan| loan.node() == node)
                .ok_or_else(|| reject("execution-dynamic-operation-loan"))?;
            let envelope = loan.single_envelope().ok_or_else(|| reject("execution-dynamic-operation-envelope"))?;
            let domain = envelope.domain();
            let valid = match (domain.family(), domain.left(), domain.right(), envelope.normal_result()) {
                (Family::Add, Value::Dynamic, Value::I64, ResultKind::SelfContainedNonAliasingDynamicCarrier) => {
                    frame.class(node) == Some(Leaf::Value(Class::Dynamic))
                        && !frame.owned_result_nodes().contains(&node)
                }
                (Family::Sub, Value::Dynamic, Value::I64, ResultKind::SelfContainedNonAliasingDynamicCarrier) => {
                    frame.class(node) == Some(Leaf::Value(Class::Dynamic))
                        && frame.owned_result_nodes().contains(&node)
                }
                (Family::Less | Family::Greater, Value::Dynamic, Value::I64, ResultKind::TrivialBool)
                | (Family::Less, Value::I64, Value::Dynamic, ResultKind::TrivialBool)
                | (Family::Equal | Family::NotEqual | Family::Less | Family::LessEqual
                    | Family::Greater | Family::GreaterEqual, Value::Dynamic, Value::String, ResultKind::TrivialBool)
                | (Family::Equal | Family::NotEqual | Family::Less | Family::LessEqual
                    | Family::Greater | Family::GreaterEqual, Value::String, Value::Dynamic, ResultKind::TrivialBool) => {
                    frame.class(node) == Some(Leaf::Value(Class::Bool))
                        && !frame.owned_result_nodes().contains(&node)
                }
                _ => false,
            };
            if !valid {
                return Err(reject("execution-dynamic-operation-domain"));
            }
            faults.push(fault(input, source, frame, node, &acquisitions, text_carriers)?);
            dynamic_operator_faults += 1;
        } else if frame.operations().iter().any(|loan| loan.node() == node
            && loan.single_envelope().is_some_and(|envelope| {
                use crate::mir::dynamic_operator_contract::{DynamicOperatorFamilyV1 as Family,
                    DynamicOperatorValueClassV1 as Value};
                envelope.domain().family() != Family::Add
                    && envelope.domain().left() == Value::String && envelope.domain().right() == Value::String
            })) {
            // The same closed Bool source projector already seals exact family,
            // domain and conditional reach. Its comparisons acquire no owner.
            if frame.class(node) != Some(Leaf::Value(Class::Bool)) || frame.owned_result_nodes().contains(&node) {
                return Err(reject("execution-comparison-result"));
            }
            let inputs = frame.text_comparison_input_order_v1(node).ok_or_else(|| reject("execution-operation-census"))?;
            let expected: Vec<_> = inputs.iter().copied().filter(|node| frame.owned_result_nodes().contains(node)).collect();
            if acquisitions != expected { return Err(reject("execution-operation-census")); }
            comparison_cores += inputs.iter().filter(|node| frame.cores().contains_key(node)).count();
            faults.push(fault(input, source, frame, node, &acquisitions, text_carriers)?);
            // Each comparison closes before the logical merge. Its acquired
            // prefix must never enter a later or skipped RHS Fault scope.
            normal_temporary_ends.extend(acquisitions.drain(..).rev());
            text_comparisons += 1;
        } else if frame.operations().iter().any(|loan| loan.node() == node
            && loan.normal_result_v1() == Some(crate::mir::dynamic_operator_contract::DynamicOperatorNormalResultV1::OwnedStringValue)) {
            if frame.class(node) != Some(Leaf::Value(Class::String)) || !frame.owned_result_nodes().contains(&node) {
                return Err(reject("execution-operation-ownership"));
            }
            // Dynamic String Add can Fault before an owned result exists. Its
            // same original operation loan owns both the Fault boundary and
            // the Normal acquisition; no call/core source is synthesized.
            faults.push(fault(input, source, frame, node, &acquisitions, text_carriers)?);
            owned_string_operations += 1;
        }
        if frame.owned_result_nodes().contains(&node) {
            let core_owns = frame.cores().get(&node).is_some_and(|core| core.owns_result());
            let operation_owns = frame.operations().iter().any(|loan| {
                if loan.node() != node {
                    return false;
                }
                if loan.normal_result_v1()
                    == Some(crate::mir::dynamic_operator_contract::DynamicOperatorNormalResultV1::OwnedStringValue)
                {
                    return true;
                }
                use crate::mir::dynamic_operator_contract::{DynamicOperatorFamilyV1 as Family,
                    DynamicOperatorNormalResultV1 as Normal, DynamicOperatorValueClassV1 as Value};
                loan.normal_result_v1() == Some(Normal::SelfContainedNonAliasingDynamicCarrier)
                    && loan.single_envelope().is_some_and(|envelope| {
                        let domain = envelope.domain();
                        domain.family() == Family::Sub
                            && domain.left() == Value::Dynamic
                            && domain.right() == Value::I64
                    })
            });
            let call_owns = frame.calls().iter().any(|call| {
                let ordinary_text_result = match call.result_source() {
                    Some(LocalCallResultSourceV1::Ordinary(loan)) => {
                        call.result() == LocalCallResultClassV1::Text
                            && loan.result() == LocalCallResultClassV1::Text
                    }
                    _ => false,
                };
                call.site().owner() == frame.source().owner()
                    && frame.source().nodes().get(node).is_some_and(|original|
                        call.site().site() == original.site())
                    && (ordinary_text_result || call.dynamic_invocation_result_lifecycle_v1().is_some())
            });
            if usize::from(core_owns) + usize::from(operation_owns) + usize::from(call_owns) != 1 {
                if crate::config::env::builder_debug_enabled() {
                    let results: Vec<_> = frame.operations().iter().filter(|loan| loan.node() == node)
                        .map(|loan| loan.normal_result_v1()).collect();
                    eprintln!("[source-loop-acquisition] owner={:?} site={:?} node={node} source={:?} class={:?} core_owns={core_owns} operation_owns={operation_owns} call_owns={call_owns} operation_results={results:?}",
                        input.owner(), frame.source().nodes().get(node).map(|source| source.site()),
                        frame.source().nodes().get(node).map(|source| source.source()), frame.class(node));
                }
                return Err(reject("execution-acquisition-family"));
            }
            acquisitions.push(node);
        }
    }
    let expected_dynamic_faults = frame.operations().iter()
        .filter(|loan| loan.single_envelope().is_some_and(is_source_loop_dynamic_operator_v1)).count();
    if calls != frame.calls().len() || cores != frame.cores().len() || dynamic_operator_faults != expected_dynamic_faults
        || owned_string_operations != frame.operations().iter()
        .filter(|loan| loan.normal_result_v1() == Some(crate::mir::dynamic_operator_contract::DynamicOperatorNormalResultV1::OwnedStringValue)).count()
        || (text_comparisons != 0 && (comparison_cores != cores || !acquisitions.is_empty())) {
        return Err(reject("execution-operation-census"));
    }
    normal_temporary_ends.extend(acquisitions.into_iter().rev().filter(|node| !(retain_root && *node == 0)));
    Ok(SourceLoopEvaluationLifecycleV2 { source: Rc::clone(frame.source()),
        normal_temporary_ends: normal_temporary_ends.into_boxed_slice(), faults: faults.into_boxed_slice(),
        text_carriers: text_carriers.values().cloned().collect(),
        text_carrier_transition: None })
}

/// A checked BoolCondition is the source owner for effect nodes retained inside
/// its Bool tree. Its physical projection owns call Invoke, branch reach, and
/// per-arm Normal/Fault End. Direct root CallSlots keep their separate Call
/// owner. Keep this outer inventory record effect-free so a skipped RHS is
/// never described as an acquisition.
fn bool_condition_execution(
    input: ResolvedFunctionLoweringInputV1<'_>,
    frame: &crate::mir::resolved_semantics::home_new_prefix::CheckedBoolConditionFrameV1,
    homes: &[LiveHomeV1],
    text_carriers: &BTreeMap<crate::mir::resolved_semantics::BindingRefV1, SourceLoopTextCarrierStateV2>,
) -> Result<SourceLoopEvaluationLifecycleV2, String> {
    if frame.source().owner() != input.owner() || frame.prior_homes() != homes
        || !frame.evaluation().reference_coverage_is_exact() {
        return Err(reject("execution-bool-condition-source"));
    }
    Ok(SourceLoopEvaluationLifecycleV2 {
        source: Rc::clone(frame.source()),
        normal_temporary_ends: Box::new([]),
        faults: Box::new([]),
        text_carriers: text_carriers.values().cloned().collect(),
        text_carrier_transition: None,
    })
}

fn is_source_loop_dynamic_operator_v1(
    envelope: &crate::mir::dynamic_operator_contract::VerifiedDynamicOperatorExecutionEnvelopeV1,
) -> bool {
    use crate::mir::dynamic_operator_contract::{DynamicOperatorFamilyV1 as Family,
        DynamicOperatorNormalResultV1 as ResultKind, DynamicOperatorValueClassV1 as Value};
    let domain = envelope.domain();
    matches!((domain.family(), domain.left(), domain.right(), envelope.normal_result()),
        (Family::Add, Value::Dynamic, Value::I64, ResultKind::SelfContainedNonAliasingDynamicCarrier)
            | (Family::Sub, Value::Dynamic, Value::I64, ResultKind::SelfContainedNonAliasingDynamicCarrier)
            | (Family::Less | Family::Greater, Value::Dynamic, Value::I64, ResultKind::TrivialBool)
            | (Family::Less, Value::I64, Value::Dynamic, ResultKind::TrivialBool)
            | (Family::Equal | Family::NotEqual | Family::Less | Family::LessEqual
                | Family::Greater | Family::GreaterEqual, Value::Dynamic, Value::String, ResultKind::TrivialBool)
            | (Family::Equal | Family::NotEqual | Family::Less | Family::LessEqual
                | Family::Greater | Family::GreaterEqual, Value::String, Value::Dynamic, ResultKind::TrivialBool))
}
fn normal_port(input: ResolvedFunctionLoweringInputV1<'_>, source: &Summary,
    port: usize, homes: &[LiveHomeV1],
    text_carriers: &BTreeMap<crate::mir::resolved_semantics::BindingRefV1, SourceLoopTextCarrierStateV2>,
) -> Result<(), String> {
    let port = source.ports.get(port).ok_or_else(|| reject("execution-normal-port"))?;
    if port.homes() != homes { return Err(reject("execution-port-homes")); }
    let mut unique = BTreeSet::new();
    let mut ends = Vec::new();
    let text_bindings: BTreeSet<_> = source.headers.iter().filter_map(|(binding, basis)|
        (basis.entry_producer().class() == Class::String).then_some(*binding)).collect();
    if text_carriers.keys().copied().collect::<BTreeSet<_>>() != text_bindings {
        return Err(reject("execution-text-carrier-census"));
    }
    for home in homes.iter().rev() {
        let binding = input.function().binding(home.binding()).ok_or_else(|| reject("execution-home-binding"))?;
        if !unique.insert(home.binding()) { return Err(reject("execution-home-duplicate")); }
        if !text_bindings.contains(&home.binding()) && port.crossed_scopes().contains(&binding.owner_scope()) {
            ends.push(home.clone());
        }
    }
    let retained: Vec<_> = homes.iter().cloned().filter(|home|
        !text_bindings.contains(&home.binding()) && !ends.contains(home)).collect();
    let prior: Vec<_> = source.condition.prior_homes().iter().filter(|home| !text_bindings.contains(&home.binding())).cloned().collect();
    if ends.as_slice() != port.normal_ends().iter().filter(|home| !text_bindings.contains(&home.binding())).cloned().collect::<Vec<_>>().as_slice()
        || retained != prior {
        return Err(reject("execution-normal-home-closure"));
    }
    verify_text_carrier_port(input, source, &port.kind, text_carriers, homes)?;
    Ok(())
}

fn verify_text_carrier_port(input: ResolvedFunctionLoweringInputV1<'_>, source: &Summary,
    port: &crate::mir::resolved_semantics::home_new_prefix::SourceLoopPortKindV1,
    text_carriers: &BTreeMap<crate::mir::resolved_semantics::BindingRefV1, SourceLoopTextCarrierStateV2>,
    homes: &[LiveHomeV1],
) -> Result<(), String> {
    use crate::mir::resolved_semantics::home_new_prefix::SourceLoopPortKindV1 as Port;
    use crate::mir::resolved_semantics::source_value_flow::SourceValueEdgeV1 as Edge;
    let (edge, after) = match port {
        Port::Backedge => (Edge::Backedge, false),
        Port::Break(site) => (Edge::Break(site.clone()), true),
        Port::Continue(site) => (Edge::Continue(site.clone()), false),
    };
    let incoming = source.incoming.as_ref().map_err(|reason| reject(reason))?;
    for (binding, state) in text_carriers {
        let mut rows = incoming.iter().filter(|row| row.basis.binding() == *binding
            && Rc::ptr_eq(&row.basis, &state.basis)).flat_map(|row| {
                if after { row.after_incoming.iter() } else { row.header_incoming.iter() }
            }).filter(|row| row.edge == edge);
        let row = rows.next().ok_or_else(|| reject("execution-text-edge-missing"))?;
        if rows.next().is_some() { return Err(reject("execution-text-edge-duplicate")); }
        let SourceProducer::Text(expected) = &row.producer else { return Err(reject("execution-text-edge-class")); };
        let matches = state.producer == *expected || matches!(&state.producer,
            TextProducer::Header(basis) if Rc::ptr_eq(basis, &state.basis)
                && state.basis.entry_producer() == &SourceProducer::Text(expected.clone()));
        if !matches { return Err(reject("execution-text-edge-producer")); }
        let carrier_homes: Vec<_> = homes.iter().filter(|home| home.binding() == *binding).collect();
        match &state.producer {
            TextProducer::Residence(acquisition) => {
                let entry = match state.basis.entry_producer() {
                    SourceProducer::Text(TextProducer::Residence(entry)) => Some(entry),
                    _ => None,
                };
                if entry.is_some_and(|entry| acquisition.same_source(&entry)) {
                    if carrier_homes.len() != 1 || *carrier_homes[0] != LiveHomeV1::Text(acquisition.clone()) {
                        return Err(reject("execution-text-entry-home"));
                    }
                } else if !carrier_homes.is_empty() {
                    return Err(reject("execution-text-transferred-home"));
                }
            }
            TextProducer::Header(_) => match state.basis.entry_producer() {
                SourceProducer::Text(TextProducer::Residence(entry)) => {
                    if carrier_homes.len() != 1 || *carrier_homes[0] != LiveHomeV1::Text(entry.clone()) {
                        return Err(reject("execution-text-header-home"));
                    }
                }
                SourceProducer::Text(TextProducer::Static { .. }) if carrier_homes.is_empty() => {},
                SourceProducer::Text(entry @ TextProducer::After { .. })
                    if carrier_homes.is_empty()
                        && text_after_entry_matches(input, *binding, state.basis.entry(), entry) => {},
                _ => return Err(reject("execution-text-header-entry")),
            },
            TextProducer::After { .. } => return Err(reject("execution-text-after-active")),
            TextProducer::Static { .. } if carrier_homes.is_empty() => {},
            TextProducer::Static { .. } => return Err(reject("execution-text-static-home")),
            TextProducer::BranchNormal(source)
                if source.binding() == *binding && source.is_owned_residence_tree_v1()
                    && carrier_homes.is_empty() => {},
            TextProducer::BranchNormal(_) => return Err(reject("execution-text-edge-producer")),
        }
    }
    Ok(())
}

fn text_after_entry_matches(
    input: ResolvedFunctionLoweringInputV1<'_>,
    binding: crate::mir::resolved_semantics::BindingRefV1,
    value: crate::mir::resolved_semantics::source_value_flow::SourceValueRefV1,
    producer: &TextProducer,
) -> bool {
    use crate::mir::resolved_semantics::source_value_flow::SourceValueJoinKindV1 as Join;
    let TextProducer::After { product, binding: source_binding } = producer else { return false; };
    if *source_binding != binding || product.source.source.owner() != input.owner() { return false; }
    let Some(basis) = product.source.headers.get(&binding) else { return false; };
    if basis.entry_producer().class() != Class::String { return false; }
    let Some(flow) = input.function().expression_source().value_flow() else { return false; };
    let Some((after, _)) = flow.loop_join(product.source.source.site(), binding, Join::LoopAfter) else { return false; };
    if after != value { return false; }
    let Ok(incoming) = &product.source.incoming else { return false; };
    let mut rows = incoming.iter().filter(|row| row.after == after
        && row.basis.binding() == binding && Rc::ptr_eq(&row.basis, basis));
    rows.next().is_some() && rows.next().is_none()
}

fn branch_text_carrier_join(
    input: ResolvedFunctionLoweringInputV1<'_>,
    site: &crate::mir::resolved_semantics::SourceStmtSiteV1,
    binding: crate::mir::resolved_semantics::BindingRefV1,
    then_state: &SourceLoopTextCarrierStateV2,
    else_state: &SourceLoopTextCarrierStateV2,
) -> Result<SourceLoopTextCarrierStateV2, String> {
    use crate::mir::resolved_semantics::home_new_prefix::BranchNormalTextSourceV1;
    if !Rc::ptr_eq(&then_state.basis, &else_state.basis)
        || then_state.basis.binding() != binding {
        return Err(reject("execution-text-branch-basis"));
    }
    let flow = input.function().expression_source().value_flow()
        .ok_or_else(|| reject("execution-text-branch-flow"))?;
    if flow.owner() != input.owner() {
        return Err(reject("execution-text-branch-owner"));
    }
    let (value, incoming) = flow.branch_join(site, binding)
        .ok_or_else(|| reject("execution-text-branch-source"))?;
    let [(then_edge, then_value), (else_edge, else_value)] = incoming else {
        return Err(reject("execution-text-branch-arms"));
    };
    if !matches!((then_edge, else_edge), (Edge::ThenNormal, Edge::ElseNormal)) {
        return Err(reject("execution-text-branch-order"));
    }
    let joined = BranchNormalTextSourceV1::new(
        std::sync::Arc::clone(flow), value, binding, site.clone(), Box::new([
            (then_edge.clone(), *then_value, then_state.producer.clone()),
            (else_edge.clone(), *else_value, else_state.producer.clone()),
        ]),
    ).ok_or_else(|| reject("execution-text-branch-producer"))?;
    Ok(SourceLoopTextCarrierStateV2 {
        basis: Rc::clone(&then_state.basis),
        producer: TextProducer::BranchNormal(joined),
    })
}

fn merge_text_carrier_branches(
    input: ResolvedFunctionLoweringInputV1<'_>,
    site: &crate::mir::resolved_semantics::SourceStmtSiteV1,
    mut then_carriers: BTreeMap<crate::mir::resolved_semantics::BindingRefV1, SourceLoopTextCarrierStateV2>,
    else_carriers: &BTreeMap<crate::mir::resolved_semantics::BindingRefV1, SourceLoopTextCarrierStateV2>,
) -> Result<BTreeMap<crate::mir::resolved_semantics::BindingRefV1, SourceLoopTextCarrierStateV2>, String> {
    if then_carriers.keys().ne(else_carriers.keys()) {
        return Err(reject("execution-text-branch-membership"));
    }
    for (binding, then_state) in &mut then_carriers {
        let else_state = else_carriers.get(binding)
            .ok_or_else(|| reject("execution-text-branch-membership"))?;
        if !Rc::ptr_eq(&then_state.basis, &else_state.basis) {
            return Err(reject("execution-text-branch-basis"));
        }
        if then_state.producer != else_state.producer {
            *then_state = branch_text_carrier_join(input, site, *binding, then_state, else_state)?;
        }
    }
    Ok(then_carriers)
}

fn body(input: ResolvedFunctionLoweringInputV1<'_>, source: &Summary, tree: &Body,
    mut homes: Vec<LiveHomeV1>, mut text_carriers: BTreeMap<crate::mir::resolved_semantics::BindingRefV1, SourceLoopTextCarrierStateV2>,
    output: &mut Vec<SourceLoopEvaluationLifecycleV2>,
) -> Result<Option<(Vec<LiveHomeV1>, BTreeMap<crate::mir::resolved_semantics::BindingRefV1, SourceLoopTextCarrierStateV2>)>, String> {
    for statement in tree.statements.iter() {
        match statement {
            Statement::Return(source) => {
                if !source.corroborates(input) || source.homes.homes().iter().ne(homes.iter().rev()) {
                    return Err(reject("execution-normal-home-closure"));
                }
                return Ok(None);
            }
            Statement::Call { observation, .. } => {
                if observation.prior_homes() != homes.as_slice() {
                    return Err(reject("execution-prior-homes"));
                }
                if matches!(observation.result(), LocalCallResultClassV1::Handle | LocalCallResultClassV1::Nullable) {
                    let (_, binding) = observation.local_binding()
                        .ok_or_else(|| reject("execution-initializer-owned-result"))?;
                    if homes.iter().any(|home| home.binding() == binding) {
                        return Err(reject("execution-home-reacquisition"));
                    }
                    // Acquisition happens only after the SAME Call's Normal.
                    // Its installed CallReceived retains End and Fault duties.
                    homes.push(LiveHomeV1::Stable(binding));
                }
            }
            Statement::OwnedArrayAppend { observation, .. } => {
                let crate::mir::resolved_semantics::home_new_prefix::NamedArrayHomeObservationV1::Append {
                    source: holder, availability, prior_homes, ..
                } = observation else { return Err(reject("execution-array-observation")); };
                if availability.is_err() || prior_homes.as_ref() != homes.as_slice()
                    || homes.iter().filter(|home| home.binding() == holder.binding()).count() != 1 {
                    return Err(reject("execution-array-homes"));
                }
                // The same holder survives Normal and backedge. Candidate
                // acquisition/cancellation belongs to the existing Append
                // packet, never a speculative source-loop acquisition.
            }
            Statement::Expression { frame, .. } => {
                if let Some(initializer) = frame.initializer_frame() {
                    output.push(evaluation(input, source, initializer.evaluation(), &homes, initializer.owns_root(), &text_carriers)?);
                    if initializer.is_text_residence() {
                        if homes.iter().any(|home| home.binding() == initializer.binding()) { return Err(reject("execution-home-reacquisition")); }
                        homes.push(LiveHomeV1::Text(initializer.text_acquisition_v1()));
                    } else if initializer.is_dynamic_invocation_result() {
                        if homes.iter().any(|home| home.binding() == initializer.binding()) {
                            return Err(reject("execution-home-reacquisition"));
                        }
                        // The checked Dynamic initializer owns the carrier
                        // after Normal. Keep that exact local in the loop's
                        // existing Home state so later Fault and exit/port
                        // cleanup reuse its LocalCommit End plan.
                        homes.push(LiveHomeV1::Stable(initializer.binding()));
                    } else if initializer.owns_root() {
                        return Err(reject("execution-initializer-owned-result"));
                    }
                } else if let Some(rebind) = frame.rebind_frame() {
                    let mut lifecycle = evaluation(input, source, rebind.evaluation(), &homes,
                        rebind.is_text_residence() || rebind.is_dynamic_loop_carrier_v1(), &text_carriers)?;
                    if rebind.is_text_residence() {
                        homes.retain(|home| home.binding() != rebind.target());
                        if let Some(basis) = rebind.text_loop_carrier_v1() {
                            let state = text_carriers.get_mut(&rebind.target())
                                .filter(|state| Rc::ptr_eq(&state.basis, basis))
                                .ok_or_else(|| reject("execution-text-rebind-basis"))?;
                            let old_residence = match &state.producer {
                                TextProducer::Header(actual) if Rc::ptr_eq(actual, basis) => SourceLoopTextOldResidenceV2::ConditionalHeader,
                                TextProducer::Residence(_) => SourceLoopTextOldResidenceV2::OwnedResidence,
                                TextProducer::BranchNormal(source) if source.is_owned_residence_tree_v1() =>
                                    SourceLoopTextOldResidenceV2::OwnedResidence,
                                _ => return Err(reject("execution-text-rebind-prior")),
                            };
                            let acquisition = rebind.text_acquisition_v1();
                            lifecycle.text_carrier_transition = Some(SourceLoopTextCarrierTransitionV2 {
                                basis: Rc::clone(basis), evaluation: Rc::clone(rebind.source()),
                                before: state.producer.clone(), after: acquisition.clone(), old_residence,
                            });
                            state.producer = TextProducer::Residence(acquisition);
                        } else {
                            homes.push(LiveHomeV1::Text(rebind.text_acquisition_v1()));
                        }
                    }
                    output.push(lifecycle);
                } else { return Err(reject("execution-root-purpose")); }
            }
            Statement::If { site, condition, then_body, else_body, .. } => {
                output.push(bool_condition_execution(input, condition, &homes, &text_carriers)?);
                let then_homes = body(input, source, then_body, homes.clone(), text_carriers.clone(), output)?;
                let else_homes = match else_body {
                    Some(branch) => body(input, source, branch, homes.clone(), text_carriers.clone(), output)?,
                    None => Some((homes.clone(), text_carriers.clone())),
                };
                let (joined_homes, joined_carriers) = match (then_homes, else_homes) {
                    (None, None) => return Ok(None),
                    (Some(state), None) | (None, Some(state)) => state,
                    (Some((a_homes, a_carriers)), Some((b_homes, b_carriers))) => {
                        if a_homes != b_homes {
                            return Err(reject("execution-branch-home-join"));
                        }
                        let carriers = merge_text_carrier_branches(input, site, a_carriers, &b_carriers)?;
                        (a_homes, carriers)
                    }
                };
                homes = joined_homes;
                text_carriers = joined_carriers;
            }
            Statement::Exit { port, .. } => { normal_port(input, source, *port, &homes, &text_carriers)?; return Ok(None); }
        }
    }
    Ok(Some((homes, text_carriers)))
}
pub(super) fn verify(input: ResolvedFunctionLoweringInputV1<'_>, source: &Summary)
    -> Result<Box<[SourceLoopEvaluationLifecycleV2]>, String> {
    let mut text_carriers = BTreeMap::new();
    for (&binding, basis) in &source.headers {
        if basis.entry_producer().class() != Class::String { continue; }
        if !matches!(basis.entry_producer(), SourceProducer::Text(_)) { return Err(reject("execution-text-entry-class")); }
        let state = SourceLoopTextCarrierStateV2 { basis: Rc::clone(basis), producer: TextProducer::Header(Rc::clone(basis)) };
        if text_carriers.insert(binding, state).is_some() { return Err(reject("execution-text-entry-duplicate")); }
    }
    let mut output = vec![bool_condition_execution(input, &source.condition, source.condition.prior_homes(), &text_carriers)?];
    let normal = body(input, source, source.control.as_ref().map_err(|reason| reject(reason))?,
        source.condition.prior_homes().to_vec(), text_carriers, &mut output)?;
    if let Some((homes, text_carriers)) = normal {
        let mut ports = source.ports.iter().enumerate().filter(|(_, port)|
            matches!(port.kind, crate::mir::resolved_semantics::home_new_prefix::SourceLoopPortKindV1::Backedge));
        let (index, _) = ports.next().ok_or_else(|| reject("execution-backedge-missing"))?;
        if ports.next().is_some() { return Err(reject("execution-backedge-duplicate")); }
        normal_port(input, source, index, &homes, &text_carriers)?;
    }
    Ok(output.into_boxed_slice())
}

impl super::source_loop_v2::VerifiedSourceLoopRecipeProductV2 {
    /// Complete port Fault responsibility with genuine acquired Text roots
    /// forwarded on Normal. Their acquisition order comes from the SAME
    /// verified execution walk, not binding-map order or physical SSA IDs.
    pub(crate) fn port_fault_homes_v2(&self, port: usize)
        -> Result<Vec<crate::mir::resolved_semantics::home_new_prefix::LiveHomeV1>, String> {
        use crate::mir::resolved_semantics::home_new_prefix::{LiveHomeV1, SourceLoopPortKindV1 as Port};
        use crate::mir::resolved_semantics::source_value_flow::SourceValueEdgeV1 as Edge;
        let port = self.source.ports.get(port).ok_or_else(|| reject("execution-normal-port"))?;
        let edge = match &port.kind {
            Port::Backedge => Edge::Backedge,
            Port::Break(site) => Edge::Break(site.clone()),
            Port::Continue(site) => Edge::Continue(site.clone()),
        };
        let incoming = self.source.incoming.as_ref().map_err(|reason| reject(reason))?;
        let mut selected = Vec::new();
        for carrier in incoming.iter() {
            let mut rows = carrier.header_incoming.iter().chain(carrier.after_incoming.iter())
                .filter(|row| row.edge == edge);
            let row = rows.next().ok_or_else(|| reject("execution-text-edge-missing"))?;
            if rows.next().is_some() { return Err(reject("execution-text-edge-duplicate")); }
            if let SourceProducer::Text(TextProducer::Residence(acquisition)) = &row.producer {
                let home = LiveHomeV1::Text(acquisition.clone());
                if !port.homes().contains(&home) { selected.push(acquisition.clone()); }
            }
        }
        let mut homes = port.homes().to_vec();
        let mut inserted = 0;
        let mut prior_len = 0;
        for lifecycle in self.execution.iter() {
            let Some(transition) = &lifecycle.text_carrier_transition else { continue; };
            if !selected.iter().any(|acquisition| acquisition.same_source(&transition.after)) { continue; }
            let acquisition = &transition.after;
            let frame = acquisition.rebind_origin_v1().ok_or_else(|| reject("execution-text-rebind-basis"))?;
            // Successful carrier rebinds already consumed the previous target
            // Homes. Keep all other prior Homes and compare the surviving
            // prefix, rather than counting a displaced entry generation.
            let prior: Vec<_> = frame.evaluation().prior_homes().iter().filter(|home|
                !selected.iter().any(|fresh| fresh.binding() == home.binding())).cloned().collect();
            if !Rc::ptr_eq(frame.source(), &lifecycle.source)
                || !port.homes().starts_with(&prior) || prior.len() < prior_len
                || selected.iter().filter(|other| other.same_source(acquisition)).count() != 1 {
                return Err(reject("execution-prior-homes"));
            }
            // Each actual rebind root is already installed and retains its
            // own original LocalCommit. It has not transferred into the Phi
            // until the Normal port edge, so Fault still owes its exact End.
            homes.insert(prior.len() + inserted, LiveHomeV1::Text(acquisition.clone()));
            inserted += 1;
            prior_len = prior.len();
        }
        if inserted != selected.len() { return Err(reject("execution-text-edge-producer")); }
        Ok(homes)
    }

}

impl super::source_loop_v2::VerifiedSourceLoopRecipeProductV2 {
    pub(crate) fn expression_text_fault_producers_v2(&self, source: &Rc<Source>)
        -> Result<Vec<(crate::mir::resolved_semantics::BindingRefV1, TextProducer)>, String> {
        let mut rows = self.schedule().frames().iter().zip(self.execution.iter())
            .filter(|(frame, _)| Rc::ptr_eq(frame.source(), source));
        let (_, lifecycle) = rows.next().ok_or_else(|| reject("schedule-original-frame"))?;
        if rows.next().is_some() || !Rc::ptr_eq(&lifecycle.source, source)
            || lifecycle.faults.iter().any(|fault| fault.text_carriers != lifecycle.text_carriers) {
            return Err(reject("schedule-original-frame"));
        }
        Ok(lifecycle.text_carriers.iter().map(|state|
            (state.basis.binding(), state.producer.clone())).collect())
    }
}
