//! Local state for the existing selected-New source prefix, not general Home Flow.
//!
//! Ordinary observations cannot carry an owned Home. The source scanner alone
//! records selected Normal installations; alias initialization stores a Handle.

use super::SelectedNewArgumentKindV1;
use super::{
    BindingRefV1, ExprChildRoleV1, OwnedExprSiteV1, ResolvedLexicalRefV1, ResolvedLiteralSourceV1,
    SourceExprSiteV1,
};
use crate::ast::ASTNode;
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use std::collections::BTreeMap;

/// Source scalar class retained from an exact literal or declaration contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SourceScalarKind {
    Integer,
    Bool,
}

#[derive(Clone)]
enum StoredLocal {
    Home {
        acquisition: super::OwnedExprSiteV1,
    },
    /// A call-result handle the caller received as an owned Home. It owes
    /// the caller's exit exactly one release like a `Home`, but its
    /// acquisition is a call site — never a `new` site — so the
    /// home-acquisition lookups must not surface it.
    ReceivedHandle,
    /// A nullable receiver-call result the caller owns only when non-null.
    /// It owes the caller's exit a checked release and must never satisfy
    /// a `Handle`-only probe — the `Void` sentinel is a real outcome.
    ReceivedNullable,
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
    },
    Trivial(Option<SourceScalarKind>),
    /// A binding produced by an inventoried call expression. The value
    /// exists at later sites but carries no scalar/rooted-storage class —
    /// it is neither a trivial local nor a Home/Map/Handle root.
    BoundValue,
    /// A binding or site holding the exact source `null` literal. It is
    /// neither a trivial scalar nor a handle root — only its own fact.
    Null,
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
    Alias(Box<str>),
}

pub(super) enum OrdinaryObservation {
    Integer(i64),
    Bool(bool),
    Null,
    TrivialLocal(BindingRefV1, Option<SourceScalarKind>),
    Handle(BindingRefV1),
    BoundValue(BindingRefV1),
}

impl OrdinaryObservation {
    pub(super) fn is_trivial(&self) -> bool {
        match self {
            Self::Integer(_) | Self::Bool(_) | Self::TrivialLocal(..) => true,
            Self::Null | Self::Handle(_) | Self::BoundValue(_) => false,
        }
    }

    pub(super) fn into_selected_argument(self) -> Option<SelectedNewArgumentKindV1> {
        match self {
            Self::Integer(value) => Some(SelectedNewArgumentKindV1::Integer(value)),
            Self::Bool(value) => Some(SelectedNewArgumentKindV1::Bool(value)),
            Self::Null => Some(SelectedNewArgumentKindV1::Null),
            Self::TrivialLocal(binding, _) => Some(SelectedNewArgumentKindV1::Local { binding }),
            Self::Handle(root) => Some(SelectedNewArgumentKindV1::Handle { binding: root }),
            Self::BoundValue(binding) => Some(SelectedNewArgumentKindV1::BoundValue { binding }),
        }
    }
}

fn stored_local_same(left: &StoredLocal, right: &StoredLocal) -> bool {
    match (left, right) {
        (StoredLocal::Home { acquisition: a }, StoredLocal::Home { acquisition: b }) => a == b,
        (StoredLocal::ReceivedHandle, StoredLocal::ReceivedHandle) => true,
        (StoredLocal::ReceivedNullable, StoredLocal::ReceivedNullable) => true,
        (StoredLocal::Map, StoredLocal::Map) => true,
        (StoredLocal::BorrowedMap, StoredLocal::BorrowedMap) => true,
        (StoredLocal::Consumed, StoredLocal::Consumed) => true,
        (StoredLocal::Handle(a), StoredLocal::Handle(b)) => a == b,
        (StoredLocal::FieldAlias { class: a }, StoredLocal::FieldAlias { class: b }) => a == b,
        (StoredLocal::Trivial(a), StoredLocal::Trivial(b)) => a == b,
        (StoredLocal::BoundValue, StoredLocal::BoundValue) => true,
        (StoredLocal::Null, StoredLocal::Null) => true,
        (StoredLocal::Uninitialized, StoredLocal::Uninitialized) => true,
        _ => false,
    }
}

#[derive(Clone)]
pub(super) struct PrefixLocalFlow<'source> {
    input: ResolvedFunctionLoweringInputV1<'source>,
    locals: BTreeMap<BindingRefV1, StoredLocal>,
}

impl<'source> PrefixLocalFlow<'source> {
    pub(super) fn new(input: ResolvedFunctionLoweringInputV1<'source>) -> Self {
        Self {
            input,
            locals: BTreeMap::new(),
        }
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
        for (binding, value) in &other.locals {
            if let Some(own) = self.locals.get(binding) {
                if !stored_local_same(own, value) {
                    return false;
                }
            }
        }
        for binding in self.locals.keys().copied().collect::<Vec<_>>() {
            if !other.locals.contains_key(&binding) {
                self.locals.insert(binding, StoredLocal::Uninitialized);
            }
        }
        for binding in other.locals.keys() {
            self.locals
                .entry(*binding)
                .or_insert(StoredLocal::Uninitialized);
        }
        true
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
            use crate::mir::exact_trivial_parameter_abi::ExactTrivialParameterAbiV1;
            let value = match kind {
                CallableParameterContractKindV1::ExactTrivial(abi)
                    if abi == ExactTrivialParameterAbiV1::I64 =>
                {
                    StoredLocal::Trivial(Some(SourceScalarKind::Integer))
                }
                CallableParameterContractKindV1::ExactTrivial(_) => return false,
                CallableParameterContractKindV1::Map => StoredLocal::BorrowedMap,
                CallableParameterContractKindV1::OpaqueHandle
                | CallableParameterContractKindV1::DeclaredHandle
                | CallableParameterContractKindV1::ExactText(_) => StoredLocal::Handle(binding),
            };
            self.locals.insert(binding, value);
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
            | StoredLocal::ReceivedHandle
            | StoredLocal::Map
            | StoredLocal::BorrowedMap => Some(OrdinaryObservation::Handle(binding)),
            // A nullable result is a produced value, never a handle root —
            // the `Void` sentinel arm keeps every Handle-classified use
            // fail-closed while `new` argument observation can still name
            // the binding's value.
            StoredLocal::ReceivedNullable => Some(OrdinaryObservation::BoundValue(binding)),
            StoredLocal::Handle(root)
                if !matches!(self.locals.get(root), Some(StoredLocal::Consumed)) =>
            {
                Some(OrdinaryObservation::Handle(*root))
            }
            StoredLocal::Handle(_) | StoredLocal::Consumed => None,
            // A field-read alias names borrowed storage, never a movable
            // value — every observation lane stays fail-closed on it.
            StoredLocal::FieldAlias { .. } => None,
            StoredLocal::Trivial(kind) => Some(OrdinaryObservation::TrivialLocal(binding, *kind)),
            StoredLocal::BoundValue => Some(OrdinaryObservation::BoundValue(binding)),
            StoredLocal::Null => Some(OrdinaryObservation::Null),
            StoredLocal::Uninitialized => None,
        }
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
        if loan.owner() != self.input.owner() {
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
        self.locals.insert(receiver, StoredLocal::Handle(receiver));
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
            StoredLocal::Home { acquisition } => Some(acquisition),
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
            StoredLocal::Home { acquisition } => Some((binding, acquisition)),
            _ => None,
        }
    }
    /// The receiver class provenance for a `receiver.field` initializer
    /// read: `OwnedHome` for a claim-local `new` local, `RootedHandle` for
    /// a handle alias or self-rooted entry/param binding (the stored root
    /// names the movable handle root the candidate/entry proof consults),
    /// `Alias` for a binding produced by an earlier proven field read — its
    /// declared class is the receiver's own class authority.
    pub(super) fn field_read_receiver(
        &self,
        binding: BindingRefV1,
    ) -> Option<FieldReadReceiverV1> {
        match self.locals.get(&binding)? {
            StoredLocal::Home { .. } => Some(FieldReadReceiverV1::OwnedHome),
            StoredLocal::Handle(root) => Some(FieldReadReceiverV1::RootedHandle(*root)),
            StoredLocal::FieldAlias { class } => {
                Some(FieldReadReceiverV1::Alias(class.clone()))
            }
            _ => None,
        }
    }

    /// A proven `receiver.field` read whose declared type is an ordinary
    /// box: the binding is a borrowed alias — no Home membership, no exit
    /// obligation, no observable move.
    pub(super) fn install_field_alias(&mut self, binding: BindingRefV1, class: Box<str>) {
        self.locals.insert(binding, StoredLocal::FieldAlias { class });
    }

    pub(super) fn consume_home(&mut self, binding: BindingRefV1) {
        self.locals.insert(binding, StoredLocal::Consumed);
    }
    pub(super) fn install_map(&mut self, binding: BindingRefV1) {
        self.locals.insert(binding, StoredLocal::Map);
    }

    /// A received call-result handle: the caller owns it as a Home but it
    /// carries no `new` acquisition site, so it installs on its own arm.
    pub(super) fn install_received_handle(&mut self, binding: BindingRefV1) {
        self.locals.insert(binding, StoredLocal::ReceivedHandle);
    }

    /// A received nullable call result: the caller owns it conditionally —
    /// the exit chain owes a checked release, and no acquisition site or
    /// scalar class ever applies.
    pub(super) fn install_received_nullable(&mut self, binding: BindingRefV1) {
        self.locals.insert(binding, StoredLocal::ReceivedNullable);
    }

    pub(super) fn install_i64_call_result(&mut self, binding: BindingRefV1) {
        self.locals.insert(
            binding,
            StoredLocal::Trivial(Some(SourceScalarKind::Integer)),
        );
    }

    /// A `me.<ArrayBox field>.m(..)` result proven scalar by the generated
    /// core-method manifest row — the class comes from the contract, never
    /// from MIR types or runtime layout.
    pub(super) fn install_scalar_call_result(
        &mut self,
        binding: BindingRefV1,
        kind: SourceScalarKind,
    ) {
        self.locals
            .insert(binding, StoredLocal::Trivial(Some(kind)));
    }

    pub(super) fn install_uninitialized(&mut self, binding: BindingRefV1) {
        self.locals.insert(binding, StoredLocal::Uninitialized);
    }

    /// Record a call-result local the flow cannot classify further. The
    /// binding stays observable for `new` arguments; it joins no Home or
    /// scalar accounting.
    pub(super) fn install_bound_value(&mut self, binding: BindingRefV1) {
        self.locals.insert(binding, StoredLocal::BoundValue);
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
    ) {
        self.locals
            .insert(binding, StoredLocal::Home { acquisition });
    }

    pub(super) fn install_observed(&mut self, binding: BindingRefV1, value: OrdinaryObservation) {
        let stored = match value {
            OrdinaryObservation::Handle(root) => StoredLocal::Handle(root),
            OrdinaryObservation::Integer(_) => {
                StoredLocal::Trivial(Some(SourceScalarKind::Integer))
            }
            OrdinaryObservation::Bool(_) => StoredLocal::Trivial(Some(SourceScalarKind::Bool)),
            OrdinaryObservation::TrivialLocal(_, kind) => StoredLocal::Trivial(kind),
            OrdinaryObservation::BoundValue(_) => StoredLocal::BoundValue,
            OrdinaryObservation::Null => StoredLocal::Null,
        };
        self.locals.insert(binding, stored);
    }
}
