//! Complete incoming validation for the existing borrowed entry owner.
//! The default requires execution globally; only qualified Static scopes execution.
use super::*;

impl OrdinaryNewClaimLedgerV1 {
    pub(super) fn checked_borrowed_entry_incoming_core_v1<'a>(
        &'a self,
        source: &'a PreparedBorrowedFormalIngressV1,
        owner: FunctionOwnerIdV1,
        scoped_static: bool,
    ) -> Result<Box<[(&'a OwnedExprSiteV1, &'a [PreparedBorrowedFormalActualV1])]>, String> {
        let mut seen = BTreeSet::new();
        let mut incoming = Vec::new();
        let owner_view = self.checked_entry_owner_view_v1(source, owner)?;
        let mut calls: Vec<_> = source.incoming.iter().collect();
        if let Some(cohort) = source.target_static.get(&owner) {
            calls.extend(cohort.incoming.iter());
        } else if source.source_only_definitions.contains_key(&owner) {
            if let Some(view) = &owner_view {
                calls.extend(view.incoming.iter().copied());
            }
        }
        let execution = scoped_static
            .then(|| self.checked_static_execution_owners_v1(source, owner, &calls))
            .transpose()?;
        for call in calls {
            let definition = source.definitions.get(&call.callee).or_else(|| {
                (call.callee == owner)
                    .then(|| owner_view.as_ref().map(|view| view.definition))
                    .flatten()
            });
            let Some(definition) = definition else {
                return Err(freeze("borrowed-entry/incoming-identity"));
            };
            if !seen.insert(&call.call) {
                return Err(freeze("borrowed-entry/incoming-identity"));
            }
            let target_roots: BTreeSet<_> = definition.origins.values().copied().collect();
            let argument_roots: BTreeSet<_> = call
                .arguments
                .iter()
                .map(|(_, _, formal)| *formal)
                .collect();
            if target_roots != argument_roots
                || argument_roots.len() != call.arguments.len()
                || call.arguments.windows(2).any(|rows| rows[0].0 >= rows[1].0)
            {
                return Err(freeze("borrowed-entry/incoming-cardinality"));
            }
            let actuals = self
                .borrowed_formal_actuals
                .get(&call.call)
                .ok_or_else(|| freeze("borrowed-entry/actuals-missing"))?
                .as_ref()
                .map_err(Clone::clone)?;
            if actuals.is_entry_only_static_bool_v1() {
                self.checked_completed_static_bool_entry_v1(source, owner)?
                    .ok_or_else(|| freeze("borrowed-entry/bool-entry-proof-missing"))?;
            }
            if execution
                .as_ref()
                .is_some_and(|owners| !owners.contains(&call.callee))
            {
                let contracts = self
                    .original_parameter_contracts
                    .as_deref()
                    .ok_or_else(|| freeze("borrowed-static/original-catalog-missing"))?;
                if super::super::borrowed_formal_actuals::revalidate_passive_object_incoming_v1(
                    source, contracts, call, actuals,
                )? {
                    continue;
                }
            }
            actuals.ordered_arguments_for_entry_v1(call)?;
            let actuals = &actuals.opaque_actuals;
            for actual in actuals.iter() {
                if definition.origins.get(&actual.formal) != Some(&actual.formal) {
                    return Err(freeze("borrowed-entry/actuals-identity"));
                }
                // A minted object view binds every incoming actual to one
                // agreed class — a sealed actual naming a different class
                // (or no class at all) is drift, never a second authority.
                if let Some(view) = source.object_views.get(&actual.formal) {
                    let agrees = match &actual.source {
                        super::super::borrowed_formal_actuals::BorrowedFormalActualSourceV1::ReceivedNullable { class, .. }
                        | super::super::borrowed_formal_actuals::BorrowedFormalActualSourceV1::TypedHome { class, .. }
                        | super::super::borrowed_formal_actuals::BorrowedFormalActualSourceV1::EntryReceiver { class, .. }
                        | super::super::borrowed_formal_actuals::BorrowedFormalActualSourceV1::DeclaredFormal { class, .. } => {
                            class.as_ref() == view.class()
                        }
                        super::super::borrowed_formal_actuals::BorrowedFormalActualSourceV1::Null => true,
                        super::super::borrowed_formal_actuals::BorrowedFormalActualSourceV1::Forwarded { formal, .. } => {
                            source
                                .object_views
                                .get(formal)
                                .is_some_and(|origin| origin.class() == view.class())
                        }
                        _ => false,
                    };
                    if !agrees {
                        return Err(freeze("borrowed-entry/object-view-drift"));
                    }
                }
            }
            if call.callee == owner {
                incoming.push((&call.call, actuals.as_ref()));
            }
        }
        if incoming.is_empty() {
            return Err(freeze("borrowed-entry/incoming-missing"));
        }
        Ok(incoming.into_boxed_slice())
    }
}
