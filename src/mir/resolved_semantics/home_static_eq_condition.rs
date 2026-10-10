//! One composed If Eq Home source: checked one-input Static Lhs and
//! zero-input Static RHS. No execution, operator envelope or packet is issued.
use super::*;

struct StaticEqPairV1 {
    statement: SourceStmtSiteV1,
    lhs: SourceExprSiteV1,
    rhs: SourceExprSiteV1,
}

fn source_pair(
    input: ResolvedFunctionLoweringInputV1<'_>,
    root: &SourceExprSiteV1,
) -> Option<StaticEqPairV1> {
    let statement = input
        .function()
        .with_if_region_for_condition(root, |row| row.site().clone())
        .ok()?;
    let binary = input.function().expression_source().binary(root)?;
    if binary.operator() != ResolvedBinaryOperatorV1::Equal || !binary_sites_match(root, binary) {
        return None;
    }
    let lhs = input.function().method_call(binary.lhs())?;
    let rhs = input.function().method_call(binary.rhs())?;
    let [argument] = lhs.arguments() else {
        return None;
    };
    if lhs.receiver() != ResolvedMethodCallReceiverSourceV1::CurrentOwner
        || rhs.receiver() != ResolvedMethodCallReceiverSourceV1::CurrentOwner
        || !rhs.arguments().is_empty()
        || !matches!(
            input.function().variable_ref(argument.site()),
            Some(ResolvedLexicalRefV1::Local(_))
        )
    {
        return None;
    }
    Some(StaticEqPairV1 {
        statement,
        lhs: binary.lhs().clone(),
        rhs: binary.rhs().clone(),
    })
}

fn checked_pair<E>(
    input: ResolvedFunctionLoweringInputV1<'_>,
    root: &SourceExprSiteV1,
    static_call: &mut impl FnMut(&OwnedExprSiteV1) -> Result<Option<StaticI64CallClaimV1>, E>,
) -> Result<Option<StaticEqPairV1>, E> {
    let Some(pair) = source_pair(input, root) else {
        return Ok(None);
    };
    let lhs = OwnedExprSiteV1::new(input.owner(), pair.lhs.clone());
    let rhs = OwnedExprSiteV1::new(input.owner(), pair.rhs.clone());
    let Some(lhs_claim) = static_call(&lhs)? else {
        return Ok(None);
    };
    let Some(rhs_claim) = static_call(&rhs)? else {
        return Ok(None);
    };
    if lhs_claim
        .current_owner_source_required_i64_arguments()
        .is_none()
        || !lhs_claim.corroborates_source(&lhs, ResolvedMethodCallReceiverSourceV1::CurrentOwner, 1)
        || !rhs_claim.is_current_owner_zeroarg()
        || !rhs_claim.corroborates_source(&rhs, ResolvedMethodCallReceiverSourceV1::CurrentOwner, 0)
    {
        return Ok(None);
    }
    Ok(Some(pair))
}

/// Source eligibility borrows the same exact If-Eq law before Home issuance.
/// A matching pair still needs both later Home observations.
pub(in crate::mir) fn contains_exact_lhs<E>(
    input: ResolvedFunctionLoweringInputV1<'_>,
    lhs: &OwnedExprSiteV1,
    static_call: &mut impl FnMut(&OwnedExprSiteV1) -> Result<Option<StaticI64CallClaimV1>, E>,
) -> Result<bool, E> {
    if lhs.owner() != input.owner() {
        return Ok(false);
    }
    for if_site in input.function().if_region_sites() {
        let root = SourcePathV1::from_node(if_site.node())
            .child(SourcePathSegmentV1::IfCondition)
            .expr();
        if checked_pair(input, &root, static_call)?.is_some_and(|pair| pair.lhs == *lhs.site()) {
            return Ok(true);
        }
    }
    Ok(false)
}

pub(super) fn contains_source_request<E>(
    input: ResolvedFunctionLoweringInputV1<'_>,
    root: &SourceExprSiteV1,
    static_call: &mut impl FnMut(&OwnedExprSiteV1) -> Result<Option<StaticI64CallClaimV1>, E>,
) -> Result<bool, E> {
    Ok(checked_pair(input, root, static_call)?.is_some())
}

pub(super) fn observe<E>(
    input: ResolvedFunctionLoweringInputV1<'_>,
    root: &SourceExprSiteV1,
    statement: &SourceStmtSiteV1,
    homes: &[BindingRefV1],
    static_call: &mut impl FnMut(&OwnedExprSiteV1) -> Result<Option<StaticI64CallClaimV1>, E>,
    borrowed_actuals: &mut impl FnMut(
        &OwnedExprSiteV1,
        BorrowedCallActualRequestV1<'_>,
    ) -> Result<Option<BorrowedCallArgumentsV1>, E>,
) -> Result<Option<Vec<LocalCallObservationV1>>, E> {
    let Some(pair) = checked_pair(input, root, static_call)? else {
        return Ok(None);
    };
    if pair.statement != *statement {
        return Ok(None);
    }
    // Check the RHS source before projecting the staged LHS actual. The
    // returned pair is published only after both observations agree.
    let Some(rhs) = local_call_flow::issue_static_i64_value_call(
        input,
        statement,
        &pair.rhs,
        homes,
        static_call,
    )?
    else {
        return Ok(None);
    };
    let Some(lhs) = local_call_flow::project_current_owner_i64_staged_value_call(
        input,
        statement,
        &pair.lhs,
        homes,
        static_call,
        borrowed_actuals,
    )?
    else {
        return Ok(None);
    };
    if lhs.site() != &OwnedExprSiteV1::new(input.owner(), pair.lhs)
        || rhs.site() != &OwnedExprSiteV1::new(input.owner(), pair.rhs)
        || lhs.arguments().len() != 1
        || !rhs.arguments().is_empty()
        || lhs.result() != LocalCallResultClassV1::I64
        || rhs.result() != LocalCallResultClassV1::I64
    {
        return Ok(None);
    }
    borrowed_actuals(rhs.site(), BorrowedCallActualRequestV1::Observe(&[]))?;
    Ok(Some(vec![lhs, rhs]))
}
