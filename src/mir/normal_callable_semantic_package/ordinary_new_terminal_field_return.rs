//! Physical consumption of one source-issued direct selected i64 field return.
//!
//! The source relation names the exact staged field read. This module only
//! reserves, emits, and verifies that one read; it never revisits raw syntax.
use super::*;
use crate::mir::resolved_semantics::{FunctionOwnerIdV1, SourceNodeSiteV1};
use crate::mir::{MirFunction, MirInstruction, ValueId};
use hakorune_mir_defs::CanonicalFieldRefV1;

pub(crate) struct PreparedTerminalI64FieldReturnV1 {
    pub(crate) site: OwnedExprSiteV1,
    pub(crate) base: ValueId,
    pub(crate) field: CanonicalFieldRefV1,
}

impl OrdinaryNewClaimLedgerV1 {
    pub(crate) fn prepare_terminal_i64_field_return(
        &self,
        owner: FunctionOwnerIdV1,
        return_site: &SourceNodeSiteV1,
        mut resolve_binding: impl FnMut(BindingRefV1, &SourceNodeSiteV1) -> Result<ValueId, String>,
    ) -> Result<Option<PreparedTerminalI64FieldReturnV1>, String> {
        let Some(relation) = self.terminal_i64_field_return.as_ref() else {
            return Ok(None);
        };
        let Some(Ok(completion)) = self.root_completion.as_ref() else {
            return Err(fault("completion-missing"));
        };
        if self.terminal_result.is_some()
            || self.terminal_unit_return.is_some()
            || self.terminal_integer_literal.is_some()
            || relation.owner() != owner
            || completion.owner() != owner
            || completion.explicit_site() != Some(relation.return_site())
            || relation.return_site().node() != return_site
            || self.terminal_i64_field_value.borrow().is_some()
        {
            return Err(fault("source-drift"));
        }
        let site = relation.field_read_site().clone();
        if relation.value_site().node() != site.site().node() {
            return Err(fault("value-site-drift"));
        }
        let receiver_site = self
            .field_reads
            .borrow()
            .get(&site)
            .map(|row| row.receiver_site.node().clone())
            .ok_or_else(|| fault("field-read-missing"))?;
        let Some((base, field)) = self
            .take_terminal_field_read(&site, |binding| resolve_binding(binding, &receiver_site))?
        else {
            return Err(fault("field-read-missing"));
        };
        Ok(Some(PreparedTerminalI64FieldReturnV1 { site, base, field }))
    }

    pub(crate) fn record_terminal_i64_field_return(&self, value: ValueId) -> Result<(), String> {
        if self.terminal_i64_field_return.is_none()
            || self.terminal_i64_field_value.replace(Some(value)).is_some()
        {
            return Err(fault("duplicate-emission"));
        }
        Ok(())
    }

    pub(super) fn terminal_i64_field_return_complete(&self) -> bool {
        self.terminal_i64_field_return.is_none() || self.terminal_i64_field_value.borrow().is_some()
    }

    pub(super) fn validate_terminal_i64_field_return(
        &self,
        owner: FunctionOwnerIdV1,
        function: &MirFunction,
    ) -> Result<(), String> {
        let Some(relation) = self.terminal_i64_field_return.as_ref() else {
            return Ok(());
        };
        if relation.owner() != owner {
            return Err(fault("foreign-owner"));
        }
        let Some(value) = *self.terminal_i64_field_value.borrow() else {
            return Err(fault("unconsumed"));
        };
        let reads = self.field_reads.borrow();
        let row = reads
            .get(relation.field_read_site())
            .ok_or_else(|| fault("field-read-missing"))?;
        let field_reads::Progress::Emitted(
            block,
            MirInstruction::ObjectFieldGet { dst, base, field },
        ) = &row.progress
        else {
            return Err(fault("field-read-not-emitted"));
        };
        if *dst != value {
            return Err(fault("result-drift"));
        }
        let exact_read = function.blocks.get(block).is_some_and(|block| {
            block.all_instructions().any(|instruction| {
                matches!(instruction,
                MirInstruction::ObjectFieldGet { dst, base: actual_base, field: actual_field }
                    if *dst == value && actual_base == base && actual_field == field)
            })
        });
        let returned = function.blocks.values().any(|block| {
            block.all_instructions().any(|instruction| {
                matches!(instruction, MirInstruction::Return { value: Some(actual) } if *actual == value)
            })
        });
        (exact_read && returned)
            .then_some(())
            .ok_or_else(|| fault("physical-drift"))
    }
}

fn fault(reason: &str) -> String {
    format!("[freeze:contract][ordinary-terminal-field-return/{reason}]")
}
