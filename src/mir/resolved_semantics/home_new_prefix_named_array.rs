//! Conditional named-Array Homes in the same source walk. No storage, Normal
//! producer, commit, borrowed value or End execution capability is issued here.
use crate::mir::resolved_semantics::home_new_prefix::LiveHomeV1;
use super::*;
use crate::mir::normal_callable_semantic_package::NamedArrayHolderSourceLoanV1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum NamedArrayHomeObservationV1 {
    FieldLoop { source: FieldArrayLoopHomeSourceV1 },
    Initializer {
        source: NamedArrayHolderSourceLoanV1,
        prefix: Result<CallerNewHomePrefixV1, HomePrefixUnavailableV1>,
    },
    Append {
        source: NamedArrayHolderSourceLoanV1,
        call: OwnedExprSiteV1,
        candidate: OwnedExprSiteV1,
        availability: Result<(), HomePrefixUnavailableV1>,
        prior_homes: Box<[LiveHomeV1]>,
        covered_statements: Box<[SourceStmtSiteV1]>,
    },
}

/// Identity-preserving loan of one Home-installed original Loop proof.
/// This carries no physical append, entry permission or runtime representation.
#[derive(Debug, Clone)]
pub(crate) struct FieldArrayLoopHomeSourceV1(std::rc::Rc<crate::mir::builder::VerifiedNoExitArrayHomeAfterV1>);
impl PartialEq for FieldArrayLoopHomeSourceV1 {
    fn eq(&self, other: &Self) -> bool { std::rc::Rc::ptr_eq(&self.0, &other.0) }
}
impl Eq for FieldArrayLoopHomeSourceV1 {}
impl NamedArrayHomeObservationV1 {
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn field_loop(
        proof: std::rc::Rc<crate::mir::builder::VerifiedNoExitArrayHomeAfterV1>,
    ) -> Self { Self::FieldLoop { source: FieldArrayLoopHomeSourceV1(proof) } }
    pub(crate) fn field_loop_proof(&self) -> Option<&std::rc::Rc<crate::mir::builder::VerifiedNoExitArrayHomeAfterV1>> {
        match self { Self::FieldLoop { source } => Some(&source.0), _ => None }
    }
}

pub(super) fn observe_initializer<E>(
    input: ResolvedFunctionLoweringInputV1<'_>,
    site: &OwnedExprSiteV1,
    binding: BindingRefV1,
    locals: &mut PrefixLocalFlow<'_>,
    homes: &mut Vec<LiveHomeV1>,
    covered: &[SourceStmtSiteV1],
    unavailable: &mut Option<HomePrefixUnavailableV1>,
    borrow: &mut impl FnMut(
        &OwnedExprSiteV1,
        BorrowedCallActualRequestV1<'_>,
    ) -> Result<Option<BorrowedCallArgumentsV1>, E>,
) -> Result<bool, E> {
    if !input
        .source()
        .expr_at(site)
        .is_ok_and(|located| matches!(located.node(), ASTNode::New { .. }))
    {
        return Ok(false);
    }
    let response = borrow(
        site,
        BorrowedCallActualRequestV1::NamedArrayHolderInitializer(binding),
    )?;
    let Some(response) = response else {
        return Ok(false);
    };
    let BorrowedCallArgumentsV1::NamedArrayHolderSource(source) = response else {
        unavailable.get_or_insert(HomePrefixUnavailableV1::SourceMismatch);
        return Ok(true);
    };
    if source.binding() != binding
        || source.construction() != *site
        || input.owner() != binding.owner()
        || homes.iter().any(|home| home.binding() == binding)
    {
        unavailable.get_or_insert(HomePrefixUnavailableV1::SourceMismatch);
        return Ok(true);
    }
    let prefix = match unavailable.as_ref() {
        Some(issue) => Err(issue.clone()),
        None => issue_new_fault_continuation_v1(input, site)
            .map_err(|_| HomePrefixUnavailableV1::SourceMismatch)
            .map(|outward_fault| CallerNewHomePrefixV1 {
                destination: binding,
                prior_homes: homes.iter().rev().cloned().collect(),
                outward_fault,
                covered_statements: covered.to_vec().into_boxed_slice(),
            }),
    };
    if let Err(issue) = &prefix {
        unavailable.get_or_insert(issue.clone());
    }
    locals.install_named_array_holder(source.clone());
    locals.retain_named_array_observation(NamedArrayHomeObservationV1::Initializer {
        source,
        prefix,
    });
    // Conditional source obligation on the construction's Normal successor.
    homes.push(LiveHomeV1::Stable(binding));
    Ok(true)
}

pub(super) fn observe_append<E>(
    input: ResolvedFunctionLoweringInputV1<'_>,
    statement: &crate::mir::compiler::located::LocatedStmtV1<'_>,
    locals: &mut PrefixLocalFlow<'_>,
    homes: &[LiveHomeV1],
    covered: &[SourceStmtSiteV1],
    unavailable: &mut Option<HomePrefixUnavailableV1>,
    borrow: &mut impl FnMut(
        &OwnedExprSiteV1,
        BorrowedCallActualRequestV1<'_>,
    ) -> Result<Option<BorrowedCallArgumentsV1>, E>,
) -> Result<bool, E> {
    if !matches!(statement.node(), ASTNode::MethodCall { .. }) {
        return Ok(false);
    }
    let call = OwnedExprSiteV1::new(
        input.owner(),
        SourceExprSiteV1::from_node(statement.site().node().clone()),
    );
    let Some(receiver) = input
        .function()
        .method_calls()
        .find(|(site, _)| *site == call.site())
        .map(|(_, row)| row.receiver())
    else {
        return Ok(false);
    };
    let ResolvedMethodCallReceiverSourceV1::Lexical(ResolvedLexicalRefV1::Local(binding)) =
        receiver
    else {
        return Ok(false);
    };
    if !locals.is_named_array_holder(binding) {
        return Ok(false);
    }
    let response = borrow(
        &call,
        BorrowedCallActualRequestV1::NamedArrayAppendStatement(binding),
    )?;
    let Some(response) = response else {
        return Ok(false);
    };
    let BorrowedCallArgumentsV1::NamedArrayHolderSource(source) = response else {
        unavailable.get_or_insert(HomePrefixUnavailableV1::SourceMismatch);
        return Ok(true);
    };
    let Some(candidate) = source.append_candidate(&call) else {
        unavailable.get_or_insert(HomePrefixUnavailableV1::SourceMismatch);
        return Ok(true);
    };
    if !homes.iter().any(|home| home.binding() == source.binding()) || !locals.has_named_array_holder(&source) {
        unavailable.get_or_insert(HomePrefixUnavailableV1::SourceMismatch);
        return Ok(true);
    }
    let observation = NamedArrayHomeObservationV1::Append {
        source,
        call,
        candidate,
        availability: unavailable.as_ref().cloned().map_or(Ok(()), Err),
        prior_homes: homes.iter().rev().cloned().collect(),
        covered_statements: covered.to_vec().into_boxed_slice(),
    };
    locals.record_source_loop_array_append_v1(statement.site(), &observation);
    locals.retain_named_array_observation(observation);
    // The source store is covered; actual candidate ownership and successful
    // commit must still be established by the existing acquisition/physical owner.
    Ok(true)
}
