//! Install the original whole Bool condition in the shared Home transaction.
//! Applicability is source morphology; selected refusal never retries a profile.
use crate::mir::resolved_semantics::home_new_prefix::LiveHomeV1;
use super::*;
use crate::mir::resolved_semantics::BodyExpressionShapeV1;
use crate::mir::resolved_value_profile::expression_plan::{
    condition_expression_source_plan_v1, ExpressionNodeSourceV1 as Node,
    ExpressionSourcePlanV1,
};
use std::rc::Rc;

pub(super) enum ConditionInstallationV1 { Unselected, Refused, Installed }

/// Request the original verified walk even with no existing Home. This passive
/// morphology selects no operand class, frame, terminal or physical permission.
pub(crate) fn requires_condition_expression_walk_v1(input: ResolvedFunctionLoweringInputV1<'_>) -> bool {
    use crate::mir::resolved_semantics::{ResolvedBinaryOperatorV1 as Op, SourcePathV1, SourcePathSegmentV1};
    input.function().if_region_sites().any(|statement| {
        let condition = SourcePathV1::from_node(statement.node()).child(SourcePathSegmentV1::IfCondition).expr();
        input.function().expression_source().binary(&condition).is_some_and(|binary| matches!(binary.operator(),
            Op::And | Op::Or | Op::Equal | Op::NotEqual | Op::Less | Op::LessEqual | Op::Greater | Op::GreaterEqual))
    })
}

/// Selection retains original local Home obligations even when a field has no
/// ready frame. Formal and received receiver lanes keep their existing owners.
fn contains_local_home_field(
    input: ResolvedFunctionLoweringInputV1<'_>, source: &ExpressionSourcePlanV1,
    locals: &PrefixLocalFlow<'_>,
) -> bool {
    source.nodes().iter().any(|node| {
        let Node::Shape(BodyExpressionShapeV1::FieldAccess { object, .. }) = node.source()
            else { return false };
        let Some(ResolvedLexicalRefV1::Local(binding)) = input.function().variable_ref(object)
            else { return false };
        let root = match locals.field_read_receiver(binding) {
            Some(local_flow::FieldReadReceiverV1::OwnedHome) => binding,
            Some(local_flow::FieldReadReceiverV1::RootedHandle(root)) => root,
            _ => return false,
        };
        locals.home_acquisition(root).is_some()
    })
}

/// Calls and field receivers remain opaque to this morphology selector. Their
/// exact original inputs are obligations of the shared leaf/Call/field owners.
fn selects(input: ResolvedFunctionLoweringInputV1<'_>, source: &ExpressionSourcePlanV1, locals: &PrefixLocalFlow<'_>) -> bool {
    use crate::mir::resolved_semantics::{ResolvedBinaryOperatorV1 as Op, ResolvedLiteralSourceV1 as Literal};
    fn supported(source: &ExpressionSourcePlanV1, index: usize) -> bool {
        let node = &source.nodes()[index];
        if let Some(binary) = node.binary() {
            if !matches!(binary.operator(), Op::And | Op::Or | Op::Equal | Op::NotEqual
                | Op::Less | Op::Greater | Op::LessEqual | Op::GreaterEqual | Op::Add) { return false; }
            return [binary.lhs(), binary.rhs()].into_iter().all(|site|
                source.nodes().iter().position(|node| node.site() == site)
                    .is_some_and(|child| child != index && supported(source, child)));
        }
        if let Node::Shape(crate::mir::resolved_semantics::BodyExpressionShapeV1::FieldAccess { object, .. }) = node.source() {
            return source.nodes().iter().find(|node| node.site() == object).is_some_and(|receiver|
                matches!(receiver.source(), Node::Reference(crate::mir::resolved_semantics::ResolvedLexicalRefV1::Local(_))
                    | Node::Shape(crate::mir::resolved_semantics::BodyExpressionShapeV1::Me {
                        receiver: crate::mir::resolved_semantics::BodyMeReceiverV1::Lexical(_), .. })));
        }
        matches!(node.source(), Node::MethodCall
            | Node::Literal(Literal::Integer(_) | Literal::Bool(_) | Literal::String(_))
            | Node::Reference(crate::mir::resolved_semantics::ResolvedLexicalRefV1::Local(_)))
    }
    // Only direct expression Calls participate here. A Call owns its receiver
    // and actual subtrees; those are not sibling scalar-expression demands.
    fn instance_calls(input: ResolvedFunctionLoweringInputV1<'_>, source: &ExpressionSourcePlanV1,
        index: usize,
    ) -> Option<usize> {
        let node = &source.nodes()[index];
        if let Some(binary) = node.binary() {
            let mut count = 0;
            for site in [binary.lhs(), binary.rhs()] {
                let child = source.nodes().iter().position(|node| node.site() == site)?;
                count += instance_calls(input, source, child)?;
            }
            return Some(count);
        }
        if !matches!(node.source(), Node::MethodCall) { return Some(0); }
        let call = input.function().method_call(node.site())?;
        let receiver = source.nodes().iter().find(|node| node.site() == call.receiver_site())?;
        matches!(receiver.source(),
            Node::Reference(crate::mir::resolved_semantics::ResolvedLexicalRefV1::Local(_))
            | Node::Shape(crate::mir::resolved_semantics::BodyExpressionShapeV1::Me {
                receiver: crate::mir::resolved_semantics::BodyMeReceiverV1::Lexical(_), .. }))
            .then_some(1)
    }
    let Some(binary) = source.nodes().first().and_then(|node| node.binary()) else { return false; };
    let selected = locals.in_source_loop_v1()
            && matches!(binary.operator(), Op::Equal | Op::NotEqual | Op::Less | Op::Greater
                | Op::LessEqual | Op::GreaterEqual)
        || matches!(binary.operator(), Op::And | Op::Or)
        || source.nodes_outside_calls().any(|(_, node)| matches!(node.source(), Node::Literal(Literal::String(_)))
            || matches!(node.source(), Node::Reference(_))
                && (locals.static_text_reference(node.site()).is_some() || locals.text_reference(node.site()).is_some()))
        || (matches!(binary.operator(), Op::Equal | Op::NotEqual | Op::Less | Op::Greater
            | Op::LessEqual | Op::GreaterEqual)
            && instance_calls(input, source, 0).is_some_and(|count| count > 0));
    selected && supported(source, 0)
}

pub(super) fn observe<E>(
    input: ResolvedFunctionLoweringInputV1<'_>, statement: &SourceStmtSiteV1,
    locals: &PrefixLocalFlow<'_>, homes: &[LiveHomeV1], prefix_known: bool,
    local_calls: &mut Vec<LocalCallObservationV1>, path_calls: &mut BTreeSet<OwnedExprSiteV1>,
    fields: &mut impl FnMut(&[LocalFieldReadRequestV1], LocalFieldReadDemandV1)
        -> Result<Option<Vec<LocalFieldReadResultV1>>, E>,
    actuals: &mut impl FnMut(&OwnedExprSiteV1, BorrowedCallActualRequestV1<'_>)
        -> Result<Option<BorrowedCallArgumentsV1>, E>,
) -> Result<ConditionInstallationV1, E> {
    let condition = crate::mir::resolved_semantics::SourcePathV1::from_node(statement.node())
        .child(crate::mir::resolved_semantics::SourcePathSegmentV1::IfCondition).expr();
    let original_profile = input.function().expression_source().binary(&condition).is_some_and(|binary|
        matches!(binary.operator(), crate::mir::resolved_semantics::ResolvedBinaryOperatorV1::And
            | crate::mir::resolved_semantics::ResolvedBinaryOperatorV1::Or
            | crate::mir::resolved_semantics::ResolvedBinaryOperatorV1::Equal
            | crate::mir::resolved_semantics::ResolvedBinaryOperatorV1::NotEqual
            | crate::mir::resolved_semantics::ResolvedBinaryOperatorV1::Less
            | crate::mir::resolved_semantics::ResolvedBinaryOperatorV1::Greater
            | crate::mir::resolved_semantics::ResolvedBinaryOperatorV1::LessEqual
            | crate::mir::resolved_semantics::ResolvedBinaryOperatorV1::GreaterEqual));
    let Ok(source) = condition_expression_source_plan_v1(input, statement)
        else { return Ok(ConditionInstallationV1::Refused) };
    if !contains_local_home_field(input, &source, locals)
        && !(original_profile && selects(input, &source, locals))
    { return Ok(ConditionInstallationV1::Unselected); }
    if !prefix_known { return Ok(ConditionInstallationV1::Refused); }
    Ok(if install_source(input, source, locals, homes, local_calls, path_calls, fields, actuals)?.is_some() {
        ConditionInstallationV1::Installed
    } else { ConditionInstallationV1::Refused })
}

pub(super) fn observe_loop<E>(
    input: ResolvedFunctionLoweringInputV1<'_>, source: &Rc<crate::mir::builder::RetainedCallableLoopSourceV1>,
    locals: &PrefixLocalFlow<'_>, homes: &[LiveHomeV1],
    local_calls: &mut Vec<LocalCallObservationV1>, path_calls: &mut BTreeSet<OwnedExprSiteV1>,
    fields: &mut impl FnMut(&[LocalFieldReadRequestV1], LocalFieldReadDemandV1)
        -> Result<Option<Vec<LocalFieldReadResultV1>>, E>,
    actuals: &mut impl FnMut(&OwnedExprSiteV1, BorrowedCallActualRequestV1<'_>)
        -> Result<Option<BorrowedCallArgumentsV1>, E>,
) -> Result<Option<Rc<CheckedBoolConditionFrameV1>>, E> {
    if !locals.in_source_loop_v1() { return Ok(None); }
    let Some(condition) = source.condition_source_v1(input) else { return Ok(None); };
    install_source(input, Rc::clone(condition), locals, homes, local_calls, path_calls, fields, actuals)
}

fn install_source<E>(
    input: ResolvedFunctionLoweringInputV1<'_>, source: Rc<ExpressionSourcePlanV1>,
    locals: &PrefixLocalFlow<'_>, homes: &[LiveHomeV1],
    local_calls: &mut Vec<LocalCallObservationV1>, path_calls: &mut BTreeSet<OwnedExprSiteV1>,
    fields: &mut impl FnMut(&[LocalFieldReadRequestV1], LocalFieldReadDemandV1)
        -> Result<Option<Vec<LocalFieldReadResultV1>>, E>,
    actuals: &mut impl FnMut(&OwnedExprSiteV1, BorrowedCallActualRequestV1<'_>)
        -> Result<Option<BorrowedCallArgumentsV1>, E>,
) -> Result<Option<Rc<CheckedBoolConditionFrameV1>>, E> {
    use super::expression_plan::CheckedScalarReferenceV1 as Scalar;
    let mut installation = None;
    let plan = qualify_condition_expression_v1(Rc::clone(&source), homes,
        &mut |site| locals.static_text_reference(site), &mut |site| locals.text_reference(site),
        &mut |site| {
            let Some(OrdinaryObservation::TrivialLocal(binding, Some(kind))) = locals.observe(site) else { return None; };
            if locals.in_source_loop_v1() {
                locals.current_scalar_read_v1(site, local_calls).map(Scalar::Original)
            } else { Some(Scalar::Obligation { binding, kind }) }
        },
        &mut |site| super::field_read::field_read_request(input, site, locals),
        &mut |site| local_call_flow::issue_source_scalar_frame_call_v1(input, &source, site,
            homes, &mut |binding| locals.field_alias_call_path(binding), actuals),
        &mut |plan| {
            let mut candidate = locals.expression_installation();
            if !candidate.approve(plan) { return false; }
            installation = Some(candidate);
            true
        },
        &mut |requests, demand| Ok(fields(requests, demand)?.is_some_and(|results|
            results.len() == requests.len() && results.iter().all(|result| *result == LocalFieldReadResultV1::Scalar))))?;
    let Some(plan) = plan else { return Ok(None) };
    let frame = Rc::clone(plan.condition_frame().expect("Bool qualifier retains its frame"));
    installation.take().expect("approved condition installation").commit();
    drop(installation);
    path_calls.extend(frame.calls().iter().map(|call| call.site().clone()));
    local_calls.extend_from_slice(frame.calls());
    Ok(Some(frame))
}
