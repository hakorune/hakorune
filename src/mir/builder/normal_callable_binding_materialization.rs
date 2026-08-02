//! Exact root-owner BindingRef materialization for selected callable bodies.
//! Direct child Lambda captures consume the forest's ordered BindingRef
//! receipt through this root ledger. Lambda bodies and descendant-owner
//! materialization remain outside this authority.

use crate::ast::ASTNode;
use crate::mir::builder::stmts::{drive_local_statement_with_receipt_v1, RawLegacyLocalInputV1};
use crate::mir::resolved_semantics::ExprChildRoleV1;
use crate::mir::{MirBuilder, ValueId};

use super::normal_callable_binding_materialization_port::{
    CallableBindingMaterializationPortV1, CallableEntryShapeV1,
};
use super::raw_invocation_source_transport::RawSourceTransportPortV1;
use super::recursive_child_lowering::{
    lower_raw_expression_with_recursion_guard_v1, RawInvocationChildPortV1,
};

impl CallableBindingMaterializationPortV1 for RawInvocationChildPortV1<'_, '_> {
    fn adopt_callable_entry_values_v1(
        &mut self,
        builder: &MirBuilder,
        shape: CallableEntryShapeV1,
    ) -> Result<(), String> {
        let Some(ledger) = self.callable_ledger.clone() else {
            return Ok(());
        };
        let values = shape.prepare_values(builder)?;
        let result = ledger
            .borrow_mut()
            .install_entry_values(values.receiver(), values.parameters());
        result
    }
}

impl RawInvocationChildPortV1<'_, '_> {
    pub(super) fn lower_callable_binding_rebind_v1(
        &mut self,
        builder: &mut MirBuilder,
        input: ASTNode,
    ) -> Result<ValueId, String> {
        let role = match &input {
            ASTNode::Assignment { target, .. }
                if matches!(target.as_ref(), ASTNode::Variable { .. }) =>
            {
                ExprChildRoleV1::AssignmentTarget
            }
            ASTNode::CompoundAssignment { target, .. }
                if matches!(target.as_ref(), ASTNode::Variable { .. }) =>
            {
                ExprChildRoleV1::CompoundAssignmentTarget
            }
            ASTNode::GroupedAssignmentExpr { .. } => ExprChildRoleV1::GroupedAssignmentTarget,
            _ => return Err(freeze("rebind-source-drift")),
        };
        let target_site = self
            .current_source_context_v1()
            .ok_or_else(|| freeze("rebind-parent-site"))?
            .child_expression(&input, role)?
            .site()
            .cloned()
            .ok_or_else(|| freeze("rebind-target-site"))?;
        let value = lower_raw_expression_with_recursion_guard_v1(builder, self, input)?;
        self.record_callable_rebind_v1(&target_site, value)?;
        Ok(value)
    }
    pub(super) fn lower_callable_local_v1(
        &mut self,
        builder: &mut MirBuilder,
        input: ASTNode,
    ) -> Result<ValueId, String> {
        let ledger = self
            .callable_ledger
            .clone()
            .expect("callable local lowering requires its scoped ledger");
        let site = self.current_callable_site_v1("local-site")?;
        let completed = drive_local_statement_with_receipt_v1(
            builder,
            self,
            RawLegacyLocalInputV1::new(input),
        )?;
        ledger
            .borrow_mut()
            .record_completed_local(&site, &completed)?;
        Ok(completed.result())
    }

    pub(super) fn read_callable_variable_v1(&self) -> Result<ValueId, String> {
        let ledger = self
            .callable_ledger
            .as_ref()
            .expect("callable variable lowering requires its scoped ledger");
        let site = self.current_callable_site_v1("variable-site")?;
        ledger.borrow_mut().read_variable(&site)
    }

    pub(super) fn record_callable_rebind_v1(
        &mut self,
        site: &crate::mir::resolved_semantics::SourceNodeSiteV1,
        value: ValueId,
    ) -> Result<(), String> {
        self.callable_ledger
            .as_ref()
            .expect("callable rebind lowering requires its scoped ledger")
            .borrow_mut()
            .rebind(site, value)
    }

    fn current_callable_site_v1(
        &self,
        reason: &str,
    ) -> Result<crate::mir::resolved_semantics::SourceNodeSiteV1, String> {
        self.current_source_context_v1()
            .and_then(|context| context.site().cloned())
            .ok_or_else(|| format!("[freeze:contract][callable-semantic-lowering/{reason}]"))
    }
}

fn freeze(reason: &str) -> String {
    format!("[freeze:contract][callable-semantic-lowering/{reason}]")
}
