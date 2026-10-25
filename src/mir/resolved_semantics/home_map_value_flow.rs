//! Recursive Map/Array value classification from the original Home walk.
use super::*;

/// Classify each `Element(ordinal)` child of a sealed `[...]` literal:
/// nested array literals recurse, nested map literals issue child flow rows,
/// and leaves classify through `map_value_leaf`. Unsupported expressions and
/// live Home elements stay `MapCandidateNotCovered`; this slice never transfers
/// a Home from an array element.
pub(super) fn observe_array_elements<E>(
    input: ResolvedFunctionLoweringInputV1<'_>,
    shape: &crate::mir::resolved_semantics::VerifiedResolvedBodyShapeInventoryV1,
    locals: &mut PrefixLocalFlow<'_>,
    array_site: &SourceExprSiteV1,
    element_count: u32,
    used: &mut std::collections::BTreeSet<BindingRefV1>,
    nested_out: &mut Vec<MapHomeObservation>,
    compatible: &mut impl FnMut(&OwnedExprSiteV1, BindingRefV1) -> Result<bool, E>,
) -> Result<Result<Vec<ArrayElementSource>, HomePrefixUnavailableV1>, E> {
    let mut elements = Vec::with_capacity(element_count as usize);
    for element_ordinal in 0..element_count {
        let mut relations = shape.relations().iter().filter(|row| {
            row.parent() == array_site.node()
                && row.role() == &SourcePathSegmentV1::Element(element_ordinal)
        });
        let Some(relation) = relations.next() else {
            return Ok(Err(HomePrefixUnavailableV1::SourceMismatch));
        };
        if relations.next().is_some() {
            return Ok(Err(HomePrefixUnavailableV1::SourceMismatch));
        }
        let element = relation.child();
        let kind = if let Some(nested_count) = array_literal_element_count(input, element) {
            ArrayElementKindV1::NestedArray(
                match observe_array_elements(
                    input,
                    shape,
                    locals,
                    element,
                    nested_count,
                    used,
                    nested_out,
                    compatible,
                )? {
                    Ok(elements) => elements,
                    Err(issue) => return Ok(Err(issue)),
                }
                .into_boxed_slice(),
            )
        } else if let Some(child_keys) = map_literal_keys(input, element) {
            // Array elements may own a nested Map. Keep the child as its own
            // source flow row and retain only its exact identity here; the
            // child itself is not allowed to consume an outer Home through
            // this first read slice, so it is observed with an empty Home
            // prefix and fails closed if it needs one.
            let child_owned = OwnedExprSiteV1::new(input.owner(), element.clone());
            match observe_map(
                input,
                &child_owned,
                MapDestinationV1::ContainedIn {
                    parent: OwnedExprSiteV1::new(input.owner(), array_site.clone()),
                    role: SourcePathSegmentV1::Element(element_ordinal),
                },
                child_keys,
                locals,
                &[],
                used,
                nested_out,
                compatible,
            )? {
                Ok((child_flow, _)) => {
                    nested_out.push(MapHomeObservation::Complete(child_flow));
                    ArrayElementKindV1::NestedMap(child_owned)
                }
                Err(issue) => {
                    nested_out.push(MapHomeObservation::Unavailable { site: child_owned });
                    return Ok(Err(issue));
                }
            }
        } else {
            let Some(value) = map_value_leaf(input, locals, element) else {
                return Ok(Err(HomePrefixUnavailableV1::MapCandidateNotCovered(
                    element.clone(),
                )));
            };
            ArrayElementKindV1::Leaf(value)
        };
        elements.push(ArrayElementSource {
            site: element.clone(),
            kind,
        });
    }
    Ok(Ok(elements))
}

/// Leaf entry/element source classification shared by map entry values and
/// array elements. `None` keeps the caller's fail-closed boundary.
pub(super) fn map_value_leaf(
    input: ResolvedFunctionLoweringInputV1<'_>,
    locals: &PrefixLocalFlow<'_>,
    site: &SourceExprSiteV1,
) -> Option<MapValueSource> {
    match locals.observe(site) {
        Some(OrdinaryObservation::Integer(value)) => Some(MapValueSource::Integer(value)),
        Some(OrdinaryObservation::Bool(value)) => Some(MapValueSource::Bool(value)),
        Some(OrdinaryObservation::TrivialLocal(binding, kind)) => {
            Some(MapValueSource::Local { binding, kind })
        }
        Some(OrdinaryObservation::Handle(root)) if locals.is_self_rooted_handle(root) => {
            Some(MapValueSource::BorrowedHandle(root))
        }
        Some(OrdinaryObservation::Handle(root))
            if locals.is_map_local(root) || locals.is_borrowed_map(root) =>
        {
            Some(MapValueSource::MapLocal(root))
        }
        _ => match input.function().expression_source().literal(site) {
            Some(ResolvedLiteralSourceV1::String(text)) => {
                Some(MapValueSource::String(text.clone()))
            }
            _ => None,
        },
    }
}

/// Shared Normal-only Home removal for literal entries and procedural installs.
/// Callers retain the precommit snapshot; this issues no physical commit.
pub(super) fn transfer_home_v1(
    locals: &mut PrefixLocalFlow<'_>,
    remaining: &mut Vec<LiveHomeV1>,
    binding: BindingRefV1,
) -> Result<(), HomePrefixUnavailableV1> {
    if remaining
        .iter()
        .filter(|home| home.binding() == binding)
        .count()
        != 1
    {
        return Err(HomePrefixUnavailableV1::SourceMismatch);
    }
    remaining.retain(|home| home.binding() != binding);
    locals.consume_home(binding);
    Ok(())
}

/// Original field set and its Normal successor; candidate remains caller-owned
/// before commit, including key/receiver checks. Slot support is not a new Home.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct MapFieldInstallV1 {
    statement: SourceStmtSiteV1,
    site: OwnedExprSiteV1,
    source: std::rc::Rc<crate::mir::normal_callable_semantic_package::MapStoredResultSourceV1>,
    receiver_read: SourceExprSiteV1,
    key: std::rc::Rc<CheckedExpressionEvaluationV1>,
    binding: BindingRefV1,
    acquisition: OwnedExprSiteV1,
    prior: Box<[LiveHomeV1]>,
    normal: Box<[LiveHomeV1]>,
}
impl MapFieldInstallV1 {
    pub(crate) fn statement(&self) -> &SourceStmtSiteV1 { &self.statement }
    pub(crate) fn site(&self) -> &OwnedExprSiteV1 {
        &self.site
    }
    pub(crate) fn source(
        &self,
    ) -> &std::rc::Rc<crate::mir::normal_callable_semantic_package::MapStoredResultSourceV1> {
        &self.source
    }
    pub(crate) fn receiver_read(&self) -> &SourceExprSiteV1 {
        &self.receiver_read
    }
    pub(crate) fn key(&self) -> &std::rc::Rc<CheckedExpressionEvaluationV1> {
        &self.key
    }
    pub(crate) fn binding(&self) -> BindingRefV1 {
        self.binding
    }
    pub(crate) fn acquisition(&self) -> &OwnedExprSiteV1 {
        &self.acquisition
    }
    pub(crate) fn prior_homes(&self) -> &[LiveHomeV1] {
        &self.prior
    }
    pub(crate) fn normal_homes(&self) -> &[LiveHomeV1] {
        &self.normal
    }
}

/// Use the original result-source census, never re-recognize a procedural set.
pub(in crate::mir::resolved_semantics::home_new_prefix) fn observe_field_install_v1<E>(
    input: ResolvedFunctionLoweringInputV1<'_>,
    statement: &crate::mir::compiler::located::LocatedStmtV1<'_>,
    locals: &mut PrefixLocalFlow<'_>,
    homes: &mut Vec<LiveHomeV1>,
    prefix_known: bool,
    calls: &[LocalCallObservationV1],
    maps: &mut Vec<MapHomeObservation>,
    port: &mut impl FnMut(
        &OwnedExprSiteV1,
        BorrowedCallActualRequestV1<'_>,
    ) -> Result<Option<BorrowedCallArgumentsV1>, E>,
) -> Result<bool, E> {
    if !prefix_known || !matches!(statement.node(), ASTNode::MethodCall { .. }) {
        return Ok(false);
    }
    let site = OwnedExprSiteV1::new(
        input.owner(),
        SourceExprSiteV1::from_node(statement.site().node().clone()),
    );
    let Some(BorrowedCallArgumentsV1::StoredMapSource(source)) =
        port(&site, BorrowedCallActualRequestV1::StoredMapSource)?
    else {
        return Ok(false);
    };
    let Some((binding, construction, set)) = source.installed_local_v1(&site) else {
        return Ok(false);
    };
    let Some((actual_binding, acquired)) = locals.direct_available_home(set.arguments()[1].site())
    else {
        return Ok(false);
    };
    if binding != actual_binding
        || acquired.owner() != input.owner()
        || acquired.site() != construction.site()
        || !locals.is_self_rooted_handle(source.receiver_v1().receiver())
        || maps.iter().any(|row| row.site() == &site)
    {
        return Ok(false);
    }
    let acquisition = acquired.clone();
    let Some(shape) = input.body_shape() else {
        return Ok(false);
    };
    let Some(crate::mir::resolved_semantics::BodyExpressionShapeV1::FieldAccess {
        object: read,
        ..
    }) = shape.expression_shape(set.receiver_site())
    else {
        return Ok(false);
    };
    if input.function().variable_ref(read)
        != Some(ResolvedLexicalRefV1::Local(source.receiver_v1().receiver()))
    {
        return Ok(false);
    }
    let read = read.clone();
    let Ok(plan) = crate::mir::resolved_value_profile::expression_plan::call_argument_expression_source_plan_v1(input, set, 0) else { return Ok(false) };
    if plan.nodes().len() != 1
        || !matches!(
            plan.nodes()[0].source(),
            crate::mir::resolved_value_profile::expression_plan::ExpressionNodeSourceV1::Reference(
                _
            )
        )
    {
        return Ok(false);
    }
    let Some(key) = super::super::expression_plan::qualify_text_evaluation_v1(
        input, plan, &site, locals, homes, calls, port, None,
    )?
    else {
        return Ok(false);
    };
    if !key.calls().is_empty()
        || !key.cores().is_empty()
        || !key.operations().is_empty()
        || !key.owned_result_nodes().is_empty()
    {
        return Ok(false);
    }
    let prior = homes.clone().into_boxed_slice();
    let mut normal = homes.clone();
    if transfer_home_v1(locals, &mut normal, binding).is_err() {
        return Ok(false);
    }
    let row = std::rc::Rc::new(MapFieldInstallV1 {
        statement: statement.site().clone(),
        site,
        source,
        receiver_read: read,
        key,
        binding,
        acquisition,
        prior,
        normal: normal.clone().into_boxed_slice(),
    });
    *homes = normal;
    maps.push(MapHomeObservation::FieldInstall(row));
    Ok(true)
}

/// Exact original get initializer, its current holder/key and unchanged Homes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct MapFieldGetLocalV1 {
    site: OwnedExprSiteV1,
    declaration: SourceBindingSiteV1,
    binding: BindingRefV1,
    source: crate::mir::normal_callable_semantic_package::StoredMapGetSourceLoanV1,
    key: std::rc::Rc<CheckedExpressionEvaluationV1>,
    prior: Box<[LiveHomeV1]>,
}
impl MapFieldGetLocalV1 {
    pub(crate) fn site(&self) -> &OwnedExprSiteV1 {
        &self.site
    }
    pub(crate) fn declaration(&self) -> &SourceBindingSiteV1 {
        &self.declaration
    }
    pub(crate) fn binding(&self) -> BindingRefV1 {
        self.binding
    }
    pub(crate) fn source(
        &self,
    ) -> &crate::mir::normal_callable_semantic_package::StoredMapGetSourceLoanV1 {
        &self.source
    }
    pub(crate) fn key(&self) -> &std::rc::Rc<CheckedExpressionEvaluationV1> {
        &self.key
    }
    pub(crate) fn prior_homes(&self) -> &[LiveHomeV1] {
        &self.prior
    }
    pub(crate) fn statement(&self) -> &SourceStmtSiteV1 {
        let SourceBindingSiteV1::Local { statement, .. } = &self.declaration else {
            unreachable!("original Local")
        };
        statement
    }
    /// A passive caller cannot replace, escape or mutate the supporting slot.
    /// This is source/lifetime evidence only, not execution or null exclusion.
    pub(crate) fn matches_input(&self, input: ResolvedFunctionLoweringInputV1<'_>) -> bool {
        let Some(shape) = input.body_shape() else {
            return false;
        };
        let Ok(uses) = input.forest().callable_source_ledger(input.owner()) else {
            return false;
        };
        let get = self.source.get();
        let mut initializers = input
            .function()
            .expression_source()
            .initializers()
            .filter(|row| row.binding() == self.binding);
        let exact = initializers.next().is_some_and(|row| {
            row.initializer_site() == Some(self.site.site())
                && row.declaration_site() == self.declaration()
        }) && initializers.next().is_none();
        input.owner() == self.site.owner() && self.binding.owner() == input.owner() && exact
            && input.function().method_call_source_v1(self.site.site()).is_some_and(|original| std::sync::Arc::ptr_eq(original, get))
            && uses.uncaptured_binding(self.binding) && uses.capture_demands().is_empty()
            && !uses.direct_call_observations().any(|_| true)
            && uses.method_calls().all(|(site, _)| site == get.site())
            && input.function().expression_source().constructions().next().is_none()
            && self.key.prior_homes() == self.prior_homes()
            && shape.effects().iter().all(|effect| match effect.kind {
                crate::mir::resolved_semantics::BodyEffectKindV1::Call => &effect.site == get.site(),
                crate::mir::resolved_semantics::BodyEffectKindV1::NonLocalControl => shape.statements().iter().any(|row|
                    matches!(row, crate::mir::resolved_semantics::BodyStatementShapeV1::Return { site, .. }
                        if site.node() == effect.site.node())),
                _ => false,
            })
    }
}

pub(in crate::mir::resolved_semantics::home_new_prefix) fn observe_field_get_local_v1<E>(
    input: ResolvedFunctionLoweringInputV1<'_>,
    at: &OwnedExprSiteV1,
    declaration: &SourceBindingSiteV1,
    binding: BindingRefV1,
    locals: &mut PrefixLocalFlow<'_>,
    homes: &[LiveHomeV1],
    prefix_known: bool,
    calls: &[LocalCallObservationV1],
    maps: &mut Vec<MapHomeObservation>,
    port: &mut impl FnMut(
        &OwnedExprSiteV1,
        BorrowedCallActualRequestV1<'_>,
    ) -> Result<Option<BorrowedCallArgumentsV1>, E>,
) -> Result<Option<Result<(), HomePrefixUnavailableV1>>, E> {
    let Some(original) = input.function().method_call_source_v1(at.site()) else {
        return Ok(None);
    };
    if original.selector() != "get" || original.arity() != 1 {
        return Ok(None);
    }
    let Some(BorrowedCallArgumentsV1::StoredMapGetSource(source)) =
        port(at, BorrowedCallActualRequestV1::StoredMapGetSource)?
    else {
        return Ok(None);
    };
    let unavailable = || {
        Some(Err(HomePrefixUnavailableV1::PrefixNotCovered(
            match declaration {
                SourceBindingSiteV1::Local { statement, .. } => statement.clone(),
                _ => unreachable!("Local declaration"),
            },
        )))
    };
    if !prefix_known
        || !locals.is_self_rooted_handle(source.receiver().receiver())
        || maps.iter().any(|row| row.site() == at)
    {
        return Ok(unavailable());
    }
    let Ok(plan) = crate::mir::resolved_value_profile::expression_plan::call_argument_expression_source_plan_v1(input, source.get(), 0) else { return Ok(unavailable()) };
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
    let Some(key) = super::super::expression_plan::qualify_text_evaluation_v1(
        input, plan, at, locals, homes, calls, port, None,
    )?
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
    let frame = std::rc::Rc::new(MapFieldGetLocalV1 {
        site: at.clone(),
        declaration: declaration.clone(),
        binding,
        source,
        key,
        prior: homes.into(),
    });
    if !frame.matches_input(input) {
        return Ok(unavailable());
    }
    locals.install_stored_get_v1(std::rc::Rc::clone(&frame));
    maps.push(MapHomeObservation::FieldGet(frame));
    Ok(Some(Ok(())))
}
