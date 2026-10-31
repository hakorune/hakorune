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
use crate::mir::resolved_semantics::home_new_prefix::LiveHomeV1;
use crate::mir::resolved_semantics::{BodyExpressionShapeV1, BodyMeReceiverV1};

/// The sealed result class of a proven local-initializer field read.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum LocalFieldReadResultV1 {
    /// Same closed all-writer class; a checked snapshot must still acquire
    /// an independent Text result on Normal before any owned use.
    Text(std::rc::Rc<crate::mir::normal_callable_semantic_package::TextFieldValueSourceV1>),
    /// A numeric-integer declared type — the binding is a trivial scalar.
    Scalar,
    /// Original selected field-call container read, without owned residence.
    ArrayField(
        std::rc::Rc<crate::mir::normal_callable_semantic_package::CanonicalArrayFieldReadSourceV1>,
    ),
    /// An ordinary-box declared type — the binding is a borrowed alias
    /// carrying the declared class name.
    Alias(Box<str>),
    /// Same source-issued field residence, borrowed without acquiring a Home.
    BorrowedArray(BorrowedArrayFieldSourceV1),
    /// Complete original Dynamic child cohort; observation acquires no Home.
    BorrowedObject(std::rc::Rc<crate::mir::normal_callable_semantic_package::DynamicStoredChildFieldV1>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ScalarFieldReadSourceV1 {
    binding: BindingRefV1,
    request: LocalFieldReadRequestV1,
    statement: SourceStmtSiteV1,
}

impl ScalarFieldReadSourceV1 {
    fn issue(
        input: ResolvedFunctionLoweringInputV1<'_>,
        binding: BindingRefV1,
        request: LocalFieldReadRequestV1,
    ) -> Option<Self> {
        let site = request.site.site();
        if binding.owner() != input.owner()
            || request.site.owner() != input.owner()
            || request.home.owner() != input.owner()
            || request.receiver.owner() != input.owner()
        {
            return None;
        }
        let mut declarations = input
            .function()
            .expression_source()
            .initializers()
            .filter(|row| row.binding() == binding && row.initializer_site() == Some(site));
        let declaration = declarations.next()?;
        if declarations.next().is_some() {
            return None;
        }
        let statement = input
            .function()
            .source_site_inventory()
            .containing_statement(site)?;
        let declaration_matches = matches!(
            declaration.declaration_site(),
            SourceBindingSiteV1::Local { statement: declared, .. } if declared == statement
        );
        declaration_matches.then_some(Self {
            binding,
            request,
            statement: statement.clone(),
        })
    }

    pub(crate) fn matches_initializer_v1(
        &self,
        binding: BindingRefV1,
        site: &SourceExprSiteV1,
    ) -> bool {
        self.binding == binding
            && self.request.site.owner() == binding.owner()
            && self.request.site.site() == site
    }

    pub(crate) fn request_site_v1(&self) -> &SourceExprSiteV1 {
        self.request.site.site()
    }

    pub(crate) fn site_v1(&self) -> &OwnedExprSiteV1 {
        &self.request.site
    }

    pub(crate) fn statement_v1(&self) -> &SourceStmtSiteV1 {
        &self.statement
    }
}

#[derive(Debug)]
pub(super) struct ObservedLocalFieldReadV1 {
    request: LocalFieldReadRequestV1,
    result: LocalFieldReadResultV1,
}

#[derive(Debug, Clone)]
pub(crate) struct BorrowedArrayFieldSourceV1 {
    origin: BorrowedArrayFieldOriginV1,
}
#[derive(Debug, Clone)]
enum BorrowedArrayFieldOriginV1 {
    Operation(std::rc::Rc<super::super::NamedArrayFieldResidenceRequirementV1>),
    FixedAlias {
        field: std::rc::Rc<crate::mir::normal_callable_semantic_package::CanonicalArrayFieldReadSourceV1>,
        initializer: super::super::ResolvedInitializerRelationV1,
    },
}
impl BorrowedArrayFieldSourceV1 {
    pub(in crate::mir) fn operation_v1(
        requirement: std::rc::Rc<super::super::NamedArrayFieldResidenceRequirementV1>,
    ) -> Self { Self { origin: BorrowedArrayFieldOriginV1::Operation(requirement) } }
    /// Original local association only; this lends no operation or Home.
    pub(in crate::mir) fn fixed_alias_v1(
        field: std::rc::Rc<crate::mir::normal_callable_semantic_package::CanonicalArrayFieldReadSourceV1>,
        initializer: super::super::ResolvedInitializerRelationV1,
    ) -> Option<Self> {
        if initializer.initializer_site() != Some(field.site().site())
            || initializer.binding().owner() != field.site().owner() { return None; }
        Some(Self { origin: BorrowedArrayFieldOriginV1::FixedAlias { field, initializer } })
    }
    pub(in crate::mir) fn fixed_source_v1(&self) -> Option<(
        &std::rc::Rc<crate::mir::normal_callable_semantic_package::CanonicalArrayFieldReadSourceV1>,
        &super::super::ResolvedInitializerRelationV1,
    )> {
        match &self.origin {
            BorrowedArrayFieldOriginV1::FixedAlias { field, initializer } => Some((field, initializer)),
            _ => None,
        }
    }
    pub(in crate::mir) fn binding(&self) -> BindingRefV1 {
        match &self.origin {
            BorrowedArrayFieldOriginV1::Operation(source) => source.binding(),
            BorrowedArrayFieldOriginV1::FixedAlias { initializer, .. } => initializer.binding(),
        }
    }
    pub(in crate::mir) fn integer_requirement_v1(&self)
        -> Option<&std::rc::Rc<super::super::NamedArrayFieldResidenceRequirementV1>> {
        match &self.origin { BorrowedArrayFieldOriginV1::Operation(source) => Some(source), _ => None }
    }
}
impl PartialEq for BorrowedArrayFieldSourceV1 {
    fn eq(&self, other: &Self) -> bool {
        match (&self.origin, &other.origin) {
            (BorrowedArrayFieldOriginV1::Operation(a), BorrowedArrayFieldOriginV1::Operation(b)) => std::rc::Rc::ptr_eq(a, b),
            (BorrowedArrayFieldOriginV1::FixedAlias { field: a, initializer: ai },
             BorrowedArrayFieldOriginV1::FixedAlias { field: b, initializer: bi }) => a == b && ai == bi,
            _ => false,
        }
    }
}
impl Eq for BorrowedArrayFieldSourceV1 {}

/// Declaration-only callers keep their existing Scalar/Alias meaning. A whole
/// source-expression frame separately demands the original value proof on the
/// SAME plan; field layout alone never supplies that permission.
#[derive(Debug, Clone)]
pub(crate) enum LocalFieldReadDemandV1 {
    Declared {
        scalar_only: bool,
    },
    SourceExpression {
        probe: bool,
        plan: std::rc::Rc<
            crate::mir::resolved_value_profile::expression_plan::ExpressionSourcePlanV1,
        >,
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
    homes: &[LiveHomeV1],
    local_field_read: &mut impl FnMut(
        &[LocalFieldReadRequestV1],
        LocalFieldReadDemandV1,
    ) -> Result<Option<Vec<LocalFieldReadResultV1>>, E>,
) -> Result<Option<ObservedLocalFieldReadV1>, E> {
    let Some(request) = field_read_request(input, site, locals) else {
        return Ok(None);
    };
    let plan = if request.owned_ready.is_some() || request.shared_support.is_some() {
        let Ok(ledger) = input.forest().callable_source_ledger(input.owner()) else {
            return Ok(None);
        };
        let mut rows = input
            .function()
            .expression_source()
            .initializers()
            .filter(|row| row.initializer_site() == Some(site));
        let Some(initializer) = rows.next() else {
            return Ok(None);
        };
        if rows.next().is_some() {
            return Ok(None);
        }
        let Ok(source) =
            crate::mir::resolved_value_profile::expression_plan::local_expression_source_plan_v1(
                &ledger,
                input.body_shape(),
                initializer,
            )
        else {
            return Ok(None);
        };
        let Some(plan) = HomeExpressionPlanV1::initializer_reads(
            source,
            homes,
            None,
            std::slice::from_ref(&request),
        ) else {
            return Ok(None);
        };
        Some(plan)
    } else {
        None
    };
    let mut installation = plan.as_ref().map(|_| locals.expression_installation());
    if let (Some(plan), Some(installation)) = (&plan, &mut installation) {
        if !installation.approve(plan) {
            return Ok(None);
        }
    }
    let demand = plan
        .as_ref()
        .map(|plan| LocalFieldReadDemandV1::SourceExpression {
            probe: false,
            plan: std::rc::Rc::clone(plan.source()),
        })
        .unwrap_or(LocalFieldReadDemandV1::Declared { scalar_only: false });
    let Some(mut results) = local_field_read(std::slice::from_ref(&request), demand)? else {
        return Ok(None);
    };
    if results.len() != 1 {
        return Ok(None);
    }
    if let Some(installation) = installation {
        installation.commit();
    }
    Ok(Some(ObservedLocalFieldReadV1 {
        request,
        result: results.remove(0),
    }))
}

/// Original received acquisition; no class, nonnull fact or Home is issued.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ReceivedFieldReadRequestV1 {
    Object {
        acquisition: OwnedExprSiteV1,
        nullable: bool,
    },
    StoredMapGet {
        frame: std::rc::Rc<super::MapFieldGetLocalV1>,
        exclusion: std::rc::Rc<super::SourceNullExclusionV1>,
    },
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
    /// Exact owning handle and payload construction for a Shared read.
    pub(crate) shared_support: Option<(OwnedExprSiteV1, OwnedExprSiteV1)>,
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
    pub(crate) path:
        Option<crate::mir::resolved_value_profile::expression_plan::OriginalExpressionFieldPathV1>,
    pub(crate) alias_path:
        Option<Box<crate::mir::resolved_semantics::home_new_prefix::FieldAliasPathV1>>,
    pub(crate) field: Box<str>,
    /// Exact prior Normal writer, retained only for the allocation-only LOCAL profile.
    pub(crate) owned_ready: Option<FieldWritePlanV1>,
}

impl LocalFieldReadRequestV1 {
    /// A source obligation only. The field batch must independently prove its
    /// complete constructor child relation before admitting this observation.
    pub(crate) fn is_nullable_object_comparison_v1(
        &self, plan: &std::rc::Rc<crate::mir::resolved_value_profile::expression_plan::ExpressionSourcePlanV1>,
    ) -> bool {
        use crate::mir::resolved_value_profile::expression_plan::ExpressionNodeSourceV1 as Node;
        if self.path.is_some() || self.received.is_some() || self.owned_ready.is_some() { return false; }
        let Some(node) = self.node_in_plan(plan).and_then(|index| plan.nodes().get(index)) else { return false; };
        plan.nodes().iter().filter_map(|row| row.binary()).any(|binary|
            binary.operator() == crate::mir::resolved_semantics::ResolvedBinaryOperatorV1::Equal
                && [(binary.lhs(), binary.rhs()), (binary.rhs(), binary.lhs())].into_iter().any(|(read, null)|
                    read == node.site() && plan.nodes().iter().any(|row| row.site() == null
                        && matches!(row.source(), Node::Literal(ResolvedLiteralSourceV1::Null)))))
    }

    pub(crate) fn node_in_plan(
        &self,
        plan: &std::rc::Rc<
            crate::mir::resolved_value_profile::expression_plan::ExpressionSourcePlanV1,
        >,
    ) -> Option<usize> {
        if self.site.owner() != plan.owner() {
            return None;
        }
        match &self.path {
            Some(path) => (std::rc::Rc::ptr_eq(path.plan(), plan)
                && path.site() == self.site
                && path.root() == self.receiver
                && path.receiver_site() == &self.receiver_site
                && path.name() == self.field.as_ref())
            .then_some(path.node()),
            None => plan.field_read_node_v1(
                self.site.site(),
                &self.receiver_site,
                self.receiver,
                &self.field,
            ),
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
    let mut shared_support = None;
    let (home, alias_class, nullable, formal) = match provenance {
        local_flow::FieldReadReceiverV1::OwnedHome => (receiver, None, false, false),
        local_flow::FieldReadReceiverV1::Shared { acquisition, payload } => {
            shared_support = Some((acquisition, payload));
            (receiver, None, false, false)
        },
        local_flow::FieldReadReceiverV1::RootedHandle(root) => (root, None, false, false),
        local_flow::FieldReadReceiverV1::Alias { class, root } => (root, Some(class), false, false),
        local_flow::FieldReadReceiverV1::ReceivedNullable => (receiver, None, true, false),
        local_flow::FieldReadReceiverV1::CheckedReceived(original) => {
            received = Some(original);
            (receiver, None, false, false)
        }
        local_flow::FieldReadReceiverV1::GuardedFormal => (receiver, None, false, true),
    };
    if let Some(ReceivedFieldReadRequestV1::StoredMapGet { frame, exclusion }) = &received {
        if frame.binding() != receiver
            || !exclusion.corroborates_initializer(input, receiver, frame.site().site(), object)
        {
            return None;
        }
    }
    let owned_ready = locals.ready_field_read(home, field)?;
    Some(LocalFieldReadRequestV1 {
        site: OwnedExprSiteV1::new(input.owner(), site.clone()),
        receiver_site: object.clone(),
        receiver,
        home,
        shared_support,
        alias_class,
        nullable,
        formal,
        received,
        path: None,
        alias_path: None,
        field: field.clone(),
        owned_ready,
    })
}

/// A nested path keeps its genuine root binding; its immediate receiver is
/// another original field expression, never a fabricated local declaration.
pub(super) fn expression_field_read_request(
    input: ResolvedFunctionLoweringInputV1<'_>,
    plan: &std::rc::Rc<crate::mir::resolved_value_profile::expression_plan::ExpressionSourcePlanV1>,
    site: &SourceExprSiteV1,
    locals: &PrefixLocalFlow<'_>,
) -> Option<LocalFieldReadRequestV1> {
    let path =
        crate::mir::resolved_value_profile::expression_plan::OriginalExpressionFieldPathV1::issue(
            std::rc::Rc::clone(plan),
            site,
        )?;
    if !path.is_nested_component() {
        let mut request = field_read_request(input, site, locals)?;
        if matches!(
            request.received,
            Some(ReceivedFieldReadRequestV1::StoredMapGet { .. })
        ) {
            request.path = Some(path);
        }
        return Some(request);
    }
    let mut request = field_read_request(input, path.first_site(), locals)?;
    if request.receiver != path.root() || input.owner() != plan.owner() {
        return None;
    }
    if request.alias_class.is_some() {
        request.alias_path = Some(Box::new(locals.field_alias_call_path(request.receiver)?));
    }
    request.site = path.site();
    request.receiver_site = path.receiver_site().clone();
    request.field = path.name().into();
    request.path = Some(path);
    Some(request)
}

/// Install an already-observed local read, retaining the same receiver request.
pub(super) fn install_local_field_read_result(
    input: ResolvedFunctionLoweringInputV1<'_>,
    site: &SourceExprSiteV1,
    binding: BindingRefV1,
    locals: &mut PrefixLocalFlow<'_>,
    unavailable: &mut Option<HomePrefixUnavailableV1>,
    observed: ObservedLocalFieldReadV1,
) {
    // A proven `receiver.field` initializer binds by its
    // declared result class — numeric scalars are trivial,
    // ordinary-box results are borrowed field aliases that
    // join no Home set.
    let ObservedLocalFieldReadV1 { request, result } = observed;
    match result {
        LocalFieldReadResultV1::ArrayField(_) | LocalFieldReadResultV1::BorrowedObject(_) | LocalFieldReadResultV1::Text(_) => {
            // Statement-call provenance never installs a local Home.
            unavailable.get_or_insert(HomePrefixUnavailableV1::SourceMismatch);
        }
        LocalFieldReadResultV1::Scalar => {
            let Some(source) = ScalarFieldReadSourceV1::issue(input, binding, request) else {
                unavailable.get_or_insert(HomePrefixUnavailableV1::SourceMismatch);
                return;
            };
            if !locals.install_scalar_field_read(
                binding,
                std::rc::Rc::new(source),
            ) {
                unavailable.get_or_insert(HomePrefixUnavailableV1::SourceMismatch);
            }
        }
        LocalFieldReadResultV1::BorrowedArray(source) => {
            let Some(request) = field_read_request(input, site, locals) else {
                unavailable.get_or_insert(HomePrefixUnavailableV1::SourceMismatch);
                return;
            };
            if !locals.install_borrowed_array_field(binding, source, request.home) {
                unavailable.get_or_insert(HomePrefixUnavailableV1::SourceMismatch);
            }
        }
        LocalFieldReadResultV1::Alias(class) => {
            let Some(request) = field_read_request(input, site, locals) else {
                unavailable.get_or_insert(HomePrefixUnavailableV1::SourceMismatch);
                return;
            };
            locals.install_field_alias(binding, class, request);
        }
    }
}
