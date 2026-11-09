//! Scalar destination of the shared original expression evaluation.
use super::*;

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
        &[super::super::super::LocalFieldReadRequestV1],
        super::super::super::LocalFieldReadDemandV1,
    ) -> Result<Option<Vec<super::super::super::LocalFieldReadResultV1>>, E>,
) -> Result<Option<Result<TerminalScalarReturnV1, super::super::HomePrefixUnavailableV1>>, E> {
    let Ok(value) = input
        .source()
        .child_expr_from_stmt(statement, super::super::ExprChildRoleV1::ReturnValue)
    else {
        return Ok(None);
    };
    let stored_field = input.body_shape().is_some_and(|shape| matches!(
        shape.expression_shape(value.site()),
        Some(crate::mir::resolved_semantics::BodyExpressionShapeV1::FieldAccess { .. })
    ));
    let binary = input
        .function()
        .expression_source()
        .binary(value.site())
        .map(|row| row.operator());
    let add = binary == Some(crate::mir::resolved_semantics::ResolvedBinaryOperatorV1::Add);
    let copy = input
        .function()
        .expression_source()
        .unary(value.site())
        .is_some_and(|unary| unary.is_explicit_copy_v1());
    // R16: a comparison or logic root names Bool by the expression-class table.
    let bool_root = binary.is_some_and(
        crate::mir::resolved_value_profile::expression_class::binary_operator_yields_bool_v1,
    ) || input
        .function()
        .expression_source()
        .unary(value.site())
        .is_some_and(|unary| {
            unary.operator() == crate::mir::resolved_semantics::ResolvedUnaryOperatorV1::Not
        });
    let purpose = OwnedExprSiteV1::new(input.owner(), value.site().clone());
    let owns_inputs = matches!(actuals(
        &purpose,
        super::super::super::BorrowedCallActualRequestV1::CallEntryHomes(input),
    )?, Some(super::super::super::BorrowedCallArgumentsV1::CallEntryHomes { taken, instance_input: true, .. }) if !taken.is_empty());
    if !stored_field
        && !add
        && !copy
        && !bool_root
        && !owns_inputs
        && binary != Some(crate::mir::resolved_semantics::ResolvedBinaryOperatorV1::Subtract)
    {
        return Ok(None);
    }
    let refused = || {
        Some(Err(
            super::super::HomePrefixUnavailableV1::ReturnValueNotCovered(statement.site().clone()),
        ))
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
    let prepared = prepare_value_evaluation_v1(
        input,
        std::rc::Rc::clone(&source),
        if bool_root {
            crate::mir::resolved_value_profile::expression_class::ExpressionValueClassV1::Bool
        } else {
            crate::mir::resolved_value_profile::expression_class::ExpressionValueClassV1::I64
        },
        &purpose,
        locals,
        homes,
        calls,
        actuals,
        None,
        &std::collections::BTreeMap::new(),
        &mut |site| {
            super::super::super::field_read::expression_field_read_request(
                input, &source, site, locals,
            )
        },
    )?;
    let (evaluation, requests) = match prepared {
        Ok(prepared) => prepared,
        Err(reason) => {
            // Add may return Text, and a comparison may compare Text. This is a passive
            // source classification, before field installation or terminal issuance;
            // its existing owner decides.
            if crate::config::env::builder_debug_enabled() {
                eprintln!(
                    "[scalar-return] owner={:?} declined: {reason}",
                    input.owner()
                );
            }
            return Ok(if add || bool_root { None } else { refused() });
        }
    };
    // The pre-existing plain scalar kernel is still the consumer for pure
    // arithmetic without field demand, explicit copy, or invocation. It has
    // no lifecycle-root ABI demand; do not route it through a framed consumer.
    if add && requests.is_empty() && !source.contains_copy_v1() && evaluation.calls().is_empty() && evaluation.cores().is_empty() {
        return Ok(None);
    }
    if stored_field {
        // Only an integer field belongs to this scalar lane; Text and others keep theirs.
        let probed = fields(
            &requests,
            super::super::super::LocalFieldReadDemandV1::SourceExpression {
                probe: true,
                plan: std::rc::Rc::clone(&source),
            },
        )?;
        if !probed.as_deref().is_some_and(|rows| {
            rows.iter()
                .all(|row| matches!(row, super::super::super::LocalFieldReadResultV1::Scalar))
        }) {
            return Ok(None);
        }
    }
    if !requests.is_empty() {
        let Some(candidate) =
            super::super::super::HomeExpressionPlanV1::terminal_fields_v1(&evaluation, &requests)
        else {
            return Ok(refused());
        };
        let mut installation = locals.expression_installation();
        if !installation.approve(&candidate) {
            return Ok(refused());
        }
        if fields(
            &requests,
            super::super::super::LocalFieldReadDemandV1::SourceExpression {
                probe: false,
                plan: source,
            },
        )?
        .is_none()
        {
            return Ok(refused());
        }
        installation.commit();
    }
    Ok(Some(
        TerminalScalarReturnV1::issue(
            input.owner(),
            statement.site().clone(),
            value.site().clone(),
        )
        .with_evaluation_v1(evaluation)
        .ok_or_else(|| {
            super::super::HomePrefixUnavailableV1::ReturnValueNotCovered(statement.site().clone())
        }),
    ))
}
