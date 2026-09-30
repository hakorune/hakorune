//! Source relation for the bounded local direct-call continuation.
//!
//! This is a child of the existing `RootHomeFlow`, not a second call
//! inventory.  The target and its affine row remain owned by the direct-call
//! resolver; this relation only records where the source places the returned
//! value and which Homes were live before the call.

use super::local_flow::{OrdinaryObservation, PrefixLocalFlow, SourceScalarKind};
use super::{
    BindingRefV1, FunctionOwnerIdV1, OwnedExprSiteV1, ResolvedLiteralSourceV1,
    ResolvedMethodCallReceiverSourceV1, SourceBindingSiteV1, SourceExprSiteV1, SourceStmtSiteV1,
};
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;

/// Package-issued membership proof for one qualified static-box call site.
///
/// The issuer's sealed index is the sole authority: this row exists only for
/// a `QualifiedUnbound` receiver whose sealed target is a `StaticBoxMethod`
/// declaration whose result disposition is `ExactI64`.  The carried ordinals
/// are the callee's required-i64 parameter positions — the flow helper seals
/// every argument's source class and must see i64-class evidence at these.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct QualifiedStaticCallClaimV1 {
    required_i64_arguments: Box<[u32]>,
}

impl QualifiedStaticCallClaimV1 {
    pub(crate) fn new(required_i64_arguments: Box<[u32]>) -> Self {
        Self {
            required_i64_arguments,
        }
    }

    pub(crate) fn required_i64_arguments(&self) -> &[u32] {
        &self.required_i64_arguments
    }
}

/// One sealed source argument on a local-call continuation row.
///
/// `Integer`/`Bool` carry the exact literal; `Scalar` names a local or
/// parameter binding the homes flow already proved trivial-scalar.  No other
/// argument shape is admitted — a missing seal keeps the site unclaimed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum LocalCallArgumentV1 {
    Integer(i64),
    Bool(bool),
    Scalar(BindingRefV1),
}

/// Source-recorded result class of one local direct-call continuation.
/// `Map` marks a call into an unannotated callee whose sealed terminal
/// relation proves a Map return; `I64` marks the exact-i64 lane.
/// `Handle` marks a call into an unannotated callee whose sealed terminal
/// relation proves a `return new` construction — the caller receives the
/// transferred object as an owned Home and owes exactly one release.
/// `Nullable` marks a `me.m(..)` receiver call whose callee claim is
/// `NullableObject` — the caller receives an owned object or the `Void`
/// sentinel and owes a checked release, never an unconditional one.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LocalCallResultClassV1 {
    I64,
    Map,
    Handle,
    Nullable,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LocalCallObservationV1 {
    owner: FunctionOwnerIdV1,
    statement: SourceStmtSiteV1,
    site: OwnedExprSiteV1,
    declaration: SourceBindingSiteV1,
    destination: BindingRefV1,
    prior_homes: Box<[BindingRefV1]>,
    arguments: Box<[LocalCallArgumentV1]>,
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
        arguments: Box<[LocalCallArgumentV1]>,
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

    pub(crate) fn arguments(&self) -> &[LocalCallArgumentV1] {
        &self.arguments
    }

    pub(crate) const fn result(&self) -> LocalCallResultClassV1 {
        self.result
    }
}

/// Issue one exact literal-argument lexical instance Call (`recv.m(...)`)
/// from already-resolved source. The caller supplies the same
/// selected-call predicate; the receiver must be a resolver-sealed lexical
/// local — this helper never resolves a target or guesses a receiver class.
pub(crate) fn issue_lexical_local_call<E>(
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
    let Some((observed_site, call)) = input
        .function()
        .method_calls()
        .find(|(observed_site, _)| *observed_site == site.site())
    else {
        return Ok(None);
    };
    if observed_site != site.site()
        || !matches!(
            call.receiver(),
            super::ResolvedMethodCallReceiverSourceV1::Lexical(
                super::ResolvedLexicalRefV1::Local(binding)
            ) if binding.owner() == input.owner()
        )
    {
        return Ok(None);
    }
    let Some(arguments) = call
        .arguments()
        .iter()
        .map(|argument| {
            match input.function().expression_source().literal(argument.site()) {
                Some(ResolvedLiteralSourceV1::Integer(value)) => {
                    Some(LocalCallArgumentV1::Integer(*value))
                }
                _ => None,
            }
        })
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

/// Issue one `local x = Alias.m(..)` qualified static-box call continuation.
///
/// The package-injected predicate is the sole membership authority — it
/// returns the sealed claim only for a `QualifiedUnbound` receiver whose
/// target is a `StaticBoxMethod` with an `ExactI64` result disposition.  The
/// inventory row only corroborates the source receiver shape; every argument
/// must seal to an Integer/Bool literal or a scalar local/parameter binding,
/// and callee-required i64 ordinals must carry i64-class evidence.
pub(crate) fn issue_qualified_static_local_call<E>(
    input: ResolvedFunctionLoweringInputV1<'_>,
    statement: &SourceStmtSiteV1,
    site: &OwnedExprSiteV1,
    declaration: SourceBindingSiteV1,
    destination: BindingRefV1,
    prior_homes: &[BindingRefV1],
    locals: &PrefixLocalFlow<'_>,
    qualified_static_call: &mut impl FnMut(
        &OwnedExprSiteV1,
    ) -> Result<Option<QualifiedStaticCallClaimV1>, E>,
) -> Result<Option<LocalCallObservationV1>, E> {
    let Some(claim) = qualified_static_call(site)? else {
        return Ok(None);
    };
    let Some((observed_site, call)) = input
        .function()
        .method_calls()
        .find(|(observed_site, _)| *observed_site == site.site())
    else {
        return Ok(None);
    };
    if observed_site != site.site()
        || call.receiver() != ResolvedMethodCallReceiverSourceV1::QualifiedUnbound
    {
        return Ok(None);
    }
    let mut arguments = Vec::with_capacity(call.arguments().len());
    for argument in call.arguments() {
        let (row, i64_evidence) = match locals.observe(argument.site()) {
            Some(OrdinaryObservation::Integer(value)) => {
                (LocalCallArgumentV1::Integer(value), true)
            }
            Some(OrdinaryObservation::Bool(value)) => (LocalCallArgumentV1::Bool(value), false),
            Some(OrdinaryObservation::TrivialLocal(binding, Some(kind))) => (
                LocalCallArgumentV1::Scalar(binding),
                kind == SourceScalarKind::Integer,
            ),
            _ => return Ok(None),
        };
        if claim
            .required_i64_arguments()
            .contains(&argument.ordinal())
            && !i64_evidence
        {
            return Ok(None);
        }
        arguments.push(row);
    }
    Ok(Some(LocalCallObservationV1::issue(
        input.owner(),
        statement.clone(),
        site.clone(),
        declaration,
        destination,
        prior_homes.iter().copied().collect(),
        arguments.into_boxed_slice(),
        LocalCallResultClassV1::I64,
    )))
}

/// Issue one `local x = me.m(..)` receiver-call continuation whose site
/// already carries the package's nullable receiver-call observation. The
/// predicate is the sole membership authority — this helper never re-reads
/// the expression or infers a callee class. Typed argument evidence stays
/// on the package row (`ReceiverCallClassObservationV1`); the flow row
/// records no argument literals because they are not `i64` literals alone.
pub(crate) fn issue_receiver_local_call<E>(
    input: ResolvedFunctionLoweringInputV1<'_>,
    statement: &SourceStmtSiteV1,
    site: &OwnedExprSiteV1,
    declaration: SourceBindingSiteV1,
    destination: BindingRefV1,
    prior_homes: &[BindingRefV1],
    local_nullable_call: &mut impl FnMut(&OwnedExprSiteV1) -> Result<bool, E>,
) -> Result<Option<LocalCallObservationV1>, E> {
    if !local_nullable_call(site)? {
        return Ok(None);
    }
    let Some((observed_site, call)) = input
        .function()
        .method_calls()
        .find(|(observed_site, _)| *observed_site == site.site())
    else {
        return Ok(None);
    };
    // The sealed observation is sole membership — the predicate already
    // proved this site is the entry-loan `me` receiver. The inventory
    // check only retains that `me` in an instance method resolves to a
    // lexical `Local` binding; `CurrentOwner`/`Other`/`QualifiedUnbound`
    // receiver shapes can never agree with the sealed row.
    if observed_site != site.site()
        || !matches!(
            call.receiver(),
            super::ResolvedMethodCallReceiverSourceV1::Lexical(
                super::ResolvedLexicalRefV1::Local(_)
            )
        )
    {
        return Ok(None);
    }
    Ok(Some(LocalCallObservationV1::issue(
        input.owner(),
        statement.clone(),
        site.clone(),
        declaration,
        destination,
        prior_homes.iter().copied().collect(),
        Box::new([]),
        LocalCallResultClassV1::Nullable,
    )))
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
                Some(ResolvedLiteralSourceV1::Integer(value)) => {
                    Some(LocalCallArgumentV1::Integer(*value))
                }
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
