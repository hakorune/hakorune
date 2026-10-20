//! Physical consumption of one source-issued readable-Map terminal.
//!
//! The source relation names the exact `return <map>.get("<literal>")`
//! boundary. This module only resolves the receiver binding into its
//! already-physical value; it never revisits raw syntax, issues a key, or
//! decides ownership — a borrowed formal stays caller-owned and an owned
//! map local keeps its own End obligation.
use super::*;
use crate::mir::resolved_semantics::{FunctionOwnerIdV1, SourceNodeSiteV1, SourceStmtSiteV1};
use crate::mir::ValueId;

/// One resolved terminal map-get: the call site is retained source
/// evidence, `map` is the receiver binding's existing physical value, and
/// `utf8` is the sealed literal key carried inline like a prepared key.
pub(crate) struct PreparedTerminalMapGetReturnV1 {
    pub(crate) return_site: SourceStmtSiteV1,
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
        let stmt_site = SourceStmtSiteV1::from_node(return_site.clone());
        let Some(relation) = self.terminal_map_get_return_for_owner_at(owner, &stmt_site) else {
            return Ok(None);
        };
        let Some(completion) = self.completion_for_owner(owner) else {
            return Err(fault("completion-missing"));
        };
        if relation.owner() != owner
            || completion.owner() != owner
            || !completion.explicit_sites().contains(&stmt_site)
        {
            return Err(fault("source-drift"));
        }
        let map = resolve_binding(relation.receiver(), relation.receiver_site().node())?;
        Ok(Some(PreparedTerminalMapGetReturnV1 {
            return_site: stmt_site,
            site: relation.call_site().clone(),
            map,
            utf8: relation.key().into(),
        }))
    }
}

fn fault(reason: &str) -> String {
    format!("[freeze:contract][ordinary-terminal-map-get/{reason}]")
}

/// The original stored-object Return, with its independent get receiver use.
/// This source loan owns neither a payload Home nor a physical result ABI.
#[derive(Debug, Clone)]
pub(crate) struct StoredMapGetReturnSourceLoanV1 {
    terminal: crate::mir::resolved_semantics::home_new_prefix::TerminalValueReturnV1,
}
impl StoredMapGetReturnSourceLoanV1 {
    pub(crate) fn terminal(&self) -> &crate::mir::resolved_semantics::home_new_prefix::TerminalValueReturnV1 { &self.terminal }
    pub(crate) fn parts(&self) -> (&std::rc::Rc<super::MapStoredResultSourceV1>,
        &std::rc::Rc<crate::mir::resolved_semantics::home_new_prefix::CheckedExpressionEvaluationV1>,
        &crate::mir::resolved_semantics::SourceExprSiteV1) {
        let crate::mir::resolved_semantics::home_new_prefix::TerminalReturnedSourceV1::StoredMapGet { source, key, receiver_read } = self.terminal.returned()
            else { unreachable!("source-issued stored get loan") };
        (source, key, receiver_read)
    }
    pub(crate) fn require_receiver(&self, context: super::CallValidationContextV1<'_>,
        function: &crate::mir::MirFunction, read: &crate::mir::builder::ExactLexicalReadV1,
    ) -> Result<ValueId, String> {
        let (source, _, site) = self.parts();
        source.receiver_v1().require_receiver_at_v1(context, function, read, site)
    }
}
impl OrdinaryNewClaimLedgerV1 {
    pub(crate) fn stored_map_get_return_source_v1(&self, owner: FunctionOwnerIdV1,
        exit: &SourceStmtSiteV1,
    ) -> Result<Option<StoredMapGetReturnSourceLoanV1>, String> {
        use crate::mir::resolved_semantics::home_new_prefix::{TerminalRelationV1, TerminalReturnedSourceV1};
        let Some(TerminalRelationV1::Value(terminal)) = self.terminal_relation_for_owner_at(owner, exit)
            else { return Ok(None) };
        let TerminalReturnedSourceV1::StoredMapGet { source, key, receiver_read: _ } = terminal.returned()
            else { return Ok(None) };
        let site = OwnedExprSiteV1::new(owner, terminal.value_site().clone());
        let retained = self.callable_result_classes.map_stored_source_v1(&site).map_err(fault)?
            .ok_or_else(|| fault("source-drift"))?;
        let get = source.present_get_v1(&site).ok_or_else(|| fault("source-drift"))?;
        let completion = self.completion_source_for_owner_v1(owner).ok_or_else(|| fault("completion-missing"))?;
        if terminal.owner() != owner || terminal.return_site() != exit || !completion.explicit_sites().contains(exit)
            || !std::rc::Rc::ptr_eq(&retained, source)
            || key.source().owner() != owner || key.source().nodes().len() != 1
            || get.arguments().len() != 1 || key.source().nodes()[0].site() != get.arguments()[0].site()
            || !matches!(key.source().root(), crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1::CallArgument { call, ordinal: 0, .. }
                if std::sync::Arc::ptr_eq(call, get))
            || !key.reference_coverage_is_exact() || !key.calls().is_empty() || !key.cores().is_empty()
            || !key.operations().is_empty() || !key.owned_result_nodes().is_empty()
        { return Err(fault("source-drift")); }
        source.receiver_v1().require_entry_source_v1(self)?;
        Ok(Some(StoredMapGetReturnSourceLoanV1 { terminal: terminal.clone() }))
    }
}
