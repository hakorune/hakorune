//! Access and one-shot consumption for source-issued terminal relations.
//!
//! Terminal relations are keyed by the exact source exit statement site.
//! Owner-only lookups that collapse to "the sole row" are gone: a caller
//! must name the exit site it is lowering, or deliberately iterate every
//! retained relation for one owner.
use super::*;
use crate::mir::resolved_semantics::{SourceNodeSiteV1, SourceStmtSiteV1};

impl OrdinaryNewClaimLedgerV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn root_owner(
        &self,
    ) -> Option<crate::mir::resolved_semantics::FunctionOwnerIdV1> {
        self.root_completion
            .as_ref()
            .and_then(|row| row.as_ref().ok())
            .map(|completion| completion.owner())
    }

    /// Every retained terminal relation for `owner`, in exit-site order.
    /// Callers that need the evidence of one exit must use the `*_at`
    /// lookup instead of picking a row out of this list.
    pub(in crate::mir::normal_callable_semantic_package) fn terminal_relations_for_owner(
        &self,
        owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
    ) -> Vec<&TerminalRelationV1> {
        if let Some(index) = self.terminal_relation_index.get(&owner) {
            return index.values().collect();
        }
        self.terminal_relation
            .values()
            .filter(|relation| relation.owner() == owner)
            .collect()
    }

    /// The retained relation at one exact source exit site for `owner`.
    /// `None` is not a fallback for "some other exit" — it means this exit
    /// has no retained relation.
    pub(in crate::mir::normal_callable_semantic_package) fn terminal_relation_for_owner_at(
        &self,
        owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
        site: &SourceStmtSiteV1,
    ) -> Option<&TerminalRelationV1> {
        if let Some(index) = self.terminal_relation_index.get(&owner) {
            return index.get(site);
        }
        self.terminal_relation
            .get(site)
            .filter(|relation| relation.owner() == owner)
    }

    /// Every Call terminal relation for `owner`, in exit-site order.
    pub(in crate::mir::normal_callable_semantic_package) fn call_relations_for_owner(
        &self,
        owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
    ) -> Vec<&crate::mir::resolved_semantics::home_new_prefix::TerminalI64CallReturnV1> {
        self.terminal_relations_for_owner(owner)
            .into_iter()
            .filter_map(|relation| match relation {
                TerminalRelationV1::Call(call) => Some(call),
                _ => None,
            })
            .collect()
    }

    /// Nullable result-class provenance: `Some(class)` only when every
    /// sealed exit is `new class(...)` or the exact `null` literal. This
    /// is a claim product only — no Handle, lifecycle, or physical ABI
    /// authorization follows from it.
    #[cfg(test)]
    pub(crate) fn nullable_callable_result_class(
        &self,
        key: &hakorune_mir_defs::CanonicalSameModuleCallableKeyV1,
    ) -> Option<&str> {
        match self.callable_result_classes.get(key) {
            Some(result_class_claim::OrdinaryNewResultClassV1::NullableObject(class)) => {
                Some(class.as_ref())
            }
            _ => None,
        }
    }

    #[cfg(test)]
    pub(in crate::mir::normal_callable_semantic_package) fn callable_result_class_claims_for_test(
        &self,
    ) -> &result_class_claim::OrdinaryNewResultClassClaimsV1 {
        &self.callable_result_classes
    }

    /// Claim-faithful `local x = me.m(..)` call-result evidence — the
    /// site-keyed index minted by the deferred claim-aware pass. Evidence
    /// only: no Handle authorization.
    #[cfg(test)]
    pub(in crate::mir::normal_callable_semantic_package) fn receiver_call_observations_for_test(
        &self,
    ) -> &BTreeMap<OwnedExprSiteV1, super::ReceiverCallClassObservationV1> {
        &self.receiver_call_observations
    }

    /// Test-only: the sole retained relation for `owner` — `None` when the
    /// owner keeps zero or several exit relations, never an arbitrary pick.
    #[cfg(test)]
    pub(crate) fn sole_terminal_relation_for_owner(
        &self,
        owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
    ) -> Option<&TerminalRelationV1> {
        let relations = self.terminal_relations_for_owner(owner);
        match relations.as_slice() {
            [relation] => Some(relation),
            _ => None,
        }
    }

    /// Test-only sole-row shorthands over `terminal_relations_for_owner`.
    #[cfg(test)]
    pub(crate) fn terminal_call_arguments_for_owner(
        &self,
        owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
    ) -> Option<&[crate::mir::resolved_semantics::home_new_prefix::TerminalCallArgumentV1]> {
        match self.call_relations_for_owner(owner).as_slice() {
            [call] => Some(call.arguments()),
            _ => None,
        }
    }

    #[cfg(test)]
    pub(crate) fn terminal_i64_add_return_for_owner(
        &self,
        owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
    ) -> Option<&TerminalI64AddReturnV1> {
        match self.sole_terminal_relation_for_owner(owner) {
            Some(TerminalRelationV1::I64Add(row)) => Some(row),
            _ => None,
        }
    }

    #[cfg(test)]
    pub(crate) fn terminal_i64_field_return_for_owner(
        &self,
        owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
    ) -> Option<&TerminalI64FieldReturnV1> {
        match self.sole_terminal_relation_for_owner(owner) {
            Some(TerminalRelationV1::I64Field(row)) => Some(row),
            _ => None,
        }
    }

    #[cfg(test)]
    pub(crate) fn terminal_integer_literal_return_for_owner(
        &self,
        owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
    ) -> Option<&TerminalIntegerLiteralReturnV1> {
        match self.sole_terminal_relation_for_owner(owner) {
            Some(TerminalRelationV1::IntegerLiteral(row)) => Some(row),
            _ => None,
        }
    }

    /// The Call terminal whose return site is this exact exit.
    pub(in crate::mir::normal_callable_semantic_package) fn call_relation_for_owner_at(
        &self,
        owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
        site: &SourceStmtSiteV1,
    ) -> Option<&crate::mir::resolved_semantics::home_new_prefix::TerminalI64CallReturnV1> {
        match self.terminal_relation_for_owner_at(owner, site) {
            Some(TerminalRelationV1::Call(call)) if call.owner() == owner => Some(call),
            _ => None,
        }
    }

    /// Test-only sole-exit shorthand — production resolves through the
    /// `*_at` accessors.
    #[cfg(test)]
    pub(in crate::mir::normal_callable_semantic_package) fn call_source_completion(
        &self,
    ) -> Option<(
        &crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1,
        &crate::mir::resolved_semantics::home_new_prefix::TerminalI64CallReturnV1,
    )> {
        let Some(Ok(completion)) = self.root_completion.as_ref() else {
            return None;
        };
        let relations = self.terminal_relation.values().collect::<Vec<_>>();
        let [TerminalRelationV1::Call(call)] = relations.as_slice() else {
            return None;
        };
        Some((completion.as_ref(), call))
    }

    /// Test-only sole-exit shorthand — `None` when `owner` keeps zero or
    /// several Call relations, never an arbitrary first pick.
    #[cfg(test)]
    pub(in crate::mir::normal_callable_semantic_package) fn call_source_completion_for_owner(
        &self,
        owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
    ) -> Option<(
        &crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1,
        &crate::mir::resolved_semantics::home_new_prefix::TerminalI64CallReturnV1,
    )> {
        let completion = self.completion_for_owner(owner)?;
        match self.call_relations_for_owner(owner).as_slice() {
            [call] => Some((completion, call)),
            _ => None,
        }
    }

    /// The caller completion plus the Call terminal seated at this exact
    /// exit site — never a sibling exit's relation.
    pub(in crate::mir::normal_callable_semantic_package) fn call_source_completion_for_owner_at(
        &self,
        owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
        site: &SourceStmtSiteV1,
    ) -> Option<(
        &crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1,
        &crate::mir::resolved_semantics::home_new_prefix::TerminalI64CallReturnV1,
    )> {
        let completion = self.completion_for_owner(owner)?;
        let call = self.call_relation_for_owner_at(owner, site)?;
        Some((completion, call))
    }

    pub(crate) fn local_call_for_owner(
        &self,
        owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
        site: &crate::mir::resolved_semantics::SourceExprSiteV1,
    ) -> Option<&crate::mir::resolved_semantics::home_new_prefix::LocalCallObservationV1> {
        self.completion_index
            .get(&owner)
            .and_then(|row| row.as_ref().ok())
            .or_else(|| {
                self.root_completion
                    .as_ref()
                    .and_then(|row| row.as_ref().ok())
                    .filter(|completion| completion.owner() == owner)
            })
            .and_then(|completion| completion.cleanup().root_flow())
            .and_then(|flow| {
                flow.local_calls()
                    .iter()
                    .find(|call| call.owner() == owner && call.site().site() == site)
            })
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.claims.borrow().is_empty()
            && self.result_claims.borrow().is_empty()
            && self.map_demands_consumed()
            && self
                .local_commits
                .borrow()
                .values()
                .all(|row| row.is_complete())
            && self.root_home_exit_is_complete()
            && self.local_call_bindings_consumed()
            && self.field_reads_complete()
            && self.birth_abi_handoffs.borrow().is_empty()
            && self.terminal_result_complete()
            && self
                .terminal_relation
                .iter()
                .filter(|(_, relation)| matches!(relation, TerminalRelationV1::IntegerLiteral(_)))
                .all(|(site, _)| {
                    self.terminal_integer_literal_value
                        .borrow()
                        .contains_key(site)
                })
            && self
                .terminal_relation_index
                .iter()
                .filter(|(owner, _)| Some(**owner) != self.root_owner())
                .flat_map(|(owner, relations)| {
                    relations
                        .iter()
                        .map(move |(site, relation)| (owner, site, relation))
                })
                .filter(|(_, _, relation)| {
                    matches!(relation, TerminalRelationV1::IntegerLiteral(_))
                })
                .all(|(owner, site, _)| {
                    self.terminal_integer_literal_values
                        .borrow()
                        .contains_key(&(*owner, site.clone()))
                })
            && self.terminal_i64_field_return_complete()
            && self.root_instance_call_is_empty()
    }

    /// Whether this exact source relation is retained in the owner index.
    /// The Root and child owners can both be indexed; membership alone
    /// does not select the physical scalar value's storage.
    pub(in crate::mir::normal_callable_semantic_package) fn terminal_relation_is_indexed(
        &self,
        owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
        site: &SourceStmtSiteV1,
    ) -> bool {
        self.terminal_relation_index
            .get(&owner)
            .is_some_and(|index| index.contains_key(site))
    }

    /// Child scalar values use owner + exit keys. The original Root keeps
    /// its site-only storage even when its SAME source table is indexed.
    pub(in crate::mir::normal_callable_semantic_package) fn terminal_scalar_uses_child_storage(
        &self,
        owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
        site: &SourceStmtSiteV1,
    ) -> bool {
        self.root_owner() != Some(owner) && self.terminal_relation_is_indexed(owner, site)
    }

    /// The sole root terminal relation — `None` when the root retains zero
    /// or several exit relations. Callers needing a specific exit must use
    /// the `*_at` accessors; this never picks an arbitrary row.
    fn sole_root_terminal_relation(&self) -> Option<&TerminalRelationV1> {
        (self.terminal_relation.len() == 1)
            .then(|| self.terminal_relation.values().next())
            .flatten()
    }

    pub(crate) fn terminal_i64_add_return(&self) -> Option<&TerminalI64AddReturnV1> {
        match self.sole_root_terminal_relation() {
            Some(TerminalRelationV1::I64Add(row)) => Some(row),
            _ => None,
        }
    }

    pub(crate) fn terminal_unit_return(&self) -> Option<&TerminalUnitReturnV1> {
        match self.sole_root_terminal_relation() {
            Some(TerminalRelationV1::Unit(row)) => Some(row),
            _ => None,
        }
    }

    pub(crate) fn terminal_integer_literal_return(
        &self,
    ) -> Option<&TerminalIntegerLiteralReturnV1> {
        match self.sole_root_terminal_relation() {
            Some(TerminalRelationV1::IntegerLiteral(row)) => Some(row),
            _ => None,
        }
    }

    pub(crate) fn terminal_i64_field_return(&self) -> Option<&TerminalI64FieldReturnV1> {
        match self.sole_root_terminal_relation() {
            Some(TerminalRelationV1::I64Field(row)) => Some(row),
            _ => None,
        }
    }

    /// The root owner's I64Add relation at this exact exit site.
    pub(crate) fn terminal_i64_add_return_at(
        &self,
        site: &SourceStmtSiteV1,
    ) -> Option<&TerminalI64AddReturnV1> {
        match self.terminal_relation.get(site) {
            Some(TerminalRelationV1::I64Add(row)) => Some(row),
            _ => None,
        }
    }

    pub(crate) fn terminal_i64_add_return_for_owner_at(
        &self,
        owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
        site: &SourceStmtSiteV1,
    ) -> Option<&TerminalI64AddReturnV1> {
        match self.terminal_relation_for_owner_at(owner, site) {
            Some(TerminalRelationV1::I64Add(row)) => Some(row),
            _ => None,
        }
    }

    /// Every Unit relation in the root map — `validate_terminal_unit_return`
    /// iterates them because a multi-exit function can bare-return from
    /// more than one site.
    pub(super) fn terminal_unit_returns(&self) -> Vec<&TerminalUnitReturnV1> {
        self.terminal_relation
            .values()
            .filter_map(|relation| match relation {
                TerminalRelationV1::Unit(row) => Some(row),
                _ => None,
            })
            .collect()
    }

    pub(crate) fn terminal_map_get_return_for_owner_at(
        &self,
        owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
        site: &SourceStmtSiteV1,
    ) -> Option<&TerminalMapGetReturnV1> {
        match self.terminal_relation_for_owner_at(owner, site) {
            Some(TerminalRelationV1::MapGet(row)) => Some(row),
            _ => None,
        }
    }

    pub(crate) fn prepare_terminal_integer_literal_return(
        &self,
        owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
        site: &SourceNodeSiteV1,
    ) -> Result<Option<i64>, String> {
        let stmt_site = SourceStmtSiteV1::from_node(site.clone());
        let Some(TerminalRelationV1::IntegerLiteral(relation)) =
            self.terminal_relation_for_owner_at(owner, &stmt_site)
        else {
            return Ok(None);
        };
        let Some(completion) = self.completion_for_owner(owner) else {
            return Err("[freeze:contract][ordinary-new/literal-completion-missing]".into());
        };
        if relation.owner() != owner
            || completion.owner() != owner
            || !completion.explicit_sites().contains(&stmt_site)
            || if self.terminal_scalar_uses_child_storage(owner, &stmt_site) {
                self.terminal_integer_literal_values
                    .borrow()
                    .contains_key(&(owner, stmt_site.clone()))
            } else {
                self.terminal_integer_literal_value
                    .borrow()
                    .contains_key(&stmt_site)
            }
        {
            return Err("[freeze:contract][ordinary-new/literal-source-drift]".into());
        }
        Ok(Some(relation.value()))
    }

    pub(crate) fn record_terminal_integer_literal_return(
        &self,
        owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
        site: &SourceNodeSiteV1,
        value: crate::mir::ValueId,
    ) -> Result<(), String> {
        let stmt_site = SourceStmtSiteV1::from_node(site.clone());
        if !matches!(
            self.terminal_relation_for_owner_at(owner, &stmt_site),
            Some(TerminalRelationV1::IntegerLiteral(_))
        ) {
            return Err("[freeze:contract][ordinary-new/literal-duplicate]".into());
        }
        if self.terminal_scalar_uses_child_storage(owner, &stmt_site) {
            if self
                .terminal_integer_literal_values
                .borrow_mut()
                .insert((owner, stmt_site), value)
                .is_some()
            {
                return Err("[freeze:contract][ordinary-new/literal-duplicate]".into());
            }
        } else if self
            .terminal_integer_literal_value
            .borrow_mut()
            .insert(stmt_site, value)
            .is_some()
        {
            return Err("[freeze:contract][ordinary-new/literal-duplicate]".into());
        }
        Ok(())
    }
}

impl OrdinaryNewClaimLedgerV1 {
    /// Source-only call returns still owe the completed-index handoff seal.
    pub(in crate::mir::normal_callable_semantic_package) fn has_pending_object_return_v1(
        &self,
        owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
    ) -> bool {
        self.terminal_relation.values()
            .chain(self.terminal_relation_index.values().flat_map(|rows| rows.values()))
            .filter(|relation| relation.owner() == owner)
            .any(|relation| {
            matches!(relation, TerminalRelationV1::Value(row)
                if matches!(row.returned(), crate::mir::resolved_semantics::home_new_prefix::TerminalReturnedSourceV1::OwnedCall(_)))
        })
    }
    pub(in crate::mir::normal_callable_semantic_package) fn validate_no_pending_object_returns_v1(
        &self,
    ) -> Result<(), String> {
        let pending = self.terminal_relation.values()
            .chain(self.terminal_relation_index.values().flat_map(|rows| rows.values()))
            .any(|relation| matches!(relation, TerminalRelationV1::Value(row)
                if matches!(row.returned(), crate::mir::resolved_semantics::home_new_prefix::TerminalReturnedSourceV1::OwnedCall(_))));
        if pending {
            Err(
                "[freeze:contract][ordinary-new/local-commit/object-return-handoff-unavailable]"
                    .to_string(),
            )
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
#[path = "ordinary_new_object_return_source_tests.rs"]
mod object_return_source_tests;

#[cfg(test)]
#[path = "ordinary_new_terminal_storage_tests.rs"]
mod storage_tests;
