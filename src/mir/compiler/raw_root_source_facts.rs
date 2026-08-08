//! DECLACCESS-MANIFEST0: exact source facts for the first Raw root slice.
//!
//! This is a source-only product. It owns the located ScalarControl0 payload
//! needed by BODY0 and seals the existing complete callable catalog once. It
//! never opens a Builder or reads ambient module state.

use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span, UnaryOperator};
use crate::mir::builder::{RawSourceLocatorV1, VerifiedSameModuleCallableDeclarationCatalogV1};
use crate::mir::raw_root_body_recipe::{
    RawRootBodyRecipeErrorV1, RawRootBodyRecipeV1, RawScriptBodyRecipeV1,
};

use super::raw_root_plan0::{RawPhysicalRootIdentityV1, RawRootKindV1, RawRootPlanV1};

mod recipe_projection;

#[cfg(test)]
mod script_result_p0;

/// Source-only rejection emitted while projecting the shared Script recipe.
///
/// This projection deliberately has no Raw root-plan, Builder, publication,
/// or invocation-brand input. Raw and canonical Script candidates share the
/// same source result classification through this boundary.
#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir) enum RawScriptRecipeProjectionErrorV1 {
    RootNotProgram,
    SourceFacts(RawRootSourceFactsErrorV1),
    Recipe(RawRootBodyRecipeErrorV1),
}

/// Project one parsed Script program into the shared source-classified body
/// recipe. This is intentionally narrower than `RawRootSourceFactsV1`: it
/// does not select a physical root, open a Raw invocation, or seal a
/// publication route.
pub(in crate::mir) fn project_raw_script_body_recipe_v1(
    source: &ASTNode,
) -> Result<RawScriptBodyRecipeV1, RawScriptRecipeProjectionErrorV1> {
    let ASTNode::Program { statements, .. } = source else {
        return Err(RawScriptRecipeProjectionErrorV1::RootNotProgram);
    };
    let program =
        classify_program(statements, &[]).map_err(RawScriptRecipeProjectionErrorV1::SourceFacts)?;
    recipe_projection::project_script_recipe(program)
        .map_err(RawScriptRecipeProjectionErrorV1::Recipe)
}

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

/// Source-owned Script result boundary.  It is deliberately located and
/// structural; no Builder ValueId or physical Return is present here.
#[derive(Debug, PartialEq)]
pub(in crate::mir) struct RawScriptResultContractV1 {
    pub(in crate::mir) prelude: Box<[RawLocatedScalarStmtV1]>,
    pub(in crate::mir) terminal: RawLocatedScriptTerminalV1,
}

#[derive(Debug, PartialEq)]
pub(in crate::mir) enum RawLocatedScriptTerminalV1 {
    EmptyUnit,
    ValueExpression {
        expression: RawLocatedScalarExprV1,
    },
    UnitExpression {
        expression: RawLocatedScalarExprV1,
        origin: crate::mir::raw_root_body_recipe::RawScriptUnitOriginV1,
    },
    UnitStatement {
        statement: RawLocatedScalarStmtV1,
        origin: crate::mir::raw_root_body_recipe::RawScriptUnitOriginV1,
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
    body_recipe: RawRootBodyRecipeV1,
    body_statement_count: usize,
    callable_count: usize,
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
                }) = methods.get_declaration(app.main().method_name())
                else {
                    return Err(RawRootSourceFactsErrorV1::MainLocatorDrift);
                };
                let metadata = RawAppMainMetadataFactsV1 {
                    parameter_count: params.len(),
                    parameter_decl_count: param_decls.len(),
                    return_annotation_present: return_type_name.is_some(),
                    uses_count: uses.len(),
                    contract_count: contracts.len(),
                    rune_count: attrs.runes.len(),
                };
                if !metadata.is_empty() {
                    return Err(RawRootSourceFactsErrorV1::AppMainMetadata { metadata });
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

    pub(in crate::mir) const fn helper_count(&self) -> usize {
        self.helper_schedule.len()
    }

    pub(in crate::mir) fn body_recipe(&self) -> &RawRootBodyRecipeV1 {
        &self.body_recipe
    }

    pub(in crate::mir) fn body_statement_count(&self) -> usize {
        self.body_statement_count
    }

    pub(in crate::mir) fn callable_count(&self) -> usize {
        self.callable_count
    }

    pub(in crate::mir) fn into_linear_body_recipe(self) -> RawRootBodyRecipeV1 {
        self.body_recipe
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir) enum RawRootSourceFactsErrorV1 {
    RootNotProgram,
    MainLocatorDrift,
    Catalog,
    Scalar { path: Box<[usize]> },
    AppMainMetadata { metadata: RawAppMainMetadataFactsV1 },
}

/// Source facts already observed while checking the narrow `Main.main/0`
/// contract.  Keeping them in the rejection avoids a second AST inspection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) struct RawAppMainMetadataFactsV1 {
    parameter_count: usize,
    parameter_decl_count: usize,
    return_annotation_present: bool,
    uses_count: usize,
    contract_count: usize,
    rune_count: usize,
}

impl RawAppMainMetadataFactsV1 {
    pub(in crate::mir) const fn parameter_count(self) -> usize {
        self.parameter_count
    }

    pub(in crate::mir) const fn parameter_decl_count(self) -> usize {
        self.parameter_decl_count
    }

    pub(in crate::mir) const fn return_annotation_present(self) -> bool {
        self.return_annotation_present
    }

    pub(in crate::mir) const fn uses_count(self) -> usize {
        self.uses_count
    }

    pub(in crate::mir) const fn contract_count(self) -> usize {
        self.contract_count
    }

    pub(in crate::mir) const fn rune_count(self) -> usize {
        self.rune_count
    }

    const fn is_empty(self) -> bool {
        self.parameter_count == 0
            && self.parameter_decl_count == 0
            && !self.return_annotation_present
            && self.uses_count == 0
            && self.contract_count == 0
            && self.rune_count == 0
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
