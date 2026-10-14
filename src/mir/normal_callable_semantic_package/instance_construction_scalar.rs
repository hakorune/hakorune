//! Existing scalar store-fold helpers under the sole construction issuer.
use super::{ConstructionStoreRhsV1, ConstructionStoreV1, ConstructionUnavailableV1};
use crate::ast::{ASTNode, LiteralValue};
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::resolved_semantics::{
    BindingRefV1, BodyExpressionShapeV1, BodyMeReceiverV1, OwnedExprSiteV1,
    SourceExprSiteV1, VerifiedResolvedBodyShapeInventoryV1,
};
use hakorune_mir_defs::{CanonicalFieldRefV1, CanonicalObjectIdV1};

/// Newest already-sealed store for `field` in this constructor.
pub(super) fn prior_store_rhs(
    stores: &[ConstructionStoreV1],
    field: CanonicalFieldRefV1,
) -> Option<&ConstructionStoreRhsV1> {
    stores
        .iter()
        .rev()
        .find(|store| store.field() == field)
        .map(|store| store.rhs())
}

/// Corroborate a `me.<field>` read site: the shape FieldAccess row
/// claims the site, its receiver is this constructor's `me` binding,
/// and the field name belongs to the declaring box. Returns the
/// canonical field plus the `me` receiver site for coverage.
pub(super) fn me_field_read_row(
    shape: &VerifiedResolvedBodyShapeInventoryV1,
    receiver: BindingRefV1,
    access_site: &SourceExprSiteV1,
    object_id: CanonicalObjectIdV1,
    fields: &[String],
) -> Result<(CanonicalFieldRefV1, SourceExprSiteV1), ConstructionUnavailableV1> {
    use ConstructionUnavailableV1 as U;
    let (object, field) = shape
        .expressions()
        .iter()
        .find_map(|expression| match expression {
            BodyExpressionShapeV1::FieldAccess { site, object, field }
                if site == access_site =>
            {
                Some((object, field))
            }
            _ => None,
        })
        .ok_or(U::SourceRelationMissing)?;
    if !shape.expressions().iter().any(|expression| {
        matches!(expression,
        BodyExpressionShapeV1::Me { site, receiver: BodyMeReceiverV1::Lexical(binding) }
            if site == object && *binding == receiver)
    }) {
        return Err(U::BodyCoverageUnsupported);
    }
    let ordinal = fields
        .iter()
        .position(|name| name == field.as_ref())
        .ok_or(U::SourceRelationMissing)?;
    let field = CanonicalFieldRefV1::from_declaration_ordinal(object_id, ordinal)
        .ok_or(U::SourceRelationMissing)?;
    Ok((field, object.clone()))
}

/// One BinOp operand sealed to an i64 literal: a direct integer
/// literal or a `me.<field>` read resolving to a prior `LiteralI64`
/// store. `Parameter`/object-field reads stay unsupported so folding
/// never replaces runtime semantics. Returned sites are the coverage
/// rows the caller registers (operand site plus any `me` site) and the
/// `me` receiver sites the store must observe at take time.
pub(super) fn scalar_literal_operand(
    input: &ResolvedFunctionLoweringInputV1<'_>,
    shape: &VerifiedResolvedBodyShapeInventoryV1,
    receiver: BindingRefV1,
    object_id: CanonicalObjectIdV1,
    fields: &[String],
    stores: &[ConstructionStoreV1],
    site: &SourceExprSiteV1,
) -> Result<(i64, Vec<SourceExprSiteV1>, Vec<SourceExprSiteV1>), ConstructionUnavailableV1>
{
    use ConstructionUnavailableV1 as U;
    let node = input
        .source()
        .expr_at(&OwnedExprSiteV1::new(input.owner(), site.clone()))
        .map_err(|_| U::SourceRelationMissing)?;
    let mut sites = vec![site.clone()];
    match node.node() {
        ASTNode::Literal {
            value: LiteralValue::Integer(value),
            ..
        } => Ok((*value, sites, Vec::new())),
        ASTNode::FieldAccess { .. } => {
            let (field, me_site) =
                me_field_read_row(shape, receiver, site, object_id, fields)?;
            sites.push(me_site.clone());
            match prior_store_rhs(stores, field) {
                Some(ConstructionStoreRhsV1::LiteralI64(value)) => {
                    Ok((*value, sites, vec![me_site]))
                }
                _ => Err(U::BodyCoverageUnsupported),
            }
        }
        _ => Err(U::BodyCoverageUnsupported),
    }
}
