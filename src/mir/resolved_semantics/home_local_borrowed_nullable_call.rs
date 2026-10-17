//! Borrowed nullable LOCAL source observation through the existing typed port.
//! Records the result binding and original root; never installs an owning Home.
use super::*;

pub(crate) fn issue_borrowed_nullable_local_call<E>(
    input: ResolvedFunctionLoweringInputV1<'_>,
    statement: &SourceStmtSiteV1,
    site: &OwnedExprSiteV1,
    declaration: SourceBindingSiteV1,
    destination: BindingRefV1,
    prior_homes: &[BindingRefV1],
    borrowed_arguments: &mut impl FnMut(
        &OwnedExprSiteV1,
        BorrowedCallActualRequestV1<'_>,
    ) -> Result<Option<BorrowedCallArgumentsV1>, E>,
) -> Result<Option<Result<LocalCallObservationV1, super::super::HomePrefixUnavailableV1>>, E> {
    let Some(response) = borrowed_arguments(
        site,
        BorrowedCallActualRequestV1::BorrowedNullableLocalArguments(destination),
    )?
    else {
        return Ok(None);
    };
    let refusal = || Some(Err(super::super::HomePrefixUnavailableV1::SourceMismatch));
    let BorrowedCallArgumentsV1::BorrowedNullableLocalSource { loan, arguments } = response else {
        return Ok(refusal());
    };
    if loan.call() != site
        || loan.destination() != destination
        || loan.declaration() != &declaration
        || loan.root().owner() != input.owner()
        || !input
            .function()
            .method_call(site.site())
            .is_some_and(|call| call.receiver_site() == loan.receiver())
        || !lexical_local_call::corroborate_local_initializer_v1(
            input,
            statement,
            site,
            &declaration,
            destination,
            prior_homes,
        )
        || !lexical_local_call::corroborate_received_arguments_v1(input, site, &arguments)
    {
        return Ok(refusal());
    }
    // This role can be issued only with the corroborated original loan.
    Ok(Some(Ok(LocalCallObservationV1 {
        owner: input.owner(),
        statement: statement.clone(),
        site: site.clone(),
        destination: LocalCallDestinationV1::LocalBinding {
            declaration,
            binding: destination,
        },
        prior_homes: prior_homes.iter().copied().collect(),
        arguments,
        result: LocalCallResultClassV1::BorrowedNullable,
        borrowed_scalar: None, unit_source: None,
        borrowed_nullable: Some(Box::new(loan)), mixed: None, field_alias: None,
    })))
}
