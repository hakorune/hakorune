//! BODY0-S0-B: AST-free LinearScalar0 value lowering.
//!
//! This module consumes the neutral recipe directly.  It deliberately does
//! not reconstruct AST nodes or ask the current module to rediscover source
//! facts.  Publication to the shell/collector remains a later ROOTBATCH0
//! responsibility.

use crate::mir::builder::root_body_completion::RootBodyResultV1;
use crate::mir::raw_root_body_recipe::{
    RawLinearScalarExprV1, RawLinearScalarStmtV1, RawLinearUnaryOperatorV1, RawRootBodyRecipeV1,
};
use crate::mir::{MirBuilder, MirInstruction, UnaryOp, ValueId};

impl MirBuilder {
    /// Lower one exact LinearScalar0 recipe into the current unpublished
    /// function.  The caller owns tracker/session lifecycle; this method only
    /// performs value lowering and returns the last-value disposition.
    pub(in crate::mir::builder) fn lower_linear_scalar_recipe_v1(
        &mut self,
        recipe: &RawRootBodyRecipeV1,
    ) -> Result<RootBodyResultV1, String> {
        let mut last = RootBodyResultV1::NoValue;
        for statement in recipe.statements() {
            last = RootBodyResultV1::Value(self.lower_linear_statement_v1(statement)?);
        }
        Ok(last)
    }

    fn lower_linear_statement_v1(
        &mut self,
        statement: &RawLinearScalarStmtV1,
    ) -> Result<ValueId, String> {
        match statement {
            RawLinearScalarStmtV1::Expr { expression, .. }
            | RawLinearScalarStmtV1::Print { expression, .. } => {
                let value = self.lower_linear_expr_v1(expression)?;
                if matches!(statement, RawLinearScalarStmtV1::Print { .. }) {
                    crate::mir::builder::stmts::print_stmt::build_print_from_value(self, value)?;
                }
                Ok(value)
            }
            RawLinearScalarStmtV1::Assignment { target, value, .. } => {
                let value = self.lower_linear_expr_v1(value)?;
                self.build_assignment_from_value(target.to_string(), value)
            }
            RawLinearScalarStmtV1::CompoundAssignment {
                target,
                operator,
                value,
                ..
            } => {
                let current = self.build_variable_access(target.to_string())?;
                let rhs = self.lower_linear_expr_v1(value)?;
                let combined = self.build_binary_op_from_values(operator.clone(), current, rhs)?;
                self.build_assignment_from_value(target.to_string(), combined)
            }
            RawLinearScalarStmtV1::Local {
                variables,
                initialized,
                ..
            } => {
                let values = initialized
                    .iter()
                    .map(|value| match value {
                        Some(value) => self.lower_linear_expr_v1(value),
                        None => crate::mir::builder::emission::constant::emit_null(self),
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                crate::mir::builder::stmts::variable_stmt::build_local_statement_from_values(
                    self,
                    variables.iter().map(ToString::to_string).collect(),
                    values,
                )
            }
        }
    }

    fn lower_linear_expr_v1(
        &mut self,
        expression: &RawLinearScalarExprV1,
    ) -> Result<ValueId, String> {
        match expression {
            RawLinearScalarExprV1::Literal { value, .. } => self.build_literal(value.clone()),
            RawLinearScalarExprV1::Variable { name, .. } => {
                self.build_variable_access(name.to_string())
            }
            RawLinearScalarExprV1::Unary {
                operator, operand, ..
            } => {
                let operand = self.lower_linear_expr_v1(operand)?;
                self.emit_linear_unary_v1(*operator, operand)
            }
            RawLinearScalarExprV1::Binary {
                operator,
                left,
                right,
                ..
            } => {
                let left = self.lower_linear_expr_v1(left)?;
                let right = self.lower_linear_expr_v1(right)?;
                self.build_binary_op_from_values(operator.clone(), left, right)
            }
        }
    }

    fn emit_linear_unary_v1(
        &mut self,
        operator: RawLinearUnaryOperatorV1,
        operand: ValueId,
    ) -> Result<ValueId, String> {
        let (operator, result_type) = match operator {
            RawLinearUnaryOperatorV1::Minus => ("-", crate::mir::MirType::Integer),
            RawLinearUnaryOperatorV1::Not => ("!", crate::mir::MirType::Bool),
            RawLinearUnaryOperatorV1::BitNot => ("~", crate::mir::MirType::Integer),
        };
        let dst = self.next_value_id();
        let op: UnaryOp =
            crate::mir::builder::ops::converters::convert_unary_operator(operator.to_string())?;
        self.emit_instruction(MirInstruction::UnaryOp { dst, op, operand })?;
        self.function_state
            .type_ctx
            .value_types
            .insert(dst, result_type);
        Ok(dst)
    }
}
