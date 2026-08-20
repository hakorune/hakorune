//! Short-lived exact source scopes for selected structured raw children.
//!
//! This adapter owns no AST policy. Callers prepare every child receipt from
//! the intact parent AST through the neutral role vocabulary before moving
//! syntax into an existing lowering owner.

use std::collections::VecDeque;

use crate::ast::ASTNode;
use crate::mir::{MirBuilder, ValueId};

use crate::mir::resolved_semantics::ExprChildRoleV1;

use super::enum_match_source_demand::EnumMatchSourceDemandPortV1;
use super::normal_script_semantic_lowering_state::{
    ScriptDirectStaticClaimTakeV1, ScriptDirectStaticClaimedRowV1,
};
use super::qmark_source_demand::QMarkPropagationSourceDemandPortV1;
use super::raw_invocation_source_transport::RawInvocationSourceContextV1;
use super::record_literal_source_demand::RecordLiteralSourceDemandPortV1;
use super::recursive_child_lowering::{
    RawFunctionHeaderLookupPortV1, RecursiveChildLoweringPortV1,
};
use super::recursive_child_lowering_port::ScriptDirectStaticClaimIngressV1;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(in crate::mir::builder) enum PreparedRawChildSourceV1 {
    Preserve,
    Exact(RawInvocationSourceContextV1),
}

impl PreparedRawChildSourceV1 {
    pub(in crate::mir::builder) fn expression_child(
        &self,
        parent: &ASTNode,
        role: ExprChildRoleV1,
    ) -> Result<Self, String> {
        match self {
            Self::Preserve => Ok(Self::Preserve),
            Self::Exact(context) => Ok(Self::Exact(context.child_expression(parent, role)?)),
        }
    }
}

pub(in crate::mir::builder) struct RawStructuredChildScopePortV1<'port, Port> {
    child: &'port mut Port,
    expressions: VecDeque<PreparedRawChildSourceV1>,
    bodies: VecDeque<PreparedRawChildSourceV1>,
    statement_body: Option<(RawInvocationSourceContextV1, usize)>,
}

impl<'port, Port> RawStructuredChildScopePortV1<'port, Port> {
    pub(super) fn child(&self) -> &Port {
        self.child
    }

    pub(super) fn child_mut(&mut self) -> &mut Port {
        self.child
    }

    pub(in crate::mir::builder) fn new(
        child: &'port mut Port,
        expressions: Vec<PreparedRawChildSourceV1>,
        bodies: Vec<PreparedRawChildSourceV1>,
    ) -> Self {
        Self {
            child,
            expressions: expressions.into(),
            bodies: bodies.into(),
            statement_body: None,
        }
    }

    pub(in crate::mir::builder) fn for_block_expression(
        child: &'port mut Port,
        prelude: PreparedRawChildSourceV1,
        tail: PreparedRawChildSourceV1,
    ) -> Self {
        let statement_body = match prelude {
            PreparedRawChildSourceV1::Preserve => None,
            PreparedRawChildSourceV1::Exact(context) => Some((context, 0)),
        };
        Self {
            child,
            expressions: [tail].into(),
            bodies: VecDeque::new(),
            statement_body,
        }
    }

    pub(in crate::mir::builder) fn for_body(
        child: &'port mut Port,
        body: PreparedRawChildSourceV1,
    ) -> Self {
        let statement_body = match &body {
            PreparedRawChildSourceV1::Preserve => None,
            PreparedRawChildSourceV1::Exact(context) => Some((context.clone(), 0)),
        };
        Self {
            child,
            expressions: VecDeque::new(),
            bodies: [body].into(),
            statement_body,
        }
    }

    fn next_expression(&mut self) -> Result<PreparedRawChildSourceV1, String> {
        self.expressions.pop_front().ok_or_else(|| {
            "[freeze:contract][raw-structured/expression-demand-overflow]".to_owned()
        })
    }

    fn next_body(&mut self) -> Result<PreparedRawChildSourceV1, String> {
        self.bodies
            .pop_front()
            .ok_or_else(|| "[freeze:contract][raw-structured/body-demand-overflow]".to_owned())
    }

    pub(in crate::mir::builder) fn complete_exact_demands_v1(self) -> Result<(), String> {
        if !self.expressions.is_empty() || !self.bodies.is_empty() {
            return Err(format!(
                "[freeze:contract][raw-structured/unconsumed-demands] expressions={} bodies={}",
                self.expressions.len(),
                self.bodies.len()
            ));
        }
        Ok(())
    }

    /// Preserve a child-lowering rejection before checking exact-demand
    /// completion. A failed child may legitimately leave later siblings
    /// unconsumed; reporting that remainder would mask the primary error.
    pub(in crate::mir::builder) fn complete_after_result_v1<T>(
        self,
        result: Result<T, String>,
    ) -> Result<T, String> {
        match result {
            Ok(value) => {
                self.complete_exact_demands_v1()?;
                Ok(value)
            }
            Err(error) => Err(error),
        }
    }
}

impl<Port> RecursiveChildLoweringPortV1 for RawStructuredChildScopePortV1<'_, Port>
where
    Port: RecursiveChildLoweringPortV1<
        BodyInput = Vec<ASTNode>,
        StatementInput = ASTNode,
        ExpressionInput = ASTNode,
    >,
{
    type BodyInput = Vec<ASTNode>;
    type StatementInput = ASTNode;
    type ExpressionInput = ASTNode;

    fn script_direct_static_claim_ingress_v1(
        &mut self,
        box_name: &str,
        method: &str,
        argument_count: usize,
    ) -> Result<ScriptDirectStaticClaimIngressV1, String> {
        self.child
            .script_direct_static_claim_ingress_v1(box_name, method, argument_count)
    }

    fn take_script_direct_static_claim_v1(
        &mut self,
        box_name: &str,
        method: &str,
        receiver: &ASTNode,
        arguments: &[ASTNode],
    ) -> Result<ScriptDirectStaticClaimTakeV1, String> {
        self.child
            .take_script_direct_static_claim_v1(box_name, method, receiver, arguments)
    }

    fn complete_script_direct_static_claim_v1(
        &mut self,
        claimed: ScriptDirectStaticClaimedRowV1,
    ) -> Result<(), String> {
        self.child.complete_script_direct_static_claim_v1(claimed)
    }

    fn try_emit_source_bound_static_call_result_v1(
        &mut self,
        builder: &mut MirBuilder,
        owner: &str,
        method: &str,
        checked_source_arity: u32,
        arguments: &[ValueId],
    ) -> Result<Option<ValueId>, String> {
        self.child.try_emit_source_bound_static_call_result_v1(
            builder,
            owner,
            method,
            checked_source_arity,
            arguments,
        )
    }

    fn lower_body(
        &mut self,
        builder: &mut MirBuilder,
        input: Self::BodyInput,
    ) -> Result<ValueId, String> {
        let source = self.next_body()?;
        self.child
            .with_prepared_child_source_v1(source, |child| child.lower_body(builder, input))
    }

    fn lower_statement(
        &mut self,
        builder: &mut MirBuilder,
        input: Self::StatementInput,
    ) -> Result<ValueId, String> {
        let Some((body, index)) = self.statement_body.as_mut() else {
            return self.child.lower_statement(builder, input);
        };
        let transport = body.structured_body_statement(input, *index)?;
        *index += 1;
        let (input, context) = RawInvocationSourceContextV1::from_transport(transport);
        self.child
            .with_prepared_child_source_v1(PreparedRawChildSourceV1::Exact(context), |child| {
                child.lower_statement(builder, input)
            })
    }

    fn lower_expression(
        &mut self,
        builder: &mut MirBuilder,
        input: Self::ExpressionInput,
    ) -> Result<ValueId, String> {
        match input {
            ASTNode::Program { statements, span } => {
                let source = self.next_body()?;
                match source {
                    PreparedRawChildSourceV1::Preserve => self
                        .child
                        .lower_expression(builder, ASTNode::Program { statements, span }),
                    source => self.child.with_prepared_child_source_v1(source, |child| {
                        child.lower_body(builder, statements)
                    }),
                }
            }
            input => {
                let source = self.next_expression()?;
                self.child.with_prepared_child_source_v1(source, |child| {
                    child.lower_expression(builder, input)
                })
            }
        }
    }
}

impl<Port> RawFunctionHeaderLookupPortV1 for RawStructuredChildScopePortV1<'_, Port>
where
    Port: RawFunctionHeaderLookupPortV1,
{
    fn with_function_headers<R>(
        &mut self,
        observe: impl for<'headers> FnOnce(
            Option<&'headers dyn super::function_signature_lookup::FunctionSignatureLookupV1>,
        ) -> R,
    ) -> R {
        self.child.with_function_headers(observe)
    }
}

impl<Port> RecordLiteralSourceDemandPortV1 for RawStructuredChildScopePortV1<'_, Port>
where
    Port: RecordLiteralSourceDemandPortV1,
{
    fn record_literal_explicit_field_count_v1(
        &self,
        literal: &ASTNode,
    ) -> Result<Option<u32>, String> {
        self.child.record_literal_explicit_field_count_v1(literal)
    }
}

impl<Port> QMarkPropagationSourceDemandPortV1 for RawStructuredChildScopePortV1<'_, Port>
where
    Port: QMarkPropagationSourceDemandPortV1,
{
    fn has_qmark_propagation_receipt_v1(&self, qmark: &ASTNode) -> Result<bool, String> {
        self.child.has_qmark_propagation_receipt_v1(qmark)
    }
}

impl<Port> EnumMatchSourceDemandPortV1 for RawStructuredChildScopePortV1<'_, Port>
where
    Port: EnumMatchSourceDemandPortV1,
{
    fn has_enum_match_scrutinee_receipt_v1(&self, expression: &ASTNode) -> Result<bool, String> {
        self.child.has_enum_match_scrutinee_receipt_v1(expression)
    }
}

#[cfg(test)]
mod tests {
    use super::{PreparedRawChildSourceV1, RawStructuredChildScopePortV1};

    #[test]
    fn exact_demand_terminal_rejects_unconsumed_receipts() {
        let mut child = ();
        assert!(
            RawStructuredChildScopePortV1::new(&mut child, Vec::new(), Vec::new())
                .complete_exact_demands_v1()
                .is_ok()
        );

        let error = RawStructuredChildScopePortV1::new(
            &mut child,
            Vec::new(),
            vec![PreparedRawChildSourceV1::Preserve],
        )
        .complete_exact_demands_v1()
        .unwrap_err();
        assert_eq!(
            error,
            "[freeze:contract][raw-structured/unconsumed-demands] expressions=0 bodies=1"
        );
    }

    #[test]
    fn failed_child_error_is_not_masked_by_later_expression_demand() {
        let mut child = ();
        let error = RawStructuredChildScopePortV1::new(
            &mut child,
            vec![
                PreparedRawChildSourceV1::Preserve,
                PreparedRawChildSourceV1::Preserve,
            ],
            Vec::new(),
        )
        .complete_after_result_v1::<()>(Err("[primary-child-error]".to_owned()))
        .unwrap_err();
        assert_eq!(error, "[primary-child-error]");
    }
}
