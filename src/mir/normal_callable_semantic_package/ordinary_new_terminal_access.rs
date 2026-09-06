//! Access and one-shot consumption for source-issued terminal relations.
use super::*;
use crate::mir::resolved_semantics::SourceNodeSiteV1;

impl OrdinaryNewClaimLedgerV1 {
    pub(crate) fn is_empty(&self) -> bool {
        self.claims.borrow().is_empty()
            && self
                .local_commits
                .borrow()
                .values()
                .all(|row| row.is_complete())
            && self.root_home_exit_is_complete()
            && self.field_reads_complete()
            && self.birth_abi_handoffs.borrow().is_empty()
            && self.terminal_result_complete()
            && (self.terminal_integer_literal.is_none()
                || self.terminal_integer_literal_value.borrow().is_some())
            && self.terminal_i64_field_return_complete()
    }

    pub(crate) fn terminal_i64_add_return(&self) -> Option<&TerminalI64AddReturnV1> {
        self.terminal_result.as_ref()
    }

    pub(crate) fn terminal_unit_return(&self) -> Option<&TerminalUnitReturnV1> {
        self.terminal_unit_return.as_ref()
    }

    pub(crate) fn terminal_integer_literal_return(
        &self,
    ) -> Option<&TerminalIntegerLiteralReturnV1> {
        self.terminal_integer_literal.as_ref()
    }

    pub(crate) fn terminal_i64_field_return(&self) -> Option<&TerminalI64FieldReturnV1> {
        self.terminal_i64_field_return.as_ref()
    }

    pub(crate) fn prepare_terminal_integer_literal_return(
        &self,
        owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
        site: &SourceNodeSiteV1,
    ) -> Result<Option<i64>, String> {
        let Some(relation) = self.terminal_integer_literal.as_ref() else {
            return Ok(None);
        };
        let Some(Ok(completion)) = self.root_completion.as_ref() else {
            return Err("[freeze:contract][ordinary-new/literal-completion-missing]".into());
        };
        if self.terminal_result.is_some()
            || self.terminal_unit_return.is_some()
            || self.terminal_i64_field_return.is_some()
            || relation.owner() != owner
            || completion.owner() != owner
            || completion.explicit_site() != Some(relation.return_site())
            || relation.return_site().node() != site
            || self.terminal_integer_literal_value.borrow().is_some()
        {
            return Err("[freeze:contract][ordinary-new/literal-source-drift]".into());
        }
        Ok(Some(relation.value()))
    }

    pub(crate) fn record_terminal_integer_literal_return(
        &self,
        value: crate::mir::ValueId,
    ) -> Result<(), String> {
        if self.terminal_integer_literal.is_none()
            || self
                .terminal_integer_literal_value
                .replace(Some(value))
                .is_some()
        {
            return Err("[freeze:contract][ordinary-new/literal-duplicate]".into());
        }
        Ok(())
    }
}
