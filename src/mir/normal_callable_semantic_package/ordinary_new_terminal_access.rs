//! Access and one-shot consumption for source-issued terminal relations.
use super::*;
use crate::mir::resolved_semantics::SourceNodeSiteV1;

impl OrdinaryNewClaimLedgerV1 {
    fn terminal_relation_for_owner(
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
