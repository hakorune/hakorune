//! Source relation for the bounded local direct-call continuation.
//!
//! This is a child of the existing `RootHomeFlow`, not a second call
//! inventory.  The target and its affine row remain owned by the direct-call
//! resolver; this relation only records where the source places the returned
//! i64 and which Homes were live before the call.

use super::{
    BindingRefV1, FunctionOwnerIdV1, OwnedExprSiteV1, ResolvedLiteralSourceV1, SourceExprSiteV1,
    SourceStmtSiteV1,
};
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LocalI64CallObservationV1 {
    owner: FunctionOwnerIdV1,
    statement: SourceStmtSiteV1,
    site: OwnedExprSiteV1,
    destination: BindingRefV1,
    prior_homes: Box<[BindingRefV1]>,
    arguments: Box<[i64]>,
}

impl LocalI64CallObservationV1 {
    pub(crate) fn issue(
        owner: FunctionOwnerIdV1,
        statement: SourceStmtSiteV1,
        site: OwnedExprSiteV1,
        destination: BindingRefV1,
        prior_homes: Box<[BindingRefV1]>,
        arguments: Box<[i64]>,
    ) -> Self {
        Self {
            owner,
            statement,
            site,
            destination,
            prior_homes,
            arguments,
        }
    }

    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(crate) fn statement(&self) -> &SourceStmtSiteV1 {
        &self.statement
    }

    pub(crate) fn site(&self) -> &OwnedExprSiteV1 {
        &self.site
    }

    pub(crate) const fn destination(&self) -> BindingRefV1 {
        self.destination
    }

    pub(crate) fn prior_homes(&self) -> &[BindingRefV1] {
        &self.prior_homes
    }

    pub(crate) fn arguments(&self) -> &[i64] {
        &self.arguments
    }
}

/// Issue one exact literal-argument local Call from already-resolved source.
/// The caller supplies the existing selected-call predicate; this helper never
/// resolves a target or turns a missing observation into a default call.
pub(crate) fn issue_local_i64_call<E>(
    input: ResolvedFunctionLoweringInputV1<'_>,
    statement: &SourceStmtSiteV1,
    site: &OwnedExprSiteV1,
    destination: BindingRefV1,
    prior_homes: &[BindingRefV1],
    is_selected_call: &mut impl FnMut(&OwnedExprSiteV1) -> Result<bool, E>,
) -> Result<Option<LocalI64CallObservationV1>, E> {
    if !is_selected_call(site)? {
        return Ok(None);
    }
    let Some((observed_site, observation)) = input
        .function()
        .direct_call_observations()
        .find(|(observed_site, _)| *observed_site == site.site())
    else {
        return Ok(None);
    };
    if observed_site != site.site() {
        return Ok(None);
    }
    let Some(arguments) = observation
        .argument_sites()
        .iter()
        .map(
            |argument| match input.function().expression_source().literal(argument) {
                Some(ResolvedLiteralSourceV1::Integer(value)) => Some(*value),
                _ => None,
            },
        )
        .collect::<Option<Vec<_>>>()
    else {
        return Ok(None);
    };
    Ok(Some(LocalI64CallObservationV1::issue(
        input.owner(),
        statement.clone(),
        site.clone(),
        destination,
        prior_homes.iter().copied().collect(),
        arguments.into_boxed_slice(),
    )))
}
