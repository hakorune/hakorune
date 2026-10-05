//! Physical consumption of one source-issued direct selected i64 field return.
//!
//! The source relation names the exact staged field read. This module only
//! reserves, emits, and verifies that one read; it never revisits raw syntax.
//! The emitted value is retained per exact exit site so one `return x.f`
//! can never satisfy a sibling exit.
use super::*;
use crate::mir::resolved_semantics::{FunctionOwnerIdV1, SourceNodeSiteV1};
use crate::mir::{MirFunction, MirInstruction, ValueId};
use hakorune_mir_defs::CanonicalFieldRefV1;

pub(crate) struct PreparedTerminalI64FieldReturnV1 {
    pub(crate) return_site: SourceStmtSiteV1,
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
        let stmt_site = SourceStmtSiteV1::from_node(return_site.clone());
        let Some(relation) = self
            .terminal_relation_for_owner_at(owner, &stmt_site)
            .and_then(|relation| match relation {
                TerminalRelationV1::I64Field(row) => Some(row),
                _ => None,
            })
        else {
            return Ok(None);
        };
        let Some(completion) = self.completion_for_owner(owner) else {
            return Err(fault("completion-missing"));
        };
        if relation.owner() != owner
            || completion.owner() != owner
            || !completion.explicit_sites().contains(&stmt_site)
            || self
                .terminal_i64_field_value_for_owner_at(owner, &stmt_site)
                .is_some()
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
        Ok(Some(PreparedTerminalI64FieldReturnV1 {
            return_site: stmt_site,
            site,
            base,
            field,
        }))
    }

    pub(crate) fn record_terminal_i64_field_return(
        &self,
        owner: FunctionOwnerIdV1,
        site: &SourceStmtSiteV1,
        value: ValueId,
    ) -> Result<(), String> {
        if !matches!(
            self.terminal_relation_for_owner_at(owner, site),
            Some(TerminalRelationV1::I64Field(_))
        ) {
            return Err(fault("duplicate-emission"));
        }
        if self.terminal_relation_is_indexed(owner, site) {
            if self
                .terminal_i64_field_values
                .borrow_mut()
                .insert((owner, site.clone()), value)
                .is_some()
            {
                return Err(fault("duplicate-emission"));
            }
        } else if self
            .terminal_i64_field_value
            .borrow_mut()
            .insert(site.clone(), value)
            .is_some()
        {
            return Err(fault("duplicate-emission"));
        }
        Ok(())
    }

    /// Every retained I64Field relation — root map and child index alike —
    /// must carry its recorded emitted value at its own exit site.
    pub(super) fn terminal_i64_field_return_complete(&self) -> bool {
        let root_ready = self
            .terminal_relation
            .iter()
            .filter(|(_, relation)| matches!(relation, TerminalRelationV1::I64Field(_)))
            .all(|(site, _)| self.terminal_i64_field_value.borrow().contains_key(site));
        let indexed_ready = self
            .terminal_relation_index
            .iter()
            .flat_map(|(owner, relations)| {
                relations
                    .iter()
                    .map(move |(site, relation)| (owner, site, relation))
            })
            .filter(|(_, _, relation)| matches!(relation, TerminalRelationV1::I64Field(_)))
            .all(|(owner, site, _)| {
                self.terminal_i64_field_values
                    .borrow()
                    .contains_key(&(*owner, site.clone()))
            });
        root_ready && indexed_ready
    }

    pub(super) fn validate_terminal_i64_field_return(
        &self,
        owner: FunctionOwnerIdV1,
        function: &MirFunction,
        mut check_read_binding: impl FnMut(
            crate::mir::BasicBlockId,
            &MirInstruction,
        ) -> Result<bool, String>,
    ) -> Result<(), String> {
        let reads = self.field_reads.borrow();
        for relation in self.terminal_relations_for_owner(owner) {
            let TerminalRelationV1::I64Field(relation) = relation else {
                continue;
            };
            let exit_site = relation.return_site().clone();
            let Some(value) = self.terminal_i64_field_value_for_owner_at(owner, &exit_site) else {
                return Err(fault("unconsumed"));
            };
            let row = reads
                .get(relation.field_read_site())
                .ok_or_else(|| fault("field-read-missing"))?;
            let field_reads::Progress::Emitted(
                block,
                instruction @ MirInstruction::ObjectFieldGet { dst, .. },
            ) = &row.progress
            else {
                return Err(fault("field-read-not-emitted"));
            };
            if *dst != value {
                return Err(fault("result-drift"));
            }
            let exact_read = check_read_binding(*block, instruction)?;
            let returned = function.blocks.values().any(|block| {
                block.all_instructions().any(|instruction| {
                    matches!(instruction, MirInstruction::Return { value: Some(actual) } if *actual == value)
                })
            });
            if !(exact_read && returned) {
                return Err(fault("physical-drift"));
            }
        }
        Ok(())
    }

    fn terminal_i64_field_value_for_owner_at(
        &self,
        owner: FunctionOwnerIdV1,
        site: &SourceStmtSiteV1,
    ) -> Option<ValueId> {
        if self.terminal_relation_is_indexed(owner, site) {
            self.terminal_i64_field_values
                .borrow()
                .get(&(owner, site.clone()))
                .copied()
        } else {
            self.terminal_i64_field_value.borrow().get(site).copied()
        }
    }
}

fn fault(reason: &str) -> String {
    format!("[freeze:contract][ordinary-terminal-field-return/{reason}]")
}
