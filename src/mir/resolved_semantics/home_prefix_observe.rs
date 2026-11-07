//! Read-only observation of stored locals: what a source site is allowed to read or move.
use super::*;

impl<'source> PrefixLocalFlow<'source> {
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn observe(&self, site: &SourceExprSiteV1) -> Option<OrdinaryObservation> {
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
            StoredLocal::BorrowedNullable(state) => state.legacy_observation_v1(binding),
            StoredLocal::StoredMapGet(_) => Some(OrdinaryObservation::BoundValue(binding)),
            StoredLocal::ReceivedMixed(_) => Some(OrdinaryObservation::BoundValue(binding)),
            StoredLocal::Handle(root)
                if !matches!(self.locals.get(root), Some(StoredLocal::Consumed | StoredLocal::MovedOut)) =>
            {
                Some(OrdinaryObservation::Handle(*root))
            }
            StoredLocal::Handle(_) | StoredLocal::Consumed | StoredLocal::MovedOut => None,
            StoredLocal::TaggedDynamic => self
                .tagged_dynamic_read_v1(site)
                .map(|_| OrdinaryObservation::TaggedDynamicLocal(binding)),
            // A field-read alias names borrowed storage, never a movable
            // value — every observation lane stays fail-closed on it.
            StoredLocal::StaticText { .. }
            | StoredLocal::Shared { .. }
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

    /// Argument-position observation for one selected `new` site. Literal
    /// and local rows come from `observe` first; a `receiver.field`
    /// expression is admitted only when the caller's issuer predicate proves
    /// the field `i64` on the receiver's own source definition — the scanner
    /// never infers a field class from MIR types or runtime layout. The
    /// predicate may be invoked twice for the same site (the observation
    /// pass and the accounting pass walk the same arguments), so the issuer
    /// must answer idempotently.
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn observe_selected_argument<E>(
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
        // An explicit `move x` actual is the operand's own handle transfer.
        if let Some(moved) = self
            .input
            .function()
            .expression_source()
            .moved(site)
        {
            return Ok(self
                .observe(moved.operand())
                .and_then(|observation| observation.into_selected_argument())
                .filter(|kind| matches!(kind, SelectedNewArgumentKindV1::Handle { .. })));
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

    /// `root` is a self-rooted handle exactly when it stores itself —
    /// i.e. a parameter-installed handle, never a live Home/Map local.
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn is_self_rooted_handle(&self, root: BindingRefV1) -> bool {
        matches!(self.locals.get(&root), Some(StoredLocal::Handle(r)) if *r == root)
    }

    /// `root` is a live map-installed local — the entry borrows it by
    /// reference; the local stays the owner and still issues its own End.
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn is_map_local(&self, root: BindingRefV1) -> bool {
        matches!(self.locals.get(&root), Some(StoredLocal::Map))
    }

    /// `root` is a `: MapBox` declared formal — caller-owned map storage
    /// borrowed read-only. It participates in reads and borrowed-entry
    /// classification but owns nothing: `is_map_local` stays owned-only so
    /// a borrowed formal can never ride the map-local return lane.
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn is_borrowed_map(&self, root: BindingRefV1) -> bool {
        matches!(self.locals.get(&root), Some(StoredLocal::BorrowedMap))
    }

    /// The sealed `new` acquisition site of a live Home local root.
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn home_acquisition(&self, root: BindingRefV1) -> Option<&super::super::OwnedExprSiteV1> {
        match self.locals.get(&root)? {
            StoredLocal::Home { acquisition, .. } => Some(acquisition),
            _ => None,
        }
    }

    pub(in crate::mir::resolved_semantics::home_new_prefix) fn direct_available_home(
        &self,
        site: &SourceExprSiteV1,
    ) -> Option<(BindingRefV1, &super::super::OwnedExprSiteV1)> {
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
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn is_field_read_candidate(&self, binding: BindingRefV1) -> bool {
        matches!(
            self.locals.get(&binding),
            Some(
                StoredLocal::Home { .. }
                    | StoredLocal::Handle(_)
                    | StoredLocal::FieldAlias { .. }
                    | StoredLocal::ReceivedNullable { .. }
                    | StoredLocal::Consumed
                    | StoredLocal::MovedOut
            )
        ) || self.has_declared_read_origin_v1(binding)
    }

    /// The binding is a live received nullable: a `NullableObject` call
    /// result the caller owns conditionally. Unlike `field_read_receiver`
    /// this lane answers without the non-null narrowing mark — a borrowed
    /// actual carries both runtime states, so the source classifier only
    /// needs the stored class itself.
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn is_received_nullable(&self, binding: BindingRefV1) -> bool {
        matches!(
            self.locals.get(&binding),
            Some(StoredLocal::ReceivedNullable { .. })
        )
    }
}
