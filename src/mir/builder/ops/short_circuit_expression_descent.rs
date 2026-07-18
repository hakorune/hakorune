//! Disconnected associated-input boundary for logical Binary expressions.
//!
//! This box owns only exact logical-operator admission and child demand. The
//! existing logical short-circuit owner retains branch layout, conditional RHS
//! timing, variable snapshots, result PHI, result type, and diagnostics.

use crate::ast::BinaryOperator;
use crate::mir::{MirBuilder, ValueId};

use super::super::recursive_child_lowering::{
    drive_legacy_expression_v1, RecursiveChildLoweringPortV1,
};
use super::logical_shortcircuit::build_logical_shortcircuit_after_lhs_v1;

pub(in crate::mir::builder) struct ShortCircuitSyntaxViewV1<'input> {
    operator: &'input BinaryOperator,
}

impl<'input> ShortCircuitSyntaxViewV1<'input> {
    pub(in crate::mir::builder) const fn new(operator: &'input BinaryOperator) -> Self {
        Self { operator }
    }

    pub(in crate::mir::builder) const fn operator(&self) -> &'input BinaryOperator {
        self.operator
    }
}

pub(in crate::mir::builder) trait ShortCircuitExpressionDescentPortV1:
    RecursiveChildLoweringPortV1
{
    type ShortCircuitInput;

    fn short_circuit_syntax<'input>(
        &self,
        input: &'input Self::ShortCircuitInput,
    ) -> Result<ShortCircuitSyntaxViewV1<'input>, String>;

    fn short_circuit_left_input(
        &self,
        input: &Self::ShortCircuitInput,
    ) -> Result<Self::ExpressionInput, String>;

    fn short_circuit_right_input(
        &self,
        input: &Self::ShortCircuitInput,
    ) -> Result<Self::ExpressionInput, String>;
}

pub(in crate::mir::builder) fn drive_short_circuit_expression_v1<Port>(
    builder: &mut MirBuilder,
    port: &mut Port,
    input: &Port::ShortCircuitInput,
) -> Result<ValueId, String>
where
    Port: ShortCircuitExpressionDescentPortV1,
{
    let operator = port.short_circuit_syntax(input)?.operator().clone();
    if !matches!(operator, BinaryOperator::And | BinaryOperator::Or) {
        return Err(format!(
            "[short-circuit-expression-descent/ordinary-binary-owned-by-bin0] operator={operator}"
        ));
    }

    let left_input = port.short_circuit_left_input(input)?;
    let lhs = drive_legacy_expression_v1(builder, port, left_input)?;

    build_logical_shortcircuit_after_lhs_v1(builder, operator, lhs, |builder| {
        let right_input = port.short_circuit_right_input(input)?;
        drive_legacy_expression_v1(builder, port, right_input)
    })
}
