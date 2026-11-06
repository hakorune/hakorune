//! The one answer to "which call is this Local initializer": kind from name
//! resolution, result class from the callee's own class source, decided by the
//! package once per site. The flow installs by the class and never asks a
//! second membership question; a site without a route stays a named rejection.
use super::local_call_flow as flow;
use super::local_flow::PrefixLocalFlow;
use super::{
    BindingRefV1, BorrowedCallActualRequestV1, BorrowedCallArgumentsV1, LiveHomeV1,
    LocalCallObservationV1, LocalCallResultClassV1, OwnedExprSiteV1, SourceBindingSiteV1,
    SourceStmtSiteV1, StaticI64CallClaimV1,
};
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum LocalCallRouteV1 {
    DirectI64,
    DirectMap,
    DirectHandle,
    Static(StaticI64CallClaimV1),
    LexicalI64,
    LexicalNullable,
    LexicalHandle,
    ReceiverNullable,
    ReceiverHandle,
}

impl LocalCallRouteV1 {
    pub(crate) const fn result(&self) -> LocalCallResultClassV1 {
        match self {
            Self::DirectI64 | Self::Static(_) | Self::LexicalI64 => LocalCallResultClassV1::I64,
            Self::DirectMap => LocalCallResultClassV1::Map,
            Self::DirectHandle | Self::LexicalHandle | Self::ReceiverHandle => {
                LocalCallResultClassV1::Handle
            }
            Self::LexicalNullable | Self::ReceiverNullable => LocalCallResultClassV1::Nullable,
        }
    }

    pub(crate) fn into_static_claim(self) -> Option<StaticI64CallClaimV1> {
        match self {
            Self::Static(claim) => Some(claim),
            _ => None,
        }
    }

    /// The readiness probe covers the static and lexical routes only.
    pub(crate) const fn in_readiness_probe(&self) -> bool {
        matches!(self, Self::Static(_) | Self::LexicalI64 | Self::LexicalNullable)
    }

    pub(crate) const fn is_lexical_i64(&self) -> bool {
        matches!(self, Self::LexicalI64)
    }

    pub(crate) const fn is_lexical_nullable(&self) -> bool {
        matches!(self, Self::LexicalNullable)
    }
}

/// Seal one routed call: the route names the issuer, the issuer checks the call
/// structure and seals the arguments. `nested_lexical_i64` answers only for
/// argument calls nested in a lexical call, from the same route question.
pub(super) fn issue_routed_local_call<E>(
    input: ResolvedFunctionLoweringInputV1<'_>,
    statement: &SourceStmtSiteV1,
    site: &OwnedExprSiteV1,
    declaration: SourceBindingSiteV1,
    destination: BindingRefV1,
    prior_homes: &[LiveHomeV1],
    locals: &PrefixLocalFlow<'_>,
    route: LocalCallRouteV1,
    nested_lexical_i64: &mut impl FnMut(&OwnedExprSiteV1) -> Result<bool, E>,
    borrowed_arguments: &mut impl FnMut(
        &OwnedExprSiteV1,
        BorrowedCallActualRequestV1<'_>,
    ) -> Result<Option<BorrowedCallArgumentsV1>, E>,
) -> Result<Option<LocalCallObservationV1>, E> {
    let result = route.result();
    match route {
        LocalCallRouteV1::DirectI64 | LocalCallRouteV1::DirectMap | LocalCallRouteV1::DirectHandle => {
            Ok(flow::issue_local_call(input, statement, site, declaration, destination, prior_homes, result))
        }
        LocalCallRouteV1::Static(claim) => flow::issue_static_i64_local_call(
            input, statement, site, declaration, destination, prior_homes, locals, claim, borrowed_arguments,
        ),
        LocalCallRouteV1::LexicalI64 => flow::issue_lexical_i64_local_call(
            input, statement, site, declaration, destination, prior_homes, locals, nested_lexical_i64, borrowed_arguments,
        ),
        LocalCallRouteV1::LexicalNullable => flow::issue_lexical_nullable_local_call(
            input, statement, site, declaration, destination, prior_homes, locals, nested_lexical_i64, borrowed_arguments,
        ),
        LocalCallRouteV1::LexicalHandle => flow::issue_lexical_local_call(
            input, statement, site, declaration, destination, prior_homes, result, borrowed_arguments,
        ),
        LocalCallRouteV1::ReceiverNullable | LocalCallRouteV1::ReceiverHandle => Ok(
            flow::issue_receiver_local_call(input, statement, site, declaration, destination, prior_homes, result),
        ),
    }
}

/// The one install table, keyed by the routed result class: I64 installs a
/// trivial scalar; Map, Handle and Nullable results are owned by the caller
/// from the call site, so each joins the live Homes before it is installed.
pub(super) fn install_routed_local_call(
    call: LocalCallObservationV1,
    binding: BindingRefV1,
    site: &OwnedExprSiteV1,
    locals: &mut PrefixLocalFlow<'_>,
    homes: &mut Vec<LiveHomeV1>,
    path_calls: &mut std::collections::BTreeSet<OwnedExprSiteV1>,
    local_calls: &mut Vec<LocalCallObservationV1>,
) {
    let result = call.result();
    path_calls.insert(call.site().clone());
    local_calls.push(call);
    if result != LocalCallResultClassV1::I64 {
        homes.push(LiveHomeV1::Stable(binding));
    }
    match result {
        LocalCallResultClassV1::I64 => locals.install_i64_call_result(binding),
        LocalCallResultClassV1::Map => locals.install_map(binding),
        LocalCallResultClassV1::Handle => locals.install_received_handle(binding, site),
        LocalCallResultClassV1::Nullable => locals.install_received_nullable(binding, site),
        _ => unreachable!("a route names only I64, Map, Handle and Nullable results"),
    }
}

/// The static-claim question, asked of the route.
pub(super) fn static_claim_of<'a, E>(
    route: &'a mut impl FnMut(&OwnedExprSiteV1) -> Result<Option<LocalCallRouteV1>, E>,
) -> impl FnMut(&OwnedExprSiteV1) -> Result<Option<StaticI64CallClaimV1>, E> + 'a {
    move |site| Ok(route(site)?.and_then(LocalCallRouteV1::into_static_claim))
}

/// The lexical-I64 question (an argument call nested in a call), asked of the route.
pub(super) fn lexical_i64_of<'a, E>(
    route: &'a mut impl FnMut(&OwnedExprSiteV1) -> Result<Option<LocalCallRouteV1>, E>,
) -> impl FnMut(&OwnedExprSiteV1) -> Result<bool, E> + 'a {
    move |site| Ok(route(site)?.is_some_and(|row| row.is_lexical_i64()))
}
