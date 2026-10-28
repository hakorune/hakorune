//! Receiver field-write admission inside `scan_statement_flow`.
//!
//! Receiver writes keep the existing `me` contract. A separate non-owning
//! formal path requires the canonical current non-null state, exact original
//! writer/receiver/value consults, a numeric field declaration and checked RHS.
//! Neither path acquires a Home or issues physical FieldSet permission.
use super::*;
use crate::mir::resolved_semantics::home_new_prefix::LiveHomeV1;
use std::rc::Rc;
#[path = "home_new_prefix_owned_field_write.rs"]
mod owned_local;
use crate::mir::resolved_semantics::{
    BodyExpressionShapeV1, BodyMeReceiverV1, ResolvedAssignmentFormV1, ResolvedAssignmentTargetV1,
    ResolvedBinaryOperatorV1, VerifiedResolvedBodyShapeInventoryV1,
};

/// Preserve an admitted field effect in the shared expression inventory. Only
/// the original writer owner can lend its plan; missing source grants no scalar
/// capability. Source-only probes keep their existing neutral-effect contract.
pub(super) fn observe<E>(
    input: ResolvedFunctionLoweringInputV1<'_>,
    statement: &crate::mir::compiler::located::LocatedStmtV1<'_>,
    locals: &mut PrefixLocalFlow<'_>,
    homes: &mut Vec<LiveHomeV1>,
    prefix_known: bool,
    local_calls: &mut Vec<LocalCallObservationV1>,
    path_calls: &mut BTreeSet<OwnedExprSiteV1>,
    actuals: &mut impl FnMut(
        &OwnedExprSiteV1, BorrowedCallActualRequestV1<'_>,
    ) -> Result<Option<BorrowedCallArgumentsV1>, E>,
    scalar_field: &mut impl HomeFieldSourceConsultV1<E>,
    view_use: &mut impl FnMut(&OwnedExprSiteV1, BorrowedViewUseRequestV1<'_>) -> Result<bool, E>,
    fields: &mut impl FnMut(
        &[LocalFieldReadRequestV1],
        LocalFieldReadDemandV1,
    ) -> Result<Option<Vec<LocalFieldReadResultV1>>, E>,
) -> Result<bool, E> {
    if let Some(admitted) = owned_local::observe(
        input,
        statement,
        locals,
        homes,
        prefix_known,
        local_calls,
        scalar_field,
        fields,
    )? {
        return Ok(admitted);
    }
    // A sealed entry writer uses the same whole-RHS owner as LOCAL. Its
    // original receiver/source permission is stronger than the neutral probe.
    if let Some(shape) = input.body_shape() {
        if let Some(assignment) = shape
            .assignment_sources()
            .iter()
            .find(|row| row.statement_site() == statement.site())
        {
            if let Some(ResolvedAssignmentTargetV1::FieldWrite { receiver }) =
                input.function().assignment_target(assignment.target_site())
            {
                if self_rooted_me(shape, receiver, locals).is_some() {
                    let entry_source = scalar_field.entry_write(input, assignment.target_site())?;
                    if crate::config::env::builder_debug_enabled() {
                        eprintln!("[field-write] entry source owner={:?} statement={:?} present={}", input.owner(), statement.site(), entry_source.is_some());
                    }
                    if let Some(entry) = entry_source
                    {
                        let text = entry.text_value().is_some();
                        let Some(frame) = CheckedFieldWriteFrameV1::for_entry(entry, homes) else {
                            return field_write_declined(input, statement, 0);
                        };
                        if text {
                            if !prefix_known || !local_call_flow::stage_expression_actuals_v1(
                                input, statement, frame.source(), locals, prefix_known,
                                homes, local_calls, path_calls, fields, actuals,
                            )? { return field_write_declined(input, statement, 1); }
                            let purpose = OwnedExprSiteV1::new(input.owner(), assignment.value_site().clone());
                            let Some(evaluation) = super::expression_plan::qualify_text_evaluation_v1(
                                input, Rc::clone(frame.source()), &purpose, locals,
                                homes, local_calls, actuals, None,
                            )? else { return field_write_declined(input, statement, 2); };
                            let Some(plan) = super::expression_plan::HomeExpressionPlanV1::text_field_write(
                                frame, Rc::clone(&evaluation),
                            ) else { return field_write_declined(input, statement, 3); };
                            let mut installation = locals.expression_installation();
                            if !installation.approve(&plan) { return field_write_declined(input, statement, 4); }
                            installation.commit();
                            path_calls.extend(evaluation.calls().iter().map(|call| call.site().clone()));
                            local_calls.extend_from_slice(evaluation.calls());
                            return Ok(true);
                        }
                        let signed_reads =
                            signed_reads(frame.source(), locals, local_calls, scalar_field);
                        let inputs: Vec<_> = if crate::config::env::builder_debug_enabled() {
                            frame.source().nodes().iter().filter(|node| matches!(node.source(),
                                crate::mir::resolved_value_profile::expression_plan::ExpressionNodeSourceV1::Reference(_)))
                                .map(|node| {
                                    let kind = match locals.observe(node.site()) {
                                        Some(OrdinaryObservation::TrivialLocal(_, Some(SourceScalarKind::Integer))) => "integer-local",
                                        Some(OrdinaryObservation::Integer(_)) => "integer-literal",
                                        Some(_) => "non-integer", None => "unavailable",
                                    };
                                    (node.site().clone(), kind, locals.current_scalar_read_v1(node.site(), local_calls).is_some(), signed_reads.contains_key(node.site()))
                                }).collect()
                        } else {
                            Vec::new()
                        };
                        let finite_reads =
                            finite_reads(input, frame.source(), locals, scalar_field)?;
                        let target = assignment.target_site();
                        let mut field_batch_ready = None;
                        let mut installation = locals.expression_installation();
                        let plan = super::expression_plan::qualify_field_write_expression_v1(
                            Rc::clone(frame.source()),
                            true,
                            Some(frame),
                            &mut |site| match locals.observe(site) {
                                Some(OrdinaryObservation::Integer(_)) => {
                                    Some(SourceScalarKind::Integer)
                                }
                                Some(OrdinaryObservation::Bool(_)) => Some(SourceScalarKind::Bool),
                                Some(OrdinaryObservation::TrivialLocal(_, kind)) => kind,
                                _ => None,
                            },
                            &mut |site| signed_reads.get(site).cloned(),
                            &mut |site| finite_reads.get(site).cloned(),
                            &mut |site| super::field_read::field_read_request(input, site, locals),
                            &mut |plan| installation.approve(plan),
                            &mut |requests, demand| {
                                let ready = fields(requests, demand)?.is_some();
                                field_batch_ready = Some(ready);
                                Ok(ready)
                            },
                        )?;
                        if plan.is_none() {
                            if crate::config::env::builder_debug_enabled() {
                                eprintln!("[field-source] owner={:?} target={target:?} RHS unavailable inputs={inputs:?} field-batch={field_batch_ready:?}", input.owner());
                            }
                            return field_write_declined(input, statement, 5);
                        }
                        installation.commit();
                        return Ok(true);
                    }
                }
            }
        }
    }
    if !observe_receiver_field_write(input, statement, locals, scalar_field, view_use)? {
        return field_write_declined(input, statement, 6);
    }
    let shape = input
        .body_shape()
        .expect("field effect validated its original shape");
    let assignment = shape
        .assignment_sources()
        .iter()
        .find(|row| row.statement_site() == statement.site())
        .expect("validated original assignment");
    let Some(source) = scalar_field.assignment_source(input.owner(), assignment.target_site())?
    else {
        return Ok(true);
    };
    if source.owner() != input.owner()
        || !matches!(source.root(), crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1::Assignment { assignment: original } if original == assignment)
    {
        return field_write_declined(input, statement, 7);
    }
    let Some(ResolvedAssignmentTargetV1::FieldWrite { receiver }) =
        input.function().assignment_target(assignment.target_site())
    else {
        return field_write_declined(input, statement, 8);
    };
    if self_rooted_me(shape, receiver, locals).is_none() {
        let Some(plan) =
            super::expression_plan::HomeExpressionPlanV1::borrowed_field_write(source, homes)
        else {
            return field_write_declined(input, statement, 9);
        };
        let mut installation = locals.expression_installation();
        if !installation.approve(&plan) {
            return field_write_declined(input, statement, 10);
        }
        installation.commit();
        return Ok(true);
    }
    // Preserve the existing neutral-effect admission for non-arithmetic writes.
    // Only scalar operations need the stronger shared field-value grounding.
    if !source.nodes().iter().any(|node| {
        node.binary().is_some_and(|binary| {
            matches!(
                binary.operator(),
                ResolvedBinaryOperatorV1::Subtract | ResolvedBinaryOperatorV1::Divide
            )
        })
    }) {
        return Ok(true);
    }
    let signed_reads = signed_reads(&source, locals, local_calls, scalar_field);
    let finite_reads = finite_reads(input, &source, locals, scalar_field)?;
    let mut installation = locals.expression_installation();
    let plan = super::expression_plan::qualify_field_write_expression_v1(
        source,
        false,
        None,
        &mut |site| match locals.observe(site) {
            Some(OrdinaryObservation::Integer(_)) => Some(SourceScalarKind::Integer),
            Some(OrdinaryObservation::Bool(_)) => Some(SourceScalarKind::Bool),
            Some(OrdinaryObservation::TrivialLocal(_, kind)) => kind,
            _ => None,
        },
        &mut |site| signed_reads.get(site).cloned(),
        &mut |site| finite_reads.get(site).cloned(),
        &mut |site| super::field_read::field_read_request(input, site, locals),
        &mut |plan| installation.approve(plan),
        &mut |requests, demand| Ok(fields(requests, demand)?.is_some()),
    )?;
    if plan.is_none() {
        return field_write_declined(input, statement, 11);
    }
    installation.commit();
    Ok(true)
}

/// `site` is a `me` expression bound to this frame's self-rooted borrowed
/// receiver — never a parameter handle and never static-box owner syntax.
pub(super) fn self_rooted_me<'a>(
    shape: &'a VerifiedResolvedBodyShapeInventoryV1,
    site: &SourceExprSiteV1,
    locals: &PrefixLocalFlow<'_>,
) -> Option<BindingRefV1> {
    let BodyExpressionShapeV1::Me {
        receiver: BodyMeReceiverV1::Lexical(binding),
        ..
    } = shape.expression_shape(site)?
    else {
        return None;
    };
    locals.is_self_rooted_handle(*binding).then_some(*binding)
}

/// `true` when `statement` is an admitted self-rooted field write. Every
/// sealed expression row under the value subtree must be Home-neutral:
/// trivial locals/literals, unary/binary structure, receiver-position `me`,
/// or a `me.<field>` read the issuer predicate proves scalar.
pub(super) fn observe_receiver_field_write<E>(
    input: ResolvedFunctionLoweringInputV1<'_>,
    statement: &crate::mir::compiler::located::LocatedStmtV1<'_>,
    locals: &PrefixLocalFlow<'_>,
    scalar_field: &mut impl HomeFieldSourceConsultV1<E>,
    view_use: &mut impl FnMut(&OwnedExprSiteV1, BorrowedViewUseRequestV1<'_>) -> Result<bool, E>,
) -> Result<bool, E> {
    if !matches!(statement.node(), ASTNode::Assignment { .. }) {
        return field_write_declined(input, statement, 12);
    }
    let Some(shape) = input.body_shape() else {
        return field_write_declined(input, statement, 13);
    };
    let Some(row) = shape
        .assignment_sources()
        .iter()
        .find(|row| row.statement_site() == statement.site())
    else {
        return field_write_declined(input, statement, 14);
    };
    // Compound `op=` reads and writes the field — this bounded admission
    // covers the plain store only.
    if row.form() != ResolvedAssignmentFormV1::Plain {
        return field_write_declined(input, statement, 15);
    }
    let Some(ResolvedAssignmentTargetV1::FieldWrite { receiver }) =
        input.function().assignment_target(row.target_site())
    else {
        return field_write_declined(input, statement, 16);
    };
    if self_rooted_me(shape, receiver, locals).is_none() {
        let Some(ResolvedLexicalRefV1::Local(binding)) = input.function().variable_ref(receiver)
        else {
            return field_write_declined(input, statement, 17);
        };
        // The canonical branch walk, not source order, proves the non-null
        // surviving path. No owned local or alias can become a borrowed formal.
        if !matches!(
            input.function().binding(binding).map(|row| row.kind()),
            Some(crate::mir::resolved_semantics::BindingKindV1::Parameter { .. })
        ) || locals.field_home(receiver) != Some(binding)
            || !locals.is_self_rooted_handle(binding)
            || !view_use(
                &OwnedExprSiteV1::new(input.owner(), receiver.clone()),
                BorrowedViewUseRequestV1::FormalFieldWriteReceiver {
                    target: row.target_site(),
                    binding,
                    root: binding,
                },
            )?
            || !view_use(
                &OwnedExprSiteV1::new(input.owner(), row.value_site().clone()),
                BorrowedViewUseRequestV1::FormalFieldWriteValue {
                    target: row.target_site(),
                },
            )?
        {
            return field_write_declined(input, statement, 18);
        }
        return Ok(true);
    }
    let prefix = row.value_site().node().segments();
    let subtree: Vec<&BodyExpressionShapeV1> = shape
        .expressions()
        .iter()
        .filter(|expression| {
            let segments = expression.site().node().segments();
            segments.len() >= prefix.len() && segments.starts_with(prefix)
        })
        .collect();
    // Receiver-position `me` nodes belong to their FieldAccess parent; a
    // `me` operand anywhere else is a self escape.
    let field_receivers: std::collections::BTreeSet<&SourceExprSiteV1> = subtree
        .iter()
        .filter_map(|expression| match expression {
            BodyExpressionShapeV1::FieldAccess { object, .. } => Some(object),
            _ => None,
        })
        .collect();
    for expression in subtree {
        match expression {
            BodyExpressionShapeV1::MapLiteral { .. }
            | BodyExpressionShapeV1::ArrayLiteral { .. }
            | BodyExpressionShapeV1::QualifiedReceiver { .. }
            | BodyExpressionShapeV1::MethodCall { .. }
            | BodyExpressionShapeV1::BlockExpr { .. } => return Ok(false),
            BodyExpressionShapeV1::Variable { site, .. } => {
                // A non-trivial leaf — a `Handle` read of an opaque formal —
                // is neutral only when the sealed draft admits a
                // dominated-view value use at this exact site; the issuer's
                // predicate is the consult, the draft stays the authority.
                if !locals.observe(site).is_some_and(|row| row.is_trivial())
                    && !view_use(
                        &OwnedExprSiteV1::new(input.owner(), site.clone()),
                        BorrowedViewUseRequestV1::Operand,
                    )?
                {
                    return field_write_declined(input, statement, 19);
                }
            }
            BodyExpressionShapeV1::Me { site, .. } => {
                if !field_receivers.contains(site) {
                    return field_write_declined(input, statement, 20);
                }
            }
            BodyExpressionShapeV1::FieldAccess {
                site,
                object,
                field,
            } => {
                let Some(home) = self_rooted_me(shape, object, locals) else {
                    return field_write_declined(input, statement, 21);
                };
                if !scalar_field.scalar_field(
                    &OwnedExprSiteV1::new(input.owner(), site.clone()),
                    object,
                    home,
                    home,
                    field,
                )? {
                    return field_write_declined(input, statement, 22);
                }
            }
            BodyExpressionShapeV1::Other { site, kind } => match &**kind {
                "UnaryOp" | "BinaryOp" => {}
                "Literal" => {
                    if !locals.observe(site).is_some_and(|row| row.is_trivial()) {
                        return field_write_declined(input, statement, 23);
                    }
                }
                _ => return Ok(false),
            },
        }
    }
    Ok(true)
}

/// Resolve current producers before the transactional expression installation
/// borrows the plan store. Only reads in this SAME original RHS are lent.
fn signed_reads<E>(
    source: &Rc<crate::mir::resolved_value_profile::expression_plan::ExpressionSourcePlanV1>,
    locals: &PrefixLocalFlow<'_>,
    calls: &[LocalCallObservationV1],
    consult: &mut impl HomeFieldSourceConsultV1<E>,
) -> std::collections::BTreeMap<
    SourceExprSiteV1,
    (
        CurrentScalarReadV1,
        Rc<crate::mir::normal_callable_semantic_package::SourceValueSumV1>,
    ),
> {
    source
        .nodes()
        .iter()
        .filter(|node| {
            matches!(node.source(),
        crate::mir::resolved_value_profile::expression_plan::ExpressionNodeSourceV1::Reference(_))
        })
        .filter_map(|node| {
            let read = locals.current_scalar_read_v1(node.site(), calls)?;
            let sum = consult.scalar_read_sum(&read)?;
            Some((node.site().clone(), (read, sum)))
        })
        .collect()
}

/// Read loans are gathered before borrowing the transactional plan store. The
/// existing formal/alias owner supplies roots; receiver identities are skipped.
fn finite_reads<E>(
    input: ResolvedFunctionLoweringInputV1<'_>,
    source: &Rc<crate::mir::resolved_value_profile::expression_plan::ExpressionSourcePlanV1>,
    locals: &PrefixLocalFlow<'_>,
    consult: &mut impl HomeFieldSourceConsultV1<E>,
) -> Result<
    std::collections::BTreeMap<
        SourceExprSiteV1,
        crate::mir::normal_callable_semantic_package::FiniteEntryReadV1,
    >,
    E,
> {
    let mut reads = std::collections::BTreeMap::new();
    for node in source.nodes() {
        if !matches!(
            node.source(),
            crate::mir::resolved_value_profile::expression_plan::ExpressionNodeSourceV1::Reference(
                _
            )
        ) {
            continue;
        }
        let Some((binding, root)) =
            super::argument_expression::entry_formal_reference(input, locals, node.site())
        else {
            note_finite_read(input, node.site(), "not an entry formal reference", false);
            continue;
        };
        let read = consult.finite_entry_read(input, source, node.site(), binding, root)?;
        note_finite_read(input, node.site(), "entry formal reference", read.is_some());
        if let Some(read) = read {
            reads.insert(node.site().clone(), read);
        }
    }
    Ok(reads)
}

/// Default-off (builder debug flag): whether a field-write RHS reference got a finite entry read.
#[inline(never)]
fn note_finite_read(input: ResolvedFunctionLoweringInputV1<'_>, site: &SourceExprSiteV1, why: &str, lent: bool) {
    if crate::config::env::builder_debug_enabled() {
        eprintln!("[finite-reads] owner={:?} site={:?}: {why}, lent={lent}", input.owner(), site);
    }
}

/// Default-off (builder debug flag): which numbered check declined a field-write statement.
#[inline(never)]
fn field_write_declined<E>(input: ResolvedFunctionLoweringInputV1<'_>, statement: &crate::mir::compiler::located::LocatedStmtV1<'_>, check: u32) -> Result<bool, E> {
    if crate::config::env::builder_debug_enabled() {
        eprintln!("[field-write] declined owner={:?} statement={:?} check={check}", input.owner(), statement.site());
    }
    Ok(false)
}
