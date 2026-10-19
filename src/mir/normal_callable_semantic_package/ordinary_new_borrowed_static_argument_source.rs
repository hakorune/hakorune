//! Source-purpose connection at the original actual observation. No Home rights.
use super::*;
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::resolved_value_profile::expression_plan::{
    ExpressionPlanRootV1, ExpressionSourcePlanV1,
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
pub(in crate::mir::normal_callable_semantic_package) fn bind_argument_sources_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
    prepared: &PreparedBorrowedFormalIngressV1,
    site: &OwnedExprSiteV1,
    previous: Option<&Result<PreparedBorrowedCallActualsV1, String>>,
    rows: Option<&mut PreparedBorrowedCallActualsV1>,
) -> Result<(), String> {
    let Some(rows) = rows else { return Ok(()) };
    if let BorrowedCallActualEvidencePhaseV1::SourceInstance(identity) = &mut rows.phase {
        let reject = || freeze("borrowed-instance/argument-source-identity");
        let mut originals = prepared.source_incoming.exact_rows().filter(|row| &row.call == site);
        let original = originals.next().ok_or_else(reject)?;
        if originals.next().is_some() || original.source.require_instance()? != &identity.source
            || original.arguments != identity.incoming_arguments { return Err(reject()); }
        let old = previous.map(|row| {
            let row = row.as_ref().map_err(|_| freeze("borrowed-actual/repeated-walk-drift"))?;
            let BorrowedCallActualEvidencePhaseV1::SourceInstance(old) = &row.phase else { return Err(reject()); };
            if !old.same_original_source_v1(identity) { return Err(reject()); }
            Ok((!old.argument_sources.is_empty()).then_some(&old.argument_sources))
        }).transpose()?.flatten();
        let method = input.function().method_call_source_v1(site.site()).ok_or_else(reject)?;
        identity.argument_sources = model::bind_original_argument_plans_v1(input,
            super::super::super::borrowed_formal_uses::BorrowedCallSourceLoanV1::Instance(&identity.source),
            method, &identity.candidates, old)?;
        return Ok(());
    }
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
    for (ordinal, candidate) in identity.candidates.iter().enumerate() {
        let ordinal = ordinal as u32;
        if candidate.ordinal != ordinal { return Err(reject()); }
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
    }
    if identity.core_arguments.keys().any(|ordinal| *ordinal as usize >= identity.candidates.len()) { return Err(reject()); }
    identity.argument_sources = model::bind_original_argument_plans_v1(input,
        super::super::super::borrowed_formal_uses::BorrowedCallSourceLoanV1::Static(&identity.source),
        identity.source.method_source(), &identity.candidates, old.map(|old| &old.argument_sources)).map_err(|_| reject())?;
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
                    && new.argument_sources.values().all(|row| matches!(row, ArgumentExpressionV1::Source(_))),
            (BorrowedCallActualEvidencePhaseV1::SourceInstance(old), BorrowedCallActualEvidencePhaseV1::SourceInstance(new)) =>
                old.same_original_source_v1(new)
                    && (old.argument_sources.is_empty()
                        || old.argument_sources.len() == new.argument_sources.len()
                            && old.argument_sources.iter().all(|(ordinal, plan)| new.argument_sources.get(ordinal)
                                .is_some_and(|other| Rc::ptr_eq(plan, other))))
                    && previous.opaque_actuals == current.opaque_actuals
                    && previous.ordered_arguments == current.ordered_arguments
                    && new.argument_sources.values().all(|row| matches!(row, ArgumentExpressionV1::Source(_))),
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
    i64_result: bool,
    cores: BTreeMap<usize, Rc<crate::mir::normal_callable_semantic_package::core_method_source::OrdinaryCoreSourceCallV1>>,
}
impl PartialEq for ArgumentExpressionSourceLoanV1 {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.plan, &other.plan) && self.i64_result == other.i64_result && self.cores.len() == other.cores.len()
            && self.cores.iter().all(|(node, core)| other.cores.get(node).is_some_and(|other| Rc::ptr_eq(core, other)))
    }
}
impl Eq for ArgumentExpressionSourceLoanV1 {}
impl ArgumentExpressionSourceLoanV1 {
    pub(crate) fn requires_i64_v1(&self) -> bool { self.i64_result }
    pub(crate) fn plan(&self) -> &Rc<ExpressionSourcePlanV1> { &self.plan }
    /// SAME closed operand premise at an exact original node. This is source
    /// class evidence only and must be joined with current Prefix availability.
    pub(crate) fn signed_operand_contains(&self, site: &crate::mir::resolved_semantics::SourceExprSiteV1) -> bool {
        self.i64_result && self.plan.nodes().iter().any(|node| node.site() == site)
            || self.cores.values().any(|receipt| { let core = receipt.source(); self.plan.nodes().iter().any(|node| node.site() == site)
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
        }) || !self.i64_result && self.resolver_source().is_none() && self.plan.nodes().first().is_some_and(|node| node.site() == site
            && matches!(node.source(), crate::mir::resolved_value_profile::expression_plan::ExpressionNodeSourceV1::Reference(_)))
    }

    pub(crate) fn resolver_source(&self) -> Option<&crate::mir::resolved_semantics::VerifiedResolverOrdinaryCoreMethodSourceV1> {
        self.cores.get(&0).map(|core| core.source())
    }
}

/// Shared by the passive projection and source lender. In particular this
/// checks the complete original tuple, not just the requested ordinal.
pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn validate_static_source_actuals_v1<'a>(
    prepared: &PreparedBorrowedFormalIngressV1,
    rows: &'a PreparedBorrowedCallActualsV1,
    site: &OwnedExprSiteV1,
    required_i64_arguments: &[u32],
) -> Result<&'a StaticSourceActualIdentityV1, String> {
    let rows = rows.source_static_snapshot_v1().ok_or_else(|| freeze("borrowed-static/source-phase-required"))?;
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
            (BorrowedCallActualValueV1::Expression, Node::Binary(_)) if identity.integer_evidence.get(index) == Some(&false) => {}
            (BorrowedCallActualValueV1::Expression, _) => return Err(freeze("borrowed-static/argument-source-identity")),
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
                    || !match (&formal.kind, receipt.source().target().result()) {
                        (kind, crate::mir::resolved_semantics::CoreMethodHomeResultRelationV1::TextToCaller) => kind.is_ordinary_borrowed_handle(),
                        (CallableParameterContractKindV1::ExactTrivial(abi), crate::mir::resolved_semantics::CoreMethodHomeResultRelationV1::I64ToCaller) => abi.is_i64(),
                        _ => false,
                    }
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
            (
                CallableParameterContractKindV1::ExactTrivial(abi),
                BorrowedCallActualValueV1::Expression | BorrowedCallActualValueV1::MethodCall(_),
                LocalCallArgumentV1::Expression { ordinal, site: original },
            ) if abi.is_i64() => *ordinal == formal.ordinal && original == &candidate.site,
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

/// During Home scanning this lends only already-qualified source arguments.
/// Accepted Home membership and executable payload belong to later activation.
pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn validate_qualified_argument_sources_v1(
    prepared: &PreparedBorrowedFormalIngressV1, pending: &PendingBorrowedFormalActualsV1,
    site: &OwnedExprSiteV1, identity: &StaticSourceActualIdentityV1,
) -> Result<(), String> {
    use crate::mir::resolved_value_profile::{expression_class::ExpressionValueClassV1 as Class,
        expression_projection::ExpressionLeafClassV1 as Leaf};
    for actual in &identity.candidates {
        let core = identity.core_arguments.contains_key(&actual.ordinal);
        let expression = core || matches!(actual.value,
            BorrowedCallActualValueV1::Expression | BorrowedCallActualValueV1::String(_) | BorrowedCallActualValueV1::Binding(_))
            || matches!(actual.value, BorrowedCallActualValueV1::SelfRooted { .. })
                && !prepared.static_arguments.contains_key(&(site.clone(), actual.ordinal));
        if !expression { continue; }
        let slot = identity.argument_sources.get(&actual.ordinal)
            .ok_or_else(|| freeze("borrowed-static/core-source-identity"))?;
        let ArgumentExpressionV1::Qualified(frame) = slot else {
            return Err(freeze(if core { "borrowed-static/core-expression-execution-pending" }
                else { "borrowed-static/text-expression-execution-pending" }));
        };
        let original = lend_argument_expression_source_v1(prepared, pending, site, actual.ordinal)?
            .ok_or_else(|| freeze("borrowed-static/argument-frame-identity"))?;
        if !Rc::ptr_eq(frame.source(), slot) || frame.source_loan() != &original
            || !frame.evaluation().reference_coverage_is_exact()
            || frame.evaluation().root_class() != Leaf::Value(if original.requires_i64_v1() { Class::I64 } else { Class::String }) {
            return Err(freeze("borrowed-static/argument-frame-identity"));
        }
    }
    Ok(())
}

pub(in crate::mir::normal_callable_semantic_package) fn lend_argument_expression_source_v1(
    prepared: &PreparedBorrowedFormalIngressV1,
    pending: &PendingBorrowedFormalActualsV1,
    site: &OwnedExprSiteV1,
    ordinal: u32,
) -> Result<Option<ArgumentExpressionSourceLoanV1>, String> {
    let rows = pending.get(site).ok_or_else(|| freeze("borrowed-static/source-unobserved"))?
        .as_ref().map_err(Clone::clone)?;
    let Some(rows) = rows.source_static_snapshot_v1() else { return Ok(None) };
    let BorrowedCallActualEvidencePhaseV1::SourceStatic(identity) = &rows.phase else { unreachable!("source snapshot") };
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
    let formal = identity.source.parameters().get(ordinal as usize).ok_or_else(|| freeze("borrowed-static/argument-formal"))?;
    let i64_result = matches!(&formal.kind, CallableParameterContractKindV1::ExactTrivial(abi) if abi.is_i64());
    Ok(Some(ArgumentExpressionSourceLoanV1 { plan: Rc::clone(plan), i64_result, cores }))
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
    let Some(original) = original_static_source_v1(prepared, site)? else { return Ok(None) };
    let Some(formal) = original.parameters().get(ordinal as usize).filter(|formal| formal.ordinal == ordinal) else { return Ok(None) };
    if matches!(&formal.kind, CallableParameterContractKindV1::ExactTrivial(abi) if abi.is_i64()) {
        let eligible = match crate::mir::resolved_semantics::home_new_prefix::borrowed_actual_source_atom_v1(input, arg.site()) {
            Some(BorrowedCallActualValueV1::Expression) => true,
            Some(BorrowedCallActualValueV1::MethodCall(method)) => prepared.ordinary_core_source_v1(
                &OwnedExprSiteV1::new(input.owner(), arg.site().clone())).is_some_and(|core|
                    std::sync::Arc::ptr_eq(&method, core.source().source())
                        && core.source().target().result() == crate::mir::resolved_semantics::CoreMethodHomeResultRelationV1::I64ToCaller),
            _ => false,
        };
        if !eligible { return Ok(None); }
        return lend_argument_expression_source_v1(prepared, pending, site, ordinal);
    }
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
        formal.ordinal == ordinal && matches!(formal.kind, CallableParameterContractKindV1::OpaqueHandle)) { return Ok(None); }
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
        ArgumentExpressionV1::Qualified(previous) => {
            if previous.as_ref() != frame.as_ref() { return Err(freeze("borrowed-static/argument-frame-current-drift")); }
            Ok(Rc::clone(previous))
        }
        ArgumentExpressionV1::Source(plan) => {
            if !Rc::ptr_eq(plan, frame.source()) { return Err(reject()); }
            *slot = ArgumentExpressionV1::Qualified(Rc::clone(frame));
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

impl OrdinaryNewClaimLedgerV1 {
    /// Retain the canonical source frame for a computed borrowed actual. This
    /// source loan does not alter its passive phase or close the callee cohort.
    pub(in crate::mir::normal_callable_semantic_package) fn borrowed_expression_actual_source_v1(
        &self, call: &OwnedExprSiteV1, ordinal: u32, site: &SourceExprSiteV1,
    ) -> Result<PreparedBorrowedFormalActualV1, String> {
        let frame = self.qualified_argument_expression_frame_v1(call, ordinal, site)?;
        let rows = self.borrowed_formal_actuals.get(call)
            .ok_or_else(|| freeze("argument-expression/observation-missing"))?.as_ref().map_err(Clone::clone)?;
        let rows = rows.source_static_snapshot_v1().ok_or_else(|| freeze("argument-expression/source-phase"))?;
        let BorrowedCallActualEvidencePhaseV1::SourceStatic(identity) = &rows.phase
            else { return Err(freeze("argument-expression/source-phase")) };
        let formal = identity.source.parameters().get(ordinal as usize)
            .filter(|formal| formal.ordinal == ordinal && formal.kind.is_ordinary_borrowed_handle())
            .ok_or_else(|| freeze("borrowed-expression/source-formal"))?;
        Ok(PreparedBorrowedFormalActualV1 {
            ordinal, site: site.clone(), formal: formal.binding,
            source: BorrowedFormalActualSourceV1::retain_expression_frame_v1(frame)?,
        })
    }

    pub(in crate::mir) fn argument_expression_frame_v1(&self, call: &OwnedExprSiteV1, ordinal: u32,
        site: &SourceExprSiteV1,
    ) -> Result<Rc<crate::mir::resolved_semantics::home_new_prefix::CheckedArgumentExpressionFrameV1>, String> {
        let frame = self.qualified_argument_expression_frame_v1(call, ordinal, site)?;
        if !frame.is_i64() { return Err(freeze("argument-expression/home-membership")); }
        Ok(frame)
    }

    /// Borrow original qualification only. This issues no executable actual,
    /// entry, ABI or publication permission; those keep their original owners.
    pub(in crate::mir) fn qualified_argument_expression_frame_v1(&self, call: &OwnedExprSiteV1, ordinal: u32,
        site: &SourceExprSiteV1,
    ) -> Result<Rc<crate::mir::resolved_semantics::home_new_prefix::CheckedArgumentExpressionFrameV1>, String> {
        let prepared = self.borrowed_formal_source.as_ref().ok_or_else(|| freeze("argument-expression/source-missing"))?
            .as_ref().map_err(Clone::clone)?;
        let rows = self.borrowed_formal_actuals.get(call).ok_or_else(|| freeze("argument-expression/observation-missing"))?
            .as_ref().map_err(Clone::clone)?;
        let rows = rows.source_static_snapshot_v1().ok_or_else(|| freeze("argument-expression/source-phase"))?;
        let BorrowedCallActualEvidencePhaseV1::SourceStatic(identity) = &rows.phase else { return Err(freeze("argument-expression/source-phase")) };
        let identity = validate_static_source_actuals_v1(prepared, rows, call, identity.source.required_i64_arguments())?;
        validate_qualified_argument_sources_v1(prepared, &self.borrowed_formal_actuals, call, identity)?;
        let Some(ArgumentExpressionV1::Qualified(frame)) = identity.argument_sources.get(&ordinal) else {
            return Err(freeze("argument-expression/frame-missing"));
        };
        if identity.source.argument_sites().get(ordinal as usize) != Some(site)
            || !self.completion_for_owner(call.owner()).and_then(|completion| completion.cleanup().root_flow())
                .is_some_and(|flow| flow.expression_plans().iter().filter_map(|plan| plan.argument_frame())
                    .any(|original| Rc::ptr_eq(original, frame))) {
            return Err(freeze("argument-expression/home-membership"));
        }
        Ok(Rc::clone(frame))
    }
}
