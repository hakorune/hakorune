//! Source-facts to neutral BODY recipe projection.
//!
//! This child module owns only the existing located-to-neutral conversion.
//! Script-result policy remains unchanged until its dedicated semantic row.

use crate::ast::{BinaryOperator, LiteralValue};
use crate::mir::builder::VerifiedSameModuleCallableDeclarationCatalogV1;
use crate::mir::raw_root_body_recipe::{
    RawLinearScalarExprV1, RawLinearScalarStmtV1, RawLinearUnaryOperatorV1,
    RawRootBodyEntryContractV1, RawRootBodyRecipeErrorV1, RawRootBodyRecipeV1,
    RawRootBodySourceSiteV1, RawScriptBodyRecipeV1, RawScriptTerminalRecipeV1,
    RawScriptUnitOriginV1, RawUnsupportedBodyStatementKindV1,
};

use super::{
    RawLocatedScalarExprV1, RawLocatedScalarStmtV1, RawLocatedScriptTerminalV1, RawRootBodyFactV1,
    RawRootPostInstallFactsV1, RawRootSourceFactsV1, RawScalarUnaryOperatorV1,
    RawScriptResultContractV1, RawSourceSiteV1,
};

impl RawRootSourceFactsV1 {
    pub(in crate::mir) fn into_post_install_parts(
        self,
    ) -> Result<
        (
            RawRootPostInstallFactsV1,
            VerifiedSameModuleCallableDeclarationCatalogV1,
        ),
        RawRootBodyRecipeErrorV1,
    > {
        let Self {
            route,
            physical,
            main,
            helper_schedule,
            callable_catalog,
            body,
        } = self;
        let body_statement_count = match &body {
            RawRootBodyFactV1::Script(program) => program.statements().len(),
            RawRootBodyFactV1::App { body, .. } => body.statements().len(),
        };
        let callable_count = callable_catalog.len();
        let body_recipe = match body {
            RawRootBodyFactV1::Script(program) => RawRootBodyRecipeV1::from_script_recipe(
                RawRootBodyEntryContractV1::script(),
                project_script_recipe(program)?,
            )?,
            RawRootBodyFactV1::App { main, body } => {
                let statements = body
                    .statements
                    .into_vec()
                    .into_iter()
                    .map(linear_statement)
                    .collect::<Result<Vec<_>, _>>()?
                    .into_boxed_slice();
                RawRootBodyRecipeV1::from_app_parts(
                    RawRootBodyEntryContractV1::app_main0(main.top_level_statement()),
                    statements,
                )?
            }
        };
        Ok((
            RawRootPostInstallFactsV1 {
                route,
                physical,
                main,
                helper_schedule,
                body_recipe,
                body_statement_count,
                callable_count,
            },
            callable_catalog,
        ))
    }
}

pub(in crate::mir) fn project_script_recipe(
    program: super::RawLocatedScalarProgramV1,
) -> Result<RawScriptBodyRecipeV1, RawRootBodyRecipeErrorV1> {
    let contract = script_contract(program)?;
    let prelude = contract
        .prelude
        .into_vec()
        .into_iter()
        .map(linear_statement)
        .collect::<Result<Vec<_>, _>>()?
        .into_boxed_slice();
    let terminal = match contract.terminal {
        RawLocatedScriptTerminalV1::EmptyUnit => RawScriptTerminalRecipeV1::EmptyUnit,
        RawLocatedScriptTerminalV1::ValueExpression { expression } => {
            RawScriptTerminalRecipeV1::ValueExpression(linear_expr(expression)?)
        }
        RawLocatedScriptTerminalV1::UnitExpression { expression, origin } => {
            RawScriptTerminalRecipeV1::UnitExpression {
                expression: linear_expr(expression)?,
                origin,
            }
        }
        RawLocatedScriptTerminalV1::UnitStatement { statement, origin } => {
            RawScriptTerminalRecipeV1::UnitStatement {
                statement: linear_statement(statement)?,
                origin,
            }
        }
    };
    RawScriptBodyRecipeV1::from_parts(prelude, terminal)
}

fn script_contract(
    program: super::RawLocatedScalarProgramV1,
) -> Result<RawScriptResultContractV1, RawRootBodyRecipeErrorV1> {
    let mut statements = program.statements.into_vec();
    let terminal = match statements.pop() {
        None => RawLocatedScriptTerminalV1::EmptyUnit,
        Some(statement @ RawLocatedScalarStmtV1::Expr { .. }) => {
            let expression = match statement {
                RawLocatedScalarStmtV1::Expr { expression, .. } => expression,
                _ => unreachable!("matched expression statement"),
            };
            let unit_origin = match &expression {
                RawLocatedScalarExprV1::Literal { value, .. }
                    if matches!(value, LiteralValue::Null | LiteralValue::Void) =>
                {
                    Some(RawScriptUnitOriginV1::VoidExpression)
                }
                _ => None,
            };
            match unit_origin {
                Some(origin) => RawLocatedScriptTerminalV1::UnitExpression { expression, origin },
                None => RawLocatedScriptTerminalV1::ValueExpression { expression },
            }
        }
        Some(statement @ RawLocatedScalarStmtV1::Print { .. }) => {
            RawLocatedScriptTerminalV1::UnitStatement {
                statement,
                origin: RawScriptUnitOriginV1::PrintStatement,
            }
        }
        Some(statement @ RawLocatedScalarStmtV1::Local { .. }) => {
            RawLocatedScriptTerminalV1::UnitStatement {
                statement,
                origin: RawScriptUnitOriginV1::LocalStatement,
            }
        }
        Some(statement @ RawLocatedScalarStmtV1::Assignment { .. }) => {
            RawLocatedScriptTerminalV1::UnitStatement {
                statement,
                origin: RawScriptUnitOriginV1::AssignmentStatement,
            }
        }
        Some(statement @ RawLocatedScalarStmtV1::CompoundAssignment { .. }) => {
            RawLocatedScriptTerminalV1::UnitStatement {
                statement,
                origin: RawScriptUnitOriginV1::CompoundAssignmentStatement,
            }
        }
        Some(statement) => return Err(unsupported_statement_error(&statement)),
    };
    Ok(RawScriptResultContractV1 {
        prelude: statements.into_boxed_slice(),
        terminal,
    })
}

fn linear_statement(
    statement: RawLocatedScalarStmtV1,
) -> Result<RawLinearScalarStmtV1, RawRootBodyRecipeErrorV1> {
    match statement {
        RawLocatedScalarStmtV1::Expr { expression, site } => Ok(RawLinearScalarStmtV1::Expr {
            expression: linear_expr(expression)?,
            site: neutral_site(&site),
        }),
        RawLocatedScalarStmtV1::Print { expression, site } => Ok(RawLinearScalarStmtV1::Print {
            expression: linear_expr(expression)?,
            site: neutral_site(&site),
        }),
        RawLocatedScalarStmtV1::Assignment {
            target,
            value,
            site,
        } => Ok(RawLinearScalarStmtV1::Assignment {
            target,
            value: linear_expr(value)?,
            site: neutral_site(&site),
        }),
        RawLocatedScalarStmtV1::CompoundAssignment {
            target,
            operator,
            value,
            site,
        } if ordinary_operator(&operator) => Ok(RawLinearScalarStmtV1::CompoundAssignment {
            target,
            operator,
            value: linear_expr(value)?,
            site: neutral_site(&site),
        }),
        RawLocatedScalarStmtV1::Local {
            variables,
            initialized,
            site,
        } => Ok(RawLinearScalarStmtV1::Local {
            variables,
            initialized: initialized
                .into_vec()
                .into_iter()
                .map(|value| value.map(linear_expr).transpose())
                .collect::<Result<Vec<_>, _>>()?
                .into_boxed_slice(),
            site: neutral_site(&site),
        }),
        other => Err(unsupported_statement_error(&other)),
    }
}

fn linear_expr(
    expression: RawLocatedScalarExprV1,
) -> Result<RawLinearScalarExprV1, RawRootBodyRecipeErrorV1> {
    match expression {
        RawLocatedScalarExprV1::Literal { value, site } => Ok(RawLinearScalarExprV1::Literal {
            value,
            site: neutral_site(&site),
        }),
        RawLocatedScalarExprV1::Variable { name, site } => Ok(RawLinearScalarExprV1::Variable {
            name,
            site: neutral_site(&site),
        }),
        RawLocatedScalarExprV1::Unary {
            operator,
            operand,
            site,
        } => Ok(RawLinearScalarExprV1::Unary {
            operator: match operator {
                RawScalarUnaryOperatorV1::Minus => RawLinearUnaryOperatorV1::Minus,
                RawScalarUnaryOperatorV1::Not => RawLinearUnaryOperatorV1::Not,
                RawScalarUnaryOperatorV1::BitNot => RawLinearUnaryOperatorV1::BitNot,
            },
            operand: Box::new(linear_expr(*operand)?),
            site: neutral_site(&site),
        }),
        RawLocatedScalarExprV1::Binary {
            operator,
            left,
            right,
            site,
        } if ordinary_operator(&operator) => Ok(RawLinearScalarExprV1::Binary {
            operator,
            left: Box::new(linear_expr(*left)?),
            right: Box::new(linear_expr(*right)?),
            site: neutral_site(&site),
        }),
        other => Err(RawRootBodyRecipeErrorV1::UnsupportedOperator {
            path: expr_path(&other),
        }),
    }
}

fn ordinary_operator(operator: &BinaryOperator) -> bool {
    !matches!(operator, BinaryOperator::And | BinaryOperator::Or)
}

fn unsupported_statement_error(statement: &RawLocatedScalarStmtV1) -> RawRootBodyRecipeErrorV1 {
    let kind = match statement {
        RawLocatedScalarStmtV1::If { .. } => RawUnsupportedBodyStatementKindV1::If,
        RawLocatedScalarStmtV1::Loop { .. } => RawUnsupportedBodyStatementKindV1::Loop,
        RawLocatedScalarStmtV1::LoopRange { .. } => RawUnsupportedBodyStatementKindV1::LoopRange,
        RawLocatedScalarStmtV1::Return { .. } => RawUnsupportedBodyStatementKindV1::Return,
        RawLocatedScalarStmtV1::Break { .. } => RawUnsupportedBodyStatementKindV1::Break,
        RawLocatedScalarStmtV1::Continue { .. } => RawUnsupportedBodyStatementKindV1::Continue,
        RawLocatedScalarStmtV1::ScopeBox { .. } => RawUnsupportedBodyStatementKindV1::ScopeBox,
        RawLocatedScalarStmtV1::Expr { .. }
        | RawLocatedScalarStmtV1::Print { .. }
        | RawLocatedScalarStmtV1::Assignment { .. }
        | RawLocatedScalarStmtV1::CompoundAssignment { .. }
        | RawLocatedScalarStmtV1::Local { .. } => {
            unreachable!("only rejected located statements reach this helper")
        }
    };
    RawRootBodyRecipeErrorV1::UnsupportedStatement {
        path: statement_path(statement),
        kind,
    }
}

fn neutral_site(site: &RawSourceSiteV1) -> RawRootBodySourceSiteV1 {
    RawRootBodySourceSiteV1::new(site.path(), site.span())
}

fn statement_path(statement: &RawLocatedScalarStmtV1) -> Box<[usize]> {
    match statement {
        RawLocatedScalarStmtV1::Expr { site, .. }
        | RawLocatedScalarStmtV1::Print { site, .. }
        | RawLocatedScalarStmtV1::Assignment { site, .. }
        | RawLocatedScalarStmtV1::CompoundAssignment { site, .. }
        | RawLocatedScalarStmtV1::Local { site, .. }
        | RawLocatedScalarStmtV1::If { site, .. }
        | RawLocatedScalarStmtV1::Loop { site, .. }
        | RawLocatedScalarStmtV1::LoopRange { site, .. }
        | RawLocatedScalarStmtV1::Return { site, .. }
        | RawLocatedScalarStmtV1::Break { site }
        | RawLocatedScalarStmtV1::Continue { site }
        | RawLocatedScalarStmtV1::ScopeBox { site, .. } => site.path().into(),
    }
}

fn expr_path(expression: &RawLocatedScalarExprV1) -> Box<[usize]> {
    match expression {
        RawLocatedScalarExprV1::Literal { site, .. }
        | RawLocatedScalarExprV1::Variable { site, .. }
        | RawLocatedScalarExprV1::Unary { site, .. }
        | RawLocatedScalarExprV1::Binary { site, .. } => site.path().into(),
    }
}
