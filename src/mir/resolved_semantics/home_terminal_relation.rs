//! Source-only terminal-return relations and the scalar classifier used by
//! the New-home prefix walk. No physical value, ABI, recipe, JSON, or backend
//! authority is issued here.
use super::local_flow::{OrdinaryObservation, PrefixLocalFlow};
use super::{
    BindingRefV1, ExprChildRoleV1, FunctionOwnerIdV1, OwnedExprSiteV1,
    ResolvedLexicalRefV1, ResolvedLiteralSourceV1, ResolvedMethodCallReceiverSourceV1,
    SourceExprSiteV1, SourceStmtSiteV1,
};
use crate::ast::ASTNode;
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::resolved_semantics::BodyExpressionShapeV1;

/// Source-only terminal shape for the selected ordinary-`New` root.
///
/// This records the decision made by the Completion ownership walk.  It has no
/// physical value, ABI, recipe, JSON, or backend authority.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TerminalI64AddReturnV1 {
    owner: FunctionOwnerIdV1,
    return_site: SourceStmtSiteV1,
    add_site: OwnedExprSiteV1,
    field_reads: [OwnedExprSiteV1; 2],
}

impl TerminalI64AddReturnV1 {
    pub(super) fn issue(
        owner: FunctionOwnerIdV1,
        return_site: SourceStmtSiteV1,
        add_site: OwnedExprSiteV1,
        field_reads: [OwnedExprSiteV1; 2],
    ) -> Self {
        Self {
            owner,
            return_site,
            add_site,
            field_reads,
        }
    }

    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(crate) fn return_site(&self) -> &SourceStmtSiteV1 {
        &self.return_site
    }

    pub(crate) fn add_site(&self) -> &OwnedExprSiteV1 {
        &self.add_site
    }

    pub(crate) fn field_reads(&self) -> &[OwnedExprSiteV1; 2] {
        &self.field_reads
    }
}

/// Exact source relation for a Completion-backed untyped integer literal return.
/// It records source identity and value only; it owns no physical representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TerminalIntegerLiteralReturnV1 {
    owner: FunctionOwnerIdV1,
    return_site: SourceStmtSiteV1,
    value_site: SourceExprSiteV1,
    value: i64,
}

/// Exact source relation for a Completion-backed direct selected i64 field
/// return. The referenced field-read row retains receiver/Home/field identity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TerminalI64FieldReturnV1 {
    owner: FunctionOwnerIdV1,
    return_site: SourceStmtSiteV1,
    value_site: SourceExprSiteV1,
    field_read_site: OwnedExprSiteV1,
}

impl TerminalI64FieldReturnV1 {
    pub(super) fn issue(
        owner: FunctionOwnerIdV1,
        return_site: SourceStmtSiteV1,
        value_site: SourceExprSiteV1,
        field_read_site: OwnedExprSiteV1,
    ) -> Self {
        Self {
            owner,
            return_site,
            value_site,
            field_read_site,
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
    pub(crate) fn field_read_site(&self) -> &OwnedExprSiteV1 {
        &self.field_read_site
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

/// The exact source value a non-i64 terminal `return <expr>` hands to the
/// caller. Binding classes carry the root identity; ownership transfer is
/// decided by the lifecycle contract, never by this row.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum TerminalReturnedSourceV1 {
    /// `return %{...}` — construction lives in the literal's MapHomeFlow
    /// row; this site joins the relation and the flow.
    MapLiteral(OwnedExprSiteV1),
    /// `return <binding>` — a live map-installed local or alias root.
    MapLocal(BindingRefV1),
    /// `return <binding>` — a live selected-New Home local; `acquisition`
    /// is its exact `new` site.
    Home {
        binding: BindingRefV1,
        acquisition: OwnedExprSiteV1,
    },
    /// `return <binding>` — a borrowed self-rooted handle root (parameter
    /// or declared handle); the caller keeps ownership.
    Handle(BindingRefV1),
    /// `return "<literal>"` — a string literal value.
    StringLiteral,
    /// `return null` — the null literal value.
    NullLiteral,
    /// `return <float>` — a float literal value.
    FloatLiteral,
}

/// Exact source relation for a non-i64 terminal `return <value>`. Records
/// which source value the site hands to the caller; no physical value, ABI,
/// recipe, JSON, or backend authority is issued.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TerminalValueReturnV1 {
    owner: FunctionOwnerIdV1,
    return_site: SourceStmtSiteV1,
    value_site: SourceExprSiteV1,
    returned: TerminalReturnedSourceV1,
}

impl TerminalValueReturnV1 {
    pub(super) fn issue(
        owner: FunctionOwnerIdV1,
        return_site: SourceStmtSiteV1,
        value_site: SourceExprSiteV1,
        returned: TerminalReturnedSourceV1,
    ) -> Self {
        Self {
            owner,
            return_site,
            value_site,
            returned,
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
    pub(crate) fn returned(&self) -> &TerminalReturnedSourceV1 {
        &self.returned
    }
}

/// Classify the exact source value a non-i64 `return <expr>` hands to the
/// caller. Literal classes come from the sealed expression-source inventory;
/// binding classes come from the running local flow. Trivial scalar locals
/// and i64-class forms are handled by `return_scalar` before this hook.
pub(super) fn terminal_returned_source(
    input: ResolvedFunctionLoweringInputV1<'_>,
    site: &SourceExprSiteV1,
    locals: &PrefixLocalFlow<'_>,
) -> Option<TerminalReturnedSourceV1> {
    match input.function().expression_source().literal(site) {
        Some(ResolvedLiteralSourceV1::String(_)) => {
            return Some(TerminalReturnedSourceV1::StringLiteral);
        }
        Some(ResolvedLiteralSourceV1::Null) => {
            return Some(TerminalReturnedSourceV1::NullLiteral);
        }
        Some(ResolvedLiteralSourceV1::Float) => {
            return Some(TerminalReturnedSourceV1::FloatLiteral);
        }
        _ => {}
    }
    let OrdinaryObservation::Handle(root) = locals.observe(site)? else {
        return None;
    };
    if locals.is_map_local(root) {
        Some(TerminalReturnedSourceV1::MapLocal(root))
    } else if locals.is_self_rooted_handle(root) {
        Some(TerminalReturnedSourceV1::Handle(root))
    } else {
        locals
            .home_acquisition(root)
            .map(|acquisition| TerminalReturnedSourceV1::Home {
                binding: root,
                acquisition: acquisition.clone(),
            })
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
    OtherTrivial,
    IntegerField(OwnedExprSiteV1),
    I64Add {
        site: OwnedExprSiteV1,
        field_reads: [OwnedExprSiteV1; 2],
    },
}

impl ReturnScalar {
    fn proves_integer(&self) -> bool {
        match self {
            Self::Integer | Self::IntegerField(_) | Self::I64Add { .. } => true,
            Self::OtherTrivial => false,
        }
    }
}

// This classifier is terminal-only: argument and prefix-local eligibility
// still belongs to the local-flow observation. Field authority is borrowed from the exact
// selected New's source definition, never from runtime layout or MIR types.
pub(super) fn return_scalar<E>(
    input: ResolvedFunctionLoweringInputV1<'_>,
    site: &SourceExprSiteV1,
    locals: &PrefixLocalFlow<'_>,
    field_is_integer: &mut impl FnMut(
        &OwnedExprSiteV1,
        &SourceExprSiteV1,
        BindingRefV1,
        BindingRefV1,
        &str,
    ) -> Result<bool, E>,
) -> Result<Option<ReturnScalar>, E> {
    if matches!(
        input.function().expression_source().literal(site),
        Some(ResolvedLiteralSourceV1::Integer(_))
    ) {
        return Ok(Some(ReturnScalar::Integer));
    }
    if locals.observe(site).is_some_and(|value| value.is_trivial()) {
        return Ok(Some(ReturnScalar::OtherTrivial));
    }
    let Ok(expr) = input
        .source()
        .expr_at(&OwnedExprSiteV1::new(input.owner(), site.clone()))
    else {
        return Ok(None);
    };
    match expr.node() {
        ASTNode::FieldAccess { field, .. } => {
            let Ok(receiver) = input
                .source()
                .child_expr_from_expr(&expr, ExprChildRoleV1::Receiver)
            else {
                return Ok(None);
            };
            let Some(OrdinaryObservation::Handle(home)) = locals.observe(receiver.site()) else {
                return Ok(None);
            };
            let Some(ResolvedLexicalRefV1::Local(binding)) =
                input.function().variable_ref(receiver.site())
            else {
                return Ok(None);
            };
            let field_site = OwnedExprSiteV1::new(input.owner(), site.clone());
            Ok(
                field_is_integer(&field_site, receiver.site(), binding, home, field)?
                    .then_some(ReturnScalar::IntegerField(field_site)),
            )
        }
        ASTNode::BinaryOp {
            operator: crate::ast::BinaryOperator::Add,
            ..
        } => {
            let Ok(left) = input
                .source()
                .child_expr_from_expr(&expr, ExprChildRoleV1::BinaryLeft)
            else {
                return Ok(None);
            };
            let Ok(right) = input
                .source()
                .child_expr_from_expr(&expr, ExprChildRoleV1::BinaryRight)
            else {
                return Ok(None);
            };
            let left = return_scalar(input, left.site(), locals, field_is_integer)?;
            let right = return_scalar(input, right.site(), locals, field_is_integer)?;
            let add_site = OwnedExprSiteV1::new(input.owner(), site.clone());
            match (left, right) {
                (
                    Some(ReturnScalar::IntegerField(left)),
                    Some(ReturnScalar::IntegerField(right)),
                ) => Ok(Some(ReturnScalar::I64Add {
                    site: add_site,
                    field_reads: [left, right],
                })),
                (Some(left), Some(right)) if left.proves_integer() && right.proves_integer() => {
                    Ok(Some(ReturnScalar::Integer))
                }
                _ => Ok(None),
            }
        }
        _ => Ok(None),
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
    pub(crate) fn call_site(&self) -> &SourceExprSiteV1 {
        &self.call_site
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

/// A terminal Call publishes its pending value only on Normal. On Fault the
/// original Completion supplies caller cleanup and outward propagation.
/// Target and argument sites stay in the package's existing affine Call row.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TerminalI64CallReturnV1 {
    owner: FunctionOwnerIdV1,
    return_site: SourceStmtSiteV1,
    call_site: SourceExprSiteV1,
    arguments: Box<[i64]>,
}
impl TerminalI64CallReturnV1 {
    pub(super) const fn issue(
        owner: FunctionOwnerIdV1,
        return_site: SourceStmtSiteV1,
        call_site: SourceExprSiteV1,
        arguments: Box<[i64]>,
    ) -> Self {
        Self {
            owner,
            return_site,
            call_site,
            arguments,
        }
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
    pub(crate) fn arguments(&self) -> &[i64] {
        &self.arguments
    }
}

/// One source-issued terminal relation. Absence stays outside this enum;
/// no ABI, physical progress or new source classification is issued here.
#[derive(Debug, Clone)]
pub(crate) enum TerminalRelationV1 {
    Call(TerminalI64CallReturnV1),
    I64Add(TerminalI64AddReturnV1),
    Unit(TerminalUnitReturnV1),
    IntegerLiteral(TerminalIntegerLiteralReturnV1),
    I64Field(TerminalI64FieldReturnV1),
    Value(TerminalValueReturnV1),
    OpaqueCall(TerminalOpaqueCallReturnV1),
    MapGet(TerminalMapGetReturnV1),
}

impl TerminalRelationV1 {
    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        match self {
            Self::Call(row) => row.owner,
            Self::I64Add(row) => row.owner,
            Self::Unit(row) => row.owner,
            Self::IntegerLiteral(row) => row.owner,
            Self::I64Field(row) => row.owner,
            Self::Value(row) => row.owner,
            Self::OpaqueCall(row) => row.owner,
            Self::MapGet(row) => row.owner,
        }
    }
}
