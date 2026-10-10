//! Pure scalar expression preflight. Source rows define operators and paths;
//! field declarations remain obligations until the whole batch is proved.
use super::*;
use crate::mir::dynamic_operator_contract::{
    issue_dynamic_operator_execution_envelope_v1, DynamicOperatorDomainV1, DynamicOperatorFamilyV1,
    DynamicOperatorValueClassV1,
};
use crate::mir::resolved_semantics::{ResolvedBinaryOperatorV1, SourcePathSegmentV1, SourcePathV1};

#[path = "home_static_eq_condition.rs"]
mod static_eq_condition;
pub(in crate::mir) use static_eq_condition::contains_exact_lhs as contains_static_eq_lhs_source_v1;

pub(super) fn observe_local_initializer<E>(
    input: ResolvedFunctionLoweringInputV1<'_>,
    site: &SourceExprSiteV1,
    locals: &PrefixLocalFlow<'_>,
    local_field_read: &mut impl FnMut(
        &[LocalFieldReadRequestV1],
        bool,
    ) -> Result<Option<Vec<LocalFieldReadResultV1>>, E>,
    statement: &SourceStmtSiteV1,
    homes: &[BindingRefV1],
    static_call: &mut impl FnMut(&OwnedExprSiteV1) -> Result<Option<StaticI64CallClaimV1>, E>,
    view_use: &mut impl FnMut(&OwnedExprSiteV1, BorrowedViewUseRequestV1<'_>) -> Result<bool, E>,
    borrowed_actuals: &mut impl FnMut(
        &OwnedExprSiteV1,
        BorrowedCallActualRequestV1<'_>,
    ) -> Result<Option<BorrowedCallArgumentsV1>, E>,
) -> Result<Option<(SourceScalarKind, Vec<LocalCallObservationV1>)>, E> {
    observe_scalar_expression(
        input,
        site,
        locals,
        None,
        true,
        local_field_read,
        statement,
        homes,
        static_call,
        &mut |operand, binary| {
            view_use(
                operand,
                BorrowedViewUseRequestV1::CheckedAddOperand { binary },
            )
        },
        borrowed_actuals,
    )
}

pub(super) fn observe_scalar_expression<E>(
    input: ResolvedFunctionLoweringInputV1<'_>,
    site: &SourceExprSiteV1,
    locals: &PrefixLocalFlow<'_>,
    required: Option<SourceScalarKind>,
    local_initializer_profile: bool,
    local_field_read: &mut impl FnMut(
        &[LocalFieldReadRequestV1],
        bool,
    ) -> Result<Option<Vec<LocalFieldReadResultV1>>, E>,
    statement: &SourceStmtSiteV1,
    homes: &[BindingRefV1],
    static_call: &mut impl FnMut(&OwnedExprSiteV1) -> Result<Option<StaticI64CallClaimV1>, E>,
    checked_compare_operand: &mut impl FnMut(&OwnedExprSiteV1, &OwnedExprSiteV1) -> Result<bool, E>,
    borrowed_actuals: &mut impl FnMut(
        &OwnedExprSiteV1,
        BorrowedCallActualRequestV1<'_>,
    ) -> Result<Option<BorrowedCallArgumentsV1>, E>,
) -> Result<Option<(SourceScalarKind, Vec<LocalCallObservationV1>)>, E> {
    if required == Some(SourceScalarKind::Bool) && !local_initializer_profile {
        if let Some(calls) = static_eq_condition::observe(
            input, site, statement, homes, static_call, borrowed_actuals,
        )? {
            return Ok(Some((SourceScalarKind::Bool, calls)));
        }
        if let Some((if_site, call_site)) = if_current_owner_ge_zero_call(input, site) {
            if if_site == *statement {
                let Some(call) = local_call_flow::issue_current_owner_i64_direct_value_call(
                    input,
                    statement,
                    &call_site,
                    homes,
                    locals,
                    static_call,
                    borrowed_actuals,
                )?
                else {
                    return Ok(None);
                };
                return Ok(Some((SourceScalarKind::Bool, vec![call])));
            }
        }
    }
    // An exact direct CurrentOwner value has no sibling expression to reject.
    // Its original ordered actuals can be staged and projected before the
    // generic scalar preflight, which still owns composed expressions.
    if required.is_none_or(|kind| kind == SourceScalarKind::Integer)
        && input.function().method_call(site).is_some()
    {
        if let Some(call) = local_call_flow::issue_current_owner_i64_direct_value_call(
            input,
            statement,
            site,
            homes,
            locals,
            static_call,
            borrowed_actuals,
        )? {
            return Ok(Some((SourceScalarKind::Integer, vec![call])));
        }
    }
    let mut requests = Vec::new();
    let mut calls = Vec::new();
    let Some(kind) = preflight(
        input,
        site,
        locals,
        &mut requests,
        &mut calls,
        statement,
        homes,
        static_call,
        None,
        local_initializer_profile,
        true,
        false,
        checked_compare_operand,
    )?
    else {
        return Ok(None);
    };
    if required.is_some_and(|expected| expected != kind) {
        return Ok(None);
    }
    if !requests.is_empty() {
        let Some(results) = local_field_read(&requests, true)? else {
            return Ok(None);
        };
        if results.len() != requests.len()
            || results
                .iter()
                .any(|result| *result != LocalFieldReadResultV1::Scalar)
        {
            return Ok(None);
        }
    }
    // Publish only after the whole original expression is proved. A rejected
    // right sibling or field batch never leaves a partial call observation.
    for call in &calls {
        borrowed_actuals(call.site(), BorrowedCallActualRequestV1::Observe(&[]))?;
    }
    Ok(Some((kind, calls)))
}

pub(super) enum GuardedMulReturnV1 {
    Unselected,
    Unavailable,
    Observed(Vec<LocalCallObservationV1>),
}

/// A returned I64 local multiplied by an exact zero-argument CurrentOwner
/// static I64 call. This is disjoint from the borrowed-formal Mul authority;
/// no call observation is published until the complete expression is proved.
pub(super) fn observe_local_mul_return<E>(
    input: ResolvedFunctionLoweringInputV1<'_>,
    site: &SourceExprSiteV1,
    statement: &SourceStmtSiteV1,
    locals: &PrefixLocalFlow<'_>,
    homes: &[BindingRefV1],
    static_call: &mut impl FnMut(&OwnedExprSiteV1) -> Result<Option<StaticI64CallClaimV1>, E>,
    borrowed_actuals: &mut impl FnMut(
        &OwnedExprSiteV1,
        BorrowedCallActualRequestV1<'_>,
    ) -> Result<Option<BorrowedCallArgumentsV1>, E>,
) -> Result<GuardedMulReturnV1, E> {
    let Some(binary) = input
        .function()
        .expression_source()
        .binary(site)
        .filter(|row| row.operator() == ResolvedBinaryOperatorV1::Multiply)
    else {
        return Ok(GuardedMulReturnV1::Unselected);
    };
    if !binary_sites_match(site, binary)
        || !normal_integer_mul_available()
        || !matches!(
            locals.observe(binary.lhs()),
            Some(OrdinaryObservation::TrivialLocal(
                _,
                Some(SourceScalarKind::Integer)
            ))
        )
    {
        return Ok(GuardedMulReturnV1::Unavailable);
    }
    let Some(call) = local_call_flow::issue_static_i64_value_call(
        input,
        statement,
        binary.rhs(),
        homes,
        static_call,
    )?
    else {
        return Ok(GuardedMulReturnV1::Unavailable);
    };
    borrowed_actuals(call.site(), BorrowedCallActualRequestV1::Observe(&[]))?;
    Ok(GuardedMulReturnV1::Observed(vec![call]))
}

/// Only an exact returned operation product lends this Normal scalar class.
/// Generic scalar expressions still exclude Multiply. Child observations are
/// staged until every original call child is available.
pub(super) fn observe_guarded_mul_return<E>(
    input: ResolvedFunctionLoweringInputV1<'_>,
    site: &SourceExprSiteV1,
    statement: &SourceStmtSiteV1,
    homes: &[BindingRefV1],
    view_use: &mut impl FnMut(&OwnedExprSiteV1, BorrowedViewUseRequestV1<'_>) -> Result<bool, E>,
    static_call: &mut impl FnMut(&OwnedExprSiteV1) -> Result<Option<StaticI64CallClaimV1>, E>,
    borrowed_actuals: &mut impl FnMut(
        &OwnedExprSiteV1,
        BorrowedCallActualRequestV1<'_>,
    ) -> Result<Option<BorrowedCallArgumentsV1>, E>,
) -> Result<GuardedMulReturnV1, E> {
    let Some(binary) = input
        .function()
        .expression_source()
        .binary(site)
        .filter(|row| row.operator() == ResolvedBinaryOperatorV1::Multiply)
    else {
        return Ok(GuardedMulReturnV1::Unselected);
    };
    if !view_use(
        &OwnedExprSiteV1::new(input.owner(), site.clone()),
        BorrowedViewUseRequestV1::IntegerMulReturn { exit: statement },
    )? {
        return Ok(GuardedMulReturnV1::Unavailable);
    }
    let mut calls = Vec::new();
    for child in [binary.lhs(), binary.rhs()] {
        if input.function().method_call(child).is_some() {
            let Some(call) = local_call_flow::issue_static_i64_value_call(
                input,
                statement,
                child,
                homes,
                static_call,
            )?
            else {
                return Ok(GuardedMulReturnV1::Unavailable);
            };
            calls.push(call);
        }
    }
    for call in &calls {
        borrowed_actuals(call.site(), BorrowedCallActualRequestV1::Observe(&[]))?;
    }
    Ok(GuardedMulReturnV1::Observed(calls))
}

fn preflight<E>(
    input: ResolvedFunctionLoweringInputV1<'_>,
    site: &SourceExprSiteV1,
    locals: &PrefixLocalFlow<'_>,
    requests: &mut Vec<LocalFieldReadRequestV1>,
    calls: &mut Vec<LocalCallObservationV1>,
    statement: &SourceStmtSiteV1,
    homes: &[BindingRefV1],
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
    let Some(row) = input.function().expression_source().binary(site) else {
        return Ok(None);
    };
    if !binary_sites_match(site, row) {
        return Ok(None);
    }
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
                && input
                    .function()
                    .expression_source()
                    .binary(row.lhs())
                    .is_some_and(|add| {
                        add.operator() == Op::Add
                            && binary_sites_match(row.lhs(), add)
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
        Op::Divide
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
    let checked_children =
        matches!(row.operator(), Op::Add | Op::Greater | Op::LessEqual).then_some(row.site());
    let Some(lhs) = preflight(
        input,
        row.lhs(),
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
        input,
        row.rhs(),
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
    Ok((lhs == operand && rhs == operand).then_some(result))
}

fn normal_integer_mul_available() -> bool {
    issue_dynamic_operator_execution_envelope_v1(DynamicOperatorDomainV1::new(
        DynamicOperatorFamilyV1::Mul,
        DynamicOperatorValueClassV1::NormalInteger,
        DynamicOperatorValueClassV1::NormalInteger,
    ))
    .is_ok()
}

/// Select only the new field-expression condition responsibility. Existing
/// conditions with no eligible field receiver keep their original admission.
pub(super) fn contains_source_request<E>(
    input: ResolvedFunctionLoweringInputV1<'_>,
    root: &SourceExprSiteV1,
    locals: &PrefixLocalFlow<'_>,
    static_call: &mut impl FnMut(&OwnedExprSiteV1) -> Result<Option<StaticI64CallClaimV1>, E>,
) -> Result<bool, E> {
    if static_eq_condition::contains_source_request(input, root, static_call)? {
        return Ok(true);
    }
    if let Some((_, call)) = if_current_owner_ge_zero_call(input, root) {
        let owned = OwnedExprSiteV1::new(input.owner(), call);
        return Ok(static_call(&owned)?.is_some_and(|claim| {
            claim
                .current_owner_source_required_i64_arguments()
                .is_some()
                && claim.corroborates_source(
                    &owned,
                    ResolvedMethodCallReceiverSourceV1::CurrentOwner,
                    1,
                )
        }));
    }
    Ok(profile_scope(input, root, locals, false, static_call)?.unwrap_or(false))
}

/// One resolver-sealed If condition: exact `CurrentOwner.i64(arg) >= 0`.
/// Verify the complete binary and sibling before the selected actual port
/// observes anything. The physical packet remains a separate obligation.
fn if_current_owner_ge_zero_call(
    input: ResolvedFunctionLoweringInputV1<'_>,
    root: &SourceExprSiteV1,
) -> Option<(SourceStmtSiteV1, SourceExprSiteV1)> {
    let if_site = input
        .function()
        .with_if_region_for_condition(root, |row| row.site().clone())
        .ok()?;
    let binary = input.function().expression_source().binary(root)?;
    if binary.operator() != ResolvedBinaryOperatorV1::GreaterEqual
        || !binary_sites_match(root, binary)
        || !matches!(
            input.function().expression_source().literal(binary.rhs()),
            Some(ResolvedLiteralSourceV1::Integer(0))
        )
        || issue_dynamic_operator_execution_envelope_v1(DynamicOperatorDomainV1::new(
            DynamicOperatorFamilyV1::GreaterEqual,
            DynamicOperatorValueClassV1::NormalInteger,
            DynamicOperatorValueClassV1::NormalInteger,
        ))
        .is_err()
    {
        return None;
    }
    let call = input.function().method_call(binary.lhs())?;
    (call.receiver() == ResolvedMethodCallReceiverSourceV1::CurrentOwner
        && call.arguments().len() == 1)
        .then(|| (if_site, binary.lhs().clone()))
}

/// Field morphology selects its existing profile; an additional static call
/// selects only through the SAME exact CurrentOwner I64 source proof. Bool,
/// Text and unknown call siblings keep their original unselected scope.
fn profile_scope<E>(
    input: ResolvedFunctionLoweringInputV1<'_>,
    site: &SourceExprSiteV1,
    locals: &PrefixLocalFlow<'_>,
    order_compare: bool,
    static_call: &mut impl FnMut(&OwnedExprSiteV1) -> Result<Option<StaticI64CallClaimV1>, E>,
) -> Result<Option<bool>, E> {
    if let Some(row) = input.function().expression_source().binary(site) {
        use ResolvedBinaryOperatorV1 as Op;
        if !matches!(
            row.operator(),
            Op::Add
                | Op::Subtract
                | Op::Equal
                | Op::NotEqual
                | Op::Less
                | Op::Greater
                | Op::LessEqual
                | Op::GreaterEqual
                | Op::And
                | Op::Or
        ) {
            return Ok(None);
        }
        let operand_order = matches!(
            row.operator(),
            Op::Less | Op::Greater | Op::LessEqual | Op::GreaterEqual
        );
        let Some(left) = profile_scope(input, row.lhs(), locals, operand_order, static_call)?
        else {
            return Ok(None);
        };
        let Some(right) = profile_scope(input, row.rhs(), locals, operand_order, static_call)?
        else {
            return Ok(None);
        };
        return Ok(Some(left || right));
    }
    if input
        .function()
        .method_calls()
        .any(|(observed, _)| observed == site)
    {
        return Ok(
            static_call(&OwnedExprSiteV1::new(input.owner(), site.clone()))?
                .filter(|claim| claim.is_current_owner_zeroarg())
                .map(|_| true),
        );
    }
    if matches!(
        input.function().expression_source().literal(site),
        Some(
            crate::mir::resolved_semantics::ResolvedLiteralSourceV1::Integer(_)
                | crate::mir::resolved_semantics::ResolvedLiteralSourceV1::Bool(_)
        )
    ) || matches!(
        input.function().variable_ref(site),
        Some(ResolvedLexicalRefV1::Local(_))
    ) {
        return Ok(Some(false));
    }
    let Some(shape) = input.body_shape() else {
        return Ok(None);
    };
    let Some(crate::mir::resolved_semantics::BodyExpressionShapeV1::FieldAccess { object, .. }) =
        shape.expression_shape(site)
    else {
        return Ok(None);
    };
    let via_me = matches!(
        shape.expression_shape(object),
        Some(crate::mir::resolved_semantics::BodyExpressionShapeV1::Me { .. })
    );
    let binding = match input.function().variable_ref(object) {
        Some(ResolvedLexicalRefV1::Local(binding)) => binding,
        _ => match shape.expression_shape(object) {
            Some(crate::mir::resolved_semantics::BodyExpressionShapeV1::Me {
                receiver: crate::mir::resolved_semantics::BodyMeReceiverV1::Lexical(binding),
                ..
            }) => *binding,
            _ => return Ok(None),
        },
    };
    if order_compare {
        if matches!(
            locals.field_read_receiver(binding),
            Some(local_flow::FieldReadReceiverV1::GuardedFormal)
        ) {
            return Ok(Some(true));
        }
        return Ok(via_me.then_some(false));
    }
    Ok(Some(locals.is_field_read_candidate(binding)))
}

fn binary_sites_match(
    root: &SourceExprSiteV1,
    row: &crate::mir::resolved_semantics::ResolvedBinaryExpressionSourceV1,
) -> bool {
    let path = SourcePathV1::from_node(root.node());
    row.site() == root
        && row.lhs() == &path.child(SourcePathSegmentV1::Lhs).expr()
        && row.rhs() == &path.child(SourcePathSegmentV1::Rhs).expr()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::resolved_semantics::ResolvedBinaryExpressionSourceV1;

    #[test]
    fn scalar_expression_binary_rejects_foreign_root_and_swapped_child_sites() {
        let path = SourcePathV1::root_body(0).child(SourcePathSegmentV1::Initializer(0));
        let root = path.expr();
        let lhs = path.child(SourcePathSegmentV1::Lhs).expr();
        let rhs = path.child(SourcePathSegmentV1::Rhs).expr();
        let good = ResolvedBinaryExpressionSourceV1::from_parts_for_test(
            root.clone(),
            ResolvedBinaryOperatorV1::Add,
            lhs.clone(),
            rhs.clone(),
        );
        assert!(binary_sites_match(&root, &good));
        assert!(!binary_sites_match(
            &SourcePathV1::root_body(1).expr(),
            &good
        ));
        let swapped = ResolvedBinaryExpressionSourceV1::from_parts_for_test(
            root.clone(),
            ResolvedBinaryOperatorV1::Add,
            rhs,
            lhs,
        );
        assert!(!binary_sites_match(&root, &swapped));
    }

    #[test]
    fn guarded_mul_return_refuses_missing_or_failed_child_before_observation() {
        let crate::ast::ASTNode::Program { statements, .. } =
            crate::parser::NyashParser::parse_from_string(
                "function probe(p) { if p <= 8 { return p * me.limit() } return 0 }",
            )
            .unwrap()
        else {
            panic!("Program")
        };
        let function = statements
            .into_iter()
            .find(|node| matches!(node, crate::ast::ASTNode::FunctionDeclaration { .. }))
            .unwrap();
        let source =
            crate::mir::compiler::VerifiedResolvedSourceUnitV1::resolve_function(function).unwrap();
        let input = source.root_function_input().unwrap();
        let binary = input
            .function()
            .expression_source()
            .binaries()
            .find(|row| row.operator() == ResolvedBinaryOperatorV1::Multiply)
            .unwrap();
        let completion =
            crate::mir::resolved_control_flow::verify_function_completion_v1(input).unwrap();
        let exit = completion
            .explicit_sites()
            .iter()
            .find(|exit| {
                SourcePathV1::from_node(exit.node())
                    .child(SourcePathSegmentV1::Value)
                    .expr()
                    == *binary.site()
            })
            .unwrap();
        for mutation in 0..4 {
            let mut observed = 0;
            let mut child_demands = 0;
            let result = observe_guarded_mul_return(
                input,
                binary.site(),
                exit,
                &[],
                &mut |_, request| {
                    assert!(matches!(
                        request,
                        BorrowedViewUseRequestV1::IntegerMulReturn { .. }
                    ));
                    if mutation == 3 {
                        return Err("original-source-error".to_owned());
                    }
                    Ok(mutation != 2)
                },
                &mut |_| {
                    child_demands += 1;
                    if mutation == 1 {
                        return Err("original-child-error".to_owned());
                    }
                    Ok(None)
                },
                &mut |_, _| {
                    observed += 1;
                    Ok(None)
                },
            );
            assert_eq!(observed, 0);
            assert_eq!(child_demands, usize::from(mutation < 2));
            match mutation {
                1 => assert_eq!(result.err().as_deref(), Some("original-child-error")),
                3 => assert_eq!(result.err().as_deref(), Some("original-source-error")),
                _ => assert!(matches!(result.unwrap(), GuardedMulReturnV1::Unavailable)),
            }
        }
    }
}
