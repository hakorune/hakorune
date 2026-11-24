//! Conditional named-Array Homes in the same source walk. No storage, Normal
//! producer, commit, borrowed value or End execution capability is issued here.
use super::*;
use crate::mir::normal_callable_semantic_package::NamedArrayHolderSourceLoanV1;
use crate::mir::resolved_semantics::home_new_prefix::LiveHomeV1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum NamedArrayHomeObservationV1 {
    FieldTextAppend(std::rc::Rc<super::CheckedFieldArrayTextAppendV1>),
    FieldIntegerAppend {
        loop_site: SourceStmtSiteV1,
        call: OwnedExprSiteV1,
        requirement: std::rc::Rc<crate::mir::resolved_semantics::NamedArrayFieldResidenceRequirementV1>,
        evaluation: std::rc::Rc<super::CheckedExpressionEvaluationV1>,
    },
    Initializer {
        source: NamedArrayHolderSourceLoanV1,
        prefix: Result<CallerNewHomePrefixV1, HomePrefixUnavailableV1>,
    },
    Append {
        source: NamedArrayHolderSourceLoanV1,
        call: OwnedExprSiteV1,
        candidate: OwnedExprSiteV1,
        availability: Result<(), HomePrefixUnavailableV1>,
        prior_homes: Box<[LiveHomeV1]>,
        covered_statements: Box<[SourceStmtSiteV1]>,
    },
}

impl NamedArrayHomeObservationV1 {
    pub(crate) fn append_call_v1(&self) -> Option<&OwnedExprSiteV1> {
        match self {
            Self::Append { call, .. } | Self::FieldIntegerAppend { call, .. } => Some(call),
            _ => None,
        }
    }

}

pub(super) fn observe_initializer<E>(
    input: ResolvedFunctionLoweringInputV1<'_>,
    site: &OwnedExprSiteV1,
    binding: BindingRefV1,
    locals: &mut PrefixLocalFlow<'_>,
    homes: &mut HomesV1,
    covered: &[SourceStmtSiteV1],
    unavailable: &mut Option<HomePrefixUnavailableV1>,
    borrow: &mut impl FnMut(
        &OwnedExprSiteV1,
        BorrowedCallActualRequestV1<'_>,
    ) -> Result<Option<BorrowedCallArgumentsV1>, E>,
) -> Result<bool, E> {
    if !input
        .source()
        .expr_at(site)
        .is_ok_and(|located| matches!(located.node(), ASTNode::New { .. }))
    {
        return Ok(false);
    }
    let response = borrow(
        site,
        BorrowedCallActualRequestV1::NamedArrayHolderInitializer(binding),
    )?;
    let Some(response) = response else {
        return Ok(false);
    };
    let BorrowedCallArgumentsV1::NamedArrayHolderSource(source) = response else {
        unavailable.get_or_insert(HomePrefixUnavailableV1::SourceMismatch);
        return Ok(true);
    };
    if source.binding() != binding
        || source.construction() != *site
        || input.owner() != binding.owner()
        || homes.iter().any(|home| home.binding() == binding)
    {
        unavailable.get_or_insert(HomePrefixUnavailableV1::SourceMismatch);
        return Ok(true);
    }
    let prefix = match unavailable.as_ref() {
        Some(issue) => Err(issue.clone()),
        None => issue_new_fault_continuation_v1(input, site)
            .map_err(|_| HomePrefixUnavailableV1::SourceMismatch)
            .map(|outward_fault| CallerNewHomePrefixV1::from_transition(binding, homes, &[], outward_fault, covered)),
    };
    if let Err(issue) = &prefix {
        unavailable.get_or_insert(issue.clone());
    }
    locals.acquire_named_array_holder(homes, source.clone());
    locals.retain_named_array_observation(NamedArrayHomeObservationV1::Initializer {
        source,
        prefix,
    });
    Ok(true)
}

pub(super) fn observe_append<E>(
    input: ResolvedFunctionLoweringInputV1<'_>,
    statement: &crate::mir::compiler::located::LocatedStmtV1<'_>,
    locals: &mut PrefixLocalFlow<'_>,
    homes: &[LiveHomeV1],
    covered: &[SourceStmtSiteV1],
    unavailable: &mut Option<HomePrefixUnavailableV1>,
    calls: &[LocalCallObservationV1],
    fields: &mut impl HomeFieldSourceConsultV1<E>,
    borrow: &mut impl FnMut(
        &OwnedExprSiteV1,
        BorrowedCallActualRequestV1<'_>,
    ) -> Result<Option<BorrowedCallArgumentsV1>, E>,
) -> Result<bool, E> {
    if !matches!(statement.node(), ASTNode::MethodCall { .. }) {
        return Ok(false);
    }
    let call = OwnedExprSiteV1::new(
        input.owner(),
        SourceExprSiteV1::from_node(statement.site().node().clone()),
    );
    let Some(original) = input.function().method_call_source_v1(call.site()) else {
        return Ok(false);
    };
    let receiver_site = original.receiver_site();
    let ResolvedMethodCallReceiverSourceV1::Lexical(ResolvedLexicalRefV1::Local(binding)) =
        original.receiver()
    else {
        return Ok(false);
    };
    if let Some(requirement) = fields.array_integer_append_source(&call) {
        let Some(loop_site) = locals.current_source_loop_site_v1().cloned() else {
            unavailable.get_or_insert(HomePrefixUnavailableV1::PrefixNotCovered(statement.site().clone()));
            return Ok(true);
        };
        if unavailable.is_some() {
            return Ok(true);
        }
        if let Some(evaluation) = field_append_argument(
            input, &call, original, binding, &requirement, locals, homes, calls, borrow,
        )? {
            let observation = NamedArrayHomeObservationV1::FieldIntegerAppend { loop_site, call, requirement, evaluation };
            locals.record_source_loop_array_append_v1(statement.site(), &observation);
            locals.retain_named_array_observation(observation);
            return Ok(true);
        }
        unavailable.get_or_insert(HomePrefixUnavailableV1::PrefixNotCovered(statement.site().clone()));
        return Ok(true);
    }
    if !locals.is_named_array_holder(binding) {
        return Ok(false);
    }
    let Some(source) = locals.available_named_array_holder(receiver_site) else {
        unavailable.get_or_insert(HomePrefixUnavailableV1::SourceMismatch);
        return Ok(true);
    };
    let response = borrow(
        &call,
        BorrowedCallActualRequestV1::NamedArrayAppendStatement {
            binding,
            source: &source,
        },
    )?;
    let Some(response) = response else {
        return Ok(false);
    };
    let BorrowedCallArgumentsV1::NamedArrayHolderSource(source) = response else {
        unavailable.get_or_insert(HomePrefixUnavailableV1::SourceMismatch);
        return Ok(true);
    };
    let Some(candidate) = source.append_candidate(&call) else {
        unavailable.get_or_insert(HomePrefixUnavailableV1::SourceMismatch);
        return Ok(true);
    };
    if !homes.iter().any(|home| home.binding() == source.binding())
        || !locals.has_named_array_holder(&source)
    {
        unavailable.get_or_insert(HomePrefixUnavailableV1::SourceMismatch);
        return Ok(true);
    }
    let observation = NamedArrayHomeObservationV1::Append {
        source,
        call,
        candidate,
        availability: unavailable.as_ref().cloned().map_or(Ok(()), Err),
        // Match expression/Call source order; the unwind owner reverses once.
        prior_homes: homes.to_vec().into_boxed_slice(),
        covered_statements: covered.to_vec().into_boxed_slice(),
    };
    locals.record_source_loop_array_append_v1(statement.site(), &observation);
    locals.retain_named_array_observation(observation);
    // The source store is covered; actual candidate ownership and successful
    // commit must still be established by the existing acquisition/physical owner.
    Ok(true)
}

fn field_append_argument<E>(
    input: ResolvedFunctionLoweringInputV1<'_>,
    call: &OwnedExprSiteV1,
    original: &std::sync::Arc<crate::mir::resolved_semantics::VerifiedResolvedMethodCallSourceV1>,
    binding: BindingRefV1,
    requirement: &crate::mir::resolved_semantics::NamedArrayFieldResidenceRequirementV1,
    locals: &PrefixLocalFlow<'_>,
    homes: &[LiveHomeV1],
    calls: &[LocalCallObservationV1],
    borrow: &mut impl FnMut(&OwnedExprSiteV1, BorrowedCallActualRequestV1<'_>)
        -> Result<Option<BorrowedCallArgumentsV1>, E>,
) -> Result<Option<std::rc::Rc<super::CheckedExpressionEvaluationV1>>, E> {
    use crate::mir::resolved_value_profile::{expression_class::ExpressionValueClassV1,
        expression_plan::call_argument_expression_source_plan_v1};
    if requirement.binding() != binding || requirement.owner() != call.owner()
        || requirement.call() != call.site()
        || input.function().variable_ref(original.receiver_site()) != Some(ResolvedLexicalRefV1::Local(binding))
        || !locals.has_array_field_residence_v1(requirement)
    { return Ok(None); }
    if !matches!(original.arguments(), [argument] if argument.site() == requirement.argument()) {
        return Ok(None);
    }
    let Ok(plan) = call_argument_expression_source_plan_v1(input, original, 0) else { return Ok(None); };
    let result = super::expression_plan::prepare_value_evaluation_v1(
        input, plan, ExpressionValueClassV1::I64, call, locals, homes, calls, borrow, None,
        &std::collections::BTreeMap::new(), None, None, &mut |_| None,
    )?;
    Ok(result.ok().and_then(|(evaluation, fields)| {
        // The argument is a passive integer expression. The Push itself
        // owns the call's Fault edge and the associated Home unwind.
        (fields.is_empty()
            && evaluation.cores().is_empty()
            && !evaluation.requires_fault_frame_v1(true, |_| None))
        .then_some(evaluation)
    }))
}
