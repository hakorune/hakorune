//! Shared structural expression view over the original callable source ledger.
//! This selects no value class, call target, Home, Recipe or execution permission.
use crate::mir::resolved_semantics::{
    BodyExpressionShapeV1, CallableSemanticSourceLedgerView, ResolvedBinaryExpressionSourceV1,
    ResolvedConditionalExpressionSourceV1, ResolvedLexicalRefV1, ResolvedLiteralSourceV1,
    ResolvedUnaryExpressionSourceV1, SourceExprSiteV1, VerifiedResolvedBodyShapeInventoryV1,
    VerifiedResolvedMethodCallSourceV1,
};

#[derive(Debug)]
pub(crate) enum ResolvedExpressionSourceV1<'a> {
    Conditional(&'a ResolvedConditionalExpressionSourceV1),
    Binary(&'a ResolvedBinaryExpressionSourceV1),
    Unary(&'a ResolvedUnaryExpressionSourceV1),
    Literal(&'a ResolvedLiteralSourceV1),
    Reference(ResolvedLexicalRefV1),
    MethodCall(&'a VerifiedResolvedMethodCallSourceV1),
    DirectCall,
    Shape(&'a BodyExpressionShapeV1),
}

impl<'a> ResolvedExpressionSourceV1<'a> {
    pub(crate) fn binary(self) -> Option<&'a ResolvedBinaryExpressionSourceV1> {
        match self {
            Self::Binary(row) => Some(row),
            _ => None,
        }
    }
}

/// Borrow the resolver's original row. Shape-only expressions retain their
/// unresolved form: a field name, receiver or construction shape is not a
/// proof of its result or its execution. Call consumers still need the original
/// target/result/input issuers, including a direct-call observation.
pub(crate) fn expression_source_v1<'a>(
    ledger: &'a CallableSemanticSourceLedgerView<'_>,
    shape: Option<&'a VerifiedResolvedBodyShapeInventoryV1>,
    site: &SourceExprSiteV1,
) -> Option<ResolvedExpressionSourceV1<'a>> {
    if shape.is_some_and(|shape| shape.owner() != ledger.owner()) {
        return None;
    }
    Some(if let Some(row) = ledger.conditional_source(site) {
        ResolvedExpressionSourceV1::Conditional(row)
    } else if let Some(row) = ledger.binary_source(site) {
        ResolvedExpressionSourceV1::Binary(row)
    } else if let Some(row) = ledger.unary_source(site) {
        ResolvedExpressionSourceV1::Unary(row)
    } else if let Some(row) = ledger.literal_source(site) {
        ResolvedExpressionSourceV1::Literal(row)
    } else if let Some(row) = ledger.variable_ref(site) {
        ResolvedExpressionSourceV1::Reference(row)
    } else if let Some(row) = ledger.method_call(site) {
        ResolvedExpressionSourceV1::MethodCall(row)
    } else if ledger.direct_call_target(site).is_some()
        || ledger.direct_call_observation(site).is_some()
    {
        ResolvedExpressionSourceV1::DirectCall
    } else {
        ResolvedExpressionSourceV1::Shape(shape?.expression_shape(site)?)
    })
}
