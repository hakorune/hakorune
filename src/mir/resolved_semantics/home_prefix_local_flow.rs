//! Local state for the existing selected-New source prefix, not general Home Flow.
//!
//! Ordinary observations cannot carry an owned Home. The source scanner alone
//! records selected Normal installations; alias initialization stores a Handle.

use super::SelectedNewArgumentKindV1;
use super::{
    BindingRefV1, ExprChildRoleV1, OwnedExprSiteV1, ResolvedLexicalRefV1, ResolvedLiteralSourceV1,
    SourceExprSiteV1, SourceStmtSiteV1,
};
use crate::ast::ASTNode;
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::loop_recipe_contract::VerifiedLoopHomeAfterLoanV1;
use std::collections::{BTreeMap, BTreeSet};
#[path = "home_prefix_borrowed_nullable.rs"]
mod borrowed_nullable;
#[path = "home_prefix_field_alias.rs"]
mod field_alias;
#[path = "home_prefix_field_readiness.rs"]
mod field_readiness;
#[path = "home_prefix_scalar.rs"]
mod scalar;
#[path = "home_prefix_source_loop.rs"]
pub(super) mod source_loop;
pub(crate) use scalar::{
    BranchNormalScalarSourceV1, CurrentScalarProducerV1, CurrentScalarReadV1, SourceScalarKind,
};
pub(crate) use source_loop::{
    BranchNormalTextSourceV1, LoopDynamicReadV1, LoopHeaderValueBasisV1, LoopTextReadV1,
    SourceLoopDynamicProducerV1, SourceLoopTextProducerV1, SourceLoopValueProducerV1,
};
#[path = "home_prefix_text.rs"]
mod text;
pub(crate) use field_alias::{FieldAliasPathV1, FieldAliasRootSourceV1};
#[cfg(test)]
pub(crate) use text::assert_identity_for_test as assert_text_residence_identity_for_test;
pub(crate) use text::{LiveHomeV1, StaticTextReadV1, TextResidenceAcquisitionV1};

/// Exact pre-Loop scalar-class snapshot minted by the running Home walk.
/// Source shape and post-Loop values are deliberately outside this loan.
#[derive(Debug, Clone)]
pub(crate) struct LoopI64PreStateRequestV1 {
    owner: super::FunctionOwnerIdV1,
    site: SourceStmtSiteV1,
    integer_bindings: BTreeSet<BindingRefV1>,
    prior_homes: Box<[LiveHomeV1]>,
    borrowed_arrays: BTreeMap<BindingRefV1, super::field_read::BorrowedArrayFieldSourceV1>,
}

impl LoopI64PreStateRequestV1 {
    pub(crate) const fn owner(&self) -> super::FunctionOwnerIdV1 {
        self.owner
    }
    pub(crate) fn site(&self) -> &SourceStmtSiteV1 {
        &self.site
    }
    pub(crate) fn prior_homes(&self) -> &[LiveHomeV1] {
        &self.prior_homes
    }
    pub(crate) fn same_snapshot(&self, other: &Self) -> bool {
        self.owner == other.owner
            && self.site == other.site
            && self.integer_bindings == other.integer_bindings
            && self.prior_homes == other.prior_homes
            && self.borrowed_arrays.len() == other.borrowed_arrays.len()
            && self.borrowed_arrays.iter().all(|(binding, source)| {
                other.borrowed_arrays.get(binding).is_some_and(|other| {
                    std::rc::Rc::ptr_eq(&source.requirement, &other.requirement)
                })
            })
    }
    pub(crate) fn borrowed_array(
        &self,
        binding: BindingRefV1,
    ) -> Option<&super::field_read::BorrowedArrayFieldSourceV1> {
        self.borrowed_arrays.get(&binding)
    }
    pub(crate) fn proves_integer(&self, binding: BindingRefV1) -> bool {
        binding.owner() == self.owner && self.integer_bindings.contains(&binding)
    }
}

#[derive(Clone)]
enum StoredLocal {
    StaticText {
        definition: SourceExprSiteV1,
    },
    Text {
        root: BindingRefV1,
        acquisition: TextResidenceAcquisitionV1,
    },
    /// Exact mutually-exclusive Text result of one source If. The branch
    /// source carries both ordered producers; neither arm is selected here.
    TextBranch {
        source: std::rc::Rc<BranchNormalTextSourceV1>,
    },
    /// A Text carrier after a verified Loop. The exact product owns the
    /// Static/Owned predecessor choice; this state grants no independent Home.
    LoopText {
        product: std::rc::Rc<crate::mir::loop_recipe_contract::VerifiedSourceLoopRecipeProductV2>,
    },
    NamedArrayHolder(crate::mir::normal_callable_semantic_package::NamedArrayHolderSourceLoanV1),
    Home {
        acquisition: super::OwnedExprSiteV1,
        /// Immutable declaration source; branch-local readiness is not shared here.
        construction_source: Option<
            std::rc::Rc<
                crate::mir::normal_callable_semantic_package::ConstructionSourceStructureV1,
            >,
        >,
        ready: BTreeMap<hakorune_mir_defs::CanonicalFieldRefV1, super::FieldWritePlanV1>,
    },
    /// A call-result handle the caller received as an owned Home. It owes
    /// the caller's exit exactly one release like a `Home`, but its
    /// acquisition is a call site — never a `new` site — so the
    /// home-acquisition lookups must not surface it.
    ReceivedHandle {
        acquisition: OwnedExprSiteV1,
    },
    /// A nullable receiver-call result the caller owns only when non-null.
    /// It owes the caller's exit a checked release and must never satisfy
    /// a `Handle`-only probe — the `Void` sentinel is a real outcome.
    ReceivedNullable {
        acquisition: OwnedExprSiteV1,
    },
    /// An opaque Dynamic invocation result retained with its exact source
    /// frame and lifecycle. It grants no read or move classification.
    DynamicInvocationResult {
        frame: std::rc::Rc<super::CheckedInitializerExpressionFrameV1>,
    },
    /// Original call result and borrowed root remain distinct, owning nothing.
    BorrowedNullable(
        Box<crate::mir::normal_callable_semantic_package::BorrowedNullableLocalSourceLoanV1>,
    ),
    StoredMapGet(std::rc::Rc<super::MapFieldGetLocalV1>),
    ReceivedMixed(Box<crate::mir::normal_callable_semantic_package::MixedLocalCallSourceLoanV1>),
    Map,
    /// A `: MapBox` declared formal — caller-owned map storage borrowed
    /// read-only for the call. It reads like a live map but owns nothing:
    /// no End obligation, no ownership transfer back, no map-local return.
    BorrowedMap,
    Consumed,
    Handle(BindingRefV1),
    /// A local bound to a proven `receiver.field` read whose declared type
    /// is an ordinary box — a borrowed alias into the receiver's storage.
    /// It joins no Home set, owes no release, and is never observable as a
    /// movable value: `observe` returns `None` so selected-`new` argument
    /// observation cannot silently transfer the root's lease. The stored
    /// class is the field's declared type name — the receiver-class
    /// authority for a later `alias.<field>` read.
    FieldAlias {
        class: Box<str>,
        root: BindingRefV1,
        borrow: Option<FieldAliasPathV1>,
    },
    BorrowedArray {
        source: super::field_read::BorrowedArrayFieldSourceV1,
        root: BindingRefV1,
    },
    Trivial(Option<SourceScalarKind>),
    /// A binding produced by an inventoried call expression. The value
    /// exists at later sites but carries no scalar/rooted-storage class —
    /// it is neither a trivial local nor a Home/Map/Handle root.
    BoundValue,
    /// A binding or site holding the exact source `null` literal. It is
    /// neither a trivial scalar nor a handle root — only its own fact.
    Null,
    /// A local whose complete original source SUM is the tagged Dynamic
    /// carrier. The source-point proof remains required at each read.
    TaggedDynamic,
    Uninitialized,
}

/// Receiver class provenance for one `receiver.field` initializer read.
/// The scanner resolves the stored-local class; the issuer predicate alone
/// decides the field declaration.
pub(super) enum FieldReadReceiverV1 {
    /// A claim-local selected `new` Home — the binding is its own root.
    OwnedHome,
    /// A handle alias or self-rooted entry/param binding — the payload is
    /// the movable handle root the candidate/entry proof consults.
    RootedHandle(BindingRefV1),
    /// A binding produced by an earlier proven field read — the payload is
    /// the field's declared class name.
    Alias {
        class: Box<str>,
        root: BindingRefV1,
    },
    /// A received nullable call result whose `== null` arm terminated —
    /// the surviving join marked it non-null, so the binding is its own
    /// live base. The issuer resolves the class from the sealed call's
    /// `NullableObject` claim, never from the stored local itself.
    ReceivedNullable,
    CheckedReceived(super::field_read::ReceivedFieldReadRequestV1),
    /// A self-rooted `OpaqueHandle` formal whose exact `== null` arm
    /// terminated — the surviving join marked it non-null. The binding
    /// itself is the borrowed base on this path; the issuer resolves the
    /// class only from the co-sealed borrowed-formal object view, never
    /// from the stored local or the declaration contract.
    GuardedFormal,
}

pub(super) enum OrdinaryObservation {
    Integer(i64),
    Bool(bool),
    Null,
    TrivialLocal(BindingRefV1, Option<SourceScalarKind>),
    Handle(BindingRefV1),
    TaggedDynamicLocal(BindingRefV1),
    BoundValue(BindingRefV1),
}

/// One exact lexical read whose original source SUM is the tagged Dynamic
/// carrier. This proof preserves the runtime tag and carries no ownership or
/// concrete payload view.
#[derive(Debug, Clone)]
pub(crate) struct TaggedDynamicReadV1 {
    points: std::rc::Rc<crate::mir::normal_callable_semantic_package::SourceValueClassFactsV1>,
    read: SourceExprSiteV1,
    binding: BindingRefV1,
    value: crate::mir::resolved_semantics::source_value_flow::SourceValueRefV1,
}
impl PartialEq for TaggedDynamicReadV1 {
    fn eq(&self, other: &Self) -> bool {
        std::rc::Rc::ptr_eq(&self.points, &other.points)
            && self.read == other.read
            && self.binding == other.binding
            && self.value == other.value
    }
}
impl Eq for TaggedDynamicReadV1 {}

/// Exact source-flow point classified as tagged Dynamic by the original SUM.
/// It carries no concrete payload tag, conversion, or ownership permission.
#[derive(Debug, Clone)]
pub(crate) struct TaggedDynamicValueV1 {
    points: std::rc::Rc<crate::mir::normal_callable_semantic_package::SourceValueClassFactsV1>,
    flow: std::sync::Arc<crate::mir::resolved_semantics::source_value_flow::VerifiedSourceValueFlowV1>,
    binding: BindingRefV1,
    value: crate::mir::resolved_semantics::source_value_flow::SourceValueRefV1,
}
impl PartialEq for TaggedDynamicValueV1 {
    fn eq(&self, other: &Self) -> bool {
        std::rc::Rc::ptr_eq(&self.points, &other.points)
            && std::sync::Arc::ptr_eq(&self.flow, &other.flow)
            && self.binding == other.binding
            && self.value == other.value
    }
}
impl Eq for TaggedDynamicValueV1 {}
impl TaggedDynamicValueV1 {
    pub(crate) fn binding(&self) -> BindingRefV1 { self.binding }
    pub(crate) fn value(&self) -> crate::mir::resolved_semantics::source_value_flow::SourceValueRefV1 {
        self.value
    }
    pub(crate) fn expression_site_v1(&self) -> Option<&SourceExprSiteV1> {
        match self.flow.origin(self.value)? {
            crate::mir::resolved_semantics::source_value_flow::SourceValueOriginV1::Expression(site) => Some(site),
            _ => None,
        }
    }
    pub(crate) fn proves_source_value_v1(&self, binding: BindingRefV1,
        value: crate::mir::resolved_semantics::source_value_flow::SourceValueRefV1,
    ) -> bool {
        self.binding == binding && self.value == value && self.points.proves_value(&self.flow, value)
    }
    pub(crate) fn proves(
        &self,
        flow: &std::sync::Arc<crate::mir::resolved_semantics::source_value_flow::VerifiedSourceValueFlowV1>,
        binding: BindingRefV1,
        value: crate::mir::resolved_semantics::source_value_flow::SourceValueRefV1,
    ) -> bool {
        self.binding == binding
            && self.value == value
            && std::sync::Arc::ptr_eq(&self.flow, flow)
            && self.points.proves_value(flow, value)
    }
}
impl TaggedDynamicReadV1 {
    pub(crate) fn binding(&self) -> BindingRefV1 {
        self.binding
    }
    pub(crate) fn read(&self) -> &SourceExprSiteV1 {
        &self.read
    }
    pub(crate) fn is_formal(&self) -> bool {
        self.points.proves_formal(self.binding)
    }
    pub(crate) fn value(&self) -> crate::mir::resolved_semantics::source_value_flow::SourceValueRefV1 {
        self.value
    }
    pub(crate) fn leaf_class(
        &self,
    ) -> crate::mir::resolved_value_profile::expression_projection::ExpressionLeafClassV1 {
        crate::mir::resolved_value_profile::expression_projection::ExpressionLeafClassV1::Value(
            crate::mir::resolved_value_profile::expression_class::ExpressionValueClassV1::Dynamic,
        )
    }
    pub(crate) fn matches_source_node(
        &self,
        source: &crate::mir::resolved_value_profile::expression_plan::ExpressionSourcePlanV1,
        node: usize,
    ) -> bool {
        let Some(original) = source.nodes().get(node) else {
            return false;
        };
        source.owner() == self.binding.owner()
            && original.site() == &self.read
            && matches!(
                original.source(),
                crate::mir::resolved_value_profile::expression_plan::ExpressionNodeSourceV1::Reference(
                    ResolvedLexicalRefV1::Local(binding)
                ) if *binding == self.binding
            )
            && self.points.proves_value(self.points.flow(), self.value)
            && self.points.proves_read(self.points.flow(), &self.read, self.binding)
                == Some(self.value)
    }
}

impl OrdinaryObservation {
    pub(super) fn is_trivial(&self) -> bool {
        match self {
            Self::Integer(_) | Self::Bool(_) | Self::TrivialLocal(..) => true,
            Self::Null | Self::Handle(_) | Self::TaggedDynamicLocal(_) | Self::BoundValue(_) => false,
        }
    }

    pub(super) fn into_selected_argument(self) -> Option<SelectedNewArgumentKindV1> {
        match self {
            Self::Integer(value) => Some(SelectedNewArgumentKindV1::Integer(value)),
            Self::Bool(value) => Some(SelectedNewArgumentKindV1::Bool(value)),
            Self::Null => Some(SelectedNewArgumentKindV1::Null),
            Self::TrivialLocal(binding, _) => Some(SelectedNewArgumentKindV1::Local { binding }),
            Self::Handle(root) => Some(SelectedNewArgumentKindV1::Handle { binding: root }),
            Self::TaggedDynamicLocal(binding) => {
                Some(SelectedNewArgumentKindV1::BoundValue { binding })
            }
            Self::BoundValue(binding) => Some(SelectedNewArgumentKindV1::BoundValue { binding }),
        }
    }
}

fn stored_local_same(left: &StoredLocal, right: &StoredLocal) -> bool {
    match (left, right) {
        (StoredLocal::StaticText { definition: a }, StoredLocal::StaticText { definition: b }) => {
            a == b
        }
        (
            StoredLocal::Text {
                root: a,
                acquisition: ap,
            },
            StoredLocal::Text {
                root: b,
                acquisition: bp,
            },
        ) => a == b && ap.same_source(bp),
        (StoredLocal::LoopText { product: a }, StoredLocal::LoopText { product: b }) => {
            std::rc::Rc::ptr_eq(a, b)
        }
        (StoredLocal::TextBranch { source: a }, StoredLocal::TextBranch { source: b }) => {
            std::rc::Rc::ptr_eq(a, b)
        }
        (StoredLocal::NamedArrayHolder(a), StoredLocal::NamedArrayHolder(b)) => a == b,
        (
            StoredLocal::BorrowedArray {
                source: a,
                root: ar,
            },
            StoredLocal::BorrowedArray {
                source: b,
                root: br,
            },
        ) => a == b && ar == br,
        (
            StoredLocal::Home {
                acquisition: a,
                construction_source: ac,
                ready: ar,
            },
            StoredLocal::Home {
                acquisition: b,
                construction_source: bc,
                ready: br,
            },
        ) => {
            a == b
                && ar.len() == br.len()
                && ar.iter().all(|(field, frame)| {
                    br.get(field).is_some_and(|other| frame.same_source(other))
                })
                && match (ac, bc) {
                    (None, None) => true,
                    (Some(a), Some(b)) => std::rc::Rc::ptr_eq(a, b),
                    _ => false,
                }
        }
        (
            StoredLocal::ReceivedHandle { acquisition: a },
            StoredLocal::ReceivedHandle { acquisition: b },
        ) => a == b,
        (
            StoredLocal::ReceivedNullable { acquisition: a },
            StoredLocal::ReceivedNullable { acquisition: b },
        ) => a == b,
        (
            StoredLocal::DynamicInvocationResult { frame: a },
            StoredLocal::DynamicInvocationResult { frame: b },
        ) => std::rc::Rc::ptr_eq(a, b),
        (StoredLocal::Map, StoredLocal::Map) => true,
        (StoredLocal::BorrowedNullable(a), StoredLocal::BorrowedNullable(b)) => a == b,
        (StoredLocal::StoredMapGet(a), StoredLocal::StoredMapGet(b)) => std::rc::Rc::ptr_eq(a, b),
        (StoredLocal::ReceivedMixed(a), StoredLocal::ReceivedMixed(b)) => a == b,
        (StoredLocal::BorrowedMap, StoredLocal::BorrowedMap) => true,
        (StoredLocal::Consumed, StoredLocal::Consumed) => true,
        (StoredLocal::Handle(a), StoredLocal::Handle(b)) => a == b,
        (
            StoredLocal::FieldAlias {
                class: a,
                root: ar,
                borrow: ab,
            },
            StoredLocal::FieldAlias {
                class: b,
                root: br,
                borrow: bb,
            },
        ) => a == b && ar == br && ab == bb,
        (StoredLocal::Trivial(a), StoredLocal::Trivial(b)) => a == b,
        (StoredLocal::BoundValue, StoredLocal::BoundValue) => true,
        (StoredLocal::TaggedDynamic, StoredLocal::TaggedDynamic) => true,
        (StoredLocal::Null, StoredLocal::Null) => true,
        (StoredLocal::Uninitialized, StoredLocal::Uninitialized) => true,
        _ => false,
    }
}

#[derive(Clone)]
pub(super) struct PrefixLocalFlow<'source> {
    null_exclusions: BTreeMap<BindingRefV1, std::rc::Rc<super::SourceNullExclusionV1>>,
    signed_formals: BTreeSet<BindingRefV1>,
    opaque_formals: BTreeSet<BindingRefV1>,
    source_loop: Option<std::rc::Rc<source_loop::SourceLoopHeaderEnvironmentV1>>,
    named_arrays:
        std::rc::Rc<std::cell::RefCell<Vec<super::named_array::NamedArrayHomeObservationV1>>>,
    input: ResolvedFunctionLoweringInputV1<'source>,
    locals: BTreeMap<BindingRefV1, StoredLocal>,
    pub(super) entry_receiver: Option<PrefixEntryReceiverV1>,
    scalar_field_reads: BTreeMap<
        BindingRefV1,
        std::rc::Rc<super::field_read::ScalarFieldReadSourceV1>,
    >,
    /// Path-sensitive non-null narrowing for `ReceivedNullable` bindings:
    /// a terminated `binding == null` arm proves the surviving fall-through
    /// carries a live object. Entries are added only by the branch join
    /// and invalidated by every re-store — the set alone proves nothing
    /// without the `ReceivedNullable` stored class.
    nonnull: std::collections::BTreeSet<BindingRefV1>,
    // Branch clones share source inventory, never the path-sensitive local classes.
    expression_plans: std::rc::Rc<std::cell::RefCell<Vec<super::HomeExpressionPlanV1>>>,
    source_value_class_facts: Option<std::rc::Rc<crate::mir::normal_callable_semantic_package::SourceValueClassFactsV1>>,
}

#[derive(Clone)]
pub(super) struct PrefixEntryReceiverV1 {
    pub(super) binding: BindingRefV1,
    pub(super) owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
    pub(super) batch_slot: u32,
    pub(super) identity: crate::parser::CallableDeclarationIdentityV1,
}

impl<'source> PrefixLocalFlow<'source> {
    /// Install a complete verified root After, never a single convenient
    /// carrier. The current walk point and both live I64 prestates must match
    /// before either local is changed.
    pub(super) fn install_loop_i64_after(
        &mut self,
        prestate: LoopI64PreStateRequestV1,
        loan: VerifiedLoopHomeAfterLoanV1,
    ) -> bool {
        let [scale, induction] = loan.carriers();
        if prestate.owner() != self.input.owner()
            || loan.owner() != prestate.owner()
            || loan.site() != prestate.site()
            || scale == induction
            || !prestate.proves_integer(scale)
            || !prestate.proves_integer(induction)
        {
            return false;
        }
        self.store(scale, StoredLocal::Trivial(Some(SourceScalarKind::Integer)));
        self.store(
            induction,
            StoredLocal::Trivial(Some(SourceScalarKind::Integer)),
        );
        true
    }

    /// Only a resolver-owned Loop at the current walk point can borrow these
    /// classes. Rebinding/branch joins already update `locals` before this
    /// snapshot; an untyped or invalidated binding is absent.
    pub(super) fn loop_i64_prestate(
        &self,
        site: &SourceStmtSiteV1,
        homes: &[LiveHomeV1],
    ) -> Option<LoopI64PreStateRequestV1> {
        self.input.function().loop_region_bundle(site).ok()?;
        let integer_bindings = self
            .locals
            .iter()
            .filter_map(|(binding, value)| {
                matches!(value, StoredLocal::Trivial(Some(SourceScalarKind::Integer)))
                    .then_some(*binding)
            })
            .collect();
        Some(LoopI64PreStateRequestV1 {
            owner: self.input.owner(),
            site: site.clone(),
            integer_bindings,
            prior_homes: homes.iter().rev().cloned().collect(),
            borrowed_arrays: self.live_borrowed_array_sources(),
        })
    }

    pub(super) fn new(input: ResolvedFunctionLoweringInputV1<'source>) -> Self {
        Self::with_source_value_class_facts(input, None)
    }

    pub(super) fn with_source_value_class_facts(
        input: ResolvedFunctionLoweringInputV1<'source>,
        source_value_class_facts: Option<std::rc::Rc<crate::mir::normal_callable_semantic_package::SourceValueClassFactsV1>>,
    ) -> Self {
        Self {
            null_exclusions: Default::default(),
            signed_formals: Default::default(),
            opaque_formals: Default::default(),
            source_loop: None,
            named_arrays: Default::default(),
            input,
            locals: BTreeMap::new(),
            entry_receiver: None,
            scalar_field_reads: BTreeMap::new(),
            nonnull: std::collections::BTreeSet::new(),
            expression_plans: Default::default(),
            source_value_class_facts,
        }
    }

    pub(super) fn expression_installation(
        &self,
    ) -> super::expression_plan::ExpressionPlanInstallationV1<'_> {
        super::expression_plan::ExpressionPlanInstallationV1::new(
            self.input.owner(),
            self.expression_plans.borrow_mut(),
        )
    }
    pub(super) fn retain_expression_plan(&mut self, plan: super::HomeExpressionPlanV1) -> bool {
        let mut installation = self.expression_installation();
        if !installation.approve(&plan) {
            return false;
        }
        installation.commit();
        true
    }
    pub(super) fn expression_plans(&self) -> Vec<super::HomeExpressionPlanV1> {
        self.expression_plans.borrow().clone()
    }

    /// Every stored-class mutation discards the binding's narrowing — a
    /// rebound or consumed binding must never inherit an old proof.
    fn store(&mut self, binding: BindingRefV1, value: StoredLocal) {
        self.nonnull.remove(&binding);
        self.null_exclusions.remove(&binding);
        self.opaque_formals.remove(&binding);
        self.scalar_field_reads.remove(&binding);
        self.locals.insert(binding, value);
    }

    pub(super) fn install_scalar_field_read(
        &mut self,
        binding: BindingRefV1,
        source: std::rc::Rc<super::field_read::ScalarFieldReadSourceV1>,
    ) -> bool {
        if !source.matches_initializer_v1(binding, source.request_site_v1()) {
            return false;
        }
        self.store(
            binding,
            StoredLocal::Trivial(Some(SourceScalarKind::Integer)),
        );
        self.scalar_field_reads.insert(binding, source);
        true
    }

    pub(super) fn scalar_field_read_source_v1(
        &self,
        binding: BindingRefV1,
        site: &SourceExprSiteV1,
    ) -> Option<std::rc::Rc<super::field_read::ScalarFieldReadSourceV1>> {
        let source = self.scalar_field_reads.get(&binding)?;
        source
            .matches_initializer_v1(binding, site)
            .then(|| std::rc::Rc::clone(source))
    }

    /// Mark the binding non-null on this surviving path. Only a live
    /// `ReceivedNullable` or a self-rooted `Handle` can carry the mark; any
    /// other stored class — including an `Uninitialized` produced by a
    /// sibling branch scope — ignores it.
    pub(super) fn mark_nonnull(&mut self, binding: BindingRefV1) {
        let narrowable = match self.locals.get(&binding) {
            Some(StoredLocal::ReceivedNullable { .. }) => true,
            Some(StoredLocal::BorrowedNullable(_) | StoredLocal::StoredMapGet(_)) => true,
            Some(StoredLocal::ReceivedMixed(_)) => true,
            Some(StoredLocal::Handle(root)) => *root == binding,
            _ => false,
        };
        if narrowable {
            self.nonnull.insert(binding);
        }
    }

    /// The receiver root for a `receiver.field` read: an ordinary Handle
    /// observation first, then a non-null-narrowed received nullable —
    /// the binding itself is the live base on this path. A self-rooted
    /// `Handle` that names a formal parameter answers only on the
    /// non-null-narrowed surviving path — the borrowed object is not
    /// proven live before the guard.
    pub(super) fn field_home(&self, site: &SourceExprSiteV1) -> Option<BindingRefV1> {
        if let Some(OrdinaryObservation::Handle(home)) = self.observe(site) {
            // A formal parameter's borrowed object is live only on the
            // non-null-narrowed path — `me` (Receiver) and rooted locals
            // keep their unconditional answer.
            if matches!(
                self.input.function().binding(home).map(|row| row.kind()),
                Some(crate::mir::resolved_semantics::BindingKindV1::Parameter { .. })
            ) && !self.nonnull.contains(&home)
            {
                return None;
            }
            return Some(home);
        }
        let ResolvedLexicalRefV1::Local(binding) = self.input.function().variable_ref(site)? else {
            return None;
        };
        (matches!(
            self.locals.get(&binding),
            Some(StoredLocal::ReceivedNullable { .. })
        ) && self.nonnull.contains(&binding))
        .then_some(binding)
    }

    /// Join a sibling fall-through branch state into this one. Both sides
    /// were forked from the same entry snapshot; a binding that exists on
    /// both sides must carry the identical stored class — disagreement is a
    /// divergence the caller names, never a merged guess.
    ///
    /// A binding present on only one side was declared inside that
    /// branch's own scope: `resolve_if` pushes an `IfThen`/`IfElse`
    /// lexical frame and `leave_region_scope` pops it, so no post-join
    /// source site can resolve that `BindingRefV1`. The join still must
    /// not keep the sibling's initialized class — the value was proven on
    /// one path only — so the surviving entry is downgraded to
    /// `Uninitialized`, which keeps `observe` and every `is_*` probe
    /// fail-closed if a future shape ever does resolve it.
    pub(super) fn join_branch(&mut self, other: &Self) -> bool {
        self.join_branch_at(other, None)
    }

    pub(super) fn join_branch_at(
        &mut self, other: &Self, site: Option<&SourceStmtSiteV1>,
    ) -> bool {
        if !source_loop::same_context(&self.source_loop, &other.source_loop) {
            return false;
        }
        for (binding, value) in &other.locals {
            if let Some(own) = self.locals.get(binding) {
                if !stored_local_same(own, value) {
                    let Some(merged) = site.and_then(|site|
                        self.branch_text_join_v1(other, *binding, site)) else {
                        return false;
                    };
                    self.store(*binding, StoredLocal::TextBranch { source: merged });
                }
            }
        }
        for binding in self.locals.keys().copied().collect::<Vec<_>>() {
            if !other.locals.contains_key(&binding) {
                self.store(binding, StoredLocal::Uninitialized);
            }
        }
        for binding in other.locals.keys() {
            if !self.locals.contains_key(binding) {
                self.store(*binding, StoredLocal::Uninitialized);
            }
        }
        self.null_exclusions
            .retain(|root, proof| other.null_exclusions.get(root) == Some(proof));
        // Narrowing is path-sensitive: a mark survives the join only when
        // both siblings carried it.
        self.nonnull
            .retain(|binding| other.nonnull.contains(binding));
        self.opaque_formals
            .retain(|binding| other.opaque_formals.contains(binding));
        true
    }

    fn branch_text_join_v1(
        &self, other: &Self, binding: BindingRefV1, site: &SourceStmtSiteV1,
    ) -> Option<std::rc::Rc<BranchNormalTextSourceV1>> {
        let debug = crate::config::env::builder_debug_enabled();
        let flow = self.input.function().expression_source().value_flow()?;
        if flow.owner() != self.input.owner() || other.input.owner() != self.input.owner() {
            return None;
        }
        let Some((value, incoming)) = flow.branch_join(site, binding) else {
            if debug { eprintln!("[home-prefix-branch-text] owner={:?} binding={binding:?} site={site:?} stop=source-join-missing", self.input.owner()); }
            return None;
        };
        let [(then_edge, then_value), (else_edge, else_value)] = incoming else {
            if debug { eprintln!("[home-prefix-branch-text] owner={:?} binding={binding:?} site={site:?} stop=two-normal-arms-missing arms={incoming:?}", self.input.owner()); }
            return None;
        };
        use crate::mir::resolved_semantics::source_value_flow::SourceValueEdgeV1 as Edge;
        if !matches!((then_edge, else_edge), (Edge::ThenNormal, Edge::ElseNormal)) {
            if debug { eprintln!("[home-prefix-branch-text] owner={:?} binding={binding:?} site={site:?} stop=arm-order-mismatch arms={then_edge:?}/{else_edge:?}", self.input.owner()); }
            return None;
        }
        let Some(then_producer) = self.branch_text_producer_v1(binding, *then_value, flow) else {
            if debug { eprintln!("[home-prefix-branch-text] owner={:?} binding={binding:?} site={site:?} stop=then-producer-missing local={}", self.input.owner(), stored_local_kind(self.locals.get(&binding))); }
            return None;
        };
        let Some(else_producer) = other.branch_text_producer_v1(binding, *else_value, flow) else {
            if debug { eprintln!("[home-prefix-branch-text] owner={:?} binding={binding:?} site={site:?} stop=else-producer-missing local={}", self.input.owner(), stored_local_kind(other.locals.get(&binding))); }
            return None;
        };
        let result = BranchNormalTextSourceV1::new(
            std::sync::Arc::clone(flow), value, binding, site.clone(),
            Box::new([
                (then_edge.clone(), *then_value, then_producer),
                (else_edge.clone(), *else_value, else_producer),
            ]),
        );
        if result.is_none() && debug { eprintln!("[home-prefix-branch-text] owner={:?} binding={binding:?} site={site:?} stop=source-witness-mismatch", self.input.owner()); }
        result
    }

    fn branch_text_producer_v1(
        &self, binding: BindingRefV1, value: crate::mir::resolved_semantics::source_value_flow::SourceValueRefV1,
        flow: &std::sync::Arc<crate::mir::resolved_semantics::source_value_flow::VerifiedSourceValueFlowV1>,
    ) -> Option<SourceLoopTextProducerV1> {
        match self.locals.get(&binding)? {
            StoredLocal::StaticText { definition } => {
                let producer = self.text_producer_for_value_v1(binding, value, flow, &[])
                    .map(|(_, producer)| producer);
                if producer.is_none() && crate::config::env::builder_debug_enabled() {
                    let basis = self.source_loop_header_basis_v1(binding);
                    let header = basis.and_then(|basis| flow.loop_join(
                        basis.source().site(), binding,
                        crate::mir::resolved_semantics::source_value_flow::SourceValueJoinKindV1::LoopHeader,
                    ));
                    eprintln!("[home-prefix-branch-text] owner={:?} binding={binding:?} value={value:?} local=StaticText definition={definition:?} origin={:?} active_header={} header_value_matches={} source_header_matches={}",
                        self.input.owner(), flow.origin(value), basis.is_some(),
                        basis.is_some_and(|basis| basis.header() == value),
                        header.is_some_and(|(header, _)| header == value));
                }
                producer
            }
            StoredLocal::Text { root, acquisition }
                if *root == binding && acquisition.binding() == binding
                    && acquisition.is_text_residence() && acquisition.owns_root()
                    && acquisition.source().owner() == flow.owner() => {
                let definition = acquisition.source().nodes().first()?.site();
                let origin = flow.alias_origin_with_expression_boundary(
                    value,
                    |site| self.input.function().variable_ref(site),
                    |site| site == definition,
                )?;
                matches!(flow.origin(origin), Some(crate::mir::resolved_semantics::source_value_flow::SourceValueOriginV1::Expression(site)) if site == definition)
                    .then(|| SourceLoopTextProducerV1::Residence(acquisition.clone()))
            }
            StoredLocal::TextBranch { source }
                if source.matches_source(flow, binding, value) =>
            {
                Some(SourceLoopTextProducerV1::BranchNormal(std::rc::Rc::clone(source)))
            }
            _ => None,
        }
    }

    // Declaration contracts are borrowed from the sole package issuer. No
    // physical signature or default capability participates in entry admission.
    pub(super) fn install_parameters(
        &mut self,
        parameters: impl IntoIterator<
            Item = (
                u32,
                BindingRefV1,
                crate::mir::callable_parameter_contract::CallableParameterContractKindV1,
            ),
        >,
    ) -> bool {
        let mut count = 0;
        for (ordinal, binding, kind) in parameters {
            if binding.owner() != self.input.owner()
                || self
                    .input
                    .function()
                    .declaration_binding(&super::SourceBindingSiteV1::Parameter { index: ordinal })
                    != Some(binding)
                || self.locals.contains_key(&binding)
            {
                return false;
            }
            use crate::mir::callable_parameter_contract::CallableParameterContractKindV1;
            let is_opaque_formal = matches!(&kind, CallableParameterContractKindV1::OpaqueHandle);
            if matches!(&kind, CallableParameterContractKindV1::ExactTrivial(abi) if abi.is_i64()) {
                self.signed_formals.insert(binding);
            }
            let tagged_dynamic = matches!(
                &kind,
                CallableParameterContractKindV1::OpaqueHandle
            ) && self
                .source_value_class_facts
                .as_ref()
                .is_some_and(|points| points.proves_formal(binding));
            let source_signed_i64 = is_opaque_formal
                && self
                    .source_value_class_facts
                    .as_ref()
                    .is_some_and(|facts| facts.proves_signed_i64_formal(binding));
            if tagged_dynamic && source_signed_i64 {
                return false;
            }
            if source_signed_i64 {
                self.signed_formals.insert(binding);
            }
            let value = if tagged_dynamic {
                StoredLocal::TaggedDynamic
            } else if source_signed_i64 {
                StoredLocal::Trivial(Some(SourceScalarKind::Integer))
            } else {
                match kind {
                CallableParameterContractKindV1::ExactTrivial(abi)
                    if abi.mir_type() == crate::mir::MirType::Integer =>
                {
                    StoredLocal::Trivial(Some(SourceScalarKind::Integer))
                }
                CallableParameterContractKindV1::ExactTrivial(_) => return false,
                CallableParameterContractKindV1::Map => StoredLocal::BorrowedMap,
                CallableParameterContractKindV1::OpaqueHandle
                | CallableParameterContractKindV1::DeclaredHandle
                | CallableParameterContractKindV1::DeclaredObject(_)
                | CallableParameterContractKindV1::ExactText(_) => StoredLocal::Handle(binding),
                }
            };
            self.store(binding, value);
            if is_opaque_formal {
                self.opaque_formals.insert(binding);
            }
            count += 1;
        }
        count
            == self
                .input
                .function()
                .declaration_sites()
                .filter(|site| matches!(site, super::SourceBindingSiteV1::Parameter { .. }))
                .count()
    }

    /// Exact unannotated formal identity at this source read. This is a
    /// Dynamic source classification only; the downstream entry-read issuer
    /// still verifies the original formal contract and physical entry.
    pub(super) fn opaque_formal_binding(
        &self,
        site: &SourceExprSiteV1,
    ) -> Option<BindingRefV1> {
        let ResolvedLexicalRefV1::Local(binding) = self.input.function().variable_ref(site)? else {
            return None;
        };
        if !self.opaque_formals.contains(&binding) {
            return None;
        }
        let crate::mir::resolved_semantics::BindingKindV1::Parameter { index } =
            self.input.function().binding(binding)?.kind()
        else {
            return None;
        };
        (self.input.function().declaration_binding(&super::SourceBindingSiteV1::Parameter { index })
            == Some(binding))
            .then_some(binding)
    }

    pub(super) fn observe(&self, site: &SourceExprSiteV1) -> Option<OrdinaryObservation> {
        match self.input.function().expression_source().literal(site) {
            Some(ResolvedLiteralSourceV1::Integer(value)) => {
                return Some(OrdinaryObservation::Integer(*value));
            }
            Some(ResolvedLiteralSourceV1::Bool(value)) => {
                return Some(OrdinaryObservation::Bool(*value));
            }
            Some(ResolvedLiteralSourceV1::Null) => {
                return Some(OrdinaryObservation::Null);
            }
            _ => {}
        }
        let ResolvedLexicalRefV1::Local(binding) = self.input.function().variable_ref(site)? else {
            return None;
        };
        match self.locals.get(&binding)? {
            StoredLocal::Home { .. }
            | StoredLocal::ReceivedHandle { .. }
            | StoredLocal::Map
            | StoredLocal::BorrowedMap => Some(OrdinaryObservation::Handle(binding)),
            // A nullable result is a produced value, never a handle root —
            // the `Void` sentinel arm keeps every Handle-classified use
            // fail-closed while `new` argument observation can still name
            // the binding's value.
            StoredLocal::ReceivedNullable { .. } => Some(OrdinaryObservation::BoundValue(binding)),
            // Preserve the exact lifecycle owner but do not grant this
            // opaque carrier a read, scalar, or movable-root classification.
            StoredLocal::DynamicInvocationResult { .. } => None,
            StoredLocal::BorrowedNullable(_) | StoredLocal::StoredMapGet(_) => {
                Some(OrdinaryObservation::BoundValue(binding))
            }
            StoredLocal::ReceivedMixed(_) => Some(OrdinaryObservation::BoundValue(binding)),
            StoredLocal::Handle(root)
                if !matches!(self.locals.get(root), Some(StoredLocal::Consumed)) =>
            {
                Some(OrdinaryObservation::Handle(*root))
            }
            StoredLocal::Handle(_) | StoredLocal::Consumed => None,
            StoredLocal::TaggedDynamic => self
                .tagged_dynamic_read_v1(site)
                .map(|_| OrdinaryObservation::TaggedDynamicLocal(binding)),
            // A field-read alias names borrowed storage, never a movable
            // value — every observation lane stays fail-closed on it.
            StoredLocal::StaticText { .. }
            | StoredLocal::Text { .. }
            | StoredLocal::TextBranch { .. }
            | StoredLocal::LoopText { .. }
            | StoredLocal::FieldAlias { .. }
            | StoredLocal::NamedArrayHolder(_)
            | StoredLocal::BorrowedArray { .. } => None,
            StoredLocal::Trivial(kind) => Some(OrdinaryObservation::TrivialLocal(binding, *kind)),
            StoredLocal::BoundValue => Some(OrdinaryObservation::BoundValue(binding)),
            StoredLocal::Null => Some(OrdinaryObservation::Null),
            StoredLocal::Uninitialized => None,
        }
    }

    pub(super) fn tagged_dynamic_read_v1(
        &self,
        site: &SourceExprSiteV1,
    ) -> Option<TaggedDynamicReadV1> {
        let ResolvedLexicalRefV1::Local(binding) = self.input.function().variable_ref(site)? else {
            return None;
        };
        if !matches!(self.locals.get(&binding), Some(StoredLocal::TaggedDynamic)) {
            return None;
        }
        let flow = self.input.function().expression_source().value_flow()?;
        let points = self.source_value_class_facts.as_ref()?;
        let value = points.proves_read(flow, site, binding)?;
        Some(TaggedDynamicReadV1 {
            points: std::rc::Rc::clone(points),
            read: site.clone(),
            binding,
            value,
        })
    }

    pub(in crate::mir::resolved_semantics::home_new_prefix) fn tagged_dynamic_value_for_value_v1(
        &self,
        binding: BindingRefV1,
        value: crate::mir::resolved_semantics::source_value_flow::SourceValueRefV1,
        flow: &std::sync::Arc<crate::mir::resolved_semantics::source_value_flow::VerifiedSourceValueFlowV1>,
    ) -> Option<TaggedDynamicValueV1> {
        if binding.owner() != self.input.owner()
            || flow.owner() != binding.owner()
            || !matches!(self.locals.get(&binding), Some(StoredLocal::TaggedDynamic))
        {
            return None;
        }
        let points = self.source_value_class_facts.as_ref()?;
        points.proves_value(flow, value).then(|| TaggedDynamicValueV1 {
            points: std::rc::Rc::clone(points),
            flow: std::sync::Arc::clone(flow),
            binding,
            value,
        })
    }

    /// Argument-position observation for one selected `new` site. Literal
    /// and local rows come from `observe` first; a `receiver.field`
    /// expression is admitted only when the caller's issuer predicate proves
    /// the field `i64` on the receiver's own source definition — the scanner
    /// never infers a field class from MIR types or runtime layout. The
    /// predicate may be invoked twice for the same site (the observation
    /// pass and the accounting pass walk the same arguments), so the issuer
    /// must answer idempotently.
    pub(super) fn observe_selected_argument<E>(
        &self,
        site: &SourceExprSiteV1,
        argument_i64_field: &mut impl FnMut(
            &OwnedExprSiteV1,
            &SourceExprSiteV1,
            BindingRefV1,
            BindingRefV1,
            &str,
        ) -> Result<bool, E>,
    ) -> Result<Option<SelectedNewArgumentKindV1>, E> {
        if let Some(observation) = self.observe(site) {
            return Ok(observation.into_selected_argument());
        }
        let Ok(expr) = self
            .input
            .source()
            .expr_at(&OwnedExprSiteV1::new(self.input.owner(), site.clone()))
        else {
            return Ok(None);
        };
        let ASTNode::FieldAccess { field, .. } = expr.node() else {
            return Ok(None);
        };
        let Ok(receiver) = self
            .input
            .source()
            .child_expr_from_expr(&expr, ExprChildRoleV1::Receiver)
        else {
            return Ok(None);
        };
        let Some(OrdinaryObservation::Handle(home)) = self.observe(receiver.site()) else {
            return Ok(None);
        };
        let Some(ResolvedLexicalRefV1::Local(binding)) =
            self.input.function().variable_ref(receiver.site())
        else {
            return Ok(None);
        };
        let field_site = OwnedExprSiteV1::new(self.input.owner(), site.clone());
        if !argument_i64_field(&field_site, receiver.site(), binding, home, field)? {
            return Ok(None);
        }
        Ok(Some(SelectedNewArgumentKindV1::I64Field {
            object: binding,
        }))
    }

    /// Install the receiver and parameters carried by a verified instance
    /// entry loan. The loan is the sole authority for these bindings; the
    /// caller passes the row minted for this exact declaration only. The
    /// receiver installs as a borrowed self-rooted Handle — it is observable
    /// as a handle root but owns no Home and owes no release.
    pub(super) fn install_entry_home(
        &mut self,
        loan: &crate::mir::resolved_semantics::VerifiedInstanceEntryHomeLoanV1,
    ) -> bool {
        // The loan is scoped to one exact declaration: a foreign-owner row
        // never installs, even if its bindings happen to match by accident.
        if loan.owner() != self.input.owner() || self.entry_receiver.is_some() {
            return false;
        }
        let receiver = loan.receiver();
        if !self.install_parameters(
            loan.parameters()
                .iter()
                .map(|row| (row.ordinal(), row.binding(), row.kind())),
        ) {
            return false;
        }
        if receiver.owner() != self.input.owner()
            || self
                .input
                .function()
                .declaration_binding(&super::SourceBindingSiteV1::Receiver)
                != Some(receiver)
            || self.locals.contains_key(&receiver)
        {
            return false;
        }
        self.store(receiver, StoredLocal::Handle(receiver));
        self.entry_receiver = Some(PrefixEntryReceiverV1 {
            binding: receiver,
            owner: loan.owner(),
            batch_slot: loan.batch_slot(),
            identity: loan.identity().clone(),
        });
        true
    }

    /// `root` is a self-rooted handle exactly when it stores itself —
    /// i.e. a parameter-installed handle, never a live Home/Map local.
    pub(super) fn is_self_rooted_handle(&self, root: BindingRefV1) -> bool {
        matches!(self.locals.get(&root), Some(StoredLocal::Handle(r)) if *r == root)
    }

    /// `root` is a live map-installed local — the entry borrows it by
    /// reference; the local stays the owner and still issues its own End.
    pub(super) fn is_map_local(&self, root: BindingRefV1) -> bool {
        matches!(self.locals.get(&root), Some(StoredLocal::Map))
    }

    /// `root` is a `: MapBox` declared formal — caller-owned map storage
    /// borrowed read-only. It participates in reads and borrowed-entry
    /// classification but owns nothing: `is_map_local` stays owned-only so
    /// a borrowed formal can never ride the map-local return lane.
    pub(super) fn is_borrowed_map(&self, root: BindingRefV1) -> bool {
        matches!(self.locals.get(&root), Some(StoredLocal::BorrowedMap))
    }

    /// The sealed `new` acquisition site of a live Home local root.
    pub(super) fn home_acquisition(&self, root: BindingRefV1) -> Option<&super::OwnedExprSiteV1> {
        match self.locals.get(&root)? {
            StoredLocal::Home { acquisition, .. } => Some(acquisition),
            _ => None,
        }
    }

    pub(super) fn direct_available_home(
        &self,
        site: &SourceExprSiteV1,
    ) -> Option<(BindingRefV1, &super::OwnedExprSiteV1)> {
        let ResolvedLexicalRefV1::Local(binding) = self.input.function().variable_ref(site)? else {
            return None;
        };
        match self.locals.get(&binding)? {
            StoredLocal::Home { acquisition, .. } => Some((binding, acquisition)),
            _ => None,
        }
    }
    /// The receiver class provenance for a `receiver.field` initializer
    /// read: `OwnedHome` for a claim-local `new` local, `RootedHandle` for
    /// a handle alias or self-rooted entry/param binding (the stored root
    /// names the movable handle root the candidate/entry proof consults),
    /// `Alias` for a binding produced by an earlier proven field read — its
    /// declared class is the receiver's own class authority.
    /// Scope selection preserves rejected receiver provenance; a dead root
    /// must not silently send a selected field condition back to the raw lane.
    pub(super) fn is_field_read_candidate(&self, binding: BindingRefV1) -> bool {
        matches!(
            self.locals.get(&binding),
            Some(
                StoredLocal::Home { .. }
                    | StoredLocal::Handle(_)
                    | StoredLocal::FieldAlias { .. }
                    | StoredLocal::ReceivedNullable { .. }
                    | StoredLocal::Consumed
            )
        )
    }

    /// The binding is a live received nullable: a `NullableObject` call
    /// result the caller owns conditionally. Unlike `field_read_receiver`
    /// this lane answers without the non-null narrowing mark — a borrowed
    /// actual carries both runtime states, so the source classifier only
    /// needs the stored class itself.
    pub(super) fn is_received_nullable(&self, binding: BindingRefV1) -> bool {
        matches!(
            self.locals.get(&binding),
            Some(StoredLocal::ReceivedNullable { .. })
        )
    }

    pub(super) fn consume_home(&mut self, binding: BindingRefV1) {
        self.store(binding, StoredLocal::Consumed);
    }
    pub(super) fn install_map(&mut self, binding: BindingRefV1) {
        self.store(binding, StoredLocal::Map);
    }

    pub(super) fn install_i64_call_result(&mut self, binding: BindingRefV1) {
        self.store(
            binding,
            StoredLocal::Trivial(Some(SourceScalarKind::Integer)),
        );
    }

    pub(super) fn install_dynamic_invocation_result(
        &mut self,
        binding: BindingRefV1,
        frame: std::rc::Rc<super::CheckedInitializerExpressionFrameV1>,
    ) {
        self.store(binding, StoredLocal::DynamicInvocationResult { frame });
    }

    /// A `me.<ArrayBox field>.m(..)` result proven scalar by the generated
    /// core-method manifest row — the class comes from the contract, never
    /// from MIR types or runtime layout.
    pub(super) fn install_scalar_call_result(
        &mut self,
        binding: BindingRefV1,
        kind: SourceScalarKind,
    ) {
        self.store(binding, StoredLocal::Trivial(Some(kind)));
    }

    pub(super) fn install_uninitialized(&mut self, binding: BindingRefV1) {
        self.store(binding, StoredLocal::Uninitialized);
    }

    /// Record a call-result local the flow cannot classify further. The
    /// binding stays observable for `new` arguments; it joins no Home or
    /// scalar accounting.
    pub(super) fn install_bound_value(&mut self, binding: BindingRefV1) {
        self.store(binding, StoredLocal::BoundValue);
    }

    /// A local bound to an inventoried call keeps a produced value at
    /// later sites even though the flow cannot classify it. Record the
    /// binding so `new` argument observation can describe it;
    /// non-inventoried initializers stay unobservable.
    pub(super) fn install_inventoried_call_result(
        &mut self,
        binding: BindingRefV1,
        site: &SourceExprSiteV1,
    ) {
        let function = self.input.function();
        let inventoried = function
            .method_calls()
            .any(|(call_site, _)| call_site == site)
            || function
                .direct_call_observations()
                .any(|(call_site, _)| call_site == site);
        if inventoried {
            self.install_bound_value(binding);
        }
    }

    pub(super) fn install_selected_normal_home(
        &mut self,
        binding: BindingRefV1,
        acquisition: super::OwnedExprSiteV1,
        construction_source: Option<
            std::rc::Rc<
                crate::mir::normal_callable_semantic_package::ConstructionSourceStructureV1,
            >,
        >,
    ) {
        self.store(
            binding,
            StoredLocal::Home {
                acquisition,
                construction_source,
                ready: BTreeMap::new(),
            },
        );
    }

    pub(super) fn install_observed(&mut self, binding: BindingRefV1, value: OrdinaryObservation) {
        let stored = match value {
            OrdinaryObservation::Handle(root) => StoredLocal::Handle(root),
            OrdinaryObservation::TaggedDynamicLocal(_) => StoredLocal::TaggedDynamic,
            OrdinaryObservation::Integer(_) => {
                StoredLocal::Trivial(Some(SourceScalarKind::Integer))
            }
            OrdinaryObservation::Bool(_) => StoredLocal::Trivial(Some(SourceScalarKind::Bool)),
            OrdinaryObservation::TrivialLocal(_, kind) => StoredLocal::Trivial(kind),
            OrdinaryObservation::BoundValue(_) => StoredLocal::BoundValue,
            OrdinaryObservation::Null => StoredLocal::Null,
        };
        self.store(binding, stored);
    }
}

fn stored_local_kind(local: Option<&StoredLocal>) -> &'static str {
    match local {
        Some(StoredLocal::Text { .. }) => "Text",
        Some(StoredLocal::TextBranch { .. }) => "TextBranch",
        Some(StoredLocal::LoopText { .. }) => "LoopText",
        Some(StoredLocal::StaticText { .. }) => "StaticText",
        Some(_) => "Other",
        None => "Absent",
    }
}

#[cfg(test)]
#[path = "home_prefix_received_call_tests.rs"]
mod received_call_tests;

#[path = "home_prefix_named_array.rs"]
mod named_array;
#[path = "home_prefix_received_call.rs"]
mod received_call;
