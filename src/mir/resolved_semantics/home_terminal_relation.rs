//! Source-only terminal-return relations and the scalar classifier used by
//! the New-home prefix walk. No physical value, ABI, recipe, JSON, or backend
//! authority is issued here.
use super::local_flow::{OrdinaryObservation, PrefixLocalFlow};
use super::LocalFieldReadRequestV1;
use super::{
    BindingRefV1, ExprChildRoleV1, FunctionOwnerIdV1, HomePrefixUnavailableV1, OwnedExprSiteV1,
    ResolvedLexicalRefV1, ResolvedLiteralSourceV1, ResolvedMethodCallReceiverSourceV1,
    SourceExprSiteV1, SourceStmtSiteV1,
};
use crate::ast::ASTNode;
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::resolved_semantics::BodyExpressionShapeV1;

#[path = "home_terminal_returned_value.rs"]
mod returned_value;
pub(super) use returned_value::{
    finite_entry_return, observe_scalar_return_expression_v1 as returned_scalar_expression_v1,
    observe_stored_return_v1, observe_text_return_expression_v1, observe_receiver_field_return_v1,
    terminal_returned_source, observe_borrowed_call_return_v1,
};
pub(crate) use returned_value::{
    BorrowedNullableReturnV1, TerminalScalarReturnV1, TerminalReturnedSourceV1, TerminalValueReturnV1,
};



/// Exact source relation for a Completion-backed untyped integer literal return.
/// It records source identity and value only; it owns no physical representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TerminalIntegerLiteralReturnV1 {
    owner: FunctionOwnerIdV1,
    return_site: SourceStmtSiteV1,
    value_site: SourceExprSiteV1,
    value: i64,
}

/// Exact Bool literal at one Completion-backed Return site. This is source
/// identity and value, not a callable result class or physical ABI.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TerminalBoolLiteralReturnV1 {
    owner: FunctionOwnerIdV1,
    return_site: SourceStmtSiteV1,
    value_site: SourceExprSiteV1,
    value: bool,
}

impl TerminalBoolLiteralReturnV1 {
    pub(super) fn issue(
        owner: FunctionOwnerIdV1,
        return_site: SourceStmtSiteV1,
        value_site: SourceExprSiteV1,
        value: bool,
    ) -> Self {
        Self {
            owner,
            return_site,
            value_site,
            value,
        }
    }

    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }
    pub(crate) fn return_site(&self) -> &SourceStmtSiteV1 {
        &self.return_site
    }
    pub(crate) const fn value(&self) -> bool {
        self.value
    }
}


impl TerminalIntegerLiteralReturnV1 {
    pub(super) fn issue(
        owner: FunctionOwnerIdV1,
        return_site: SourceStmtSiteV1,
        value_site: SourceExprSiteV1,
        value: i64,
    ) -> Self {
        Self {
            owner,
            return_site,
            value_site,
            value,
        }
    }
    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }
    pub(crate) fn return_site(&self) -> &SourceStmtSiteV1 {
        &self.return_site
    }
    pub(crate) fn value_site(&self) -> &SourceExprSiteV1 {
        &self.value_site
    }
    pub(crate) const fn value(&self) -> i64 {
        self.value
    }
}

/// Issue the exact scalar root result for an AppMain with no selected Home
/// obligations. This uses the same verified completion and source loan as the
/// caller; it does not scan or reinterpret unrelated prefix statements.
pub(crate) fn issue_terminal_integer_literal_return_from_completion_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
    completion: &crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1,
) -> Result<Option<TerminalIntegerLiteralReturnV1>, HomePrefixUnavailableV1> {
    if completion.owner() != input.owner() {
        return Err(HomePrefixUnavailableV1::SourceMismatch);
    }
    let Some(return_site) = completion.explicit_site() else {
        return Ok(None);
    };
    let body = input
        .source()
        .root_body()
        .map_err(|_| HomePrefixUnavailableV1::SourceMismatch)?;
    let mut exact = None;
    for index in 0..body.statements().len() {
        let statement = input
            .source()
            .body_stmt(&body, index)
            .map_err(|_| HomePrefixUnavailableV1::SourceMismatch)?;
        if statement.site() == return_site {
            if exact.replace(statement).is_some() {
                return Err(HomePrefixUnavailableV1::SourceMismatch);
            }
        }
    }
    let Some(statement) = exact else {
        return Err(HomePrefixUnavailableV1::SourceMismatch);
    };
    if !matches!(statement.node(), ASTNode::Return { value: Some(_), .. }) {
        return Ok(None);
    }
    let value = input
        .source()
        .child_expr_from_stmt(&statement, ExprChildRoleV1::ReturnValue)
        .map_err(|_| HomePrefixUnavailableV1::SourceMismatch)?;
    let Some(ResolvedLiteralSourceV1::Integer(integer)) =
        input.function().expression_source().literal(value.site())
    else {
        return Ok(None);
    };
    Ok(Some(TerminalIntegerLiteralReturnV1::issue(
        input.owner(),
        return_site.clone(),
        value.site().clone(),
        *integer,
    )))
}

/// Exact source relation for a Completion-backed explicit bare return.
/// This contains no physical value or ABI category.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TerminalUnitReturnV1 {
    owner: FunctionOwnerIdV1,
    return_site: SourceStmtSiteV1,
}

impl TerminalUnitReturnV1 {
    pub(super) fn issue(owner: FunctionOwnerIdV1, return_site: SourceStmtSiteV1) -> Self {
        Self { owner, return_site }
    }
    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }
    pub(crate) fn return_site(&self) -> &SourceStmtSiteV1 {
        &self.return_site
    }
}

/// Sealed MapLiteral shape keys at an exact expression site. The shape rows
/// are membership facts; positions (initializer, return value, container slot)
/// stay the caller's concern.
pub(super) fn map_literal_keys<'a>(
    input: ResolvedFunctionLoweringInputV1<'a>,
    site: &SourceExprSiteV1,
) -> Option<&'a [Box<str>]> {
    input
        .body_shape()?
        .expressions()
        .iter()
        .find_map(|row| match row {
            BodyExpressionShapeV1::MapLiteral {
                site: map_site,
                keys,
            } if map_site == site => Some(keys.as_ref()),
            _ => None,
        })
}

/// Sealed ArrayLiteral element count at an exact expression site. Element
/// children are the sealed `Element(ordinal)` relations on the same shape.
pub(super) fn array_literal_element_count(
    input: ResolvedFunctionLoweringInputV1<'_>,
    site: &SourceExprSiteV1,
) -> Option<u32> {
    match input.body_shape()?.expression_shape(site)? {
        BodyExpressionShapeV1::ArrayLiteral { element_count, .. } => Some(*element_count),
        _ => None,
    }
}

#[derive(PartialEq, Eq)]
pub(super) enum ReturnScalar {
    Integer,
    /// A local whose stored kind is Bool (R16).
    Bool,
    OtherTrivial,
}

impl ReturnScalar {
    /// R16: the kind a returned local stores (`Bool` only for a Bool local).
    pub(super) fn stored_kind(&self) -> super::local_flow::SourceScalarKind {
        use super::local_flow::SourceScalarKind as Kind;
        if matches!(self, Self::Bool) { Kind::Bool } else { Kind::Integer }
    }
    fn proves_integer(&self) -> bool {
        match self {
            Self::Integer => true,
            Self::Bool | Self::OtherTrivial => false,
        }
    }
}

// This classifier is terminal-only: argument and prefix-local eligibility
// still belongs to the local-flow observation. Field authority is borrowed from the exact
// selected New's source definition, never from runtime layout or MIR types.
pub(super) fn return_scalar(
    input: ResolvedFunctionLoweringInputV1<'_>,
    site: &SourceExprSiteV1,
    locals: &PrefixLocalFlow<'_>,
) -> Option<ReturnScalar> {
    if matches!(
        input.function().expression_source().literal(site),
        Some(ResolvedLiteralSourceV1::Integer(_))
    ) {
        return Some(ReturnScalar::Integer);
    }
    match locals.observe(site) {
        // A local bound to a proven-i64 source returns an integer scalar:
        // the stored `SourceScalarKind` is the sole class authority —
        // `OtherTrivial` must never reclassify it.
        Some(OrdinaryObservation::TrivialLocal(
            _,
            Some(super::local_flow::SourceScalarKind::Integer),
        )) => {
            return Some(ReturnScalar::Integer);
        }
        Some(OrdinaryObservation::TrivialLocal(
            _,
            Some(super::local_flow::SourceScalarKind::Bool),
        )) => {
            return Some(ReturnScalar::Bool);
        }
        Some(value) if value.is_trivial() => {
            return Some(ReturnScalar::OtherTrivial);
        }
        _ => {}
    }
    let Ok(expr) = input
        .source()
        .expr_at(&OwnedExprSiteV1::new(input.owner(), site.clone()))
    else {
        return None;
    };
    match expr.node() {

        ASTNode::BinaryOp {
            operator: crate::ast::BinaryOperator::Add,
            ..
        } => {
            let Ok(left) = input
                .source()
                .child_expr_from_expr(&expr, ExprChildRoleV1::BinaryLeft)
            else {
                return None;
            };
            let Ok(right) = input
                .source()
                .child_expr_from_expr(&expr, ExprChildRoleV1::BinaryRight)
            else {
                return None;
            };
            let left = return_scalar(input, left.site(), locals);
            let right = return_scalar(input, right.site(), locals);
            match (left, right) {

                (Some(left), Some(right)) if left.proves_integer() && right.proves_integer() => {
                    Some(ReturnScalar::Integer)
                }
                _ => None,
            }
        }
        _ => None,
    }
}

/// Exact source relation for a `return <call>` whose sealed method-call row
/// proves a qualified receiver outside the lexical environment. This row
/// records only owner and sites: it owns no callee identity, result class,
/// argument handoff, Invoke emission, or ABI authority. It is not the affine
/// `Call` relation and it is not a `Value` source classification.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TerminalOpaqueCallReturnV1 {
    owner: FunctionOwnerIdV1,
    return_site: SourceStmtSiteV1,
    call_site: SourceExprSiteV1,
}

impl TerminalOpaqueCallReturnV1 {
    pub(super) const fn issue(
        owner: FunctionOwnerIdV1,
        return_site: SourceStmtSiteV1,
        call_site: SourceExprSiteV1,
    ) -> Self {
        Self {
            owner,
            return_site,
            call_site,
        }
    }
    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }
    pub(crate) fn return_site(&self) -> &SourceStmtSiteV1 {
        &self.return_site
    }
}

/// How the receiver of a terminal `<map>.get("...")` holds the map.
/// `OwnedLocal` reads a map the function still owes its own End for;
/// `BorrowedParameter` reads caller-owned storage under the checked-map
/// argument contract — it never gains or loses release responsibility.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TerminalMapGetReceiverClassV1 {
    OwnedLocal,
    BorrowedParameter,
}

/// Exact source relation for `return <map-binding>.get("<literal>")` — the
/// bounded readable-Map terminal. The row records owner, sites, receiver
/// binding/class and the sealed literal key only; it owns no physical
/// value, kernel symbol, ABI, recipe, JSON, or backend authority.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TerminalMapGetReturnV1 {
    owner: FunctionOwnerIdV1,
    return_site: SourceStmtSiteV1,
    call_site: OwnedExprSiteV1,
    receiver_site: SourceExprSiteV1,
    receiver: BindingRefV1,
    receiver_class: TerminalMapGetReceiverClassV1,
    key: Box<str>,
}

impl TerminalMapGetReturnV1 {
    pub(super) fn issue(
        owner: FunctionOwnerIdV1,
        return_site: SourceStmtSiteV1,
        call_site: OwnedExprSiteV1,
        receiver_site: SourceExprSiteV1,
        receiver: BindingRefV1,
        receiver_class: TerminalMapGetReceiverClassV1,
        key: Box<str>,
    ) -> Self {
        Self {
            owner,
            return_site,
            call_site,
            receiver_site,
            receiver,
            receiver_class,
            key,
        }
    }
    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }
    pub(crate) fn return_site(&self) -> &SourceStmtSiteV1 {
        &self.return_site
    }
    pub(crate) fn call_site(&self) -> &OwnedExprSiteV1 {
        &self.call_site
    }
    pub(crate) fn receiver_site(&self) -> &SourceExprSiteV1 {
        &self.receiver_site
    }
    pub(crate) const fn receiver(&self) -> BindingRefV1 {
        self.receiver
    }
    pub(crate) const fn receiver_class(&self) -> TerminalMapGetReceiverClassV1 {
        self.receiver_class
    }
    pub(crate) fn key(&self) -> &str {
        &self.key
    }
}

/// Classify `return <receiver>.get("<literal>")` on a live map binding.
/// The sealed method-call row is the receiver/selector/argument authority;
/// the running local flow decides owned vs borrowed — nothing here
/// reclassifies either side. Non-get selectors, non-literal keys, and
/// non-map receivers stay uncovered and fail at the caller's boundary.
///
/// Entry inspection is deliberately absent: every compilable map carries
/// tagged payloads (`I64`/`Bool`/`Text`/`EmptyArray`/`Residence`/
/// `BorrowedHandle`), so a checked read dispatches safely on any entry —
/// `I64` reads back its value, a missing key records Fault 114, and every
/// non-i64 payload records Fault 104. A call-returned map whose literal
/// flow row lives in the callee is readable for the same reason: the
/// callee could only have installed tagged payloads.
pub(super) fn terminal_map_get(
    input: ResolvedFunctionLoweringInputV1<'_>,
    return_site: &SourceStmtSiteV1,
    site: &SourceExprSiteV1,
    locals: &PrefixLocalFlow<'_>,
) -> Option<TerminalMapGetReturnV1> {
    let row = input
        .function()
        .method_calls()
        .find(|(call_site, _)| **call_site == *site)
        .map(|(_, row)| row)?;
    if row.owner() != input.owner() || row.selector() != "get" || row.arity() != 1 {
        return None;
    }
    let ResolvedMethodCallReceiverSourceV1::Lexical(ResolvedLexicalRefV1::Local(binding)) =
        row.receiver()
    else {
        return None;
    };
    let receiver_class = if locals.is_map_local(binding) {
        TerminalMapGetReceiverClassV1::OwnedLocal
    } else if locals.is_borrowed_map(binding) {
        TerminalMapGetReceiverClassV1::BorrowedParameter
    } else {
        return None;
    };
    let [argument] = row.arguments() else {
        return None;
    };
    let key = match input
        .function()
        .expression_source()
        .literal(argument.site())
    {
        Some(ResolvedLiteralSourceV1::String(text)) => text.clone(),
        _ => return None,
    };
    Some(TerminalMapGetReturnV1::issue(
        input.owner(),
        return_site.clone(),
        OwnedExprSiteV1::new(input.owner(), site.clone()),
        row.receiver_site().clone(),
        binding,
        receiver_class,
        key,
    ))
}

/// One sealed terminal-Call argument class. `I64` carries the literal value
/// the scalar lane re-materializes; `Map` names the exact `%{...}` literal
/// site whose `CallArgument` flow row carries the construction facts — the
/// row records the source class only, never the pointer it becomes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum TerminalCallArgumentV1 {
    I64(i64),
    Map(OwnedExprSiteV1),
    /// Original ordered projection; package retains opaque domain authority.
    Lexical(super::local_call_flow::LocalCallArgumentV1),
}
impl TerminalCallArgumentV1 {
    pub(crate) const fn map_site(&self) -> Option<&OwnedExprSiteV1> {
        match self {
            Self::I64(_) | Self::Lexical(_) => None,
            Self::Map(site) => Some(site),
        }
    }
}

/// A terminal Call publishes its pending value only on Normal. On Fault the
/// original Completion supplies caller cleanup and outward propagation.
/// Target and argument sites stay in the package's existing affine Call row.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TerminalI64CallReturnV1 {
    owner: FunctionOwnerIdV1,
    return_site: SourceStmtSiteV1,
    call_site: SourceExprSiteV1,
    arguments: Box<[TerminalCallArgumentV1]>,
    source_scalar:
        Option<crate::mir::normal_callable_semantic_package::BorrowedScalarResultSourceLoanV1>,
}
impl TerminalI64CallReturnV1 {
    pub(super) const fn issue(
        owner: FunctionOwnerIdV1,
        return_site: SourceStmtSiteV1,
        call_site: SourceExprSiteV1,
        arguments: Box<[TerminalCallArgumentV1]>,
    ) -> Self {
        Self {
            owner,
            return_site,
            call_site,
            arguments,
            source_scalar: None,
        }
    }
    pub(super) fn issue_source_scalar(
        owner: FunctionOwnerIdV1,
        return_site: SourceStmtSiteV1,
        call_site: SourceExprSiteV1,
        arguments: Box<[TerminalCallArgumentV1]>,
        loan: crate::mir::normal_callable_semantic_package::BorrowedScalarResultSourceLoanV1,
    ) -> Self {
        assert_eq!(
            loan.source().call_site(),
            &super::OwnedExprSiteV1::new(owner, call_site.clone())
        );
        Self {
            owner,
            return_site,
            call_site,
            arguments,
            source_scalar: Some(loan),
        }
    }
    pub(crate) fn source_scalar(
        &self,
    ) -> Option<&crate::mir::normal_callable_semantic_package::BorrowedScalarResultSourceLoanV1>
    {
        self.source_scalar.as_ref()
    }
    pub(crate) fn scalar_kind(&self) -> super::local_flow::SourceScalarKind {
        self.source_scalar
            .as_ref()
            .map(|loan| loan.kind())
            .unwrap_or(super::local_flow::SourceScalarKind::Integer)
    }
    pub(crate) fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }
    pub(crate) fn return_site(&self) -> &SourceStmtSiteV1 {
        &self.return_site
    }
    pub(crate) fn call_site(&self) -> &SourceExprSiteV1 {
        &self.call_site
    }
    pub(crate) fn arguments(&self) -> &[TerminalCallArgumentV1] {
        &self.arguments
    }
}

/// One source-issued terminal relation. Absence stays outside this enum;
/// no ABI, physical progress or new source classification is issued here.
#[derive(Debug, Clone)]
pub(crate) enum TerminalRelationV1 {
    Call(TerminalI64CallReturnV1),
    Unit(TerminalUnitReturnV1),
    IntegerLiteral(TerminalIntegerLiteralReturnV1),
    BoolLiteral(TerminalBoolLiteralReturnV1),
    Scalar(TerminalScalarReturnV1),
    Value(TerminalValueReturnV1),
    OpaqueCall(TerminalOpaqueCallReturnV1),
    MapGet(TerminalMapGetReturnV1),
}

impl TerminalRelationV1 {
    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        match self {
            Self::Call(row) => row.owner,
            Self::Unit(row) => row.owner,
            Self::IntegerLiteral(row) => row.owner,
            Self::BoolLiteral(row) => row.owner,
            Self::Scalar(row) => row.owner(),
            Self::Value(row) => row.owner(),
            Self::OpaqueCall(row) => row.owner,
            Self::MapGet(row) => row.owner,
        }
    }

    /// The scalar class this exit's relation proves, or `None` when it proves no scalar (Unit, a Text
    /// or Map value, an opaque call, or a call with no scalar source loan). The one table both the
    /// declaration check (R2) and the finalized-participant check read.
    pub(crate) fn scalar_class(&self) -> Option<super::local_flow::SourceScalarKind> {
        use super::local_flow::SourceScalarKind as Kind;
        match self {
            Self::IntegerLiteral(_) | Self::MapGet(_) => Some(Kind::Integer),
            Self::Scalar(row) => Some(row.scalar_kind()),
            Self::BoolLiteral(_) => Some(Kind::Bool),
            Self::Call(call) => call.source_scalar().map(|_| call.scalar_kind()),
            Self::Unit(_) | Self::Value(_) | Self::OpaqueCall(_) => None,
        }
    }

    /// The exact source exit statement this relation terminates. Every row
    /// kind carries the same field; keying consumers by this site is how
    /// sibling exits stay independent.
    pub(crate) fn return_site(&self) -> &SourceStmtSiteV1 {
        match self {
            Self::Call(row) => row.return_site(),
            Self::Unit(row) => row.return_site(),
            Self::IntegerLiteral(row) => row.return_site(),
            Self::BoolLiteral(row) => row.return_site(),
            Self::Scalar(row) => row.return_site(),
            Self::Value(row) => row.return_site(),
            Self::OpaqueCall(row) => row.return_site(),
            Self::MapGet(row) => row.return_site(),
        }
    }
}
