//! Move binding state and source-proven Call-entry Home transfer.
//! Argument evaluation retains the original Home position until entry.
use super::*;
use super::super::{BorrowedCallActualRequestV1, BorrowedCallArgumentsV1, HomePrefixUnavailableV1, LocalCallObservationV1};

impl PrefixLocalFlow<'_> {
    pub(in crate::mir::resolved_semantics::home_new_prefix::local_flow) fn consume_home(&mut self, binding: BindingRefV1) {
        self.store(binding, StoredLocal::Consumed);
    }

    /// An argument Home leaves the frame at the invoke's Normal edge, so later
    /// uses are `UseAfterMove` exactly as after a method-call entry transfer.
    pub(in crate::mir::resolved_semantics::home_new_prefix::local_flow) fn consume_argument_home(&mut self, binding: BindingRefV1) {
        self.store(binding, StoredLocal::MovedOut);
    }

    pub(in crate::mir::resolved_semantics::home_new_prefix) fn was_moved_out(&self, binding: BindingRefV1) -> bool {
        matches!(self.locals.get(&binding), Some(StoredLocal::MovedOut))
    }

    /// S3b-M2: the Local binding an explicit `move` names, or the refusal.
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn move_operand_binding_v1(
        &self,
        moved: &crate::mir::resolved_semantics::ResolvedUnaryExpressionSourceV1,
    ) -> Result<BindingRefV1, HomePrefixUnavailableV1> {
        match self.input.function().variable_ref(moved.operand()) {
            Some(ResolvedLexicalRefV1::Local(binding)) => Ok(binding),
            _ => Err(HomePrefixUnavailableV1::MoveUnsupported(moved.site().clone())),
        }
    }

    /// S3b-M3: reading a binding that is no longer usable is `UseAfterMove` at `site`.
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn use_after_move_v1(
        &self,
        bindings: impl IntoIterator<Item = BindingRefV1>,
        site: &SourceExprSiteV1,
    ) -> Option<HomePrefixUnavailableV1> {
        bindings
            .into_iter()
            .any(|binding| self.was_moved_out(binding))
            .then(|| HomePrefixUnavailableV1::UseAfterMove(site.clone()))
    }

    /// S3b-M2a: the binding of a local `move`, which must still be usable.
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn live_move_operand_v1(
        &self,
        moved: &crate::mir::resolved_semantics::ResolvedUnaryExpressionSourceV1,
    ) -> Result<BindingRefV1, HomePrefixUnavailableV1> {
        let binding = self.move_operand_binding_v1(moved)?;
        match self.use_after_move_v1([binding], moved.operand()) {
            Some(issue) => Err(issue),
            None => Ok(binding),
        }
    }

    /// S3b-M4: a local move or share move keeps its responsibility only inside one scope.
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn move_scope_v1(
        &self,
        destination: BindingRefV1,
        source: BindingRefV1,
        statement: &SourceStmtSiteV1,
        moved: &crate::mir::resolved_semantics::ResolvedUnaryExpressionSourceV1,
    ) -> Result<(), HomePrefixUnavailableV1> {
        if self.same_scope_v1(destination, source, statement) {
            Ok(())
        } else {
            Err(HomePrefixUnavailableV1::MoveUnsupported(moved.site().clone()))
        }
    }

    pub(in crate::mir::resolved_semantics::home_new_prefix) fn has_moved_out(&self) -> bool {
        self.locals.values().any(|local| matches!(local, StoredLocal::MovedOut))
    }

    /// S3b-R2b: move the existing Home to another name at its original cleanup
    /// position. The caller has already checked the source site and lexical scope.
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn move_alias(
        &mut self,
        from: BindingRefV1,
        to: BindingRefV1,
        homes: &mut HomesV1,
    ) -> bool {
        if from.owner() != self.input.owner() || to.owner() != self.input.owner() {
            return false;
        }
        let Some(local @ (StoredLocal::Home { .. } | StoredLocal::Shared { .. })) = self.locals.get(&from).cloned() else {
            return false;
        };
        let acquisition = match &local {
            StoredLocal::Home { acquisition, .. } | StoredLocal::Shared { acquisition, .. } => acquisition,
            _ => unreachable!(),
        };
        if homes.iter().any(|home| home.binding() != from
            && self.movable_acquisition_v1(home.binding()) == Some(acquisition)) {
            return false;
        }
        let mut matches = homes.iter().enumerate().filter(|(_, home)| home.binding() == from);
        let Some((index, home)) = matches.next() else { return false };
        if matches.next().is_some() || homes.iter().any(|home| home.binding() == to) {
            return false;
        }
        let origin = match home {
            LiveHomeV1::Stable(binding) => *binding,
            LiveHomeV1::Moved { origin, .. } => *origin,
            LiveHomeV1::Text(_) => return false,
        };
        homes.move_at(index, LiveHomeV1::Moved { origin, binding: to });
        self.consume_argument_home(from);
        self.store(to, local);
        true
    }

    /// Validate the original expression order without changing its retained
    /// actual snapshots or the caller's pre-entry cleanup responsibilities.
    /// Shared by method-call entry and `new` argument transfer.
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn validate_pending_call_uses_v1(
        &self, call: &OwnedExprSiteV1, transfer_sites: &[SourceExprSiteV1],
    ) -> Result<(), HomePrefixUnavailableV1> {
        use crate::mir::resolved_value_profile::expression_children::{expression_children_v1,
            ExpressionChildrenV1, ExpressionChildReachV1};
        let input = self.input;
        let ledger = input.forest().callable_source_ledger(input.owner())
            .map_err(|_| HomePrefixUnavailableV1::SourceMismatch)?;
        // Only the local maps are mutated. Shared source plans and observations
        // remain untouched; no simulated Call or execution row is installed.
        let mut simulated = self.clone();
        let mut pending = vec![(call.site().clone(), false)];
        while let Some((site, after)) = pending.pop() {
            let moved = input.function().expression_source().moved(&site);
            if after {
                if let Some(moved) = moved {
                    let binding = simulated.move_operand_binding_v1(moved)?;
                    if simulated.home_acquisition(binding).is_none() {
                        return Err(HomePrefixUnavailableV1::MoveUnsupported(site));
                    }
                    simulated.consume_argument_home(binding);
                } else if transfer_sites.contains(&site) {
                    // A plain `new` argument whose Home leaves the frame transfers
                    // without a `move` operator; the caller already proved the Home.
                    if let Some(ResolvedLexicalRefV1::Local(binding)) = input.function().variable_ref(&site) {
                        simulated.consume_argument_home(binding);
                    }
                }
                continue;
            }
            let reference = input.function().variable_ref(&site).and_then(|reference| match reference {
                ResolvedLexicalRefV1::Local(binding) => Some(binding), _ => None,
            });
            // Retain the alias root even after a previous statement moved it;
            // ordinary observe() intentionally hides unavailable handles.
            let root = reference.and_then(|binding| match self.locals.get(&binding) {
                Some(StoredLocal::Handle(root)) => Some(*root),
                _ => None,
            }).or_else(|| match self.observe(&site) {
                Some(OrdinaryObservation::Handle(root)) => Some(root), _ => None,
            });
            if let Some(issue) = simulated.use_after_move_v1(reference.into_iter().chain(root), &site) {
                return Err(issue);
            }
            if let Some(method) = input.function().method_call_source_v1(&site) {
                if let crate::mir::resolved_semantics::ResolvedMethodCallReceiverSourceV1::Lexical(
                    ResolvedLexicalRefV1::Local(binding)) = method.receiver() {
                    if let Some(issue) = simulated.use_after_move_v1([binding], method.receiver_site()) {
                        return Err(issue);
                    }
                }
            }
            let children: Vec<SourceExprSiteV1> = if let Some(construction) = input.function().expression_source().construction(&site) {
                // Arguments evaluate in source order, always.
                construction.arguments().to_vec()
            } else {
                let children = expression_children_v1(&ledger, input.body_shape(), &site)
                    .map_err(|_| HomePrefixUnavailableV1::SourceMismatch)?;
                let ExpressionChildrenV1::Known(children) = children else {
                    return Err(HomePrefixUnavailableV1::MoveUnsupported(site));
                };
                if children.iter().any(|child| child.reach != ExpressionChildReachV1::Always) {
                    return Err(HomePrefixUnavailableV1::MoveUnsupported(site));
                }
                children.into_iter().map(|child| child.site).collect()
            };
            pending.push((site, true));
            pending.extend(children.into_iter().rev().map(|child| (child, false)));
        }
        Ok(())
    }

    /// Commit the evaluator's final responsibility set before installing a
    /// destination. Name availability is checked separately in source order.
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn commit_expression_end_v1(
        &mut self,
        evaluation: &crate::mir::resolved_semantics::home_new_prefix::CheckedExpressionEvaluationV1,
        homes: &mut HomesV1,
    ) -> Result<(), HomePrefixUnavailableV1> {
        let ownership = evaluation.ownership_v1();
        if ownership.has_transfers() {
            if evaluation.source().owner() != self.input.owner()
                || &**homes != evaluation.prior_homes() {
                return Err(HomePrefixUnavailableV1::SourceMismatch);
            }
            let root = OwnedExprSiteV1::new(self.input.owner(), evaluation.source().nodes()[0].site().clone());
            self.validate_pending_call_uses_v1(&root, &[])?;
            for entry in ownership.entries() {
                if entry.taken.iter().any(|(_, root, acquisition)| self.home_acquisition(*root) != Some(acquisition)) {
                    return Err(HomePrefixUnavailableV1::MoveUnsupported(entry.site.site().clone()));
                }
            }
        }
        for entry in ownership.entries() {
            if !entry.preserves_reads || !entry.taken.is_empty() {
                self.invalidate_declared_read_call_v1(evaluation.source().root().statement());
            }
            for (binding, _, _) in &entry.taken { self.consume_argument_home(*binding); }
        }
        if ownership.has_transfers() { homes.commit(HomesV1::snapshot(ownership.end())); }
        Ok(())
    }

    pub(in crate::mir::resolved_semantics::home_new_prefix) fn commit_statement_takes_v1<E>(
        &mut self,
        statement: &SourceStmtSiteV1,
        calls: &[LocalCallObservationV1],
        homes: &mut HomesV1,
        unavailable: &mut Option<HomePrefixUnavailableV1>,
        port: &mut impl FnMut(&OwnedExprSiteV1, BorrowedCallActualRequestV1<'_>)
            -> Result<Option<BorrowedCallArgumentsV1>, E>,
    ) -> Result<(), E> {
        let plans = self.expression_plans();
        let mut seen = BTreeSet::new();
        for call in calls.iter().filter(|call| call.statement() == statement) {
            // These purpose installers commit their evaluation before acquiring
            // a destination. Other expression owners retain their old boundary.
            if plans.iter().any(|plan| (plan.initializer_frame().is_some()
                || plan.rebind_frame().is_some() || plan.print_frame().is_some()
                || plan.condition_frame().is_some()) && plan.evaluation().is_some_and(|evaluation|
                    evaluation.source().owner() == call.owner()
                        && evaluation.source().root().statement() == statement
                        && evaluation.ownership_v1().contains_site(call.site()))) { continue; }
            if !seen.insert(call.site().clone()) { continue; }
            let entry = port(call.site(), BorrowedCallActualRequestV1::CallEntryHomes(self.input))?;
            let Some(BorrowedCallArgumentsV1::CallEntryHomes { taken, preserves_reads, .. }) = entry else {
                self.invalidate_declared_read_call_v1(statement);
                continue;
            };
            // Preservation is source-proved for the entire callee closure.
            // Unknown effects and any entry transfer keep the old invalidation.
            if !preserves_reads || !taken.is_empty() {
                self.invalidate_declared_read_call_v1(statement);
            }
            if !taken.is_empty() || self.has_moved_out() {
                if let Err(issue) = self.validate_pending_call_uses_v1(call.site(), &[]) {
                    unavailable.get_or_insert(issue);
                    continue;
                }
            }
            let mut bindings = BTreeSet::new();
            let valid = taken.iter().all(|(binding, root, acquisition)| {
                bindings.insert(*binding)
                    && self.home_acquisition(*root) == Some(acquisition)
                    && homes.iter().filter(|home| home.binding() == *binding
                        && home.stable_origin().is_some()).count() == 1
            });
            if !valid {
                unavailable.get_or_insert_with(|| HomePrefixUnavailableV1::MoveUnsupported(call.site().site().clone()));
                continue;
            }
            // The observation retains its pre-entry prefix for argument Fault.
            let taken: Vec<BindingRefV1> = taken.iter().map(|(binding, _, _)| *binding).collect();
            self.release_to_callee(homes, &taken);
        }
        Ok(())
    }
}
