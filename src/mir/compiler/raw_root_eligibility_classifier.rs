//! RAW-SOURCE0 OWNER0 ELIGIBILITY0: ScalarControl0 source classifier.
//!
//! This module is deliberately disconnected from `RawRootPlanV1`.  It owns
//! only a recursive, wildcard-free classification of the first eligible body
//! grammar.  It does not open a Builder, inspect live module state, allocate a
//! slot, or choose a publication route.

use crate::ast::{ASTNode, BinaryOperator, UnaryOperator};

/// The unary operators admitted by the first scalar expression grammar.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum RawScalarUnaryOperator0V1 {
    Minus,
    Not,
    BitNot,
}

/// The expression witness retained by ScalarControl0.  Literal payloads are
/// intentionally not copied: the bound source remains their authority.
#[derive(Debug, PartialEq)]
pub(in crate::mir) enum RawScalarExpr0V1 {
    Literal,
    Variable(Box<str>),
    Unary {
        operator: RawScalarUnaryOperator0V1,
        operand: Box<RawScalarExpr0V1>,
    },
    Binary {
        operator: BinaryOperator,
        left: Box<RawScalarExpr0V1>,
        right: Box<RawScalarExpr0V1>,
    },
}

/// One recursively classified ScalarControl0 statement.
#[derive(Debug, PartialEq)]
pub(in crate::mir) enum RawScalarControl0V1 {
    Expr(RawScalarExpr0V1),
    Print(RawScalarExpr0V1),
    Assignment {
        target: Box<str>,
        value: RawScalarExpr0V1,
    },
    CompoundAssignment {
        target: Box<str>,
        operator: BinaryOperator,
        value: RawScalarExpr0V1,
    },
    Local {
        variables: Box<[Box<str>]>,
        initialized: Box<[Option<RawScalarExpr0V1>]>,
    },
    If {
        condition: RawScalarExpr0V1,
        then_body: Box<[RawScalarControl0V1]>,
        else_body: Option<Box<[RawScalarControl0V1]>>,
    },
    Loop {
        condition: RawScalarExpr0V1,
        body: Box<[RawScalarControl0V1]>,
    },
    LoopRange {
        variable: Box<str>,
        start: RawScalarExpr0V1,
        end: RawScalarExpr0V1,
        body: Box<[RawScalarControl0V1]>,
    },
    Return(Option<RawScalarExpr0V1>),
    Break,
    Continue,
    ScopeBox(Box<[RawScalarControl0V1]>),
}

/// A classified statement list.  The list preserves source order and owns
/// no AST node, ValueId, Builder state, or publication identity.
#[derive(Debug, PartialEq)]
pub(in crate::mir) struct RawScalarControl0ProgramV1 {
    statements: Box<[RawScalarControl0V1]>,
}

impl RawScalarControl0ProgramV1 {
    pub(in crate::mir) fn statements(&self) -> &[RawScalarControl0V1] {
        &self.statements
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum RawScalarUnsupportedSurface0V1 {
    NestedProgram,
    Using,
    Import,
    BuildGate,
    AsyncOrContext,
    Match,
    Closure,
    Declaration,
    StaticData,
    ProcessGlobalSlot,
    Call,
    FieldOrIndex,
    UnsupportedExpression,
    UnsupportedStatement,
}

#[derive(Debug, PartialEq)]
pub(in crate::mir) enum RawScalarControl0ErrorV1 {
    UnsupportedSurface {
        path: Box<[usize]>,
        surface: RawScalarUnsupportedSurface0V1,
    },
    UnsupportedUnary {
        path: Box<[usize]>,
        operator: UnaryOperator,
    },
    InvalidAssignmentTarget {
        path: Box<[usize]>,
    },
    InvalidLocalCardinality {
        path: Box<[usize]>,
        variables: usize,
        initial_values: usize,
        type_annotations: usize,
    },
    TypedLocalBinding {
        path: Box<[usize]>,
    },
    BreakOutsideLoop {
        path: Box<[usize]>,
    },
    ContinueOutsideLoop {
        path: Box<[usize]>,
    },
}

impl RawScalarControl0ErrorV1 {
    pub(in crate::mir) fn path(&self) -> &[usize] {
        match self {
            Self::UnsupportedSurface { path, .. }
            | Self::UnsupportedUnary { path, .. }
            | Self::InvalidAssignmentTarget { path }
            | Self::InvalidLocalCardinality { path, .. }
            | Self::TypedLocalBinding { path }
            | Self::BreakOutsideLoop { path }
            | Self::ContinueOutsideLoop { path } => path,
        }
    }
}

/// Pure classifier for the first Raw eligible body grammar.
#[derive(Debug, Clone, Copy, Default)]
pub(in crate::mir) struct RawScalarControl0ClassifierV1;

impl RawScalarControl0ClassifierV1 {
    /// Classify one source-ordered statement list recursively.  No wildcard
    /// arm exists in the implementation: adding an AST variant requires an
    /// explicit accepted or typed-unsupported disposition here.
    pub(in crate::mir) fn classify_statements(
        statements: &[ASTNode],
    ) -> Result<RawScalarControl0ProgramV1, RawScalarControl0ErrorV1> {
        let mut path = Vec::new();
        let classified = Self::classify_statement_list(statements, &mut path, 0)?;
        Ok(RawScalarControl0ProgramV1 {
            statements: classified.into_boxed_slice(),
        })
    }

    fn classify_statement_list(
        statements: &[ASTNode],
        path: &mut Vec<usize>,
        loop_depth: usize,
    ) -> Result<Vec<RawScalarControl0V1>, RawScalarControl0ErrorV1> {
        let mut out = Vec::with_capacity(statements.len());
        for (index, statement) in statements.iter().enumerate() {
            path.push(index);
            let classified = Self::classify_statement(statement, path, loop_depth)?;
            path.pop();
            out.push(classified);
        }
        Ok(out)
    }

    fn classify_statement(
        node: &ASTNode,
        path: &mut Vec<usize>,
        loop_depth: usize,
    ) -> Result<RawScalarControl0V1, RawScalarControl0ErrorV1> {
        match node {
            ASTNode::Literal { .. } => Ok(RawScalarControl0V1::Expr(RawScalarExpr0V1::Literal)),
            ASTNode::Variable { name, .. } => Ok(RawScalarControl0V1::Expr(
                RawScalarExpr0V1::Variable(name.clone().into_boxed_str()),
            )),
            ASTNode::UnaryOp {
                operator, operand, ..
            } => Ok(RawScalarControl0V1::Expr(Self::classify_unary(
                operator, operand, path,
            )?)),
            ASTNode::BinaryOp {
                operator,
                left,
                right,
                ..
            } => Ok(RawScalarControl0V1::Expr(RawScalarExpr0V1::Binary {
                operator: operator.clone(),
                left: Box::new(Self::classify_expr(left, path, 0)?),
                right: Box::new(Self::classify_expr(right, path, 1)?),
            })),
            ASTNode::Print { expression, .. } => Ok(RawScalarControl0V1::Print(
                Self::classify_expr(expression, path, 0)?,
            )),
            ASTNode::Assignment { target, value, .. } => {
                let target = Self::variable_target(target, path)?;
                let value = Self::classify_expr(value, path, 1)?;
                Ok(RawScalarControl0V1::Assignment { target, value })
            }
            ASTNode::CompoundAssignment {
                target,
                operator,
                value,
                ..
            } => {
                let target = Self::variable_target(target, path)?;
                let value = Self::classify_expr(value, path, 1)?;
                Ok(RawScalarControl0V1::CompoundAssignment {
                    target,
                    operator: operator.clone(),
                    value,
                })
            }
            ASTNode::Local {
                variables,
                initial_values,
                declared_type_names,
                ..
            } => {
                if variables.len() != initial_values.len()
                    || variables.len() != declared_type_names.len()
                {
                    return Err(RawScalarControl0ErrorV1::InvalidLocalCardinality {
                        path: path.clone().into_boxed_slice(),
                        variables: variables.len(),
                        initial_values: initial_values.len(),
                        type_annotations: declared_type_names.len(),
                    });
                }
                if declared_type_names.iter().any(Option::is_some) {
                    return Err(RawScalarControl0ErrorV1::TypedLocalBinding {
                        path: path.clone().into_boxed_slice(),
                    });
                }
                let mut values = Vec::with_capacity(initial_values.len());
                for (index, value) in initial_values.iter().enumerate() {
                    values.push(
                        value
                            .as_deref()
                            .map(|expr| Self::classify_expr(expr, path, index))
                            .transpose()?,
                    );
                }
                Ok(RawScalarControl0V1::Local {
                    variables: variables
                        .iter()
                        .map(|name| name.clone().into_boxed_str())
                        .collect::<Vec<_>>()
                        .into_boxed_slice(),
                    initialized: values.into_boxed_slice(),
                })
            }
            ASTNode::If {
                condition,
                then_body,
                else_body,
                ..
            } => {
                let condition = Self::classify_expr(condition, path, 0)?;
                let then_body = Self::classify_nested(then_body, path, 1, loop_depth)?;
                let else_body = else_body
                    .as_deref()
                    .map(|body| Self::classify_nested(body, path, 2, loop_depth))
                    .transpose()?;
                Ok(RawScalarControl0V1::If {
                    condition,
                    then_body,
                    else_body,
                })
            }
            ASTNode::Loop {
                condition, body, ..
            } => {
                let condition = Self::classify_expr(condition, path, 0)?;
                let body = Self::classify_nested(body, path, 1, loop_depth + 1)?;
                Ok(RawScalarControl0V1::Loop { condition, body })
            }
            ASTNode::LoopRange {
                var_name,
                start,
                end,
                body,
                ..
            } => {
                let start = Self::classify_expr(start, path, 0)?;
                let end = Self::classify_expr(end, path, 1)?;
                let body = Self::classify_nested(body, path, 2, loop_depth + 1)?;
                Ok(RawScalarControl0V1::LoopRange {
                    variable: var_name.clone().into_boxed_str(),
                    start,
                    end,
                    body,
                })
            }
            ASTNode::Return { value, .. } => Ok(RawScalarControl0V1::Return(
                value
                    .as_deref()
                    .map(|expr| Self::classify_expr(expr, path, 0))
                    .transpose()?,
            )),
            ASTNode::Break { .. } => {
                if loop_depth == 0 {
                    return Err(RawScalarControl0ErrorV1::BreakOutsideLoop {
                        path: path.clone().into_boxed_slice(),
                    });
                }
                Ok(RawScalarControl0V1::Break)
            }
            ASTNode::Continue { .. } => {
                if loop_depth == 0 {
                    return Err(RawScalarControl0ErrorV1::ContinueOutsideLoop {
                        path: path.clone().into_boxed_slice(),
                    });
                }
                Ok(RawScalarControl0V1::Continue)
            }
            ASTNode::ScopeBox { body, .. } => Ok(RawScalarControl0V1::ScopeBox(
                Self::classify_nested(body, path, 0, loop_depth)?,
            )),
            ASTNode::Program { .. } => {
                Self::unsupported(path, RawScalarUnsupportedSurface0V1::NestedProgram)
            }
            ASTNode::UsingStatement { .. } => {
                Self::unsupported(path, RawScalarUnsupportedSurface0V1::Using)
            }
            ASTNode::ImportStatement { .. } => {
                Self::unsupported(path, RawScalarUnsupportedSurface0V1::Import)
            }
            ASTNode::BuildGate { .. } => {
                Self::unsupported(path, RawScalarUnsupportedSurface0V1::BuildGate)
            }
            ASTNode::TaskScope { .. }
            | ASTNode::ContextScope { .. }
            | ASTNode::FastMemRegion { .. }
            | ASTNode::AwaitExpression { .. }
            | ASTNode::QMarkPropagate { .. }
            | ASTNode::Nowait { .. } => {
                Self::unsupported(path, RawScalarUnsupportedSurface0V1::AsyncOrContext)
            }
            ASTNode::MatchExpr { .. } | ASTNode::EnumMatchExpr { .. } => {
                Self::unsupported(path, RawScalarUnsupportedSurface0V1::Match)
            }
            ASTNode::Lambda { .. } => {
                Self::unsupported(path, RawScalarUnsupportedSurface0V1::Closure)
            }
            ASTNode::New { .. } | ASTNode::FromCall { .. } => {
                Self::unsupported(path, RawScalarUnsupportedSurface0V1::ProcessGlobalSlot)
            }
            ASTNode::BoxDeclaration { .. }
            | ASTNode::FunctionDeclaration { .. }
            | ASTNode::EnumDeclaration { .. }
            | ASTNode::BrandDeclaration { .. }
            | ASTNode::TypeAliasDeclaration { .. }
            | ASTNode::GlobalVar { .. } => {
                Self::unsupported(path, RawScalarUnsupportedSurface0V1::Declaration)
            }
            ASTNode::StaticConstTable { .. } => {
                Self::unsupported(path, RawScalarUnsupportedSurface0V1::StaticData)
            }
            ASTNode::ArrayLiteral { .. }
            | ASTNode::MapLiteral { .. }
            | ASTNode::RecordLiteral { .. }
            | ASTNode::RecordUpdate { .. }
            | ASTNode::CheckExpr { .. }
            | ASTNode::GroupedAssignmentExpr { .. } => {
                Self::unsupported(path, RawScalarUnsupportedSurface0V1::UnsupportedExpression)
            }
            ASTNode::BlockExpr { .. }
            | ASTNode::TryCatch { .. }
            | ASTNode::Throw { .. }
            | ASTNode::Release { .. } => {
                Self::unsupported(path, RawScalarUnsupportedSurface0V1::UnsupportedStatement)
            }
            ASTNode::Arrow { .. }
            | ASTNode::MethodCall { .. }
            | ASTNode::FunctionCall { .. }
            | ASTNode::Call { .. } => Self::unsupported(path, RawScalarUnsupportedSurface0V1::Call),
            ASTNode::FieldAccess { .. }
            | ASTNode::Index { .. }
            | ASTNode::This { .. }
            | ASTNode::Me { .. }
            | ASTNode::ThisField { .. }
            | ASTNode::MeField { .. } => {
                Self::unsupported(path, RawScalarUnsupportedSurface0V1::FieldOrIndex)
            }
            ASTNode::Outbox { .. } => {
                Self::unsupported(path, RawScalarUnsupportedSurface0V1::UnsupportedStatement)
            }
        }
    }

    fn classify_nested(
        statements: &[ASTNode],
        path: &mut Vec<usize>,
        edge: usize,
        loop_depth: usize,
    ) -> Result<Box<[RawScalarControl0V1]>, RawScalarControl0ErrorV1> {
        path.push(edge);
        let result = Self::classify_statement_list(statements, path, loop_depth);
        path.pop();
        result.map(Vec::into_boxed_slice)
    }

    fn classify_expr(
        node: &ASTNode,
        path: &mut Vec<usize>,
        edge: usize,
    ) -> Result<RawScalarExpr0V1, RawScalarControl0ErrorV1> {
        path.push(edge);
        let result = match node {
            ASTNode::Literal { .. } => Ok(RawScalarExpr0V1::Literal),
            ASTNode::Variable { name, .. } => {
                Ok(RawScalarExpr0V1::Variable(name.clone().into_boxed_str()))
            }
            ASTNode::UnaryOp {
                operator, operand, ..
            } => Self::classify_unary(operator, operand, path),
            ASTNode::BinaryOp {
                operator,
                left,
                right,
                ..
            } => Ok(RawScalarExpr0V1::Binary {
                operator: operator.clone(),
                left: Box::new(Self::classify_expr(left, path, 0)?),
                right: Box::new(Self::classify_expr(right, path, 1)?),
            }),
            ASTNode::Lambda { .. } => {
                Self::unsupported(path, RawScalarUnsupportedSurface0V1::Closure)
            }
            ASTNode::New { .. } | ASTNode::FromCall { .. } => {
                Self::unsupported(path, RawScalarUnsupportedSurface0V1::ProcessGlobalSlot)
            }
            ASTNode::MethodCall { .. }
            | ASTNode::FunctionCall { .. }
            | ASTNode::Call { .. }
            | ASTNode::Arrow { .. } => {
                Self::unsupported(path, RawScalarUnsupportedSurface0V1::Call)
            }
            ASTNode::FieldAccess { .. }
            | ASTNode::Index { .. }
            | ASTNode::This { .. }
            | ASTNode::Me { .. }
            | ASTNode::ThisField { .. }
            | ASTNode::MeField { .. } => {
                Self::unsupported(path, RawScalarUnsupportedSurface0V1::FieldOrIndex)
            }
            ASTNode::ArrayLiteral { .. }
            | ASTNode::MapLiteral { .. }
            | ASTNode::RecordLiteral { .. }
            | ASTNode::RecordUpdate { .. }
            | ASTNode::CheckExpr { .. }
            | ASTNode::GroupedAssignmentExpr { .. }
            | ASTNode::MatchExpr { .. }
            | ASTNode::EnumMatchExpr { .. } => {
                Self::unsupported(path, RawScalarUnsupportedSurface0V1::UnsupportedExpression)
            }
            ASTNode::Program { .. }
            | ASTNode::ScopeBox { .. }
            | ASTNode::BlockExpr { .. }
            | ASTNode::If { .. }
            | ASTNode::Loop { .. }
            | ASTNode::LoopRange { .. }
            | ASTNode::Return { .. }
            | ASTNode::Break { .. }
            | ASTNode::Continue { .. }
            | ASTNode::Release { .. }
            | ASTNode::Print { .. }
            | ASTNode::Assignment { .. }
            | ASTNode::CompoundAssignment { .. }
            | ASTNode::Local { .. }
            | ASTNode::Outbox { .. }
            | ASTNode::UsingStatement { .. }
            | ASTNode::ImportStatement { .. }
            | ASTNode::BuildGate { .. }
            | ASTNode::TaskScope { .. }
            | ASTNode::ContextScope { .. }
            | ASTNode::FastMemRegion { .. }
            | ASTNode::AwaitExpression { .. }
            | ASTNode::QMarkPropagate { .. }
            | ASTNode::Nowait { .. }
            | ASTNode::TryCatch { .. }
            | ASTNode::Throw { .. }
            | ASTNode::BoxDeclaration { .. }
            | ASTNode::FunctionDeclaration { .. }
            | ASTNode::EnumDeclaration { .. }
            | ASTNode::BrandDeclaration { .. }
            | ASTNode::TypeAliasDeclaration { .. }
            | ASTNode::GlobalVar { .. }
            | ASTNode::StaticConstTable { .. } => {
                Self::unsupported(path, RawScalarUnsupportedSurface0V1::UnsupportedExpression)
            }
        };
        path.pop();
        result
    }

    fn classify_unary(
        operator: &UnaryOperator,
        operand: &ASTNode,
        path: &mut Vec<usize>,
    ) -> Result<RawScalarExpr0V1, RawScalarControl0ErrorV1> {
        let operator = match operator {
            UnaryOperator::Minus => RawScalarUnaryOperator0V1::Minus,
            UnaryOperator::Not => RawScalarUnaryOperator0V1::Not,
            UnaryOperator::BitNot => RawScalarUnaryOperator0V1::BitNot,
            UnaryOperator::Weak => {
                return Err(RawScalarControl0ErrorV1::UnsupportedUnary {
                    path: path.clone().into_boxed_slice(),
                    operator: operator.clone(),
                })
            }
        };
        Ok(RawScalarExpr0V1::Unary {
            operator,
            operand: Box::new(Self::classify_expr(operand, path, 0)?),
        })
    }

    fn variable_target(
        target: &ASTNode,
        path: &mut Vec<usize>,
    ) -> Result<Box<str>, RawScalarControl0ErrorV1> {
        match target {
            ASTNode::Variable { name, .. } => Ok(name.clone().into_boxed_str()),
            _ => Err(RawScalarControl0ErrorV1::InvalidAssignmentTarget {
                path: path.clone().into_boxed_slice(),
            }),
        }
    }

    fn unsupported<T>(
        path: &[usize],
        surface: RawScalarUnsupportedSurface0V1,
    ) -> Result<T, RawScalarControl0ErrorV1> {
        Err(RawScalarControl0ErrorV1::UnsupportedSurface {
            path: path.to_vec().into_boxed_slice(),
            surface,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};

    fn literal(value: i64) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::Integer(value),
            span: Span::unknown(),
        }
    }

    #[test]
    fn scalar_control_preserves_recursive_order_and_loop_depth() {
        let source = vec![ASTNode::Loop {
            condition: Box::new(literal(1)),
            body: vec![
                ASTNode::Print {
                    expression: Box::new(ASTNode::BinaryOp {
                        operator: BinaryOperator::Add,
                        left: Box::new(literal(1)),
                        right: Box::new(literal(2)),
                        span: Span::unknown(),
                    }),
                    span: Span::unknown(),
                },
                ASTNode::Continue {
                    span: Span::unknown(),
                },
            ],
            span: Span::unknown(),
        }];
        let program = RawScalarControl0ClassifierV1::classify_statements(&source).unwrap();
        assert_eq!(program.statements().len(), 1);
        assert!(matches!(
            program.statements()[0],
            RawScalarControl0V1::Loop { .. }
        ));
    }

    #[test]
    fn unsupported_lambda_is_typed_and_path_bearing() {
        let source = vec![ASTNode::Print {
            expression: Box::new(ASTNode::Lambda {
                params: Vec::new(),
                body: Vec::new(),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        }];
        let error = RawScalarControl0ClassifierV1::classify_statements(&source).unwrap_err();
        assert!(matches!(
            error,
            RawScalarControl0ErrorV1::UnsupportedSurface {
                surface: RawScalarUnsupportedSurface0V1::Closure,
                ..
            }
        ));
        assert!(!error.path().is_empty());
    }

    #[test]
    fn local_cardinality_and_typed_bindings_are_checked() {
        let mismatch = vec![ASTNode::Local {
            variables: vec!["x".into()],
            initial_values: Vec::new(),
            declared_type_names: vec![None],
            span: Span::unknown(),
        }];
        assert!(matches!(
            RawScalarControl0ClassifierV1::classify_statements(&mismatch),
            Err(RawScalarControl0ErrorV1::InvalidLocalCardinality { .. })
        ));

        let typed = vec![ASTNode::Local {
            variables: vec!["x".into()],
            initial_values: vec![Some(Box::new(literal(1)))],
            declared_type_names: vec![Some("i64".into())],
            span: Span::unknown(),
        }];
        assert!(matches!(
            RawScalarControl0ClassifierV1::classify_statements(&typed),
            Err(RawScalarControl0ErrorV1::TypedLocalBinding { .. })
        ));
    }
}
