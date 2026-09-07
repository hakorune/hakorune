//! Script BindingRef materialization hooks for the selected raw port.
//!
//! These hooks consume already-sealed semantic receipts. They never resolve a
//! name or inspect an AST shape to choose an authority.

use crate::ast::ASTNode;
use crate::mir::builder::recursive_child_lowering::RecursiveChildLoweringPortV1;
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
        let relation = ledger
            .borrow()
            .local_relation(&site)
            .cloned()
            .ok_or_else(|| "[freeze:contract][script-lexical/local-binding]".to_owned())?;
        let binding = relation.binding();
        let initializer_source = if relation.initializer_site().is_some() {
            Some(self.prepare_expression_child_source_v1(
                &input,
                crate::mir::resolved_semantics::ExprChildRoleV1::LocalInitializer(0),
            )?)
        } else {
            None
        };
        let source_relation = relation.clone();
        let observations = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let input = RawLegacyLocalInputV1::from_script_relation(
            input,
            relation,
            &site,
            initializer_source.as_ref(),
        )?
        .observe_into(std::rc::Rc::clone(&observations));
        ledger.borrow_mut().consume_array_local(&source_relation)?;
        let value = drive_local_statement_v1(builder, self, input)?;
        let observations = std::rc::Rc::try_unwrap(observations)
            .map_err(|_| "[freeze:contract][script-array/observer-alias]".to_owned())?
            .into_inner();
        ledger.borrow_mut().record_array_local_emission(
            builder,
            &source_relation,
            value,
            observations,
        )?;
        ledger.borrow_mut().record(binding, value)?;
        ledger.borrow_mut().complete_array_local(&source_relation)?;
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
