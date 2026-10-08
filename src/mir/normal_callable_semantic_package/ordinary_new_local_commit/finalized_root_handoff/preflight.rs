//! Borrow original source inventories and finished cleanup before any affine move.
use super::*;
use crate::mir::resolved_semantics::home_new_prefix::ObjectReturnAcquisitionV1;

impl OrdinaryNewClaimLedgerV1 {
    pub(super) fn preflight_finalized_root_exits(
        &self,
        module: &crate::mir::MirModule,
        root_owner: FunctionOwnerIdV1,
        root_symbol: &str,
        root_projection: &physical_boundary::FinishedBindings,
    ) -> Result<BTreeSet<SourceStmtSiteV1>, String> {
        let mut symbols = BTreeSet::from([root_symbol.to_owned()]);
        let mut owners = BTreeSet::from([root_owner]);
        let root_calls = self.preflight_finished_owner_exits(
            module,
            root_owner,
            root_symbol,
            root_projection,
            true,
        )?;
        let children = self.child_physical_validation.borrow();
        for (owner, state) in children.iter() {
            let ChildPhysicalValidation::FinishingChecked { symbol, projection } = state else {
                return Err(freeze("artifact-child-not-finished"));
            };
            if !owners.insert(*owner) || !symbols.insert(symbol.clone()) {
                return Err(freeze("artifact-child-finished-identity"));
            }
            self.preflight_finished_owner_exits(module, *owner, symbol, projection, false)?;
        }
        if self
            .root_exits
            .borrow()
            .iter()
            .any(|((owner, _), progress)| {
                matches!(progress, RootHomeExitProgress::Emitted { .. }) && !owners.contains(owner)
            })
        {
            return Err(freeze("artifact-exit-owner-not-finished"));
        }
        if !self.local_call_bindings_consumed() {
            return Err(freeze("artifact-local-call-unconsumed"));
        }
        Ok(root_calls)
    }

    fn preflight_finished_owner_exits(
        &self,
        module: &crate::mir::MirModule,
        owner: FunctionOwnerIdV1,
        symbol: &str,
        projection: &physical_boundary::FinishedBindings,
        root: bool,
    ) -> Result<BTreeSet<SourceStmtSiteV1>, String> {
        let function = module
            .functions
            .get(symbol)
            .filter(|function| function.signature.name == symbol)
            .ok_or_else(|| freeze("final-cleanup/function-missing"))?;
        let completion = self
            .completion_for_owner(owner)
            .filter(|completion| completion.owner() == owner)
            .ok_or_else(|| freeze("artifact-owner-completion-unavailable"))?;
        let object_selected = self.has_pending_object_return_v1(owner)
            || self
                .normal_return_dispositions
                .as_ref()
                .is_some_and(|rows| rows.keys().any(|(actual, _)| *actual == owner));
        let construction_ready = self.object_return_construction_ready_v1(owner)?;
        if object_selected && !construction_ready {
            return Err(freeze("artifact-object-return-source-unavailable"));
        }
        let all_exits_ready = completion
            .cleanup()
            .root_flow()
            .is_some_and(|flow| flow.all_exits_ready());
        let mut calls = BTreeSet::new();
        let exits = self.root_exits.borrow();
        // Enumerate the source first: iterating physical rows would hide a missing sibling.
        for exit in completion.explicit_sites() {
            let terminal = self.terminal_relation_for_owner_at(owner, exit);
            if terminal.is_some_and(|row| row.owner() != owner || row.return_site() != exit) {
                return Err(freeze("artifact-root-site-drift"));
            }
            let owned = match terminal {
                Some(TerminalRelationV1::Value(value)) => match value.returned() {
                    TerminalReturnedSourceV1::OwnedCall(call) => Some(call),
                    _ => None,
                },
                _ => None,
            };
            let normal = self.normal_exit_projection_v1(owner, exit)?;
            if owned.is_some() && normal.is_none() {
                return Err(freeze("artifact-object-return-projection-missing"));
            }
            let call_source = self.verified_terminal_call_source_v1(owner, exit)?;
            let direct = owned.is_some_and(|call| {
                matches!(call.acquisition(), ObjectReturnAcquisitionV1::Direct { .. })
            });
            if direct && call_source.is_none() {
                return Err(freeze("direct-result-source-missing"));
            }
            let call_required = call_source.is_some();
            if call_required {
                calls.insert(exit.clone());
            }
            let selected = owned.is_some() || call_required;
            let progress = exits.get(&(owner, exit.clone()));
            let Some(RootHomeExitProgress::Emitted {
                order,
                origins,
                bindings,
                entry,
            }) = progress
            else {
                if selected
                    || (construction_ready
                        && all_exits_ready
                        && !matches!(progress, Some(RootHomeExitProgress::Unavailable)))
                {
                    return Err(freeze(
                        if matches!(progress, Some(RootHomeExitProgress::Unavailable)) {
                            "root-call-entry-unavailable"
                        } else {
                            "artifact-call-physical-missing"
                        },
                    ));
                }
                continue; // Only the original unselected source-only boundary may remain unavailable.
            };
            if call_required != matches!(entry, RootHomeExitEntry::Call { .. })
                || (owned.is_some() && !direct && !matches!(entry, RootHomeExitEntry::Plain { .. }))
            {
                return Err(freeze("artifact-call-terminal-drift"));
            }
            if order.normal()
                != origins
                    .iter()
                    .map(|row| row.origin().clone())
                    .collect::<Vec<_>>()
            {
                return Err(freeze("root-exit-origin-order"));
            }
            let rebound = root && call_required;
            if rebound {
                self.validate_rebound_call_entry(
                    owner, exit, function, projection, entry, bindings,
                )?;
            } else {
                self.validate_call_entry(owner, exit, function, Some(projection), entry, bindings)?;
            }
            super::super::finalized_root_cleanup::validate_finished_cleanup_entry(
                self,
                owner,
                exit,
                function,
                projection,
                order,
                entry,
                bindings,
                if rebound { None } else { Some(projection) },
            )?;
        }
        for ((actual, exit), progress) in exits.iter().filter(|((actual, _), _)| *actual == owner) {
            if *actual != owner || !completion.explicit_sites().contains(exit) {
                return Err(freeze("artifact-call-terminal-drift"));
            }
            if matches!(
                progress,
                RootHomeExitProgress::Emitted {
                    entry: RootHomeExitEntry::Call { .. },
                    ..
                }
            ) && !calls.contains(exit)
            {
                return Err(freeze("artifact-call-terminal-drift"));
            }
        }
        Ok(calls)
    }
}
