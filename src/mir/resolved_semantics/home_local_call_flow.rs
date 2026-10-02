//! Source relation for the bounded local direct-call continuation.
//!
//! This is a child of the existing `RootHomeFlow`, not a second call
//! inventory.  The target and its affine row remain owned by the direct-call
//! resolver; this relation only records where the source places the returned
//! value and which Homes were live before the call.

use super::local_flow::{OrdinaryObservation, PrefixLocalFlow, SourceScalarKind};
use super::{
    BindingRefV1, FunctionOwnerIdV1, OwnedExprSiteV1, ResolvedLexicalRefV1,
    ResolvedLiteralSourceV1, ResolvedMethodCallReceiverSourceV1, SourceBindingSiteV1,
    SourceExprSiteV1, SourceStmtSiteV1,
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
/// parameter binding the homes flow already proved trivial-scalar.
/// `CallResult` is a nested i64-result lexical call sitting directly in
/// argument position — its sealed row carries the inner call's own
/// arguments and the live prior-Home set at the enclosing statement, so
/// the emitter can re-run the same call evidence without re-reading
/// source.  No other argument shape is admitted — a missing seal keeps
/// the site unclaimed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum LocalCallArgumentV1 {
    Integer(i64),
    Bool(bool),
    Scalar(BindingRefV1),
    CallResult(Box<ArgumentCallObservationV1>),
    /// Original opaque argument reference; the package ledger owns its domain.
    BorrowedActual {
        ordinal: u32,
        site: SourceExprSiteV1,
    },
}

/// One proven-i64 lexical call sitting in direct argument position of a
/// claimed call — `local x = recv.m(recv2.m2(..))` seals the inner call
/// here.  There is no destination binding or `local` statement for an
/// argument-position call, so this row is intentionally not a
/// `LocalCallObservationV1`; the emitter folds the inner Invoke into the
/// outer call's recorded binding group.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ArgumentCallObservationV1 {
    site: OwnedExprSiteV1,
    prior_homes: Box<[BindingRefV1]>,
    arguments: Box<[LocalCallArgumentV1]>,
}

impl ArgumentCallObservationV1 {
    fn issue(
        site: OwnedExprSiteV1,
        prior_homes: Box<[BindingRefV1]>,
        arguments: Box<[LocalCallArgumentV1]>,
    ) -> Self {
        Self {
            site,
            prior_homes,
            arguments,
        }
    }

    pub(crate) fn site(&self) -> &OwnedExprSiteV1 {
        &self.site
    }

    pub(crate) fn prior_homes(&self) -> &[BindingRefV1] {
        &self.prior_homes
    }

    pub(crate) fn arguments(&self) -> &[LocalCallArgumentV1] {
        &self.arguments
    }
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

/// Exact source destination; discard never installs a binding or owns a Home.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum LocalCallDestinationV1 {
    LocalBinding {
        declaration: SourceBindingSiteV1,
        binding: BindingRefV1,
    },
    Discard,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LocalCallObservationV1 {
    owner: FunctionOwnerIdV1,
    statement: SourceStmtSiteV1,
    site: OwnedExprSiteV1,
    destination: LocalCallDestinationV1,
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
            destination: LocalCallDestinationV1::LocalBinding {
                declaration,
                binding: destination,
            },
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

    /// Owned-result consumers must explicitly require a receiving local.
    pub(crate) fn local_binding(&self) -> Option<(&SourceBindingSiteV1, BindingRefV1)> {
        match &self.destination {
            LocalCallDestinationV1::LocalBinding {
                declaration,
                binding,
            } => Some((declaration, *binding)),
            LocalCallDestinationV1::Discard => None,
        }
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
            match input
                .function()
                .expression_source()
                .literal(argument.site())
            {
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
        if claim.required_i64_arguments().contains(&argument.ordinal()) && !i64_evidence {
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

/// Issue an exact lexical instance-call local continuation. The package
/// callback lends a selected borrowed call's original ordered arguments;
/// otherwise the existing strict-I64 predicate owns membership. Result class
/// remains source-proved I64 and is corroborated at physical emission.
pub(crate) fn issue_lexical_i64_local_call<E>(
    input: ResolvedFunctionLoweringInputV1<'_>,
    statement: &SourceStmtSiteV1,
    site: &OwnedExprSiteV1,
    declaration: SourceBindingSiteV1,
    destination: BindingRefV1,
    prior_homes: &[BindingRefV1],
    locals: &PrefixLocalFlow<'_>,
    is_selected_call: &mut impl FnMut(&OwnedExprSiteV1) -> Result<bool, E>,
    borrowed_arguments: &mut impl FnMut(
        &OwnedExprSiteV1,
        Option<&[BorrowedCallActualCandidateV1]>,
    ) -> Result<Option<Box<[LocalCallArgumentV1]>>, E>,
) -> Result<Option<LocalCallObservationV1>, E> {
    issue_lexical_i64_call(
        input,
        statement,
        site,
        LocalCallDestinationV1::LocalBinding {
            declaration,
            binding: destination,
        },
        prior_homes,
        locals,
        true,
        is_selected_call,
        borrowed_arguments,
    )
}

/// Only an exact statement call with selected borrowed-I64 evidence can discard.
pub(crate) fn issue_lexical_i64_discard_call<E>(
    input: ResolvedFunctionLoweringInputV1<'_>,
    statement: &crate::mir::compiler::located::LocatedStmtV1<'_>,
    prior_homes: &[BindingRefV1],
    locals: &PrefixLocalFlow<'_>,
    is_selected_call: &mut impl FnMut(&OwnedExprSiteV1) -> Result<bool, E>,
    borrowed_arguments: &mut impl FnMut(
        &OwnedExprSiteV1,
        Option<&[BorrowedCallActualCandidateV1]>,
    ) -> Result<Option<Box<[LocalCallArgumentV1]>>, E>,
) -> Result<Option<LocalCallObservationV1>, E> {
    if statement.owner() != input.owner()
        || !matches!(statement.node(), crate::ast::ASTNode::MethodCall { .. })
    {
        return Ok(None);
    }
    let site = OwnedExprSiteV1::new(
        input.owner(),
        SourceExprSiteV1::from_node(statement.site().node().clone()),
    );
    issue_lexical_i64_call(
        input,
        statement.site(),
        &site,
        LocalCallDestinationV1::Discard,
        prior_homes,
        locals,
        false,
        is_selected_call,
        borrowed_arguments,
    )
}

fn issue_lexical_i64_call<E>(
    input: ResolvedFunctionLoweringInputV1<'_>,
    statement: &SourceStmtSiteV1,
    site: &OwnedExprSiteV1,
    destination: LocalCallDestinationV1,
    prior_homes: &[BindingRefV1],
    locals: &PrefixLocalFlow<'_>,
    allow_strict: bool,
    is_selected_call: &mut impl FnMut(&OwnedExprSiteV1) -> Result<bool, E>,
    borrowed_arguments: &mut impl FnMut(
        &OwnedExprSiteV1,
        Option<&[BorrowedCallActualCandidateV1]>,
    ) -> Result<Option<Box<[LocalCallArgumentV1]>>, E>,
) -> Result<Option<LocalCallObservationV1>, E> {
    let Some(arguments) = seal_lexical_i64_arguments_at(
        input,
        site,
        prior_homes,
        locals,
        allow_strict,
        is_selected_call,
        borrowed_arguments,
    )?
    else {
        return Ok(None);
    };
    Ok(Some(LocalCallObservationV1 {
        owner: input.owner(),
        statement: statement.clone(),
        site: site.clone(),
        destination,
        prior_homes: prior_homes.iter().copied().collect(),
        arguments: arguments.into_boxed_slice(),
        result: LocalCallResultClassV1::I64,
    }))
}

/// Direct ReturnValue only; this does not create a local/discard continuation.
pub(super) fn issue_borrowed_i64_terminal_call<E>(
    input: ResolvedFunctionLoweringInputV1<'_>,
    statement: &crate::mir::compiler::located::LocatedStmtV1<'_>,
    prior_homes: &[BindingRefV1],
    locals: &PrefixLocalFlow<'_>,
    is_selected_call: &mut impl FnMut(&OwnedExprSiteV1) -> Result<bool, E>,
    borrowed_arguments: &mut impl FnMut(
        &OwnedExprSiteV1,
        Option<&[BorrowedCallActualCandidateV1]>,
    ) -> Result<Option<Box<[LocalCallArgumentV1]>>, E>,
) -> Result<Option<super::TerminalI64CallReturnV1>, E> {
    if statement.owner() != input.owner()
        || !matches!(
            statement.node(),
            crate::ast::ASTNode::Return { value: Some(_), .. }
        )
    {
        return Ok(None);
    }
    let Ok(value) = input.source().child_expr_from_stmt(
        statement,
        crate::mir::resolved_semantics::ExprChildRoleV1::ReturnValue,
    ) else {
        return Ok(None);
    };
    if !matches!(value.node(), crate::ast::ASTNode::MethodCall { .. }) {
        return Ok(None);
    }
    let site = OwnedExprSiteV1::new(input.owner(), value.site().clone());
    let Some(arguments) = seal_lexical_i64_arguments_at(
        input,
        &site,
        prior_homes,
        locals,
        true,
        is_selected_call,
        borrowed_arguments,
    )?
    else {
        return Ok(None);
    };
    if !borrowed_actuals::contains_borrowed_actual_v1(&arguments) {
        return Ok(None);
    }
    Ok(Some(super::TerminalI64CallReturnV1::issue(
        input.owner(),
        statement.site().clone(),
        value.site().clone(),
        arguments
            .into_iter()
            .map(super::TerminalCallArgumentV1::Lexical)
            .collect(),
    )))
}

fn seal_lexical_i64_arguments_at<E>(
    input: ResolvedFunctionLoweringInputV1<'_>,
    site: &OwnedExprSiteV1,
    prior_homes: &[BindingRefV1],
    locals: &PrefixLocalFlow<'_>,
    allow_strict: bool,
    is_selected_call: &mut impl FnMut(&OwnedExprSiteV1) -> Result<bool, E>,
    borrowed_arguments: &mut impl FnMut(
        &OwnedExprSiteV1,
        Option<&[BorrowedCallActualCandidateV1]>,
    ) -> Result<Option<Box<[LocalCallArgumentV1]>>, E>,
) -> Result<Option<Vec<LocalCallArgumentV1>>, E> {
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
            ResolvedMethodCallReceiverSourceV1::Lexical(ResolvedLexicalRefV1::Local(binding))
                if binding.owner() == input.owner()
        )
    {
        return Ok(None);
    }
    seal_i64_call_arguments(
        input,
        locals,
        call,
        prior_homes,
        allow_strict,
        is_selected_call,
        borrowed_arguments,
    )
}

/// Demand the selected borrowed projection first, without re-observing locals.
/// Only positively unselected sites may use strict-I64 sealing. Its Integer/
/// Scalar arguments and direct argument-call recursion retain source order
/// and the enclosing live-Home set. No other subtree is accepted here.
fn seal_i64_call_arguments<E>(
    input: ResolvedFunctionLoweringInputV1<'_>,
    locals: &PrefixLocalFlow<'_>,
    call: &crate::mir::resolved_semantics::VerifiedResolvedMethodCallSourceV1,
    prior_homes: &[BindingRefV1],
    allow_strict: bool,
    is_selected_call: &mut impl FnMut(&OwnedExprSiteV1) -> Result<bool, E>,
    borrowed_arguments: &mut impl FnMut(
        &OwnedExprSiteV1,
        Option<&[BorrowedCallActualCandidateV1]>,
    ) -> Result<Option<Box<[LocalCallArgumentV1]>>, E>,
) -> Result<Option<Vec<LocalCallArgumentV1>>, E> {
    let owned = OwnedExprSiteV1::new(input.owner(), call.site().clone());
    if let Some(arguments) = borrowed_arguments(&owned, None)? {
        return Ok(Some(arguments.into_vec()));
    }
    if !allow_strict || !is_selected_call(&owned)? {
        return Ok(None);
    }
    let mut arguments = Vec::with_capacity(call.arguments().len());
    for argument in call.arguments() {
        let row = match locals.observe(argument.site()) {
            Some(OrdinaryObservation::Integer(value)) => LocalCallArgumentV1::Integer(value),
            Some(OrdinaryObservation::TrivialLocal(binding, Some(SourceScalarKind::Integer))) => {
                LocalCallArgumentV1::Scalar(binding)
            }
            _ => {
                let Some(inner) = seal_argument_call(
                    input,
                    locals,
                    argument.site(),
                    prior_homes,
                    is_selected_call,
                    borrowed_arguments,
                )?
                else {
                    return Ok(None);
                };
                LocalCallArgumentV1::CallResult(Box::new(inner))
            }
        };
        arguments.push(row);
    }
    Ok(Some(arguments))
}

/// Seal one argument-position method call — the inner site must be a
/// lexical-receiver `Lexical(Local)` method call whose selected callee is
/// i64-proven by the same package predicate the outer claim used. Nested
/// deeper expression shapes (binary operands, condition operands) never
/// reach `method_calls()` here and stay unclaimed.
fn seal_argument_call<E>(
    input: ResolvedFunctionLoweringInputV1<'_>,
    locals: &PrefixLocalFlow<'_>,
    site: &SourceExprSiteV1,
    prior_homes: &[BindingRefV1],
    is_selected_call: &mut impl FnMut(&OwnedExprSiteV1) -> Result<bool, E>,
    borrowed_arguments: &mut impl FnMut(
        &OwnedExprSiteV1,
        Option<&[BorrowedCallActualCandidateV1]>,
    ) -> Result<Option<Box<[LocalCallArgumentV1]>>, E>,
) -> Result<Option<ArgumentCallObservationV1>, E> {
    let owned = OwnedExprSiteV1::new(input.owner(), site.clone());
    let Some((observed_site, call)) = input
        .function()
        .method_calls()
        .find(|(observed_site, _)| *observed_site == site)
    else {
        return Ok(None);
    };
    if observed_site != site
        || !matches!(
            call.receiver(),
            ResolvedMethodCallReceiverSourceV1::Lexical(ResolvedLexicalRefV1::Local(binding))
                if binding.owner() == input.owner()
        )
    {
        return Ok(None);
    }
    let Some(inner) = seal_i64_call_arguments(
        input,
        locals,
        call,
        prior_homes,
        true,
        is_selected_call,
        borrowed_arguments,
    )?
    else {
        return Ok(None);
    };
    Ok(Some(ArgumentCallObservationV1::issue(
        owned,
        prior_homes.iter().copied().collect(),
        inner.into_boxed_slice(),
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
            super::ResolvedMethodCallReceiverSourceV1::Lexical(super::ResolvedLexicalRefV1::Local(
                _
            ))
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

#[path = "home_local_call_borrowed_actuals.rs"]
mod borrowed_actuals;
pub(super) use borrowed_actuals::observe_borrowed_call_actuals;
pub(crate) use borrowed_actuals::{BorrowedCallActualCandidateV1, BorrowedCallActualValueV1};
