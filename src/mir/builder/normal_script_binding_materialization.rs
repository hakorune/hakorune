//! Script BindingRef materialization hooks for the selected raw port.
//!
//! These hooks consume already-sealed semantic receipts. They never resolve a
//! name or inspect an AST shape to choose an authority.

use crate::ast::ASTNode;
use crate::mir::builder::stmts::async_stmt::build_nowait_statement_with_port_v1;
use crate::mir::builder::stmts::{drive_local_statement_v1, RawLegacyLocalInputV1};
use crate::mir::{MirBuilder, ValueId};

use super::super::raw_invocation_source_transport::RawSourceTransportPortV1;
use super::super::recursive_child_lowering::RawInvocationChildPortV1;

impl RawInvocationChildPortV1<'_, '_> {
    pub(in crate::mir::builder) fn lower_script_local_v1(
        &mut self,
        builder: &mut MirBuilder,
        input: ASTNode,
    ) -> Result<ValueId, String> {
        let ledger = self
            .semantic_ledger
            .clone()
            .expect("script local lowering requires semantic ledger");
        let site = self
            .current_source_context_v1()
            .and_then(|context| context.site().cloned())
            .ok_or_else(|| "[freeze:contract][script-lexical/local-site]".to_owned())?;
        let binding = ledger
            .borrow()
            .local_binding(&site)
            .ok_or_else(|| "[freeze:contract][script-lexical/local-binding]".to_owned())?;
        let value = drive_local_statement_v1(builder, self, RawLegacyLocalInputV1::new(input))?;
        ledger.borrow_mut().record(binding, value)?;
        Ok(value)
    }

    pub(in crate::mir::builder) fn lower_script_nowait_v1(
        &mut self,
        builder: &mut MirBuilder,
        input: ASTNode,
    ) -> Result<ValueId, String> {
        let ledger = self
            .semantic_ledger
            .clone()
            .expect("script nowait lowering requires semantic ledger");
        let site = self
            .current_source_context_v1()
            .and_then(|context| context.site().cloned())
            .ok_or_else(|| "[freeze:contract][script-lexical/nowait-site]".to_owned())?;
        let binding = ledger
            .borrow()
            .nowait_binding(&site)
            .ok_or_else(|| "[freeze:contract][script-lexical/nowait-binding]".to_owned())?;
        let ASTNode::Nowait {
            variable,
            expression,
            ..
        } = input
        else {
            unreachable!("script nowait lowering only receives Nowait")
        };
        let value = build_nowait_statement_with_port_v1(builder, self, variable, *expression)?;
        ledger.borrow_mut().record(binding, value)?;
        Ok(value)
    }

    pub(in crate::mir::builder) fn lower_script_outbox_v1(
        &mut self,
        builder: &mut MirBuilder,
        input: ASTNode,
    ) -> Result<ValueId, String> {
        let ledger = self
            .semantic_ledger
            .clone()
            .expect("script Outbox lowering requires semantic ledger");
        let site = self
            .current_source_context_v1()
            .and_then(|context| context.site().cloned())
            .ok_or_else(|| "[freeze:contract][script-lexical/outbox-site]".to_owned())?;
        let ASTNode::Outbox { variables, .. } = input else {
            unreachable!("script Outbox lowering only receives Outbox")
        };
        if ledger.borrow().outbox_binding_count(&site)? != variables.len() {
            return Err("[freeze:contract][script-lexical/outbox-source-drift]".to_owned());
        }
        let receipt =
            crate::mir::builder::stmts::variable_stmt::build_outbox_statement_with_receipt_v1(
                builder, variables,
            )?;
        ledger
            .borrow_mut()
            .record_outbox_receipt(&site, receipt.bindings())?;
        Ok(receipt.result())
    }
}
