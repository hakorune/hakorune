//! Select source-owned lowering for a RawInvocation node.

use std::ops::ControlFlow;

use super::{
    ASTNode, ExprChildRoleV1, MirBuilder, RawInvocationChildPortV1, RawSourceTransportPortV1,
    SourceLedgerV1, ValueId,
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum SourceNodePosition {
    Statement,
    Expression,
}

impl RawInvocationChildPortV1<'_, '_> {
    // Early statement selection preserves the block span and expression observers
    // used by the remaining arms on their way through ordinary expression descent.
    pub(super) fn lower_source_node_v1(
        &mut self,
        builder: &mut MirBuilder,
        input: ASTNode,
        position: SourceNodePosition,
    ) -> Result<ControlFlow<ValueId, ASTNode>, String> {
        use SourceLedgerV1::{Callable, None as NoLedger, Script};

        let expression = position == SourceNodePosition::Expression;
        let rebind = if expression {
            binding_rebind_target_role_v1(&input)
        } else {
            None
        };
        let value = match (&self.source_ledger, input, rebind) {
            (NoLedger, input, _) => return Ok(ControlFlow::Continue(input)),
            (_, local @ ASTNode::Local { .. }, _) => self
                .lower_source_local_v1(builder, local)
                .map(|completed| completed.result()),
            (
                Script(_),
                ASTNode::Nowait {
                    variable,
                    expression,
                    ..
                },
                _,
            ) => self.lower_script_nowait_v1(builder, variable, *expression),
            (Script(_), ASTNode::Outbox { variables, .. }, _) => {
                self.lower_script_outbox_v1(builder, variables)
            }
            (Script(_), node @ ASTNode::Return { .. }, _) if expression => {
                self.lower_script_return_v1(builder, node)
            }
            (Script(_), node, Some(role)) => {
                self.lower_script_binding_rebind_v1(builder, node, role)
            }
            (Callable(_), node, Some(role)) => {
                self.lower_callable_binding_rebind_v1(builder, node, Some(role))
            }
            (Callable(_), ASTNode::MapLiteral { .. }, _) if expression => {
                self.lower_callable_map_v1(builder)
            }
            (Callable(_), ASTNode::New { .. }, _)
                if expression && self.issued_builtin_empty_map_construction_v1()? =>
            {
                self.lower_callable_map_v1(builder)
            }
            (Callable(_), ASTNode::Variable { .. }, _) if expression => {
                self.read_callable_variable_v1()
            }
            (Script(ledger), ASTNode::Variable { .. }, _) if expression => {
                let site = self
                    .current_source_context_v1()
                    .and_then(|context| context.site().cloned())
                    .ok_or_else(|| "[freeze:contract][script-lexical/variable-site]".to_owned())?;
                let binding = ledger.borrow().variable_binding(&site).ok_or_else(|| {
                    "[freeze:contract][script-lexical/variable-binding]".to_owned()
                })?;
                ledger
                    .borrow()
                    .value(binding)
                    .ok_or_else(|| "[freeze:contract][script-lexical/variable-value]".to_owned())
            }
            (_, input, _) => return Ok(ControlFlow::Continue(input)),
        }?;
        Ok(ControlFlow::Break(value))
    }
}

pub(in crate::mir::builder) fn binding_rebind_target_role_v1(
    node: &ASTNode,
) -> Option<ExprChildRoleV1> {
    match node {
        ASTNode::Assignment { target, .. }
            if matches!(target.as_ref(), ASTNode::Variable { .. }) =>
        {
            Some(ExprChildRoleV1::AssignmentTarget)
        }
        ASTNode::CompoundAssignment { target, .. }
            if matches!(target.as_ref(), ASTNode::Variable { .. }) =>
        {
            Some(ExprChildRoleV1::CompoundAssignmentTarget)
        }
        ASTNode::GroupedAssignmentExpr { .. } => Some(ExprChildRoleV1::GroupedAssignmentTarget),
        _ => None,
    }
}
