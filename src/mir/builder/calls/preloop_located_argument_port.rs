//! Candidate-only wrapper for one source-sealed pre-loop call argument.
//!
//! This PORT0 vocabulary wraps the existing `MethodCallLoweringPortV1`
//! capability without changing the ordinary port or creating a second ordered
//! argument driver. The exact selected source owner always remains inside this
//! Port; argument projection releases only an unforgeable one-shot token.

use crate::ast::ASTNode;
use crate::mir::builder::me_call_header_observation::{
    MeCallHeaderObservationPortV1, MeCallParameterObservationV1, MethodCallLoweringPortV1,
};
use crate::mir::builder::recursive_child_lowering::{
    RawFunctionHeaderLookupPortV1, RecursiveChildLoweringPortV1,
};
use crate::mir::source_instance_result_contract::PreparedPreloopLocatedArgumentV1;
use crate::mir::{MirBuilder, MirType, TypeOpKind, ValueId};

use super::extern_calls::EnvMethodSpec;
use super::method_call_descent::{MethodCallDescentPortV1, MethodCallSyntaxViewV1};
use super::method_call_terminal::MethodCallValueTerminalPortV1;
use super::preloop_located_argument_ingress::{
    lower_selected_preloop_located_argument_v1, PreloopLocatedArgumentIngressErrorV1,
    RejectedPreloopLocatedArgumentIngressV1,
};
use super::preloop_located_argument_rejection::PreloopLocatedArgumentPortErrorV1;
use super::preloop_nested_result_receipt::{
    EmittedNestedInstanceCallV1, ReachedPreloopNestedPhysicalCallV1,
};
use super::CallArgumentDescentPortV1;

/// One-shot state is retained in the candidate Port, not in `MirBuilder`.
/// `Transitioning` is a private synchronous move slot only; every externally
/// observable non-terminal or terminal state retains the exact source owner.
#[derive(Debug)]
pub(super) enum PreloopSelectedArgumentStateV1<'site, 'view, 'catalog> {
    Armed(PreparedPreloopLocatedArgumentV1<'site, 'view, 'catalog>),
    InFlight(PreparedPreloopLocatedArgumentV1<'site, 'view, 'catalog>),
    Transitioning,
    ReachedPhysical(ReachedPreloopNestedPhysicalCallV1<'site, 'view, 'catalog>),
    Emitted(EmittedNestedInstanceCallV1),
    Rejected(RejectedPreloopLocatedArgumentIngressV1<'site, 'view, 'catalog>),
}

/// The private seal prevents a caller from supplying a different source owner
/// after the Port has armed its exact structural `CallArgument` relation.
#[derive(Debug)]
pub(super) struct PreloopSelectedArgumentTokenV1 {
    _seal: PreloopSelectedArgumentTokenSealV1,
}

#[derive(Debug)]
struct PreloopSelectedArgumentTokenSealV1(());

impl PreloopSelectedArgumentTokenV1 {
    fn new() -> Self {
        Self {
            _seal: PreloopSelectedArgumentTokenSealV1(()),
        }
    }
}

/// Expression input carried by the candidate-only Port.
///
/// The wrapper prevents the generic Raw AST blanket from claiming this Port.
/// Ordinary expressions retain their existing input; the selected relation
/// stays in the Port until the candidate ingress consumes it.
#[derive(Debug)]
pub(super) enum PreloopLocatedExpressionInputV1<'site, 'view, 'catalog, ExpressionInput> {
    Ordinary(ExpressionInput),
    Selected {
        token: PreloopSelectedArgumentTokenV1,
        _lifetime:
            std::marker::PhantomData<PreparedPreloopLocatedArgumentV1<'site, 'view, 'catalog>>,
    },
}

/// Stack-scoped candidate wrapper. Its ordinary port remains the sole owner
/// of ordinary syntax, ordered descent, terminal emission, and header facts.
/// The selected source relation is consumed only by the bounded located
/// ingress and remains retained through success and rejection.
#[derive(Debug)]
pub(super) struct PreloopLocatedArgumentPortV1<'site, 'view, 'catalog, Port>
where
    Port: MethodCallLoweringPortV1
        + CallArgumentDescentPortV1<ArgumentsInput = [ASTNode]>
        + RawFunctionHeaderLookupPortV1,
{
    ordinary: Port,
    selected_index: u32,
    selected: PreloopSelectedArgumentStateV1<'site, 'view, 'catalog>,
}

impl<'site, 'view, 'catalog, Port> PreloopLocatedArgumentPortV1<'site, 'view, 'catalog, Port>
where
    Port: MethodCallLoweringPortV1
        + CallArgumentDescentPortV1<ArgumentsInput = [ASTNode]>
        + RawFunctionHeaderLookupPortV1,
{
    pub(super) fn new(
        ordinary: Port,
        selected: PreparedPreloopLocatedArgumentV1<'site, 'view, 'catalog>,
    ) -> Self {
        Self {
            selected_index: selected.selected().index(),
            ordinary,
            selected: PreloopSelectedArgumentStateV1::Armed(selected),
        }
    }

    pub(super) const fn selected_index(&self) -> u32 {
        self.selected_index
    }

    pub(super) fn selected_state(&self) -> &PreloopSelectedArgumentStateV1<'site, 'view, 'catalog> {
        &self.selected
    }

    pub(super) fn into_emitted_nested_result(
        self,
    ) -> Result<
        EmittedNestedInstanceCallV1,
        RejectedPreloopLocatedArgumentIngressV1<'site, 'view, 'catalog>,
    > {
        match self.selected {
            PreloopSelectedArgumentStateV1::Emitted(receipt) => Ok(receipt),
            PreloopSelectedArgumentStateV1::Rejected(rejected) => Err(rejected),
            PreloopSelectedArgumentStateV1::Armed(source) => {
                Err(RejectedPreloopLocatedArgumentIngressV1::completion(
                    source,
                    PreloopLocatedArgumentIngressErrorV1::SelectedArgumentNotReached,
                ))
            }
            PreloopSelectedArgumentStateV1::InFlight(source) => {
                Err(RejectedPreloopLocatedArgumentIngressV1::completion(
                    source,
                    PreloopLocatedArgumentIngressErrorV1::SelectedArgumentNotCompleted,
                ))
            }
            PreloopSelectedArgumentStateV1::ReachedPhysical(reached) => {
                Err(reached.reject_outer_not_completed())
            }
            PreloopSelectedArgumentStateV1::Transitioning => {
                unreachable!("private synchronous pre-loop transition escaped")
            }
        }
    }

    pub(super) fn discard(self) {}

    fn arm_selected_token(&mut self) -> Result<PreloopSelectedArgumentTokenV1, String> {
        let selected = std::mem::replace(
            &mut self.selected,
            PreloopSelectedArgumentStateV1::Transitioning,
        );
        match selected {
            PreloopSelectedArgumentStateV1::Armed(source) => {
                self.selected = PreloopSelectedArgumentStateV1::InFlight(source);
                Ok(PreloopSelectedArgumentTokenV1::new())
            }
            terminal => {
                self.selected = terminal;
                Err(
                    PreloopLocatedArgumentPortErrorV1::SelectedArgumentUnavailable {
                        index: self.selected_index,
                    }
                    .bounded_message(),
                )
            }
        }
    }
}

impl<'site, 'view, 'catalog, Port> RecursiveChildLoweringPortV1
    for PreloopLocatedArgumentPortV1<'site, 'view, 'catalog, Port>
where
    Port: MethodCallLoweringPortV1
        + CallArgumentDescentPortV1<ArgumentsInput = [ASTNode]>
        + RawFunctionHeaderLookupPortV1,
{
    type BodyInput = Port::BodyInput;
    type StatementInput = Port::StatementInput;
    type ExpressionInput =
        PreloopLocatedExpressionInputV1<'site, 'view, 'catalog, Port::ExpressionInput>;

    fn lower_body(
        &mut self,
        builder: &mut MirBuilder,
        input: Self::BodyInput,
    ) -> Result<ValueId, String> {
        self.ordinary.lower_body(builder, input)
    }

    fn lower_statement(
        &mut self,
        builder: &mut MirBuilder,
        input: Self::StatementInput,
    ) -> Result<ValueId, String> {
        self.ordinary.lower_statement(builder, input)
    }

    fn lower_expression(
        &mut self,
        builder: &mut MirBuilder,
        input: Self::ExpressionInput,
    ) -> Result<ValueId, String> {
        match input {
            PreloopLocatedExpressionInputV1::Ordinary(input) => {
                self.ordinary.lower_expression(builder, input)
            }
            PreloopLocatedExpressionInputV1::Selected {
                token: _token,
                _lifetime: _,
            } => {
                let previous = std::mem::replace(
                    &mut self.selected,
                    PreloopSelectedArgumentStateV1::Transitioning,
                );
                match previous {
                    PreloopSelectedArgumentStateV1::InFlight(source) => {
                        match lower_selected_preloop_located_argument_v1(
                            builder,
                            &mut self.ordinary,
                            source,
                        ) {
                            Ok(reached) => {
                                let final_destination = reached.final_destination();
                                self.selected =
                                    PreloopSelectedArgumentStateV1::ReachedPhysical(reached);
                                Ok(final_destination)
                            }
                            Err(rejected) => {
                                let report = rejected.bounded_report();
                                self.selected = PreloopSelectedArgumentStateV1::Rejected(rejected);
                                Err(report)
                            }
                        }
                    }
                    terminal => {
                        self.selected = terminal;
                        Err(
                            PreloopLocatedArgumentPortErrorV1::SelectedArgumentUnavailable {
                                index: self.selected_index,
                            }
                            .bounded_message(),
                        )
                    }
                }
            }
        }
    }
}

impl<'site, 'view, 'catalog, Port> CallArgumentDescentPortV1
    for PreloopLocatedArgumentPortV1<'site, 'view, 'catalog, Port>
where
    Port: MethodCallLoweringPortV1
        + CallArgumentDescentPortV1<ArgumentsInput = [ASTNode]>
        + RawFunctionHeaderLookupPortV1,
{
    type ArgumentsInput = Port::ArgumentsInput;

    fn argument_count(&self, input: &Self::ArgumentsInput) -> usize {
        self.ordinary.argument_count(input)
    }

    fn argument_syntax<'input>(
        &self,
        input: &'input Self::ArgumentsInput,
        index: usize,
    ) -> Option<&'input ASTNode> {
        self.ordinary.argument_syntax(input, index)
    }

    fn argument_expression_input(
        &mut self,
        input: &Self::ArgumentsInput,
        index: usize,
    ) -> Result<Self::ExpressionInput, String> {
        if u32::try_from(index).ok() == Some(self.selected_index) {
            return self.arm_selected_token().map(|token| {
                PreloopLocatedExpressionInputV1::Selected {
                    token,
                    _lifetime: std::marker::PhantomData,
                }
            });
        }

        self.ordinary
            .argument_expression_input(input, index)
            .map(PreloopLocatedExpressionInputV1::Ordinary)
    }
}

impl<'site, 'view, 'catalog, Port> MethodCallDescentPortV1
    for PreloopLocatedArgumentPortV1<'site, 'view, 'catalog, Port>
where
    Port: MethodCallLoweringPortV1
        + CallArgumentDescentPortV1<ArgumentsInput = [ASTNode]>
        + RawFunctionHeaderLookupPortV1,
{
    type MethodCallInput = Port::MethodCallInput;

    fn method_call_syntax<'input>(
        &self,
        input: &'input Self::MethodCallInput,
    ) -> Result<MethodCallSyntaxViewV1<'input>, String> {
        self.ordinary.method_call_syntax(input)
    }

    fn receiver_expression_input(
        &self,
        input: &Self::MethodCallInput,
    ) -> Result<Self::ExpressionInput, String> {
        self.ordinary
            .receiver_expression_input(input)
            .map(PreloopLocatedExpressionInputV1::Ordinary)
    }

    fn call_arguments_input<'input>(
        &self,
        input: &'input Self::MethodCallInput,
    ) -> Result<&'input Self::ArgumentsInput, String> {
        self.ordinary.call_arguments_input(input)
    }
}

impl<'site, 'view, 'catalog, Port> MeCallHeaderObservationPortV1
    for PreloopLocatedArgumentPortV1<'site, 'view, 'catalog, Port>
where
    Port: MethodCallLoweringPortV1
        + CallArgumentDescentPortV1<ArgumentsInput = [ASTNode]>
        + RawFunctionHeaderLookupPortV1,
{
    fn observe_me_call_parameters(
        &mut self,
        builder: &MirBuilder,
        symbol: &str,
    ) -> MeCallParameterObservationV1 {
        self.ordinary.observe_me_call_parameters(builder, symbol)
    }
}

impl<'site, 'view, 'catalog, Port> MethodCallValueTerminalPortV1
    for PreloopLocatedArgumentPortV1<'site, 'view, 'catalog, Port>
where
    Port: MethodCallLoweringPortV1
        + CallArgumentDescentPortV1<ArgumentsInput = [ASTNode]>
        + RawFunctionHeaderLookupPortV1,
{
    fn emit_typeop_value_terminal(
        &mut self,
        builder: &mut MirBuilder,
        value: ValueId,
        op: TypeOpKind,
        ty: MirType,
    ) -> Result<ValueId, String> {
        self.ordinary
            .emit_typeop_value_terminal(builder, value, op, ty)
    }

    fn emit_static_global_value_terminal(
        &mut self,
        builder: &mut MirBuilder,
        owner: &str,
        method: &str,
        checked_source_arity: u32,
        arguments: Vec<ValueId>,
    ) -> Result<ValueId, String> {
        let result = self.ordinary.emit_static_global_value_terminal(
            builder,
            owner,
            method,
            checked_source_arity,
            arguments,
        );
        let previous = std::mem::replace(
            &mut self.selected,
            PreloopSelectedArgumentStateV1::Transitioning,
        );
        self.selected = match (result.as_ref(), previous) {
            (Ok(_), PreloopSelectedArgumentStateV1::ReachedPhysical(reached)) => {
                PreloopSelectedArgumentStateV1::Emitted(reached.complete_after_outer_success())
            }
            (Err(detail), PreloopSelectedArgumentStateV1::ReachedPhysical(reached)) => {
                PreloopSelectedArgumentStateV1::Rejected(
                    reached.reject_outer_terminal(detail.clone()),
                )
            }
            (_, other) => other,
        };
        result
    }

    fn emit_me_lowered_global_value_terminal(
        &mut self,
        builder: &mut MirBuilder,
        owner: &str,
        method: &str,
        checked_source_arity: u32,
        arguments: Vec<ValueId>,
    ) -> Result<ValueId, String> {
        self.ordinary.emit_me_lowered_global_value_terminal(
            builder,
            owner,
            method,
            checked_source_arity,
            arguments,
        )
    }

    fn emit_env_value_terminal(
        &mut self,
        builder: &mut MirBuilder,
        spec: &EnvMethodSpec,
        arguments: Vec<ValueId>,
    ) -> Result<ValueId, String> {
        self.ordinary
            .emit_env_value_terminal(builder, spec, arguments)
    }

    fn emit_standard_value_terminal(
        &mut self,
        builder: &mut MirBuilder,
        receiver: ValueId,
        method: String,
        arguments: Vec<ValueId>,
    ) -> Result<ValueId, String> {
        self.ordinary
            .emit_standard_value_terminal(builder, receiver, method, arguments)
    }
}
