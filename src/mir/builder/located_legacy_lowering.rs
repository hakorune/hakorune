//! Disconnected SITE0-R0-EXPR0-L0 located lowering session.
//!
//! The session is stack-scoped and borrows one activation plan. It owns the
//! matching source view and caller ledger, claims every MethodCall before any
//! child descent, and permits raw legacy delegation only after the ledger has
//! proved that the complete prefix contains no activation rows. It is not
//! stored in `MirBuilder` and has no production constructor caller.

use crate::ast::ASTNode;
use crate::mir::callable_result_representation::{
    CallableResultCallerLedgerErrorV1, CallableResultLegacyLocationErrorV1,
    ClaimedCallableResultActivationSiteV1, LegacyBodyInputV1, LegacyExprInputV1, LegacyStmtInputV1,
    VerifiedCallableResultActivationPlanV1, VerifiedCallableResultCallerLedgerV1,
    VerifiedCallableResultInactivePrefixV1, VerifiedCallableResultLegacySourceViewV1,
};
use crate::mir::resolved_semantics::ExprChildRoleV1;
use crate::mir::{MirBuilder, MirType, TypeOpKind, ValueId};

use super::calls::extern_calls::EnvMethodSpec;
use super::calls::{
    emit_env_value_terminal_raw_v1, emit_global_value_terminal_raw_v1,
    emit_standard_value_terminal_raw_v1, emit_typeop_value_terminal_raw_v1,
    CallArgumentDescentPortV1, MethodCallDescentPortV1, MethodCallSyntaxViewV1,
    MethodCallValueTerminalPortV1,
};
use super::ops::{
    drive_ordinary_binary_expression_v1, BinaryExpressionDescentPortV1, BinarySyntaxViewV1,
};
use super::recursive_child_lowering::{
    drive_raw_legacy_body_v1, drive_raw_legacy_expression_v1, drive_raw_legacy_statement_v1,
    with_legacy_expression_recursion_guard_v1, RecursiveChildLoweringPortV1,
};
use super::CanonicalSameModuleCallableKeyV1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir) enum LocatedLegacyLoweringErrorV1 {
    Location(CallableResultLegacyLocationErrorV1),
    Ledger(CallableResultCallerLedgerErrorV1),
    Lowering(String),
    Poisoned,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LocatedLegacyLoweringStateV1 {
    Active,
    Failed,
}

pub(in crate::mir) struct LocatedLegacyLoweringSessionV1<'plan> {
    source: VerifiedCallableResultLegacySourceViewV1<'plan>,
    ledger: VerifiedCallableResultCallerLedgerV1<'plan>,
    state: LocatedLegacyLoweringStateV1,
}

pub(in crate::mir::builder) struct ClaimedLocatedMethodCallInputV1<'plan> {
    expression: LegacyExprInputV1<'plan>,
    _claim: ClaimedCallableResultActivationSiteV1<'plan>,
}

impl<'plan> LocatedLegacyLoweringSessionV1<'plan> {
    pub(in crate::mir) fn verify(
        plan: &'plan VerifiedCallableResultActivationPlanV1,
        caller: &CanonicalSameModuleCallableKeyV1,
    ) -> Result<Self, LocatedLegacyLoweringErrorV1> {
        let source = VerifiedCallableResultLegacySourceViewV1::verify(plan, caller)
            .map_err(LocatedLegacyLoweringErrorV1::Location)?;
        let ledger = VerifiedCallableResultCallerLedgerV1::verify(plan, caller)
            .map_err(LocatedLegacyLoweringErrorV1::Ledger)?;
        Ok(Self {
            source,
            ledger,
            state: LocatedLegacyLoweringStateV1::Active,
        })
    }

    pub(in crate::mir) fn lower_expression(
        &mut self,
        builder: &mut MirBuilder,
        input: LegacyExprInputV1<'plan>,
    ) -> Result<ValueId, LocatedLegacyLoweringErrorV1> {
        self.require_active()?;
        let result = self.lower_expression_active(builder, input);
        self.retain_failure(result)
    }

    pub(in crate::mir) fn lower_body(
        &mut self,
        builder: &mut MirBuilder,
        input: LegacyBodyInputV1<'plan>,
    ) -> Result<ValueId, LocatedLegacyLoweringErrorV1> {
        self.require_active()?;
        let result = self
            .ledger
            .prove_body_inactive(&input)
            .map_err(LocatedLegacyLoweringErrorV1::Ledger)
            .and_then(|proof| delegate_inactive_body(builder, input, proof));
        self.retain_failure(result)
    }

    pub(in crate::mir) fn lower_statement(
        &mut self,
        builder: &mut MirBuilder,
        input: LegacyStmtInputV1<'plan>,
    ) -> Result<ValueId, LocatedLegacyLoweringErrorV1> {
        self.require_active()?;
        let result = self
            .ledger
            .prove_stmt_inactive(&input)
            .map_err(LocatedLegacyLoweringErrorV1::Ledger)
            .and_then(|proof| delegate_inactive_statement(builder, input, proof));
        self.retain_failure(result)
    }

    pub(in crate::mir) fn finish(self) -> Result<(), LocatedLegacyLoweringErrorV1> {
        if self.state == LocatedLegacyLoweringStateV1::Failed {
            return Err(LocatedLegacyLoweringErrorV1::Poisoned);
        }
        self.ledger
            .finish()
            .map_err(LocatedLegacyLoweringErrorV1::Ledger)
    }

    fn lower_expression_active(
        &mut self,
        builder: &mut MirBuilder,
        input: LegacyExprInputV1<'plan>,
    ) -> Result<ValueId, LocatedLegacyLoweringErrorV1> {
        if matches!(input.node(), ASTNode::MethodCall { .. }) {
            let claim = self
                .ledger
                .claim(&input)
                .map_err(LocatedLegacyLoweringErrorV1::Ledger)?;
            let guarded_node_kind = std::mem::discriminant(input.node());
            let claimed = ClaimedLocatedMethodCallInputV1 {
                expression: input,
                _claim: claim,
            };
            return with_legacy_expression_recursion_guard_v1(
                builder,
                guarded_node_kind,
                |builder| builder.build_method_call_from_input_v1(self, &claimed),
            )
            .map_err(LocatedLegacyLoweringErrorV1::Lowering);
        }

        if matches!(input.node(), ASTNode::BinaryOp { .. }) {
            let guarded_node_kind = std::mem::discriminant(input.node());
            return with_legacy_expression_recursion_guard_v1(
                builder,
                guarded_node_kind,
                |builder| drive_ordinary_binary_expression_v1(builder, self, &input),
            )
            .map_err(LocatedLegacyLoweringErrorV1::Lowering);
        }

        let proof = self
            .ledger
            .prove_expr_inactive(&input)
            .map_err(LocatedLegacyLoweringErrorV1::Ledger)?;
        delegate_inactive_expression(builder, input, proof)
    }

    fn require_active(&self) -> Result<(), LocatedLegacyLoweringErrorV1> {
        if self.state == LocatedLegacyLoweringStateV1::Active {
            Ok(())
        } else {
            Err(LocatedLegacyLoweringErrorV1::Poisoned)
        }
    }

    fn retain_failure<T>(
        &mut self,
        result: Result<T, LocatedLegacyLoweringErrorV1>,
    ) -> Result<T, LocatedLegacyLoweringErrorV1> {
        if result.is_err() {
            self.state = LocatedLegacyLoweringStateV1::Failed;
        }
        result
    }
}

fn delegate_inactive_expression(
    builder: &mut MirBuilder,
    input: LegacyExprInputV1<'_>,
    _proof: VerifiedCallableResultInactivePrefixV1<'_>,
) -> Result<ValueId, LocatedLegacyLoweringErrorV1> {
    drive_raw_legacy_expression_v1(builder, input.node().clone())
        .map_err(LocatedLegacyLoweringErrorV1::Lowering)
}

fn delegate_inactive_statement(
    builder: &mut MirBuilder,
    input: LegacyStmtInputV1<'_>,
    _proof: VerifiedCallableResultInactivePrefixV1<'_>,
) -> Result<ValueId, LocatedLegacyLoweringErrorV1> {
    drive_raw_legacy_statement_v1(builder, input.node().clone())
        .map_err(LocatedLegacyLoweringErrorV1::Lowering)
}

fn delegate_inactive_body(
    builder: &mut MirBuilder,
    input: LegacyBodyInputV1<'_>,
    _proof: VerifiedCallableResultInactivePrefixV1<'_>,
) -> Result<ValueId, LocatedLegacyLoweringErrorV1> {
    drive_raw_legacy_body_v1(builder, input.statements().to_vec())
        .map_err(LocatedLegacyLoweringErrorV1::Lowering)
}

impl<'plan> BinaryExpressionDescentPortV1 for LocatedLegacyLoweringSessionV1<'plan> {
    type BinaryInput = LegacyExprInputV1<'plan>;

    fn binary_syntax<'input>(
        &self,
        input: &'input Self::BinaryInput,
    ) -> Result<BinarySyntaxViewV1<'input>, String> {
        match input.node() {
            ASTNode::BinaryOp { operator, .. } => Ok(BinarySyntaxViewV1::new(operator)),
            _ => Err("[located-lowering/binary-input-mismatch]".to_string()),
        }
    }

    fn binary_left_input(
        &self,
        input: &Self::BinaryInput,
    ) -> Result<Self::ExpressionInput, String> {
        self.source
            .child_expr(input, ExprChildRoleV1::BinaryLeft)
            .map_err(|error| format!("[located-lowering/location] {error:?}"))
    }

    fn binary_right_input(
        &self,
        input: &Self::BinaryInput,
    ) -> Result<Self::ExpressionInput, String> {
        self.source
            .child_expr(input, ExprChildRoleV1::BinaryRight)
            .map_err(|error| format!("[located-lowering/location] {error:?}"))
    }
}

impl<'plan> RecursiveChildLoweringPortV1 for LocatedLegacyLoweringSessionV1<'plan> {
    type BodyInput = LegacyBodyInputV1<'plan>;
    type StatementInput = LegacyStmtInputV1<'plan>;
    type ExpressionInput = LegacyExprInputV1<'plan>;

    fn lower_body(
        &mut self,
        builder: &mut MirBuilder,
        input: Self::BodyInput,
    ) -> Result<ValueId, String> {
        Self::lower_body(self, builder, input).map_err(render_session_error)
    }

    fn lower_statement(
        &mut self,
        builder: &mut MirBuilder,
        input: Self::StatementInput,
    ) -> Result<ValueId, String> {
        Self::lower_statement(self, builder, input).map_err(render_session_error)
    }

    fn lower_expression(
        &mut self,
        builder: &mut MirBuilder,
        input: Self::ExpressionInput,
    ) -> Result<ValueId, String> {
        Self::lower_expression(self, builder, input).map_err(render_session_error)
    }
}

impl<'plan> CallArgumentDescentPortV1 for LocatedLegacyLoweringSessionV1<'plan> {
    type ArgumentsInput = ClaimedLocatedMethodCallInputV1<'plan>;

    fn argument_count(&self, input: &Self::ArgumentsInput) -> usize {
        match input.expression.node() {
            ASTNode::MethodCall { arguments, .. } => arguments.len(),
            _ => 0,
        }
    }

    fn argument_syntax<'input>(
        &self,
        input: &'input Self::ArgumentsInput,
        index: usize,
    ) -> Option<&'input ASTNode> {
        match input.expression.node() {
            ASTNode::MethodCall { arguments, .. } => arguments.get(index),
            _ => None,
        }
    }

    fn argument_expression_input(
        &self,
        input: &Self::ArgumentsInput,
        index: usize,
    ) -> Result<Self::ExpressionInput, String> {
        let index = u32::try_from(index)
            .map_err(|_| format!("[located-lowering/argument-index-overflow] index={index}"))?;
        self.source
            .child_expr(&input.expression, ExprChildRoleV1::CallArgument(index))
            .map_err(|error| format!("[located-lowering/location] {error:?}"))
    }
}

impl<'plan> MethodCallDescentPortV1 for LocatedLegacyLoweringSessionV1<'plan> {
    type MethodCallInput = ClaimedLocatedMethodCallInputV1<'plan>;

    fn method_call_syntax<'input>(
        &self,
        input: &'input Self::MethodCallInput,
    ) -> Result<MethodCallSyntaxViewV1<'input>, String> {
        match input.expression.node() {
            ASTNode::MethodCall {
                object,
                method,
                arguments,
                ..
            } => Ok(MethodCallSyntaxViewV1::new(object, method, arguments)),
            _ => Err("[located-lowering/method-input-mismatch]".to_string()),
        }
    }

    fn receiver_expression_input(
        &self,
        input: &Self::MethodCallInput,
    ) -> Result<Self::ExpressionInput, String> {
        self.source
            .child_expr(&input.expression, ExprChildRoleV1::Receiver)
            .map_err(|error| format!("[located-lowering/location] {error:?}"))
    }

    fn call_arguments_input<'input>(
        &self,
        input: &'input Self::MethodCallInput,
    ) -> Result<&'input Self::ArgumentsInput, String> {
        Ok(input)
    }
}

impl MethodCallValueTerminalPortV1 for LocatedLegacyLoweringSessionV1<'_> {
    fn emit_typeop_value_terminal(
        &mut self,
        builder: &mut MirBuilder,
        _input: &Self::MethodCallInput,
        value: ValueId,
        op: TypeOpKind,
        ty: MirType,
    ) -> Result<ValueId, String> {
        emit_typeop_value_terminal_raw_v1(builder, value, op, ty)
    }

    fn emit_static_global_value_terminal(
        &mut self,
        builder: &mut MirBuilder,
        _input: &Self::MethodCallInput,
        owner: &str,
        method: &str,
        checked_source_arity: u32,
        arguments: Vec<ValueId>,
    ) -> Result<ValueId, String> {
        emit_global_value_terminal_raw_v1(builder, owner, method, checked_source_arity, arguments)
            .map(|(value, _)| value)
    }

    fn emit_me_lowered_global_value_terminal(
        &mut self,
        builder: &mut MirBuilder,
        _input: &Self::MethodCallInput,
        owner: &str,
        method: &str,
        checked_source_arity: u32,
        arguments: Vec<ValueId>,
    ) -> Result<ValueId, String> {
        let (value, target) = emit_global_value_terminal_raw_v1(
            builder,
            owner,
            method,
            checked_source_arity,
            arguments,
        )?;
        builder.annotate_call_result_from_func_name(value, &target);
        Ok(value)
    }

    fn emit_env_value_terminal(
        &mut self,
        builder: &mut MirBuilder,
        _input: &Self::MethodCallInput,
        spec: &EnvMethodSpec,
        arguments: Vec<ValueId>,
    ) -> Result<ValueId, String> {
        emit_env_value_terminal_raw_v1(builder, spec, arguments)
    }

    fn emit_standard_value_terminal(
        &mut self,
        builder: &mut MirBuilder,
        _input: &Self::MethodCallInput,
        receiver: ValueId,
        method: String,
        arguments: Vec<ValueId>,
    ) -> Result<ValueId, String> {
        emit_standard_value_terminal_raw_v1(builder, receiver, method, arguments)
    }
}

fn render_session_error(error: LocatedLegacyLoweringErrorV1) -> String {
    format!("[located-lowering/session] {error:?}")
}
