//! Live raw/default associated-input boundary for ordinary Binary expressions.
//!
//! This box owns only the one-time operator observation and ordered child
//! demand. Operator conversion, arithmetic/comparison semantics, destination
//! allocation, and representation facts remain in the existing completion
//! owner. Logical `And` / `Or` remain owned by SC0 and reject before children.

use crate::ast::{ASTNode, BinaryOperator};
use crate::mir::{MirBuilder, ValueId};

use super::super::recursive_child_lowering::{
    drive_legacy_expression_v1, RawAstChildLoweringPortV1, RecursiveChildLoweringPortV1,
};

pub(in crate::mir::builder) struct RawLegacyBinaryInputV1 {
    left: ASTNode,
    operator: BinaryOperator,
    right: ASTNode,
}

impl RawLegacyBinaryInputV1 {
    pub(in crate::mir::builder) const fn new(
        left: ASTNode,
        operator: BinaryOperator,
        right: ASTNode,
    ) -> Self {
        Self {
            left,
            operator,
            right,
        }
    }
}

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

    fn prepare_binary_source_v1(
        &mut self,
        _operator: &BinaryOperator,
    ) -> Result<
        Option<crate::mir::normal_callable_semantic_package::BorrowedCompareSourceLoanV1>,
        String,
    > {
        Ok(None)
    }

    fn complete_binary_source_v1(
        &mut self,
        _loan: crate::mir::normal_callable_semantic_package::BorrowedCompareSourceLoanV1,
        _children: (ValueId, ValueId),
        _completed: &super::CompletedOrdinaryBinaryV1,
    ) -> Result<
        std::rc::Rc<crate::mir::normal_callable_semantic_package::BorrowedCompareMaterializationV1>,
        String,
    > {
        Err("[freeze:contract][borrowed-compare/consumer-unavailable]".into())
    }

    /// Observation of the same finalized append; the default issues no source
    /// proof. Source-scoped consumers must corroborate their retained loan.
    fn complete_binary_expression_v1(
        &mut self,
        _completed: &super::CompletedOrdinaryBinaryV1,
    ) -> Result<(), String> {
        Ok(())
    }
}

impl<Port> BinaryExpressionDescentPortV1 for Port
where
    Port: RawAstChildLoweringPortV1,
{
    type BinaryInput = RawLegacyBinaryInputV1;

    fn prepare_binary_source_v1(
        &mut self,
        operator: &BinaryOperator,
    ) -> Result<
        Option<crate::mir::normal_callable_semantic_package::BorrowedCompareSourceLoanV1>,
        String,
    > {
        self.prepare_borrowed_compare_source_v1(operator)
    }

    fn complete_binary_source_v1(
        &mut self,
        loan: crate::mir::normal_callable_semantic_package::BorrowedCompareSourceLoanV1,
        children: (ValueId, ValueId),
        completed: &super::CompletedOrdinaryBinaryV1,
    ) -> Result<
        std::rc::Rc<crate::mir::normal_callable_semantic_package::BorrowedCompareMaterializationV1>,
        String,
    > {
        self.complete_borrowed_compare_source_v1(loan, children, completed)
    }

    fn complete_binary_expression_v1(
        &mut self,
        completed: &super::CompletedOrdinaryBinaryV1,
    ) -> Result<(), String> {
        self.complete_ordinary_binary_expression_v1(completed)
    }

    fn binary_syntax<'input>(
        &self,
        input: &'input Self::BinaryInput,
    ) -> Result<BinarySyntaxViewV1<'input>, String> {
        Ok(BinarySyntaxViewV1::new(&input.operator))
    }

    fn binary_left_input(
        &self,
        input: &Self::BinaryInput,
    ) -> Result<Self::ExpressionInput, String> {
        Ok(input.left.clone())
    }

    fn binary_right_input(
        &self,
        input: &Self::BinaryInput,
    ) -> Result<Self::ExpressionInput, String> {
        Ok(input.right.clone())
    }
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

    let source = port.prepare_binary_source_v1(&operator)?;
    let left_input = port.binary_left_input(input)?;
    let left = drive_legacy_expression_v1(builder, port, left_input)?;
    let right_input = port.binary_right_input(input)?;
    let right = drive_legacy_expression_v1(builder, port, right_input)?;

    let completed = builder.build_binary_op_from_values_recorded(operator, left, right)?;
    if let Some(source) = source {
        let record = port.complete_binary_source_v1(source, (left, right), &completed)?;
        super::super::ssa::local::checked_compare::install(builder, record)?;
    }
    port.complete_binary_expression_v1(&completed)?;
    Ok(completed.value())
}
