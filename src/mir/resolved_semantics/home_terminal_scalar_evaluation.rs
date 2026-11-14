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
    fields: &mut impl crate::mir::resolved_semantics::home_new_prefix::HomeFieldSourceConsultV1<E>,
) -> Result<Option<Result<TerminalScalarReturnV1, super::super::HomePrefixUnavailableV1>>, E> {
    let Ok(value) = input
        .source()
        .child_expr_from_stmt(statement, super::super::ExprChildRoleV1::ReturnValue)
    else {
        return Ok(None);
    };
    let stored_field = input.body_shape().is_some_and(|shape| {
        matches!(
            shape.expression_shape(value.site()),
            Some(crate::mir::resolved_semantics::BodyExpressionShapeV1::FieldAccess { .. })
        )
    });
    let binary = input
        .function()
        .expression_source()
        .binary(value.site())
        .map(|row| row.operator());
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
    let owned_parameter_receiver = input
        .function()
        .method_call_source_v1(value.site())
        .is_some_and(|call| {
            use crate::mir::resolved_semantics::{
                BindingKindV1, ResolvedLexicalRefV1, ResolvedMethodCallReceiverSourceV1,
            };
            let ResolvedMethodCallReceiverSourceV1::Lexical(ResolvedLexicalRefV1::Local(binding)) =
                call.receiver()
            else {
                return false;
            };
            input
                .function()
                .binding(binding)
                .is_some_and(|row| matches!(row.kind(), BindingKindV1::Parameter { .. }))
                && homes.iter().any(|home| home.is_stable_binding(binding))
        });
    let integer_source = input.body_shape().and_then(|shape| {
        let ledger = input.forest().callable_source_ledger(input.owner()).ok()?;
        let source = crate::mir::resolved_value_profile::expression_plan::statement_expression_source_plan_v1(
            &ledger, shape, statement.site()).ok()??;
        use crate::mir::resolved_value_profile::{
            expression_class::ExpressionValueClassV1 as Class,
            expression_projection::{project_expression_class_v1, ExpressionLeafClassV1 as Leaf},
        };
        project_expression_class_v1(&source, |index| {
            let node = &source.nodes()[index];
            match locals.current_scalar_read_v1(node.site(), calls) {
                Some(read) if read.kind() == crate::mir::resolved_semantics::home_new_prefix::SourceScalarKind::Integer =>
                    Leaf::Value(Class::I64),
                _ => Leaf::Unresolved,
            }
        })
    }) == Some(crate::mir::resolved_value_profile::expression_class::ExpressionValueClassV1::I64);
    let previous_scalar_candidate = stored_field
        || owned_parameter_receiver
        || integer_source
        || binary.is_some()
        || copy
        || bool_root
        || owns_inputs;
    if !previous_scalar_candidate
        && input
            .function()
            .method_call_source_v1(value.site())
            .is_none()
    {
        return Ok(None);
    }
    let refused = || {
        Some(Err(
            super::super::HomePrefixUnavailableV1::ReturnValueNotCovered(statement.site().clone()),
        ))
    };
    let preselection_unavailable = || {
        if previous_scalar_candidate {
            refused()
        } else {
            None
        }
    };
    let Some(shape) = input.body_shape() else {
        return Ok(preselection_unavailable());
    };
    let Ok(ledger) = input.forest().callable_source_ledger(input.owner()) else {
        return Ok(preselection_unavailable());
    };
    let Ok(Some(source)) =
        crate::mir::resolved_value_profile::expression_plan::statement_expression_source_plan_v1(
            &ledger,
            shape,
            statement.site(),
        )
    else {
        return Ok(preselection_unavailable());
    };
    let mut root_argument_evaluation = false;
    if !previous_scalar_candidate {
        if let Some(call) = input.function().method_call_source_v1(value.site()) {
            use crate::mir::resolved_semantics::home_new_prefix::{
                BorrowedCallActualRequestV1, BorrowedCallArgumentsV1,
            };
            for argument in call.arguments() {
                if let Some(BorrowedCallArgumentsV1::ArgumentExpressionSource(loan)) = actuals(
                    &purpose,
                    BorrowedCallActualRequestV1::ArgumentExpressionSource(
                        input,
                        argument.ordinal(),
                    ),
                )? {
                    if !matches!(loan.plan().nodes().first().map(|node| node.source()),
                        Some(crate::mir::resolved_value_profile::expression_plan::ExpressionNodeSourceV1::Literal(_)
                            | crate::mir::resolved_value_profile::expression_plan::ExpressionNodeSourceV1::Reference(_)))
                    { root_argument_evaluation = true; }
                }
            }
        }
    }
    let root_call_class = if previous_scalar_candidate {
        None
    } else {
        match actuals(
            &purpose,
            super::super::super::BorrowedCallActualRequestV1::SourceResultArguments {
                expression: Some(&source),
            },
        )? {
            Some(super::super::super::BorrowedCallArgumentsV1::SourceResult {
                loan: super::super::super::LocalCallResultSourceV1::Ordinary(loan),
                ..
            }) if loan
                .static_source()
                .is_some_and(|original| original.is_scalar_return_source_v1(&source)) =>
            {
                Some(loan.result())
            }
            _ => None,
        }
    };
    let root_core = if previous_scalar_candidate || root_call_class.is_some() {
        None
    } else {
        use crate::mir::resolved_semantics::home_new_prefix::{
            BorrowedCallActualRequestV1, BorrowedCallArgumentsV1,
        };
        match actuals(
            &purpose,
            BorrowedCallActualRequestV1::ExpressionCoreSource {
                input,
                plan: &source,
            },
        )? {
            Some(BorrowedCallArgumentsV1::ExpressionCoreSource(core)) => {
                let Some(class) = core.scalar_read_result_class_v1() else {
                    return Ok(None);
                };
                Some((core, class))
            }
            None if root_argument_evaluation => None,
            _ => return Ok(None),
        }
    };
    let core_result_class = root_core.as_ref().map(|(_, class)| *class);
    let root_core = root_core.map(|(core, _)| core);
    let field_seeds = if root_core.is_some() {
        super::super::super::field_read::text_field_seeds_v1(input, &source, locals, fields)
    } else {
        Default::default()
    };
    // The complete Return evaluation borrows its Call and field leaves from
    // their existing source issuers and retains their ordered cleanup proof.
    let prepared = prepare_value_evaluation_v1(
        input,
        std::rc::Rc::clone(&source),
        core_result_class
            .or(root_call_class.map(|_| {
                crate::mir::resolved_value_profile::expression_class::ExpressionValueClassV1::I64
            }))
            .unwrap_or(if bool_root {
                crate::mir::resolved_value_profile::expression_class::ExpressionValueClassV1::Bool
            } else {
                crate::mir::resolved_value_profile::expression_class::ExpressionValueClassV1::I64
            }),
        &purpose,
        locals,
        homes,
        calls,
        actuals,
        None,
        &field_seeds,
        root_core,
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
            return Ok(if integer_source { refused() } else { None });
        }
    };
    // Qualified scalar Core operands keep their acquisition and Fault duties
    // in the shared evaluation; other terminal owners retain their boundary.
    if !previous_scalar_candidate
        && !root_argument_evaluation
        && core_result_class.is_none()
        && root_call_class.is_none()
        && evaluation.requires_fault_frame_v1(true, |_| None)
    {
        return Ok(None);
    }
    // A value already available at this point needs no new evaluation.
    // Every computation uses the shared expression writer.
    if evaluation.operations().is_empty()
        && evaluation.calls().is_empty()
        && evaluation.cores().is_empty()
        && evaluation.owned_result_nodes().is_empty()
        && requests.is_empty()
        && matches!(source.nodes().first().map(|node| node.source()),
            Some(crate::mir::resolved_value_profile::expression_plan::ExpressionNodeSourceV1::Literal(_)
                | crate::mir::resolved_value_profile::expression_plan::ExpressionNodeSourceV1::Reference(_)))
    {
        return Ok(None);
    }

    if stored_field {
        // Only an integer field belongs to this scalar lane; Text and others keep theirs.
        let probed = fields.local_field_read(
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
        if fields
            .local_field_read(
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
