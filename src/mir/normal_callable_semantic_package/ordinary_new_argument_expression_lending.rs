//! Original argument source loans and purpose-specific reads.
use super::*;

pub(in crate::mir::normal_callable_semantic_package) fn lend_argument_expression_source_v1(
    prepared: &PreparedBorrowedFormalIngressV1,
    pending: &PendingBorrowedFormalActualsV1,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    site: &OwnedExprSiteV1,
    ordinal: u32,
) -> Result<Option<ArgumentExpressionSourceLoanV1>, String> {
    lend_argument_source_inner_v1(prepared, pending, Some(contracts), site, ordinal)
}
pub(super) fn lend_argument_source_inner_v1(
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
            if let Some(source) = original_argument_call_source_v1(prepared, &site)? {
                calls.insert(node, ArgumentMethodSourceV1::OrdinaryCall(source));
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
            ArgumentMethodSourceV1::OrdinaryCall(row) => Some(row.method_source()),
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
                | BorrowedCallActualValueV1::MethodCall(_)
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

pub(super) fn dynamic_invocation_argument_source_v1(
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
                        | BorrowedCallActualValueV1::MethodCall(_)
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
                if ordinary_argument_call_source_v1(
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
                    && sum.alternatives().contains(&super::super::super::super::borrowed_formal_source::SourceValueAlternativeV1::Text)) { return Ok(None); }
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
                    &super::super::super::super::borrowed_formal_source::SourceValueAlternativeV1::Text,
                )
        })
    {
        return Err(freeze("borrowed-text-entry/source-domain-unclosed"));
    }
    super::super::super::super::borrowed_formal_entry::issue_text_entry_read_requirement_v1(
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
                    &super::super::super::super::borrowed_formal_source::SourceValueAlternativeV1::Text,
                )
        })
    {
        return Err(reject());
    }
    super::super::super::super::borrowed_formal_entry::issue_text_entry_read_requirement_v1(
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
