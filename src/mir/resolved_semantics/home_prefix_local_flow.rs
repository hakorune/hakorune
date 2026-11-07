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
#[path = "home_call_entry_transfer.rs"]
mod call_entry_transfer;
#[path = "home_prefix_borrowed_nullable.rs"]
mod borrowed_nullable;
#[path = "home_prefix_borrowed_flow.rs"]
mod borrowed_flow;
pub(crate) use borrowed_flow::{BorrowedReadFlowV1, BorrowedFlowReadV1};
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
    BranchNormalTextSourceV1, LoopHeaderValueBasisV1, LoopTextReadV1, SourceLoopTextProducerV1,
};
#[path = "home_prefix_text.rs"]
mod text;
#[path = "home_prefix_tagged_dynamic.rs"]
mod tagged_dynamic;
pub(crate) use tagged_dynamic::{TaggedDynamicReadV1, TaggedDynamicValueV1};
#[path = "home_prefix_stored_local.rs"]
mod stored_local;
#[path = "home_prefix_observe.rs"]
mod observe;
#[path = "home_prefix_entry_install.rs"]
mod entry_install;
#[path = "home_prefix_text_join.rs"]
mod text_join;
use stored_local::{stored_local_kind, stored_local_same};
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
                    source == other
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
    /// An independently owned Shared handle, not the payload's New Home.
    Shared {
        acquisition: super::OwnedExprSiteV1,
        payload_acquisition: super::OwnedExprSiteV1,
        construction_source: std::rc::Rc<crate::mir::normal_callable_semantic_package::ConstructionSourceStructureV1>,
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
    BorrowedNullable(borrowed_nullable::BorrowedNullableStateV1),
    StoredMapGet(std::rc::Rc<super::MapFieldGetLocalV1>),
    ReceivedMixed(Box<crate::mir::normal_callable_semantic_package::MixedLocalCallSourceLoanV1>),
    Map,
    /// A `: MapBox` declared formal — caller-owned map storage borrowed
    /// read-only for the call. It reads like a live map but owns nothing:
    /// no End obligation, no ownership transfer back, no map-local return.
    BorrowedMap,
    Consumed,
    /// Explicit source move, distinct from the legacy implicit take path.
    MovedOut,
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
    /// Exact Shared handle and original payload acquisition; a read must
    /// borrow the payload through this handle, never reinterpret the handle.
    Shared { acquisition: OwnedExprSiteV1, payload: OwnedExprSiteV1 },
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
    ) -> super::expression_plan::ExpressionPlanInstallationV1<'_, '_> {
        super::expression_plan::ExpressionPlanInstallationV1::new(
            self.input.owner(),
            self,
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



    // Declaration contracts are borrowed from the sole package issuer. No
    // physical signature or default capability participates in entry admission.













    pub(super) fn install_map(&mut self, binding: BindingRefV1) {
        self.store(binding, StoredLocal::Map);
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


#[cfg(test)]
#[path = "home_prefix_received_call_tests.rs"]
mod received_call_tests;

#[path = "home_prefix_named_array.rs"]
mod named_array;
#[path = "home_prefix_received_call.rs"]
mod received_call;
#[path = "home_prefix_local_flow_shared.rs"]
mod shared;
