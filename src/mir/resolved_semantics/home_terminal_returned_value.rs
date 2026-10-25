//! Exact returned-value source identity for the existing terminal Home walk.
//! Lifecycle acquisition and physical result admission remain separate owners.
use super::super::expression_plan::{prepare_value_evaluation_v1, qualify_text_evaluation_v1};
use super::{
    ASTNode, BindingRefV1, FunctionOwnerIdV1, OrdinaryObservation, OwnedExprSiteV1,
    PrefixLocalFlow, ResolvedFunctionLoweringInputV1, ResolvedLiteralSourceV1, SourceExprSiteV1,
    SourceStmtSiteV1,
};

/// Exact source relation for a Completion-backed `return <expr>` whose
/// scalar classifier proves an i64 value that is neither a literal nor a
/// field read: a local bound to a proven-i64 source (literal store, exact
/// formal, field-read result, or classified call result) or a trivial
/// integer expression. Ordinary scalar proof stays in the source inventory and
/// current flow; a direct finite entry read retains its original source loan.
/// Neither form grants a physical result ABI.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TerminalI64ScalarReturnV1 {
    owner: FunctionOwnerIdV1,
    return_site: SourceStmtSiteV1,
    value_site: SourceExprSiteV1,
    entry_read: Option<crate::mir::normal_callable_semantic_package::FiniteEntryReadV1>,
    evaluation: Option<
        std::rc::Rc<crate::mir::resolved_semantics::home_new_prefix::CheckedExpressionEvaluationV1>,
    >,
}

impl TerminalI64ScalarReturnV1 {
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn issue(
        owner: FunctionOwnerIdV1,
        return_site: SourceStmtSiteV1,
        value_site: SourceExprSiteV1,
    ) -> Self {
        Self {
            owner,
            return_site,
            value_site,
            entry_read: None,
            evaluation: None,
        }
    }
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn with_entry_read_v1(
        mut self,
        read: crate::mir::normal_callable_semantic_package::FiniteEntryReadV1,
    ) -> Option<Self> {
        let required = read.requirement();
        if self.evaluation.is_some()
            || !read.signed_only()
            || required.node() != 0
            || required.read() != &self.value_site
            || required.source_plan().owner() != self.owner
            || required.source_plan().root().statement() != &self.return_site
        {
            return None;
        }
        self.entry_read = Some(read);
        Some(self)
    }
    pub(crate) fn evaluation(
        &self,
    ) -> Option<
        &std::rc::Rc<
            crate::mir::resolved_semantics::home_new_prefix::CheckedExpressionEvaluationV1,
        >,
    > {
        self.evaluation.as_ref()
    }
    fn with_evaluation_v1(
        mut self,
        evaluation: std::rc::Rc<
            crate::mir::resolved_semantics::home_new_prefix::CheckedExpressionEvaluationV1,
        >,
    ) -> Option<Self> {
        use crate::mir::resolved_value_profile::{
            expression_class::ExpressionValueClassV1, expression_plan::ExpressionPlanRootV1,
            expression_projection::ExpressionLeafClassV1,
            expression_root::StatementExpressionRoleV1,
        };
        if self.entry_read.is_some()
            || evaluation.source().owner() != self.owner
            || !matches!(evaluation.source().root(), ExpressionPlanRootV1::Statement {
                statement, role: StatementExpressionRoleV1::ReturnValue, value,
            } if statement == &self.return_site && value == &self.value_site)
            || evaluation.root_class() != ExpressionLeafClassV1::Value(ExpressionValueClassV1::I64)
            || !evaluation.reference_coverage_is_exact()
        {
            return None;
        }
        self.evaluation = Some(evaluation);
        Some(self)
    }
    pub(crate) fn entry_read(
        &self,
    ) -> Option<&crate::mir::normal_callable_semantic_package::FiniteEntryReadV1> {
        self.entry_read.as_ref()
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
}

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
        source: crate::mir::normal_callable_semantic_package::StoredMapGetSourceLoanV1,
        key: std::rc::Rc<
            crate::mir::resolved_semantics::home_new_prefix::CheckedExpressionEvaluationV1,
        >,
    },
    /// Same payload after a checked field install; no caller Home remains.
    StoredMapInstalled(
        std::rc::Rc<crate::mir::resolved_semantics::home_new_prefix::MapFieldInstallV1>,
    ),
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
    TextExpression(
        std::rc::Rc<crate::mir::resolved_semantics::home_new_prefix::CheckedExpressionEvaluationV1>,
    ),
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
    /// `return <dynamic local>` — preserve its original runtime tag without
    /// granting a scalar result class, conversion, or ownership transfer.
    TaggedDynamic(crate::mir::resolved_semantics::home_new_prefix::TaggedDynamicReadV1),
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
        if root == acquisition.binding()
            && acquisition.owns_root()
            && acquisition.is_text_residence()
        {
            return Some(TerminalReturnedSourceV1::TextResidence {
                binding,
                acquisition,
            });
        }
        return None;
    }
    let observation = locals.observe(site)?;
    match observation {
        OrdinaryObservation::TaggedDynamicLocal(binding) => {
            let read = locals.tagged_dynamic_read_v1(site)?;
            (read.binding() == binding).then_some(TerminalReturnedSourceV1::TaggedDynamic(read))
        }
        OrdinaryObservation::Handle(root) if locals.is_map_local(root) => {
            Some(TerminalReturnedSourceV1::MapLocal(root))
        }
        OrdinaryObservation::Handle(root) if locals.is_self_rooted_handle(root) => {
            Some(TerminalReturnedSourceV1::Handle(root))
        }
        OrdinaryObservation::Handle(root) => locals
            .home_acquisition(root)
            .map(|acquisition| TerminalReturnedSourceV1::Home {
                binding: root,
                acquisition: acquisition.clone(),
            }),
        _ => None,
    }
}

/// Borrow the original entry value at an exact Return; no scalar ABI grant.
pub(in crate::mir::resolved_semantics::home_new_prefix) fn finite_entry_return<E>(
    input: ResolvedFunctionLoweringInputV1<'_>,
    return_site: &SourceStmtSiteV1,
    value_site: &SourceExprSiteV1,
    locals: &PrefixLocalFlow<'_>,
    port: &mut impl FnMut(
        &OwnedExprSiteV1,
        crate::mir::resolved_semantics::home_new_prefix::BorrowedCallActualRequestV1<'_>,
    ) -> Result<
        Option<crate::mir::resolved_semantics::home_new_prefix::BorrowedCallArgumentsV1>,
        E,
    >,
) -> Result<Option<super::TerminalI64ScalarReturnV1>, E> {
    use crate::mir::resolved_semantics::home_new_prefix::{
        BorrowedCallActualRequestV1 as Request, BorrowedCallArgumentsV1 as Response,
    };
    let Some((binding, root)) =
        super::super::argument_expression::text_reference(input, locals, value_site)
    else {
        return Ok(None);
    };
    let Ok(ledger) = input.forest().callable_source_ledger(input.owner()) else {
        return Ok(None);
    };
    let Some(shape) = input.body_shape() else {
        return Ok(None);
    };
    let Ok(Some(plan)) =
        crate::mir::resolved_value_profile::expression_plan::statement_expression_source_plan_v1(
            &ledger,
            shape,
            return_site,
        )
    else {
        return Ok(None);
    };
    let owned = OwnedExprSiteV1::new(input.owner(), value_site.clone());
    let Some(Response::FiniteEntryRead(read)) = port(
        &owned,
        Request::FiniteEntryRead {
            input,
            plan: &plan,
            read: value_site,
            binding,
            root,
        },
    )?
    else {
        return Ok(None);
    };
    Ok(super::TerminalI64ScalarReturnV1::issue(
        input.owner(),
        return_site.clone(),
        value_site.clone(),
    )
    .with_entry_read_v1(read))
}

/// Seal one fresh Text result at its original ReturnValue destination. The
/// existing residence-return path retains transfers of installed Text Homes.
pub(in crate::mir::resolved_semantics::home_new_prefix) fn observe_text_return_expression_v1<E>(
    input: ResolvedFunctionLoweringInputV1<'_>,
    statement: &crate::mir::compiler::located::LocatedStmtV1<'_>,
    locals: &PrefixLocalFlow<'_>,
    homes: &[crate::mir::resolved_semantics::home_new_prefix::LiveHomeV1],
    calls: &[crate::mir::resolved_semantics::home_new_prefix::LocalCallObservationV1],
    actuals: &mut impl FnMut(
        &OwnedExprSiteV1,
        crate::mir::resolved_semantics::home_new_prefix::BorrowedCallActualRequestV1<'_>,
    ) -> Result<
        Option<crate::mir::resolved_semantics::home_new_prefix::BorrowedCallArgumentsV1>,
        E,
    >,
    fields: &mut impl FnMut(
        &[super::super::LocalFieldReadRequestV1],
        super::super::LocalFieldReadDemandV1,
    ) -> Result<Option<Vec<super::super::LocalFieldReadResultV1>>, E>,
) -> Result<Option<TerminalValueReturnV1>, E> {
    use crate::mir::resolved_value_profile::{
        expression_children::{ExpressionChildReachV1, ExpressionChildrenV1},
        expression_plan::ExpressionPlanRootV1,
        expression_root::StatementExpressionRoleV1,
    };
    let Some(shape) = input.body_shape() else {
        return Ok(None);
    };
    let Ok(ledger) = input.forest().callable_source_ledger(input.owner()) else {
        return Ok(None);
    };
    let Ok(Some(source)) =
        crate::mir::resolved_value_profile::expression_plan::statement_expression_source_plan_v1(
            &ledger,
            shape,
            statement.site(),
        )
    else {
        return Ok(None);
    };
    let ExpressionPlanRootV1::Statement {
        role: StatementExpressionRoleV1::ReturnValue,
        value,
        ..
    } = source.root()
    else {
        return Ok(None);
    };
    if locals.text_reference(value).is_some()
        || source
            .nodes()
            .first()
            .is_none_or(|node| node.site() != value)
        || source.nodes().iter().any(|node| {
            !matches!(node.children(), ExpressionChildrenV1::Known(children)
            if children.iter().all(|child| child.reach == ExpressionChildReachV1::Always))
        })
    {
        return Ok(None);
    }
    // Scalar/field terminals keep their own observer and pending input phase.
    use crate::mir::resolved_value_profile::expression_plan::ExpressionNodeSourceV1 as Node;
    let mut root = 0;
    while let Some(tail) = source.transparent_tail_node_v1(root) {
        root = tail;
    }
    match source.nodes()[root].source() {
        Node::Shape(crate::mir::resolved_semantics::BodyExpressionShapeV1::FieldAccess { .. })
        | Node::MethodCall
        | Node::Literal(ResolvedLiteralSourceV1::String(_))
        | Node::Reference(_) => {}
        Node::Binary(binary)
            if binary.operator()
                == crate::mir::resolved_semantics::ResolvedBinaryOperatorV1::Add => {}
        _ => return Ok(None),
    }
    let purpose_site = OwnedExprSiteV1::new(input.owner(), value.clone());
    let mut field_seeds = std::collections::BTreeMap::new();
    if matches!(source.nodes()[root].source(), Node::Shape(crate::mir::resolved_semantics::BodyExpressionShapeV1::FieldAccess { .. })) {
        if let Some(request) = super::super::field_read::expression_field_read_request(input, &source, source.nodes()[root].site(), locals) {
            // The retained Text-return source covers this declaration's own
            // receiver. A foreign formal keeps its existing field/result lane;
            // probing it here must not demand scalar ingress ahead of that lane.
            if input.function().binding(request.receiver).is_none_or(|binding|
                binding.kind() != crate::mir::resolved_semantics::BindingKindV1::Receiver) {
                return Ok(None);
            }
            if let Some(classes) = fields(std::slice::from_ref(&request), super::super::LocalFieldReadDemandV1::SourceExpression {
                probe: true, plan: std::rc::Rc::clone(&source),
            })? {
                if matches!(classes.as_slice(), [super::super::LocalFieldReadResultV1::Text(_)]) {
                    field_seeds.insert(root, crate::mir::resolved_value_profile::expression_projection::ExpressionLeafClassV1::Value(
                        crate::mir::resolved_value_profile::expression_class::ExpressionValueClassV1::String));
                }
            }
        }
    }
    let Some((evaluation, requests)) = prepare_value_evaluation_v1(
        input,
        std::rc::Rc::clone(&source),
        crate::mir::resolved_value_profile::expression_class::ExpressionValueClassV1::String,
        &purpose_site,
        locals,
        homes,
        calls,
        actuals,
        None,
        &field_seeds,
        &mut |site| {
            let request = super::super::field_read::expression_field_read_request(
                input, &source, site, locals,
            )?;
            (matches!(request.received, Some(super::super::ReceivedFieldReadRequestV1::StoredMapGet { .. }))
                || request.received.is_none() && request.path.is_none())
                .then_some(request)
        },
    )?
    else {
        return Ok(None);
    };
    if !requests.is_empty() {
        let Some(candidate) =
            super::super::HomeExpressionPlanV1::terminal_fields_v1(&evaluation, &requests)
        else {
            return Ok(None);
        };
        let mut installation = locals.expression_installation();
        if !installation.approve(&candidate) {
            return Ok(None);
        }
        if fields(
            &requests,
            super::super::LocalFieldReadDemandV1::SourceExpression {
                probe: false,
                plan: std::rc::Rc::clone(&source),
            },
        )?
        .is_none()
        {
            return Ok(None);
        }
        installation.commit();
    }
    Ok(Some(TerminalValueReturnV1::issue(
        input.owner(),
        statement.site().clone(),
        value.clone(),
        TerminalReturnedSourceV1::TextExpression(evaluation),
    )))
}

/// Selected storage Return never falls through to Fresh/object or scalar rules.
/// Miss-path set/transfer remains unavailable until its Normal delta is qualified.
pub(in crate::mir::resolved_semantics::home_new_prefix) fn observe_stored_return_v1<E>(
    input: ResolvedFunctionLoweringInputV1<'_>,
    statement: &crate::mir::compiler::located::LocatedStmtV1<'_>,
    locals: &PrefixLocalFlow<'_>,
    homes: &[crate::mir::resolved_semantics::home_new_prefix::LiveHomeV1],
    calls: &[crate::mir::resolved_semantics::home_new_prefix::LocalCallObservationV1],
    maps: &[crate::mir::resolved_semantics::home_new_prefix::MapHomeObservation],
    actuals: &mut impl FnMut(
        &OwnedExprSiteV1,
        crate::mir::resolved_semantics::home_new_prefix::BorrowedCallActualRequestV1<'_>,
    ) -> Result<
        Option<crate::mir::resolved_semantics::home_new_prefix::BorrowedCallArgumentsV1>,
        E,
    >,
) -> Result<Option<Result<TerminalValueReturnV1, super::HomePrefixUnavailableV1>>, E> {
    use crate::mir::resolved_semantics::home_new_prefix::{
        BorrowedCallActualRequestV1 as Request, BorrowedCallArgumentsV1 as Response,
    };
    if !matches!(statement.node(), ASTNode::Return { value: Some(_), .. }) {
        return Ok(None);
    }
    let Ok(value) = input
        .source()
        .child_expr_from_stmt(statement, super::ExprChildRoleV1::ReturnValue)
    else {
        return Ok(None);
    };
    let site = OwnedExprSiteV1::new(input.owner(), value.site().clone());
    let Some(Response::StoredMapSource(source)) = actuals(&site, Request::StoredMapSource)? else {
        return Ok(None);
    };
    let unavailable = || {
        Some(Err(super::HomePrefixUnavailableV1::ReturnValueNotCovered(
            statement.site().clone(),
        )))
    };
    if let Some((binding, construction, set)) = source.installed_local_v1(&site) {
        let mut installs =
            maps.iter().filter_map(|row| {
                match row {
            crate::mir::resolved_semantics::home_new_prefix::MapHomeObservation::FieldInstall(row)
                if std::rc::Rc::ptr_eq(row.source(), &source) && row.binding() == binding
                    && row.site().owner() == set.owner() && row.site().site() == set.site()
                    && row.acquisition().site() == construction.site() => Some(row),
            _ => None,
        }
            });
        let Some(install) = installs.next() else {
            return Ok(unavailable());
        };
        if installs.next().is_some()
            || homes.iter().any(|home| home.binding() == binding)
            || input.function().variable_ref(value.site())
                != Some(crate::mir::resolved_semantics::ResolvedLexicalRefV1::Local(
                    binding,
                ))
        {
            return Ok(unavailable());
        }
        return Ok(Some(Ok(TerminalValueReturnV1::issue(
            input.owner(),
            statement.site().clone(),
            value.site().clone(),
            TerminalReturnedSourceV1::StoredMapInstalled(std::rc::Rc::clone(install)),
        ))));
    }
    let Some(Response::StoredMapGetSource(get_source)) =
        actuals(&site, Request::StoredMapGetSource)?
    else {
        return Ok(unavailable());
    };
    if get_source.result_source_v1().is_none_or(|result| !std::rc::Rc::ptr_eq(&source, result)) {
        return Ok(unavailable());
    }
    let get = get_source.get();
    let receiver_read = get_source.receiver().receiver_site().clone();
    let Ok(plan) = crate::mir::resolved_value_profile::expression_plan::call_argument_expression_source_plan_v1(input, get, 0)
        else { return Ok(unavailable()) };
    // Source currently admits an exact symbolic parameter key only. Computed
    // effects must not run before the receiver's checked field borrow.
    if plan.nodes().len() != 1
        || !matches!(
            plan.nodes()[0].source(),
            crate::mir::resolved_value_profile::expression_plan::ExpressionNodeSourceV1::Reference(
                _
            )
        )
    {
        return Ok(unavailable());
    }
    let Some(key) =
        qualify_text_evaluation_v1(input, plan, &site, locals, homes, calls, actuals, None)?
    else {
        return Ok(unavailable());
    };
    if !key.calls().is_empty()
        || !key.cores().is_empty()
        || !key.operations().is_empty()
        || !key.owned_result_nodes().is_empty()
    {
        return Ok(unavailable());
    }
    Ok(Some(Ok(TerminalValueReturnV1::issue(
        input.owner(),
        statement.site().clone(),
        value.site().clone(),
        TerminalReturnedSourceV1::StoredMapGet {
            source: get_source,
            key,
            receiver_read,
        },
    ))))
}

/// A selected scalar operation borrows the existing ReturnValue evaluator.
/// Selection is original source morphology; unavailable never retries a terminal.
pub(in crate::mir::resolved_semantics::home_new_prefix) fn observe_scalar_return_expression_v1<
    E,
>(
    input: ResolvedFunctionLoweringInputV1<'_>,
    statement: &crate::mir::compiler::located::LocatedStmtV1<'_>,
    locals: &PrefixLocalFlow<'_>,
    homes: &[crate::mir::resolved_semantics::home_new_prefix::LiveHomeV1],
    calls: &[crate::mir::resolved_semantics::home_new_prefix::LocalCallObservationV1],
    actuals: &mut impl FnMut(
        &OwnedExprSiteV1,
        crate::mir::resolved_semantics::home_new_prefix::BorrowedCallActualRequestV1<'_>,
    ) -> Result<
        Option<crate::mir::resolved_semantics::home_new_prefix::BorrowedCallArgumentsV1>,
        E,
    >,
    fields: &mut impl FnMut(
        &[super::super::LocalFieldReadRequestV1],
        super::super::LocalFieldReadDemandV1,
    ) -> Result<Option<Vec<super::super::LocalFieldReadResultV1>>, E>,
) -> Result<Option<Result<TerminalI64ScalarReturnV1, super::HomePrefixUnavailableV1>>, E> {
    let Ok(value) = input
        .source()
        .child_expr_from_stmt(statement, super::ExprChildRoleV1::ReturnValue)
    else {
        return Ok(None);
    };
    if !input
        .function()
        .expression_source()
        .binary(value.site())
        .is_some_and(|binary| {
            binary.operator() == crate::mir::resolved_semantics::ResolvedBinaryOperatorV1::Subtract
        })
    {
        return Ok(None);
    }
    let refused = || {
        Some(Err(super::HomePrefixUnavailableV1::ReturnValueNotCovered(
            statement.site().clone(),
        )))
    };
    let Some(shape) = input.body_shape() else {
        return Ok(refused());
    };
    let Ok(ledger) = input.forest().callable_source_ledger(input.owner()) else {
        return Ok(refused());
    };
    let Ok(Some(source)) =
        crate::mir::resolved_value_profile::expression_plan::statement_expression_source_plan_v1(
            &ledger,
            shape,
            statement.site(),
        )
    else {
        return Ok(refused());
    };
    // The complete Return evaluation borrows its Call and field leaves from
    // their existing source issuers and retains their ordered cleanup proof.
    let purpose = OwnedExprSiteV1::new(input.owner(), value.site().clone());
    let Some((evaluation, requests)) = prepare_value_evaluation_v1(
        input,
        std::rc::Rc::clone(&source),
        crate::mir::resolved_value_profile::expression_class::ExpressionValueClassV1::I64,
        &purpose,
        locals,
        homes,
        calls,
        actuals,
        None,
        &std::collections::BTreeMap::new(),
        &mut |site| {
            super::super::field_read::expression_field_read_request(input, &source, site, locals)
        },
    )?
    else {
        return Ok(refused());
    };
    if !requests.is_empty() {
        let Some(candidate) =
            super::super::HomeExpressionPlanV1::terminal_fields_v1(&evaluation, &requests)
        else {
            return Ok(refused());
        };
        let mut installation = locals.expression_installation();
        if !installation.approve(&candidate) {
            return Ok(refused());
        }
        if fields(
            &requests,
            super::super::LocalFieldReadDemandV1::SourceExpression { probe: false, plan: source },
        )?
        .is_none()
        {
            return Ok(refused());
        }
        installation.commit();
    }
    Ok(Some(
        TerminalI64ScalarReturnV1::issue(
            input.owner(),
            statement.site().clone(),
            value.site().clone(),
        )
        .with_evaluation_v1(evaluation)
        .ok_or_else(|| {
            super::HomePrefixUnavailableV1::ReturnValueNotCovered(statement.site().clone())
        }),
    ))
}
