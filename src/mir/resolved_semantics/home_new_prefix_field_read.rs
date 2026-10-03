//! Local-initializer `receiver.field` read admission inside
//! `scan_statement_flow`.
//!
//! `local x = recv.field` is a covered prefix statement when the receiver
//! binding carries one of three provenances — a claim-local `new` Home, a
//! rooted handle binding (entry loan or parameter), or a binding produced
//! by an earlier proven field read — and the issuer predicate proves the
//! field declaration on that provenance's own class authority. The
//! declared type name decides the binding class: a numeric-integer scalar
//! installs `Trivial`, an ordinary-box name installs a `FieldAlias` — a
//! borrowed, non-observable handle into the receiver's storage.
//!
//! The direct-initializer adapter allows Scalar/Alias. The sibling pure
//! scalar-expression observer collects these same passive requests and
//! submits a Scalar-only batch after proving its complete root.
use super::*;
use crate::mir::resolved_semantics::{BodyExpressionShapeV1, BodyMeReceiverV1};

/// The sealed result class of a proven local-initializer field read.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum LocalFieldReadResultV1 {
    /// A numeric-integer declared type — the binding is a trivial scalar.
    Scalar,
    /// An ordinary-box declared type — the binding is a borrowed alias
    /// carrying the declared class name.
    Alias(Box<str>),
}

/// `Some(result)` when `site` is a proven `receiver.field` FieldAccess.
/// The scanner supplies the exact read site, the receiver's expression
/// site, its binding, and the stored-local provenance; the predicate alone
/// decides the field declaration and result class.
pub(super) fn observe_local_field_read<E>(
    input: ResolvedFunctionLoweringInputV1<'_>,
    site: &SourceExprSiteV1,
    locals: &PrefixLocalFlow<'_>,
    local_field_read: &mut impl FnMut(
        &[LocalFieldReadRequestV1],
        bool,
    ) -> Result<Option<Vec<LocalFieldReadResultV1>>, E>,
) -> Result<Option<LocalFieldReadResultV1>, E> {
    let Some(request) = field_read_request(input, site, locals) else {
        return Ok(None);
    };
    let Some(mut results) = local_field_read(&[request], false)? else {
        return Ok(None);
    };
    Ok((results.len() == 1).then(|| results.remove(0)))
}

/// Passive exact-site request; declaration proof and atomic staging belong
/// exclusively to the existing issuer, not to this source descriptor.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LocalFieldReadRequestV1 {
    pub(crate) site: OwnedExprSiteV1,
    pub(crate) receiver_site: SourceExprSiteV1,
    pub(crate) receiver: BindingRefV1,
    pub(crate) home: BindingRefV1,
    pub(crate) alias_class: Option<Box<str>>,
    /// The receiver is a non-null-narrowed received nullable: the scanner
    /// proved liveness, and the issuer must resolve the class from the
    /// sealed call's `NullableObject` claim — `alias_class` stays empty.
    pub(crate) nullable: bool,
    pub(crate) field: Box<str>,
}

pub(super) fn field_read_request(
    input: ResolvedFunctionLoweringInputV1<'_>,
    site: &SourceExprSiteV1,
    locals: &PrefixLocalFlow<'_>,
) -> Option<LocalFieldReadRequestV1> {
    let Some(shape) = input.body_shape() else {
        return None;
    };
    let Some(BodyExpressionShapeV1::FieldAccess { object, field, .. }) =
        shape.expression_shape(site)
    else {
        return None;
    };
    if object
        != &crate::mir::resolved_semantics::SourcePathV1::from_node(site.node())
            .child(crate::mir::resolved_semantics::SourcePathSegmentV1::Receiver)
            .expr()
    {
        return None;
    }
    let receiver = match input.function().variable_ref(object) {
        Some(ResolvedLexicalRefV1::Local(receiver)) => receiver,
        _ => match shape.expression_shape(object) {
            Some(BodyExpressionShapeV1::Me {
                receiver: BodyMeReceiverV1::Lexical(binding),
                ..
            }) => *binding,
            _ => return None,
        },
    };
    let Some(provenance) = locals.field_read_receiver(receiver) else {
        return None;
    };
    let (home, alias_class, nullable) = match provenance {
        local_flow::FieldReadReceiverV1::OwnedHome => (receiver, None, false),
        local_flow::FieldReadReceiverV1::RootedHandle(root) => (root, None, false),
        local_flow::FieldReadReceiverV1::Alias { class, root } => (root, Some(class), false),
        local_flow::FieldReadReceiverV1::ReceivedNullable => (receiver, None, true),
    };
    Some(LocalFieldReadRequestV1 {
        site: OwnedExprSiteV1::new(input.owner(), site.clone()),
        receiver_site: object.clone(),
        receiver,
        home,
        alias_class,
        nullable,
        field: field.clone(),
    })
}
