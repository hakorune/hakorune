//! Access and one-shot consumption for source-issued terminal relations.
use super::*;
use crate::mir::resolved_semantics::SourceNodeSiteV1;

impl OrdinaryNewClaimLedgerV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn root_owner(
        &self,
    ) -> Option<crate::mir::resolved_semantics::FunctionOwnerIdV1> {
        self.root_completion
            .as_ref()
            .and_then(|row| row.as_ref().ok())
            .map(|completion| completion.owner())
    }

    pub(super) fn terminal_relation_for_owner(
        &self,
        owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
    ) -> Option<&TerminalRelationV1> {
        self.terminal_relation_index
            .get(&owner)
            .map(std::rc::Rc::as_ref)
            .or_else(|| {
                self.terminal_relation
                    .as_ref()
                    .filter(|relation| relation.owner() == owner)
            })
    }

    pub(in crate::mir::normal_callable_semantic_package) fn call_source_completion(&self) -> Option<(
        &crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1,
        &crate::mir::resolved_semantics::home_new_prefix::TerminalI64CallReturnV1,
    )> {
        match (self.root_completion.as_ref(), self.terminal_relation.as_ref()) {
            (Some(Ok(completion)), Some(TerminalRelationV1::Call(call))) => Some((completion, call)),
            _ => None,
        }
    }

    pub(in crate::mir::normal_callable_semantic_package) fn call_source_completion_for_owner(
        &self,
        owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
    ) -> Option<(
        &crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1,
        &crate::mir::resolved_semantics::home_new_prefix::TerminalI64CallReturnV1,
    )> {
        self.call_source_completion().filter(|(completion, terminal)| {
            completion.owner() == owner && terminal.owner() == owner
        })
    }

    pub(crate) fn local_i64_call_for_owner(
        &self,
        owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
        site: &crate::mir::resolved_semantics::SourceExprSiteV1,
    ) -> Option<&crate::mir::resolved_semantics::home_new_prefix::LocalI64CallObservationV1>
    {
        self.root_completion
            .as_ref()
            .and_then(|row| row.as_ref().ok())
            .and_then(|completion| completion.cleanup().root_flow())
            .and_then(|flow| {
                flow.local_calls()
                    .iter()
                    .find(|call| call.owner() == owner && call.site().site() == site)
            })
    }

    pub(in crate::mir::normal_callable_semantic_package) fn local_i64_call_count_for_owner(
        &self,
        owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
    ) -> Option<usize> {
        self.completion_for_owner(owner)
            .and_then(|completion| completion.cleanup().root_flow())
            .map(|flow| flow.local_calls().len())
    }

    pub(in crate::mir::normal_callable_semantic_package) fn map_install_owners(
        &self,
    ) -> Result<Box<[crate::mir::resolved_semantics::FunctionOwnerIdV1]>, ()> {
        if !self.requires_map_lifecycle_consumer() {
            return Ok(Box::new([]));
        }
        if self.app_main_identity.is_none() {
            return Err(());
        }
        let mut owners = BTreeMap::new();
        for completion in self
            .completion_index
            .values()
            .filter_map(|row| row.as_ref().ok())
            .chain(self.root_completion.iter().filter_map(|row| row.as_ref().ok()))
        {
            if completion
                .cleanup()
                .root_flow()
                .is_some_and(|flow| !flow.maps().is_empty())
            {
                owners.insert(completion.owner(), ());
            }
        }
        let root_owner = self
            .root_completion
            .as_ref()
            .and_then(|row| row.as_ref().ok())
            .map(|completion| completion.owner());
        for owner in owners.keys().copied() {
            let completion = self.completion_for_owner(owner).ok_or(())?;
            let flow = completion.cleanup().root_flow().ok_or(())?;
            let terminal = self.terminal_relation_for_owner(owner).ok_or(())?;
            let root_call = root_owner == Some(owner) && self.call_source_completion_for_owner(owner).is_some();
            if flow.maps().iter().any(|m| {
                m.complete().is_none_or(|map| {
                    map.entries().iter().any(|entry| {
                        entry.transfer_home().is_none()
                            && entry.value_source().and_then(|v| v.scalar_kind()).is_none()
                    })
                })
            }) || !matches!(completion.cleanup().terminal_homes(), Some(Ok(_)))
                || (!root_call
                    && !matches!(
                        terminal,
                        TerminalRelationV1::IntegerLiteral(_)
                            | TerminalRelationV1::I64Add(_)
                            | TerminalRelationV1::I64Field(_)
                    ))
                || self
                    .claims
                    .borrow()
                    .values()
                    .filter(|c| c.site.owner() == owner)
                    .any(|c| {
                        c.construction.is_err()
                            || c.destruction != ObjectDestructionDispositionV1::PlainI64NoHook
                            || c.home_prefix.is_err()
                            || c.argument_rows.is_err()
                    })
            {
                return Err(());
            }
        }
        Ok(owners.into_keys().collect())
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.claims.borrow().is_empty()
            && self.map_demands_consumed()
            && self
                .local_commits
                .borrow()
                .values()
                .all(|row| row.is_complete())
            && self.root_home_exit_is_complete()
            && self.field_reads_complete()
            && self.birth_abi_handoffs.borrow().is_empty()
            && self.terminal_result_complete()
            && (self.terminal_integer_literal_return().is_none()
                || self.terminal_integer_literal_value.borrow().is_some())
            && self
                .terminal_relation_index
                .iter()
                .filter(|(_, relation)| {
                    matches!(relation.as_ref(), TerminalRelationV1::IntegerLiteral(_))
                })
                .all(|(owner, _)| self.terminal_integer_literal_values.borrow().contains_key(owner))
            && self.terminal_i64_field_return_complete()
    }

    pub(crate) fn terminal_i64_add_return(&self) -> Option<&TerminalI64AddReturnV1> {
        match self.terminal_relation.as_ref() {
            Some(TerminalRelationV1::I64Add(row)) => Some(row),
            _ => None,
        }
    }

    pub(crate) fn terminal_unit_return(&self) -> Option<&TerminalUnitReturnV1> {
        match self.terminal_relation.as_ref() {
            Some(TerminalRelationV1::Unit(row)) => Some(row),
            _ => None,
        }
    }

    pub(crate) fn terminal_integer_literal_return(
        &self,
    ) -> Option<&TerminalIntegerLiteralReturnV1> {
        match self.terminal_relation.as_ref() {
            Some(TerminalRelationV1::IntegerLiteral(row)) => Some(row),
            _ => None,
        }
    }

    pub(crate) fn terminal_integer_literal_return_for_owner(
        &self,
        owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
    ) -> Option<&TerminalIntegerLiteralReturnV1> {
        match self.terminal_relation_for_owner(owner) {
            Some(TerminalRelationV1::IntegerLiteral(row)) => Some(row),
            _ => None,
        }
    }

    pub(crate) fn terminal_i64_field_return(&self) -> Option<&TerminalI64FieldReturnV1> {
        match self.terminal_relation.as_ref() {
            Some(TerminalRelationV1::I64Field(row)) => Some(row),
            _ => None,
        }
    }

    pub(crate) fn terminal_i64_field_return_for_owner(
        &self,
        owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
    ) -> Option<&TerminalI64FieldReturnV1> {
        match self.terminal_relation_for_owner(owner) {
            Some(TerminalRelationV1::I64Field(row)) => Some(row),
            _ => None,
        }
    }

    pub(crate) fn prepare_terminal_integer_literal_return(
        &self,
        owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
        site: &SourceNodeSiteV1,
    ) -> Result<Option<i64>, String> {
        let Some(relation) = self.terminal_integer_literal_return_for_owner(owner) else {
            let foreign_indexed = self.terminal_relation_index.values().any(|candidate| {
                matches!(candidate.as_ref(), TerminalRelationV1::IntegerLiteral(row)
                    if row.return_site().node() == site)
            });
            let foreign_root = self.terminal_relation.as_ref().is_some_and(|candidate| {
                matches!(candidate, TerminalRelationV1::IntegerLiteral(row)
                    if row.return_site().node() == site)
            });
            if foreign_indexed || foreign_root {
                return Err("[freeze:contract][ordinary-new/literal-owner-drift]".into());
            }
            return Ok(None);
        };
        let Some(completion) = self.completion_for_owner(owner) else {
            return Err("[freeze:contract][ordinary-new/literal-completion-missing]".into());
        };
        if relation.owner() != owner
            || completion.owner() != owner
            || completion.explicit_site() != Some(relation.return_site())
            || relation.return_site().node() != site
            || if self
                .terminal_relation
                .as_ref()
                .is_some_and(|terminal| terminal.owner() == owner)
            {
                self.terminal_integer_literal_value.borrow().is_some()
            } else {
                self.terminal_integer_literal_values.borrow().contains_key(&owner)
            }
        {
            return Err("[freeze:contract][ordinary-new/literal-source-drift]".into());
        }
        Ok(Some(relation.value()))
    }

    pub(crate) fn record_terminal_integer_literal_return(
        &self,
        owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
        value: crate::mir::ValueId,
    ) -> Result<(), String> {
        if self.terminal_integer_literal_return_for_owner(owner).is_none() {
            return Err("[freeze:contract][ordinary-new/literal-duplicate]".into());
        }
        if self
            .terminal_relation
            .as_ref()
            .is_some_and(|terminal| terminal.owner() == owner)
        {
            if self
                .terminal_integer_literal_value
                .replace(Some(value))
                .is_some()
            {
                return Err("[freeze:contract][ordinary-new/literal-duplicate]".into());
            }
        } else if self
            .terminal_integer_literal_values
            .borrow_mut()
            .insert(owner, value)
            .is_some()
        {
            return Err("[freeze:contract][ordinary-new/literal-duplicate]".into());
        }
        Ok(())
    }
}
