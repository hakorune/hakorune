use super::*;
use crate::mir::builder::recursive_child_lowering_port::DeclaredInstanceReceiverIngressV1;

impl RecursiveChildLoweringPortV1 for RawInvocationChildPortV1<'_, '_> {
    type BodyInput = Vec<ASTNode>;
    type StatementInput = ASTNode;
    type ExpressionInput = ASTNode;

    fn script_direct_static_claim_ingress_v1(
        &mut self,
        _box_name: &str,
        _method: &str,
        _argument_count: usize,
    ) -> Result<ScriptDirectStaticClaimIngressV1, String> {
        self.script_direct_static_claim_ingress_inner_v1(_box_name, _method, _argument_count)
    }

    fn take_script_direct_static_claim_v1(
        &mut self,
        box_name: &str,
        method: &str,
        _receiver: &ASTNode,
        arguments: &[ASTNode],
    ) -> Result<ScriptDirectStaticClaimTakeV1, String> {
        self.take_script_direct_static_claim_inner_v1(box_name, method, _receiver, arguments)
    }

    fn complete_script_direct_static_claim_v1(
        &mut self,
        claimed: ScriptDirectStaticClaimedRowV1,
    ) -> Result<(), String> {
        self.complete_script_direct_static_claim_inner_v1(claimed)
    }

    fn take_declared_instance_receiver_value_v1(
        &mut self,
        builder: &MirBuilder,
    ) -> Result<DeclaredInstanceReceiverIngressV1, String> {
        self.take_declared_instance_receiver_value_inner_v1(builder)
    }

    fn cleanup_exit_policy_v1(
        &self,
    ) -> crate::mir::builder::control_flow::cleanup::CleanupExitPolicyV1 {
        self.cleanup_exit_policy
    }

    fn lower_body(
        &mut self,
        builder: &mut MirBuilder,
        input: Self::BodyInput,
    ) -> Result<ValueId, String> {
        match self.current_source_context_v1() {
            Some(context) => {
                crate::mir::builder::raw_invocation_body::drive_located_invocation_body_v1(
                    builder, self, input, context,
                )
            }
            None => Err("[freeze:contract][raw-invocation/missing-root-body-receipt]".to_owned()),
        }
    }

    fn lower_statement(
        &mut self,
        builder: &mut MirBuilder,
        input: Self::StatementInput,
    ) -> Result<ValueId, String> {
        if self.active_source.is_none() {
            return Err(
                "[freeze:contract][raw-invocation/missing-statement-source-receipt]".to_owned(),
            );
        }
        if self.callable_ledger.is_some() && matches!(input, ASTNode::Local { .. }) {
            return self.lower_callable_local_v1(builder, input);
        }
        if self.semantic_ledger.is_some() {
            return match input {
                local @ ASTNode::Local { .. } => self.lower_script_local_v1(builder, local),
                nowait @ ASTNode::Nowait { .. } => self.lower_script_nowait_v1(builder, nowait),
                outbox @ ASTNode::Outbox { .. } => self.lower_script_outbox_v1(builder, outbox),
                other => crate::mir::builder::stmts::block_stmt::build_statement_with_port_v1(
                    builder, self, other,
                ),
            };
        }
        crate::mir::builder::stmts::block_stmt::build_statement_with_port_v1(builder, self, input)
    }

    fn lower_expression(
        &mut self,
        builder: &mut MirBuilder,
        input: Self::ExpressionInput,
    ) -> Result<ValueId, String> {
        if self.active_source.is_none() {
            return Err(
                "[freeze:contract][raw-invocation/missing-expression-source-receipt]".to_owned(),
            );
        }
        if self.semantic_ledger.is_some() && matches!(input, ASTNode::Local { .. }) {
            return self.lower_script_local_v1(builder, input);
        }
        if self.callable_ledger.is_some() && matches!(input, ASTNode::Local { .. }) {
            return self.lower_callable_local_v1(builder, input);
        }
        if self.semantic_ledger.is_some() && matches!(input, ASTNode::Nowait { .. }) {
            return self.lower_script_nowait_v1(builder, input);
        }
        if self.semantic_ledger.is_some() && matches!(input, ASTNode::Outbox { .. }) {
            return self.lower_script_outbox_v1(builder, input);
        }
        if self.semantic_ledger.is_some()
            && (matches!(
                &input,
                ASTNode::Assignment { target, .. } | ASTNode::CompoundAssignment { target, .. }
                    if matches!(target.as_ref(), ASTNode::Variable { .. })
            ) || matches!(&input, ASTNode::GroupedAssignmentExpr { .. }))
        {
            return self.lower_script_binding_rebind_v1(builder, input);
        }
        if self.callable_ledger.is_some()
            && (matches!(
                &input,
                ASTNode::Assignment { target, .. } | ASTNode::CompoundAssignment { target, .. }
                    if matches!(target.as_ref(), ASTNode::Variable { .. })
            ) || matches!(&input, ASTNode::GroupedAssignmentExpr { .. }))
        {
            return self.lower_callable_binding_rebind_v1(builder, input);
        }
        if self.callable_ledger.is_some() && matches!(input, ASTNode::Variable { .. }) {
            return self.read_callable_variable_v1();
        }
        if let (Some(ledger), ASTNode::Variable { .. }) = (&self.semantic_ledger, &input) {
            let site = self
                .current_source_context_v1()
                .and_then(|context| context.site().cloned())
                .ok_or_else(|| "[freeze:contract][script-lexical/variable-site]".to_owned())?;
            let binding = ledger
                .borrow()
                .variable_binding(&site)
                .ok_or_else(|| "[freeze:contract][script-lexical/variable-binding]".to_owned())?;
            return ledger
                .borrow()
                .value(binding)
                .ok_or_else(|| "[freeze:contract][script-lexical/variable-value]".to_owned());
        }
        lower_raw_expression_with_recursion_guard_v1(builder, self, input)
    }

    fn prepare_expression_child_source_v1(
        &self,
        parent: &ASTNode,
        role: ExprChildRoleV1,
    ) -> Result<PreparedRawChildSourceV1, String> {
        let context = self
            .current_source_context_v1()
            .ok_or_else(|| {
                "[freeze:contract][raw-invocation/missing-parent-expression-receipt]".to_owned()
            })?
            .child_expression(parent, role)?;
        Ok(PreparedRawChildSourceV1::Exact(context))
    }

    fn prepare_body_child_source_v1(
        &self,
        parent: &ASTNode,
        role: BodyChildRoleV1,
    ) -> Result<PreparedRawChildSourceV1, String> {
        let context = self
            .current_source_context_v1()
            .ok_or_else(|| {
                "[freeze:contract][raw-invocation/missing-parent-body-receipt]".to_owned()
            })?
            .child_body(parent, role)?;
        Ok(PreparedRawChildSourceV1::Exact(context))
    }

    fn prepare_body_statement_source_v1(
        &self,
        statement: &ASTNode,
        index: usize,
    ) -> Result<PreparedRawChildSourceV1, String> {
        let context = self
            .current_source_context_v1()
            .ok_or_else(|| {
                "[freeze:contract][raw-invocation/missing-parent-statement-receipt]".to_owned()
            })?
            .child_statement(statement, index)?;
        Ok(PreparedRawChildSourceV1::Exact(context))
    }

    fn with_prepared_child_source_v1<R>(
        &mut self,
        source: PreparedRawChildSourceV1,
        execute: impl FnOnce(&mut Self) -> R,
    ) -> R {
        match source {
            PreparedRawChildSourceV1::Preserve => execute(self),
            PreparedRawChildSourceV1::Exact(source) => {
                let parent = self.active_source.replace(source);
                let result = execute(self);
                self.active_source = parent;
                result
            }
        }
    }

    fn with_call_argument_source_v1<R>(
        &mut self,
        index: usize,
        execute: impl FnOnce(&mut Self) -> R,
    ) -> R {
        let source = self
            .active_source
            .as_ref()
            .map(|source| source.child_call_argument(index));
        let parent = source.and_then(|source| self.active_source.replace(source));
        let result = execute(self);
        self.active_source = parent;
        result
    }
}
