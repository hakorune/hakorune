//! Ordered passive actual candidates from the same prefix-local state.
//! Source membership and typed-object domain are verified by the package.
use super::*;
use crate::mir::resolved_semantics::home_new_prefix::LiveHomeV1;
use crate::mir::resolved_semantics::BindingKindV1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum BorrowedCallActualValueV1 {
    Integer(i64),
    Bool(bool),
    /// Original literal only; no Text entry, residence or execution grant.
    String(Box<str>),
    Null,
    Scalar(BindingRefV1, SourceScalarKind),
    ReceivedNullable(BindingRefV1),
    ReceivedMixed(BindingRefV1),
    Home {
        binding: BindingRefV1,
        root: BindingRefV1,
        acquisition: OwnedExprSiteV1,
    },
    SelfRooted {
        binding: BindingRefV1,
        root: BindingRefV1,
    },
    Binding(BindingRefV1),
    /// Exact original MethodCall membership; no result class or execution proof.
    MethodCall(std::sync::Arc<crate::mir::resolved_semantics::VerifiedResolvedMethodCallSourceV1>),
    /// Original field spelling, without a live parent or execution proof.
    FieldAccess,
    /// The prefix still owns this exact definite received-call binding.
    ReceivedField {
        parent: BindingRefV1,
        acquisition: OwnedExprSiteV1,
    },
    /// Original expression membership, without value class or execution rights.
    Expression,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct BorrowedCallActualCandidateV1 {
    pub(crate) ordinal: u32,
    pub(crate) site: SourceExprSiteV1,
    pub(crate) value: BorrowedCallActualValueV1,
}

/// Original source spelling only. A Binding has no flow, Home or entry grant.
/// The same observer below supplies those proofs from the prefix-local state.
pub(crate) fn borrowed_actual_source_atom_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
    site: &SourceExprSiteV1,
) -> Option<BorrowedCallActualValueV1> {
    let source = input.function().expression_source();
    match source.literal(site) {
        Some(ResolvedLiteralSourceV1::Integer(value)) => {
            Some(BorrowedCallActualValueV1::Integer(*value))
        }
        Some(ResolvedLiteralSourceV1::Bool(value)) => Some(BorrowedCallActualValueV1::Bool(*value)),
        Some(ResolvedLiteralSourceV1::Null) => Some(BorrowedCallActualValueV1::Null),
        Some(ResolvedLiteralSourceV1::String(value)) => Some(BorrowedCallActualValueV1::String(value.clone())),
        _ => source
            .negative_integer_immediate(site)
            .map(BorrowedCallActualValueV1::Integer)
            .or_else(|| match input.function().variable_ref(site) {
                Some(ResolvedLexicalRefV1::Local(binding)) => {
                    Some(BorrowedCallActualValueV1::Binding(binding))
                }
                _ => input.function().method_call_source_v1(site).map(|source| BorrowedCallActualValueV1::MethodCall(std::sync::Arc::clone(source)))
                    .or_else(|| matches!(input.body_shape().and_then(|shape| shape.expression_shape(site)),
                        Some(crate::mir::resolved_semantics::BodyExpressionShapeV1::FieldAccess { .. }))
                        .then_some(BorrowedCallActualValueV1::FieldAccess))
                    .or_else(|| source.binary(site).is_some().then_some(BorrowedCallActualValueV1::Expression)),
            }),
    }
}

fn observe_one_borrowed_call_actuals(
    input: ResolvedFunctionLoweringInputV1<'_>,
    site: &OwnedExprSiteV1,
    locals: &PrefixLocalFlow<'_>,
    prefix_known: bool,
    prior_homes: &[LiveHomeV1],
) -> Option<Vec<BorrowedCallActualCandidateV1>> {
    if site.owner() != input.owner() {
        return None;
    }
    let (_, call) = input
        .function()
        .method_calls()
        .find(|(observed, _)| *observed == site.site())?;
    Some(
        call.arguments()
            .iter()
            .map(|argument| {
                let site = argument.site();
                let value = match borrowed_actual_source_atom_v1(input, site) {
                    Some(BorrowedCallActualValueV1::FieldAccess) => {
                        let received = prefix_known.then(|| {
                            let crate::mir::resolved_semantics::BodyExpressionShapeV1::FieldAccess { object, .. } =
                                input.body_shape()?.expression_shape(site)? else { return None; };
                            let ResolvedLexicalRefV1::Local(parent) = input.function().variable_ref(object)?
                                else { return None; };
                            if !prior_homes.iter().any(|home| home.binding() == parent) { return None; }
                            let (acquisition, nullable) = locals.received_call_acquisition(parent)?;
                            (!nullable).then(|| BorrowedCallActualValueV1::ReceivedField {
                                parent, acquisition: acquisition.clone(),
                            })
                        }).flatten();
                        received.unwrap_or(BorrowedCallActualValueV1::FieldAccess)
                    }
                    Some(BorrowedCallActualValueV1::Binding(_)) => {
                        if !prefix_known {
                            // An uncovered path proves no flow state, but
                            // sealed Parameter/`me` bindings are installed
                            // at entry — their self-rooted handle and
                            // trivial scalar observations are entry-stable
                            // facts, never branch-dependent evidence.
                            // Everything else stays `Unknown`.
                            match input.function().variable_ref(site) {
                                Some(ResolvedLexicalRefV1::Local(binding))
                                    if input.function().binding(binding).is_some_and(
                                        |record| {
                                            matches!(
                                                record.kind(),
                                                BindingKindV1::Parameter { .. }
                                                    | BindingKindV1::Receiver
                                            )
                                        },
                                    ) =>
                                {
                                    match locals.observe(site) {
                                        Some(OrdinaryObservation::Handle(root))
                                            if locals.is_self_rooted_handle(root) =>
                                        {
                                            BorrowedCallActualValueV1::SelfRooted { binding, root }
                                        }
                                        Some(OrdinaryObservation::TrivialLocal(
                                            actual,
                                            Some(kind),
                                        )) if actual == binding => {
                                            BorrowedCallActualValueV1::Scalar(binding, kind)
                                        }
                                        _ => BorrowedCallActualValueV1::Unknown,
                                    }
                                }
                                _ => BorrowedCallActualValueV1::Unknown,
                            }
                        } else {
                            match input.function().variable_ref(site) {
                                Some(ResolvedLexicalRefV1::Local(binding)) => match locals
                                    .observe(site)
                                {
                                    Some(OrdinaryObservation::TrivialLocal(actual, Some(kind)))
                                        if actual == binding =>
                                    {
                                        BorrowedCallActualValueV1::Scalar(binding, kind)
                                    }
                                    Some(OrdinaryObservation::Handle(root)) => {
                                        if let Some(acquisition) = locals.home_acquisition(root) {
                                            BorrowedCallActualValueV1::Home {
                                                binding,
                                                root,
                                                acquisition: acquisition.clone(),
                                            }
                                        } else if locals.is_self_rooted_handle(root) {
                                            BorrowedCallActualValueV1::SelfRooted { binding, root }
                                        } else {
                                            BorrowedCallActualValueV1::Binding(binding)
                                        }
                                    }
                                    // A received nullable observes as a
                                    // bound value; the stored class alone
                                    // proves it — the non-null mark stays a
                                    // field-read concern since the actual
                                    // carries both runtime states.
                                    Some(OrdinaryObservation::BoundValue(actual))
                                        if actual == binding
                                            && locals.is_received_nullable(binding) =>
                                    {
                                        BorrowedCallActualValueV1::ReceivedNullable(binding)
                                    }
                                    Some(OrdinaryObservation::BoundValue(actual))
                                        if actual == binding && locals.is_received_mixed_v1(binding) =>
                                    {
                                        BorrowedCallActualValueV1::ReceivedMixed(binding)
                                    }
                                    _ => BorrowedCallActualValueV1::Binding(binding),
                                },
                                _ => BorrowedCallActualValueV1::Unknown,
                            }
                        }
                    }
                    Some(value) => value,
                    None => BorrowedCallActualValueV1::Unknown,
                };
                BorrowedCallActualCandidateV1 {
                    ordinal: argument.ordinal(),
                    site: site.clone(),
                    value,
                }
            })
            .collect(),
    )
}

/// Observe only calls at exact direct argument sites, in source evaluation
/// order. The callback stages each original inner call before its enclosing
/// call; this does not turn a CallResult into an opaque payload candidate.
pub(in crate::mir::resolved_semantics::home_new_prefix) fn observe_borrowed_call_actuals(
    input: ResolvedFunctionLoweringInputV1<'_>,
    site: &OwnedExprSiteV1,
    locals: &PrefixLocalFlow<'_>,
    prefix_known: bool,
    prior_homes: &[LiveHomeV1],
) -> Vec<(OwnedExprSiteV1, Vec<BorrowedCallActualCandidateV1>)> {
    if site.owner() != input.owner() {
        return Vec::new();
    }
    let mut pending = vec![(site.clone(), false)];
    let mut observed = Vec::new();
    while let Some((site, children_observed)) = pending.pop() {
        if children_observed {
            if let Some(actuals) =
                observe_one_borrowed_call_actuals(input, &site, locals, prefix_known, prior_homes)
            {
                observed.push((site, actuals));
            }
            continue;
        }
        let Some((_, call)) = input
            .function()
            .method_calls()
            .find(|(candidate, _)| *candidate == site.site())
        else {
            continue;
        };
        pending.push((site, true));
        // Reverse stack insertion preserves the declared argument order.
        // A binary/condition subtree is not a direct method-call site and
        // cannot acquire coverage merely by containing a call somewhere.
        for argument in call.arguments().iter().rev() {
            pending.push((
                OwnedExprSiteV1::new(input.owner(), argument.site().clone()),
                false,
            ));
        }
    }
    observed
}

/// Only trees containing the original borrowed projection select the new Return seam.
pub(super) fn contains_borrowed_actual_v1(arguments: &[super::LocalCallArgumentV1]) -> bool {
    arguments.iter().any(|argument| match argument {
        super::LocalCallArgumentV1::BorrowedActual { .. } => true,
        super::LocalCallArgumentV1::CallResult(inner) => {
            contains_borrowed_actual_v1(inner.arguments())
        }
        _ => false,
    })
}

/// Stage calls from a statement's original source expression plan. Direct call
/// statements retain their exact root; value statements use the same
/// postorder source traversal as other whole-expression consumers.
pub(in crate::mir::resolved_semantics::home_new_prefix) fn stage_statement_actuals<E>(
    input: ResolvedFunctionLoweringInputV1<'_>,
    statement: &crate::mir::compiler::located::LocatedStmtV1<'_>,
    locals: &PrefixLocalFlow<'_>,
    prefix_known: bool,
    prior_homes: &[LiveHomeV1],
    local_calls: &[LocalCallObservationV1],
    fields: &mut impl FnMut(
        &[crate::mir::resolved_semantics::home_new_prefix::LocalFieldReadRequestV1],
        crate::mir::resolved_semantics::home_new_prefix::LocalFieldReadDemandV1,
    ) -> Result<
        Option<Vec<crate::mir::resolved_semantics::home_new_prefix::LocalFieldReadResultV1>>,
        E,
    >,
    port: &mut impl FnMut(
        &OwnedExprSiteV1,
        BorrowedCallActualRequestV1<'_>,
    ) -> Result<Option<BorrowedCallArgumentsV1>, E>,
) -> Result<(), E> {
    match statement.node() {
        crate::ast::ASTNode::Return { value: Some(_), .. } => {
            let Some(shape) = input.body_shape() else {
                return Ok(());
            };
            let Ok(ledger) = input.forest().callable_source_ledger(input.owner()) else {
                return Ok(());
            };
            let Ok(Some(source)) = crate::mir::resolved_value_profile::expression_plan::statement_expression_source_plan_v1(
                &ledger, shape, statement.site()) else { return Ok(()); };
            let _ = stage_expression_actuals_v1(
                input,
                statement,
                &source,
                locals,
                prefix_known,
                prior_homes,
                local_calls,
                fields,
                port,
            )?;
        }
        crate::ast::ASTNode::MethodCall { .. } => {
            let root = OwnedExprSiteV1::new(
                input.owner(),
                SourceExprSiteV1::from_node(statement.site().node().clone()),
            );
            stage_argument_expressions(
                input,
                statement,
                &root,
                locals,
                prefix_known,
                prior_homes,
                local_calls,
                fields,
                port,
            )?;
        }
        _ => {}
    }
    Ok(())
}

/// Stage original actuals once, then qualify only from this real covered point.
/// Nested/unobserved source staging continues to use Observe without this grant.
pub(in crate::mir::resolved_semantics::home_new_prefix) fn stage_argument_expressions<E>(
    input: ResolvedFunctionLoweringInputV1<'_>,
    statement: &crate::mir::compiler::located::LocatedStmtV1<'_>,
    root: &OwnedExprSiteV1,
    locals: &PrefixLocalFlow<'_>,
    prefix_known: bool,
    homes: &[LiveHomeV1],
    local_calls: &[LocalCallObservationV1],
    fields: &mut impl FnMut(
        &[crate::mir::resolved_semantics::home_new_prefix::LocalFieldReadRequestV1],
        crate::mir::resolved_semantics::home_new_prefix::LocalFieldReadDemandV1,
    ) -> Result<
        Option<Vec<crate::mir::resolved_semantics::home_new_prefix::LocalFieldReadResultV1>>,
        E,
    >,
    port: &mut impl FnMut(
        &OwnedExprSiteV1,
        BorrowedCallActualRequestV1<'_>,
    ) -> Result<Option<BorrowedCallArgumentsV1>, E>,
) -> Result<(), E> {
    stage_argument_roots_v1(
        input,
        statement,
        std::iter::once(root.clone()),
        locals,
        prefix_known,
        homes,
        local_calls,
        fields,
        port,
    )
}

/// Qualify actuals at the same real expression-entry Prefix. Conditional
/// subtrees retain their control owner; this grants no execution or temporary Home.
pub(in crate::mir::resolved_semantics::home_new_prefix) fn stage_expression_actuals_v1<E>(
    input: ResolvedFunctionLoweringInputV1<'_>,
    statement: &crate::mir::compiler::located::LocatedStmtV1<'_>,
    source: &crate::mir::resolved_value_profile::expression_plan::ExpressionSourcePlanV1,
    locals: &PrefixLocalFlow<'_>,
    prefix_known: bool,
    homes: &[LiveHomeV1],
    local_calls: &[LocalCallObservationV1],
    fields: &mut impl FnMut(
        &[crate::mir::resolved_semantics::home_new_prefix::LocalFieldReadRequestV1],
        crate::mir::resolved_semantics::home_new_prefix::LocalFieldReadDemandV1,
    ) -> Result<
        Option<Vec<crate::mir::resolved_semantics::home_new_prefix::LocalFieldReadResultV1>>,
        E,
    >,
    port: &mut impl FnMut(
        &OwnedExprSiteV1,
        BorrowedCallActualRequestV1<'_>,
    ) -> Result<Option<BorrowedCallArgumentsV1>, E>,
) -> Result<bool, E> {
    use crate::mir::resolved_value_profile::{
        expression_children::{ExpressionChildReachV1, ExpressionChildrenV1},
        expression_plan::ExpressionNodeSourceV1,
    };
    if source.owner() != input.owner()
        || source.root().statement() != statement.site()
        || source.nodes().iter().any(|node| {
            !matches!(node.children(), ExpressionChildrenV1::Known(children)
            if children.iter().all(|child| child.reach == ExpressionChildReachV1::Always))
        })
    {
        return Ok(false);
    }
    let Ok(order) = source.ordered_postorder_v1() else {
        return Ok(false);
    };
    let roots = order.into_iter().filter_map(|node| {
        let node = &source.nodes()[node];
        matches!(node.source(), ExpressionNodeSourceV1::MethodCall)
            .then(|| OwnedExprSiteV1::new(input.owner(), node.site().clone()))
    });
    stage_argument_roots_v1(
        input,
        statement,
        roots,
        locals,
        prefix_known,
        homes,
        local_calls,
        fields,
        port,
    )?;
    Ok(true)
}

fn stage_argument_roots_v1<E>(
    input: ResolvedFunctionLoweringInputV1<'_>,
    statement: &crate::mir::compiler::located::LocatedStmtV1<'_>,
    roots: impl IntoIterator<Item = OwnedExprSiteV1>,
    locals: &PrefixLocalFlow<'_>,
    prefix_known: bool,
    homes: &[LiveHomeV1],
    local_calls: &[LocalCallObservationV1],
    fields: &mut impl FnMut(
        &[crate::mir::resolved_semantics::home_new_prefix::LocalFieldReadRequestV1],
        crate::mir::resolved_semantics::home_new_prefix::LocalFieldReadDemandV1,
    ) -> Result<
        Option<Vec<crate::mir::resolved_semantics::home_new_prefix::LocalFieldReadResultV1>>,
        E,
    >,
    port: &mut impl FnMut(
        &OwnedExprSiteV1,
        BorrowedCallActualRequestV1<'_>,
    ) -> Result<Option<BorrowedCallArgumentsV1>, E>,
) -> Result<(), E> {
    // Direct-argument recursion and whole-expression traversal can encounter
    // the same source Call. Qualify it once within this operation only.
    let mut seen = std::collections::BTreeSet::new();
    for root in roots {
        if crate::config::env::builder_debug_enabled() {
            eprintln!(
                "[stage-expression-root] statement={:?} root={root:?} prefix_known={prefix_known}",
                statement.site()
            );
        }
        for (call_site, actuals) in
            observe_borrowed_call_actuals(input, &root, locals, prefix_known, homes)
        {
            if !seen.insert(call_site.clone()) {
                continue;
            }
            if crate::config::env::builder_debug_enabled() {
                eprintln!(
                    "[stage-expression-call] statement={:?} call={call_site:?} actuals={actuals:?}",
                    statement.site()
                );
            }
            port(
                &call_site,
                BorrowedCallActualRequestV1::Observe(input, &actuals),
            )?;
            super::super::argument_expression::observe(
                input,
                statement,
                &call_site,
                &actuals,
                locals,
                prefix_known,
                homes,
                local_calls,
                fields,
                port,
            )?;
        }
    }
    Ok(())
}
