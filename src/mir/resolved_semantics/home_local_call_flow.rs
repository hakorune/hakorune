//! Source relation for the bounded local direct-call continuation.
//!
//! This is a child of the existing `RootHomeFlow`, not a second call
//! inventory.  The target and its affine row remain owned by the direct-call
//! resolver; this relation only records where the source places the returned
//! value and which Homes were live before the call.

use super::{
    BindingRefV1, FunctionOwnerIdV1, OwnedExprSiteV1, ResolvedLiteralSourceV1, SourceBindingSiteV1,
    SourceExprSiteV1, SourceStmtSiteV1,
};
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;

/// Source-recorded result class of one local direct-call continuation.
/// `Map` marks a call into an unannotated callee whose sealed terminal
/// relation proves a Map return; `I64` marks the exact-i64 lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LocalCallResultClassV1 {
    I64,
    Map,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LocalCallObservationV1 {
    owner: FunctionOwnerIdV1,
    statement: SourceStmtSiteV1,
    site: OwnedExprSiteV1,
    declaration: SourceBindingSiteV1,
    destination: BindingRefV1,
    prior_homes: Box<[BindingRefV1]>,
    arguments: Box<[i64]>,
    result: LocalCallResultClassV1,
}

impl LocalCallObservationV1 {
    pub(crate) fn issue(
        owner: FunctionOwnerIdV1,
        statement: SourceStmtSiteV1,
        site: OwnedExprSiteV1,
        declaration: SourceBindingSiteV1,
        destination: BindingRefV1,
        prior_homes: Box<[BindingRefV1]>,
        arguments: Box<[i64]>,
        result: LocalCallResultClassV1,
    ) -> Self {
        Self {
            owner,
            statement,
            site,
            declaration,
            destination,
            prior_homes,
            arguments,
            result,
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

    /// The receiving binding's own declaration site; the physical commit
    /// row keys its local-statement install on this exact site.
    pub(crate) const fn declaration(&self) -> &SourceBindingSiteV1 {
        &self.declaration
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

    pub(crate) const fn result(&self) -> LocalCallResultClassV1 {
        self.result
    }
}

/// Issue one exact literal-argument local Call from already-resolved source.
/// The caller supplies the existing selected-call predicate and the result
/// class that predicate proved; this helper never resolves a target or turns
/// a missing observation into a default call.
pub(crate) fn issue_local_call<E>(
    input: ResolvedFunctionLoweringInputV1<'_>,
    statement: &SourceStmtSiteV1,
    site: &OwnedExprSiteV1,
    declaration: SourceBindingSiteV1,
    destination: BindingRefV1,
    prior_homes: &[BindingRefV1],
    result: LocalCallResultClassV1,
    is_selected_call: &mut impl FnMut(&OwnedExprSiteV1) -> Result<bool, E>,
) -> Result<Option<LocalCallObservationV1>, E> {
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
    Ok(Some(LocalCallObservationV1::issue(
        input.owner(),
        statement.clone(),
        site.clone(),
        declaration,
        destination,
        prior_homes.iter().copied().collect(),
        arguments.into_boxed_slice(),
        result,
    )))
}
