//! DECLACCESS-MANIFEST0: exact source facts for the first Raw root slice.
//!
//! This is a source-only product. It owns the located ScalarControl0 payload
//! needed by BODY0 and seals the existing complete callable catalog once. It
//! never opens a Builder or reads ambient module state.

use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span, UnaryOperator};
use crate::mir::builder::{RawSourceLocatorV1, VerifiedSameModuleCallableDeclarationCatalogV1};
use crate::mir::raw_root_body_recipe::{
    RawLinearScalarExprV1, RawLinearScalarStmtV1, RawLinearUnaryOperatorV1, RawRootBodyEntryV1,
    RawRootBodyRecipeErrorV1, RawRootBodyRecipeV1, RawRootBodySourceSiteV1,
};

use super::raw_root_plan0::{RawPhysicalRootIdentityV1, RawRootKindV1, RawRootPlanV1};

#[derive(Debug, PartialEq)]
pub(in crate::mir) struct RawSourceSiteV1 {
    path: Box<[usize]>,
    span: Span,
}

impl RawSourceSiteV1 {
    fn new(path: &[usize], span: Span) -> Self {
        Self {
            path: path.to_vec().into_boxed_slice(),
            span,
        }
    }

    pub(in crate::mir) fn path(&self) -> &[usize] {
        &self.path
    }

    pub(in crate::mir) const fn span(&self) -> Span {
        self.span
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum RawScalarUnaryOperatorV1 {
    Minus,
    Not,
    BitNot,
}

#[derive(Debug, PartialEq)]
pub(in crate::mir) enum RawLocatedScalarExprV1 {
    Literal {
        value: LiteralValue,
        site: RawSourceSiteV1,
    },
    Variable {
        name: Box<str>,
        site: RawSourceSiteV1,
    },
    Unary {
        operator: RawScalarUnaryOperatorV1,
        operand: Box<Self>,
        site: RawSourceSiteV1,
    },
    Binary {
        operator: BinaryOperator,
        left: Box<Self>,
        right: Box<Self>,
        site: RawSourceSiteV1,
    },
}

#[derive(Debug, PartialEq)]
pub(in crate::mir) enum RawLocatedScalarStmtV1 {
    Expr {
        expression: RawLocatedScalarExprV1,
        site: RawSourceSiteV1,
    },
    Print {
        expression: RawLocatedScalarExprV1,
        site: RawSourceSiteV1,
    },
    Assignment {
        target: Box<str>,
        value: RawLocatedScalarExprV1,
        site: RawSourceSiteV1,
    },
    CompoundAssignment {
        target: Box<str>,
        operator: BinaryOperator,
        value: RawLocatedScalarExprV1,
        site: RawSourceSiteV1,
    },
    Local {
        variables: Box<[Box<str>]>,
        initialized: Box<[Option<RawLocatedScalarExprV1>]>,
        site: RawSourceSiteV1,
    },
    If {
        condition: RawLocatedScalarExprV1,
        then_body: Box<[Self]>,
        else_body: Option<Box<[Self]>>,
        site: RawSourceSiteV1,
    },
    Loop {
        condition: RawLocatedScalarExprV1,
        body: Box<[Self]>,
        site: RawSourceSiteV1,
    },
    LoopRange {
        variable: Box<str>,
        start: RawLocatedScalarExprV1,
        end: RawLocatedScalarExprV1,
        body: Box<[Self]>,
        site: RawSourceSiteV1,
    },
    Return {
        value: Option<RawLocatedScalarExprV1>,
        site: RawSourceSiteV1,
    },
    Break {
        site: RawSourceSiteV1,
    },
    Continue {
        site: RawSourceSiteV1,
    },
    ScopeBox {
        body: Box<[Self]>,
        site: RawSourceSiteV1,
    },
}

#[derive(Debug, PartialEq)]
pub(in crate::mir) struct RawLocatedScalarProgramV1 {
    statements: Box<[RawLocatedScalarStmtV1]>,
}

impl RawLocatedScalarProgramV1 {
    pub(in crate::mir) fn statements(&self) -> &[RawLocatedScalarStmtV1] {
        &self.statements
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum RawRootSourceRouteV1 {
    Script,
    App,
}

#[derive(Debug, PartialEq)]
pub(in crate::mir) enum RawRootBodyFactV1 {
    Script(RawLocatedScalarProgramV1),
    App {
        main: RawSourceLocatorV1,
        body: RawLocatedScalarProgramV1,
    },
}

#[derive(Debug)]
pub(in crate::mir) struct RawRootSourceFactsV1 {
    route: RawRootSourceRouteV1,
    physical: RawPhysicalRootIdentityV1,
    main: Option<RawSourceLocatorV1>,
    helper_schedule: Box<[RawSourceLocatorV1]>,
    callable_catalog: VerifiedSameModuleCallableDeclarationCatalogV1,
    body: RawRootBodyFactV1,
}

/// Source facts that remain after the Builder-owned declaration/catalog
/// projection has been consumed.  BODY0 keeps this product opaque until its
/// own typed handoff; it is not a second classifier or catalog authority.
#[derive(Debug, PartialEq)]
pub(in crate::mir) struct RawRootPostInstallFactsV1 {
    route: RawRootSourceRouteV1,
    physical: RawPhysicalRootIdentityV1,
    main: Option<RawSourceLocatorV1>,
    helper_schedule: Box<[RawSourceLocatorV1]>,
    body: RawRootBodyFactV1,
}

impl RawRootSourceFactsV1 {
    #[cfg(test)]
    pub(in crate::mir) fn empty_for_test(route: RawRootSourceRouteV1) -> Self {
        let catalog =
            VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(&ASTNode::Program {
                statements: Vec::new(),
                span: Span::unknown(),
            })
            .expect("empty test catalog");
        let body = match route {
            RawRootSourceRouteV1::Script => RawRootBodyFactV1::Script(RawLocatedScalarProgramV1 {
                statements: Box::new([]),
            }),
            RawRootSourceRouteV1::App => RawRootBodyFactV1::App {
                main: RawSourceLocatorV1::for_test(0, "Main", "main", "Main.main/0", 0),
                body: RawLocatedScalarProgramV1 {
                    statements: Box::new([]),
                },
            },
        };
        Self {
            route,
            physical: RawPhysicalRootIdentityV1::fixed(),
            main: None,
            helper_schedule: Box::new([]),
            callable_catalog: catalog,
            body,
        }
    }

    pub(in crate::mir) fn from_source(
        source: &crate::mir::builder::OwnedRawSourceV1,
        plan: &RawRootPlanV1,
    ) -> Result<Self, RawRootSourceFactsErrorV1> {
        let catalog = VerifiedSameModuleCallableDeclarationCatalogV1::seal_root(source.ast())
            .map_err(|_| RawRootSourceFactsErrorV1::Catalog)?;
        let ASTNode::Program { statements, .. } = source.ast() else {
            return Err(RawRootSourceFactsErrorV1::RootNotProgram);
        };
        match plan.kind() {
            RawRootKindV1::Script(_) => Ok(Self {
                route: RawRootSourceRouteV1::Script,
                physical: plan.physical(),
                main: None,
                helper_schedule: Box::new([]),
                callable_catalog: catalog,
                body: RawRootBodyFactV1::Script(classify_program(statements, &[])?),
            }),
            RawRootKindV1::App(app) => {
                let Some(ASTNode::BoxDeclaration {
                    name,
                    methods,
                    is_static,
                    ..
                }) = statements.get(app.main().top_level_statement())
                else {
                    return Err(RawRootSourceFactsErrorV1::MainLocatorDrift);
                };
                if name != "Main" || !*is_static {
                    return Err(RawRootSourceFactsErrorV1::MainLocatorDrift);
                }
                let Some(ASTNode::FunctionDeclaration {
                    body,
                    params,
                    param_decls,
                    return_type_name,
                    uses,
                    contracts,
                    attrs,
                    ..
                }) = methods.get(app.main().method_name())
                else {
                    return Err(RawRootSourceFactsErrorV1::MainLocatorDrift);
                };
                if !params.is_empty()
                    || !param_decls.is_empty()
                    || return_type_name.is_some()
                    || !uses.is_empty()
                    || !contracts.is_empty()
                    || !attrs.is_empty()
                {
                    return Err(RawRootSourceFactsErrorV1::AppMainMetadata);
                }
                Ok(Self {
                    route: RawRootSourceRouteV1::App,
                    physical: plan.physical(),
                    main: Some(app.main().clone()),
                    helper_schedule: app.static_children().to_vec().into_boxed_slice(),
                    callable_catalog: catalog,
                    body: RawRootBodyFactV1::App {
                        main: app.main().clone(),
                        body: classify_program(body, &[])?,
                    },
                })
            }
        }
    }

    pub(in crate::mir) const fn route(&self) -> RawRootSourceRouteV1 {
        self.route
    }
    pub(in crate::mir) const fn physical(&self) -> RawPhysicalRootIdentityV1 {
        self.physical
    }
    pub(in crate::mir) fn main(&self) -> Option<&RawSourceLocatorV1> {
        self.main.as_ref()
    }
    pub(in crate::mir) fn helper_schedule(&self) -> &[RawSourceLocatorV1] {
        &self.helper_schedule
    }

    pub(in crate::mir) const fn helper_count(&self) -> usize {
        self.helper_schedule.len()
    }

    pub(in crate::mir) fn body_statement_count(&self) -> usize {
        match &self.body {
            RawRootBodyFactV1::Script(program) => program.statements().len(),
            RawRootBodyFactV1::App { body, .. } => body.statements().len(),
        }
    }

    pub(in crate::mir) fn callable_count(&self) -> usize {
        self.callable_catalog.len()
    }
    pub(in crate::mir) fn callable_catalog(
        &self,
    ) -> &VerifiedSameModuleCallableDeclarationCatalogV1 {
        &self.callable_catalog
    }
    pub(in crate::mir) fn body(&self) -> &RawRootBodyFactV1 {
        &self.body
    }

    pub(in crate::mir) fn into_post_install_parts(
        self,
    ) -> (
        RawRootPostInstallFactsV1,
        VerifiedSameModuleCallableDeclarationCatalogV1,
    ) {
        let Self {
            route,
            physical,
            main,
            helper_schedule,
            callable_catalog,
            body,
        } = self;
        (
            RawRootPostInstallFactsV1 {
                route,
                physical,
                main,
                helper_schedule,
                body,
            },
            callable_catalog,
        )
    }
}

impl RawRootPostInstallFactsV1 {
    pub(in crate::mir) const fn route(&self) -> RawRootSourceRouteV1 {
        self.route
    }

    pub(in crate::mir) const fn physical(&self) -> RawPhysicalRootIdentityV1 {
        self.physical
    }

    pub(in crate::mir) fn main(&self) -> Option<&RawSourceLocatorV1> {
        self.main.as_ref()
    }

    pub(in crate::mir) fn helper_schedule(&self) -> &[RawSourceLocatorV1] {
        &self.helper_schedule
    }

    pub(in crate::mir) fn body(&self) -> &RawRootBodyFactV1 {
        &self.body
    }

    pub(in crate::mir) fn into_linear_body_recipe(
        self,
    ) -> Result<RawRootBodyRecipeV1, RawRootBodyRecipeErrorV1> {
        let (entry, body) = match self.body {
            RawRootBodyFactV1::Script(program) => (RawRootBodyEntryV1::Script, program),
            RawRootBodyFactV1::App { main, body } => (
                RawRootBodyEntryV1::AppMain0Void {
                    top_level_statement: main.top_level_statement(),
                },
                body,
            ),
        };
        let statements = body
            .statements
            .into_vec()
            .into_iter()
            .map(linear_statement)
            .collect::<Result<Vec<_>, _>>()?
            .into_boxed_slice();
        RawRootBodyRecipeV1::from_parts(entry, statements)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir) enum RawRootSourceFactsErrorV1 {
    RootNotProgram,
    MainLocatorDrift,
    Catalog,
    Scalar { path: Box<[usize]> },
    AppMainMetadata,
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
        other => Err(RawRootBodyRecipeErrorV1::UnsupportedStatement {
            path: statement_path(&other),
        }),
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

fn classify_program(
    nodes: &[ASTNode],
    prefix: &[usize],
) -> Result<RawLocatedScalarProgramV1, RawRootSourceFactsErrorV1> {
    let mut statements = Vec::with_capacity(nodes.len());
    for (index, node) in nodes.iter().enumerate() {
        let mut path = prefix.to_vec();
        path.push(index);
        statements.push(classify_stmt(node, &path)?);
    }
    Ok(RawLocatedScalarProgramV1 {
        statements: statements.into_boxed_slice(),
    })
}

fn classify_expr(
    node: &ASTNode,
    path: &[usize],
) -> Result<RawLocatedScalarExprV1, RawRootSourceFactsErrorV1> {
    match node {
        ASTNode::Literal { value, span } => Ok(RawLocatedScalarExprV1::Literal {
            value: value.clone(),
            site: RawSourceSiteV1::new(path, *span),
        }),
        ASTNode::Variable { name, span } => Ok(RawLocatedScalarExprV1::Variable {
            name: name.clone().into_boxed_str(),
            site: RawSourceSiteV1::new(path, *span),
        }),
        ASTNode::UnaryOp {
            operator,
            operand,
            span,
        } => {
            let operator = match operator {
                UnaryOperator::Minus => RawScalarUnaryOperatorV1::Minus,
                UnaryOperator::Not => RawScalarUnaryOperatorV1::Not,
                UnaryOperator::BitNot => RawScalarUnaryOperatorV1::BitNot,
                UnaryOperator::Weak => {
                    return Err(RawRootSourceFactsErrorV1::Scalar { path: path.into() })
                }
            };
            let mut child = path.to_vec();
            child.push(0);
            Ok(RawLocatedScalarExprV1::Unary {
                operator,
                operand: Box::new(classify_expr(operand, &child)?),
                site: RawSourceSiteV1::new(path, *span),
            })
        }
        ASTNode::BinaryOp {
            operator,
            left,
            right,
            span,
        } => {
            let mut l = path.to_vec();
            l.push(0);
            let mut r = path.to_vec();
            r.push(1);
            Ok(RawLocatedScalarExprV1::Binary {
                operator: operator.clone(),
                left: Box::new(classify_expr(left, &l)?),
                right: Box::new(classify_expr(right, &r)?),
                site: RawSourceSiteV1::new(path, *span),
            })
        }
        _ => Err(RawRootSourceFactsErrorV1::Scalar { path: path.into() }),
    }
}

fn classify_stmt(
    node: &ASTNode,
    path: &[usize],
) -> Result<RawLocatedScalarStmtV1, RawRootSourceFactsErrorV1> {
    let site = |span| RawSourceSiteV1::new(path, span);
    match node {
        ASTNode::Literal { span, .. }
        | ASTNode::Variable { span, .. }
        | ASTNode::UnaryOp { span, .. }
        | ASTNode::BinaryOp { span, .. } => Ok(RawLocatedScalarStmtV1::Expr {
            expression: classify_expr(node, path)?,
            site: site(*span),
        }),
        ASTNode::Print { expression, span } => {
            let mut p = path.to_vec();
            p.push(0);
            Ok(RawLocatedScalarStmtV1::Print {
                expression: classify_expr(expression, &p)?,
                site: site(*span),
            })
        }
        ASTNode::Assignment {
            target,
            value,
            span,
        } => {
            let ASTNode::Variable { name, .. } = target.as_ref() else {
                return Err(RawRootSourceFactsErrorV1::Scalar { path: path.into() });
            };
            let mut p = path.to_vec();
            p.push(1);
            Ok(RawLocatedScalarStmtV1::Assignment {
                target: name.clone().into_boxed_str(),
                value: classify_expr(value, &p)?,
                site: site(*span),
            })
        }
        ASTNode::CompoundAssignment {
            target,
            operator,
            value,
            span,
        } => {
            let ASTNode::Variable { name, .. } = target.as_ref() else {
                return Err(RawRootSourceFactsErrorV1::Scalar { path: path.into() });
            };
            let mut p = path.to_vec();
            p.push(1);
            Ok(RawLocatedScalarStmtV1::CompoundAssignment {
                target: name.clone().into_boxed_str(),
                operator: operator.clone(),
                value: classify_expr(value, &p)?,
                site: site(*span),
            })
        }
        ASTNode::Local {
            variables,
            initial_values,
            declared_type_names,
            span,
        } => {
            if variables.len() != initial_values.len()
                || variables.len() != declared_type_names.len()
                || declared_type_names.iter().any(Option::is_some)
            {
                return Err(RawRootSourceFactsErrorV1::Scalar { path: path.into() });
            }
            let initialized = initial_values
                .iter()
                .enumerate()
                .map(|(i, value)| {
                    value
                        .as_deref()
                        .map(|expr| {
                            let mut p = path.to_vec();
                            p.push(i);
                            classify_expr(expr, &p)
                        })
                        .transpose()
                })
                .collect::<Result<Vec<_>, _>>()?;
            Ok(RawLocatedScalarStmtV1::Local {
                variables: variables
                    .iter()
                    .map(|v| v.clone().into_boxed_str())
                    .collect(),
                initialized: initialized.into_boxed_slice(),
                site: site(*span),
            })
        }
        ASTNode::If {
            condition,
            then_body,
            else_body,
            span,
        } => {
            let mut c = path.to_vec();
            c.push(0);
            let mut t = path.to_vec();
            t.push(1);
            let mut e = path.to_vec();
            e.push(2);
            let else_body = else_body
                .as_deref()
                .map(|body| classify_program(body, &e).map(|p| p.statements))
                .transpose()?;
            Ok(RawLocatedScalarStmtV1::If {
                condition: classify_expr(condition, &c)?,
                then_body: classify_program(then_body, &t)?.statements,
                else_body,
                site: site(*span),
            })
        }
        ASTNode::Loop {
            condition,
            body,
            span,
        } => {
            let mut c = path.to_vec();
            c.push(0);
            let mut b = path.to_vec();
            b.push(1);
            Ok(RawLocatedScalarStmtV1::Loop {
                condition: classify_expr(condition, &c)?,
                body: classify_program(body, &b)?.statements,
                site: site(*span),
            })
        }
        ASTNode::LoopRange {
            var_name,
            start,
            end,
            body,
            span,
        } => {
            let mut s = path.to_vec();
            s.push(0);
            let mut e = path.to_vec();
            e.push(1);
            let mut b = path.to_vec();
            b.push(2);
            Ok(RawLocatedScalarStmtV1::LoopRange {
                variable: var_name.clone().into_boxed_str(),
                start: classify_expr(start, &s)?,
                end: classify_expr(end, &e)?,
                body: classify_program(body, &b)?.statements,
                site: site(*span),
            })
        }
        ASTNode::Return { value, span } => {
            let mut v = path.to_vec();
            v.push(0);
            let value = value
                .as_deref()
                .map(|expr| classify_expr(expr, &v))
                .transpose()?;
            Ok(RawLocatedScalarStmtV1::Return {
                value,
                site: site(*span),
            })
        }
        ASTNode::Break { span } => Ok(RawLocatedScalarStmtV1::Break { site: site(*span) }),
        ASTNode::Continue { span } => Ok(RawLocatedScalarStmtV1::Continue { site: site(*span) }),
        ASTNode::ScopeBox { body, span } => Ok(RawLocatedScalarStmtV1::ScopeBox {
            body: classify_program(body, &path_with_child(path, 0))?.statements,
            site: site(*span),
        }),
        _ => Err(RawRootSourceFactsErrorV1::Scalar { path: path.into() }),
    }
}

fn path_with_child(path: &[usize], child: usize) -> Vec<usize> {
    let mut nested = path.to_vec();
    nested.push(child);
    nested
}
