//! Exact Null source loan and same-append observation in the original exit owner.
use super::*;
use crate::mir::builder::emission::constant::CompletedConstV1;
use crate::mir::resolved_semantics::{SourceNodeSiteV1, SourcePathSegmentV1};
use crate::mir::ConstValue;
use std::collections::BTreeMap;

#[derive(Debug)]
pub(crate) struct TerminalNullReturnSourceLoanV1 {
    site: OwnedExprSiteV1,
    exit: SourceStmtSiteV1,
    table: Rc<BTreeMap<SourceStmtSiteV1, TerminalRelationV1>>,
}
#[derive(Debug)]
pub(super) struct TerminalNullReturnProducerV1 {
    loan: TerminalNullReturnSourceLoanV1,
    original: (BasicBlockId, MirInstruction),
}
impl OrdinaryNewClaimLedgerV1 {
    pub(crate) fn prepare_terminal_null_literal_v1(
        &self,
        site: OwnedExprSiteV1,
    ) -> Result<Option<TerminalNullReturnSourceLoanV1>, String> {
        let segments = site.site().node().segments();
        if segments.last() != Some(&SourcePathSegmentV1::Value) {
            return Ok(None);
        }
        let exit = SourceStmtSiteV1::from_node(SourceNodeSiteV1::from_segments(
            segments[..segments.len() - 1].to_vec(),
        ));
        let Some(table) = self.terminal_relation_index.get(&site.owner()) else {
            return Ok(None);
        };
        let Some(TerminalRelationV1::Value(value)) = table.get(&exit) else {
            return Ok(None);
        };
        if !matches!(value.returned(), TerminalReturnedSourceV1::NullLiteral) {
            return Ok(None);
        }
        if value.owner() != site.owner()
            || value.return_site() != &exit
            || value.value_site() != site.site()
        {
            return Err(freeze("terminal-null/source-identity"));
        }
        let completion = self
            .completion_index
            .get(&site.owner())
            .ok_or_else(|| freeze("terminal-null/completion-missing"))?
            .as_ref()
            .map_err(|issue| format!("{}: {issue:?}", freeze("terminal-null/completion")))?;
        if completion.owner() != site.owner() || !completion.explicit_sites().contains(&exit) {
            return Err(freeze("terminal-null/completion-identity"));
        }
        match self.root_exits.borrow().get(&(site.owner(), exit.clone())) {
            Some(RootHomeExitProgress::Prepared(_)) => {}
            None | Some(RootHomeExitProgress::Unprepared | RootHomeExitProgress::Unavailable) => {
                return Ok(None)
            }
            _ => return Err(freeze("terminal-null/exit-not-prepared")),
        }
        Ok(Some(TerminalNullReturnSourceLoanV1 {
            site,
            exit,
            table: Rc::clone(table),
        }))
    }

    pub(crate) fn complete_terminal_null_literal_v1(
        &self,
        loan: TerminalNullReturnSourceLoanV1,
        site: OwnedExprSiteV1,
        completed: &CompletedConstV1,
    ) -> Result<(), String> {
        if loan.site != site
            || !self
                .terminal_relation_index
                .get(&site.owner())
                .is_some_and(|table| Rc::ptr_eq(table, &loan.table))
        {
            return Err(freeze("terminal-null/loan-identity"));
        }
        if !matches!(&completed.original().1, MirInstruction::Const { dst, value: ConstValue::Null } if *dst == completed.value())
        {
            return Err(freeze("terminal-null/append-kind"));
        }
        let mut exits = self.root_exits.borrow_mut();
        let Some(RootHomeExitProgress::Prepared(order)) =
            exits.get_mut(&(site.owner(), loan.exit.clone()))
        else {
            return Err(freeze("terminal-null/exit-not-prepared"));
        };
        if order.null_return.is_some() {
            return Err(freeze("terminal-null/duplicate-append"));
        }
        order.null_return = Some(TerminalNullReturnProducerV1 {
            loan,
            original: completed.original().clone(),
        });
        Ok(())
    }
}
impl RootHomeCleanupOrderV1 {
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn null_return_binding(
        &self,
    ) -> Option<&(BasicBlockId, MirInstruction)> {
        self.null_return.as_ref().map(|producer| &producer.original)
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn validate_null_return(
        &self,
        entry: &RootHomeExitEntry,
        bindings: &[(BasicBlockId, MirInstruction)],
        producer_projection: Option<&super::super::physical_boundary::FinishedBindings>,
        binding_projection: Option<&super::super::physical_boundary::FinishedBindings>,
    ) -> Result<(), String> {
        let Some(producer) = &self.null_return else {
            return Ok(());
        };
        if !matches!(entry, RootHomeExitEntry::Plain { .. }) {
            return Err(freeze("terminal-null/entry-kind"));
        }
        let original = match producer_projection {
            Some(projection) => projection
                .binding(producer.original.0, &producer.original.1)?
                .ok_or_else(|| freeze("terminal-null/producer-removed"))?,
            None => producer.original.clone(),
        };
        let MirInstruction::Const {
            dst,
            value: ConstValue::Null,
        } = original.1
        else {
            return Err(freeze("terminal-null/producer-kind"));
        };
        let bindings = match binding_projection {
            Some(projection) => projection.bindings(bindings)?,
            None => bindings.to_vec(),
        };
        let mut returns = bindings
            .iter()
            .filter_map(|(_, instruction)| match instruction {
                MirInstruction::Return { value } => Some(value),
                _ => None,
            });
        if returns.next() != Some(&Some(dst)) || returns.next().is_some() {
            return Err(freeze("terminal-null/return-value"));
        }
        // The retained source loan is unchanged across the affine exit transition.
        let Some(TerminalRelationV1::Value(value)) = producer.loan.table.get(&producer.loan.exit)
        else {
            return Err(freeze("terminal-null/retained-source"));
        };
        if value.owner() != producer.loan.site.owner()
            || value.value_site() != producer.loan.site.site()
            || !matches!(value.returned(), TerminalReturnedSourceV1::NullLiteral)
        {
            return Err(freeze("terminal-null/retained-source"));
        }
        Ok(())
    }
}

#[cfg(test)]
#[path = "root_home_null_return_tests.rs"]
mod tests;
