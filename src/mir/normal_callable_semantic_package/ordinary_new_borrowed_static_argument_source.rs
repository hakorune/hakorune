//! Source-purpose connection at the original actual observation. No Home rights.
use super::*;
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::resolved_value_profile::expression_plan::{
    call_argument_expression_source_plan_v1, ExpressionPlanRootV1, ExpressionSourcePlanV1,
};

/// Original outer Call and ordinal identity, independent of the value's class.
fn corroborates_argument_plan_v1(
    plan: &ExpressionSourcePlanV1,
    source: &StaticIncomingSourceV1,
    ordinal: u32,
    site: &SourceExprSiteV1,
) -> bool {
    matches!(plan.root(), ExpressionPlanRootV1::CallArgument { call, ordinal: original, .. }
        if *original == ordinal && std::sync::Arc::ptr_eq(call, source.method_source()))
        && plan.owner() == source.call_site().owner()
        && source.argument_sites().get(ordinal as usize) == Some(site)
        && plan.nodes().first().is_some_and(|node| node.site() == site)
}

pub(super) fn corroborates_argument_source_v1(
    plan: &ExpressionSourcePlanV1,
    source: &StaticIncomingSourceV1,
    ordinal: u32,
    method: &std::sync::Arc<crate::mir::resolved_semantics::VerifiedResolvedMethodCallSourceV1>,
) -> bool {
    corroborates_argument_plan_v1(plan, source, ordinal, method.site())
        && method.owner() == plan.owner()
        && plan.nodes().first().is_some_and(|node| {
            matches!(
            node.source(),
            crate::mir::resolved_value_profile::expression_plan::ExpressionNodeSourceV1::MethodCall
        )
        })
}

/// Re-observation borrows every original argument plan Rc. This is source
/// coverage only: literal, local and Core inputs share no execution shortcut.
pub(in crate::mir::normal_callable_semantic_package) fn bind_argument_sources_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
    prepared: &PreparedBorrowedFormalIngressV1,
    contracts: Option<&[OwnedCallableParameterContractDeclarationV1]>,
    site: &OwnedExprSiteV1,
    previous: Option<&Result<PreparedBorrowedCallActualsV1, String>>,
    rows: Option<&mut PreparedBorrowedCallActualsV1>,
) -> Result<(), String> {
    let Some(rows) = rows else { return Ok(()) };
    if let BorrowedCallActualEvidencePhaseV1::SourceObject(identity) = &mut rows.phase {
        let reject = || freeze("borrowed-object/source-actual-identity");
        let mut originals = prepared
            .source_incoming
            .exact_rows()
            .filter(|row| &row.call == site);
        let original = originals.next().ok_or_else(reject)?;
        if originals.next().is_some()
            || original.source.require_instance()? != &identity.source
            || original.arguments != identity.incoming_arguments
        {
            return Err(reject());
        }
        let old = previous
            .map(|row| {
                let row = row
                    .as_ref()
                    .map_err(|_| freeze("borrowed-actual/repeated-walk-drift"))?;
                let BorrowedCallActualEvidencePhaseV1::SourceObject(old) = &row.phase else {
                    return Err(reject());
                };
                if !old.same_original_source_v1(identity) {
                    return Err(reject());
                }
                Ok((!old.argument_sources.is_empty()).then_some(&old.argument_sources))
            })
            .transpose()?
            .flatten();
        let method = input
            .function()
            .method_call_source_v1(site.site())
            .ok_or_else(reject)?;
        identity.argument_sources = model::bind_original_argument_plans_v1(
            input,
            super::super::super::borrowed_formal_uses::BorrowedCallSourceLoanV1::Instance(
                &identity.source,
            ),
            method,
            &identity.candidates,
            old,
        )?;
        return Ok(());
    }
    if let BorrowedCallActualEvidencePhaseV1::SourceInstance(identity) = &mut rows.phase {
        let reject = || freeze("borrowed-instance/argument-source-identity");
        let contracts = contracts.ok_or_else(reject)?;
        let original = crate::mir::normal_callable_semantic_package::OrdinaryNewClaimLedgerV1::original_instance_input_call_v1(prepared, contracts, site)?.ok_or_else(reject)?;
        if original.source.require_instance()? != &identity.source
            || original.arguments != identity.incoming_arguments
        {
            return Err(reject());
        }
        let old = previous
            .map(|row| {
                let row = row
                    .as_ref()
                    .map_err(|_| freeze("borrowed-actual/repeated-walk-drift"))?;
                let BorrowedCallActualEvidencePhaseV1::SourceInstance(old) = &row.phase else {
                    return Err(reject());
                };
                if !old.same_original_source_v1(identity) {
                    return Err(reject());
                }
                Ok((!old.argument_sources.is_empty()).then_some(&old.argument_sources))
            })
            .transpose()?
            .flatten();
        let method = input
            .function()
            .method_call_source_v1(site.site())
            .ok_or_else(reject)?;
        identity.argument_sources = model::bind_original_argument_plans_v1(
            input,
            super::super::super::borrowed_formal_uses::BorrowedCallSourceLoanV1::Instance(
                &identity.source,
            ),
            method,
            &identity.candidates,
            old,
        )?;
        return Ok(());
    }
    let BorrowedCallActualEvidencePhaseV1::SourceStatic(identity) = &mut rows.phase else {
        return Ok(());
    };
    let reject = || freeze("borrowed-static/core-source-identity");
    if !original_static_source_v1(prepared, site)?
        .is_some_and(|source| Rc::ptr_eq(source, &identity.source))
    {
        return Err(reject());
    }
    let old = previous
        .map(|rows| {
            let rows = rows
                .as_ref()
                .map_err(|_| freeze("borrowed-actual/repeated-walk-drift"))?;
            let BorrowedCallActualEvidencePhaseV1::SourceStatic(old) = &rows.phase else {
                return Err(reject());
            };
            if !Rc::ptr_eq(&old.source, &identity.source)
                || old.candidates != identity.candidates
                || old.integer_evidence != identity.integer_evidence
                || old.core_arguments.len() != identity.core_arguments.len()
                || old.argument_sources.len() != identity.candidates.len()
            {
                return Err(reject());
            }
            Ok(old)
        })
        .transpose()?;
    if input.owner() != site.owner()
        || identity.source.call_site() != site
        || identity.candidates.len() != identity.source.argument_sites().len()
        || !input
            .function()
            .method_call_source_v1(site.site())
            .is_some_and(|call| std::sync::Arc::ptr_eq(call, identity.source.method_source()))
    {
        return Err(reject());
    }
    for (ordinal, candidate) in identity.candidates.iter().enumerate() {
        let ordinal = ordinal as u32;
        if candidate.ordinal != ordinal {
            return Err(reject());
        }
        if let BorrowedCallActualValueV1::SelfRooted { binding, root } = candidate.value {
            if !prepared
                .static_arguments
                .contains_key(&(site.clone(), ordinal))
            {
                let flow = input
                    .function()
                    .expression_source()
                    .value_flow()
                    .ok_or_else(reject)?;
                if flow.read(&candidate.site, binding).and_then(|value| {
                    flow.alias_entry_origin(value, |site| input.function().variable_ref(site))
                }) != Some(root)
                {
                    return Err(reject());
                }
            }
        }
        if let BorrowedCallActualValueV1::MethodCall(method) = &candidate.value {
            if ordinary_text_argument_source_v1(
                prepared,
                &OwnedExprSiteV1::new(site.owner(), candidate.site.clone()),
                method,
            )?
            .is_some()
            {
                if identity.core_arguments.contains_key(&ordinal) {
                    return Err(reject());
                }
                continue;
            }
            let receipt = identity.core_arguments.get(&ordinal).ok_or_else(reject)?;
            if !std::sync::Arc::ptr_eq(method, receipt.source().source())
                || !input
                    .function()
                    .method_call_source_v1(method.site())
                    .is_some_and(|call| std::sync::Arc::ptr_eq(call, method))
                || old.is_some_and(|old| {
                    !old.core_arguments
                        .get(&ordinal)
                        .is_some_and(|original| Rc::ptr_eq(original, receipt))
                })
            {
                return Err(reject());
            }
        } else if identity.core_arguments.contains_key(&ordinal) {
            return Err(reject());
        }
    }
    if identity
        .core_arguments
        .keys()
        .any(|ordinal| *ordinal as usize >= identity.candidates.len())
    {
        return Err(reject());
    }
    identity.argument_sources = model::bind_original_argument_plans_v1(
        input,
        super::super::super::borrowed_formal_uses::BorrowedCallSourceLoanV1::Static(
            &identity.source,
        ),
        identity.source.method_source(),
        &identity.candidates,
        old.map(|old| &old.argument_sources),
    )
    .map_err(|_| reject())?;
    Ok(())
}

/// Only explicit source Observe can invalidate a previous Home walk's current
/// qualification. Generic staging remains strict; executable phases never reset.
pub(in crate::mir::normal_callable_semantic_package) fn stage_observed_call_actuals_v1(
    staged: &mut PendingBorrowedFormalActualsV1,
    site: &OwnedExprSiteV1,
    result: Result<Option<PreparedBorrowedCallActualsV1>, String>,
) {
    let retain_static_qualification = match (staged.get(site), &result) {
        (Some(Ok(previous)), Ok(Some(current))) => match (&previous.phase, &current.phase) {
            (
                BorrowedCallActualEvidencePhaseV1::SourceStatic(old),
                BorrowedCallActualEvidencePhaseV1::SourceStatic(new),
            ) if old.same_original_source_v1(new)
                && previous.opaque_actuals == current.opaque_actuals
                && previous.ordered_arguments == current.ordered_arguments
                && old.argument_sources.len() == new.argument_sources.len()
                && new
                    .argument_sources
                    .values()
                    .all(|slot| matches!(slot, ArgumentExpressionV1::Source(_))) =>
            {
                old.argument_sources.iter().all(|(ordinal, old_slot)| {
                    match (old_slot, new.argument_sources.get(ordinal)) {
                        (
                            ArgumentExpressionV1::Source(old),
                            Some(ArgumentExpressionV1::Source(new)),
                        ) => Rc::ptr_eq(old, new),
                        (
                            ArgumentExpressionV1::Qualified(frame),
                            Some(ArgumentExpressionV1::Source(plan)),
                        ) => {
                            Rc::ptr_eq(frame.source(), plan)
                                && Rc::ptr_eq(frame.source_loan().plan(), plan)
                                && frame.evaluation().reference_coverage_is_exact()
                        }
                        _ => false,
                    }
                }) && old
                    .argument_sources
                    .values()
                    .any(|slot| matches!(slot, ArgumentExpressionV1::Qualified(_)))
            }
            _ => false,
        },
        _ => false,
    };
    let reset = match (staged.get(site), &result) {
        (Some(Ok(previous)), Ok(Some(current))) => match (&previous.phase, &current.phase) {
            (
                BorrowedCallActualEvidencePhaseV1::SourceStatic(old),
                BorrowedCallActualEvidencePhaseV1::SourceStatic(new),
            ) => {
                old.same_original_source_v1(new)
                    && previous.opaque_actuals == current.opaque_actuals
                    && previous.ordered_arguments == current.ordered_arguments
                    && new
                        .argument_sources
                        .values()
                        .all(|row| matches!(row, ArgumentExpressionV1::Source(_)))
            }
            (
                BorrowedCallActualEvidencePhaseV1::SourceInstance(old),
                BorrowedCallActualEvidencePhaseV1::SourceInstance(new),
            ) => {
                old.same_original_source_v1(new)
                    && (old.argument_sources.is_empty()
                        || old.argument_sources.len() == new.argument_sources.len()
                            && old.argument_sources.iter().all(|(ordinal, plan)| {
                                new.argument_sources
                                    .get(ordinal)
                                    .is_some_and(|other| Rc::ptr_eq(plan, other))
                            }))
                    && previous.opaque_actuals == current.opaque_actuals
                    && previous.ordered_arguments == current.ordered_arguments
                    && new
                        .argument_sources
                        .values()
                        .all(|row| matches!(row, ArgumentExpressionV1::Source(_)))
            }
            (
                BorrowedCallActualEvidencePhaseV1::SourceObject(old),
                BorrowedCallActualEvidencePhaseV1::SourceObject(new),
            ) => {
                old.same_original_source_v1(new)
                    && (old.argument_sources.is_empty()
                        || old.argument_sources.len() == new.argument_sources.len()
                            && old.argument_sources.iter().all(|(ordinal, plan)| {
                                new.argument_sources
                                    .get(ordinal)
                                    .is_some_and(|other| Rc::ptr_eq(plan, other))
                            }))
                    && previous.opaque_actuals == current.opaque_actuals
                    && previous.ordered_arguments == current.ordered_arguments
                    && new
                        .argument_sources
                        .values()
                        .all(|row| matches!(row, ArgumentExpressionV1::Source(_)))
            }
            _ => false,
        },
        _ => false,
    };
    if crate::config::env::builder_debug_enabled() && reset {
        eprintln!("[argument-expression] source re-observe reset site={site:?}");
    }
    if reset {
        let mut current = result
            .expect("checked observation")
            .expect("checked source tuple");
        if retain_static_qualification {
            let Some(Ok(previous)) = staged.get(site) else {
                unreachable!("retained source row")
            };
            let BorrowedCallActualEvidencePhaseV1::SourceStatic(old) = &previous.phase else {
                unreachable!("retained Static source row")
            };
            let BorrowedCallActualEvidencePhaseV1::SourceStatic(new) = &mut current.phase else {
                unreachable!("retained Static source row")
            };
            for (ordinal, old_slot) in &old.argument_sources {
                if let ArgumentExpressionV1::Qualified(frame) = old_slot {
                    let Some(ArgumentExpressionV1::Source(plan)) =
                        new.argument_sources.get(ordinal)
                    else {
                        unreachable!("exact source-plan correspondence checked")
                    };
                    debug_assert!(Rc::ptr_eq(frame.source(), plan));
                    new.argument_sources
                        .insert(*ordinal, ArgumentExpressionV1::Qualified(Rc::clone(frame)));
                }
            }
        }
        staged.insert(site.clone(), Ok(current));
    } else {
        super::super::stage_borrowed_call_actuals_v1(staged, site, result);
    }
}

#[derive(Debug, Clone)]
enum ArgumentMethodSourceV1 {
    Core(Rc<crate::mir::normal_callable_semantic_package::core_method_source::OrdinaryCoreSourceCallV1>),
    OrdinaryText(Rc<StaticIncomingSourceV1>),
    DynamicArrayCarrierAppend(
        crate::mir::source_call_target::VerifiedSourceBoundDynamicMemberCallV1,
    ),
}
impl PartialEq for ArgumentMethodSourceV1 {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Core(a), Self::Core(b)) => Rc::ptr_eq(a, b),
            (Self::OrdinaryText(a), Self::OrdinaryText(b)) => Rc::ptr_eq(a, b),
            (Self::DynamicArrayCarrierAppend(a), Self::DynamicArrayCarrierAppend(b)) => a == b,
            _ => false,
        }
    }
}
impl Eq for ArgumentMethodSourceV1 {}
impl ArgumentMethodSourceV1 {
    fn core(&self) -> Option<&Rc<crate::mir::normal_callable_semantic_package::core_method_source::OrdinaryCoreSourceCallV1>>{
        match self {
            Self::Core(row) => Some(row),
            _ => None,
        }
    }
    fn dynamic_array_carrier_append(
        &self,
    ) -> Option<&crate::mir::source_call_target::VerifiedSourceBoundDynamicMemberCallV1> {
        match self {
            Self::DynamicArrayCarrierAppend(source) => Some(source),
            _ => None,
        }
    }
}

/// One original source loan, never an evaluation, entry or ownership grant.
/// Core and ordinary Text calls retain their own SAME original source products.
#[derive(Debug, Clone)]
pub(crate) struct ArgumentExpressionSourceLoanV1 {
    plan: Rc<ExpressionSourcePlanV1>,
    i64_result: bool,
    calls: BTreeMap<usize, ArgumentMethodSourceV1>,
    dynamic_array_carrier_append:
        Option<crate::mir::normal_callable_semantic_package::NamedArrayHolderSourceLoanV1>,
}
impl PartialEq for ArgumentExpressionSourceLoanV1 {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.plan, &other.plan)
            && self.i64_result == other.i64_result
            && self.calls == other.calls
            && self.dynamic_array_carrier_append == other.dynamic_array_carrier_append
    }
}
impl Eq for ArgumentExpressionSourceLoanV1 {}
impl ArgumentExpressionSourceLoanV1 {
    pub(crate) fn requires_i64_v1(&self) -> bool {
        self.i64_result
    }
    pub(crate) fn plan(&self) -> &Rc<ExpressionSourcePlanV1> {
        &self.plan
    }
    pub(crate) fn dynamic_array_carrier_append_v1(
        &self,
    ) -> Option<&crate::mir::normal_callable_semantic_package::NamedArrayHolderSourceLoanV1> {
        self.dynamic_array_carrier_append.as_ref()
    }
    pub(crate) fn dynamic_array_carrier_call_is_v1(&self, site: &SourceExprSiteV1) -> bool {
        self.calls.iter().any(|(node, source)| {
            source
                .dynamic_array_carrier_append()
                .is_some_and(|original| {
                    original.owner() == self.plan.owner()
                        && original.call_site() == site
                        && self
                            .plan
                            .nodes()
                            .get(*node)
                            .is_some_and(|node| node.site() == site)
                })
        })
    }
    pub(crate) fn dynamic_array_carrier_append_root_v1(&self) -> bool {
        let Some(holder) = self.dynamic_array_carrier_append.as_ref() else {
            return false;
        };
        let Some(root) = self.plan.nodes().first() else {
            return false;
        };
        let crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1::CallArgument {
            call,
            ordinal,
            ..
        } = self.plan.root()
        else {
            return false;
        };
        *ordinal < call.arity()
            && holder.binding().owner() == self.plan.owner()
            && holder
                .append_candidate(&OwnedExprSiteV1::new(
                    self.plan.owner(),
                    call.site().clone(),
                ))
                .is_some_and(|candidate| candidate.site() == root.site())
    }
    pub(crate) fn ordinary_text_call_is(&self, site: &SourceExprSiteV1) -> bool {
        self.calls.iter().any(|(node, source)| matches!(source, ArgumentMethodSourceV1::OrdinaryText(original)
            if original.call_site().owner() == self.plan.owner() && original.call_site().site() == site
                && self.plan.nodes().get(*node).is_some_and(|node| node.site() == site)))
    }
    pub(crate) fn signed_operand_contains(&self, site: &SourceExprSiteV1) -> bool {
        self.i64_result && self.plan.nodes().iter().any(|node| node.site() == site)
            || self.calls.values().filter_map(ArgumentMethodSourceV1::core).any(|receipt| { let core = receipt.source();
                self.plan.nodes().iter().any(|node| node.site() == site)
                    && core.source().arguments().iter().zip(core.target().parameters()).any(|(arg, kind)|
                        *kind == crate::mir::resolved_semantics::CoreMethodHomeParameterRelationV1::I64Parameter
                            && site.node().segments().starts_with(arg.site().node().segments())) })
    }
    pub(crate) fn dynamic_operand_contains(&self, site: &SourceExprSiteV1) -> bool {
        self.calls
            .values()
            .filter_map(ArgumentMethodSourceV1::core)
            .any(|receipt| receipt.operands().dynamic_i64_operand_contains(site))
    }
    pub(crate) fn text_operand_is(&self, site: &SourceExprSiteV1) -> bool {
        self.calls.values().filter_map(ArgumentMethodSourceV1::core).any(|receipt| {
            let core = receipt.source(); site == core.source().receiver_site()
                || core.source().arguments().iter().zip(core.target().parameters()).any(|(arg, kind)|
                    *kind == crate::mir::resolved_semantics::CoreMethodHomeParameterRelationV1::TextParameter && arg.site() == site)
        }) || !self.i64_result && self.resolver_source().is_none() && self.plan.nodes_outside_calls().any(|(_, node)| node.site() == site
            && matches!(node.source(), crate::mir::resolved_value_profile::expression_plan::ExpressionNodeSourceV1::Reference(_)))
    }
    pub(crate) fn resolver_source(
        &self,
    ) -> Option<&crate::mir::resolved_semantics::VerifiedResolverOrdinaryCoreMethodSourceV1> {
        self.calls
            .get(&0)
            .and_then(ArgumentMethodSourceV1::core)
            .map(|core| core.source())
    }
}

/// Invocation-local borrowed view of the SAME original ordinal product. The
/// optional catalog is unnecessary for Static (its source owns all formals),
/// and mandatory for Instance; absence never invents an empty formal contract.
struct ArgumentSourceIdentityRefV1<'a> {
    source: super::super::super::borrowed_formal_uses::BorrowedCallSourceLoanV1<'a>,
    candidates: &'a [BorrowedCallActualCandidateV1],
    slots: &'a BTreeMap<u32, ArgumentExpressionV1>,
    formals: &'a [crate::mir::normal_callable_semantic_package::model::OwnedCallableParameterContractV1],
}
fn argument_source_identity_v1<'a>(
    prepared: &'a PreparedBorrowedFormalIngressV1,
    contracts: Option<&'a [OwnedCallableParameterContractDeclarationV1]>,
    rows: &'a PreparedBorrowedCallActualsV1,
    site: &OwnedExprSiteV1,
) -> Result<Option<ArgumentSourceIdentityRefV1<'a>>, String> {
    use super::super::super::borrowed_formal_uses::BorrowedCallSourceLoanV1 as Loan;
    let rows = rows.source_static_snapshot_v1().unwrap_or(rows);
    match &rows.phase {
        BorrowedCallActualEvidencePhaseV1::SourceStatic(identity) => {
            let identity = validate_static_source_actuals_v1(
                prepared,
                rows,
                site,
                identity.source.required_i64_arguments(),
            )?;
            Ok(Some(ArgumentSourceIdentityRefV1 {
                source: Loan::Static(&identity.source),
                candidates: &identity.candidates,
                slots: &identity.argument_sources,
                formals: identity.source.parameters(),
            }))
        }
        BorrowedCallActualEvidencePhaseV1::SourceInstance(identity)
        | BorrowedCallActualEvidencePhaseV1::ExecutableSourceInstance(identity) => {
            let reject = || freeze("borrowed-instance/argument-source-identity");
            let contracts = contracts.ok_or_else(reject)?;
            let original = crate::mir::normal_callable_semantic_package::OrdinaryNewClaimLedgerV1::original_instance_input_call_v1(prepared, contracts, site)?.ok_or_else(reject)?;
            if original.source.require_instance()? != &identity.source
                || original.arguments != identity.incoming_arguments
            {
                return Err(reject());
            }
            let mut matches = contracts
                .iter()
                .filter(|row| row.owner == identity.source.callee_owner());
            let contract = matches.next().ok_or_else(reject)?;
            if matches.next().is_some()
                || contract.batch_slot != identity.source.target_batch_slot()
                || contract.mode != Loan::Instance(&identity.source).declaration_mode()
                || contract.parameters.len() != identity.candidates.len()
                || contract
                    .parameters
                    .iter()
                    .enumerate()
                    .any(|(index, formal)| {
                        formal.ordinal as usize != index
                            || formal.binding.owner() != contract.owner
                            || identity.source.argument_sites().get(index)
                                != identity
                                    .candidates
                                    .get(index)
                                    .map(|candidate| &candidate.site)
                    })
            {
                return Err(reject());
            }
            let mut rebuilt = instance_source::prepare_instance_source_actuals_v1(
                prepared,
                contracts,
                site,
                &identity.candidates,
            )?
            .ok_or_else(reject)?;
            model::retain_instance_argument_sources_v1(&mut rebuilt, identity)?;
            if rebuilt.ordered_arguments != rows.ordered_arguments {
                return Err(reject());
            }
            match &rows.phase {
                BorrowedCallActualEvidencePhaseV1::SourceInstance(_) if &rebuilt != rows => {
                    return Err(reject())
                }
                BorrowedCallActualEvidencePhaseV1::ExecutableSourceInstance(_) => {
                    rows.checked_ordered_arguments_v1(&original)?;
                }
                _ => {}
            }
            Ok(Some(ArgumentSourceIdentityRefV1 {
                source: Loan::Instance(&identity.source),
                candidates: &identity.candidates,
                slots: &identity.argument_sources,
                formals: &contract.parameters,
            }))
        }
        BorrowedCallActualEvidencePhaseV1::SourceObject(identity)
        | BorrowedCallActualEvidencePhaseV1::ExecutableSourceObject(identity) => {
            let reject = || freeze("borrowed-object/source-actual-identity");
            let contracts = contracts.ok_or_else(reject)?;
            let mut originals = prepared
                .source_incoming
                .exact_rows()
                .filter(|row| &row.call == site);
            let original = originals.next().ok_or_else(reject)?;
            if originals.next().is_some()
                || original.source.require_instance()? != &identity.source
                || original.arguments != identity.incoming_arguments
            {
                return Err(reject());
            }
            let mut matches = contracts
                .iter()
                .filter(|row| row.owner == identity.source.callee_owner());
            let contract = matches.next().ok_or_else(reject)?;
            if matches.next().is_some()
                || contract.batch_slot != identity.source.target_batch_slot()
                || contract.mode != Loan::Instance(&identity.source).declaration_mode()
                || contract.parameters.len() != identity.candidates.len()
                || contract
                    .parameters
                    .iter()
                    .enumerate()
                    .any(|(index, formal)| {
                        formal.ordinal as usize != index
                            || formal.binding.owner() != contract.owner
                            || identity.source.argument_sites().get(index)
                                != identity
                                    .candidates
                                    .get(index)
                                    .map(|candidate| &candidate.site)
                    })
            {
                return Err(reject());
            }
            let mut rebuilt = object_source::prepare_object_source_actuals_v1(
                prepared,
                contracts,
                site,
                &identity.candidates,
            )?
            .ok_or_else(reject)?;
            model::retain_object_argument_sources_v1(&mut rebuilt, identity)?;
            if rebuilt.ordered_arguments != rows.ordered_arguments {
                return Err(reject());
            }
            match &rows.phase {
                BorrowedCallActualEvidencePhaseV1::SourceObject(_) if &rebuilt != rows => {
                    return Err(reject())
                }
                BorrowedCallActualEvidencePhaseV1::ExecutableSourceObject(_) => {
                    rows.checked_ordered_arguments_v1(original)?;
                }
                _ => {}
            }
            Ok(Some(ArgumentSourceIdentityRefV1 {
                source: Loan::Instance(&identity.source),
                candidates: &identity.candidates,
                slots: &identity.argument_sources,
                formals: &contract.parameters,
            }))
        }
        _ => Ok(None),
    }
}

/// Shared by the passive projection and source lender. In particular this
/// checks the complete original tuple, not just the requested ordinal.
pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn validate_static_source_actuals_v1<
    'a,
>(
    prepared: &PreparedBorrowedFormalIngressV1,
    rows: &'a PreparedBorrowedCallActualsV1,
    site: &OwnedExprSiteV1,
    required_i64_arguments: &[u32],
) -> Result<&'a StaticSourceActualIdentityV1, String> {
    let rows = rows
        .source_static_snapshot_v1()
        .ok_or_else(|| freeze("borrowed-static/source-phase-required"))?;
    let BorrowedCallActualEvidencePhaseV1::SourceStatic(identity) = &rows.phase else {
        return Err(freeze("borrowed-static/source-phase-required"));
    };
    let retained = original_static_source_v1(prepared, site)?
        .ok_or_else(|| freeze("borrowed-static/source-observation-missing"))?;
    if !Rc::ptr_eq(retained, &identity.source)
        || identity.source.call_site() != site
        || required_i64_arguments != identity.source.required_i64_arguments()
        || rows.ordered_arguments.len() != identity.source.argument_sites().len()
        || identity.candidates.len() != rows.ordered_arguments.len()
        || identity.integer_evidence.len() != rows.ordered_arguments.len()
        || !rows.opaque_actuals.is_empty()
    {
        return Err(freeze("borrowed-static/source-projection-identity"));
    }
    for (index, ((formal, candidate), argument)) in identity
        .source
        .parameters()
        .iter()
        .zip(identity.candidates.iter())
        .zip(rows.ordered_arguments.iter())
        .enumerate()
    {
        if formal.ordinal as usize != index
            || candidate.ordinal != formal.ordinal
            || formal.binding.owner() != identity.source.callee_owner()
            || identity.source.argument_sites().get(index) != Some(&candidate.site)
        {
            return Err(freeze("borrowed-static/source-projection-identity"));
        }
        let plan = identity
            .argument_sources
            .get(&candidate.ordinal)
            .filter(|plan| {
                corroborates_argument_plan_v1(
                    plan,
                    &identity.source,
                    candidate.ordinal,
                    &candidate.site,
                )
            })
            .ok_or_else(|| freeze("borrowed-static/core-source-identity"))?;
        use crate::mir::resolved_semantics::{
            ResolvedLexicalRefV1 as Ref, ResolvedLiteralSourceV1 as Literal,
        };
        use crate::mir::resolved_value_profile::expression_plan::ExpressionNodeSourceV1 as Node;
        match (&candidate.value, plan.nodes()[0].source()) {
            (BorrowedCallActualValueV1::Expression, Node::Binary(_))
                if identity.integer_evidence.get(index) == Some(&false) => {}
            (BorrowedCallActualValueV1::Expression, _) => {
                return Err(freeze("borrowed-static/argument-source-identity"))
            }
            (
                BorrowedCallActualValueV1::String(value),
                Node::Literal(Literal::String(original)),
            ) if value == original => {}
            (BorrowedCallActualValueV1::String(_), _) => {
                return Err(freeze("borrowed-static/argument-source-identity"))
            }
            (
                BorrowedCallActualValueV1::Binding(binding),
                Node::Reference(Ref::Local(original)),
            ) if binding == original => {}
            (BorrowedCallActualValueV1::Binding(_), _) => {
                return Err(freeze("borrowed-static/argument-source-identity"))
            }
            _ => {}
        }
        match (
            &candidate.value,
            identity.core_arguments.get(&candidate.ordinal),
        ) {
            (BorrowedCallActualValueV1::MethodCall(method), Some(receipt)) => {
                let site = OwnedExprSiteV1::new(site.owner(), candidate.site.clone());
                if !identity.argument_sources.get(&candidate.ordinal).is_some_and(|plan|
                    corroborates_argument_source_v1(plan, identity.source.as_ref(), candidate.ordinal, method))
                    || !prepared.ordinary_core_source_v1(&site).is_some_and(|original| Rc::ptr_eq(original, receipt))
                    || !std::sync::Arc::ptr_eq(method, receipt.source().source())
                    || method.owner() != site.owner() || method.site() != site.site()
                    || identity.integer_evidence.get(index) != Some(&false)
                    || receipt.source().target().schema() != crate::mir::resolved_semantics::CoreMethodHomeSchemaV1::StringBoxText
                    || !match (&formal.kind, receipt.source().target().result()) {
                        (kind, crate::mir::resolved_semantics::CoreMethodHomeResultRelationV1::TextToCaller) => kind.is_ordinary_borrowed_handle(),
                        (CallableParameterContractKindV1::ExactTrivial(abi), crate::mir::resolved_semantics::CoreMethodHomeResultRelationV1::I64ToCaller) => abi.is_i64(),
                        _ => false,
                    }
                { return Err(freeze("borrowed-static/core-source-identity")); }
            }
            (BorrowedCallActualValueV1::MethodCall(method), None)
                if ordinary_text_argument_source_v1(
                    prepared,
                    &OwnedExprSiteV1::new(site.owner(), candidate.site.clone()),
                    method,
                )?
                .is_some() =>
            {
                if !matches!(formal.kind, CallableParameterContractKindV1::OpaqueHandle)
                    || identity.integer_evidence.get(index) != Some(&false)
                    || !corroborates_argument_source_v1(
                        plan,
                        &identity.source,
                        candidate.ordinal,
                        method,
                    )
                {
                    return Err(freeze("borrowed-static/core-source-identity"));
                }
            }
            (BorrowedCallActualValueV1::MethodCall(_), None) | (_, Some(_)) => {
                return Err(freeze("borrowed-static/core-source-identity"))
            }
            (_, None) => {}
        }
        let matches = match (&formal.kind, &candidate.value, argument) {
            (kind, _, LocalCallArgumentV1::BorrowedActual { ordinal, site })
                if kind.is_ordinary_borrowed_handle() =>
            {
                *ordinal == formal.ordinal && site == &candidate.site
            }
            (
                CallableParameterContractKindV1::ExactTrivial(abi),
                BorrowedCallActualValueV1::Integer(expected),
                LocalCallArgumentV1::Integer(actual),
            ) if abi.is_i64() => expected == actual,
            (
                CallableParameterContractKindV1::ExactTrivial(abi),
                BorrowedCallActualValueV1::Scalar(expected, SourceScalarKind::Integer),
                LocalCallArgumentV1::Scalar(actual),
            ) if abi.is_i64() => expected == actual && actual.owner() == site.owner(),
            (
                CallableParameterContractKindV1::ExactTrivial(abi),
                BorrowedCallActualValueV1::Expression | BorrowedCallActualValueV1::MethodCall(_),
                LocalCallArgumentV1::Expression {
                    ordinal,
                    site: original,
                },
            ) if abi.is_i64() => *ordinal == formal.ordinal && original == &candidate.site,
            _ => false,
        };
        if !matches {
            return Err(freeze("borrowed-static/source-projection-identity"));
        }
    }
    if identity.argument_sources.len() != identity.candidates.len()
        || identity
            .argument_sources
            .keys()
            .any(|ordinal| *ordinal as usize >= identity.candidates.len())
        || identity
            .core_arguments
            .keys()
            .any(|ordinal| *ordinal as usize >= identity.candidates.len())
    {
        return Err(freeze("borrowed-static/core-source-identity"));
    }
    Ok(identity)
}

/// During Home scanning this lends only already-qualified source arguments.
/// Accepted Home membership and executable payload belong to later activation.
pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn validate_qualified_argument_sources_v1(
    prepared: &PreparedBorrowedFormalIngressV1,
    pending: &PendingBorrowedFormalActualsV1,
    site: &OwnedExprSiteV1,
    identity: &StaticSourceActualIdentityV1,
) -> Result<(), String> {
    use crate::mir::resolved_value_profile::{
        expression_class::ExpressionValueClassV1 as Class,
        expression_projection::ExpressionLeafClassV1 as Leaf,
    };
    for actual in &identity.candidates {
        let core = identity.core_arguments.contains_key(&actual.ordinal);
        let ordinary_text = match &actual.value {
            BorrowedCallActualValueV1::MethodCall(method) => ordinary_text_argument_source_v1(
                prepared,
                &OwnedExprSiteV1::new(site.owner(), actual.site.clone()),
                method,
            )?
            .is_some(),
            _ => false,
        };
        let finite_integer = matches!(
            actual.value,
            BorrowedCallActualValueV1::Scalar(_, SourceScalarKind::Integer)
        ) && identity
            .source
            .parameters()
            .get(actual.ordinal as usize)
            .is_some_and(|formal| {
                matches!(formal.kind, CallableParameterContractKindV1::OpaqueHandle)
                    && prepared
                        .source_value_sum_v1(formal.binding)
                        .is_some_and(|sum| sum.signed_only())
            });
        let expression = core
            || ordinary_text
            || finite_integer
            || matches!(
                actual.value,
                BorrowedCallActualValueV1::Expression
                    | BorrowedCallActualValueV1::String(_)
                    | BorrowedCallActualValueV1::Binding(_)
            )
            || matches!(actual.value, BorrowedCallActualValueV1::SelfRooted { .. })
                && !prepared
                    .static_arguments
                    .contains_key(&(site.clone(), actual.ordinal));
        if !expression {
            continue;
        }
        let slot = identity
            .argument_sources
            .get(&actual.ordinal)
            .ok_or_else(|| freeze("borrowed-static/core-source-identity"))?;
        let ArgumentExpressionV1::Qualified(frame) = slot else {
            if crate::config::env::builder_debug_enabled() {
                eprintln!("[static-argument] site={site:?} ordinal={} target={:?} actual={:?} pending={slot:?}",
                    actual.ordinal, identity.source.target(), actual.value);
            }
            return Err(freeze(if core {
                "borrowed-static/core-expression-execution-pending"
            } else {
                "borrowed-static/text-expression-execution-pending"
            }));
        };
        let original =
            lend_argument_source_inner_v1(prepared, pending, None, site, actual.ordinal)?
                .ok_or_else(|| freeze("borrowed-static/argument-frame-identity"))?;
        if !Rc::ptr_eq(frame.source(), slot)
            || frame.source_loan() != &original
            || !frame.evaluation().reference_coverage_is_exact()
            || frame.evaluation().root_class()
                != Leaf::Value(if original.requires_i64_v1() {
                    Class::I64
                } else if original.dynamic_array_carrier_append_root_v1()
                    && original
                        .plan()
                        .nodes()
                        .first()
                        .is_some_and(|root| original.dynamic_array_carrier_call_is_v1(root.site()))
                {
                    Class::Dynamic
                } else {
                    Class::String
                })
        {
            return Err(freeze("borrowed-static/argument-frame-identity"));
        }
    }
    Ok(())
}

pub(in crate::mir::normal_callable_semantic_package) fn lend_argument_expression_source_v1(
    prepared: &PreparedBorrowedFormalIngressV1,
    pending: &PendingBorrowedFormalActualsV1,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    site: &OwnedExprSiteV1,
    ordinal: u32,
) -> Result<Option<ArgumentExpressionSourceLoanV1>, String> {
    lend_argument_source_inner_v1(prepared, pending, Some(contracts), site, ordinal)
}
pub(in crate::mir::normal_callable_semantic_package) fn lend_array_carrier_append_argument_source_v1(
    prepared: &PreparedBorrowedFormalIngressV1,
    pending: &PendingBorrowedFormalActualsV1,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    site: &OwnedExprSiteV1,
    ordinal: u32,
    holder: &crate::mir::normal_callable_semantic_package::NamedArrayHolderSourceLoanV1,
    dynamic_calls: &[crate::mir::source_call_target::VerifiedSourceBoundDynamicMemberCallV1],
) -> Result<Option<ArgumentExpressionSourceLoanV1>, String> {
    lend_argument_source_inner_for_array_v1(
        prepared,
        pending,
        Some(contracts),
        site,
        ordinal,
        Some(holder.clone()),
        dynamic_calls,
    )
}
fn lend_argument_source_inner_v1(
    prepared: &PreparedBorrowedFormalIngressV1,
    pending: &PendingBorrowedFormalActualsV1,
    contracts: Option<&[OwnedCallableParameterContractDeclarationV1]>,
    site: &OwnedExprSiteV1,
    ordinal: u32,
) -> Result<Option<ArgumentExpressionSourceLoanV1>, String> {
    lend_argument_source_inner_for_array_v1(prepared, pending, contracts, site, ordinal, None, &[])
}
fn lend_argument_source_inner_for_array_v1(
    prepared: &PreparedBorrowedFormalIngressV1,
    pending: &PendingBorrowedFormalActualsV1,
    contracts: Option<&[OwnedCallableParameterContractDeclarationV1]>,
    site: &OwnedExprSiteV1,
    ordinal: u32,
    dynamic_array_carrier_append: Option<
        crate::mir::normal_callable_semantic_package::NamedArrayHolderSourceLoanV1,
    >,
    dynamic_calls: &[crate::mir::source_call_target::VerifiedSourceBoundDynamicMemberCallV1],
) -> Result<Option<ArgumentExpressionSourceLoanV1>, String> {
    let rows = pending
        .get(site)
        .ok_or_else(|| freeze("borrowed-static/source-unobserved"))?
        .as_ref()
        .map_err(Clone::clone)?;
    if let Some(source) = dynamic_invocation_argument_source_v1(rows, site, ordinal)? {
        return Ok(Some(source));
    }
    let Some(identity) = argument_source_identity_v1(prepared, contracts, rows, site)? else {
        return Ok(None);
    };
    let candidate = identity
        .candidates
        .get(ordinal as usize)
        .filter(|row| row.ordinal == ordinal)
        .ok_or_else(|| freeze("borrowed-static/core-source-identity"))?;
    let plan = identity
        .slots
        .get(&ordinal)
        .ok_or_else(|| freeze("borrowed-static/core-source-identity"))?;
    let dynamic_array_carrier_append = match dynamic_array_carrier_append {
        Some(holder)
            if holder.is_dynamic_carrier_append_only_v1()
                && holder.binding().owner() == plan.owner()
                && matches!(plan.root(), ExpressionPlanRootV1::CallArgument {
                    call: original, ordinal: original_ordinal, ..
                } if *original_ordinal == ordinal
                    && holder.append_candidate(&OwnedExprSiteV1::new(plan.owner(), original.site().clone()))
                        .is_some_and(|candidate| plan.nodes().first().is_some_and(|node| node.site() == candidate.site()))) =>
        {
            Some(holder)
        }
        Some(_) => return Ok(None),
        None => None,
    };
    let mut calls = BTreeMap::new();
    let mut call_inputs: Vec<&SourceExprSiteV1> = Vec::new();
    for (node, original) in plan.nodes().iter().enumerate() {
        if call_inputs.iter().any(|call| {
            original
                .site()
                .node()
                .segments()
                .starts_with(call.node().segments())
        }) {
            continue;
        }
        if !matches!(
            original.source(),
            crate::mir::resolved_value_profile::expression_plan::ExpressionNodeSourceV1::MethodCall
        ) {
            continue;
        }
        let site = OwnedExprSiteV1::new(plan.owner(), original.site().clone());
        if let Some(core) = prepared.ordinary_core_source_v1(&site) {
            calls.insert(node, ArgumentMethodSourceV1::Core(Rc::clone(core)));
        } else if node == 0 && dynamic_array_carrier_append.is_some() {
            let mut matches = dynamic_calls.iter().filter(|source| {
                source.owner() == plan.owner() && source.call_site() == site.site()
            });
            if let Some(dynamic) = matches.next() {
                if matches.next().is_some() || dynamic.result_site() != site.site() {
                    return Err(freeze("borrowed-static/dynamic-array-append-source"));
                }
                calls.insert(
                    node,
                    ArgumentMethodSourceV1::DynamicArrayCarrierAppend(dynamic.clone()),
                );
            }
            call_inputs.push(original.site());
        } else {
            if let Some(original) = original_static_source_v1(prepared, &site)? {
                if let Some(source) =
                    ordinary_text_argument_source_v1(prepared, &site, original.method_source())?
                {
                    calls.insert(node, ArgumentMethodSourceV1::OrdinaryText(source));
                }
            }
            call_inputs.push(original.site());
        }
    }
    if let BorrowedCallActualValueV1::MethodCall(method) = &candidate.value {
        let Some(source) = calls.get(&0) else {
            return Ok(None);
        };
        let original = match source {
            ArgumentMethodSourceV1::Core(core) => Some(core.source().source()),
            ArgumentMethodSourceV1::OrdinaryText(row) => Some(row.method_source()),
            ArgumentMethodSourceV1::DynamicArrayCarrierAppend(_) => {
                if source
                    .dynamic_array_carrier_append()
                    .is_some_and(|dynamic| dynamic.call_site() == &candidate.site)
                {
                    None
                } else {
                    return Err(freeze("borrowed-static/dynamic-array-append-identity"));
                }
            }
        };
        if original.is_some_and(|original| !std::sync::Arc::ptr_eq(method, original)) {
            return Err(freeze("borrowed-static/core-source-identity"));
        }
    }
    let formal = identity
        .formals
        .get(ordinal as usize)
        .ok_or_else(|| freeze("borrowed-static/argument-formal"))?;
    let i64_result = matches!(&formal.kind, CallableParameterContractKindV1::ExactTrivial(abi) if abi.is_i64())
        || matches!(
            candidate.value,
            BorrowedCallActualValueV1::Expression
                | BorrowedCallActualValueV1::Binding(_)
                | BorrowedCallActualValueV1::Scalar(_, SourceScalarKind::Integer)
        ) && matches!(formal.kind, CallableParameterContractKindV1::OpaqueHandle)
            && prepared
                .source_value_sum_v1(formal.binding)
                .is_some_and(|sum| sum.signed_only());
    Ok(Some(ArgumentExpressionSourceLoanV1 {
        plan: Rc::clone(plan),
        i64_result,
        calls,
        dynamic_array_carrier_append,
    }))
}

fn dynamic_invocation_argument_source_v1(
    rows: &PreparedBorrowedCallActualsV1,
    site: &OwnedExprSiteV1,
    ordinal: u32,
) -> Result<Option<ArgumentExpressionSourceLoanV1>, String> {
    let Some(dynamic) = rows.dynamic_invocation_arguments.as_ref() else {
        return Ok(None);
    };
    let reject = || freeze("dynamic-argument/source-identity");
    let target = dynamic.source.target();
    if site.owner() != target.owner()
        || dynamic.source.call_site() != site
        || dynamic.candidates.len() != target.arguments().len()
        || dynamic.arguments.len() != dynamic.candidates.len()
        || rows.ordered_arguments.len() != dynamic.candidates.len()
    {
        return Err(reject());
    }
    for (index, (candidate, original)) in dynamic
        .candidates
        .iter()
        .zip(target.arguments())
        .enumerate()
    {
        let ordered = rows.ordered_arguments.get(index);
        if candidate.ordinal as usize != index
            || candidate.ordinal != original.ordinal()
            || &candidate.site != original.site()
            || dynamic.source.argument_sites().get(index) != Some(&candidate.site)
            || !matches!(ordered, Some(LocalCallArgumentV1::Expression { ordinal, site })
                if *ordinal == candidate.ordinal && site == &candidate.site)
        {
            return Err(reject());
        }
    }
    let candidate = dynamic
        .candidates
        .get(ordinal as usize)
        .filter(|candidate| candidate.ordinal == ordinal)
        .ok_or_else(reject)?;
    let slot = dynamic.arguments.get(&ordinal).ok_or_else(reject)?;
    let plan = match slot {
        ArgumentExpressionV1::Source(plan) => Rc::clone(plan),
        ArgumentExpressionV1::Qualified(frame) => Rc::clone(frame.source()),
    };
    let ExpressionPlanRootV1::CallArgument {
        statement: _,
        call,
        ordinal: source_ordinal,
    } = plan.root()
    else {
        return Err(reject());
    };
    if *source_ordinal != ordinal
        || plan.owner() != site.owner()
        || call.owner() != site.owner()
        || call.site() != site.site()
        || call.arity() != target.dispatch().arity()
        || call.arguments().len() != target.arguments().len()
        || call
            .arguments()
            .iter()
            .zip(target.arguments())
            .any(|(actual, original)| {
                actual.ordinal() != original.ordinal() || actual.site() != original.site()
            })
        || plan
            .nodes()
            .first()
            .is_none_or(|node| node.site() != &candidate.site)
    {
        return Err(reject());
    }
    Ok(Some(ArgumentExpressionSourceLoanV1 {
        plan,
        i64_result: true,
        calls: BTreeMap::new(),
        dynamic_array_carrier_append: None,
    }))
}

/// The covered observer selects original Text source roles in the same pending
/// argument product. Core results and local/reference reads keep distinct proof.
pub(in crate::mir::normal_callable_semantic_package) fn observed_argument_source_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
    prepared: &PreparedBorrowedFormalIngressV1,
    pending: &PendingBorrowedFormalActualsV1,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    site: &OwnedExprSiteV1,
    ordinal: u32,
) -> Result<Option<ArgumentExpressionSourceLoanV1>, String> {
    if input.owner() != site.owner() {
        return Err(freeze("borrowed-static/core-source-identity"));
    }
    let Some(call) = input.function().method_call_source_v1(site.site()) else {
        return Ok(None);
    };
    let Some(arg) = call
        .arguments()
        .get(ordinal as usize)
        .filter(|arg| arg.ordinal() == ordinal)
    else {
        return Ok(None);
    };
    let Some(rows) = pending.get(site) else {
        return Ok(None);
    };
    let rows = rows.as_ref().map_err(Clone::clone)?;
    if rows.dynamic_invocation_arguments.is_some() {
        return dynamic_invocation_argument_source_v1(rows, site, ordinal);
    }
    if !matches!(
        rows.phase,
        BorrowedCallActualEvidencePhaseV1::SourceStatic(_)
            | BorrowedCallActualEvidencePhaseV1::SourceInstance(_)
            | BorrowedCallActualEvidencePhaseV1::SourceObject(_)
    ) {
        return Ok(None);
    }
    let Some(identity) = argument_source_identity_v1(prepared, Some(contracts), rows, site)? else {
        return Ok(None);
    };
    let Some(formal) = identity
        .formals
        .get(ordinal as usize)
        .filter(|formal| formal.ordinal == ordinal)
    else {
        return Ok(None);
    };
    if !identity.slots.get(&ordinal).is_some_and(|plan| {
        matches!(plan.root(), ExpressionPlanRootV1::CallArgument { call: original, .. }
        if std::sync::Arc::ptr_eq(original, call))
    }) {
        return Err(freeze("borrowed-static/core-source-identity"));
    }
    if matches!(&formal.kind, CallableParameterContractKindV1::ExactTrivial(abi) if abi.is_i64())
        || matches!(formal.kind, CallableParameterContractKindV1::OpaqueHandle)
            && matches!(
                identity
                    .candidates
                    .get(ordinal as usize)
                    .map(|actual| &actual.value),
                Some(
                    BorrowedCallActualValueV1::Expression
                        | BorrowedCallActualValueV1::Binding(_)
                        | BorrowedCallActualValueV1::Scalar(_, SourceScalarKind::Integer)
                )
            )
            && prepared
                .source_value_sum_v1(formal.binding)
                .is_some_and(|sum| sum.signed_only())
    {
        let eligible = match crate::mir::resolved_semantics::home_new_prefix::borrowed_actual_source_atom_v1(input, arg.site()) {
            Some(BorrowedCallActualValueV1::Expression) => true,
            Some(BorrowedCallActualValueV1::Binding(_)) => matches!(formal.kind, CallableParameterContractKindV1::OpaqueHandle),
            Some(BorrowedCallActualValueV1::MethodCall(method)) => prepared.ordinary_core_source_v1(
                &OwnedExprSiteV1::new(input.owner(), arg.site().clone())).is_some_and(|core|
                    std::sync::Arc::ptr_eq(&method, core.source().source())
                        && core.source().target().result() == crate::mir::resolved_semantics::CoreMethodHomeResultRelationV1::I64ToCaller),
            _ => false,
        };
        if !eligible {
            return Ok(None);
        }
        return lend_argument_expression_source_v1(prepared, pending, contracts, site, ordinal);
    }
    if let Some(core) =
        prepared.ordinary_core_source_v1(&OwnedExprSiteV1::new(site.owner(), arg.site().clone()))
    {
        if core.source().target().result()
            != crate::mir::resolved_semantics::CoreMethodHomeResultRelationV1::TextToCaller
        {
            return Ok(None);
        }
        if !input
            .function()
            .method_call_source_v1(arg.site())
            .is_some_and(|method| std::sync::Arc::ptr_eq(method, core.source().source()))
        {
            return Err(freeze("borrowed-static/core-source-identity"));
        }
    } else {
        match crate::mir::resolved_semantics::home_new_prefix::borrowed_actual_source_atom_v1(
            input,
            arg.site(),
        ) {
            Some(BorrowedCallActualValueV1::MethodCall(method)) => {
                if ordinary_text_argument_source_v1(
                    prepared,
                    &OwnedExprSiteV1::new(input.owner(), arg.site().clone()),
                    &method,
                )?
                .is_none()
                {
                    return Ok(None);
                }
            }
            Some(BorrowedCallActualValueV1::String(_) | BorrowedCallActualValueV1::Binding(_)) => {}
            _ if identity
                .slots
                .get(&ordinal)
                .and_then(|plan| plan.nodes().first())
                .and_then(|node| node.binary())
                .is_some_and(|binary| {
                    binary.operator()
                        == crate::mir::resolved_semantics::ResolvedBinaryOperatorV1::Add
                }) => {}
            _ => return Ok(None),
        }
    }
    if let Some(crate::mir::resolved_semantics::ResolvedLexicalRefV1::Local(binding)) =
        input.function().variable_ref(arg.site())
    {
        if let Some(flow) = input.function().expression_source().value_flow() {
            if let Some(root) = flow.read(arg.site(), binding).and_then(|value| {
                flow.alias_entry_origin(value, |site| input.function().variable_ref(site))
            }) {
                if !prepared.source_value_sum_v1(root).is_some_and(|sum| sum.alternatives().len() == 1
                    && sum.alternatives().contains(&super::super::super::borrowed_formal_source::SourceValueAlternativeV1::Text)) { return Ok(None); }
            }
        }
    }
    if !matches!(formal.kind, CallableParameterContractKindV1::OpaqueHandle) {
        return Ok(None);
    }
    lend_argument_expression_source_v1(prepared, pending, contracts, site, ordinal)
}

pub(in crate::mir::normal_callable_semantic_package) fn observed_carrier_array_append_argument_source_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
    site: &OwnedExprSiteV1,
    ordinal: u32,
    holder: &crate::mir::normal_callable_semantic_package::NamedArrayHolderSourceLoanV1,
    dynamic_calls: &[crate::mir::source_call_target::VerifiedSourceBoundDynamicMemberCallV1],
    retained: Option<&ArgumentExpressionSourceLoanV1>,
) -> Result<Option<ArgumentExpressionSourceLoanV1>, String> {
    if input.owner() != site.owner()
        || holder.binding().owner() != site.owner()
        || !holder.is_dynamic_carrier_append_only_v1()
    {
        return Ok(None);
    }
    let Some(call) = input.function().method_call_source_v1(site.site()) else {
        return Ok(None);
    };
    let Some(actual) = call
        .arguments()
        .get(ordinal as usize)
        .filter(|actual| actual.ordinal() == ordinal)
    else {
        return Ok(None);
    };
    if !holder
        .append_candidate(site)
        .is_some_and(|candidate| candidate.site() == actual.site())
    {
        return Ok(None);
    }
    let observed_plan = call_argument_expression_source_plan_v1(input, &call, ordinal)
        .map_err(|_| freeze("array-append/dynamic-expression-plan"))?;
    let plan = if let Some(retained) = retained {
        if observed_plan.as_ref() != retained.plan().as_ref() {
            return Err(freeze("array-append/dynamic-expression-source-drift"));
        }
        Rc::clone(retained.plan())
    } else {
        observed_plan
    };
    let Some(root) = plan.nodes().first() else {
        return Ok(None);
    };
    if root.site() != actual.site()
        || !matches!(
            root.source(),
            crate::mir::resolved_value_profile::expression_plan::ExpressionNodeSourceV1::MethodCall
        )
    {
        return Ok(None);
    }
    let Some(method) = input.function().method_call_source_v1(root.site()) else {
        return Ok(None);
    };
    let mut targets = dynamic_calls
        .iter()
        .filter(|target| target.owner() == input.owner() && target.call_site() == root.site());
    let Some(target) = targets.next() else {
        return Ok(None);
    };
    if targets.next().is_some()
        || target.result_site() != root.site()
        || target.receiver_site() != method.receiver_site()
        || target.dispatch().selector() != method.selector()
        || target.dispatch().arity() != method.arity()
        || target.arguments().len() != method.arguments().len()
        || target
            .arguments()
            .iter()
            .zip(method.arguments())
            .any(|(source, original)| {
                source.ordinal() != original.ordinal() || source.site() != original.site()
            })
    {
        return Err(freeze("array-append/dynamic-expression-target"));
    }
    let source = ArgumentExpressionSourceLoanV1 {
        plan,
        i64_result: false,
        calls: BTreeMap::from([(
            0,
            ArgumentMethodSourceV1::DynamicArrayCarrierAppend(target.clone()),
        )]),
        dynamic_array_carrier_append: Some(holder.clone()),
    };
    let accepted = source.dynamic_array_carrier_append_root_v1()
        && source.dynamic_array_carrier_call_is_v1(actual.site());
    if !accepted {
        return Ok(None);
    }
    if retained.is_some_and(|retained| retained != &source) {
        return Err(freeze("array-append/dynamic-source-current-drift"));
    }
    Ok(Some(source))
}

pub(in crate::mir::normal_callable_semantic_package) fn lend_text_entry_read_v1(
    prepared: &PreparedBorrowedFormalIngressV1,
    pending: &PendingBorrowedFormalActualsV1,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    site: &OwnedExprSiteV1,
    loan: &ArgumentExpressionSourceLoanV1,
    read: &crate::mir::resolved_semantics::SourceExprSiteV1,
    binding: BindingRefV1,
    root: BindingRefV1,
) -> Result<crate::mir::normal_callable_semantic_package::BorrowedTextEntryReadRequirementV1, String>
{
    let ExpressionPlanRootV1::CallArgument { ordinal, call, .. } = loan.plan().root() else {
        return Err(freeze("borrowed-text-entry/source-identity"));
    };
    if !loan.text_operand_is(read)
        || call.site() != site.site()
        || loan.plan().owner() != site.owner()
        || lend_argument_expression_source_v1(prepared, pending, contracts, site, *ordinal)?
            .as_ref()
            != Some(loan)
    {
        return Err(freeze("borrowed-text-entry/source-identity"));
    }
    if loan.resolver_source().is_none()
        && !prepared.source_value_sum_v1(root).is_some_and(|sum| {
            sum.alternatives().len() == 1
                && sum.alternatives().contains(
                    &super::super::super::borrowed_formal_source::SourceValueAlternativeV1::Text,
                )
        })
    {
        return Err(freeze("borrowed-text-entry/source-domain-unclosed"));
    }
    super::super::super::borrowed_formal_entry::issue_text_entry_read_requirement_v1(
        contracts,
        loan.plan(),
        read,
        binding,
        root,
        prepared.original_formal_read_v1(site.owner(), read, binding, root)?,
    )
}

pub(in crate::mir::normal_callable_semantic_package) fn lend_field_array_text_entry_read_v1(
    prepared: &PreparedBorrowedFormalIngressV1,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    input: ResolvedFunctionLoweringInputV1<'_>,
    site: &OwnedExprSiteV1,
    source: &crate::mir::normal_callable_semantic_package::CheckedFieldArrayAccessSourceV1,
    plan: &Rc<ExpressionSourcePlanV1>,
    read: &crate::mir::resolved_semantics::SourceExprSiteV1,
    binding: BindingRefV1,
    root: BindingRefV1,
) -> Result<crate::mir::normal_callable_semantic_package::BorrowedTextEntryReadRequirementV1, String>
{
    let reject = || freeze("borrowed-text-entry/source-identity");
    let Some(method) = source.append_method() else {
        return Err(reject());
    };
    if !source.matches_original_call_v1(input)
        || source.call() != site
        || input.owner() != site.owner()
        || plan.owner() != input.owner()
        || method.owner() != input.owner()
        || !matches!(
            plan.root(),
            ExpressionPlanRootV1::CallArgument {
                call,
                ordinal: 0,
                ..
            } if std::sync::Arc::ptr_eq(call, method)
        )
        || !plan.nodes().iter().any(|node| {
            node.site() == read
                && matches!(
                    node.source(),
                    crate::mir::resolved_value_profile::expression_plan::ExpressionNodeSourceV1::Reference(
                        crate::mir::resolved_semantics::ResolvedLexicalRefV1::Local(actual)
                    ) if *actual == binding
                )
        })
        || !prepared.source_value_sum_v1(root).is_some_and(|sum| {
            sum.alternatives().len() == 1
                && sum.alternatives().contains(
                    &super::super::super::borrowed_formal_source::SourceValueAlternativeV1::Text,
                )
        })
    {
        return Err(reject());
    }
    super::super::super::borrowed_formal_entry::issue_text_entry_read_requirement_v1(
        contracts,
        plan,
        read,
        binding,
        root,
        prepared.original_formal_read_v1(input.owner(), read, binding, root)?,
    )
}

/// Retain qualification in the SAME pending ordinal as its original source.
/// Repeated completion within one observation reuses the exact current frame.
pub(in crate::mir::normal_callable_semantic_package) fn complete_argument_expression_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
    prepared: &PreparedBorrowedFormalIngressV1,
    pending: &mut PendingBorrowedFormalActualsV1,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    site: &OwnedExprSiteV1,
    dynamic_source_calls: &[crate::mir::source_call_target::VerifiedSourceBoundDynamicMemberCallV1],
    frame: &Rc<crate::mir::resolved_semantics::home_new_prefix::CheckedArgumentExpressionFrameV1>,
) -> Result<
    Rc<crate::mir::resolved_semantics::home_new_prefix::CheckedArgumentExpressionFrameV1>,
    String,
> {
    let reject = || freeze("borrowed-static/argument-frame-identity");
    let ExpressionPlanRootV1::CallArgument { call, ordinal, .. } = frame.source().root() else {
        return Err(reject());
    };
    if input.owner() != site.owner()
        || call.owner() != site.owner()
        || call.site() != site.site()
        || frame.source().owner() != site.owner()
        || !frame.evaluation().reference_coverage_is_exact()
    {
        return Err(reject());
    }
    let retained_carrier = frame.source_loan().dynamic_array_carrier_append_v1();
    let original = match retained_carrier {
        Some(holder) => observed_carrier_array_append_argument_source_v1(
            input,
            site,
            *ordinal,
            holder,
            dynamic_source_calls,
            Some(frame.source_loan()),
        )?,
        None => lend_argument_expression_source_v1(prepared, pending, contracts, site, *ordinal)?,
    };
    let original = original.ok_or_else(reject)?;
    if frame.source_loan() != &original || !Rc::ptr_eq(frame.source(), original.plan()) {
        return Err(reject());
    }
    // A Dynamic carrier append belongs to the original Array Home walk, not
    // to the borrowed-formal pending table for a callable argument.
    if retained_carrier.is_some() {
        return Ok(Rc::clone(frame));
    }
    let rows = pending
        .get_mut(site)
        .ok_or_else(reject)?
        .as_mut()
        .map_err(|error| error.clone())?;
    if let Some(dynamic) = rows.dynamic_invocation_arguments.as_mut() {
        let slot = dynamic.arguments.get_mut(ordinal).ok_or_else(reject)?;
        match slot {
            ArgumentExpressionV1::Qualified(previous) => {
                if previous.as_ref() != frame.as_ref() {
                    return Err(freeze("dynamic-argument/frame-current-drift"));
                }
                return Ok(Rc::clone(previous));
            }
            ArgumentExpressionV1::Source(plan) => {
                if !Rc::ptr_eq(plan, frame.source()) {
                    return Err(reject());
                }
                *slot = ArgumentExpressionV1::Qualified(Rc::clone(frame));
                return Ok(Rc::clone(frame));
            }
        }
    }
    let slots = match &mut rows.phase {
        BorrowedCallActualEvidencePhaseV1::SourceStatic(identity) => &mut identity.argument_sources,
        BorrowedCallActualEvidencePhaseV1::SourceInstance(identity) => {
            &mut identity.argument_sources
        }
        BorrowedCallActualEvidencePhaseV1::SourceObject(identity) => &mut identity.argument_sources,
        _ => return Err(reject()),
    };
    let slot = slots.get_mut(ordinal).ok_or_else(reject)?;
    match slot {
        ArgumentExpressionV1::Qualified(previous) => {
            if previous.as_ref() != frame.as_ref() {
                return Err(freeze("borrowed-static/argument-frame-current-drift"));
            }
            Ok(Rc::clone(previous))
        }
        ArgumentExpressionV1::Source(plan) => {
            if !Rc::ptr_eq(plan, frame.source()) {
                return Err(reject());
            }
            *slot = ArgumentExpressionV1::Qualified(Rc::clone(frame));
            Ok(Rc::clone(frame))
        }
    }
}

pub(in crate::mir::normal_callable_semantic_package) fn lend_expression_core_source_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
    prepared: &PreparedBorrowedFormalIngressV1,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    plan: &Rc<ExpressionSourcePlanV1>,
    site: &OwnedExprSiteV1,
) -> Result<Option<crate::mir::normal_callable_semantic_package::ExpressionCoreSourceLoanV1>, String>
{
    if site.owner() != input.owner() || plan.owner() != input.owner() {
        return Err(freeze("expression-core/source-identity"));
    }
    let Some(core) = prepared.ordinary_core_source_v1(site) else {
        return Ok(None);
    };
    let node = plan
        .nodes()
        .iter()
        .position(|node| node.site() == site.site())
        .ok_or_else(|| freeze("expression-core/source-identity"))?;
    let receiver_read =
        lend_core_text_entry_receiver_read_v1(input, prepared, contracts, plan, core)?;
    crate::mir::normal_callable_semantic_package::ExpressionCoreSourceLoanV1::lend(
        input, plan, node, core,
    )?
    .with_text_entry_receiver_read_v1(receiver_read)
    .map(Some)
}

fn lend_core_text_entry_receiver_read_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
    prepared: &PreparedBorrowedFormalIngressV1,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    plan: &Rc<ExpressionSourcePlanV1>,
    core: &Rc<
        crate::mir::normal_callable_semantic_package::core_method_source::OrdinaryCoreSourceCallV1,
    >,
) -> Result<
    Option<crate::mir::normal_callable_semantic_package::BorrowedTextEntryReadRequirementV1>,
    String,
> {
    use crate::mir::resolved_semantics::{
        ResolvedLexicalRefV1, ResolvedMethodCallReceiverSourceV1,
    };
    let source = core.source().source();
    let debug = crate::config::env::builder_debug_enabled();
    if core.source().target().checked_code_point_law_v1().is_none() {
        if debug {
            eprintln!(
                "[core-entry-read] site={:?} no checked code-point law",
                source.site()
            );
        }
        return Ok(None);
    }
    let ResolvedMethodCallReceiverSourceV1::Lexical(ResolvedLexicalRefV1::Local(binding)) =
        source.receiver()
    else {
        if debug {
            eprintln!(
                "[core-entry-read] site={:?} receiver is not a lexical local",
                source.site()
            );
        }
        return Ok(None);
    };
    let Some(flow) = input.function().expression_source().value_flow() else {
        if debug {
            eprintln!(
                "[core-entry-read] site={:?} source value flow unavailable",
                source.site()
            );
        }
        return Ok(None);
    };
    let Some(root) = flow
        .read(source.receiver_site(), binding)
        .and_then(|value| {
            flow.alias_entry_origin(value, |site| input.function().variable_ref(site))
        })
    else {
        if debug {
            eprintln!(
                "[core-entry-read] site={:?} receiver entry origin unavailable",
                source.site()
            );
        }
        return Ok(None);
    };
    let Some(expression_site) = plan.root().expression_site() else {
        if debug {
            eprintln!(
                "[core-entry-read] site={:?} expression root unavailable",
                source.site()
            );
        }
        return Ok(None);
    };
    let expression_root = OwnedExprSiteV1::new(input.owner(), expression_site.clone());
    let Some(read) = lend_finite_entry_read_v1(
        input,
        prepared,
        contracts,
        &expression_root,
        plan,
        source.receiver_site(),
        binding,
        root,
    )?
    else {
        if debug {
            eprintln!(
                "[core-entry-read] site={:?} canonical finite entry read unavailable",
                source.site()
            );
        }
        return Ok(None);
    };
    let requirement = read.requirement().clone();
    if debug {
        eprintln!(
            "[core-entry-read] site={:?} receiver={binding:?} root={root:?} requirement retained",
            source.site()
        );
    }
    Ok(Some(requirement))
}

impl OrdinaryNewClaimLedgerV1 {
    /// Retain the canonical source frame for a computed borrowed actual. This
    /// source loan does not alter its passive phase or close the callee cohort.
    pub(in crate::mir::normal_callable_semantic_package) fn borrowed_expression_actual_source_v1(
        &self,
        call: &OwnedExprSiteV1,
        ordinal: u32,
        site: &SourceExprSiteV1,
    ) -> Result<PreparedBorrowedFormalActualV1, String> {
        let frame = self.qualified_argument_expression_frame_v1(call, ordinal, site)?;
        let rows = self
            .borrowed_formal_actuals
            .get(call)
            .ok_or_else(|| freeze("argument-expression/observation-missing"))?
            .as_ref()
            .map_err(Clone::clone)?;
        let contracts = self
            .original_parameter_contracts
            .as_deref()
            .ok_or_else(|| freeze("argument-expression/source-contracts"))?;
        let identity = argument_source_identity_v1(
            self.borrowed_formal_source
                .as_ref()
                .ok_or_else(|| freeze("argument-expression/source-missing"))?
                .as_ref()
                .map_err(Clone::clone)?,
            Some(contracts),
            rows,
            call,
        )?
        .ok_or_else(|| freeze("argument-expression/source-phase"))?;
        let formal = identity
            .formals
            .get(ordinal as usize)
            .filter(|formal| {
                formal.ordinal == ordinal
                    && matches!(formal.kind, CallableParameterContractKindV1::OpaqueHandle)
            })
            .ok_or_else(|| freeze("borrowed-expression/source-formal"))?;
        Ok(PreparedBorrowedFormalActualV1 {
            ordinal,
            site: site.clone(),
            formal: formal.binding,
            source: BorrowedFormalActualSourceV1::retain_expression_frame_v1(frame)?,
        })
    }

    pub(in crate::mir) fn argument_expression_frame_v1(
        &self,
        call: &OwnedExprSiteV1,
        ordinal: u32,
        site: &SourceExprSiteV1,
    ) -> Result<
        Rc<crate::mir::resolved_semantics::home_new_prefix::CheckedArgumentExpressionFrameV1>,
        String,
    > {
        let frame = self.qualified_argument_expression_frame_v1(call, ordinal, site)?;
        if !frame.is_i64() {
            return Err(freeze("argument-expression/home-membership"));
        }
        Ok(frame)
    }

    fn root_flow_argument_expression_frame_v1(
        &self,
        call: &OwnedExprSiteV1,
        ordinal: u32,
        site: &SourceExprSiteV1,
    ) -> Result<
        Option<
            Rc<crate::mir::resolved_semantics::home_new_prefix::CheckedArgumentExpressionFrameV1>,
        >,
        String,
    > {
        let Some(flow) = self
            .completion_for_owner(call.owner())
            .and_then(|completion| completion.cleanup().root_flow())
        else {
            return Ok(None);
        };
        let mut matches = flow
            .expression_plans()
            .iter()
            .filter_map(|plan| plan.argument_frame())
            .filter(|frame| {
                frame.source().owner() == call.owner()
                    && Rc::ptr_eq(frame.source_loan().plan(), frame.source())
                    && frame.evaluation().reference_coverage_is_exact()
                    && matches!(
                        frame.source().root(),
                        ExpressionPlanRootV1::CallArgument {
                            call: original,
                            ordinal: original_ordinal,
                            ..
                        } if original.owner() == call.owner()
                            && original.site() == call.site()
                            && *original_ordinal == ordinal
                            && original.arguments().get(ordinal as usize).is_some_and(|argument|
                                argument.ordinal() == ordinal && argument.site() == site)
                            && frame.source().nodes().first().is_some_and(|root| root.site() == site)
                    )
            });
        let Some(frame) = matches.next() else {
            return Ok(None);
        };
        if matches.next().is_some() {
            return Err(freeze("argument-expression/root-flow-ambiguous"));
        }
        Ok(Some(Rc::clone(frame)))
    }

    /// Borrow original qualification only. This issues no executable actual,
    /// entry, ABI or publication permission; those keep their original owners.
    pub(in crate::mir) fn qualified_argument_expression_frame_v1(
        &self,
        call: &OwnedExprSiteV1,
        ordinal: u32,
        site: &SourceExprSiteV1,
    ) -> Result<
        Rc<crate::mir::resolved_semantics::home_new_prefix::CheckedArgumentExpressionFrameV1>,
        String,
    > {
        let Some(rows) = self.borrowed_formal_actuals.get(call) else {
            let root = self.root_flow_argument_expression_frame_v1(call, ordinal, site)?;
            let loop_frame = self.source_loop_argument_expression_frame_v2(call, ordinal, site)?;
            let frame = match (root, loop_frame) {
                (Some(root), Some(loop_frame)) if Rc::ptr_eq(&root, &loop_frame) => Some(root),
                (Some(_), Some(_)) => {
                    return Err(freeze("argument-expression/root-flow-ambiguous"))
                }
                (Some(frame), None) | (None, Some(frame)) => Some(frame),
                (None, None) => None,
            };
            return frame.ok_or_else(|| freeze("argument-expression/observation-missing"));
        };
        let prepared = self
            .borrowed_formal_source
            .as_ref()
            .ok_or_else(|| freeze("argument-expression/source-missing"))?
            .as_ref()
            .map_err(Clone::clone)?;
        let rows = rows.as_ref().map_err(Clone::clone)?;
        if rows.dynamic_invocation_arguments.is_some() {
            let original = dynamic_invocation_argument_source_v1(rows, call, ordinal)?
                .ok_or_else(|| freeze("dynamic-argument/source-missing"))?;
            let dynamic = rows
                .dynamic_invocation_arguments
                .as_ref()
                .expect("checked Dynamic argument row");
            let Some(ArgumentExpressionV1::Qualified(frame)) = dynamic.arguments.get(&ordinal)
            else {
                return Err(freeze("dynamic-argument/frame-missing"));
            };
            if frame.source_loan() != &original
                || !Rc::ptr_eq(frame.source(), original.plan())
                || !frame.is_i64()
                || !frame.evaluation().reference_coverage_is_exact()
                || frame
                    .source()
                    .nodes()
                    .first()
                    .is_none_or(|node| node.site() != site)
                || dynamic.source.argument_sites().get(ordinal as usize) != Some(site)
            {
                return Err(freeze("dynamic-argument/frame-identity"));
            }
            return Ok(Rc::clone(frame));
        }
        let contracts = self
            .original_parameter_contracts
            .as_deref()
            .ok_or_else(|| freeze("argument-expression/source-contracts"))?;
        let identity = argument_source_identity_v1(prepared, Some(contracts), rows, call)?
            .ok_or_else(|| freeze("argument-expression/source-phase"))?;
        if let Some(snapshot) = rows.source_static_snapshot_v1() {
            let BorrowedCallActualEvidencePhaseV1::SourceStatic(original) = &snapshot.phase else {
                unreachable!("source snapshot")
            };
            validate_qualified_argument_sources_v1(
                prepared,
                &self.borrowed_formal_actuals,
                call,
                original,
            )?;
        }
        let Some(ArgumentExpressionV1::Qualified(frame)) = identity.slots.get(&ordinal) else {
            return Err(format!(
                "{}: call={call:?}; ordinal={ordinal}; argument={site:?}",
                freeze("argument-expression/frame-missing")
            ));
        };
        let source = lend_argument_expression_source_v1(
            prepared,
            &self.borrowed_formal_actuals,
            contracts,
            call,
            ordinal,
        )?
        .ok_or_else(|| freeze("argument-expression/source-missing"))?;
        if frame.source_loan() != &source
            || !Rc::ptr_eq(frame.source(), source.plan())
            || !frame.evaluation().reference_coverage_is_exact()
            || identity.source.argument_sites().get(ordinal as usize) != Some(site)
            || !self
                .completion_for_owner(call.owner())
                .and_then(|completion| completion.cleanup().root_flow())
                .is_some_and(|flow| {
                    flow.expression_plans()
                        .iter()
                        .filter_map(|plan| plan.argument_frame())
                        .any(|original| Rc::ptr_eq(original, frame))
                })
        {
            return Err(freeze("argument-expression/home-membership"));
        }
        Ok(Rc::clone(frame))
    }
}
