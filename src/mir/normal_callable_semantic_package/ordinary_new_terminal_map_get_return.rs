//! Physical consumption of one source-issued readable-Map terminal.
//!
//! The source relation names the exact `return <map>.get("<literal>")`
//! boundary. This module only resolves the receiver binding into its
//! already-physical value; it never revisits raw syntax, issues a key, or
//! decides ownership — a borrowed formal stays caller-owned and an owned
//! map local keeps its own End obligation.
use super::*;
use crate::mir::resolved_semantics::{FunctionOwnerIdV1, SourceNodeSiteV1};
use crate::mir::ValueId;

/// One resolved terminal map-get: the call site is retained source
/// evidence, `map` is the receiver binding's existing physical value, and
/// `utf8` is the sealed literal key carried inline like a prepared key.
pub(crate) struct PreparedTerminalMapGetReturnV1 {
    pub(crate) site: OwnedExprSiteV1,
    pub(crate) map: ValueId,
    pub(crate) utf8: String,
}

impl OrdinaryNewClaimLedgerV1 {
    pub(crate) fn prepare_terminal_map_get_return(
        &self,
        owner: FunctionOwnerIdV1,
        return_site: &SourceNodeSiteV1,
        mut resolve_binding: impl FnMut(BindingRefV1, &SourceNodeSiteV1) -> Result<ValueId, String>,
    ) -> Result<Option<PreparedTerminalMapGetReturnV1>, String> {
        let Some(relation) = self.terminal_map_get_return_for_owner(owner) else {
            return Ok(None);
        };
        let Some(completion) = self.completion_for_owner(owner) else {
            return Err(fault("completion-missing"));
        };
        if relation.owner() != owner
            || completion.owner() != owner
            || completion.explicit_site() != Some(relation.return_site())
            || relation.return_site().node() != return_site
        {
            return Err(fault("source-drift"));
        }
        let map = resolve_binding(relation.receiver(), relation.receiver_site().node())?;
        Ok(Some(PreparedTerminalMapGetReturnV1 {
            site: relation.call_site().clone(),
            map,
            utf8: relation.key().into(),
        }))
    }
}

fn fault(reason: &str) -> String {
    format!("[freeze:contract][ordinary-terminal-map-get/{reason}]")
}
