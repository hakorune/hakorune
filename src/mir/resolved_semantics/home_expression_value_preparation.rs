//! Prepare one checked value evaluation independently of its destination.
use super::*;

pub(in crate::mir::resolved_semantics::home_new_prefix) fn prepare_value_evaluation_v1<E>(
    input: ResolvedFunctionLoweringInputV1<'_>,
    source: std::rc::Rc<
        crate::mir::resolved_value_profile::expression_plan::ExpressionSourcePlanV1,
    >,
    expected: crate::mir::resolved_value_profile::expression_class::ExpressionValueClassV1,
    purpose_site: &OwnedExprSiteV1,
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
    field_text_source: Option<
        &crate::mir::normal_callable_semantic_package::CheckedFieldArrayAccessSourceV1,
    >,
    field_seeds: &std::collections::BTreeMap<
        usize,
        crate::mir::resolved_value_profile::expression_projection::ExpressionLeafClassV1,
    >,
    field_request: &mut impl FnMut(
        &SourceExprSiteV1,
    ) -> Option<super::super::super::LocalFieldReadRequestV1>,
) -> Result<
    Result<
        (
            std::rc::Rc<
                crate::mir::resolved_semantics::home_new_prefix::CheckedExpressionEvaluationV1,
            >,
            Vec<super::super::super::LocalFieldReadRequestV1>,
        ),
        &'static str,
    >,
    E,
> {
    use crate::mir::resolved_value_profile::expression_projection::ExpressionLeafClassV1;
    let Some(prepared) = crate::mir::resolved_semantics::home_new_prefix::expression_plan::prepare_expression_calls_v1(&source,
        crate::mir::resolved_semantics::home_new_prefix::expression_plan::ExpressionLeafProfileV1::Print, field_seeds, homes,
        &mut |site| super::super::super::local_call_flow::issue_source_expression_call_v1(input, &source, site, homes,
            &mut |binding| locals.field_alias_call_path(binding), actuals))? else { return Ok(Err("call-preparation-declined")) };
    let mut initial = Vec::new();
    for (node, original) in source.nodes().iter().enumerate() {
        if !prepared.is_active_reference(node) {
            continue;
        }
        let Some((binding, root)) =
            crate::mir::resolved_semantics::home_new_prefix::argument_expression::text_reference(
                input,
                locals,
                original.site(),
            )
        else {
            continue;
        };
        let owned = purpose_site.clone();
        if let Some(field_text_source) = field_text_source {
            let Some(crate::mir::resolved_semantics::home_new_prefix::BorrowedCallArgumentsV1::TextEntryRead(read)) = actuals(&owned,
                crate::mir::resolved_semantics::home_new_prefix::BorrowedCallActualRequestV1::FieldArrayTextEntryRead {
                    input,
                    source: field_text_source,
                    plan: &source,
                    read: original.site(),
                    binding,
                    root,
                })? else {
                if std::env::var_os("HAKO_DEBUG_FIELD_TEXT_APPEND").is_some() {
                    eprintln!("[field-text-append] owner={:?} reason=text-entry-read-unavailable read={:?}", input.owner(), original.site());
                }
                return Ok(Err("text-entry-read-unavailable"));
            };
            initial.push((node, crate::mir::resolved_semantics::home_new_prefix::CheckedExpressionReferenceV1::ArgumentEntry(read)));
        } else if let Some(crate::mir::resolved_semantics::home_new_prefix::BorrowedCallArgumentsV1::FiniteEntryRead(read)) = actuals(&owned,
            crate::mir::resolved_semantics::home_new_prefix::BorrowedCallActualRequestV1::FiniteEntryRead { input, plan: &source, read: original.site(), binding, root })? {
            initial.push((node, crate::mir::resolved_semantics::home_new_prefix::CheckedExpressionReferenceV1::FiniteEntry {
                read, exclusion: locals.entry_null_exclusion(original.site(), binding, root),
            }));
        }
    }
    let (evaluation, fields) = match crate::mir::resolved_semantics::home_new_prefix::expression_plan::qualify_statement_evaluation_v1(prepared, initial,
        &mut |site| locals.static_text_reference(site), &mut |site| locals.text_reference(site),
        &mut |site| locals.loop_text_reference_v1(site, homes),
        &mut |site| locals.current_scalar_read_v1(site, calls), field_request)? {
            Ok(qualified) => qualified,
            Err(reason) => {
                if std::env::var_os("HAKO_DEBUG_FIELD_TEXT_APPEND").is_some() {
                    eprintln!("[field-text-append] owner={:?} reason=evaluation-qualification-missing:{reason}", input.owner());
                }
                return Ok(Err(reason));
            }
        };
    if evaluation.root_class() != ExpressionLeafClassV1::Value(expected) {
        return Ok(Err("root-class-mismatch"));
    }
    Ok(Ok((std::rc::Rc::new(evaluation), fields)))
}
