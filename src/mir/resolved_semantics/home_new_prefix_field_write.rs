//! Receiver field-write admission inside `scan_statement_flow`.
//!
//! `me.<field> = <rhs>` is a Home-neutral statement when the resolver sealed
//! the target as a `FieldWrite` on this frame's self-rooted `me` and the RHS
//! subtree is provably free of `new` sites, call observations, map/array
//! literals, block expressions, and reads of owned bindings. Scalar
//! `me.<field>` reads inside the RHS are proven by the caller's issuer
//! predicate — the scanner never infers a field class from MIR types or
//! runtime layout. Anything else keeps `PrefixNotCovered`; this module mints
//! no ledger row and the raw lane's plain `FieldSet` emission is unchanged.
use super::*;
use crate::mir::resolved_semantics::{
    BodyExpressionShapeV1, BodyMeReceiverV1, ResolvedAssignmentFormV1,
    ResolvedAssignmentTargetV1, VerifiedResolvedBodyShapeInventoryV1,
};

/// `site` is a `me` expression bound to this frame's self-rooted borrowed
/// receiver — never a parameter handle and never static-box owner syntax.
fn self_rooted_me<'a>(
    shape: &'a VerifiedResolvedBodyShapeInventoryV1,
    site: &SourceExprSiteV1,
    locals: &PrefixLocalFlow<'_>,
) -> Option<BindingRefV1> {
    let BodyExpressionShapeV1::Me {
        receiver: BodyMeReceiverV1::Lexical(binding),
        ..
    } = shape.expression_shape(site)?
    else {
        return None;
    };
    locals.is_self_rooted_handle(*binding).then_some(*binding)
}

/// `true` when `statement` is an admitted self-rooted field write. Every
/// sealed expression row under the value subtree must be Home-neutral:
/// trivial locals/literals, unary/binary structure, receiver-position `me`,
/// or a `me.<field>` read the issuer predicate proves scalar.
pub(super) fn observe_receiver_field_write<E>(
    input: ResolvedFunctionLoweringInputV1<'_>,
    statement: &crate::mir::compiler::located::LocatedStmtV1<'_>,
    locals: &PrefixLocalFlow<'_>,
    scalar_field: &mut impl FnMut(
        &OwnedExprSiteV1,
        &SourceExprSiteV1,
        BindingRefV1,
        BindingRefV1,
        &str,
    ) -> Result<bool, E>,
) -> Result<bool, E> {
    if !matches!(statement.node(), ASTNode::Assignment { .. }) {
        return Ok(false);
    }
    let Some(shape) = input.body_shape() else {
        return Ok(false);
    };
    let Some(row) = shape
        .assignment_sources()
        .iter()
        .find(|row| row.statement_site() == statement.site())
    else {
        return Ok(false);
    };
    // Compound `op=` reads and writes the field — this bounded admission
    // covers the plain store only.
    if row.form() != ResolvedAssignmentFormV1::Plain {
        return Ok(false);
    }
    let Some(ResolvedAssignmentTargetV1::FieldWrite { receiver }) = input
        .function()
        .assignment_target(row.target_site())
    else {
        return Ok(false);
    };
    if self_rooted_me(shape, receiver, locals).is_none() {
        return Ok(false);
    }
    let prefix = row.value_site().node().segments();
    let subtree: Vec<&BodyExpressionShapeV1> = shape
        .expressions()
        .iter()
        .filter(|expression| {
            let segments = expression.site().node().segments();
            segments.len() >= prefix.len() && segments.starts_with(prefix)
        })
        .collect();
    // Receiver-position `me` nodes belong to their FieldAccess parent; a
    // `me` operand anywhere else is a self escape.
    let field_receivers: std::collections::BTreeSet<&SourceExprSiteV1> = subtree
        .iter()
        .filter_map(|expression| match expression {
            BodyExpressionShapeV1::FieldAccess { object, .. } => Some(object),
            _ => None,
        })
        .collect();
    for expression in subtree {
        match expression {
            BodyExpressionShapeV1::MapLiteral { .. }
            | BodyExpressionShapeV1::ArrayLiteral { .. }
            | BodyExpressionShapeV1::QualifiedReceiver { .. }
            | BodyExpressionShapeV1::MethodCall { .. }
            | BodyExpressionShapeV1::BlockExpr { .. } => return Ok(false),
            BodyExpressionShapeV1::Variable { site, .. } => {
                if !locals.observe(site).is_some_and(|row| row.is_trivial()) {
                    return Ok(false);
                }
            }
            BodyExpressionShapeV1::Me { site, .. } => {
                if !field_receivers.contains(site) {
                    return Ok(false);
                }
            }
            BodyExpressionShapeV1::FieldAccess {
                site,
                object,
                field,
            } => {
                let Some(home) = self_rooted_me(shape, object, locals) else {
                    return Ok(false);
                };
                if !scalar_field(
                    &OwnedExprSiteV1::new(input.owner(), site.clone()),
                    object,
                    home,
                    home,
                    field,
                )? {
                    return Ok(false);
                }
            }
            BodyExpressionShapeV1::Other { site, kind } => match &**kind {
                "UnaryOp" | "BinaryOp" => {}
                "Literal" => {
                    if !locals.observe(site).is_some_and(|row| row.is_trivial()) {
                        return Ok(false);
                    }
                }
                _ => return Ok(false),
            },
        }
    }
    Ok(true)
}
