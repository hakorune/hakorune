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
//! Subtree positions (call arguments, operators, conditions) are out of
//! scope: this observer only admits the direct initializer of a `local`
//! statement. Anything else keeps `PrefixNotCovered`.
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
        &OwnedExprSiteV1,
        &SourceExprSiteV1,
        BindingRefV1,
        BindingRefV1,
        Option<&str>,
        &str,
    ) -> Result<Option<LocalFieldReadResultV1>, E>,
) -> Result<Option<LocalFieldReadResultV1>, E> {
    let Some(shape) = input.body_shape() else {
        return Ok(None);
    };
    let Some(BodyExpressionShapeV1::FieldAccess { object, field, .. }) =
        shape.expression_shape(site)
    else {
        return Ok(None);
    };
    let receiver = match input.function().variable_ref(object) {
        Some(ResolvedLexicalRefV1::Local(receiver)) => receiver,
        _ => match shape.expression_shape(object) {
            Some(BodyExpressionShapeV1::Me {
                receiver: BodyMeReceiverV1::Lexical(binding),
                ..
            }) => *binding,
            _ => return Ok(None),
        },
    };
    let Some(provenance) = locals.field_read_receiver(receiver) else {
        return Ok(None);
    };
    let (home, alias_class) = match provenance {
        local_flow::FieldReadReceiverV1::OwnedHome => (receiver, None),
        local_flow::FieldReadReceiverV1::RootedHandle(root) => (root, None),
        local_flow::FieldReadReceiverV1::Alias(class) => (receiver, Some(class)),
    };
    local_field_read(
        &OwnedExprSiteV1::new(input.owner(), site.clone()),
        object,
        receiver,
        home,
        alias_class.as_deref(),
        field,
    )
}
