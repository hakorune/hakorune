//! Install the original whole Bool condition in the shared Home transaction.
//! Applicability is source morphology; selected refusal never retries a profile.
use super::*;
use crate::mir::resolved_semantics::home_new_prefix::LiveHomeV1;
use crate::mir::resolved_semantics::BodyExpressionShapeV1;
use crate::mir::resolved_value_profile::expression_plan::{
    condition_expression_source_plan_v1, ExpressionNodeSourceV1 as Node, ExpressionSourcePlanV1,
};
use std::rc::Rc;

pub(super) enum ConditionInstallationV1 {
    Unselected,
    Refused,
    Installed,
}

/// Request the original verified walk even with no existing Home. This passive
/// morphology selects no operand class, frame, terminal or physical permission.
fn requests_condition(
    input: ResolvedFunctionLoweringInputV1<'_>,
    condition: &SourceExprSiteV1,
) -> bool {
    use crate::mir::resolved_semantics::ResolvedBinaryOperatorV1 as Op;
    input.function().method_call(condition).is_some_and(|call| {
        call.selector() == "has"
            && input.body_shape().is_some_and(|shape| {
                matches!(
                    shape.expression_shape(call.receiver_site()),
                    Some(BodyExpressionShapeV1::FieldAccess { .. })
                )
            })
    }) || matches!(
        input.function().variable_ref(condition),
        Some(ResolvedLexicalRefV1::Local(_))
    ) || input
        .function()
        .expression_source()
        .binary(condition)
        .is_some_and(|binary| {
            matches!(
                binary.operator(),
                Op::And
                    | Op::Or
                    | Op::Equal
                    | Op::NotEqual
                    | Op::Less
                    | Op::LessEqual
                    | Op::Greater
                    | Op::GreaterEqual
            )
        })
}
pub(crate) fn requires_condition_expression_walk_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
) -> bool {
    use crate::mir::resolved_semantics::{SourcePathSegmentV1, SourcePathV1};
    input.function().if_region_sites().any(|statement| {
        let condition = SourcePathV1::from_node(statement.node())
            .child(SourcePathSegmentV1::IfCondition)
            .expr();
        requests_condition(input, &condition) || has_unconditional_nested_call_v1(input, &statement)
    })
}

pub(crate) fn requires_nested_call_argument_walk_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
) -> bool {
    input
        .function()
        .if_region_sites()
        .any(|statement| has_unconditional_nested_call_v1(input, &statement))
}

/// A call nested in another call's original argument needs the same Prefix
/// walk as Local/Return expressions so its owned result and cleanup stay tied
/// to that argument frame. This selects only source plans whose child reach is
/// unconditional; short-circuit calls keep their existing control owner.
fn has_unconditional_nested_call_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
    statement: &SourceStmtSiteV1,
) -> bool {
    use crate::mir::resolved_value_profile::{
        expression_children::{ExpressionChildReachV1, ExpressionChildrenV1},
        expression_plan::ExpressionNodeSourceV1,
    };
    let Ok(source) = condition_expression_source_plan_v1(input, statement) else {
        return false;
    };
    if source.nodes().iter().any(|node| {
        !matches!(node.children(), ExpressionChildrenV1::Known(children)
            if children.iter().all(|child| child.reach == ExpressionChildReachV1::Always))
    }) {
        return false;
    }
    source.nodes().iter().any(|outer| {
        if !matches!(outer.source(), ExpressionNodeSourceV1::MethodCall) {
            return false;
        }
        let Some(call) = input.function().method_call(outer.site()) else {
            return false;
        };
        call.arguments().iter().any(|argument| {
            source.nodes().iter().any(|inner| {
                inner
                    .site()
                    .node()
                    .segments()
                    .starts_with(argument.site().node().segments())
                    && matches!(inner.source(), ExpressionNodeSourceV1::MethodCall)
            })
        })
    })
}

/// Selection retains original local Home obligations even when a field has no
/// ready frame. Formal and received receiver lanes keep their existing owners.
fn contains_local_home_field(
    input: ResolvedFunctionLoweringInputV1<'_>,
    source: &ExpressionSourcePlanV1,
    locals: &PrefixLocalFlow<'_>,
) -> bool {
    source.nodes().iter().any(|node| {
        let Node::Shape(BodyExpressionShapeV1::FieldAccess { object, .. }) = node.source() else {
            return false;
        };
        let Some(ResolvedLexicalRefV1::Local(binding)) = input.function().variable_ref(object)
        else {
            return false;
        };
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
fn selects(
    input: ResolvedFunctionLoweringInputV1<'_>,
    source: &ExpressionSourcePlanV1,
    statement: &SourceStmtSiteV1,
    locals: &PrefixLocalFlow<'_>,
    local_calls: &[LocalCallObservationV1],
) -> bool {
    use crate::mir::resolved_semantics::{
        ResolvedBinaryOperatorV1 as Op, ResolvedLiteralSourceV1 as Literal,
    };
    fn supported(source: &ExpressionSourcePlanV1, index: usize) -> bool {
        let node = &source.nodes()[index];
        if let Some(binary) = node.binary() {
            if !matches!(
                binary.operator(),
                Op::And
                    | Op::Or
                    | Op::Equal
                    | Op::NotEqual
                    | Op::Less
                    | Op::Greater
                    | Op::LessEqual
                    | Op::GreaterEqual
                    | Op::Add
                    | Op::Subtract
            ) {
                return false;
            }
            return [binary.lhs(), binary.rhs()].into_iter().all(|site| {
                source
                    .nodes()
                    .iter()
                    .position(|node| node.site() == site)
                    .is_some_and(|child| child != index && supported(source, child))
            });
        }
        if let Node::Shape(crate::mir::resolved_semantics::BodyExpressionShapeV1::FieldAccess {
            object,
            ..
        }) = node.source()
        {
            return source
                .nodes()
                .iter()
                .find(|node| node.site() == object)
                .is_some_and(|receiver| {
                    matches!(
                        receiver.source(),
                        Node::Reference(
                            crate::mir::resolved_semantics::ResolvedLexicalRefV1::Local(_)
                        ) | Node::Shape(
                            crate::mir::resolved_semantics::BodyExpressionShapeV1::Me {
                                receiver: crate::mir::resolved_semantics::BodyMeReceiverV1::Lexical(
                                    _
                                ),
                                ..
                            }
                        )
                    )
                });
        }
        matches!(
            node.source(),
            Node::MethodCall
                | Node::Literal(
                    Literal::Integer(_) | Literal::Bool(_) | Literal::String(_) | Literal::Null
                )
                | Node::Reference(crate::mir::resolved_semantics::ResolvedLexicalRefV1::Local(
                    _
                ))
        )
    }
    // Only direct expression Calls participate here. A Call owns its receiver
    // and actual subtrees; those are not sibling scalar-expression demands.
    fn instance_calls(
        input: ResolvedFunctionLoweringInputV1<'_>,
        source: &ExpressionSourcePlanV1,
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
        if !matches!(node.source(), Node::MethodCall) {
            return Some(0);
        }
        let call = input.function().method_call(node.site())?;
        let receiver = source
            .nodes()
            .iter()
            .find(|node| node.site() == call.receiver_site())?;
        matches!(
            receiver.source(),
            Node::Reference(crate::mir::resolved_semantics::ResolvedLexicalRefV1::Local(
                _
            )) | Node::Shape(crate::mir::resolved_semantics::BodyExpressionShapeV1::Me {
                receiver: crate::mir::resolved_semantics::BodyMeReceiverV1::Lexical(_),
                ..
            })
        )
        .then_some(1)
    }
    let Some(binary) = source.nodes().first().and_then(|node| node.binary()) else {
        // Borrow the existing Bool condition owner, including its exact lexical
        // read and final producer correspondence. No scalar/Loop permission is
        // inferred from a bare reference or from its eventual MIR carrier.
        if source
            .nodes()
            .first()
            .is_some_and(|node| matches!(node.source(), Node::MethodCall))
        {
            return requests_condition(input, source.nodes()[0].site())
                || has_unconditional_nested_call_v1(input, statement);
        }
        return !locals.in_source_loop_v1()
            && source.nodes().len() == 1
            && matches!(
                source.nodes()[0].source(),
                Node::Reference(ResolvedLexicalRefV1::Local(_))
            );
    };
    // Plain scalar comparisons borrow a complete original read census. Calls
    // and field receivers retain their existing owners; one scalar sibling does
    // not authorize selecting their whole expression through another Call lane.
    let plain_scalar = source.nodes().iter().all(|node| match node.source() {
        Node::Binary(_) | Node::Literal(Literal::Integer(_) | Literal::Bool(_)) => true,
        Node::Reference(_) => locals
            .current_scalar_read_v1(node.site(), local_calls)
            .is_some(),
        _ => false,
    }) && source
        .nodes()
        .iter()
        .any(|node| matches!(node.source(), Node::Reference(_)));
    let nullable_null = binary.operator() == Op::Equal
        && [(binary.lhs(), binary.rhs()), (binary.rhs(), binary.lhs())]
            .into_iter()
            .any(|(read, null)| {
                locals.stored_get_reference_v1(read).is_some()
                    && source.nodes().iter().any(|node| {
                        node.site() == null && matches!(node.source(), Node::Literal(Literal::Null))
                    })
            });
    let selected = nullable_null
        || locals.in_source_loop_v1()
            && matches!(
                binary.operator(),
                Op::Equal
                    | Op::NotEqual
                    | Op::Less
                    | Op::Greater
                    | Op::LessEqual
                    | Op::GreaterEqual
            )
        || (plain_scalar
            && matches!(
                binary.operator(),
                Op::Equal
                    | Op::NotEqual
                    | Op::Less
                    | Op::Greater
                    | Op::LessEqual
                    | Op::GreaterEqual
            ))
        || matches!(binary.operator(), Op::And | Op::Or)
        || source.nodes_outside_calls().any(|(_, node)| {
            matches!(node.source(), Node::Literal(Literal::String(_)))
                || matches!(node.source(), Node::Reference(_))
                    && (locals.static_text_reference(node.site()).is_some()
                        || locals.text_reference(node.site()).is_some())
        })
        || (matches!(
            binary.operator(),
            Op::Equal | Op::NotEqual | Op::Less | Op::Greater | Op::LessEqual | Op::GreaterEqual
        ) && instance_calls(input, source, 0).is_some_and(|count| count > 0));
    selected && supported(source, 0)
}

pub(super) fn observe<E>(
    input: ResolvedFunctionLoweringInputV1<'_>,
    statement: &SourceStmtSiteV1,
    locals: &PrefixLocalFlow<'_>,
    homes: &[LiveHomeV1],
    prefix_known: bool,
    local_calls: &mut Vec<LocalCallObservationV1>,
    path_calls: &mut BTreeSet<OwnedExprSiteV1>,
    fields: &mut impl FnMut(
        &[LocalFieldReadRequestV1],
        LocalFieldReadDemandV1,
    ) -> Result<Option<Vec<LocalFieldReadResultV1>>, E>,
    actuals: &mut impl FnMut(
        &OwnedExprSiteV1,
        BorrowedCallActualRequestV1<'_>,
    ) -> Result<Option<BorrowedCallArgumentsV1>, E>,
) -> Result<ConditionInstallationV1, E> {
    let condition = crate::mir::resolved_semantics::SourcePathV1::from_node(statement.node())
        .child(crate::mir::resolved_semantics::SourcePathSegmentV1::IfCondition)
        .expr();
    let original_profile = requests_condition(input, &condition);
    let nested_call_profile = has_unconditional_nested_call_v1(input, statement);
    let Ok(source) = condition_expression_source_plan_v1(input, statement) else {
        return Ok(ConditionInstallationV1::Refused);
    };
    // Original finite-input membership selects this owner before execution.
    // Other borrowed profiles keep their existing null-guard owner; a selected
    // finite-input failure never retries that owner.
    let mut entry_guard = Vec::new();
    if let Some(binary) = source.nodes().first().and_then(|node| node.binary()) {
        if binary.operator() == crate::mir::resolved_semantics::ResolvedBinaryOperatorV1::Equal {
            for (read, null) in [(binary.lhs(), binary.rhs()), (binary.rhs(), binary.lhs())] {
                if !source.nodes().iter().any(|node| {
                    node.site() == null
                        && matches!(
                            node.source(),
                            Node::Literal(
                                crate::mir::resolved_semantics::ResolvedLiteralSourceV1::Null
                            )
                        )
                }) {
                    continue;
                }
                let Some((binding, root)) =
                    super::argument_expression::text_reference(input, locals, read)
                else {
                    continue;
                };
                let owned = OwnedExprSiteV1::new(input.owner(), source.nodes()[0].site().clone());
                if let Some(BorrowedCallArgumentsV1::FiniteEntryRead(original)) = actuals(
                    &owned,
                    BorrowedCallActualRequestV1::FiniteEntryRead {
                        input,
                        plan: &source,
                        read,
                        binding,
                        root,
                    },
                )? {
                    if original.null_compare_v1().is_some() {
                        let Some(index) =
                            source.nodes().iter().position(|node| node.site() == read)
                        else {
                            return Ok(ConditionInstallationV1::Refused);
                        };
                        entry_guard.push((
                            index,
                            CheckedExpressionReferenceV1::FiniteEntry {
                                read: original,
                                exclusion: None,
                            },
                        ));
                    }
                }
            }
        }
    }
    let selected = (original_profile || nested_call_profile)
        && selects(input, &source, statement, locals, local_calls);
    if entry_guard.is_empty()
        && !contains_local_home_field(input, &source, locals)
        && !selected
    {
        return Ok(ConditionInstallationV1::Unselected);
    }
    if !prefix_known {
        return Ok(ConditionInstallationV1::Refused);
    }
    let installed = install_source(
            input,
            source,
            &entry_guard,
            locals,
            homes,
            local_calls,
            path_calls,
            fields,
            actuals,
        )?;
    Ok(
        if installed.is_some() {
            ConditionInstallationV1::Installed
        } else {
            ConditionInstallationV1::Refused
        },
    )
}

pub(super) fn observe_loop<E>(
    input: ResolvedFunctionLoweringInputV1<'_>,
    source: &Rc<crate::mir::builder::RetainedCallableLoopSourceV1>,
    locals: &PrefixLocalFlow<'_>,
    homes: &[LiveHomeV1],
    local_calls: &mut Vec<LocalCallObservationV1>,
    path_calls: &mut BTreeSet<OwnedExprSiteV1>,
    fields: &mut impl FnMut(
        &[LocalFieldReadRequestV1],
        LocalFieldReadDemandV1,
    ) -> Result<Option<Vec<LocalFieldReadResultV1>>, E>,
    actuals: &mut impl FnMut(
        &OwnedExprSiteV1,
        BorrowedCallActualRequestV1<'_>,
    ) -> Result<Option<BorrowedCallArgumentsV1>, E>,
) -> Result<Option<Rc<CheckedBoolConditionFrameV1>>, E> {
    if !locals.in_source_loop_v1() {
        return Ok(None);
    }
    let Some(condition) = source.condition_source_v1(input) else {
        return Ok(None);
    };
    let Some(frame) = install_source(
        input,
        Rc::clone(condition),
        &[],
        locals,
        homes,
        local_calls,
        path_calls,
        fields,
        actuals,
    )? else { return Ok(None); };

    // A LoopCondition's call nodes are retained by its BoolCondition owner,
    // while their argument purposes live as separate checked expression
    // frames. Stage those frames here so conditional calls can reuse them at
    // their original reach without asking the completed root-flow inventory.
    let Ok(statement) = input.source().exact_stmt(source.site()) else {
        return Ok(None);
    };
    for call in frame.calls() {
        let root = OwnedExprSiteV1::new(input.owner(), call.site().site().clone());
        super::local_call_flow::stage_argument_expressions(
            input, &statement, &root, locals, true, homes, local_calls, fields, actuals,
        )?;
    }
    Ok(Some(frame))
}

fn install_source<E>(
    input: ResolvedFunctionLoweringInputV1<'_>,
    source: Rc<ExpressionSourcePlanV1>,
    source_references: &[(usize, CheckedExpressionReferenceV1)],
    locals: &PrefixLocalFlow<'_>,
    homes: &[LiveHomeV1],
    local_calls: &mut Vec<LocalCallObservationV1>,
    path_calls: &mut BTreeSet<OwnedExprSiteV1>,
    fields: &mut impl FnMut(
        &[LocalFieldReadRequestV1],
        LocalFieldReadDemandV1,
    ) -> Result<Option<Vec<LocalFieldReadResultV1>>, E>,
    actuals: &mut impl FnMut(
        &OwnedExprSiteV1,
        BorrowedCallActualRequestV1<'_>,
    ) -> Result<Option<BorrowedCallArgumentsV1>, E>,
) -> Result<Option<Rc<CheckedBoolConditionFrameV1>>, E> {
    use super::expression_plan::CheckedScalarReferenceV1 as Scalar;
    let Some(prepared) = super::expression_plan::prepare_expression_calls_v1(
        &source,
        super::expression_plan::ExpressionLeafProfileV1::BoolCondition,
        &BTreeMap::new(),
        homes,
        &mut |site| {
            let mut receiver_path = |binding| locals.field_alias_call_path(binding);
            if let Some(ExpressionCallSourceV1::Ordinary(call)) =
                local_call_flow::issue_source_expression_call_v1(
                    input,
                    &source,
                    site,
                    homes,
                    &mut receiver_path,
                    actuals,
                )?
            {
                if call.dynamic_invocation_result_lifecycle_v1().is_some() {
                    return Ok(Some(ExpressionCallSourceV1::Ordinary(call)));
                }
            }
            match actuals(
                site,
                BorrowedCallActualRequestV1::ExpressionCoreSource {
                    input,
                    plan: &source,
                },
            )? {
                Some(BorrowedCallArgumentsV1::ExpressionCoreSource(core)) => {
                    return Ok(Some(ExpressionCallSourceV1::Core(core)))
                }
                Some(_) => return Ok(None),
                None => {}
            }
            if let Some(call) = local_call_flow::issue_source_static_bool_frame_call_v1(
                input,
                &source,
                site,
                homes,
                actuals,
            )? {
                return Ok(Some(ExpressionCallSourceV1::Ordinary(call)));
            }
            local_call_flow::issue_source_scalar_frame_call_v1(
                input,
                &source,
                site,
                homes,
                &mut |binding| locals.field_alias_call_path(binding),
                actuals,
            )
            .map(|call| call.map(ExpressionCallSourceV1::Ordinary))
        },
    )?
    else {
        return Ok(None);
    };
    let mut initial_references = source_references.to_vec();
    let owned = OwnedExprSiteV1::new(input.owner(), source.nodes()[0].site().clone());
    // Core inputs remain active; Ordinary Calls own their transferred children.
    for (index, node) in source.nodes().iter().enumerate() {
        if !prepared.is_active_reference(index)
            || initial_references
                .iter()
                .any(|(original, _)| *original == index)
        {
            continue;
        }
        if !matches!(node.source(), Node::Reference(_)) {
            continue;
        }
        if let Some(frame) = locals.stored_get_reference_v1(node.site()) {
            initial_references.push((index, CheckedExpressionReferenceV1::StoredMapGet(frame)));
            continue;
        }
        if let Some(frame) = locals.dynamic_invocation_result_reference_v1(node.site()) {
            initial_references.push((
                index,
                CheckedExpressionReferenceV1::DynamicInvocationResult(frame),
            ));
            continue;
        }
        if let Some(read) = locals.loop_dynamic_reference_v1(node.site()) {
            initial_references.push((index, CheckedExpressionReferenceV1::LoopDynamic(read)));
            continue;
        }
        let Some((binding, root)) =
            super::argument_expression::text_reference(input, locals, node.site())
        else {
            continue;
        };
        if let Some(BorrowedCallArgumentsV1::FiniteEntryRead(read)) = actuals(
            &owned,
            BorrowedCallActualRequestV1::FiniteEntryRead {
                input,
                plan: &source,
                read: node.site(),
                binding,
                root,
            },
        )? {
            initial_references.push((
                index,
                CheckedExpressionReferenceV1::FiniteEntry {
                    read,
                    exclusion: None,
                },
            ));
        }
    }
    let mut installation = None;
    let plan = qualify_condition_expression_v1(
        prepared,
        &initial_references,
        &mut |site| locals.static_text_reference(site),
        &mut |site| locals.text_reference(site),
        &mut |site| {
            let Some(OrdinaryObservation::TrivialLocal(binding, Some(kind))) = locals.observe(site)
            else {
                return None;
            };
            if let Some(read) = locals.current_scalar_read_v1(site, local_calls) {
                Some(Scalar::Original(read))
            } else if locals.in_source_loop_v1() {
                None
            } else {
                Some(Scalar::Obligation { binding, kind })
            }
        },
        &mut |site| super::field_read::field_read_request(input, site, locals),
        &mut |plan| {
            let mut candidate = locals.expression_installation();
            if !candidate.approve(plan) {
                return false;
            }
            installation = Some(candidate);
            true
        },
        &mut |requests, demand| {
            Ok(fields(requests, demand)?.is_some_and(|results| {
                results.len() == requests.len()
                    && results
                        .iter()
                        .all(|result| *result == LocalFieldReadResultV1::Scalar)
            }))
        },
    )?;
    let Some(plan) = plan else {
        return Ok(None);
    };
    let frame = Rc::clone(
        plan.condition_frame()
            .expect("Bool qualifier retains its frame"),
    );
    installation
        .take()
        .expect("approved condition installation")
        .commit();
    drop(installation);
    path_calls.extend(frame.calls().iter().map(|call| call.site().clone()));
    local_calls.extend_from_slice(frame.calls());
    Ok(Some(frame))
}
