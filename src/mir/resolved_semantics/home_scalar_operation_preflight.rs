//! Same-owner scalar operator/operand preflight; parent publishes whole-expression results.
use super::*;
use crate::mir::resolved_semantics::home_new_prefix::LiveHomeV1;
use crate::mir::resolved_value_profile::expression_source::{
    expression_source_v1, ResolvedExpressionSourceV1,
};

pub(super) fn preflight<E>(
    operation: &mut Option<HomeExpressionPlanV1>,
    input: ResolvedFunctionLoweringInputV1<'_>,
    ledger: &crate::mir::resolved_semantics::CallableSemanticSourceLedgerView<'_>,
    site: &SourceExprSiteV1,
    locals: &PrefixLocalFlow<'_>,
    requests: &mut Vec<LocalFieldReadRequestV1>,
    calls: &mut Vec<LocalCallObservationV1>,
    statement: &SourceStmtSiteV1,
    homes: &[LiveHomeV1],
    static_call: &mut impl FnMut(&OwnedExprSiteV1) -> Result<Option<StaticI64CallClaimV1>, E>,
    checked_binary: Option<&SourceExprSiteV1>,
    local_initializer_profile: bool,
    is_root: bool,
    allow_mul_here: bool,
    checked_compare_operand: &mut impl FnMut(&OwnedExprSiteV1, &OwnedExprSiteV1) -> Result<bool, E>,
) -> Result<Option<SourceScalarKind>, E> {
    match locals.observe(site) {
        Some(OrdinaryObservation::Integer(_)) => return Ok(Some(SourceScalarKind::Integer)),
        Some(OrdinaryObservation::Bool(_)) => return Ok(Some(SourceScalarKind::Bool)),
        Some(OrdinaryObservation::TrivialLocal(_, Some(kind))) => return Ok(Some(kind)),
        _ => {}
    }
    if let Some(binary) = checked_binary {
        let owned = OwnedExprSiteV1::new(input.owner(), site.clone());
        let binary = OwnedExprSiteV1::new(input.owner(), binary.clone());
        if checked_compare_operand(&owned, &binary)? {
            return Ok(Some(SourceScalarKind::Integer));
        }
    }
    if let Some(call) =
        local_call_flow::issue_static_i64_value_call(input, statement, site, homes, static_call)?
    {
        calls.push(call);
        return Ok(Some(SourceScalarKind::Integer));
    }
    if let Some(request) = field_read::field_read_request(input, site, locals) {
        requests.push(request);
        return Ok(Some(SourceScalarKind::Integer));
    }
    let Some(row) = expression_source_v1(ledger, input.body_shape(), site)
        .and_then(ResolvedExpressionSourceV1::binary)
    else {
        return Ok(None);
    };
    let Some([lhs_site, rhs_site]) = binary_children_v1(site, row) else {
        return Ok(None);
    };
    use ResolvedBinaryOperatorV1 as Op;
    use SourceScalarKind as Kind;
    if matches!(row.operator(), Op::Equal | Op::NotEqual) {
        let alias_null = |value: &SourceExprSiteV1, null: &SourceExprSiteV1| {
            let Some(ResolvedLexicalRefV1::Local(binding)) = input.function().variable_ref(value)
            else {
                return false;
            };
            matches!(
                locals.field_read_receiver(binding),
                Some(local_flow::FieldReadReceiverV1::Alias { .. })
            ) && matches!(locals.observe(null), Some(OrdinaryObservation::Null))
        };
        if alias_null(row.lhs(), row.rhs()) || alias_null(row.rhs(), row.lhs()) {
            return Ok(Some(Kind::Bool));
        }
    }
    let (operand, result) = match row.operator() {
        Op::Add | Op::Subtract => (Kind::Integer, Kind::Integer),
        Op::Multiply
            if local_initializer_profile
                && is_root
                && expression_source_v1(ledger, input.body_shape(), row.lhs())
                    .and_then(ResolvedExpressionSourceV1::binary)
                    .is_some_and(|add| {
                        add.operator() == Op::Add
                            && binary_children_v1(row.lhs(), add).is_some()
                            && matches!(
                                input.function().expression_source().literal(add.lhs()),
                                Some(ResolvedLiteralSourceV1::Integer(_))
                            )
                            && matches!(
                                locals.observe(add.rhs()),
                                Some(OrdinaryObservation::TrivialLocal(_, Some(Kind::Integer)))
                            )
                    })
                && matches!(
                    locals.observe(row.rhs()),
                    Some(OrdinaryObservation::TrivialLocal(_, Some(Kind::Integer)))
                )
                && normal_integer_mul_available() =>
        {
            (Kind::Integer, Kind::Integer)
        }
        Op::Multiply
            if allow_mul_here
                && matches!(
                    locals.observe(row.lhs()),
                    Some(OrdinaryObservation::TrivialLocal(_, Some(Kind::Integer)))
                )
                && matches!(
                    input.function().expression_source().literal(row.rhs()),
                    Some(ResolvedLiteralSourceV1::Integer(_))
                )
                && normal_integer_mul_available() =>
        {
            (Kind::Integer, Kind::Integer)
        }
        Op::Divide | Op::Modulo
            if local_initializer_profile
                && is_root
                && matches!(input.function().expression_source().literal(row.rhs()),
                    Some(ResolvedLiteralSourceV1::Integer(divisor)) if *divisor > 0) =>
        {
            (Kind::Integer, Kind::Integer)
        }
        Op::Equal | Op::NotEqual | Op::Less | Op::Greater | Op::LessEqual | Op::GreaterEqual => {
            (Kind::Integer, Kind::Bool)
        }
        // Calls beneath short-circuit operators need path-specific Homes.
        // Keep this pure profile closed until that observation is established.
        Op::And | Op::Or => (Kind::Bool, Kind::Bool),
        _ => return Ok(None),
    };
    let mark = requests.len();
    let call_mark = calls.len();
    let checked_children = matches!(
        row.operator(),
        Op::Add
            | Op::Equal
            | Op::NotEqual
            | Op::Less
            | Op::Greater
            | Op::LessEqual
            | Op::GreaterEqual
    )
    .then_some(row.site());
    let Some(lhs) = preflight(
        &mut None,
        input,
        ledger,
        lhs_site,
        locals,
        requests,
        calls,
        statement,
        homes,
        static_call,
        checked_children,
        local_initializer_profile,
        false,
        false,
        checked_compare_operand,
    )?
    else {
        return Ok(None);
    };
    let Some(rhs) = preflight(
        &mut None,
        input,
        ledger,
        rhs_site,
        locals,
        requests,
        calls,
        statement,
        homes,
        static_call,
        checked_children,
        local_initializer_profile,
        false,
        is_root && local_initializer_profile && row.operator() == Op::Subtract,
        checked_compare_operand,
    )?
    else {
        return Ok(None);
    };
    if matches!(row.operator(), Op::And | Op::Or) && calls.len() != call_mark {
        return Ok(None);
    }
    if matches!(
        row.operator(),
        Op::Less | Op::Greater | Op::LessEqual | Op::GreaterEqual
    ) {
        for request in &requests[mark..] {
            let me_receiver = input
                .body_shape()
                .and_then(|shape| shape.expression_shape(&request.receiver_site))
                .is_some_and(|shape| {
                    matches!(
                        shape,
                        crate::mir::resolved_semantics::BodyExpressionShapeV1::Me { .. }
                    )
                });
            if !request.formal && !me_receiver {
                return Ok(None);
            }
        }
    }
    if lhs != operand || rhs != operand {
        return Ok(None);
    }
    if local_initializer_profile
        && is_root
        && matches!(row.operator(), Op::Subtract | Op::Divide | Op::Modulo)
    {
        let mut initializers = input
            .function()
            .expression_source()
            .initializers()
            .filter(|initializer| initializer.initializer_site() == Some(site));
        let Some(initializer) = initializers.next() else {
            return Ok(None);
        };
        if initializers.next().is_some()
            || initializer.binding().owner() != input.owner()
            || !matches!(initializer.declaration_site(),
                crate::mir::resolved_semantics::SourceBindingSiteV1::Local { statement: original, .. }
                    if original == statement)
        {
            return Ok(None);
        }
        let positive_divisor = if matches!(row.operator(), Op::Divide | Op::Modulo) {
            match input.function().expression_source().literal(row.rhs()) {
                Some(ResolvedLiteralSourceV1::Integer(value)) => Some(*value),
                _ => return Ok(None),
            }
        } else {
            None
        };
        let Ok(source) =
            crate::mir::resolved_value_profile::expression_plan::local_expression_source_plan_v1(
                ledger,
                input.body_shape(),
                initializer,
            )
        else {
            return Ok(None);
        };
        let Some(plan) = HomeExpressionPlanV1::scalar_root(source, row, positive_divisor) else {
            return Ok(None);
        };
        *operation = Some(plan);
    }
    Ok(Some(result))
}
