//! Source-only terminal-return relations and the scalar classifier used by
//! the New-home prefix walk. No physical value, ABI, recipe, JSON, or backend
//! authority is issued here.
use super::local_flow::{OrdinaryObservation, PrefixLocalFlow};
use super::{
    BindingRefV1, ExprChildRoleV1, FunctionOwnerIdV1, OwnedExprSiteV1, ResolvedLexicalRefV1,
    ResolvedLiteralSourceV1, SourceExprSiteV1, SourceStmtSiteV1,
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
}

impl TerminalRelationV1 {
    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        match self {
            Self::Call(row) => row.owner,
            Self::I64Add(row) => row.owner,
            Self::Unit(row) => row.owner,
            Self::IntegerLiteral(row) => row.owner,
            Self::I64Field(row) => row.owner,
        }
    }
}
