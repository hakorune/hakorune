//! Source execution/lifecycle obligations of the SAME checked Loop tree.
//! These are analysis templates, never runtime acquisitions or physical grants.
use crate::mir::resolved_semantics::home_new_prefix::LiveHomeV1;
use super::source_loop_v2::{reject, Summary, Body, Statement, Evaluation, Source};
use std::{rc::Rc, collections::BTreeSet};
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::resolved_semantics::{OwnedExprSiteV1,
    CoreMethodHomeParameterRelationV1 as Parameter, CoreMethodHomeResultRelationV1 as ResultKind};
use crate::mir::resolved_value_profile::{expression_projection::ExpressionLeafClassV1 as Leaf,
    expression_class::ExpressionValueClassV1 as Class};

#[derive(Debug)]
pub(crate) struct SourceLoopFaultV2 {
    pub(crate) node: usize,
    pub(crate) continuation: crate::mir::resolved_control_flow::LoopExpressionFaultContinuationV1,
    /// Only acquisitions whose Normal already occurred before this operation.
    pub(crate) prior_acquisitions: Box<[usize]>,
}
#[derive(Debug)]
pub(crate) struct SourceLoopEvaluationLifecycleV2 {
    pub(crate) source: Rc<Source>,
    pub(crate) normal_temporary_ends: Box<[usize]>,
    pub(crate) faults: Box<[SourceLoopFaultV2]>,
}
fn fault(input: ResolvedFunctionLoweringInputV1<'_>, source: &Summary, frame: &Evaluation,
    node: usize, prior_acquisitions: &[usize],
) -> Result<SourceLoopFaultV2, String> {
    let site = OwnedExprSiteV1::new(input.owner(), frame.source().nodes()[node].site().clone());
    let continuation = crate::mir::resolved_control_flow::issue_loop_expression_fault_continuation_v1(
        input, source.source.site(), source.source.frame(), &site).map_err(reject)?;
    Ok(SourceLoopFaultV2 { node, continuation, prior_acquisitions: prior_acquisitions.to_vec().into_boxed_slice() })
}
fn evaluation(input: ResolvedFunctionLoweringInputV1<'_>, source: &Summary, frame: &Evaluation,
    homes: &[LiveHomeV1], retain_root: bool,
) -> Result<SourceLoopEvaluationLifecycleV2, String> {
    if frame.prior_homes() != homes { return Err(reject("execution-prior-homes")); }
    let mut acquisitions = Vec::new();
    let mut faults = Vec::new();
    let mut cores = 0;
    let mut text_comparisons = 0;
    for &node in frame.ordered_nodes() {
        if let Some(core) = frame.cores().get(&node) {
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
                    leaf(argument.site()) != Some(Leaf::Value(match parameter {
                        Parameter::I64Parameter => Class::I64,
                        Parameter::TextParameter => Class::String,
                        Parameter::TextRetainedByReceiver => return true,
                    }))
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
            faults.push(fault(input, source, frame, node, &acquisitions)?);
            cores += 1;
        } else if frame.operations().iter().any(|loan| loan.node() == node
            && loan.single_envelope().is_some_and(|envelope| {
                use crate::mir::dynamic_operator_contract::DynamicOperatorValueClassV1 as Value;
                envelope.domain().left() == Value::String && envelope.domain().right() == Value::String
            })) {
            // The same closed Bool source projector already seals exact family,
            // domain and conditional reach. Its comparisons acquire no owner.
            if frame.class(node) != Some(Leaf::Value(Class::Bool)) || frame.owned_result_nodes().contains(&node) {
                return Err(reject("execution-comparison-result"));
            }
            faults.push(fault(input, source, frame, node, &acquisitions)?);
            text_comparisons += 1;
        }
        if frame.owned_result_nodes().contains(&node) {
            if !frame.cores().get(&node).is_some_and(|core| core.owns_result()) { return Err(reject("execution-acquisition-family")); }
            acquisitions.push(node);
        }
    }
    if cores != frame.cores().len() || (text_comparisons != 0 && !frame.cores().is_empty()) {
        return Err(reject("execution-operation-census"));
    }
    let normal_temporary_ends = acquisitions.into_iter().rev()
        .filter(|node| !(retain_root && *node == 0)).collect();
    Ok(SourceLoopEvaluationLifecycleV2 { source: Rc::clone(frame.source()), normal_temporary_ends, faults: faults.into_boxed_slice() })
}
fn normal_port(input: ResolvedFunctionLoweringInputV1<'_>, source: &Summary,
    port: usize, homes: &[LiveHomeV1],
) -> Result<(), String> {
    let port = source.ports.get(port).ok_or_else(|| reject("execution-normal-port"))?;
    if port.homes() != homes { return Err(reject("execution-port-homes")); }
    let mut unique = BTreeSet::new();
    let mut ends = Vec::new();
    for home in homes.iter().rev() {
        let binding = input.function().binding(home.binding()).ok_or_else(|| reject("execution-home-binding"))?;
        if !unique.insert(home.binding()) { return Err(reject("execution-home-duplicate")); }
        if port.crossed_scopes().contains(&binding.owner_scope()) { ends.push(home.clone()); }
    }
    let retained: Vec<_> = homes.iter().cloned().filter(|home| !ends.contains(home)).collect();
    if ends.as_slice() != port.normal_ends() || retained != source.condition.prior_homes() {
        return Err(reject("execution-normal-home-closure"));
    }
    Ok(())
}
fn body(input: ResolvedFunctionLoweringInputV1<'_>, source: &Summary, tree: &Body,
    mut homes: Vec<LiveHomeV1>, output: &mut Vec<SourceLoopEvaluationLifecycleV2>,
) -> Result<Option<Vec<LiveHomeV1>>, String> {
    for statement in tree.statements.iter() {
        match statement {
            Statement::Call { observation, .. } => {
                if observation.prior_homes() != homes.as_slice() {
                    return Err(reject("execution-prior-homes"));
                }
                // Scalar Local/Discard leaves caller Homes unchanged. All
                // argument acquisition/End/Fault remains inside the SAME Call.
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
                    output.push(evaluation(input, source, initializer.evaluation(), &homes, initializer.owns_root())?);
                    if initializer.owns_root() {
                        if homes.iter().any(|home| home.binding() == initializer.binding()) { return Err(reject("execution-home-reacquisition")); }
                        homes.push(LiveHomeV1::Text(initializer.text_acquisition_v1()));
                    }
                } else if let Some(rebind) = frame.rebind_frame() {
                    output.push(evaluation(input, source, rebind.evaluation(), &homes, rebind.is_text_residence())?);
                    if rebind.is_text_residence() {
                        homes.retain(|home| home.binding() != rebind.target());
                        homes.push(LiveHomeV1::Text(rebind.text_acquisition_v1()));
                    }
                } else { return Err(reject("execution-root-purpose")); }
            }
            Statement::If { condition, then_body, else_body, .. } => {
                output.push(evaluation(input, source, condition.evaluation(), &homes, false)?);
                let then_homes = body(input, source, then_body, homes.clone(), output)?;
                let else_homes = match else_body {
                    Some(branch) => body(input, source, branch, homes.clone(), output)?,
                    None => Some(homes.clone()),
                };
                homes = match (then_homes, else_homes) {
                    (None, None) => return Ok(None),
                    (Some(homes), None) | (None, Some(homes)) => homes,
                    (Some(a), Some(b)) if a == b => a,
                    _ => return Err(reject("execution-branch-home-join")),
                };
            }
            Statement::Exit { port, .. } => { normal_port(input, source, *port, &homes)?; return Ok(None); }
        }
    }
    Ok(Some(homes))
}
pub(super) fn verify(input: ResolvedFunctionLoweringInputV1<'_>, source: &Summary)
    -> Result<Box<[SourceLoopEvaluationLifecycleV2]>, String> {
    let mut output = vec![evaluation(input, source, source.condition.evaluation(), source.condition.prior_homes(), false)?];
    let normal = body(input, source, source.control.as_ref().map_err(|reason| reject(reason))?,
        source.condition.prior_homes().to_vec(), &mut output)?;
    if let Some(homes) = normal {
        let mut ports = source.ports.iter().enumerate().filter(|(_, port)|
            matches!(port.kind, crate::mir::resolved_semantics::home_new_prefix::SourceLoopPortKindV1::Backedge));
        let (index, _) = ports.next().ok_or_else(|| reject("execution-backedge-missing"))?;
        if ports.next().is_some() { return Err(reject("execution-backedge-duplicate")); }
        normal_port(input, source, index, &homes)?;
    }
    Ok(output.into_boxed_slice())
}
