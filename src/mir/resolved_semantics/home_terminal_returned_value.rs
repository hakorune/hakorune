//! Exact returned-value source identity for the existing terminal Home walk.
//! Lifecycle acquisition and physical result admission remain separate owners.
use super::{
    ASTNode, BindingRefV1, FunctionOwnerIdV1, OrdinaryObservation, OwnedExprSiteV1,
    PrefixLocalFlow, ResolvedFunctionLoweringInputV1, ResolvedLiteralSourceV1, SourceExprSiteV1,
    SourceStmtSiteV1,
};

/// The exact source value a non-i64 terminal `return <expr>` hands to the
/// caller. Binding classes carry the root identity; ownership transfer is
/// decided by the lifecycle contract, never by this row.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum TerminalReturnedSourceV1 {
    /// `return %{...}` — construction lives in the literal's MapHomeFlow
    /// row; this site joins the relation and the flow.
    MapLiteral(OwnedExprSiteV1),
    /// Original qualified call return, pending Completion/leaf handoff proof.
    OwnedCall(Box<super::super::TerminalObjectReturnObligationV1>),
    /// `return <binding>` — a live map-installed local or alias root.
    MapLocal(BindingRefV1),
    /// `return <binding>` — a live selected-New Home local; `acquisition`
    /// is its exact `new` site.
    Home {
        binding: BindingRefV1,
        acquisition: OwnedExprSiteV1,
    },
    /// `return <binding>` — a borrowed self-rooted handle root (parameter
    /// or declared handle); the caller keeps ownership.
    Handle(BindingRefV1),
    /// Non-null source result of the original borrowed LOCAL call, owning nothing.
    BorrowedNullableLocal(
        Box<crate::mir::normal_callable_semantic_package::BorrowedNullableLocalSourceLoanV1>,
    ),
    /// Original guarded Map get plus its qualified passive Text-key read.
    StoredMapGet {
        receiver_read: SourceExprSiteV1,
        source: std::rc::Rc<crate::mir::normal_callable_semantic_package::MapStoredResultSourceV1>,
        key: std::rc::Rc<crate::mir::resolved_semantics::home_new_prefix::CheckedExpressionEvaluationV1>,
    },
    /// `return new <class>(...)` — the returned value is the fresh
    /// construction at this exact site; the fresh object's ownership
    /// transfers to the caller at the Return edge. This records only the
    /// returned-source identity — the lifecycle authority is the
    /// result-position claim row, and the physical Handle result ABI is a
    /// separate downstream family.
    Construction(OwnedExprSiteV1),
    /// `return "<literal>"` — a string literal value.
    StringLiteral,
    /// Same original ReturnValue evaluation; fresh Text is acquired only later.
    TextExpression(std::rc::Rc<crate::mir::resolved_semantics::home_new_prefix::CheckedExpressionEvaluationV1>),
    /// Exact read of the current owned Text acquisition. The original exit
    /// keeps this residence live for Fault cleanup; Normal transfer is a
    /// downstream disposition, never an Object Home or a new acquisition.
    TextResidence {
        binding: BindingRefV1,
        acquisition: crate::mir::resolved_semantics::home_new_prefix::TextResidenceAcquisitionV1,
    },
    /// `return null` — the null literal value.
    NullLiteral,
    /// `return <float>` — a float literal value.
    FloatLiteral,
}

/// Exact source relation for a non-i64 terminal `return <value>`. Records
/// which source value the site hands to the caller; no physical value, ABI,
/// recipe, JSON, or backend authority is issued.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TerminalValueReturnV1 {
    owner: FunctionOwnerIdV1,
    return_site: SourceStmtSiteV1,
    value_site: SourceExprSiteV1,
    returned: TerminalReturnedSourceV1,
}

impl TerminalValueReturnV1 {
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn issue(
        owner: FunctionOwnerIdV1,
        return_site: SourceStmtSiteV1,
        value_site: SourceExprSiteV1,
        returned: TerminalReturnedSourceV1,
    ) -> Self {
        Self {
            owner,
            return_site,
            value_site,
            returned,
        }
    }
    /// Deliberate identity corruption for independent admission tests only.
    #[cfg(test)]
    pub(crate) fn with_value_site_for_test(&self, site: SourceExprSiteV1) -> Self {
        let mut changed = self.clone();
        changed.value_site = site;
        changed
    }

    /// Deliberate exit-identity corruption for retention boundary tests only.
    #[cfg(test)]
    pub(crate) fn with_return_site_for_test(&self, site: SourceStmtSiteV1) -> Self {
        let mut changed = self.clone();
        changed.return_site = site;
        changed
    }

    /// Keep the original exit identity while corrupting only its owned-call snapshot.
    #[cfg(test)]
    pub(crate) fn with_object_arguments_for_test(
        &self,
        arguments: crate::mir::resolved_semantics::home_new_prefix::ObjectCallSourceSupportV1,
    ) -> Self {
        let mut changed = self.clone();
        let TerminalReturnedSourceV1::OwnedCall(original) = &self.returned else {
            panic!("owned-call test fixture required")
        };
        changed.returned = TerminalReturnedSourceV1::OwnedCall(Box::new(
            original.with_arguments_for_test(arguments),
        ));
        changed
    }

    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }
    pub(crate) fn return_site(&self) -> &SourceStmtSiteV1 {
        &self.return_site
    }
    pub(crate) fn value_site(&self) -> &SourceExprSiteV1 {
        &self.value_site
    }
    pub(crate) fn returned(&self) -> &TerminalReturnedSourceV1 {
        &self.returned
    }
}

/// Classify the exact source value a non-i64 `return <expr>` hands to the
/// caller. Literal classes come from the sealed expression-source inventory;
/// binding classes come from the running local flow. Trivial scalar locals
/// and i64-class forms are handled by `return_scalar` before this hook.
pub(in crate::mir::resolved_semantics::home_new_prefix) fn terminal_returned_source(
    input: ResolvedFunctionLoweringInputV1<'_>,
    site: &SourceExprSiteV1,
    locals: &PrefixLocalFlow<'_>,
) -> Option<TerminalReturnedSourceV1> {
    match input.function().expression_source().literal(site) {
        Some(ResolvedLiteralSourceV1::String(_)) => {
            return Some(TerminalReturnedSourceV1::StringLiteral);
        }
        Some(ResolvedLiteralSourceV1::Null) => {
            return Some(TerminalReturnedSourceV1::NullLiteral);
        }
        Some(ResolvedLiteralSourceV1::Float) => {
            return Some(TerminalReturnedSourceV1::FloatLiteral);
        }
        _ => {}
    }
    if matches!(
        input
            .source()
            .expr_at(&OwnedExprSiteV1::new(input.owner(), site.clone()))
            .map(|expr| expr.node()),
        Ok(ASTNode::New { .. })
    ) {
        return Some(TerminalReturnedSourceV1::Construction(
            OwnedExprSiteV1::new(input.owner(), site.clone()),
        ));
    }
    if let Some(loan) = locals.borrowed_nullable_return(site) {
        return Some(TerminalReturnedSourceV1::BorrowedNullableLocal(Box::new(
            loan.clone(),
        )));
    }
    if let Some((binding, root, acquisition)) = locals.text_reference(site) {
        if root == acquisition.binding() && acquisition.owns_root()
            && acquisition.is_text_residence()
        {
            return Some(TerminalReturnedSourceV1::TextResidence { binding, acquisition });
        }
        return None;
    }
    let OrdinaryObservation::Handle(root) = locals.observe(site)? else {
        return None;
    };
    if locals.is_map_local(root) {
        Some(TerminalReturnedSourceV1::MapLocal(root))
    } else if locals.is_self_rooted_handle(root) {
        Some(TerminalReturnedSourceV1::Handle(root))
    } else {
        locals
            .home_acquisition(root)
            .map(|acquisition| TerminalReturnedSourceV1::Home {
                binding: root,
                acquisition: acquisition.clone(),
            })
    }
}

/// Borrow the original entry value at an exact Return; no scalar ABI grant.
pub(in crate::mir::resolved_semantics::home_new_prefix) fn finite_entry_return<E>(
    input: ResolvedFunctionLoweringInputV1<'_>, return_site: &SourceStmtSiteV1,
    value_site: &SourceExprSiteV1, locals: &PrefixLocalFlow<'_>,
    port: &mut impl FnMut(&OwnedExprSiteV1,
        crate::mir::resolved_semantics::home_new_prefix::BorrowedCallActualRequestV1<'_>)
        -> Result<Option<crate::mir::resolved_semantics::home_new_prefix::BorrowedCallArgumentsV1>, E>,
) -> Result<Option<super::TerminalI64ScalarReturnV1>, E> {
    use crate::mir::resolved_semantics::home_new_prefix::{BorrowedCallActualRequestV1 as Request, BorrowedCallArgumentsV1 as Response};
    let Some((binding, root)) = super::super::argument_expression::text_reference(input, locals, value_site) else { return Ok(None) };
    let Ok(ledger) = input.forest().callable_source_ledger(input.owner()) else { return Ok(None) };
    let Some(shape) = input.body_shape() else { return Ok(None) };
    let Ok(Some(plan)) = crate::mir::resolved_value_profile::expression_plan::statement_expression_source_plan_v1(
        &ledger, shape, return_site) else { return Ok(None) };
    let owned = OwnedExprSiteV1::new(input.owner(), value_site.clone());
    let Some(Response::FiniteEntryRead(read)) = port(&owned, Request::FiniteEntryRead {
        input, plan: &plan, read: value_site, binding, root })? else { return Ok(None) };
    Ok(super::TerminalI64ScalarReturnV1::issue(input.owner(), return_site.clone(), value_site.clone())
        .with_entry_read_v1(read))
}

/// Seal one fresh Text result at its original ReturnValue destination. The
/// existing residence-return path retains transfers of installed Text Homes.
pub(in crate::mir::resolved_semantics::home_new_prefix) fn observe_text_return_expression_v1<E>(
    input: ResolvedFunctionLoweringInputV1<'_>,
    statement: &crate::mir::compiler::located::LocatedStmtV1<'_>,
    locals: &PrefixLocalFlow<'_>, homes: &[crate::mir::resolved_semantics::home_new_prefix::LiveHomeV1], calls: &[crate::mir::resolved_semantics::home_new_prefix::LocalCallObservationV1],
    actuals: &mut impl FnMut(&OwnedExprSiteV1, crate::mir::resolved_semantics::home_new_prefix::BorrowedCallActualRequestV1<'_>)
        -> Result<Option<crate::mir::resolved_semantics::home_new_prefix::BorrowedCallArgumentsV1>, E>,
) -> Result<Option<TerminalValueReturnV1>, E> {
    use crate::mir::resolved_value_profile::{expression_plan::ExpressionPlanRootV1,
        expression_root::StatementExpressionRoleV1, expression_children::{ExpressionChildrenV1, ExpressionChildReachV1}};
    let Some(shape) = input.body_shape() else { return Ok(None) };
    let Ok(ledger) = input.forest().callable_source_ledger(input.owner()) else { return Ok(None) };
    let Ok(Some(source)) = crate::mir::resolved_value_profile::expression_plan::statement_expression_source_plan_v1(
        &ledger, shape, statement.site()) else { return Ok(None) };
    let ExpressionPlanRootV1::Statement { role: StatementExpressionRoleV1::ReturnValue, value, .. } = source.root()
        else { return Ok(None) };
    if locals.text_reference(value).is_some() || source.nodes().first().is_none_or(|node| node.site() != value)
        || source.nodes().iter().any(|node| !matches!(node.children(), ExpressionChildrenV1::Known(children)
            if children.iter().all(|child| child.reach == ExpressionChildReachV1::Always))) { return Ok(None) }
    // Scalar/field terminals keep their own observer and pending input phase.
    // A fresh Core root borrows its original source once before any ordinary
    // Call demand; unsupported ordinary Text-call Returns remain unselected.
    use crate::mir::resolved_value_profile::expression_plan::ExpressionNodeSourceV1 as Node;
    let mut root = 0;
    while let Some(tail) = source.transparent_tail_node_v1(root) { root = tail; }
    match source.nodes()[root].source() {
        Node::MethodCall | Node::Literal(ResolvedLiteralSourceV1::String(_)) | Node::Reference(_) => {},
        Node::Binary(binary) if binary.operator() == crate::mir::resolved_semantics::ResolvedBinaryOperatorV1::Add => {},
        _ => return Ok(None),
    }
    let purpose_site = OwnedExprSiteV1::new(input.owner(), value.clone());
    let Some(evaluation) = qualify_terminal_text_evaluation_v1(input, std::rc::Rc::clone(&source), &purpose_site, locals, homes, calls, actuals)?
        else { return Ok(None) };
    Ok(Some(TerminalValueReturnV1::issue(input.owner(), statement.site().clone(), value.clone(),
        TerminalReturnedSourceV1::TextExpression(evaluation))))
}

/// Shared original Text evaluation for terminal values and passive Map keys.
/// The purpose chooses the source root; this function grants no result/Home ABI.
fn qualify_terminal_text_evaluation_v1<E>(
    input: ResolvedFunctionLoweringInputV1<'_>,
    source: std::rc::Rc<crate::mir::resolved_value_profile::expression_plan::ExpressionSourcePlanV1>,
    purpose_site: &OwnedExprSiteV1,
    locals: &PrefixLocalFlow<'_>, homes: &[crate::mir::resolved_semantics::home_new_prefix::LiveHomeV1],
    calls: &[crate::mir::resolved_semantics::home_new_prefix::LocalCallObservationV1],
    actuals: &mut impl FnMut(&OwnedExprSiteV1, crate::mir::resolved_semantics::home_new_prefix::BorrowedCallActualRequestV1<'_>)
        -> Result<Option<crate::mir::resolved_semantics::home_new_prefix::BorrowedCallArgumentsV1>, E>,
) -> Result<Option<std::rc::Rc<crate::mir::resolved_semantics::home_new_prefix::CheckedExpressionEvaluationV1>>, E> {
    use crate::mir::resolved_value_profile::{expression_class::ExpressionValueClassV1,
        expression_projection::ExpressionLeafClassV1};
    use crate::mir::resolved_value_profile::expression_plan::ExpressionNodeSourceV1 as Node;
    let mut root = 0;
    while let Some(tail) = source.transparent_tail_node_v1(root) { root = tail; }
    // Core source consultation is passive. Ordinary Call Returns keep their
    // existing terminal/input owner until their complete result handoff is sealed.
    let mut cores = std::collections::BTreeMap::new();
    for node in source.nodes() {
        if !matches!(node.source(), Node::MethodCall) { continue; }
        let site = OwnedExprSiteV1::new(input.owner(), node.site().clone());
        let Some(crate::mir::resolved_semantics::home_new_prefix::BorrowedCallArgumentsV1::ExpressionCoreSource(core)) = actuals(&site,
            crate::mir::resolved_semantics::home_new_prefix::BorrowedCallActualRequestV1::ExpressionCoreSource { input, plan: &source })?
            else { return Ok(None) };
        if node.site() == source.nodes()[root].site() && !core.owns_result() { return Ok(None); }
        cores.insert(site, core);
    }
    let Some(prepared) = crate::mir::resolved_semantics::home_new_prefix::expression_plan::prepare_expression_calls_v1(&source,
        crate::mir::resolved_semantics::home_new_prefix::expression_plan::ExpressionLeafProfileV1::Print, &std::collections::BTreeMap::new(), homes,
        &mut |site| Ok(cores.get(site).cloned().map(
            crate::mir::resolved_semantics::home_new_prefix::expression_plan::ExpressionCallSourceV1::Core)))? else { return Ok(None) };
    let mut initial = Vec::new();
    for (node, original) in source.nodes().iter().enumerate() {
        if !prepared.is_active_reference(node) { continue; }
        let Some((binding, root)) = crate::mir::resolved_semantics::home_new_prefix::argument_expression::text_reference(input, locals, original.site()) else { continue };
        let owned = purpose_site.clone();
        if let Some(crate::mir::resolved_semantics::home_new_prefix::BorrowedCallArgumentsV1::FiniteEntryRead(read)) = actuals(&owned,
            crate::mir::resolved_semantics::home_new_prefix::BorrowedCallActualRequestV1::FiniteEntryRead { input, plan: &source, read: original.site(), binding, root })? {
            initial.push((node, crate::mir::resolved_semantics::home_new_prefix::CheckedExpressionReferenceV1::FiniteEntry {
                read, exclusion: locals.entry_null_exclusion(original.site(), binding, root),
            }));
        }
    }
    let Some((evaluation, fields)) = crate::mir::resolved_semantics::home_new_prefix::expression_plan::qualify_statement_evaluation_v1(prepared, initial,
        &mut |site| locals.static_text_reference(site), &mut |site| locals.text_reference(site),
        &mut |site| locals.current_scalar_read_v1(site, calls), &mut |_| None)? else {
            return Ok(None);
        };
    if !fields.is_empty() || evaluation.root_class() != ExpressionLeafClassV1::Value(ExpressionValueClassV1::String) {
        return Ok(None);
    }
    Ok(Some(std::rc::Rc::new(evaluation)))
}

/// Selected storage Return never falls through to Fresh/object or scalar rules.
/// Miss-path set/transfer remains unavailable until its Normal delta is qualified.
pub(in crate::mir::resolved_semantics::home_new_prefix) fn observe_stored_return_v1<E>(
    input: ResolvedFunctionLoweringInputV1<'_>,
    statement: &crate::mir::compiler::located::LocatedStmtV1<'_>,
    locals: &PrefixLocalFlow<'_>, homes: &[crate::mir::resolved_semantics::home_new_prefix::LiveHomeV1],
    calls: &[crate::mir::resolved_semantics::home_new_prefix::LocalCallObservationV1],
    actuals: &mut impl FnMut(&OwnedExprSiteV1, crate::mir::resolved_semantics::home_new_prefix::BorrowedCallActualRequestV1<'_>)
        -> Result<Option<crate::mir::resolved_semantics::home_new_prefix::BorrowedCallArgumentsV1>, E>,
) -> Result<Option<Result<TerminalValueReturnV1, super::HomePrefixUnavailableV1>>, E> {
    use crate::mir::resolved_semantics::home_new_prefix::{BorrowedCallActualRequestV1 as Request, BorrowedCallArgumentsV1 as Response};
    if !matches!(statement.node(), ASTNode::Return { value: Some(_), .. }) { return Ok(None) }
    let Ok(value) = input.source().child_expr_from_stmt(statement, super::ExprChildRoleV1::ReturnValue)
        else { return Ok(None) };
    let site = OwnedExprSiteV1::new(input.owner(), value.site().clone());
    let Some(Response::StoredReturnSource(source)) = actuals(&site, Request::StoredReturnSource)? else { return Ok(None) };
    let unavailable = || Some(Err(super::HomePrefixUnavailableV1::ReturnValueNotCovered(statement.site().clone())));
    let Some(get) = source.present_get_v1(&site) else { return Ok(unavailable()) };
    use crate::mir::resolved_semantics::{BodyExpressionShapeV1 as Shape, BodyMeReceiverV1};
    let Some(shape) = input.body_shape() else { return Ok(unavailable()) };
    let Some(Shape::FieldAccess { object: receiver_read, field, .. }) = shape.expression_shape(get.receiver_site())
        else { return Ok(unavailable()) };
    let Some(Shape::FieldAccess { field: original, .. }) = shape.expression_shape(source.receiver_v1().field_site())
        else { return Ok(unavailable()) };
    if field != original || !matches!(shape.expression_shape(receiver_read),
        Some(Shape::Me { receiver: BodyMeReceiverV1::Lexical(binding), .. })
            if *binding == source.receiver_v1().receiver()) { return Ok(unavailable()) }
    let receiver_read = receiver_read.clone();
    let Ok(plan) = crate::mir::resolved_value_profile::expression_plan::call_argument_expression_source_plan_v1(input, get, 0)
        else { return Ok(unavailable()) };
    // Source currently admits an exact symbolic parameter key only. Computed
    // effects must not run before the receiver's checked field borrow.
    if plan.nodes().len() != 1 || !matches!(plan.nodes()[0].source(),
        crate::mir::resolved_value_profile::expression_plan::ExpressionNodeSourceV1::Reference(_)) {
        return Ok(unavailable());
    }
    let Some(key) = qualify_terminal_text_evaluation_v1(input, plan, &site, locals, homes, calls, actuals)?
        else { return Ok(unavailable()) };
    if !key.calls().is_empty() || !key.cores().is_empty() || !key.operations().is_empty()
        || !key.owned_result_nodes().is_empty() { return Ok(unavailable()) }
    Ok(Some(Ok(TerminalValueReturnV1::issue(input.owner(), statement.site().clone(), value.site().clone(),
        TerminalReturnedSourceV1::StoredMapGet { source, key, receiver_read }))))
}
