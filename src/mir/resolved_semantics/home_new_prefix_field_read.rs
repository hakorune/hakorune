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
    /// Original selected field-call container read, without owned residence.
    DeclaredArray(std::rc::Rc<crate::mir::normal_callable_semantic_package::DeclaredArrayFieldReadSourceV1>),
    /// An ordinary-box declared type — the binding is a borrowed alias
    /// carrying the declared class name.
    Alias(Box<str>),
    /// Same source-issued field residence, borrowed without acquiring a Home.
    BorrowedArray(BorrowedArrayFieldSourceV1),
}

#[derive(Debug, Clone)]
pub(crate) struct BorrowedArrayFieldSourceV1 {
    pub(in crate::mir) requirement: std::rc::Rc<super::super::NamedArrayFieldResidenceRequirementV1>,
}
impl PartialEq for BorrowedArrayFieldSourceV1 {
    fn eq(&self, other: &Self) -> bool { std::rc::Rc::ptr_eq(&self.requirement, &other.requirement) }
}
impl Eq for BorrowedArrayFieldSourceV1 {}

/// Declaration-only callers keep their existing Scalar/Alias meaning. A whole
/// source-expression frame separately demands the original value proof on the
/// SAME plan; field layout alone never supplies that permission.
#[derive(Debug, Clone)]
pub(crate) enum LocalFieldReadDemandV1 {
    Declared { scalar_only: bool },
    SourceExpression {
        plan: std::rc::Rc<crate::mir::resolved_value_profile::expression_plan::ExpressionSourcePlanV1>,
    },
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
        LocalFieldReadDemandV1,
    ) -> Result<Option<Vec<LocalFieldReadResultV1>>, E>,
) -> Result<Option<LocalFieldReadResultV1>, E> {
    let Some(request) = field_read_request(input, site, locals) else {
        return Ok(None);
    };
    let Some(mut results) = local_field_read(&[request], LocalFieldReadDemandV1::Declared { scalar_only: false })? else {
        return Ok(None);
    };
    Ok((results.len() == 1).then(|| results.remove(0)))
}

/// Original received acquisition; no class, nonnull fact or Home is issued.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ReceivedFieldReadRequestV1 {
    Object { acquisition: OwnedExprSiteV1, nullable: bool },
    Mixed(Box<crate::mir::normal_callable_semantic_package::MixedLocalCallSourceLoanV1>),
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
    /// The receiver is a non-null-narrowed `OpaqueHandle` formal: the
    /// scanner proved the surviving path carries a live borrowed object,
    /// and the issuer must resolve the class from the co-sealed
    /// borrowed-formal object view — `alias_class` stays empty and the
    /// request never mints a release.
    pub(crate) formal: bool,
    pub(crate) received: Option<ReceivedFieldReadRequestV1>,
    pub(crate) path: Option<crate::mir::resolved_value_profile::expression_plan::OriginalExpressionFieldPathV1>,
    pub(crate) alias_path: Option<Box<crate::mir::resolved_semantics::home_new_prefix::FieldAliasPathV1>>,
    pub(crate) field: Box<str>,
}

impl LocalFieldReadRequestV1 {
    pub(crate) fn node_in_plan(&self, plan: &std::rc::Rc<crate::mir::resolved_value_profile::expression_plan::ExpressionSourcePlanV1>) -> Option<usize> {
        if self.site.owner() != plan.owner() { return None; }
        match &self.path {
            Some(path) => (std::rc::Rc::ptr_eq(path.plan(), plan) && path.site() == self.site
                && path.root() == self.receiver && path.receiver_site() == &self.receiver_site
                && path.name() == self.field.as_ref()).then_some(path.node()),
            None => plan.field_read_node_v1(self.site.site(), &self.receiver_site, self.receiver, &self.field),
        }
    }
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
    let mut received = None;
    let (home, alias_class, nullable, formal) = match provenance {
        local_flow::FieldReadReceiverV1::OwnedHome => (receiver, None, false, false),
        local_flow::FieldReadReceiverV1::RootedHandle(root) => (root, None, false, false),
        local_flow::FieldReadReceiverV1::Alias { class, root } => {
            (root, Some(class), false, false)
        }
        local_flow::FieldReadReceiverV1::ReceivedNullable => (receiver, None, true, false),
        local_flow::FieldReadReceiverV1::CheckedReceived(original) => {
            received = Some(original);
            (receiver, None, false, false)
        }
        local_flow::FieldReadReceiverV1::GuardedFormal => (receiver, None, false, true),
    };
    Some(LocalFieldReadRequestV1 {
        site: OwnedExprSiteV1::new(input.owner(), site.clone()),
        receiver_site: object.clone(),
        receiver,
        home,
        alias_class,
        nullable,
        formal,
        received,
        path: None,
        alias_path: None,
        field: field.clone(),
    })
}

/// A nested path keeps its genuine root binding; its immediate receiver is
/// another original field expression, never a fabricated local declaration.
pub(super) fn expression_field_read_request(
    input: ResolvedFunctionLoweringInputV1<'_>,
    plan: &std::rc::Rc<crate::mir::resolved_value_profile::expression_plan::ExpressionSourcePlanV1>,
    site: &SourceExprSiteV1, locals: &PrefixLocalFlow<'_>,
) -> Option<LocalFieldReadRequestV1> {
    let path = crate::mir::resolved_value_profile::expression_plan::OriginalExpressionFieldPathV1::issue(
        std::rc::Rc::clone(plan), site)?;
    if !path.is_nested_component() { return field_read_request(input, site, locals); }
    let mut request = field_read_request(input, path.first_site(), locals)?;
    if request.receiver != path.root() || input.owner() != plan.owner() { return None; }
    if request.alias_class.is_some() { request.alias_path = Some(Box::new(locals.field_alias_call_path(request.receiver)?)); }
    request.site = path.site();
    request.receiver_site = path.receiver_site().clone();
    request.field = path.name().into();
    request.path = Some(path);
    Some(request)
}
