//! Disconnected associated-input boundary for ordinary Binary expressions.
//!
//! This box owns only the one-time operator observation and ordered child
//! demand. Operator conversion, arithmetic/comparison semantics, destination
//! allocation, and representation facts remain in the existing completion
//! owner. Logical `And` / `Or` remain owned by SC0 and reject before children.

use crate::ast::BinaryOperator;
use crate::mir::{MirBuilder, ValueId};

use super::super::recursive_child_lowering::{
    drive_legacy_expression_v1, RecursiveChildLoweringPortV1,
};

pub(in crate::mir::builder) struct BinarySyntaxViewV1<'input> {
    operator: &'input BinaryOperator,
}

impl<'input> BinarySyntaxViewV1<'input> {
    pub(in crate::mir::builder) const fn new(operator: &'input BinaryOperator) -> Self {
        Self { operator }
    }

    pub(in crate::mir::builder) const fn operator(&self) -> &'input BinaryOperator {
        self.operator
    }
}

pub(in crate::mir::builder) trait BinaryExpressionDescentPortV1:
    RecursiveChildLoweringPortV1
{
    type BinaryInput;

    fn binary_syntax<'input>(
        &self,
        input: &'input Self::BinaryInput,
    ) -> Result<BinarySyntaxViewV1<'input>, String>;

    fn binary_left_input(&self, input: &Self::BinaryInput)
        -> Result<Self::ExpressionInput, String>;

    fn binary_right_input(
        &self,
        input: &Self::BinaryInput,
    ) -> Result<Self::ExpressionInput, String>;
}

pub(in crate::mir::builder) fn drive_ordinary_binary_expression_v1<Port>(
    builder: &mut MirBuilder,
    port: &mut Port,
    input: &Port::BinaryInput,
) -> Result<ValueId, String>
where
    Port: BinaryExpressionDescentPortV1,
{
    let operator = port.binary_syntax(input)?.operator().clone();
    if matches!(operator, BinaryOperator::And | BinaryOperator::Or) {
        return Err(format!(
            "[binary-expression-descent/logical-short-circuit-owned-by-sc0] operator={operator}"
        ));
    }

    let left_input = port.binary_left_input(input)?;
    let left = drive_legacy_expression_v1(builder, port, left_input)?;
    let right_input = port.binary_right_input(input)?;
    let right = drive_legacy_expression_v1(builder, port, right_input)?;

    builder.build_binary_op_from_values(operator, left, right)
}
