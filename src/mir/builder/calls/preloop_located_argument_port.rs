//! Candidate-only wrapper for one source-sealed pre-loop call argument.
//!
//! This PORT0 vocabulary wraps the existing `MethodCallLoweringPortV1`
//! capability without changing the ordinary port or creating a second ordered
//! argument driver. Its selected argument is deliberately fail-closed until
//! the later I0 ingress owns the isolated candidate transaction.

use crate::ast::ASTNode;
use crate::mir::builder::me_call_header_observation::{
    MeCallHeaderObservationPortV1, MeCallParameterObservationV1,
};
use crate::mir::builder::recursive_child_lowering::RecursiveChildLoweringPortV1;
use crate::mir::source_instance_result_contract::PreparedPreloopLocatedArgumentV1;
use crate::mir::{MirBuilder, MirType, TypeOpKind, ValueId};

use super::extern_calls::EnvMethodSpec;
use super::method_call_descent::{MethodCallDescentPortV1, MethodCallSyntaxViewV1};
use super::method_call_terminal::MethodCallValueTerminalPortV1;
use super::{CallArgumentDescentPortV1, PreloopLocatedArgumentPortErrorV1};

/// One-shot state is retained in the candidate Port, not in `MirBuilder`.
/// PORT0 creates only the armed state; I0 will own the consuming transition.
#[derive(Debug)]
pub(in crate::mir::builder) enum PreloopSelectedArgumentStateV1<'site, 'view, 'catalog> {
    Armed(PreparedPreloopLocatedArgumentV1<'site, 'view, 'catalog>),
    InFlight,
    Consumed,
    Poisoned,
}

/// Expression input carried by the candidate-only Port.
///
/// The wrapper prevents the generic Raw AST blanket from claiming this Port.
/// Ordinary expressions retain their existing input; the selected relation
/// stays source-located until the later candidate ingress consumes it.
#[derive(Debug)]
pub(in crate::mir::builder) enum PreloopLocatedExpressionInputV1<
    'site,
    'view,
    'catalog,
    ExpressionInput,
> {
    Ordinary(ExpressionInput),
    Selected(PreparedPreloopLocatedArgumentV1<'site, 'view, 'catalog>),
}

/// Route state is separate from source association state. The later ingress
/// may only select an exact prepared `MeStandardUnified` route; PORT0 has no
/// execution consumer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) enum PreloopLocatedArgumentRouteStateV1 {
    Unconnected,
    MeStandardUnified,
    AlternateRejected,
}

/// Stack-scoped candidate wrapper. Its ordinary port remains the sole owner
/// of ordinary syntax, ordered descent, terminal emission, and header facts.
/// The selected source relation is only an opaque future ingress capability.
#[derive(Debug)]
pub(in crate::mir::builder) struct PreloopLocatedArgumentPortV1<'site, 'view, 'catalog, Port>
where
    Port: MethodCallDescentPortV1,
{
    ordinary: Port,
    selected_index: u32,
    selected: PreloopSelectedArgumentStateV1<'site, 'view, 'catalog>,
    route: PreloopLocatedArgumentRouteStateV1,
}

impl<'site, 'view, 'catalog, Port> PreloopLocatedArgumentPortV1<'site, 'view, 'catalog, Port>
where
    Port: MethodCallDescentPortV1,
{
    pub(in crate::mir::builder) fn new(
        ordinary: Port,
        selected: PreparedPreloopLocatedArgumentV1<'site, 'view, 'catalog>,
    ) -> Self {
        Self {
            selected_index: selected.selected().index(),
            ordinary,
            selected: PreloopSelectedArgumentStateV1::Armed(selected),
            route: PreloopLocatedArgumentRouteStateV1::Unconnected,
        }
    }

    pub(in crate::mir::builder) const fn selected_index(&self) -> u32 {
        self.selected_index
    }

    pub(in crate::mir::builder) const fn route(&self) -> PreloopLocatedArgumentRouteStateV1 {
        self.route
    }

    pub(in crate::mir::builder) fn selected_state(
        &self,
    ) -> &PreloopSelectedArgumentStateV1<'site, 'view, 'catalog> {
        &self.selected
    }

    pub(in crate::mir::builder) fn discard(self) {}

    fn take_selected(
        &mut self,
    ) -> Result<PreparedPreloopLocatedArgumentV1<'site, 'view, 'catalog>, String> {
        let selected =
            std::mem::replace(&mut self.selected, PreloopSelectedArgumentStateV1::InFlight);
        match selected {
            PreloopSelectedArgumentStateV1::Armed(selected) => Ok(selected),
            PreloopSelectedArgumentStateV1::InFlight
            | PreloopSelectedArgumentStateV1::Consumed
            | PreloopSelectedArgumentStateV1::Poisoned => {
                self.selected = PreloopSelectedArgumentStateV1::Poisoned;
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
    Port: MethodCallDescentPortV1,
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
            PreloopLocatedExpressionInputV1::Selected(selected) => {
                self.selected = PreloopSelectedArgumentStateV1::Poisoned;
                selected.discard();
                Err(PreloopLocatedArgumentPortErrorV1::CandidateIngressPending.bounded_message())
            }
        }
    }
}

impl<'site, 'view, 'catalog, Port> CallArgumentDescentPortV1
    for PreloopLocatedArgumentPortV1<'site, 'view, 'catalog, Port>
where
    Port: MethodCallDescentPortV1,
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
            return self
                .take_selected()
                .map(PreloopLocatedExpressionInputV1::Selected);
        }

        self.ordinary
            .argument_expression_input(input, index)
            .map(PreloopLocatedExpressionInputV1::Ordinary)
    }
}

impl<'site, 'view, 'catalog, Port> MethodCallDescentPortV1
    for PreloopLocatedArgumentPortV1<'site, 'view, 'catalog, Port>
where
    Port: MethodCallDescentPortV1,
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
    Port: MethodCallDescentPortV1 + MeCallHeaderObservationPortV1,
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
    Port: MethodCallDescentPortV1 + MethodCallValueTerminalPortV1,
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
        self.ordinary.emit_static_global_value_terminal(
            builder,
            owner,
            method,
            checked_source_arity,
            arguments,
        )
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
