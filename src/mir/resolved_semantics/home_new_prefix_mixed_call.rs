//! Install one mixed local observation in the existing ordered Home walk.
use super::*;

pub(super) fn observe<E>(
    input: ResolvedFunctionLoweringInputV1<'_>,
    statement: &SourceStmtSiteV1,
    site: &OwnedExprSiteV1,
    declaration: &SourceBindingSiteV1,
    binding: BindingRefV1,
    homes: &mut HomesV1,
    locals: &mut PrefixLocalFlow<'_>,
    calls: &mut Vec<LocalCallObservationV1>,
    covered: &mut BTreeSet<OwnedExprSiteV1>,
    unavailable: &mut Option<HomePrefixUnavailableV1>,
    borrowed_actuals: &mut impl FnMut(
        &OwnedExprSiteV1,
        BorrowedCallActualRequestV1<'_>,
    ) -> Result<Option<BorrowedCallArgumentsV1>, E>,
) -> Result<bool, E> {
    let Some(selected) = local_call_flow::issue_mixed_local_call(
        input,
        statement,
        site,
        declaration.clone(),
        binding,
        homes,
        borrowed_actuals,
    )?
    else {
        return Ok(false);
    };
    match selected {
        Ok(call)
            if locals.acquire_received_mixed(
                homes,
                call.mixed().expect("mixed source loan").clone(),
                calls,
                covered,
            ) =>
        {
            covered.insert(call.site().clone());
            calls.push(call);
        }
        Ok(_) => {
            unavailable.get_or_insert(HomePrefixUnavailableV1::SourceMismatch);
        }
        Err(issue) => {
            unavailable.get_or_insert(issue);
        }
    }
    Ok(true)
}
