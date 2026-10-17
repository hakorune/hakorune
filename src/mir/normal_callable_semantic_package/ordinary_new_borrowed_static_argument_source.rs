//! Source-purpose connection at the original actual observation. No Home rights.
use super::*;
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::resolved_value_profile::expression_plan::{
    call_argument_expression_source_plan_v1, ExpressionPlanRootV1, ExpressionSourcePlanV1,
};

/// Original outer Call and ordinal identity, independent of the value's class.
fn corroborates_argument_plan_v1(
    plan: &ExpressionSourcePlanV1, source: &StaticIncomingSourceV1,
    ordinal: u32, site: &SourceExprSiteV1,
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
    corroborates_argument_plan_v1(plan, source, ordinal, method.site()) && method.owner() == plan.owner()
        && plan.nodes().first().is_some_and(|node|
            matches!(node.source(), crate::mir::resolved_value_profile::expression_plan::ExpressionNodeSourceV1::MethodCall))
}

/// Re-observation borrows every original argument plan Rc. This is source
/// coverage only: literal, local and Core inputs share no execution shortcut.
pub(in crate::mir::normal_callable_semantic_package) fn bind_static_argument_sources_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
    prepared: &PreparedBorrowedFormalIngressV1,
    site: &OwnedExprSiteV1,
    previous: Option<&Result<PreparedBorrowedCallActualsV1, String>>,
    rows: Option<&mut PreparedBorrowedCallActualsV1>,
) -> Result<(), String> {
    let Some(rows) = rows else { return Ok(()) };
    let BorrowedCallActualEvidencePhaseV1::SourceStatic(identity) = &mut rows.phase else { return Ok(()) };
    let reject = || freeze("borrowed-static/core-source-identity");
    if !original_static_source_v1(prepared, site)?.is_some_and(|source| Rc::ptr_eq(source, &identity.source)) { return Err(reject()); }
    let old = previous.map(|rows| {
        let rows = rows.as_ref().map_err(|_| freeze("borrowed-actual/repeated-walk-drift"))?;
        let BorrowedCallActualEvidencePhaseV1::SourceStatic(old) = &rows.phase else { return Err(reject()) };
        if !Rc::ptr_eq(&old.source, &identity.source) || old.candidates != identity.candidates
            || old.integer_evidence != identity.integer_evidence
            || old.core_arguments.len() != identity.core_arguments.len()
            || old.argument_sources.len() != identity.candidates.len()
        { return Err(reject()); }
        Ok(old)
    }).transpose()?;
    if input.owner() != site.owner() || identity.source.call_site() != site
        || identity.candidates.len() != identity.source.argument_sites().len()
        || !input.function().method_call_source_v1(site.site()).is_some_and(|call|
            std::sync::Arc::ptr_eq(call, identity.source.method_source()))
    { return Err(reject()); }
    let mut plans = BTreeMap::new();
    for (ordinal, candidate) in identity.candidates.iter().enumerate() {
        let ordinal = ordinal as u32;
        if candidate.ordinal != ordinal { return Err(reject()); }
        // The binder checks spelling and binding identity, never the local's
        // class, current value or ownership. Those remain separate demands.
        let atom = crate::mir::resolved_semantics::home_new_prefix::borrowed_actual_source_atom_v1(input, &candidate.site);
        let same = match (&candidate.value, atom.as_ref()) {
            (BorrowedCallActualValueV1::Scalar(binding, _) | BorrowedCallActualValueV1::SelfRooted { binding, .. },
                Some(BorrowedCallActualValueV1::Binding(original))) => binding == original,
            (BorrowedCallActualValueV1::MethodCall(method), Some(BorrowedCallActualValueV1::MethodCall(original))) => std::sync::Arc::ptr_eq(method, original),
            (expected, Some(original)) => expected == original,
            _ => false,
        };
        if !same { return Err(reject()); }
        if let BorrowedCallActualValueV1::SelfRooted { binding, root } = candidate.value {
            if !prepared.static_arguments.contains_key(&(site.clone(), ordinal)) {
                let flow = input.function().expression_source().value_flow().ok_or_else(reject)?;
                if flow.read(&candidate.site, binding).and_then(|value|
                    flow.alias_entry_origin(value, |site| input.function().variable_ref(site))) != Some(root) {
                    return Err(reject());
                }
            }
        }
        if let BorrowedCallActualValueV1::MethodCall(method) = &candidate.value {
            let receipt = identity.core_arguments.get(&ordinal).ok_or_else(reject)?;
            if !std::sync::Arc::ptr_eq(method, receipt.source().source())
                || !input.function().method_call_source_v1(method.site()).is_some_and(|call| std::sync::Arc::ptr_eq(call, method))
                || old.is_some_and(|old| !old.core_arguments.get(&ordinal).is_some_and(|original| Rc::ptr_eq(original, receipt)))
            { return Err(reject()); }
        } else if identity.core_arguments.contains_key(&ordinal) { return Err(reject()); }
        let plan = if let Some(old) = old {
            StaticArgumentExpressionV1::Source(Rc::clone(old.argument_sources.get(&ordinal).ok_or_else(reject)?))
        } else {
            call_argument_expression_source_plan_v1(input, identity.source.method_source(), ordinal).map_err(|_| reject())?.into()
        };
        if !corroborates_argument_plan_v1(&plan, &identity.source, candidate.ordinal, &candidate.site)
            || input.function().source_site_inventory().containing_statement(site.site()) != Some(plan.root().statement())
        { return Err(reject()); }
        plans.insert(ordinal, plan);
    }
    if identity.core_arguments.keys().any(|ordinal| *ordinal as usize >= identity.candidates.len()) { return Err(reject()); }
    identity.argument_sources = plans;
    Ok(())
}

/// Only explicit source Observe can invalidate a previous Home walk's current
/// qualification. Generic staging remains strict; executable phases never reset.
pub(in crate::mir::normal_callable_semantic_package) fn stage_observed_call_actuals_v1(
    staged: &mut PendingBorrowedFormalActualsV1, site: &OwnedExprSiteV1,
    result: Result<Option<PreparedBorrowedCallActualsV1>, String>,
) {
    let reset = match (staged.get(site), &result) {
        (Some(Ok(previous)), Ok(Some(current))) => match (&previous.phase, &current.phase) {
            (BorrowedCallActualEvidencePhaseV1::SourceStatic(old), BorrowedCallActualEvidencePhaseV1::SourceStatic(new)) =>
                old.same_original_source_v1(new)
                    && previous.opaque_actuals == current.opaque_actuals
                    && previous.ordered_arguments == current.ordered_arguments
                    && new.argument_sources.values().all(|row| matches!(row, StaticArgumentExpressionV1::Source(_))),
            _ => false,
        },
        _ => false,
    };
    if reset {
        staged.insert(site.clone(), Ok(result.expect("checked observation").expect("checked source tuple")));
    } else {
        super::super::stage_borrowed_call_actuals_v1(staged, site, result);
    }
}

/// One original source loan, never an evaluation, entry or ownership grant.
/// Rc identity is retained across the callback; no source plan is reissued.
#[derive(Debug, Clone)]
pub(crate) struct ArgumentExpressionSourceLoanV1 {
    plan: Rc<ExpressionSourcePlanV1>,
    cores: BTreeMap<usize, Rc<crate::mir::normal_callable_semantic_package::core_method_source::OrdinaryCoreSourceCallV1>>,
}
impl PartialEq for ArgumentExpressionSourceLoanV1 {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.plan, &other.plan) && self.cores.len() == other.cores.len()
            && self.cores.iter().all(|(node, core)| other.cores.get(node).is_some_and(|other| Rc::ptr_eq(core, other)))
    }
}
impl Eq for ArgumentExpressionSourceLoanV1 {}
impl ArgumentExpressionSourceLoanV1 {
    pub(crate) fn plan(&self) -> &Rc<ExpressionSourcePlanV1> { &self.plan }
    /// SAME closed operand premise at an exact original node. This is source
    /// class evidence only and must be joined with current Prefix availability.
    pub(crate) fn signed_operand_contains(&self, site: &crate::mir::resolved_semantics::SourceExprSiteV1) -> bool {
        self.cores.values().any(|receipt| { let core = receipt.source(); self.plan.nodes().iter().any(|node| node.site() == site)
            && core.source().arguments().iter().zip(core.target().parameters())
                .any(|(arg, kind)| *kind == crate::mir::resolved_semantics::CoreMethodHomeParameterRelationV1::I64Parameter
                    && site.node().segments().starts_with(arg.site().node().segments())) })
    }
    pub(crate) fn text_operand_is(&self, site: &crate::mir::resolved_semantics::SourceExprSiteV1) -> bool {
        self.cores.values().any(|receipt| {
            let core = receipt.source();
            site == core.source().receiver_site()
                || core.source().arguments().iter().zip(core.target().parameters())
                    .any(|(arg, kind)| *kind == crate::mir::resolved_semantics::CoreMethodHomeParameterRelationV1::TextParameter && arg.site() == site)
        }) || self.resolver_source().is_none() && self.plan.nodes().first().is_some_and(|node| node.site() == site
            && matches!(node.source(), crate::mir::resolved_value_profile::expression_plan::ExpressionNodeSourceV1::Reference(_)))
    }

    pub(crate) fn resolver_source(&self) -> Option<&crate::mir::resolved_semantics::VerifiedResolverOrdinaryCoreMethodSourceV1> {
        self.cores.get(&0).map(|core| core.source())
    }
}

/// Shared by the passive projection and source lender. In particular this
/// checks the complete original tuple, not just the requested ordinal.
pub(super) fn validate_static_source_actuals_v1<'a>(
    prepared: &PreparedBorrowedFormalIngressV1,
    rows: &'a PreparedBorrowedCallActualsV1,
    site: &OwnedExprSiteV1,
    required_i64_arguments: &[u32],
) -> Result<&'a StaticSourceActualIdentityV1, String> {
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
        let plan = identity.argument_sources.get(&candidate.ordinal)
            .filter(|plan| corroborates_argument_plan_v1(plan, &identity.source, candidate.ordinal, &candidate.site))
            .ok_or_else(|| freeze("borrowed-static/core-source-identity"))?;
        use crate::mir::resolved_value_profile::expression_plan::ExpressionNodeSourceV1 as Node;
        use crate::mir::resolved_semantics::{ResolvedLiteralSourceV1 as Literal, ResolvedLexicalRefV1 as Ref};
        match (&candidate.value, plan.nodes()[0].source()) {
            (BorrowedCallActualValueV1::String(value), Node::Literal(Literal::String(original))) if value == original => {}
            (BorrowedCallActualValueV1::String(_), _) => return Err(freeze("borrowed-static/argument-source-identity")),
            (BorrowedCallActualValueV1::Binding(binding), Node::Reference(Ref::Local(original))) if binding == original => {}
            (BorrowedCallActualValueV1::Binding(_), _) => return Err(freeze("borrowed-static/argument-source-identity")),
            _ => {}
        }
        match (&candidate.value, identity.core_arguments.get(&candidate.ordinal)) {
            (BorrowedCallActualValueV1::MethodCall(method), Some(receipt)) => {
                let site = OwnedExprSiteV1::new(site.owner(), candidate.site.clone());
                if !identity.argument_sources.get(&candidate.ordinal).is_some_and(|plan|
                    corroborates_argument_source_v1(plan, identity.source.as_ref(), candidate.ordinal, method))
                    || !prepared.ordinary_core_source_v1(&site).is_some_and(|original| Rc::ptr_eq(original, receipt))
                    || !std::sync::Arc::ptr_eq(method, receipt.source().source())
                    || method.owner() != site.owner() || method.site() != site.site()
                    || identity.integer_evidence.get(index) != Some(&false)
                    || receipt.source().target().schema() != crate::mir::resolved_semantics::CoreMethodHomeSchemaV1::StringBoxText
                    || receipt.source().target().result() != crate::mir::resolved_semantics::CoreMethodHomeResultRelationV1::TextToCaller
                { return Err(freeze("borrowed-static/core-source-identity")); }
            }
            (BorrowedCallActualValueV1::MethodCall(_), None) | (_, Some(_)) => return Err(freeze("borrowed-static/core-source-identity")),
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
            _ => false,
        };
        if !matches {
            return Err(freeze("borrowed-static/source-projection-identity"));
        }
    }
    if identity.argument_sources.len() != identity.candidates.len()
        || identity.argument_sources.keys().any(|ordinal| *ordinal as usize >= identity.candidates.len())
        || identity.core_arguments.keys().any(|ordinal| *ordinal as usize >= identity.candidates.len()) {
        return Err(freeze("borrowed-static/core-source-identity"));
    }
    Ok(identity)
}

pub(in crate::mir::normal_callable_semantic_package) fn lend_argument_expression_source_v1(
    prepared: &PreparedBorrowedFormalIngressV1,
    pending: &PendingBorrowedFormalActualsV1,
    site: &OwnedExprSiteV1,
    ordinal: u32,
) -> Result<Option<ArgumentExpressionSourceLoanV1>, String> {
    let rows = pending.get(site).ok_or_else(|| freeze("borrowed-static/source-unobserved"))?
        .as_ref().map_err(Clone::clone)?;
    let BorrowedCallActualEvidencePhaseV1::SourceStatic(identity) = &rows.phase else { return Ok(None) };
    let identity = validate_static_source_actuals_v1(prepared, rows, site, identity.source.required_i64_arguments())?;
    let candidate = identity.candidates.get(ordinal as usize)
        .filter(|row| row.ordinal == ordinal).ok_or_else(|| freeze("borrowed-static/core-source-identity"))?;
    let plan = identity.argument_sources.get(&ordinal).ok_or_else(|| freeze("borrowed-static/core-source-identity"))?;
    let mut cores = BTreeMap::new();
    let mut call_inputs: Vec<&SourceExprSiteV1> = Vec::new();
    for (node, original) in plan.nodes().iter().enumerate() {
        if call_inputs.iter().any(|call| original.site().node().segments().starts_with(call.node().segments())) { continue; }
        if !matches!(original.source(), crate::mir::resolved_value_profile::expression_plan::ExpressionNodeSourceV1::MethodCall) { continue; }
        let site = OwnedExprSiteV1::new(plan.owner(), original.site().clone());
        if let Some(core) = prepared.ordinary_core_source_v1(&site) { cores.insert(node, Rc::clone(core)); }
        else { call_inputs.push(original.site()); }
    }
    if matches!(candidate.value, BorrowedCallActualValueV1::MethodCall(_))
        && !cores.get(&0).is_some_and(|core| identity.core_arguments.get(&ordinal).is_some_and(|root| Rc::ptr_eq(core, root))) {
        return Err(freeze("borrowed-static/core-source-identity"));
    }
    Ok(Some(ArgumentExpressionSourceLoanV1 { plan: Rc::clone(plan), cores }))
}

/// The covered observer selects original Text source roles in the same pending
/// argument product. Core results and local/reference reads keep distinct proof.
pub(in crate::mir::normal_callable_semantic_package) fn observed_argument_source_v1(
    input: ResolvedFunctionLoweringInputV1<'_>, prepared: &PreparedBorrowedFormalIngressV1,
    pending: &PendingBorrowedFormalActualsV1, site: &OwnedExprSiteV1, ordinal: u32,
) -> Result<Option<ArgumentExpressionSourceLoanV1>, String> {
    if input.owner() != site.owner() { return Err(freeze("borrowed-static/core-source-identity")); }
    let Some(call) = input.function().method_call_source_v1(site.site()) else { return Ok(None) };
    let Some(arg) = call.arguments().get(ordinal as usize).filter(|arg| arg.ordinal() == ordinal) else { return Ok(None) };
    if let Some(core) = prepared.ordinary_core_source_v1(&OwnedExprSiteV1::new(site.owner(), arg.site().clone())) {
        if core.source().target().result() != crate::mir::resolved_semantics::CoreMethodHomeResultRelationV1::TextToCaller { return Ok(None); }
        if !input.function().method_call_source_v1(arg.site()).is_some_and(|method| std::sync::Arc::ptr_eq(method, core.source().source())) {
            return Err(freeze("borrowed-static/core-source-identity"));
        }
    } else if !matches!(crate::mir::resolved_semantics::home_new_prefix::borrowed_actual_source_atom_v1(input, arg.site()),
        Some(BorrowedCallActualValueV1::String(_) | BorrowedCallActualValueV1::Binding(_))) {
        return Ok(None);
    }
    if let Some(crate::mir::resolved_semantics::ResolvedLexicalRefV1::Local(binding)) = input.function().variable_ref(arg.site()) {
        if let Some(flow) = input.function().expression_source().value_flow() {
            if let Some(root) = flow.read(arg.site(), binding).and_then(|value|
                flow.alias_entry_origin(value, |site| input.function().variable_ref(site))) {
                if !prepared.source_value_sum_v1(root).is_some_and(|sum| sum.alternatives().len() == 1
                    && sum.alternatives().contains(&super::super::super::borrowed_formal_source::SourceValueAlternativeV1::Text)) { return Ok(None); }
            }
        }
    }
    // Only canonical Static source participates. Preserve a selected original
    // refusal; do not turn an Err into an absent loan and invite another route.
    let Some(original) = original_static_source_v1(prepared, site)? else { return Ok(None) };
    if !original.parameters().get(ordinal as usize).is_some_and(|formal|
        formal.ordinal == ordinal && formal.kind.is_ordinary_borrowed_handle()) { return Ok(None); }
    if let Some(rows) = pending.get(site) {
        let rows = rows.as_ref().map_err(Clone::clone)?;
        if !matches!(rows.phase, BorrowedCallActualEvidencePhaseV1::SourceStatic(_)) { return Ok(None); }
    }
    lend_argument_expression_source_v1(prepared, pending, site, ordinal)
}

pub(in crate::mir::normal_callable_semantic_package) fn lend_text_entry_read_v1(
    prepared: &PreparedBorrowedFormalIngressV1, pending: &PendingBorrowedFormalActualsV1,
    contracts: &[OwnedCallableParameterContractDeclarationV1], site: &OwnedExprSiteV1,
    loan: &ArgumentExpressionSourceLoanV1, read: &crate::mir::resolved_semantics::SourceExprSiteV1,
    binding: BindingRefV1, root: BindingRefV1,
) -> Result<crate::mir::normal_callable_semantic_package::BorrowedTextEntryReadRequirementV1, String> {
    let ExpressionPlanRootV1::CallArgument { ordinal, call, .. } = loan.plan().root() else {
        return Err(freeze("borrowed-text-entry/source-identity"));
    };
    if !loan.text_operand_is(read) || call.site() != site.site() || loan.plan().owner() != site.owner()
        || lend_argument_expression_source_v1(prepared, pending, site, *ordinal)?.as_ref() != Some(loan) {
        return Err(freeze("borrowed-text-entry/source-identity"));
    }
    if loan.resolver_source().is_none() && !prepared.source_value_sum_v1(root).is_some_and(|sum|
        sum.alternatives().len() == 1 && sum.alternatives().contains(&super::super::super::borrowed_formal_source::SourceValueAlternativeV1::Text)) {
        return Err(freeze("borrowed-text-entry/source-domain-unclosed"));
    }
    super::super::super::borrowed_formal_entry::issue_text_entry_read_requirement_v1(contracts, loan.plan(), read, binding, root,
        prepared.original_formal_read_v1(site.owner(), read, binding, root)?)
}

/// Retain qualification in the SAME pending ordinal as its original source.
/// Repeated completion within one observation reuses the exact current frame.
pub(in crate::mir::normal_callable_semantic_package) fn complete_argument_expression_v1(
    prepared: &PreparedBorrowedFormalIngressV1, pending: &mut PendingBorrowedFormalActualsV1,
    site: &OwnedExprSiteV1,
    frame: &Rc<crate::mir::resolved_semantics::home_new_prefix::CheckedArgumentExpressionFrameV1>,
) -> Result<Rc<crate::mir::resolved_semantics::home_new_prefix::CheckedArgumentExpressionFrameV1>, String> {
    let reject = || freeze("borrowed-static/argument-frame-identity");
    let ExpressionPlanRootV1::CallArgument { call, ordinal, .. } = frame.source().root() else { return Err(reject()); };
    if call.owner() != site.owner() || call.site() != site.site()
        || frame.source().owner() != site.owner() || !frame.evaluation().reference_coverage_is_exact() {
        return Err(reject());
    }
    let original = lend_argument_expression_source_v1(prepared, pending, site, *ordinal)?.ok_or_else(reject)?;
    if frame.source_loan() != &original || !Rc::ptr_eq(frame.source(), original.plan()) {
        return Err(reject());
    }
    let rows = pending.get_mut(site).ok_or_else(reject)?.as_mut().map_err(|error| error.clone())?;
    let BorrowedCallActualEvidencePhaseV1::SourceStatic(identity) = &mut rows.phase else { return Err(reject()); };
    let slot = identity.argument_sources.get_mut(ordinal).ok_or_else(reject)?;
    match slot {
        StaticArgumentExpressionV1::Qualified(previous) => {
            if previous.as_ref() != frame.as_ref() { return Err(freeze("borrowed-static/argument-frame-current-drift")); }
            Ok(Rc::clone(previous))
        }
        StaticArgumentExpressionV1::Source(plan) => {
            if !Rc::ptr_eq(plan, frame.source()) { return Err(reject()); }
            *slot = StaticArgumentExpressionV1::Qualified(Rc::clone(frame));
            Ok(Rc::clone(frame))
        }
    }
}


pub(in crate::mir::normal_callable_semantic_package) fn lend_expression_core_source_v1(
    input: ResolvedFunctionLoweringInputV1<'_>, prepared: &PreparedBorrowedFormalIngressV1,
    plan: &Rc<ExpressionSourcePlanV1>, site: &OwnedExprSiteV1,
) -> Result<Option<crate::mir::normal_callable_semantic_package::ExpressionCoreSourceLoanV1>, String> {
    if site.owner() != input.owner() || plan.owner() != input.owner() { return Err(freeze("expression-core/source-identity")); }
    let Some(core) = prepared.ordinary_core_source_v1(site) else { return Ok(None) };
    let node = plan.nodes().iter().position(|node| node.site() == site.site())
        .ok_or_else(|| freeze("expression-core/source-identity"))?;
    crate::mir::normal_callable_semantic_package::ExpressionCoreSourceLoanV1::lend(input, plan, node, core).map(Some)
}
