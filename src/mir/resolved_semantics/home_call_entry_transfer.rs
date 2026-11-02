//! Move binding state and source-proven Call-entry Home transfer.
//! Argument evaluation retains the original Home position until entry.
use super::*;
use super::super::{BorrowedCallActualRequestV1, BorrowedCallArgumentsV1, HomePrefixUnavailableV1, LocalCallObservationV1};

impl PrefixLocalFlow<'_> {
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn consume_home(&mut self, binding: BindingRefV1) {
        self.store(binding, StoredLocal::Consumed);
    }

    pub(in crate::mir::resolved_semantics::home_new_prefix) fn was_moved_out(&self, binding: BindingRefV1) -> bool {
        matches!(self.locals.get(&binding), Some(StoredLocal::MovedOut))
    }

    pub(in crate::mir::resolved_semantics::home_new_prefix) fn has_moved_out(&self) -> bool {
        self.locals.values().any(|local| matches!(local, StoredLocal::MovedOut))
    }

    /// Transfer the existing Home at its original cleanup position. The
    /// caller has already checked the source site and lexical scope.
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn transfer_home_v1(
        &mut self,
        from: BindingRefV1,
        to: BindingRefV1,
        homes: &mut [LiveHomeV1],
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
        homes[index] = LiveHomeV1::Moved { origin, binding: to };
        self.store(from, StoredLocal::MovedOut);
        self.store(to, local);
        true
    }

    /// Validate the original expression order without changing its retained
    /// actual snapshots or the caller's pre-entry cleanup responsibilities.
    fn validate_pending_call_uses_v1(&self, call: &OwnedExprSiteV1) -> Result<(), HomePrefixUnavailableV1> {
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
            let moved = input.function().expression_source().unary(&site).filter(|unary|
                unary.operator() == crate::mir::resolved_semantics::ResolvedUnaryOperatorV1::Move);
            if after {
                if let Some(moved) = moved {
                    let Some(ResolvedLexicalRefV1::Local(binding)) = input.function().variable_ref(moved.operand())
                        else { return Err(HomePrefixUnavailableV1::MoveUnsupported(site)); };
                    if simulated.home_acquisition(binding).is_none() {
                        return Err(HomePrefixUnavailableV1::MoveUnsupported(site));
                    }
                    simulated.store(binding, StoredLocal::MovedOut);
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
            if reference.into_iter().chain(root).any(|binding| simulated.was_moved_out(binding)) {
                return Err(HomePrefixUnavailableV1::UseAfterMove(site));
            }
            if let Some(method) = input.function().method_call_source_v1(&site) {
                if matches!(method.receiver(), crate::mir::resolved_semantics::ResolvedMethodCallReceiverSourceV1::Lexical(
                    ResolvedLexicalRefV1::Local(binding)) if simulated.was_moved_out(binding)) {
                    return Err(HomePrefixUnavailableV1::UseAfterMove(method.receiver_site().clone()));
                }
            }
            let children = expression_children_v1(&ledger, input.body_shape(), &site)
                .map_err(|_| HomePrefixUnavailableV1::SourceMismatch)?;
            let ExpressionChildrenV1::Known(children) = children else {
                return Err(HomePrefixUnavailableV1::MoveUnsupported(site));
            };
            if children.iter().any(|child| child.reach != ExpressionChildReachV1::Always) {
                return Err(HomePrefixUnavailableV1::MoveUnsupported(site));
            }
            pending.push((site, true));
            pending.extend(children.into_iter().rev().map(|child| (child.site, false)));
        }
        Ok(())
    }

    pub(in crate::mir::resolved_semantics::home_new_prefix) fn commit_statement_takes_v1<E>(
        &mut self,
        statement: &SourceStmtSiteV1,
        calls: &[LocalCallObservationV1],
        homes: &mut Vec<LiveHomeV1>,
        unavailable: &mut Option<HomePrefixUnavailableV1>,
        port: &mut impl FnMut(&OwnedExprSiteV1, BorrowedCallActualRequestV1<'_>)
            -> Result<Option<BorrowedCallArgumentsV1>, E>,
    ) -> Result<(), E> {
        let mut seen = BTreeSet::new();
        for call in calls.iter().filter(|call| call.statement() == statement) {
            if !seen.insert(call.site().clone()) { continue; }
            let entry = port(call.site(), BorrowedCallActualRequestV1::CallEntryHomes(self.input))?;
            let Some(BorrowedCallArgumentsV1::CallEntryHomes { taken, preserves_reads }) = entry else {
                self.invalidate_declared_read_call_v1(statement);
                continue;
            };
            // Preservation is source-proved for the entire callee closure.
            // Unknown effects and any entry transfer keep the old invalidation.
            if !preserves_reads || !taken.is_empty() {
                self.invalidate_declared_read_call_v1(statement);
            }
            if !taken.is_empty() || self.has_moved_out() {
                if let Err(issue) = self.validate_pending_call_uses_v1(call.site()) {
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
            for (binding, _, _) in taken.iter() { self.store(*binding, StoredLocal::MovedOut); }
            homes.retain(|home| !bindings.contains(&home.binding()));
        }
        Ok(())
    }
}
